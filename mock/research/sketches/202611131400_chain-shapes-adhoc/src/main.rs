//! Ad-hoc spike: one four-stage map over `u32` columns, written in the shapes
//! an engine, a framework or a person could give it, each timed with
//! `std::time::Instant`. It is a spike of the stack audit and no bench: one
//! machine, one kernel, no harness, no surrounding workload.
//!
//! The four stages, per record:
//!
//! ```text
//! t1  = a * 3 + b
//! t2  = t1 ^ (t1 >> 7)
//! t3  = t2 * 0x9E3779B1 + c
//! out = rotl(t3, 5) ^ a
//! ```
//!
//! All arithmetic wraps. Every arm must produce the same `out` column; the
//! driver checks a checksum of it against the first arm before it times.

use std::hint::black_box;
use std::time::Instant;

const K: u32 = 0x9E37_79B1;

#[inline(always)]
fn s1(a: u32, b: u32) -> u32 {
    a.wrapping_mul(3).wrapping_add(b)
}
#[inline(always)]
fn s2(t1: u32) -> u32 {
    t1 ^ (t1 >> 7)
}
#[inline(always)]
fn s3(t2: u32, c: u32) -> u32 {
    t2.wrapping_mul(K).wrapping_add(c)
}
#[inline(always)]
fn s4(t3: u32, a: u32) -> u32 {
    t3.rotate_left(5) ^ a
}

/// The columns of one run. `t1..t3` are the intermediates a unit-per-stage
/// engine keeps as stores; the fused arms never touch them.
struct Cols {
    a:   Vec<u32>,
    b:   Vec<u32>,
    c:   Vec<u32>,
    t1:  Vec<u32>,
    t2:  Vec<u32>,
    t3:  Vec<u32>,
    out: Vec<u32>,
}

impl Cols {
    fn new(n: usize) -> Self {
        let mut x = 0x2545_F491_4F6C_DD1Du64;
        let mut next = || {
            x ^= x << 13;
            x ^= x >> 7;
            x ^= x << 17;
            (x >> 16) as u32
        };
        let a: Vec<u32> = (0 .. n).map(|_| next()).collect();
        let b: Vec<u32> = (0 .. n).map(|_| next()).collect();
        let c: Vec<u32> = (0 .. n).map(|_| next()).collect();
        Cols {
            a,
            b,
            c,
            t1: vec![1; n],
            t2: vec![1; n],
            t3: vec![1; n],
            out: vec![1; n],
        }
    }
}

// ---- arm: one fused loop over slices -------------------------------------

#[inline(never)]
fn fused_slices(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    let n = out.len();
    let (a, b, c) = (&a[.. n], &b[.. n], &c[.. n]);
    for i in 0 .. n {
        out[i] = s4(s3(s2(s1(a[i], b[i])), c[i]), a[i]);
    }
}

// ---- arm: one fused loop over raw pointers held in a struct ---------------

/// What a context that hands out raw column pointers looks like to the
/// optimiser: four pointers of unknown relation.
struct RawCols {
    a:   *const u32,
    b:   *const u32,
    c:   *const u32,
    out: *mut u32,
}

#[inline(never)]
fn fused_rawptr(p: &RawCols, n: usize) {
    for i in 0 .. n {
        // SAFETY: the driver hands four live, disjoint buffers of at least `n`.
        unsafe {
            let a = *p.a.add(i);
            *p.out.add(i) = s4(s3(s2(s1(a, *p.b.add(i))), *p.c.add(i)), a);
        }
    }
}

// ---- arm: an iterator chain ----------------------------------------------

#[inline(never)]
fn iter_chain(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    out.iter_mut()
        .zip(a.iter().zip(b.iter()).zip(c.iter()))
        .for_each(|(o, ((&a, &b), &c))| *o = s4(s3(s2(s1(a, b)), c), a));
}

// ---- the four stages as separate loops over a range -----------------------

