//! The disk a cell may use, and the refusal of a call whose disk cannot hold
//! it.
//!
//! The plan (section 8) has the hooks library read the free space before a
//! group of cells starts and refuse the group where it is under the group's
//! budget plus 1 GB. The harness runs no hook before a cell, so the refusal is
//! made in the untimed setup of every call instead, where the cell's own
//! target directory is counted as free, since the call reuses or removes it.
//!
//! FIXME(the_bench_harness_has_no_hook_before_a_cell_so_a_refused_cel): a
//! refusal panics in the setup and so fails the cell; the plan records it as
//! absent with its line, which needs a hook before the cell.

use std::path::Path;
use std::process::Command;

use crate::measured::{TARGET_BYTES, disk_key, scaled};

/// The margin the plan keeps beside a cell's budget.
pub const MARGIN: u64 = 1 << 30;

/// The disk budget of a cell at `n` crates: the largest target directory a
/// cell of the same incremental setting left ([`TARGET_BYTES`]), carried to
/// `n` by [`scaled`]. A point below every measured one takes the smallest
/// measured reading, since no fixture is smaller than the small point.
pub fn budget(n: usize, incremental: bool) -> u64 {
    let key = disk_key(incremental);
    scaled(&TARGET_BYTES, key, n).unwrap_or_else(|| {
        TARGET_BYTES
            .iter()
            .filter(|(k, ..)| *k == key)
            .map(|(.., v)| *v)
            .min()
            .unwrap_or(0)
    })
}

/// Why a call is refused, or `None` where `free` bytes on the disk, with the
/// `held` bytes the cell's own target directory already takes, hold `budget`
/// and the margin.
pub fn refusal(free: u64, held: u64, budget: u64) -> Option<String> {
    let have = free.saturating_add(held);
    let need = budget.saturating_add(MARGIN);
    (have < need).then(|| {
        format!(
            "{free} bytes free and {held} held by the cell, under its budget of {budget} bytes and the margin of \
             {MARGIN}; refused before anything was built"
        )
    })
}

/// The bytes free to an unprivileged writer on the filesystem holding `path`,
/// read from POSIX `df -Pk`.
///
/// FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): a host
/// with no POSIX `df` cannot read its free space, and its calls are refused.
pub fn free_bytes(path: &Path) -> Result<u64, String> {
    let out = Command::new("df")
        .arg("-Pk")
        .arg(path)
        .output()
        .map_err(|e| format!("df: {e}"))?;
    let text = String::from_utf8_lossy(&out.stdout);
    if !out.status.success() {
        return Err(format!(
            "df -Pk {}: {}",
            path.display(),
            String::from_utf8_lossy(&out.stderr).trim()
        ));
    }
    free_of_df(&text).ok_or_else(|| format!("df -Pk {}: no free space in {text:?}", path.display()))
}

/// The available bytes in the output of `df -Pk`: the fourth field of its
/// second line, in units of 1024 bytes.
pub fn free_of_df(text: &str) -> Option<u64> {
    let kib: u64 = text
        .lines()
        .nth(1)?
        .split_whitespace()
        .nth(3)?
        .parse()
        .ok()?;
    kib.checked_mul(1024)
}
