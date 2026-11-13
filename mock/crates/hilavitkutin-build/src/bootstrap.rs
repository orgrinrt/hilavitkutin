//! What a build script prints and writes, as text.
//!
//! The consumer's `build.rs` owns everything that touches the host: it
//! reads cargo's environment, prints the directives and writes the file.
//! This module turns a [`BuildConfig`] into the text of both, through
//! `core::fmt::Write`, so the crate itself stays `no_std`.
//!
//! A crate's whole `build.rs`:
//!
//! ```no_run
//! fn main() {
//!     let mut out = String::new();
//!     hilavitkutin_build::configure(
//!         |key| std::env::var(key).ok().into(),
//!         &mut out,
//!     )
//!     .expect("a String takes every write");
//!     print!("{out}");
//! }
//! ```
//!
//! The directives set the crate's rustc flags in
//! `HILAVITKUTIN_BUILD_RUSTFLAGS`, which the committed workspace wrapper
//! appends. The config file is a placeholder read by nothing yet; its
//! real schema is deferred to a follow-up round (see `BACKLOG.md.tmpl`).

use core::fmt::Write;

use notko::Maybe;

use crate::config::BuildConfig;
use crate::flags::write_rustflags;
use crate::pragma::PragmaSet;

/// The variable a crate's build script sets on its own rustc, and the
/// wrapper appends to rustc's arguments. A macro, so the one name is
/// spliced into both texts at compile time.
macro_rules! rustflags_var {
    () => {
        "HILAVITKUTIN_BUILD_RUSTFLAGS"
    };
}

/// Write the `cargo::` directives the build script prints, one per line.
///
/// The script reruns only when it changes, since what it writes depends
/// on nothing else in the crate. The `arvo_fast_math` cfg is declared
/// always and set when the `FastMath` pragma is active. The crate's rustc
/// flags go to `HILAVITKUTIN_BUILD_RUSTFLAGS` on its own rustc, empty when
/// no pragma writes one.
pub fn write_directives<W: Write>(cfg: &BuildConfig, out: &mut W) -> core::fmt::Result {
    out.write_str("cargo::rerun-if-changed=build.rs\n")?;
    out.write_str("cargo::rustc-check-cfg=cfg(arvo_fast_math)\n")?;
    if cfg.fast_math() {
        out.write_str("cargo::rustc-cfg=arvo_fast_math\n")?;
    }
    out.write_str(concat!("cargo::rustc-env=", rustflags_var!(), "="))?;
    write_rustflags(cfg.pragmas, out)?;
    out.write_char('\n')
}

/// Write the text of the rustc workspace wrapper.
///
/// A consumer commits it at `.cargo/hilavitkutin-rustc-wrapper`, names it
/// as `build.rustc-workspace-wrapper` in `.cargo/config.toml`, and checks
/// its copy against this text. Where `HILAVITKUTIN_BUILD_RUSTFLAGS` is
/// unset the wrapper is rustc unchanged.
pub fn write_wrapper<W: Write>(out: &mut W) -> core::fmt::Result {
    out.write_str(concat!(
        "#!/bin/sh\n",
        "# Written by hilavitkutin-build: appends the flags a crate's build script chose for it.\n",
        "exec \"$@\" $",
        rustflags_var!(),
        "\n",
    ))
}

/// What a build script reads one environment variable through.
///
/// The build script owns the environment; it passes a closure such as
/// `|key| std::env::var(key).ok().into()`.
pub trait Env<S: AsRef<str>>: FnMut(&'static str) -> Maybe<S> {} // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72

impl<S: AsRef<str>, F: FnMut(&'static str) -> Maybe<S>> Env<S> for F {} // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72

/// Resolve the build from cargo's environment and write the directives,
/// with the profile's default pragmas.
pub fn configure<S: AsRef<str>, W: Write>(env: impl Env<S>, out: &mut W) -> core::fmt::Result {
    // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    configure_with(env, |pragmas| pragmas, out)
}

/// Resolve the build from cargo's environment, let `tune` change the
/// profile's default pragmas, and write the directives.
///
/// A crate turns off what does not hold for it here: a deterministic
/// simulation crate passes `|p| p.without(Pragma::FastMath)`.
pub fn configure_with<S: AsRef<str>, W: Write>(
    // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    mut env: impl Env<S>,
    tune: impl FnOnce(PragmaSet) -> PragmaSet,
    out: &mut W,
) -> core::fmt::Result {
    let profile = env("PROFILE");
    let features = env("CARGO_CFG_TARGET_FEATURE");
    let mut cfg = BuildConfig::from_cargo(borrowed(&profile), borrowed(&features));
    cfg.pragmas = tune(cfg.pragmas);
    write_directives(&cfg, out)
}

/// Borrow the text out of a value the build script read.
fn borrowed<S: AsRef<str>>(value: &Maybe<S>) -> Maybe<&str> {
    // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    match value {
        Maybe::Is(s) => Maybe::Is(s.as_ref()),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// Write the body of the generated config file.
///
/// The build script chooses where it goes; `$OUT_DIR/hilavitkutin-config.toml`
/// is the conventional place.
// FIXME: placeholder schema (profile and fast_math only); the wrapper-generation round defines the real one and its reader.
pub fn write_config_file<W: Write>(cfg: &BuildConfig, out: &mut W) -> core::fmt::Result {
    out.write_str(
        "# generated by hilavitkutin-build (skeleton placeholder)\n\
         # real schema lands with the wrapper-generation round\n",
    )?;
    writeln!(out, "profile = {:?}", cfg.profile)?;
    writeln!(out, "fast_math = {}", cfg.fast_math())
}