#[inline(always)]
fn stage1(a: &[u32], b: &[u32], t1: &mut [u32]) {
    for i in 0 .. t1.len() {
        t1[i] = s1(a[i], b[i]);
    }
}
#[inline(always)]
fn stage2(t1: &[u32], t2: &mut [u32]) {
    for i in 0 .. t2.len() {
        t2[i] = s2(t1[i]);
    }
}
#[inline(always)]
fn stage3(t2: &[u32], c: &[u32], t3: &mut [u32]) {
    for i in 0 .. t3.len() {
        t3[i] = s3(t2[i], c[i]);
    }
}
#[inline(always)]
fn stage4(t3: &[u32], a: &[u32], out: &mut [u32]) {
    for i in 0 .. out.len() {
        out[i] = s4(t3[i], a[i]);
    }
}

// ---- arm: one whole pass per stage, intermediates are columns -------------

#[inline(never)]
fn pass_per_stage(k: &mut Cols) {
    let n = k.out.len();
    stage1(&k.a[.. n], &k.b[.. n], &mut k.t1[.. n]);
    stage2(&k.t1[.. n], &mut k.t2[.. n]);
    stage3(&k.t2[.. n], &k.c[.. n], &mut k.t3[.. n]);
    stage4(&k.t3[.. n], &k.a[.. n], &mut k.out[.. n]);
}

// ---- arm: morsel by morsel, a loop per stage, intermediates are columns ---

#[inline(never)]
fn morsel_columns(k: &mut Cols, m: usize) {
    let n = k.out.len();
    let mut lo = 0;
    while lo < n {
        let hi = (lo + m).min(n);
        stage1(&k.a[lo .. hi], &k.b[lo .. hi], &mut k.t1[lo .. hi]);
        stage2(&k.t1[lo .. hi], &mut k.t2[lo .. hi]);
        stage3(&k.t2[lo .. hi], &k.c[lo .. hi], &mut k.t3[lo .. hi]);
        stage4(&k.t3[lo .. hi], &k.a[lo .. hi], &mut k.out[lo .. hi]);
        lo = hi;
    }
}

// ---- arm: morsel by morsel, intermediates in one reused scratch -----------

#[inline(never)]
fn morsel_scratch(k: &mut Cols, m: usize, s: &mut [Vec<u32>; 3]) {
    let n = k.out.len();
    let [x1, x2, x3] = s;
    let mut lo = 0;
    while lo < n {
        let hi = (lo + m).min(n);
        let w = hi - lo;
        stage1(&k.a[lo .. hi], &k.b[lo .. hi], &mut x1[.. w]);
        stage2(&x1[.. w], &mut x2[.. w]);
        stage3(&x2[.. w], &k.c[lo .. hi], &mut x3[.. w]);
        stage4(&x3[.. w], &k.a[lo .. hi], &mut k.out[lo .. hi]);
        lo = hi;
    }
}

// ---- arm: morsel by morsel, each stage behind a function pointer ----------

type Stage = fn(&mut Cols, usize, usize);

fn st1(k: &mut Cols, lo: usize, hi: usize) {
    stage1(&k.a[lo .. hi], &k.b[lo .. hi], &mut k.t1[lo .. hi]);
}
fn st2(k: &mut Cols, lo: usize, hi: usize) {
    stage2(&k.t1[lo .. hi], &mut k.t2[lo .. hi]);
}
fn st3(k: &mut Cols, lo: usize, hi: usize) {
    stage3(&k.t2[lo .. hi], &k.c[lo .. hi], &mut k.t3[lo .. hi]);
}
fn st4(k: &mut Cols, lo: usize, hi: usize) {
    stage4(&k.t3[lo .. hi], &k.a[lo .. hi], &mut k.out[lo .. hi]);
}

#[inline(never)]
fn morsel_fnptr(k: &mut Cols, m: usize, table: &[Stage]) {
    let n = k.out.len();
    let mut lo = 0;
    while lo < n {
        let hi = (lo + m).min(n);
        for f in table {
            f(k, lo, hi);
        }
        lo = hi;
    }
}

// ---- arm: a call per record through a function pointer --------------------

type PerRecord = fn(u32, u32, u32) -> u32;

fn whole(a: u32, b: u32, c: u32) -> u32 {
    s4(s3(s2(s1(a, b)), c), a)
}

