//! A set is a tuple of pragma types. These hold what it folds to, what its
//! verdict is for each rule broken and kept, the order of its walk, and
//! how its flags are put together.

use core::fmt;

use hilavitkutin_build::tools::WILD;
use hilavitkutin_build::{
    Bfd,
    Bolt,
    BuildStd,
    ClangCc,
    ClangDriver,
    Cranelift,
    DriverGroup,
    DylibDomain,
    ExpandedLto,
    FastMath,
    Flags,
    GccCc,
    GccDriver,
    Group,
    Incremental,
    LinkerGroup,
    LoopFusion,
    LoopOptimization,
    LtoGroup,
    MathPeephole,
    MimallocAllocator,
    Mold,
    NoIncremental,
    ParallelCodegen,
    Pgo,
    Polly,
    Pragma,
    Profile,
    Profiling,
    Reach,
    Set,
    Shared,
    SharedGenerics,
    Site,
    ThinLto,
    ToolPath,
    Visit,
    Wild,
    checked,
    crate_reach,
    each_tool,
    write_rustflags,
    writes_flags,
};
use notko::Maybe;

const TWICE: &str = "the set names one pragma twice";
const NEEDS_FAST_MATH: &str = "MathPeephole stands on FastMath, which the set does not hold";
const NEEDS_A_PROFILE: &str = "Profiling stands on Pgo or Bolt, and the set holds neither";
const NO_LTO: &str = "Cranelift builds without LTO, and the set holds an LTO pragma";

/// The verdict is a constant of the set's type, so each is read here as
/// one. A set that keeps every rule has the empty string, and each set
/// that breaks one stands beside the set that keeps it.
#[test]
fn a_verdict_is_the_sentence_of_the_rule_the_set_breaks() {
    const VERDICTS: [(&str, &str, &str); 22] = [
        ("empty", <() as Set>::VERDICT, ""),
        (
            "all, unmet",
            <(MathPeephole,) as Set>::VERDICT,
            NEEDS_FAST_MATH,
        ),
        (
            "all, met after",
            <(MathPeephole, FastMath) as Set>::VERDICT,
            "",
        ),
        (
            "all, met before",
            <(FastMath, MathPeephole) as Set>::VERDICT,
            "",
        ),
        (
            "any, unmet",
            <(Profiling, FastMath) as Set>::VERDICT,
            NEEDS_A_PROFILE,
        ),
        (
            "any, met by the first",
            <(Profiling, Pgo) as Set>::VERDICT,
            "",
        ),
        (
            "any, met by the second",
            <(Bolt, Profiling) as Set>::VERDICT,
            "",
        ),
        ("none, alone", <(Cranelift,) as Set>::VERDICT, ""),
        (
            "none, broken after",
            <(Cranelift, ThinLto) as Set>::VERDICT,
            NO_LTO,
        ),
        (
            "none, broken before",
            <(ExpandedLto, Cranelift) as Set>::VERDICT,
            NO_LTO,
        ),
        ("one linker", <(Mold,) as Set>::VERDICT, ""),
        (
            "two linkers",
            <(Mold, Wild) as Set>::VERDICT,
            LinkerGroup::CLASH,
        ),
        (
            "two linkers, apart",
            <(Bfd, FastMath, Mold) as Set>::VERDICT,
            LinkerGroup::CLASH,
        ),
        (
            "two LTO",
            <(ThinLto, ExpandedLto) as Set>::VERDICT,
            LtoGroup::CLASH,
        ),
        (
            "two drivers",
            <(GccDriver, ClangDriver) as Set>::VERDICT,
            DriverGroup::CLASH,
        ),
        (
            "one of each group",
            <(Mold, DylibDomain, ThinLto, Incremental, GccDriver, GccCc) as Set>::VERDICT,
            "",
        ),
        (
            "no group, many",
            <(FastMath, Pgo, Bolt, Polly, BuildStd) as Set>::VERDICT,
            "",
        ),
        (
            "one pragma twice",
            <(FastMath, FastMath) as Set>::VERDICT,
            TWICE,
        ),
        (
            "one pragma twice, two values",
            <(ParallelCodegen<0>, ParallelCodegen<4>) as Set>::VERDICT,
            TWICE,
        ),
        (
            "one pragma twice comes before its group",
            <(Mold, Mold) as Set>::VERDICT,
            TWICE,
        ),
        (
            "both incrementals",
            <(Incremental, NoIncremental) as Set>::VERDICT,
            "the set holds Incremental and NoIncremental, and a profile is one of the two",
        ),
        (
            "the first thing wrong is the one named",
            <(MathPeephole, Mold, Wild) as Set>::VERDICT,
            NEEDS_FAST_MATH,
        ),
    ];
    for (what, verdict, expected) in VERDICTS {
        assert_eq!(verdict, expected, "{what}");
    }
    assert!(!LinkerGroup::CLASH.is_empty() && LinkerGroup::CLASH != LtoGroup::CLASH);
}

