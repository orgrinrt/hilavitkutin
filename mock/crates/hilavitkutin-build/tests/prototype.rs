//! A prototype names one set per profile. These hold the crate's three to
//! the sets the design gives them, and to the directives they write.

mod common;

use common::{held, walked};
use hilavitkutin_build::{
    Cranelift,
    Default as Defaults,
    Deterministic,
    GccCc,
    GccDriver,
    PragmaSet,
    Prototype,
    Reach,
    Set,
    ThinLto,
    Workspace,
    claimed,
    configure,
    configure_as,
};
use notko::Maybe;

/// The five sets of a prototype as the names they hold, dev, dev-opt,
/// release, profiling and ci, each also held to its own rules and to
/// naming its members in the order of their bits.
fn sets<P: Prototype>() -> [Vec<&'static str>; 5] {
    fn set<S: Set>() -> Vec<&'static str> {
        assert_eq!(S::VERDICT, "");
        let names = held(PragmaSet::of::<S>());
        assert_eq!(
            walked::<S>(),
            names,
            "a set names its members in the order of their bits"
        );
        names
    }
    [
        set::<P::Dev>(),
        set::<P::DevOpt>(),
        set::<P::Release>(),
        set::<P::Profiling>(),
        set::<P::Ci>(),
    ]
}

#[test]
fn the_default_sets_are_the_designs() {
    assert_eq!(sets::<Defaults>(), [
        vec!["ParallelCodegen", "SharedGenerics"],
        vec!["LoopOptimization", "ParallelCodegen", "SharedGenerics"],
        vec!["LoopOptimization", "MathPeephole", "FastMath", "Pgo", "Bolt", "ParallelCodegen"],
        vec![
            "LoopOptimization",
            "MathPeephole",
            "FastMath",
            "Pgo",
            "Bolt",
            "Profiling",
            "ParallelCodegen"
        ],
        vec!["LoopOptimization", "ParallelCodegen"],
    ]);
}

/// `Deterministic` is `Default` with neither `FastMath` nor `MathPeephole`,
/// which stands on it, and with nothing else gone.
#[test]
fn deterministic_is_default_without_fast_math_and_what_stands_on_it() {
    let without = |names: Vec<&'static str>| -> Vec<&'static str> {
        names
            .into_iter()
            .filter(|name| !matches!(*name, "FastMath" | "MathPeephole"))
            .collect()
    };
    assert_eq!(sets::<Deterministic>(), sets::<Defaults>().map(without));
    assert_eq!(sets::<Deterministic>()[2], [
        "LoopOptimization",
        "Pgo",
        "Bolt",
        "ParallelCodegen"
    ]);
}

/// `Workspace` is `Default` with `ExpandedLto` in release and in ci.
#[test]
fn workspace_is_default_with_expanded_lto_in_release_and_ci() {
    let [dev, dev_opt, release, profiling, ci] = sets::<Workspace>();
    let [default_dev, default_dev_opt, _, default_profiling, _] = sets::<Defaults>();
    assert_eq!(
        (dev, dev_opt, profiling),
        (default_dev, default_dev_opt, default_profiling)
    );
    assert_eq!(release, [
        "LoopOptimization",
        "MathPeephole",
        "FastMath",
        "ExpandedLto",
        "Pgo",
        "Bolt",
        "ParallelCodegen"
    ]);
    assert_eq!(ci, ["LoopOptimization", "ExpandedLto", "ParallelCodegen"]);
}

/// Only `Workspace` reaches past its crate, so the other two are what a
/// script that claims nothing may name.
#[test]
fn only_workspace_holds_a_pragma_of_workspace_reach() {
    fn reaches<P: Prototype>() -> [Reach; 5] {
        [
            <P::Dev as Set>::REACH,
            <P::DevOpt as Set>::REACH,
            <P::Release as Set>::REACH,
            <P::Profiling as Set>::REACH,
            <P::Ci as Set>::REACH,
        ]
    }
    assert_eq!(reaches::<Defaults>(), [Reach::Crate; 5]);
    assert_eq!(reaches::<Deterministic>(), [Reach::Crate; 5]);
    assert_eq!(reaches::<Workspace>(), [
        Reach::Crate,
        Reach::Crate,
        Reach::Workspace,
        Reach::Crate,
        Reach::Workspace
    ]);
}

