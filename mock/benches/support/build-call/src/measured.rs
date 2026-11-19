//! What earlier runs measured, and a reading carried to a point no run has
//! measured yet.
//!
//! Two tables: the first timed call of each cell, from which the bound on a
//! call is set, and the largest target directory a cell left, from which the
//! disk budget of a cell is set. Each row names the run it was read from and
//! is replaced when a run measures its key at a new point. Both are read
//! through [`scaled`], so a point between two measured ones is never guessed
//! two ways.

/// A table row: a key, the point it was measured at, the reading.
pub type Row = (u16, usize, u64);

/// The key of a cell's first call: its scenario and profile codes.
pub const fn call_key(scenario: u8, profile: u8) -> u16 {
    ((scenario as u16) << 8) | profile as u16
}

/// The key of a cell's disk use: whether it builds incrementally.
pub const fn disk_key(incremental: bool) -> u16 {
    incremental as u16
}

/// The first timed call measured at a cell, in whole seconds rounded up, the
/// slower of the bench's two arms.
pub const FIRST_CALL_S: [Row; 11] = [
    // build-static, run 20261116T063249Z-f04daa0a-dirty, first warm pass
    (call_key(0, 0), 25, 17),
    (call_key(1, 0), 25, 6),
    (call_key(2, 0), 25, 6),
    (call_key(3, 0), 25, 15),
    // build-static at 97, first warm pass: the clean sweep from run
    // 20261119T171604Z-fe400621 (123.17 s on, 118.47 s off) and the edit
    // sweeps from run 20261119T203124Z-4228372a (leaf 5.95 s off, mid 33.47 s
    // off, root 87.31 s off)
    (call_key(0, 0), 97, 124),
    (call_key(1, 0), 97, 6),
    (call_key(2, 0), 97, 34),
    (call_key(3, 0), 97, 88),
    // link-static, run 20261116T083526Z-b2bbf4ba, first warm pass: the
    // relink, 0.157 s at dev 25 and 0.138 s at release 25.
    (call_key(4, 0), 25, 1),
    (call_key(4, 1), 25, 1),
    // link-static at dev 97 is the exception: the relink reads 0.5 s, but the
    // probe of the harness gives a cell ten times its bound counted from the
    // whole first call, whose untimed setup builds the 97 crates (118.47 s,
    // the non-incremental clean build of run 20261119T171604Z-fe400621), so a
    // row of 1 s skipped the cell twice, at 30 s and at 120 s. The row is the
    // build plus the relink, rounded up, until the ticket
    // `the_bench_harness_probe_counts_the_untimed_setup_of_a_build` lets the
    // probe leave the setup out.
    (call_key(4, 0), 97, 119),
];

/// The largest target directory a cell left, in bytes.
pub const TARGET_BYTES: [Row; 4] = [
    // build-static, run 20261116T063249Z-f04daa0a-dirty: incremental on after
    // a root edit, and off after a leaf edit
    (disk_key(true), 25, 562_731_093),
    (disk_key(false), 25, 82_479_237),
    // build-static at 97, run 20261119T203124Z-4228372a: incremental on after
    // a root edit, and off after a leaf edit
    (disk_key(true), 97, 3_893_504_668),
    (disk_key(false), 97, 567_452_989),
];

/// The reading of `key` at `n`: the row of `key` at the largest point at or
/// below `n`, scaled by `n` over that point and rounded up. `None` where
/// `key` has no row at or below `n`.
pub const fn scaled(rows: &[Row], key: u16, n: usize) -> Option<u64> {
    let mut best: Option<(usize, u64)> = None;
    let mut i = 0;
    while i < rows.len() {
        let (k, at, value) = rows[i];
        if k == key && at <= n {
            best = match best {
                Some((b, _)) if b >= at => best,
                _ => Some((at, value)),
            };
        }
        i += 1;
    }
    match best {
        Some((at, value)) => Some((value * n as u64).div_ceil(at as u64)),
        None => None,
    }
}