/// The two assertions hold of sets that keep their rule, in a `const`
/// item as a script has them. The sets they refuse are built and seen
/// refused in `tests/refused.rs`.
const _: () = {
    checked::<(MathPeephole, FastMath)>();
    checked::<(Mold, ThinLto)>();
    crate_reach::<(Mold, DylibDomain, FastMath)>();
    crate_reach::<()>();
};

#[test]
fn a_set_folds_its_members_into_its_constants() {
    type Mixed = (Mold, ThinLto, ClangDriver, FastMath);
    assert_eq!(
        <Mixed as Set>::MASK,
        Mold::BIT | ThinLto::BIT | ClangDriver::BIT | FastMath::BIT
    );
    assert_eq!(
        <Mixed as Set>::GROUPS,
        LinkerGroup::BIT | LtoGroup::BIT | DriverGroup::BIT
    );
    assert_eq!(<Mixed as Set>::SHARED, ClangDriver::BIT);
    assert_eq!(<Mixed as Set>::REACH, Reach::Workspace);

    assert_eq!(
        <(GccCc, GccDriver) as Set>::SHARED,
        GccCc::BIT | GccDriver::BIT
    );
    assert_eq!(<(ThinLto, Cranelift, NoIncremental) as Set>::SHARED, 0);
    assert_eq!(<(Mold, DylibDomain, FastMath) as Set>::REACH, Reach::Crate);
    assert_eq!(<(FastMath, ExpandedLto) as Set>::REACH, Reach::Workspace);
    assert_eq!(
        (<() as Set>::MASK, <() as Set>::GROUPS, <() as Set>::SHARED),
        (0, 0, 0)
    );
    assert_eq!(<() as Set>::REACH, Reach::Crate);
}

/// A walk that keeps the name of every member it is handed, and refuses
/// at one place where it is told to.
struct Names {
    seen:      Vec<&'static str>,
    refuse_at: Option<usize>,
}

impl Visit for Names {
    fn pragma<P: Pragma>(&mut self) -> fmt::Result {
        if self.refuse_at == Some(self.seen.len()) {
            return Err(fmt::Error);
        }
        self.seen.push(P::NAME);
        Ok(())
    }
}

fn walked<S: Set>() -> Vec<&'static str> {
    let mut names = Names {
        seen:      Vec::new(),
        refuse_at: None,
    };
    S::each(&mut names).expect("the walk refuses nothing");
    names.seen
}

#[test]
fn a_walk_is_handed_every_member_once_in_the_order_named() {
    assert_eq!(walked::<()>(), [""; 0]);
    assert_eq!(walked::<(Pgo,)>(), ["Pgo"]);
    assert_eq!(walked::<(Pgo, Bolt)>(), ["Pgo", "Bolt"]);
    assert_eq!(walked::<(Bolt, Pgo)>(), ["Bolt", "Pgo"]);
    assert_eq!(
        walked::<(
            Mold,
            FastMath,
            Pgo,
            Bolt,
            Polly,
            BuildStd,
            LoopFusion,
            ThinLto
        )>(),
        ["Mold", "FastMath", "Pgo", "Bolt", "Polly", "BuildStd", "LoopFusion", "ThinLto"]
    );
    type Sixteen = (
        LoopOptimization,
        Polly,
        MathPeephole,
        FastMath,
        ExpandedLto,
        Pgo,
        Bolt,
        Profiling,
        BuildStd,
        ParallelCodegen<1>,
        SharedGenerics,
        LoopFusion,
        MimallocAllocator,
        Mold,
        DylibDomain,
        Incremental,
    );
    assert_eq!(walked::<Sixteen>(), [
        "LoopOptimization",
        "Polly",
        "MathPeephole",
        "FastMath",
        "ExpandedLto",
        "Pgo",
        "Bolt",
        "Profiling",
        "BuildStd",
        "ParallelCodegen",
        "SharedGenerics",
        "LoopFusion",
        "MimallocAllocator",
        "Mold",
        "DylibDomain",
        "Incremental"
    ]);
    assert_eq!(<Sixteen as Set>::VERDICT, "");
}

