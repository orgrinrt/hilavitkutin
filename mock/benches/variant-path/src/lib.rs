//! What a resolved variant path has to look like, and the mistake both
//! bench drivers made with it.
//!
//! `BenchManifest::for_size` resolves a bare-stem variant entry to a full
//! platform dylib path, appending the prefix and suffix itself. Both
//! drivers in this tree mapped a shaping of their own over the result, so
//! every entry came out as `liblibema_neon.so.so`, which no build
//! produces. The bench then died at load with a missing file rather than
//! at config time, where it would have said something useful.
//!
//! The check and the historical shaping live here together on purpose. A
//! check with no example of what it is meant to catch tends to drift into
//! accepting everything, and a driver testing against a name it made up
//! itself proves nothing about the paths it actually hands the harness.

use std::path::{Path, PathBuf};

/// Accept a dylib file name carrying the platform prefix and suffix
/// exactly once, and refuse one carrying either twice.
///
/// The name is the file name alone, not the full path. On a platform with
/// an empty prefix the prefix half of the check is skipped, since every
/// name trivially starts with the empty string.
pub fn shaped_once(name: &str) -> Result<(), String> {
    let prefix = std::env::consts::DLL_PREFIX;
    let suffix = std::env::consts::DLL_SUFFIX;
    let inner = name
        .strip_prefix(prefix)
        .ok_or_else(|| format!("`{name}` lacks the `{prefix}` prefix"))?
        .strip_suffix(suffix)
        .ok_or_else(|| format!("`{name}` lacks the `{suffix}` suffix"))?;
    if inner.contains(suffix) {
        return Err(format!("`{name}` carries `{suffix}` twice"));
    }
    if !prefix.is_empty() && inner.starts_with(prefix) {
        return Err(format!("`{name}` carries `{prefix}` twice"));
    }
    Ok(())
}

/// The shaping both drivers applied on top of `for_size` until df8a9419.
///
/// Reproduced exactly, `file_name` and all, so a test can feed a driver's
/// real resolved paths through it and watch [`shaped_once`] refuse them.
/// Nothing in a driver calls this; it exists so the check has a true
/// negative to answer rather than a string the test invented.
pub fn shaped_as_the_drivers_once_did(p: &Path) -> PathBuf {
    let parent = p.parent().map(Path::to_path_buf).unwrap_or_default();
    let stem = p.file_name().and_then(|s| s.to_str()).unwrap_or("");
    parent.join(format!(
        "{}{}{}",
        std::env::consts::DLL_PREFIX,
        stem,
        std::env::consts::DLL_SUFFIX
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The check refuses what the drivers used to produce.
    ///
    /// This is the pair that gives the drivers' own tests their meaning:
    /// a name shaped once is accepted, the same name put through the old
    /// shaping is refused, and the refusal says which affix doubled.
    #[test]
    fn the_check_refuses_the_shaping_the_drivers_once_did() {
        let prefix = std::env::consts::DLL_PREFIX;
        let suffix = std::env::consts::DLL_SUFFIX;
        let resolved = PathBuf::from(format!("variants/ema_neon/{prefix}ema_neon{suffix}"));

        let name = resolved.file_name().unwrap().to_str().unwrap();
        shaped_once(name).expect("a path the harness resolved is accepted");

        let doubled = shaped_as_the_drivers_once_did(&resolved);
        let doubled_name = doubled.file_name().unwrap().to_str().unwrap();
        let err = shaped_once(doubled_name).expect_err("the old shaping is refused");
        assert!(
            err.contains("twice"),
            "the refusal should say an affix doubled, got `{err}`"
        );
    }

    /// A name missing the affixes altogether is refused too, and for its
    /// own reason rather than the doubling one.
    #[test]
    fn a_bare_stem_is_refused_as_missing_rather_than_doubled() {
        let err = shaped_once("ema_neon").expect_err("a bare stem is not a dylib name");
        assert!(
            err.contains("lacks"),
            "a bare stem lacks an affix rather than doubling one, got `{err}`"
        );
    }
}
