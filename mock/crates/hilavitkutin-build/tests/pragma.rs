//! What a pragma is, read off its constants: its bit, its group, its
//! reach, what it requires and the tool it waits on, each as a literal.

mod common;

use common::{PRAGMAS, pragmas};
use hilavitkutin_build::{
    All,
    Any,
    BackendGroup,
    Bolt,
    CcGroup,
    Cranelift,
    Default as Defaults,
    DriverGroup,
    EdgeGroup,
    ExpandedLto,
    FastMath,
    Free,
    Group,
    IncrementalGroup,
    LinkerGroup,
    LoopOptimization,
    LtoGroup,
    Mask,
    MathPeephole,
    Not,
    ParallelCodegen,
    Pgo,
    Pragma,
    PragmaSet,
    Profiling,
    Prototype,
    Reach,
    Requirement,
    SharedGenerics,
    ThinLto,
};

/// The mask of two pragmas that shared a bit would miss a bit, and the
/// list is held to the count, so a pragma left out of it shows as a gap.
#[test]
fn every_two_pragma_bits_are_apart() {
    let rows = pragmas();
    assert_eq!(rows.len(), PRAGMAS);
    let mut mask: Mask = 0;
    for (at, row) in rows.iter().enumerate() {
        assert_eq!(row.bit, 1 << at, "{} is the pragma at place {at}", row.name);
        assert_eq!(mask & row.bit, 0, "{} shares its bit", row.name);
        mask |= row.bit;
    }
    assert_eq!(mask, (1 << PRAGMAS) - 1);
    let mut names: Vec<_> = rows.iter().map(|row| row.name).collect();
    names.sort_unstable();
    names.dedup();
    assert_eq!(names.len(), PRAGMAS, "two pragmas share a name");
}

#[test]
fn a_pragma_with_a_value_has_one_bit_and_one_name_whatever_the_value() {
    assert_eq!(
        <ParallelCodegen<0> as Pragma>::BIT,
        <ParallelCodegen<7> as Pragma>::BIT
    );
    assert_eq!(<ParallelCodegen<7> as Pragma>::NAME, "ParallelCodegen");
}

#[test]
fn the_seven_groups_are_apart_and_two_of_them_are_shared() {
    let groups: [(&str, Mask, &str, bool); 7] = [
        (
            "linker",
            LinkerGroup::BIT,
            LinkerGroup::CLASH,
            LinkerGroup::SHARED,
        ),
        ("edge", EdgeGroup::BIT, EdgeGroup::CLASH, EdgeGroup::SHARED),
        ("lto", LtoGroup::BIT, LtoGroup::CLASH, LtoGroup::SHARED),
        (
            "incremental",
            IncrementalGroup::BIT,
            IncrementalGroup::CLASH,
            IncrementalGroup::SHARED,
        ),
        (
            "backend",
            BackendGroup::BIT,
            BackendGroup::CLASH,
            BackendGroup::SHARED,
        ),
        (
            "driver",
            DriverGroup::BIT,
            DriverGroup::CLASH,
            DriverGroup::SHARED,
        ),
        ("cc", CcGroup::BIT, CcGroup::CLASH, CcGroup::SHARED),
    ];
    let mut mask: Mask = 0;
    for (at, (name, bit, clash, shared)) in groups.into_iter().enumerate() {
        assert_eq!(bit, 1 << at, "{name}");
        mask |= bit;
        assert!(clash.starts_with("the set holds "), "{name}: {clash}");
        assert_eq!(shared, matches!(name, "driver" | "cc"), "{name}");
    }
    assert_eq!(mask, 0b111_1111);
    assert_eq!(Free::BIT, 0);
    assert_eq!(Free::CLASH, "");
    assert!(!Free::SHARED);
}

#[test]
fn every_pragma_is_of_the_group_and_the_reach_the_design_gives_it() {
    for row in pragmas() {
        let (group, reach) = match row.name {
            "Mold" | "Wild" | "Bfd" => (LinkerGroup::BIT, Reach::Crate),
            "DylibDomain" => (EdgeGroup::BIT, Reach::Crate),
            "ThinLto" | "ExpandedLto" => (LtoGroup::BIT, Reach::Workspace),
            "Incremental" | "NoIncremental" => (IncrementalGroup::BIT, Reach::Workspace),
            "Cranelift" => (BackendGroup::BIT, Reach::Workspace),
            "GccDriver" | "ClangDriver" => (DriverGroup::BIT, Reach::Workspace),
            "GccCc" | "ClangCc" => (CcGroup::BIT, Reach::Workspace),
            "ChecksumFreshness" => (0, Reach::Workspace),
            _ => (0, Reach::Crate),
        };
        assert_eq!(row.group, group, "{}", row.name);
        assert_eq!(row.reach, reach, "{}", row.name);
    }
}

