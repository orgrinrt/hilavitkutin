//! What the tests of several binaries share: the list of every pragma.

#![allow(dead_code)]

use hilavitkutin_build::{
    Bfd,
    Bolt,
    BuildStd,
    ChecksumFreshness,
    ClangCc,
    ClangDriver,
    Cranelift,
    DylibDomain,
    ExpandedLto,
    FastMath,
    GccCc,
    GccDriver,
    Group,
    Incremental,
    LoopFusion,
    LoopOptimization,
    Mask,
    MathPeephole,
    MimallocAllocator,
    Mold,
    NoIncremental,
    ParallelCodegen,
    Pgo,
    Polly,
    Pragma,
    Profiling,
    Reach,
    SharedGenerics,
    Target,
    ThinLto,
    Wild,
};
use notko::Maybe;

/// What the tests read of one pragma.
pub struct Row {
    pub name:     &'static str,
    pub bit:      Mask,
    pub group:    Mask,
    pub reach:    Reach,
    pub needs:    Vec<&'static str>,
    pub admitted: fn(&Target) -> Maybe<&'static str>,
}

fn row<P: Pragma>() -> Row {
    Row {
        name:     P::NAME,
        bit:      P::BIT,
        group:    <P::Group as Group>::BIT,
        reach:    P::REACH,
        needs:    P::NEEDS.iter().map(|tool| tool.name).collect(),
        admitted: P::admitted,
    }
}

/// The names of the pragmas a folded set holds, in the order of their bits.
pub fn held(set: hilavitkutin_build::PragmaSet) -> Vec<&'static str> {
    macro_rules! names {
        ($($pragma:ty),*) => {
            [$((set.contains::<$pragma>(), <$pragma as Pragma>::NAME)),*]
        };
    }
    names![
        LoopOptimization,
        Polly,
        MathPeephole,
        FastMath,
        ExpandedLto,
        Pgo,
        Bolt,
        Profiling,
        BuildStd,
        ParallelCodegen<0>,
        SharedGenerics,
        LoopFusion,
        MimallocAllocator,
        Mold,
        Wild,
        Bfd,
        DylibDomain,
        ThinLto,
        Incremental,
        NoIncremental,
        Cranelift,
        GccDriver,
        ClangDriver,
        GccCc,
        ClangCc,
        ChecksumFreshness
    ]
    .into_iter()
    .filter_map(|(held, name)| held.then_some(name))
    .collect()
}

/// A walk that keeps the name of every member it is handed.
struct Names(Vec<&'static str>);

impl hilavitkutin_build::Visit for Names {
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result {
        self.0.push(P::NAME);
        Ok(())
    }
}

/// The names of a set's members, in the order the set names them.
pub fn walked<S: hilavitkutin_build::Set>() -> Vec<&'static str> {
    let mut names = Names(Vec::new());
    S::each(&mut names).expect("the walk refuses nothing");
    names.0
}

/// How many pragmas the crate has.
pub const PRAGMAS: usize = 26;

/// Every pragma of the crate, each named once, in the order of its bit.
pub fn pragmas() -> Vec<Row> {
    vec![
        row::<LoopOptimization>(),
        row::<Polly>(),
        row::<MathPeephole>(),
        row::<FastMath>(),
        row::<ExpandedLto>(),
        row::<Pgo>(),
        row::<Bolt>(),
        row::<Profiling>(),
        row::<BuildStd>(),
        row::<ParallelCodegen<0>>(),
        row::<SharedGenerics>(),
        row::<LoopFusion>(),
        row::<MimallocAllocator>(),
        row::<Mold>(),
        row::<Wild>(),
        row::<Bfd>(),
        row::<DylibDomain>(),
        row::<ThinLto>(),
        row::<Incremental>(),
        row::<NoIncremental>(),
        row::<Cranelift>(),
        row::<GccDriver>(),
        row::<ClangDriver>(),
        row::<GccCc>(),
        row::<ClangCc>(),
        row::<ChecksumFreshness>(),
    ]
}
