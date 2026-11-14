//! Every arm of every workload against the reference digest, at a size small
//! enough to run in the default suite.
//!
//! The harness validates each cell before timing it, but only on a timing run.
//! These are the same comparison on every `run-checks.sh`, so an arm that
//! drifted from its workload is caught by the checks rather than by a run that
//! takes minutes to reach it.

use mockspace_bench_core::Routine;

use super::*;

/// Records per check. Not a multiple of the morsel, so a tail is exercised.
const N: usize = 4096 + 37;

/// The arms each workload has.
fn kinds(w: Workload) -> Vec<ArmKind> {
    let mut k = vec![ArmKind::Engine, ArmKind::Std, ArmKind::EngineStartup, ArmKind::StdStartup];
    if w.has_parallel() {
        k.extend([ArmKind::EnginePar, ArmKind::StdPar]);
    }
    k
}

/// One call of an arm, the way `bench_arm::call` makes it.
fn once(arm: &mut dyn Arm<u64>, records: &[u32]) -> u64 {
    arm.load(records);
    arm.frame();
    arm.digest()
}

#[test]
fn every_arm_computes_the_reference_and_follows_its_input() {
    for w in Workload::ALL {
        let first = tagged_records_vec(w, 1);
        let second = tagged_records_vec(w, 2);
        let (want1, want2) = (w.reference(&first), w.reference(&second));
        assert_ne!(want1, want2, "{}: two seeds give one reference", w.name());
        for kind in kinds(w) {
            let mut arm = w.arm(kind, N);
            // Twice, on different input, so an arm that kept its first frame's
            // output or never reread its input fails the second.
            assert_eq!(
                once(&mut *arm, &first),
                want1,
                "{} {kind:?}, first call",
                w.name()
            );
            assert_eq!(
                once(&mut *arm, &second),
                want2,
                "{} {kind:?}, second call",
                w.name()
            );
            // A threaded engine arm has workers parked on its scheduler, so it
            // is kept alive as the worker processes keep theirs.
            std::mem::forget(arm);
        }
    }
}

#[test]
fn a_workload_with_no_threaded_arm_refuses_one() {
    for w in Workload::ALL.into_iter().filter(|w| !w.has_parallel()) {
        for kind in [ArmKind::EnginePar, ArmKind::StdPar] {
            let refused = std::panic::catch_unwind(|| drop(w.arm(kind, N)));
            assert!(refused.is_err(), "{} built a {kind:?} arm", w.name());
        }
    }
}

#[test]
fn the_tags_and_names_round_trip() {
    for w in Workload::ALL {
        assert_eq!(Workload::from_tag(w.tag()), Some(w));
        assert_eq!(Workload::from_name(w.name()), Some(w));
    }
    assert_eq!(Workload::from_tag(0), None);
    assert_eq!(Workload::from_name("nothing"), None);
}

/// The routine's check is what the harness relies on; it has to refuse a
/// wrong digest and a wrong tag, or validation passes arms it never compared.
#[test]
fn the_routine_refuses_a_wrong_digest_and_a_wrong_tag() {
    const R: usize = 256;
    type Ew = Evs<R, 1>;
    let input = <Ew as Routine>::build_input(9);
    let right = Workload::ElementWise.reference(&input.records);
    assert!(<Ew as Routine>::validate_output(&input, &right).is_ok());
    assert!(<Ew as Routine>::validate_output(&input, &(right ^ 1)).is_err());
    let other = <Evs<R, 2> as Routine>::build_input(9);
    assert!(<Ew as Routine>::validate_output(&other, &right).is_err());
}

/// The workloads differ in what they compute, with one pair alike by design:
/// `accumulator` appends exactly the chain `element_wise` writes, so the two
/// differ only in the dispatch path that produces it. A section naming the
/// wrong one of those two is caught by the tag in `validate_output`, which the
/// test above holds to refusing.
#[test]
fn the_references_differ_but_for_the_one_pair_alike_by_design() {
    let records = tagged_records_vec(Workload::ElementWise, 3);
    let r = |w: Workload| w.reference(&records);
    assert_eq!(r(Workload::ElementWise), r(Workload::Accumulator));
    let distinct = [Workload::ElementWise, Workload::Branching, Workload::WideParallel].map(r);
    assert_ne!(distinct[0], distinct[1]);
    assert_ne!(distinct[0], distinct[2]);
    assert_ne!(distinct[1], distinct[2]);
}