#[inline(never)]
fn call_per_record(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32], f: PerRecord) {
    let n = out.len();
    let (a, b, c) = (&a[.. n], &b[.. n], &c[.. n]);
    for i in 0 .. n {
        out[i] = f(a[i], b[i], c[i]);
    }
}

// ---- arm: hand-written AVX2, the stand-in for hand-written assembly -------

#[cfg(target_arch = "x86_64")]
#[target_feature(enable = "avx2")]
#[inline(never)]
unsafe fn hand_avx2(a: &[u32], b: &[u32], c: &[u32], out: &mut [u32]) {
    use std::arch::x86_64::*;
    let n = out.len();
    let three = _mm256_set1_epi32(3);
    let k = _mm256_set1_epi32(K as i32);
    let mut i = 0;
    // SAFETY: every load and store stays below `n`, which every slice covers.
    unsafe {
        while i + 16 <= n {
            for j in [i, i + 8] {
                let va = _mm256_loadu_si256(a.as_ptr().add(j).cast());
                let vb = _mm256_loadu_si256(b.as_ptr().add(j).cast());
                let vc = _mm256_loadu_si256(c.as_ptr().add(j).cast());
                let t1 = _mm256_add_epi32(_mm256_mullo_epi32(va, three), vb);
                let t2 = _mm256_xor_si256(t1, _mm256_srli_epi32::<7>(t1));
                let t3 = _mm256_add_epi32(_mm256_mullo_epi32(t2, k), vc);
                let r = _mm256_or_si256(_mm256_slli_epi32::<5>(t3), _mm256_srli_epi32::<27>(t3));
                _mm256_storeu_si256(out.as_mut_ptr().add(j).cast(), _mm256_xor_si256(r, va));
            }
            i += 16;
        }
    }
    while i < n {
        out[i] = whole(a[i], b[i], c[i]);
        i += 1;
    }
}

// ---- threaded arms: four ceil slices on scoped threads --------------------

#[inline(never)]
fn par_fused(k: &mut Cols, w: usize) {
    let n = k.out.len();
    let per = n.div_ceil(w);
    let (a, b, c) = (&k.a[..], &k.b[..], &k.c[..]);
    std::thread::scope(|s| {
        for (i, out) in k.out.chunks_mut(per).enumerate() {
            let lo = i * per;
            let hi = lo + out.len();
            s.spawn(move || fused_slices(&a[lo .. hi], &b[lo .. hi], &c[lo .. hi], out));
        }
    });
}

#[inline(never)]
fn par_pass_per_stage(k: &mut Cols, w: usize) {
    let n = k.out.len();
    let per = n.div_ceil(w);
    let (a, b, c) = (&k.a[..], &k.b[..], &k.c[..]);
    std::thread::scope(|s| {
        let parts =
            k.t1.chunks_mut(per)
                .zip(k.t2.chunks_mut(per))
                .zip(k.t3.chunks_mut(per))
                .zip(k.out.chunks_mut(per))
                .enumerate();
        for (i, (((t1, t2), t3), out)) in parts {
            let lo = i * per;
            let hi = lo + out.len();
            s.spawn(move || {
                stage1(&a[lo .. hi], &b[lo .. hi], t1);
                stage2(t1, t2);
                stage3(t2, &c[lo .. hi], t3);
                stage4(t3, &a[lo .. hi], out);
            });
        }
    });
}

// ---- driver ---------------------------------------------------------------

fn checksum(out: &[u32]) -> u64 {
    let mut h = 0xCBF2_9CE4_8422_2325u64;
    for &v in out {
        h = (h ^ v as u64).wrapping_mul(0x0000_0100_0000_01B3);
    }
    h
}

