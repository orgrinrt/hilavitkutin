//! Nothing the driver inherits reaches the fixture's build: not a linker or a
//! profile setting in a variable, not a `RUSTC`, not a configuration in the
//! cargo home or in a directory above the fixture. Each is set here to a value
//! that breaks a build, a call is shown to build and link as the default arm
//! must, and each is then shown to break the same cargo command where it does
//! reach it.
//!
//! This binary sets its own process environment, so it holds one test.

use std::path::Path;
use std::process::Command;

use build_call::record::{self, Line};
use build_call::{Arm, Call, Dirs, Profile, Scenario, cargo, root, toolchain};

const BENCH: &str = "test-environment";
const ARM: &str = "default_relink";
const N: usize = 25;

fn put(path: &Path, text: &str) {
    std::fs::create_dir_all(path.parent().expect("a parent")).expect("the directory is made");
    std::fs::write(path, text).expect("the file is written");
}

/// The relink's cargo command for the cell, its executable removed first so
/// the build has to link.
fn relink_command() -> Command {
    let input = [Scenario::Link.code(), Profile::Dev.code(), 0, 0, 0, 0, 0, 0];
    drop(Call::ready(BENCH, ARM, Arm::Relink, &input, N));
    cargo::command(&Dirs::of(BENCH, ARM, N), false, &[
        "build",
        "-p",
        "fx_facade",
        "--message-format=json",
    ])
}

/// What a command printed to stderr, asserting it failed.
fn failed(mut c: Command, what: &str) -> String {
    let out = c.output().expect("cargo starts");
    let said = String::from_utf8_lossy(&out.stderr).into_owned();
    assert!(
        !out.status.success(),
        "{what} reached the build and did not break it: {said}"
    );
    said
}

#[test]
fn an_inherited_environment_and_configuration_do_not_reach_the_fixture_build() {
    let tc = toolchain::get();
    let bench_dir = root().join(BENCH);
    put(
        &bench_dir.join(".cargo/config.toml"),
        "[build]\nrustc-wrapper = \"/nonexistent/build-call-above\"\n",
    );
    let home = bench_dir.join("hostile-home");
    put(
        &home.join("config.toml"),
        "[build]\nrustc-wrapper = \"/nonexistent/build-call-home\"\n",
    );
    let linker_var = format!(
        "CARGO_TARGET_{}_LINKER",
        tc.host.to_uppercase().replace(['-', '.'], "_")
    );
    let hostile: [(&str, &str); 4] = [
        (&linker_var, "/nonexistent/build-call-linker"),
        ("RUSTC", "/nonexistent/build-call-rustc"),
        ("CARGO_BUILD_JOBS", "0"),
        ("CARGO_PROFILE_DEV_OPT_LEVEL", "9"),
    ];
    // SAFETY: this binary holds this one test, so no other thread reads the
    // environment while it is written.
    unsafe {
        std::env::set_var("CARGO_HOME", &home);
        for (k, v) in hostile {
            std::env::set_var(k, v);
        }
    }

    record::reset(&root(), BENCH);
    let input = [Scenario::Link.code(), Profile::Dev.code(), 0, 0, 0, 0, 0, 0];
    let mut call = Call::ready(BENCH, ARM, Arm::Relink, &input, N);
    let out = call.run();
    call.record(&out);
    let (lines, _): (Vec<Line>, _) = record::read(&root(), BENCH);
    let verdict = record::check(&lines, BENCH, Scenario::Link, Profile::Dev, N);
    assert!(verdict.is_ok(), "{verdict:?}");

    for (k, v) in hostile {
        let mut c = relink_command();
        c.env(k, v);
        failed(c, k);
    }
    let mut above = relink_command();
    above.current_dir(Dirs::of(BENCH, ARM, N).ws());
    assert!(
        failed(above, "a configuration above the fixture")
            .contains("/nonexistent/build-call-above")
    );
    let mut home_config = relink_command();
    home_config.env("CARGO_HOME", &home);
    assert!(
        failed(home_config, "the cargo home's configuration")
            .contains("/nonexistent/build-call-home")
    );
}
