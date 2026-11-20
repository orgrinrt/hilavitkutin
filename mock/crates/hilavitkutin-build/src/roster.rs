//! The pragmas that steer optimisation: passes, profiles, codegen.
//!
//! A pragma here writes what the pinned toolchain carries and the build
//! script needs nothing else for. One that waits on a plugin, a profile
//! or a tool names the tool and writes nothing meanwhile.

use core::fmt::{self, Write};

use crate::choice::LtoGroup;
use crate::pragma::{All, Any, Free, Mask, Pragma, Reach, Slot, Words, Written};
use crate::tools::{LLVM_BOLT, MATH_PEEPHOLE, MIMALLOC, POLKA_PASSES, POLLY, PROFDATA, Tool};

/// The built-in LLVM options `LoopOptimization` owns without its plugin.
const LOOP_OPTIMIZATION: [&str; 5] = [
    "-Cllvm-args=-enable-loop-distribute",
    "-Cllvm-args=-enable-loop-versioning-licm",
    "-Cllvm-args=-enable-interleaved-mem-accesses",
    "-Cllvm-args=-loop-prefetch-writes",
    "-Cllvm-args=-enable-histogram-loop-vectorization",
];

/// Loop passes beyond the default pipeline.
#[derive(Debug, Clone, Copy)]
pub struct LoopOptimization;

impl Pragma for LoopOptimization {
    type Group = Free;
    type Requires = ();

    const BIT: Mask = Slot::LoopOptimization.bit();
    const NAME: Words = "LoopOptimization";
    const NEEDS: &'static [&'static Tool] = &[&POLKA_PASSES];

    // FIXME(placeholders_tagged_hilavitkutin_build_build_scripts_find): the `-Zllvm-plugins` flag for polka-passes.so waits on the build script finding the plugin.
    fn write_rustflags<W: Write>(_with: &Written<'_>, out: &mut W) -> fmt::Result {
        let [first, rest @ ..] = LOOP_OPTIMIZATION;
        out.write_str(first)?;
        for flag in rest {
            out.write_char(' ')?;
            out.write_str(flag)?;
        }
        Ok(())
    }
}

/// Fast float math in the configured crate's own code, by the
/// `arvo_fast_math` cfg. It reaches no other crate, arvo included.
#[derive(Debug, Clone, Copy)]
pub struct FastMath;

impl Pragma for FastMath {
    type Group = Free;
    type Requires = ();

    const BIT: Mask = Slot::FastMath.bit();
    const NAME: Words = "FastMath";

    fn write_cfgs<W: Write>(out: &mut W) -> fmt::Result {
        out.write_str("arvo_fast_math\n")
    }
}

/// Fat LTO with one codegen unit, for the whole workspace.
#[derive(Debug, Clone, Copy)]
pub struct ExpandedLto;

impl Pragma for ExpandedLto {
    type Group = LtoGroup;
    type Requires = ();

    const BIT: Mask = Slot::ExpandedLto.bit();
    const NAME: Words = "ExpandedLto";
    const REACH: Reach = Reach::Workspace;

    fn write_cargo_config<W: Write>(with: &Written<'_>, out: &mut W) -> fmt::Result {
        let profile = with.profile();
        writeln!(
            out,
            "profile.{profile}.lto = \"fat\"\nprofile.{profile}.codegen-units = 1"
        )
    }
}

/// Frontend threads, `-Zthreads=N`. Zero lets rustc choose. The pragma
/// has one bit whatever its count.
#[derive(Debug, Clone, Copy)]
pub struct ParallelCodegen<const N: u8>; // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a const parameter, a thread count in [0, 255]; tracked: #72

#[rustfmt::skip] // keeps the allow on the line it governs
impl<const N: u8> Pragma for ParallelCodegen<N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a const parameter, a thread count in [0, 255]; tracked: #72
    type Group = Free;
    type Requires = ();

    const BIT: Mask = Slot::ParallelCodegen.bit();
    const NAME: Words = "ParallelCodegen";

    fn write_rustflags<W: Write>(_with: &Written<'_>, out: &mut W) -> fmt::Result {
        write!(out, "-Zthreads={N}")
    }
}

/// Generic instances shared across crates, `-Zshare-generics=y`.
#[derive(Debug, Clone, Copy)]
pub struct SharedGenerics;

impl Pragma for SharedGenerics {
    type Group = Free;
    type Requires = ();

    const BIT: Mask = Slot::SharedGenerics.bit();
    const NAME: Words = "SharedGenerics";

    fn write_rustflags<W: Write>(_with: &Written<'_>, out: &mut W) -> fmt::Result {
        out.write_str("-Zshare-generics=y")
    }
}

/// A pragma that writes nothing yet: its name, its place, what it
/// requires of its set and the tools it waits on.
macro_rules! waiting {
    ($(#[$doc:meta])* $name:ident, $requires:ty, $unmet:expr, $tool:expr) => {
        $(#[$doc])*
        #[derive(Debug, Clone, Copy)]
        pub struct $name;

        impl Pragma for $name {
            type Group = Free;
            type Requires = $requires;

            const BIT: Mask = Slot::$name.bit();
            const NAME: Words = stringify!($name);
            const NEEDS: &'static [&'static Tool] = $tool;
            const UNMET: Words = $unmet;
        }
    };
}

waiting!(
    /// The polyhedral optimiser of a Polly-enabled LLVM.
    Polly,
    (),
    "",
    &[&POLLY]
);
waiting!(
    /// Float peephole rewrites. Stands on `FastMath`.
    MathPeephole,
    All<(FastMath,)>,
    "MathPeephole stands on FastMath, which the set does not hold",
    &[&MATH_PEEPHOLE]
);
waiting!(
    /// Profile-guided optimisation, from profiles found on disk.
    Pgo,
    (),
    "",
    &[&PROFDATA]
);
waiting!(
    /// Post-link binary rewriting.
    Bolt,
    (),
    "",
    &[&LLVM_BOLT]
);
waiting!(
    /// Profiling runs after the build. Stands on `Pgo` or `Bolt`.
    Profiling,
    Any<(Pgo, Bolt)>,
    "Profiling stands on Pgo or Bolt, and the set holds neither",
    &[&PROFDATA]
);
waiting!(
    /// std rebuilt from source with the build's flags.
    BuildStd,
    (),
    "",
    &[]
);
waiting!(
    /// Adjacent loops fused. Experimental.
    LoopFusion,
    (),
    "",
    &[&POLKA_PASSES]
);
waiting!(
    /// A recommendation of mimalloc; the consumer sets its allocator.
    MimallocAllocator,
    (),
    "",
    &[&MIMALLOC]
);