#[test]
fn every_pragma_waits_on_the_tools_the_design_gives_it() {
    for row in pragmas() {
        let needs: &[&str] = match row.name {
            "LoopOptimization" | "LoopFusion" => &["polka-passes"],
            "MathPeephole" => &["math-peephole"],
            "Polly" => &["polly"],
            "Pgo" | "Profiling" => &["llvm-profdata"],
            "Bolt" => &["llvm-bolt"],
            "MimallocAllocator" => &["mimalloc"],
            "Mold" => &["mold"],
            "Wild" => &["wild", "clang"],
            "Cranelift" => &["cranelift"],
            "GccDriver" => &["gcc"],
            "ClangDriver" => &["clang"],
            "GccCc" => &["gcc", "g++", "ar"],
            "ClangCc" => &["clang", "clang++", "llvm-ar"],
            _ => &[],
        };
        assert_eq!(row.needs, needs, "{}", row.name);
    }
}

/// A script keeps the tools of a set in forty-eight places, three for
/// each of sixteen members, and every need is a row of the table.
#[test]
fn no_pragma_needs_more_than_three_tools_and_each_is_a_row_of_the_table() {
    for row in pragmas() {
        assert!(row.needs.len() <= 3, "{} needs {:?}", row.name, row.needs);
        for need in &row.needs {
            assert!(
                hilavitkutin_build::TOOLS
                    .iter()
                    .any(|tool| tool.name == *need),
                "{} needs {need}, which is no row",
                row.name
            );
        }
    }
}

#[test]
fn a_requirement_is_the_three_masks_of_its_form() {
    let both = FastMath::BIT | Pgo::BIT;
    type Both = (FastMath, Pgo);
    assert_eq!(
        (
            <All<Both> as Requirement>::ALL,
            <All<Both> as Requirement>::ANY,
            <All<Both> as Requirement>::NONE
        ),
        (both, 0, 0)
    );
    assert_eq!(
        (
            <Any<Both> as Requirement>::ALL,
            <Any<Both> as Requirement>::ANY,
            <Any<Both> as Requirement>::NONE
        ),
        (0, both, 0)
    );
    assert_eq!(
        (
            <Not<Both> as Requirement>::ALL,
            <Not<Both> as Requirement>::ANY,
            <Not<Both> as Requirement>::NONE
        ),
        (0, 0, both)
    );
    assert_eq!(
        (
            <() as Requirement>::ALL,
            <() as Requirement>::ANY,
            <() as Requirement>::NONE
        ),
        (0, 0, 0)
    );
}

#[test]
fn the_three_pragmas_that_require_name_what_the_design_says() {
    assert_eq!(
        <<MathPeephole as Pragma>::Requires as Requirement>::ALL,
        FastMath::BIT
    );
    assert_eq!(
        <<Profiling as Pragma>::Requires as Requirement>::ANY,
        Pgo::BIT | Bolt::BIT
    );
    assert_eq!(
        <<Cranelift as Pragma>::Requires as Requirement>::NONE,
        ThinLto::BIT | ExpandedLto::BIT
    );
    assert_eq!(
        MathPeephole::UNMET,
        "MathPeephole stands on FastMath, which the set does not hold"
    );
    assert_eq!(
        Profiling::UNMET,
        "Profiling stands on Pgo or Bolt, and the set holds neither"
    );
    assert_eq!(
        Cranelift::UNMET,
        "Cranelift builds without LTO, and the set holds an LTO pragma"
    );
}

#[test]
fn a_folded_set_holds_its_members_and_no_other_pragma() {
    let release = PragmaSet::of::<<Defaults as Prototype>::Release>();
    assert!(release.contains::<LoopOptimization>());
    assert!(release.contains::<MathPeephole>());
    assert!(release.contains::<FastMath>());
    assert!(release.contains::<Pgo>());
    assert!(release.contains::<Bolt>());
    assert!(release.contains::<ParallelCodegen<0>>());
    assert!(release.contains::<ParallelCodegen<9>>());
    assert!(!release.contains::<SharedGenerics>());
    assert!(!release.contains::<ExpandedLto>());
    assert!(!release.contains::<Profiling>());

    let empty = PragmaSet::of::<()>();
    assert_eq!(empty, PragmaSet::default());
    assert!(!empty.contains::<FastMath>());
    assert!(!empty.contains::<ParallelCodegen<0>>());
    assert_ne!(empty, release);
    assert_eq!(
        PragmaSet::of::<(Pgo, FastMath)>(),
        PragmaSet::of::<(FastMath, Pgo)>()
    );
}

#[test]
fn a_set_is_folded_before_the_program_runs() {
    const RELEASE: PragmaSet = PragmaSet::of::<<Defaults as Prototype>::Release>();
    const HELD: bool = RELEASE.contains::<FastMath>();
    const NOT_HELD: bool = RELEASE.contains::<SharedGenerics>();
    assert!(HELD);
    assert!(!NOT_HELD);
}