/// A prototype that holds one driver and one C compiler in every profile,
/// and other workspace pragmas that differ by profile.
struct SameShared;

impl Prototype for SameShared {
    type Ci = (GccDriver, GccCc);
    type Dev = (GccDriver, Cranelift, GccCc);
    type DevOpt = (GccCc, GccDriver);
    type Profiling = (GccDriver, GccCc, ThinLto);
    type Release = (ThinLto, GccDriver, GccCc);
}

/// The claim holds of the crate's three and of a prototype whose shared
/// groups agree, in a `const` item as the claiming script has it. The
/// prototype it refuses is built and seen refused in `tests/refused.rs`.
const _: () = {
    claimed::<Workspace>();
    claimed::<Defaults>();
    claimed::<Deterministic>();
    claimed::<SameShared>();
};

/// The five LLVM options LoopOptimization owns, as the flags carry them.
const LOOP_FLAGS: &str = "-Cllvm-args=-enable-loop-distribute \
    -Cllvm-args=-enable-loop-versioning-licm \
    -Cllvm-args=-enable-interleaved-mem-accesses \
    -Cllvm-args=-loop-prefetch-writes \
    -Cllvm-args=-enable-histogram-loop-vectorization";

/// A build of `profile` with the wrapper named, where nothing warns.
fn wrapped(profile: &'static str) -> impl Fn(&'static str) -> Maybe<&'static str> {
    move |key| {
        match key {
            "PROFILE" => Maybe::Is(profile),
            "RUSTC_WORKSPACE_WRAPPER" => Maybe::Is(".cargo/hilavitkutin-rustc-wrapper"),
            _ => Maybe::Isnt,
        }
    }
}

fn written<P: Prototype>(profile: &'static str) -> String {
    let mut out = String::new();
    configure_as::<P, _, _>(wrapped(profile), &mut out).expect("writing to a String cannot fail");
    out
}

/// The directives of the two prototypes a crate's script names, as the
/// text they were before a set was a type: a consumer's flags do not move.
#[test]
fn the_directives_of_default_and_deterministic_are_the_text_they_were() {
    const HEAD: &str = "cargo::rerun-if-env-changed=RUSTC_WORKSPACE_WRAPPER\n\
                        cargo::rustc-check-cfg=cfg(arvo_fast_math)\n";
    // The profile names cargo passes, and one the crate does not know.
    for debug in ["debug", "bench"] {
        let dev = format!(
            "{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS=-Zthreads=0 -Zshare-generics=y\n"
        );
        assert_eq!(written::<Defaults>(debug), dev);
        assert_eq!(written::<Deterministic>(debug), dev);
    }
    assert_eq!(
        written::<Defaults>("release"),
        format!(
            "{HEAD}cargo::rustc-cfg=arvo_fast_math\n\
             cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={LOOP_FLAGS} -Zthreads=0\n"
        )
    );
    assert_eq!(
        written::<Deterministic>("release"),
        format!("{HEAD}cargo::rustc-env=HILAVITKUTIN_BUILD_RUSTFLAGS={LOOP_FLAGS} -Zthreads=0\n")
    );
}

#[test]
fn configure_takes_the_default_prototype() {
    for profile in ["debug", "release", "bench"] {
        let mut configured = String::new();
        configure(wrapped(profile), &mut configured).expect("writing to a String cannot fail");
        assert_eq!(configured, written::<Defaults>(profile), "{profile}");
    }
    assert_ne!(
        written::<Defaults>("release"),
        written::<Deterministic>("release")
    );
}

/// A sink that refuses every write.
struct Refusing;

impl core::fmt::Write for Refusing {
    fn write_str(&mut self, _: &str) -> core::fmt::Result {
        Err(core::fmt::Error)
    }
}

#[test]
fn a_refusing_writer_is_reported() {
    assert!(configure_as::<Defaults, _, _>(wrapped("release"), &mut Refusing).is_err());
    assert!(configure_as::<Deterministic, _, _>(wrapped("debug"), &mut Refusing).is_err());
}
