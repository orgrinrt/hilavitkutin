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
/// slower of the bench's two arms, with one row that is not a timed call
/// (the last, a placeholder). `tests/measured.rs` reads each row back from the
/// run directory it cites.
pub const FIRST_CALL_S: [Row; 11] = [
    // build-static at 25, run 20261119T150158Z-ffb57e5a, first warm pass:
    // clean 22.28 s on, leaf 5.80 s off, mid 6.14 s off, root 16.83 s off
    (call_key(0, 0), 25, 23),
    (call_key(1, 0), 25, 6),
    (call_key(2, 0), 25, 7),
    (call_key(3, 0), 25, 17),
    // build-static at 97, first warm pass: the clean sweep from run
    // 20261119T171604Z-fe400621 (123.17 s on, 118.47 s off) and the edit
    // sweeps from run 20261119T203124Z-4228372a (leaf 5.95 s off, mid 33.47 s
    // off, root 87.31 s off)
    (call_key(0, 0), 97, 124),
    (call_key(1, 0), 97, 6),
    (call_key(2, 0), 97, 34),
    (call_key(3, 0), 97, 88),
    // link-static at 25, run 20261119T134226Z-07a9e9a3, first warm pass: the
    // relink, under one second at dev and at release
    (call_key(4, 0), 25, 1),
    (call_key(4, 1), 25, 1),
    // FIXME(placeholders_tagged_the_bench_harness_probe_counts_the): a
    // placeholder, not a timed call. The relink at dev 97 reads 0.51 s in run
    // 20261119T222847Z-2eb71914, but the probe of the harness gives a cell ten
    // times its bound from the whole first call, whose untimed setup builds
    // the 97 crates, so a row of 1 s skipped the cell at 30 s and a bound
    // scaled from 25 skipped it at 120 s. This row is the non-incremental
    // clean build of run 20261119T171604Z-fe400621 (118.47 s) plus that
    // relink, rounded up, and it lets a bound of 357 s on a 0.5 s call stand
    // until the probe leaves the setup out.
    (call_key(4, 0), 97, 119),
];

/// The largest target directory a cell left, in bytes: with incremental on
/// after a root edit and off after a leaf edit, from the build-static runs
/// `20261119T150158Z-ffb57e5a` (25) and `20261119T203124Z-4228372a` (97) and
/// the clean sweep `20261119T171604Z-fe400621` (97).
pub const TARGET_BYTES: [Row; 4] = [
    (disk_key(true), 25, 562_731_084),
    (disk_key(false), 25, 82_479_236),
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
