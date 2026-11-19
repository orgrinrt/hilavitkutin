//! What a build script prints, as text.
//!
//! The consumer's `build.rs` owns everything that touches the host: it
//! reads cargo's environment and prints the directives. This module
//! turns a resolved build and the set of its profile into that text,
//! through `core::fmt::Write`, so the crate itself stays `no_std`.
//!
//! A crate's whole build script, at whatever path the crate keeps it:
//!
//! ```no_run
//! fn main() {
//!     hilavitkutin_build::script!(Default);
//! }
//! ```
//!
//! The directives set the crate's rustc flags in
//! `HILAVITKUTIN_BUILD_RUSTFLAGS`, which the workspace wrapper appends.

use core::fmt::Write;

use notko::Maybe;

use crate::config::BuildConfig;
use crate::flags::{write_rustflags, writes_flags};
use crate::pragma::Pragma;
use crate::profile::Profile;
use crate::prototype::{Default, Prototype};
use crate::set::{Set, Site, Visit, checked, crate_reach};
use crate::target::Target;
use crate::tools::Source;
use crate::wiring::{Flags, WRAPPER_NAME, names_the_wrapper};

/// The variable a crate's build script sets on its own rustc, and the
/// wrapper appends to rustc's arguments.
const RUSTFLAGS_VAR: &str = "HILAVITKUTIN_BUILD_RUSTFLAGS"; // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra

/// A writer that starts every line with the name of a directive.
struct Directive<'a, W: Write> {
    out:   &'a mut W,
    name:  &'static str,
    /// Whether the next text starts a line.
    start: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state, the crate takes no arvo dependency; tracked: #72
}

impl<'a, W: Write> Directive<'a, W> {
    fn of(name: &'static str, out: &'a mut W) -> Self {
        Directive {
            out,
            name,
            start: true,
        }
    }
}

impl<W: Write> Write for Directive<'_, W> {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn write_str(&mut self, text: &str) -> core::fmt::Result { // lint:allow(no-bare-string) reason: the signature of `core::fmt::Write`; tracked: #72
        for part in text.split_inclusive('\n') {
            if self.start {
                self.out.write_str(self.name)?;
            }
            self.out.write_str(part)?;
            self.start = part.ends_with('\n');
        }
        Ok(())
    }
}

/// The walk that writes every member's cfgs.
struct Cfgs<'a, W: Write> {
    out: Directive<'a, W>,
}

impl<W: Write> Visit for Cfgs<'_, W> {
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result {
        P::write_cfgs(&mut self.out)
    }
}

/// The walk that writes an error for every member the build cannot carry.
struct Refusals<'a, 's, W: Write> {
    target: &'a Target,
    site:   &'a Site<'s>,
    out:    &'a mut W,
}

impl<W: Write> Visit for Refusals<'_, '_, W> {
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result {
        let name = P::NAME;
        if let Maybe::Is(why) = P::admitted(self.target) {
            writeln!(
                self.out,
                "cargo::error=hilavitkutin-build: {name} {why}: name a prototype whose set for this profile \
                 leaves {name} out on this target"
            )?;
        }
        if let Maybe::Is(tool) = P::TOOL
            && matches!(tool.source, Source::LocalBuild(_))
        {
            let path = self.site.path_of(tool);
            let tool = tool.name;
            match path {
                Maybe::Isnt => {
                    writeln!(
                        self.out,
                        "cargo::error=hilavitkutin-build: {name} was given no path for {tool}, which only \
                         `script!` finds or builds: name {name} in a script that is `script!`"
                    )?;
                },
                // The wrapper splits its flags on spaces, so a path with
                // one would reach rustc as two arguments.
                Maybe::Is(path) if path.contains(char::is_whitespace) => {
                    writeln!(
                        self.out,
                        "cargo::error=hilavitkutin-build: {name} was given `{path}` for {tool}, and the wrapper \
                         cannot carry a path with a space: set HILAVITKUTIN_BUILD_TOOLS_HOME to a directory \
                         without one"
                    )?;
                },
                Maybe::Is(_) => {},
            }
        }
        Ok(())
    }
}

/// Write the `cargo::` directives the build script prints for the set
/// `S`, one per line.
///
/// The script reruns when cargo names another workspace wrapper, and
/// names no file: a script is kept at whatever path its project chose,
/// and a path that is not there has cargo rerun it on every build. The
/// `arvo_fast_math` cfg is declared always, and every cfg a pragma of the
/// set writes is set. The crate's rustc flags go to
/// `HILAVITKUTIN_BUILD_RUSTFLAGS` on its own rustc, empty when no pragma
/// writes one, a linker's flags among them. After them comes one
/// `cargo::error` per pragma the target does not admit and per pragma
/// whose locally built tool the site has no path for, which fails the
/// build.
pub fn write_directives<S: Set, W: Write>(
    cfg: &BuildConfig,
    site: &Site<'_>,
    out: &mut W,
) -> core::fmt::Result {
    out.write_str("cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER\n")?;
    out.write_str("cargo::rustc-check-cfg=cfg(arvo_fast_math)\n")?;
    S::each(&mut Cfgs {
        out: Directive::of("cargo::rustc-cfg=", out),
    })?;
    write!(out, "cargo::rustc-env={RUSTFLAGS_VAR}=")?;
    write_rustflags::<S, W>(site, out)?;
    out.write_char('\n')?;
    S::each(&mut Refusals {
        target: &cfg.target,
        site,
        out,
    })
}