/// Time `f` over the columns: `SAMPLES` samples, each of enough calls to pass
/// about 40 ms, returning the lowest and the median nanoseconds per record.
fn time(k: &mut Cols, n: usize, mut f: impl FnMut(&mut Cols)) -> (f64, f64) {
    const SAMPLES: usize = 9;
    f(k);
    let t = Instant::now();
    f(k);
    let once = t.elapsed().as_secs_f64().max(1e-9);
    let reps = ((0.04 / once).ceil() as usize).clamp(1, 1 << 20);
    let mut v = Vec::with_capacity(SAMPLES);
    for _ in 0 .. SAMPLES {
        let t = Instant::now();
        for _ in 0 .. reps {
            f(black_box(k));
        }
        v.push(t.elapsed().as_secs_f64() * 1e9 / (reps as f64 * n as f64));
    }
    v.sort_by(|x, y| x.partial_cmp(y).unwrap());
    (v[0], v[SAMPLES / 2])
}

fn main() {
    let sizes: Vec<usize> = std::env::args()
        .skip(1)
        .map(|s| s.parse().expect("a record count"))
        .collect();
    let sizes = if sizes.is_empty() {
        vec![1 << 12, 1 << 16, 1 << 20, 1 << 24, 1 << 26]
    } else {
        sizes
    };
    let table: [Stage; 4] = [st1, st2, st3, st4];
    let per_record: PerRecord = black_box(whole);
    let workers = std::thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(1)
        .min(4);

    println!(
        "| records | arm | lowest ns/record | median ns/record | median against fused_slices |"
    );
    println!("| --: | --- | --: | --: | --: |");
    for &n in &sizes {
        let mut k = Cols::new(n);
        let mut scratch: [Vec<u32>; 3] = [vec![0; 65536], vec![0; 65536], vec![0; 65536]];

        let mut rows: Vec<(String, f64, f64)> = Vec::new();
        let mut want = None;
        let mut run = |name: &str, k: &mut Cols, f: &mut dyn FnMut(&mut Cols)| {
            k.out.fill(1);
            f(k);
            let got = checksum(&k.out);
            match want {
                None => want = Some(got),
                Some(w) => assert_eq!(w, got, "arm {name} disagrees at n = {n}"),
            }
            let (lo, med) = time(k, n, &mut *f);
            rows.push((name.to_string(), lo, med));
        };

        run("fused_slices", &mut k, &mut |k| {
            fused_slices(&k.a, &k.b, &k.c, &mut k.out)
        });
        run("fused_rawptr", &mut k, &mut |k| {
            let p = RawCols {
                a:   k.a.as_ptr(),
                b:   k.b.as_ptr(),
                c:   k.c.as_ptr(),
                out: k.out.as_mut_ptr(),
            };
            fused_rawptr(black_box(&p), k.out.len())
        });
        run("iter_chain", &mut k, &mut |k| {
            iter_chain(&k.a, &k.b, &k.c, &mut k.out)
        });
        #[cfg(target_arch = "x86_64")]
        if std::is_x86_feature_detected!("avx2") {
            // SAFETY: the feature was just detected.
            run("hand_avx2", &mut k, &mut |k| unsafe {
                hand_avx2(&k.a, &k.b, &k.c, &mut k.out)
            });
        }
        run("pass_per_stage", &mut k, &mut |k| pass_per_stage(k));
        for m in [256usize, 4096, 65536] {
            run(&format!("morsel_columns_{m}"), &mut k, &mut |k| {
                morsel_columns(k, m)
            });
        }
        for m in [256usize, 4096, 65536] {
            run(&format!("morsel_scratch_{m}"), &mut k, &mut |k| {
                morsel_scratch(k, m, &mut scratch)
            });
        }
        for m in [256usize, 4096] {
            run(&format!("morsel_fnptr_{m}"), &mut k, &mut |k| {
                morsel_fnptr(k, m, black_box(&table))
            });
        }
        run("call_per_record", &mut k, &mut |k| {
            call_per_record(&k.a, &k.b, &k.c, &mut k.out, per_record)
        });
        if n >= 1 << 16 {
            run(&format!("par{workers}_fused"), &mut k, &mut |k| {
                par_fused(k, workers)
            });
            run(&format!("par{workers}_pass_per_stage"), &mut k, &mut |k| {
                par_pass_per_stage(k, workers)
            });
        }

        let base = rows[0].2;
        for (name, lo, med) in &rows {
            println!("| {n} | {name} | {lo:.3} | {med:.3} | {:.2}x |", med / base);
        }
    }
}
