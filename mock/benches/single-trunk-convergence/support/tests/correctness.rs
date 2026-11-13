//! Every arm against `sequential` and against a reference computed here.
//!
//! The harness validates arms against each other before it times them, on
//! the sizes and seeds of the bench sections. These tests cover what that
//! does not: record counts that are not a multiple of the morsel or of the
//! worker count, worker counts the bench does not sweep (3), fewer records
//! than workers, and the append's output sequence, including what a
//! head-and-tail append without the design's sequential fallback produces.

use stc_support::arms::{
    Arm,
    HeadTail,
    HeadTailDynamic,
    KernelNway,
    KernelSeq,
    Nway,
    Sequential,
    StdParallel,
    run_once,
};
use stc_support::engine::Op;
use stc_support::kernel::{Heavy, Kernel, Light, Mix, fold_step};
use stc_support::split::{HeadTailDyn, HeadTailMid, Split, morsel};
use stc_support::workload::{APPEND, FOLD_HEAVY, FOLD_LIGHT};
use stc_support::{ConvInput, ConvOutput, cache, data, walk};

const SEED: u64 = 0x5EED_CAFE_F00D_0001;

/// Record counts: below one morsel, ragged, exactly one morsel, ragged
/// across several, and a size where every worker gets many morsels.
fn sizes() -> [usize; 7] {
    let m = morsel();
    [1, 3, 17, m, m + 1, 5 * m + 37, 64 * m + 5]
}

const WORKERS: [u32; 5] = [1, 2, 3, 4, 8];

fn reference_fold<K: Kernel>(xs: &[u32]) -> ConvOutput {
    ConvOutput {
        value: xs.iter().fold(0u64, |a, &x| fold_step::<K>(a, x)),
        count: xs.len() as u64,
    }
}

fn reference_append<K: Kernel>(xs: &[u32]) -> (Vec<u32>, ConvOutput) {
    let out: Vec<u32> = xs.iter().map(|&x| K::apply(x)).collect();
    let o = ConvOutput {
        value: data::hash(&out),
        count: out.len() as u64,
    };
    (out, o)
}

fn run<A: Arm>(kind: u32, workers: u32, n: usize) -> ConvOutput {
    let out = run_once::<A>(
        &ConvInput {
            seed: SEED,
            kind,
            workers,
        },
        n,
    );
    cache::clear();
    out
}

