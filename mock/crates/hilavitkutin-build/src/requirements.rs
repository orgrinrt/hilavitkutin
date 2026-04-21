//! External-toolchain requirements per pragma.
//!
//! Each `Pragma` may need external tooling (a Polly-enabled LLVM,
//! `profraw` -> `profdata` conversion, BOLT, an LLVM pass plugin,
//! or a mimalloc crate in Cargo.toml). `REQUIREMENTS` is a static
//! `&'static [PragmaRequirement]` table so runtime-detection code in
//! the wrapper-script round can consume it as data.
//!
//! Pragma-to-pragma ordering constraints (e.g. `Profiling` implies
//! `Any<(Pgo, Bolt)>`) are NOT modelled here — they belong to the
//! pragma-resolution stage (follow-up round).

use crate::pragma::Pragma;

/// External capability / artefact a pragma needs before it can do
/// anything useful.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Requirement {
    /// LLVM built with Polly support (distribution-specific).
    PollyEnabledLlvm,
    /// `.profraw` collection + `profdata` merge tooling.
    ProfrawProfdata,
    /// `llvm-bolt` binary in `$PATH`.
    LlvmBolt,
    /// An LLVM pass plugin `.so` (e.g. `polka-passes.so`,
    /// `math-peephole.so`) loaded via `-Z llvm-plugins`.
    LlvmPassPlugin,
    /// `mimalloc` crate declared in the consumer's `Cargo.toml` with
    /// `#[global_allocator]` wired up.
    MimallocCrate,
}

/// Static pragma -> requirements mapping.
#[derive(Debug)]
pub struct PragmaRequirement {
    pub pragma: Pragma,
    pub requires: &'static [Requirement],
}

/// The pragma -> requirements table. Every `Pragma` variant appears
/// exactly once; pragmas with no external requirement list an empty
/// slice.
///
/// `ParallelCodegen` is listed with `u8::MAX` as a sentinel — the
/// param is irrelevant for requirement lookup; callers should match
/// on the discriminant only.
pub const REQUIREMENTS: &[PragmaRequirement] = &[
    PragmaRequirement {
        pragma: Pragma::LoopOptimization,
        requires: &[Requirement::LlvmPassPlugin],
    },
    PragmaRequirement {
        pragma: Pragma::Polly,
        requires: &[Requirement::PollyEnabledLlvm],
    },
    PragmaRequirement {
        pragma: Pragma::MathPeephole,
        requires: &[Requirement::LlvmPassPlugin],
    },
    PragmaRequirement {
        pragma: Pragma::FastMath,
        requires: &[],
    },
    PragmaRequirement {
        pragma: Pragma::ExpandedLto,
        requires: &[],
    },
    PragmaRequirement {
        pragma: Pragma::Pgo,
        requires: &[Requirement::ProfrawProfdata],
    },
    PragmaRequirement {
        pragma: Pragma::Bolt,
        requires: &[Requirement::LlvmBolt],
    },
    PragmaRequirement {
        pragma: Pragma::Profiling,
        requires: &[Requirement::ProfrawProfdata],
    },
    PragmaRequirement {
        pragma: Pragma::BuildStd,
        requires: &[],
    },
    PragmaRequirement {
        pragma: Pragma::ParallelCodegen(u8::MAX), // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: sentinel for discriminant-only lookup; param irrelevant; tracked: #72
        requires: &[],
    },
    PragmaRequirement {
        pragma: Pragma::SharedGenerics,
        requires: &[],
    },
    PragmaRequirement {
        pragma: Pragma::LoopFusion,
        requires: &[Requirement::LlvmPassPlugin],
    },
    PragmaRequirement {
        pragma: Pragma::MimallocAllocator,
        requires: &[Requirement::MimallocCrate],
    },
];

/// Look up the requirements row for a given pragma. Matches on the
/// discriminant — `ParallelCodegen(n)` lookups ignore `n`.
pub fn requirements_for(p: Pragma) -> &'static [Requirement] {
    for row in REQUIREMENTS {
        if same_variant(row.pragma, p) {
            return row.requires;
        }
    }
    &[]
}

const fn same_variant(a: Pragma, b: Pragma) -> bool { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: internal discriminant-equality helper; tracked: #72
    use Pragma::*;
    matches!(
        (a, b),
        (LoopOptimization, LoopOptimization)
            | (Polly, Polly)
            | (MathPeephole, MathPeephole)
            | (FastMath, FastMath)
            | (ExpandedLto, ExpandedLto)
            | (Pgo, Pgo)
            | (Bolt, Bolt)
            | (Profiling, Profiling)
            | (BuildStd, BuildStd)
            | (ParallelCodegen(_), ParallelCodegen(_))
            | (SharedGenerics, SharedGenerics)
            | (LoopFusion, LoopFusion)
            | (MimallocAllocator, MimallocAllocator)
    )
}
