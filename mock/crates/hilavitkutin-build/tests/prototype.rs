//! A prototype is a whole build script by name. These hold each one to the
//! directives it writes, read against `configure` and `configure_with`,
//! which a script that names no prototype still calls, over every profile
//! name cargo can pass and every profile the crate has.

use hilavitkutin_build::{Pragma, PragmaSet, Profile, Prototype, configure, configure_with};
use notko::Maybe;

/// The values of `PROFILE` a script can meet: the two cargo passes, and one
/// the crate does not know, which resolves as a dev build.
const PROFILE_NAMES: [&str; 3] = ["debug", "release", "bench"];

/// Every profile the crate has.
const PROFILES: [Profile; 5] =
    [Profile::Dev, Profile::DevOpt, Profile::Release, Profile::Profiling, Profile::Ci];

/// One of every pragma, to ask a set about each.
const PRAGMAS: [Pragma; 13] = [
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

/// A build of `profile` with no workspace wrapper named, where `configure`
/// warns when there are flags.
fn unwrapped(profile: &'static str) -> impl Fn(&'static str) -> Maybe<&'static str> {
    move |key| {
        match key {
            "PROFILE" => Maybe::Is(profile),
            _ => Maybe::Isnt,
        }
    }
}

/// The same build with the wrapper named, where nothing warns.
fn wrapped(profile: &'static str) -> impl Fn(&'static str) -> Maybe<&'static str> {
    move |key| {
        match key {
            "PROFILE" => Maybe::Is(profile),
            "RUSTC_WORKSPACE_WRAPPER" => Maybe::Is(".cargo/hilavitkutin-rustc-wrapper"),
            _ => Maybe::Isnt,
        }
    }
}

fn written(prototype: Prototype, profile: &'static str) -> String {
    let mut out = String::new();
    prototype
        .configure(unwrapped(profile), &mut out)
        .expect("writing to a String cannot fail");
    out
}

#[test]
fn default_writes_what_configure_writes_less_the_warning() {
    for profile in PROFILE_NAMES {
        let mut configured = String::new();
        configure(unwrapped(profile), &mut configured).expect("writing to a String cannot fail");
        // Every profile writes a flag, so `configure` warns in each.
        let (directives, warning) = configured
            .split_once("cargo::warning=")
            .expect("configure warns where no wrapper is named");
        assert!(warning.contains("so they are dropped"), "{profile}");
        assert_eq!(
            written(Prototype::Default, profile),
            directives,
            "{profile}"
        );

        let mut quiet = String::new();
        configure(wrapped(profile), &mut quiet).expect("writing to a String cannot fail");
        assert_eq!(written(Prototype::Default, profile), quiet, "{profile}");
    }
    assert!(written(Prototype::Default, "release").contains("cargo::rustc-cfg=arvo_fast_math\n"));
    assert!(!written(Prototype::Default, "debug").contains("cargo::rustc-cfg=arvo_fast_math\n"));
}

#[test]
fn deterministic_writes_what_configure_with_writes_without_fast_math() {
    for profile in PROFILE_NAMES {
        let out = written(Prototype::Deterministic, profile);
        assert!(!out.contains("cargo::rustc-cfg=arvo_fast_math"), "{out}");
        assert!(
            out.contains("cargo::rustc-check-cfg=cfg(arvo_fast_math)\n"),
            "{out}"
        );

        let mut tuned = String::new();
        configure_with(
            wrapped(profile),
            |pragmas| pragmas.without(Pragma::FastMath),
            &mut tuned,
        )
        .expect("writing to a String cannot fail");
        assert_eq!(out, tuned, "{profile}");
    }
    // The release build is where the two prototypes part.
    assert_ne!(
        written(Prototype::Deterministic, "release"),
        written(Prototype::Default, "release")
    );
    assert!(
        written(Prototype::Deterministic, "release")
            .contains("-Cllvm-args=-enable-loop-distribute")
    );
}

#[test]
fn no_prototype_writes_a_warning() {
    for prototype in [Prototype::Default, Prototype::Deterministic] {
        for profile in PROFILE_NAMES {
            let out = written(prototype, profile);
            assert!(
                !out.contains("cargo::warning"),
                "{prototype:?} {profile}: {out}"
            );
        }
    }
}

#[test]
fn a_prototype_hands_back_the_build_it_resolved() {
    let mut out = String::new();
    let default = Prototype::Default
        .configure(unwrapped("release"), &mut out)
        .expect("writing to a String cannot fail");
    assert_eq!(default.profile, Profile::Release);
    assert_eq!(default.pragmas, Profile::Release.default_pragmas());

    let deterministic = Prototype::Deterministic
        .configure(unwrapped("release"), &mut out)
        .expect("writing to a String cannot fail");
    assert_eq!(deterministic.profile, Profile::Release);
    assert!(!deterministic.fast_math());
    // MathPeephole stands on FastMath, so the resolved set drops it too.
    assert!(!deterministic.pragmas.contains(Pragma::MathPeephole));
    assert!(deterministic.pragmas.contains(Pragma::LoopOptimization));

    let dev = Prototype::Default
        .configure(unwrapped("debug"), &mut out)
        .expect("writing to a String cannot fail");
    assert_eq!(dev.profile, Profile::Dev);
}

/// Over every profile's default set: `Default` leaves it as it is, and
/// `Deterministic` differs from it in `FastMath` and in nothing else.
#[test]
fn tune_takes_fast_math_out_for_deterministic_alone() {
    for profile in PROFILES {
        let defaults = profile.default_pragmas();
        assert_eq!(Prototype::Default.tune(defaults), defaults, "{profile:?}");
        let tuned = Prototype::Deterministic.tune(defaults);
        for pragma in PRAGMAS {
            let expected = match pragma {
                Pragma::FastMath => false,
                _ => defaults.contains(pragma),
            };
            assert_eq!(tuned.contains(pragma), expected, "{profile:?} {pragma:?}");
        }
        assert_eq!(
            tuned.parallel_codegen_units(),
            defaults.parallel_codegen_units(),
            "{profile:?}"
        );
    }
    assert!(
        PROFILES
            .iter()
            .any(|profile| profile.default_pragmas().contains(Pragma::FastMath)),
        "no profile has FastMath, so the loop above took nothing out"
    );
}

#[test]
fn tune_runs_before_the_program_does() {
    const RELEASE: PragmaSet = Profile::Release.default_pragmas();
    const TUNED: PragmaSet = Prototype::Deterministic.tune(RELEASE);
    assert!(RELEASE.contains(Pragma::FastMath));
    assert!(!TUNED.contains(Pragma::FastMath));
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
    for prototype in [Prototype::Default, Prototype::Deterministic] {
        assert!(
            prototype
                .configure(unwrapped("release"), &mut Refusing)
                .is_err()
        );
    }
}
