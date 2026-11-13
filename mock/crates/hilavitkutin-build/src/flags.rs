//! The rustc flags each pragma owns, as the wrapper appends them.
//!
//! Every flag has exactly one owner pragma (DESIGN §Flag ownership by
//! pragma). Only the flags whose mechanism the pinned toolchain carries
//! and the build script needs nothing else for are written; a pragma
//! waiting on a plugin, a profile or a tool writes nothing.

use core::fmt::{Arguments, Write};

use crate::pragma::{Pragma, PragmaSet};

/// The built-in LLVM options LoopOptimization owns without its plugin.
const LOOP_OPTIMIZATION: [&str; 5] = [
    // lint:allow(no-bare-static-str) reason: a build-dependency only, it reaches no binary; tracked: no_bare_static_str_does_not_know_a_build_dependency_only_cra
    "-Cllvm-args=-enable-loop-distribute",
    "-Cllvm-args=-enable-loop-versioning-licm",
    "-Cllvm-args=-enable-interleaved-mem-accesses",
    "-Cllvm-args=-loop-prefetch-writes",
    "-Cllvm-args=-enable-histogram-loop-vectorization",
];

/// A space-separated flag list being written.
struct FlagList<'a, W: Write> {
    out:   &'a mut W,
    empty: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state, the crate takes no arvo dependency; tracked: #72
}

impl<W: Write> FlagList<'_, W> {
    /// Append one flag, a space before it unless it is the first.
    fn push(&mut self, flag: Arguments<'_>) -> core::fmt::Result {
        if !self.empty {
            self.out.write_char(' ')?;
        }
        self.empty = false;
        self.out.write_fmt(flag)
    }

    /// Append the flags `pragma` owns.
    fn pragma(&mut self, pragma: Pragma) -> core::fmt::Result {
        match pragma {
            Pragma::LoopOptimization => {
                // FIXME(hilavitkutin_build_build_scripts_find_the_plugins_and_profil): the `-Zllvm-plugins` flag for polka-passes.so waits on the build script finding the plugin.
                for flag in LOOP_OPTIMIZATION {
                    self.push(format_args!("{flag}"))?;
                }
                Ok(())
            },
            Pragma::ParallelCodegen(units) => self.push(format_args!("-Zthreads={units}")),
            Pragma::SharedGenerics => self.push(format_args!("-Zshare-generics=y")),
            // FastMath is a cfg, written as a directive. The rest wait on a
            // plugin, a profile, a tool or the consumer's Cargo profile.
            Pragma::FastMath
            | Pragma::Polly
            | Pragma::MathPeephole
            | Pragma::ExpandedLto
            | Pragma::Pgo
            | Pragma::Bolt
            | Pragma::Profiling
            | Pragma::BuildStd
            | Pragma::LoopFusion
            | Pragma::MimallocAllocator => Ok(()),
        }
    }
}

/// Write the rustc flags of every pragma in `pragmas`, separated by single
/// spaces, with none before the first and none after the last.
///
/// This is the value of `HILAVITKUTIN_BUILD_RUSTFLAGS`, which the wrapper
/// splits on spaces; no flag contains one.
pub fn write_rustflags<W: Write>(pragmas: PragmaSet, out: &mut W) -> core::fmt::Result {
    let mut list = FlagList {
        out,
        empty: true,
    };
    for pragma in pragmas.iter() {
        list.pragma(pragma)?;
    }
    Ok(())
}

/// A sink that keeps only whether anything was written to it.
struct Anything {
    wrote: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state, the crate takes no arvo dependency; tracked: #72
}

impl Write for Anything {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        // lint:allow(no-bare-string) reason: the core::fmt::Write contract; tracked: #72
        self.wrote |= !s.is_empty();
        Ok(())
    }
}

/// Whether any pragma in `pragmas` writes a rustc flag, so the crate has
/// something for the wrapper to hand over.
pub fn writes_flags(pragmas: PragmaSet) -> bool {
    // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: build-time predicate; tracked: #72
    let mut sink = Anything {
        wrote: false,
    };
    write_rustflags(pragmas, &mut sink).is_ok() && sink.wrote
}