/// Run every arm on one workload and compare each with `sequential` and with
/// the reference, naming every arm that differs.
fn check_all(kind: u32, reference: impl Fn(&[u32]) -> ConvOutput) {
    let mut wrong = Vec::new();
    for n in sizes() {
        let xs = data::column(SEED, n);
        let want = reference(&xs);
        let seq = run::<Sequential>(kind, 1, n);
        if seq != want {
            wrong.push(format!("sequential n={n}: {seq:?}, reference {want:?}"));
        }
        for w in WORKERS {
            let arms: [(&str, ConvOutput); 6] = [
                ("nway", run::<Nway>(kind, w, n)),
                ("head_tail", run::<HeadTail>(kind, w, n)),
                ("head_tail_dyn", run::<HeadTailDynamic>(kind, w, n)),
                ("kernel_seq", run::<KernelSeq>(kind, w, n)),
                ("kernel_nway", run::<KernelNway>(kind, w, n)),
                ("std_parallel", run::<StdParallel>(kind, w, n)),
            ];
            for (name, got) in arms {
                if got != seq {
                    wrong.push(format!("{name} n={n} W={w}: {got:?}, sequential {seq:?}"));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "arms that differ:\n{}", wrong.join("\n"));
}

#[test]
fn every_arm_equals_sequential_on_the_light_fold() {
    check_all(FOLD_LIGHT, reference_fold::<Light>);
}

#[test]
fn every_arm_equals_sequential_on_the_heavy_fold() {
    check_all(FOLD_HEAVY, reference_fold::<Heavy>);
}

/// The append's output is a sequence, and every arm the bench times keeps
/// the sequence `sequential` produces: the N-way arms because each core
/// appends its own slice into its own region and the regions are compacted
/// in core sequence, the head-and-tail arms because the design sends a
/// non-commutative fiber to the sequential fallback.
#[test]
fn every_arm_keeps_the_append_sequence() {
    check_all(APPEND, |xs| reference_append::<Mix>(xs).1);
}

/// What head-and-tail does to an append without the fallback: the tail
/// walker appends its half from the last record back, so the merged output
/// holds the head half in sequence and the tail half reversed. The
/// fallback is what keeps the sequence, and this is the difference it
/// prevents.
#[test]
fn head_and_tail_without_the_fallback_reverses_the_tail_half() {
    for n in sizes() {
        let xs = data::column(SEED, n);
        let (want, want_out) = reference_append::<Mix>(&xs);
        let mut arm = walk::append::<Mix, HeadTailMid>(xs.clone(), HeadTailMid::new(), 2);
        arm(Op::Frame);
        let got = arm(Op::Read);
        let mid = n / 2;
        let mut expect: Vec<u32> = want[.. mid].to_vec();
        expect.extend(want[mid ..].iter().rev());
        let expect_out = ConvOutput {
            value: data::hash(&expect),
            count: n as u64,
        };
        assert_eq!(
            got, expect_out,
            "n={n}: the split append is not head then reversed tail"
        );
        let reversed_tail_differs = want[mid ..].iter().ne(want[mid ..].iter().rev());
        assert_eq!(
            got != want_out,
            reversed_tail_differs,
            "n={n}: the split append differs from sequential exactly when its tail half is not \
             a palindrome"
        );
    }
}

/// The floating meeting point lands on a record the two walkers both
/// reached, and every record is folded once, at every size and on repeated
/// frames over one state.
#[test]
fn the_dynamic_meeting_point_covers_every_record_once() {
    for n in sizes() {
        let xs = data::column(SEED, n);
        let want = reference_fold::<Light>(&xs);
        let split = HeadTailDyn::new();
        for frame in 0 .. 8 {
            split.begin(n);
            let mut covered = vec![0u8; n];
            for core in 0 .. 2 {
                split.walk(core, 2, n, &mut |r, _| {
                    for i in r {
                        covered[i] += 1;
                    }
                });
            }
            assert!(
                covered.iter().all(|&c| c == 1),
                "n={n} frame {frame}: a record covered other than once"
            );
            assert!(
                split.meeting() <= n,
                "n={n}: meeting record {} past the fiber",
                split.meeting()
            );
        }
        let mut arm = walk::fold::<Light, HeadTailDyn>(xs, HeadTailDyn::new(), 2);
        for _ in 0 .. 8 {
            arm(Op::Frame);
            assert_eq!(
                arm(Op::Read),
                want,
                "n={n}: the two-walker dynamic fold is off"
            );
        }
    }
}

/// The negative case for the checks above: a split that walks a range twice
/// is caught by both the reference comparison and the count.
#[test]
fn a_double_walked_range_is_caught() {
    struct Twice;
    impl Split for Twice {
        fn walk(
            &self,
            _core: usize,
            _walkers: usize,
            n: usize,
            visit: &mut impl FnMut(std::ops::Range<usize>, stc_support::split::Dir),
        ) {
            visit(0 .. n, stc_support::split::Dir::Forward);
            visit(0 .. n.min(1), stc_support::split::Dir::Forward);
        }
    }
    let xs = data::column(SEED, 100);
    let want = reference_fold::<Light>(&xs);
    let mut arm = walk::fold::<Light, Twice>(xs, Twice, 1);
    arm(Op::Frame);
    let got = arm(Op::Read);
    assert_ne!(got, want);
    assert_eq!(got.count, want.count + 1);
}

/// The walker count each arm's fold is built with: head-and-tail runs two
/// walkers at any W from 2 up and one at W = 1, the crew's N-way runs W, the
/// one-walker arms one. Every fold takes its count from `Arm::walkers`, so
/// this holds what the bench times, and a head-and-tail at W = 4 compared with
/// an N-way at W = 4 is two walkers against four.
#[test]
fn every_arm_runs_the_walkers_it_claims() {
    for w in 1 ..= 8 {
        assert_eq!(HeadTail::walkers(w), w.min(2), "head_tail at W={w}");
        assert_eq!(
            HeadTailDynamic::walkers(w),
            w.min(2),
            "head_tail_dyn at W={w}"
        );
        assert_eq!(KernelNway::walkers(w), w, "kernel_nway at W={w}");
        assert_eq!(KernelSeq::walkers(w), 1, "kernel_seq at W={w}");
        assert_eq!(Sequential::walkers(w), 1, "sequential at W={w}");
    }
}

/// The floating meeting point with both walkers claiming at once, so the
/// compare and swap on the shared cursor is contended: every record is still
/// walked once, and the meeting record lands inside the fiber. The test above
/// walks core 0 to the end before core 1 starts, which never enters the race.
///
/// The walkers start together behind a barrier, over enough blocks that each
/// one claims some on most frames. A frame where one walker claimed nothing
/// proves no race, so the test also asserts that both claimed blocks on some
/// frame; without it a host that ran the threads one after another would pass.
#[test]
fn the_dynamic_meeting_point_holds_under_contended_claims() {
    use std::sync::Barrier;
    use std::sync::atomic::{AtomicU8, AtomicUsize, Ordering};

    let n = morsel() * 512 + 37;
    let split = HeadTailDyn::new();
    let covered: Vec<AtomicU8> = (0 .. n).map(|_| AtomicU8::new(0)).collect();
    let mut both_claimed = 0usize;
    for frame in 0 .. 64 {
        split.begin(n);
        covered.iter().for_each(|c| c.store(0, Ordering::Relaxed));
        let claims = [AtomicUsize::new(0), AtomicUsize::new(0)];
        let start = Barrier::new(2);
        std::thread::scope(|sc| {
            for core in 0 .. 2 {
                let (split, covered, claims, start) = (&split, &covered, &claims, &start);
                sc.spawn(move || {
                    start.wait();
                    split.walk(core, 2, n, &mut |r, _| {
                        claims[core].fetch_add(1, Ordering::Relaxed);
                        for i in r {
                            covered[i].fetch_add(1, Ordering::Relaxed);
                        }
                    });
                });
            }
        });
        let bad = covered.iter().position(|c| c.load(Ordering::Relaxed) != 1);
        assert!(
            bad.is_none(),
            "frame {frame}: record {bad:?} covered other than once"
        );
        assert!(
            split.meeting() <= n,
            "frame {frame}: meeting record {} past the fiber",
            split.meeting()
        );
        if claims.iter().all(|c| c.load(Ordering::Relaxed) > 0) {
            both_claimed += 1;
        }
    }
    assert!(
        both_claimed > 0,
        "no frame had both walkers claiming, so the race was never run"
    );
}
