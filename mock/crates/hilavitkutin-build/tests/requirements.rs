//! Every `Pragma` has exactly the external-tool requirements DESIGN
//! Q4f documents.

use hilavitkutin_build::{Pragma, REQUIREMENTS, Requirement, requirements_for};

#[test]
fn table_covers_all_pragmas_exactly_once() {
    // One row per variant, 13 total.
    assert_eq!(REQUIREMENTS.len(), 13);

    // No duplicate variants in the table.
    let variants = [
        Pragma::LoopOptimization,
        Pragma::Polly,
        Pragma::MathPeephole,
        Pragma::FastMath,
        Pragma::ExpandedLto,
        Pragma::Pgo,
        Pragma::Bolt,
        Pragma::Profiling,
        Pragma::BuildStd,
        Pragma::ParallelCodegen(0),
        Pragma::SharedGenerics,
        Pragma::LoopFusion,
        Pragma::MimallocAllocator,
    ];
    for v in variants {
        // Exactly one row whose discriminant matches `v`.
        let matches = REQUIREMENTS
            .iter()
            .filter(|r| discriminant_eq(r.pragma, v))
            .count();
        assert_eq!(matches, 1, "variant missing or duplicated: {v:?}");
    }
}

#[test]
fn loop_optimization_needs_pass_plugin() {
    assert_eq!(
        requirements_for(Pragma::LoopOptimization),
        &[Requirement::LlvmPassPlugin]
    );
}

#[test]
fn polly_needs_polly_llvm() {
    assert_eq!(
        requirements_for(Pragma::Polly),
        &[Requirement::PollyEnabledLlvm]
    );
}

#[test]
fn math_peephole_needs_pass_plugin() {
    assert_eq!(
        requirements_for(Pragma::MathPeephole),
        &[Requirement::LlvmPassPlugin]
    );
}

#[test]
fn pgo_needs_profraw_tooling() {
    assert_eq!(
        requirements_for(Pragma::Pgo),
        &[Requirement::ProfrawProfdata]
    );
}

#[test]
fn bolt_needs_llvm_bolt() {
    assert_eq!(requirements_for(Pragma::Bolt), &[Requirement::LlvmBolt]);
}

#[test]
fn profiling_needs_profraw_tooling() {
    assert_eq!(
        requirements_for(Pragma::Profiling),
        &[Requirement::ProfrawProfdata]
    );
}

#[test]
fn loop_fusion_needs_pass_plugin() {
    assert_eq!(
        requirements_for(Pragma::LoopFusion),
        &[Requirement::LlvmPassPlugin]
    );
}

#[test]
fn mimalloc_allocator_needs_mimalloc_crate() {
    assert_eq!(
        requirements_for(Pragma::MimallocAllocator),
        &[Requirement::MimallocCrate]
    );
}

#[test]
fn pragmas_without_external_requirements() {
    for p in [
        Pragma::FastMath,
        Pragma::ExpandedLto,
        Pragma::BuildStd,
        Pragma::ParallelCodegen(4),
        Pragma::SharedGenerics,
    ] {
        assert_eq!(requirements_for(p), &[], "expected no reqs for {p:?}");
    }
}

fn discriminant_eq(a: Pragma, b: Pragma) -> bool {
    core::mem::discriminant(&a) == core::mem::discriminant(&b)
}