#[test]
fn a_walk_stops_at_the_member_its_visit_refuses() {
    let mut names = Names {
        seen:      Vec::new(),
        refuse_at: Some(2),
    };
    assert!(<(Pgo, Bolt, Polly, BuildStd) as Set>::each(&mut names).is_err());
    assert_eq!(names.seen, ["Pgo", "Bolt"]);
}

fn flags<S: Set>() -> String {
    let mut out = String::new();
    write_rustflags::<S, _>(&Site::new(Profile::Release, "", &[]), &mut out)
        .expect("a String takes every write");
    out
}

/// One space between two pragmas that wrote, and none for one that wrote
/// nothing, wherever it stands in the set.
#[test]
fn a_sets_flags_are_its_members_flags_with_one_space_between() {
    const LOOP: &str = "-Cllvm-args=-enable-loop-distribute -Cllvm-args=-enable-loop-versioning-licm \
                        -Cllvm-args=-enable-interleaved-mem-accesses -Cllvm-args=-loop-prefetch-writes \
                        -Cllvm-args=-enable-histogram-loop-vectorization";
    assert_eq!(flags::<()>(), "");
    assert_eq!(flags::<(Pgo, Bolt, FastMath)>(), "");
    assert_eq!(flags::<(SharedGenerics,)>(), "-Zshare-generics=y");
    assert_eq!(flags::<(Pgo, SharedGenerics)>(), "-Zshare-generics=y");
    assert_eq!(flags::<(SharedGenerics, Pgo)>(), "-Zshare-generics=y");
    assert_eq!(
        flags::<(ParallelCodegen<3>, Pgo, Bolt, SharedGenerics)>(),
        "-Zthreads=3 -Zshare-generics=y"
    );
    assert_eq!(
        flags::<(
            Pgo,
            LoopOptimization,
            Bolt,
            ParallelCodegen<0>,
            FastMath,
            SharedGenerics,
            Polly
        )>(),
        format!("{LOOP} -Zthreads=0 -Zshare-generics=y")
    );
    assert_eq!(
        flags::<(Mold, DylibDomain, Pgo)>(),
        "-Clink-arg=-fuse-ld=mold -Cprefer-dynamic"
    );
    assert_eq!(flags::<(ParallelCodegen<255>,)>(), "-Zthreads=255");

    // A member's flags are written with the tool its site has for it.
    let tools = [ToolPath::new(&WILD, "/t/wild")];
    let mut found = String::new();
    write_rustflags::<(SharedGenerics, Wild, DylibDomain), _>(
        &Site::new(Profile::Release, "", &tools),
        &mut found,
    )
    .expect("a String takes every write");
    assert_eq!(
        found,
        "-Zshare-generics=y -Clinker=clang -Clink-arg=--ld-path=/t/wild -Cprefer-dynamic"
    );
    assert_eq!(
        flags::<(SharedGenerics, Wild, DylibDomain)>(),
        "-Zshare-generics=y -Cprefer-dynamic"
    );
}

