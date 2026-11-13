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
use crate::flags::{write_rustflags, writes_flags};
use crate::pragma::PragmaSet;

/// The variable a crate's build script sets on its own rustc, and the
/// wrapper appends to rustc's arguments.
const RUSTFLAGS_VAR: &str = "HILAVITKUTIN_BUILD_RUSTFLAGS"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

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
    write!(out, "cargo::rustc-env={RUSTFLAGS_VAR}=")?;
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
    write!(
        out,
        "#!/bin/sh\n\
         # Written by hilavitkutin-build: appends the flags a crate's build script chose for it.\n\
         set -f\n\
         exec \"$@\" ${RUSTFLAGS_VAR}\n"
    )
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
    configure_with(env, |pragmas| pragmas, out)
}

/// Resolve the build from cargo's environment, let `tune` change the
/// profile's default pragmas, and write the directives.
///
/// The tuned set is made coherent, so a pragma whose required pragma
/// was taken out goes with it. Where the crate has flags to hand over
/// and cargo names no workspace wrapper whose path ends with
/// `hilavitkutin-rustc-wrapper` (`RUSTC_WORKSPACE_WRAPPER`, which cargo
/// sets for a build script only where its configuration names one;
/// `cargo clippy` names `clippy-driver`), a `cargo::warning` says the
/// flags are dropped.
///
/// A crate whose own code must not take fast float math passes
/// `|p| p.without(Pragma::FastMath)`, so the `arvo_fast_math` cfg is
/// never set for it. The cfg reaches that crate alone and never arvo,
/// whose `Float` is resolved in arvo's own build.
pub fn configure_with<S: AsRef<str>, W: Write>(
    mut env: impl Env<S>,
    tune: impl FnOnce(PragmaSet) -> PragmaSet,
    out: &mut W,
) -> core::fmt::Result {
    let profile = env("PROFILE");
    let features = env("CARGO_CFG_TARGET_FEATURE");
    let mut cfg = BuildConfig::from_cargo(borrowed(&profile), borrowed(&features));
    cfg.pragmas = tune(cfg.pragmas).coherent();
    write_directives(&cfg, out)?;
    let wrapped = match env("RUSTC_WORKSPACE_WRAPPER") {
        Maybe::Is(path) => path.as_ref().ends_with("hilavitkutin-rustc-wrapper"),
        Maybe::Isnt => false,
    };
    if !wrapped && writes_flags(cfg.pragmas) {
        out.write_str(
            "cargo::warning=hilavitkutin-build chose rustc flags for this crate and cargo names no workspace wrapper \
             to append them, so they are dropped: run cargo where `.cargo/config.toml` names \
             `.cargo/hilavitkutin-rustc-wrapper`\n",
        )?;
    }
    Ok(())
}

/// Borrow the text out of a value the build script read.
fn borrowed<S: AsRef<str>>(value: &Maybe<S>) -> Maybe<&str> {
    match value {
        Maybe::Is(s) => Maybe::Is(s.as_ref()),
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// Write the body of the generated config file.
///
/// The build script chooses where it goes; `$OUT_DIR/hilavitkutin-config.toml`
/// is the conventional place.
// FIXME(hilavitkutin_build_placeholder_schema_profile_and_fast_math): placeholder schema (profile and fast_math only); the wrapper-generation round defines the real one and its reader.
pub fn write_config_file<W: Write>(cfg: &BuildConfig, out: &mut W) -> core::fmt::Result {
    out.write_str(
        "# generated by hilavitkutin-build (skeleton placeholder)\n\
         # real schema lands with the wrapper-generation round\n",
    )?;
    writeln!(out, "profile = {:?}", cfg.profile)?;
    writeln!(out, "fast_math = {}", cfg.fast_math())
}