/// The record column the harness would hand an arm of `w` for `seed`.
fn tagged_records_vec(w: Workload, seed: u64) -> Vec<u32> {
    let bytes = bench_arm::tagged_bytes::<N>(w.tag(), seed);
    bytes[8 ..]
        .chunks_exact(4)
        .map(|c| u32::from_le_bytes([c[0], c[1], c[2], c[3]]))
        .collect()
}

/// The control for `every_arm_computes_the_reference_and_follows_its_input`:
/// an arm with one stage dropped has to fail the same comparison, or that test
/// would pass arms it cannot tell apart.
#[test]
fn an_arm_with_a_stage_dropped_fails_the_comparison() {
    let records = tagged_records_vec(Workload::ElementWise, 1);
    let mut dropped = std_arm(
        (Vec::new(), vec![0u32; N]),
        |b| &mut b.0,
        |b| {
            for (d, &v) in b.1.iter_mut().zip(b.0.iter()) {
                *d = stage4(stage3(stage1(v)));
            }
        },
        |b| fnv1a_u32(&b.1),
    );
    assert_ne!(
        once(&mut *dropped, &records),
        Workload::ElementWise.reference(&records)
    );
}

/// `Crew::for_chunks` hands every worker the same range of the input and of
/// each output, and the ranges cover every record once: the outputs equal a
/// serial map, at a length that is not a multiple of the workers, at fewer
/// records than workers, and at none, for one worker, two, three and as many
/// as the crew takes.
#[test]
fn the_crew_covers_every_record_once_in_its_workers_ranges() {
    for workers in [1, 2, 3, hilavitkutin::thread::MAX_CORES] {
        let mut crew = Crew::new(workers);
        assert_eq!(crew.workers(), workers);
        for n in [0usize, 2, 3, 10, 1001] {
            let input: Vec<u32> = (0 .. n as u32).collect();
            let (mut a, mut b) = (vec![u32::MAX; n], vec![u32::MAX; n]);
            crew.for_chunks(&input, [&mut a[..], &mut b[..]], &|i, [x, y]| {
                for (k, &v) in i.iter().enumerate() {
                    x[k] = v * 2;
                    y[k] = v + 7;
                }
            });
            assert_eq!(
                a,
                input.iter().map(|v| v * 2).collect::<Vec<_>>(),
                "n={n}, workers={workers}"
            );
            assert_eq!(
                b,
                input.iter().map(|v| v + 7).collect::<Vec<_>>(),
                "n={n}, workers={workers}"
            );
        }
    }
}

/// The control for the test above: a worker that writes nothing leaves its
/// range at the sentinel, and the comparison sees it.
#[test]
fn a_worker_that_skips_its_range_is_seen() {
    let mut crew = Crew::new(3);
    let input: Vec<u32> = (0 .. 10).collect();
    let mut a = vec![u32::MAX; 10];
    crew.for_chunks(&input, [&mut a[..]], &|i, [x]| {
        if i.first() != Some(&0) {
            for (k, &v) in i.iter().enumerate() {
                x[k] = v * 2;
            }
        }
    });
    assert_ne!(a, input.iter().map(|v| v * 2).collect::<Vec<_>>());
    assert_eq!(
        &a[.. 4],
        &[u32::MAX; 4],
        "the first worker's range is four records"
    );
}

/// A worker whose closure panics still ends the frame: the panic is raised
/// on the caller, as a scoped thread's is at its join, rather than leaving
/// the caller waiting on a barrier one worker never reaches. The crew runs
/// the next frame and drops cleanly after it.
#[test]
fn a_worker_that_panics_fails_the_frame_instead_of_hanging_it() {
    let mut crew = Crew::new(3);
    let caught = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        crew.frame(&|core: usize| {
            if core == 1 {
                panic!("worker one fails");
            }
        });
    }));
    let payload = caught.expect_err("a worker panicked and the frame returned normally");
    assert_eq!(payload.downcast_ref::<&str>(), Some(&"worker one fails"));
    let ran = std::sync::atomic::AtomicUsize::new(0);
    crew.frame(&|_core: usize| {
        ran.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    });
    assert_eq!(ran.into_inner(), 3, "the crew runs a frame after the panic");
}