/// Write the text of the rustc workspace wrapper.
///
/// `script!` keeps the file at the workspace's
/// `.cargo/hilavitkutin-rustc-wrapper` to this text, and the workspace's
/// `.cargo/config.toml` names it as `build.rustc-workspace-wrapper`.
/// Where `HILAVITKUTIN_BUILD_RUSTFLAGS` is unset the wrapper is rustc
/// unchanged.
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

/// Resolve the build from cargo's environment and write the directives
/// of the [`Default`] prototype.
pub fn configure<S: AsRef<str>, W: Write>(env: impl Env<S>, out: &mut W) -> core::fmt::Result {
    configure_as::<Default, S, W>(env, out)
}

/// Resolve the build from cargo's environment and write the directives
/// of the prototype `P`, for a script written by hand.
///
/// The prototype's five sets are held to their requirements here, and to
/// crate reach: a script written by hand claims no workspace. A pragma
/// whose tool is built on the machine is refused in the directives, since
/// only `script!` finds or builds one.
///
/// Where the crate has flags to hand over and cargo names no workspace
/// wrapper whose file name is `hilavitkutin-rustc-wrapper`
/// (`RUSTC_WORKSPACE_WRAPPER`, which cargo sets for a build script only
/// where its configuration names one; `cargo clippy` names
/// `clippy-driver`, and a check loses nothing without the flags, which
/// only steer code generation), a `cargo::warning` says the flags are
/// dropped.
///
/// A crate whose own code must not take fast float math names
/// [`Deterministic`](crate::Deterministic), so the `arvo_fast_math` cfg is
/// never set for it. The cfg reaches that crate alone and never arvo,
/// whose `Float` is resolved in arvo's own build.
pub fn configure_as<P: Prototype, S: AsRef<str>, W: Write>(
    mut env: impl Env<S>,
    out: &mut W,
) -> core::fmt::Result {
    const {
        checked::<P::Dev>();
        crate_reach::<P::Dev>();
        checked::<P::DevOpt>();
        crate_reach::<P::DevOpt>();
        checked::<P::Release>();
        crate_reach::<P::Release>();
        checked::<P::Profiling>();
        crate_reach::<P::Profiling>();
        checked::<P::Ci>();
        crate_reach::<P::Ci>();
    };
    let cfg = BuildConfig::from_env::<P, S>(&mut env);
    let triple = env("TARGET");
    let triple = match borrowed(&triple) {
        Maybe::Is(triple) => triple,
        Maybe::Isnt => "",
    };
    let site = Site::new(cfg.profile, triple, &[]);
    let flags = match cfg.profile {
        Profile::Dev => {
            write_directives::<P::Dev, W>(&cfg, &site, out)?;
            writes_flags::<P::Dev>(&site)
        },
        Profile::DevOpt => {
            write_directives::<P::DevOpt, W>(&cfg, &site, out)?;
            writes_flags::<P::DevOpt>(&site)
        },
        Profile::Release => {
            write_directives::<P::Release, W>(&cfg, &site, out)?;
            writes_flags::<P::Release>(&site)
        },
        Profile::Profiling => {
            write_directives::<P::Profiling, W>(&cfg, &site, out)?;
            writes_flags::<P::Profiling>(&site)
        },
        Profile::Ci => {
            write_directives::<P::Ci, W>(&cfg, &site, out)?;
            writes_flags::<P::Ci>(&site)
        },
    };
    let wrapper = env("RUSTC_WORKSPACE_WRAPPER");
    if !names_the_wrapper(borrowed(&wrapper)) && matches!(flags, Flags::Written) {
        writeln!(
            out,
            "cargo::warning=hilavitkutin-build chose rustc flags for this crate and cargo names no workspace wrapper \
             to append them, so they are dropped: run cargo where `.cargo/config.toml` names \
             `.cargo/{WRAPPER_NAME}`"
        )?;
    }
    Ok(())
}

/// Borrow the text out of a value the build script read.
#[rustfmt::skip] // keeps the allow on the signature it governs
pub(crate) fn borrowed<S: AsRef<str>>(value: &Maybe<S>) -> Maybe<&str> { // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    match value {
        Maybe::Is(s) => Maybe::Is(s.as_ref()),
        Maybe::Isnt => Maybe::Isnt,
    }
}