/// Whether a set has flags is asked at its site: a pragma that writes its
/// flag only with its tool's path has none without it.
#[test]
fn a_set_has_flags_where_a_member_writes_one() {
    let bare = Site::new(Profile::Release, "", &[]);
    assert_eq!(writes_flags::<()>(&bare), Flags::Nothing);
    assert_eq!(
        writes_flags::<(Pgo, FastMath, ThinLto)>(&bare),
        Flags::Nothing
    );
    assert_eq!(writes_flags::<(Pgo, Mold)>(&bare), Flags::Written);
    assert_eq!(writes_flags::<(Pgo, SharedGenerics)>(&bare), Flags::Written);
    assert_eq!(writes_flags::<(Wild,)>(&bare), Flags::Nothing);
    let tools = [ToolPath::new(&WILD, "/t/wild")];
    assert_eq!(
        writes_flags::<(Wild,)>(&Site::new(Profile::Release, "", &tools)),
        Flags::Written
    );
}

#[test]
fn a_site_hands_every_member_its_own_tool_and_what_is_owed() {
    let tools = [ToolPath::new(&WILD, "/tools/wild-0.10.0/bin/wild")];
    let site = Site::new(Profile::Release, "x86_64-unknown-linux-gnu", &tools);
    assert_eq!(site.profile(), Profile::Release);

    let wild = site.written::<Wild>();
    assert_eq!(wild.profile(), "release");
    assert_eq!(wild.triple(), "x86_64-unknown-linux-gnu");
    assert_eq!(wild.tool(), Maybe::Is("/tools/wild-0.10.0/bin/wild"));
    assert_eq!(wild.shared(), Shared::Owed);
    // A pragma is handed its own tool's path and no other's, and of its
    // own the first: `ClangCc` needs clang first and `Wild` second.
    assert_eq!(site.written::<Mold>().tool(), Maybe::Isnt);
    let both = [
        ToolPath::new(&hilavitkutin_build::tools::CLANG, "/usr/bin/clang"),
        ToolPath::new(&WILD, "/tools/wild"),
    ];
    let both = Site::new(Profile::Release, "", &both);
    assert_eq!(both.written::<Wild>().tool(), Maybe::Is("/tools/wild"));
    assert_eq!(
        both.written::<ClangCc>().tool(),
        Maybe::Is("/usr/bin/clang")
    );
    assert_eq!(site.written::<FastMath>().tool(), Maybe::Isnt);
    assert_eq!(
        Site::new(Profile::Dev, "", &[]).written::<Wild>().tool(),
        Maybe::Isnt
    );
    assert_eq!(
        Site::new(Profile::Dev, "", &[]).written::<Wild>().profile(),
        "dev"
    );

    // After a set that held a driver, every driver's shared key is written,
    // and no other group's.
    let after = site.after::<(ClangDriver, FastMath)>();
    assert_eq!(after.written::<ClangDriver>().shared(), Shared::Written);
    assert_eq!(after.written::<GccDriver>().shared(), Shared::Written);
    assert_eq!(after.written::<ClangCc>().shared(), Shared::Owed);
    assert_eq!(after.written::<Cranelift>().shared(), Shared::Owed);
    // A pragma of no group is told by its own bit.
    assert_eq!(after.written::<FastMath>().shared(), Shared::Written);
    assert_eq!(after.written::<Pgo>().shared(), Shared::Owed);
    assert_eq!(
        <ClangDriver as Pragma>::BIT & <<ClangDriver as Pragma>::Group as Group>::BIT,
        0
    );
}

#[test]
fn the_tools_of_a_set_are_handed_over_in_its_order() {
    let named = |each: fn(&mut dyn FnMut(&'static str))| {
        let mut names = Vec::new();
        each(&mut |name| names.push(name));
        names
    };
    assert_eq!(
        named(|push| each_tool::<()>(|tool| push(tool.name))),
        [""; 0]
    );
    assert_eq!(
        named(|push| each_tool::<(Bfd, FastMath, DylibDomain)>(|tool| push(tool.name))),
        [""; 0]
    );
    assert_eq!(
        named(|push| each_tool::<(ClangDriver, Mold, ClangCc, Wild)>(|tool| push(tool.name))),
        ["clang", "mold", "clang", "clang++", "llvm-ar", "wild", "clang"]
    );
    assert_eq!(
        named(|push| each_tool::<(MimallocAllocator, Pgo)>(|tool| push(tool.name))),
        ["mimalloc", "llvm-profdata"]
    );
}
