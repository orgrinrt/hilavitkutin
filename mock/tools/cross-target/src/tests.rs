//! What the comparison says of two streams, and the whole path once: the
//! probe built and run for the host and for wasm, as it is and skewed.

use std::path::Path;

use mockspace::tool::{NotALint, Outcome, Purpose, Tool as _, ToolContext};

use super::{CrossTarget, TARGETS, both, differences, probe, stream};

const WASM: (&str, &str, &str) = TARGETS[0];

fn tool() -> &'static Path {
    Path::new(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn a_stream_is_a_digest_a_line_and_nothing_else() {
    assert_eq!(
        stream("00000000000000ff\nb7e412fb73e79f92\n"),
        Some(vec![0xFF, 0xB7E4_12FB_73E7_9F92])
    );
    assert_eq!(stream(""), None, "no line is no stream");
    assert_eq!(stream("ff\n"), None, "a short line is not a digest");
    assert_eq!(
        stream("00000000000000ff\nwarning: unused\n"),
        None,
        "one line that is no digest and the whole output is refused"
    );
    assert_eq!(stream("+0000000000000ff\n"), None, "a sign is no hex digit");
}

#[test]
fn the_same_stream_twice_differs_nowhere() {
    assert!(differences(&[1, 2, 3], &[1, 2, 3], "there").is_empty());
}

#[test]
fn a_frame_that_differs_is_named_by_its_number() {
    let said = differences(&[1, 2, 3], &[1, 9, 3], "there");
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(said[0].starts_with("frame 2:"), "{said:?}");
    assert!(
        said[0].contains("0000000000000002") && said[0].contains("0000000000000009"),
        "{said:?}"
    );
}

#[test]
fn a_shorter_stream_differs_though_every_shared_frame_agrees() {
    let said = differences(&[1, 2, 3], &[1, 2], "there");
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(
        said[0].contains("3 frames") && said[0].contains("is 2"),
        "{said:?}"
    );
}

#[test]
fn the_wasm_build_is_run_by_the_runner_beside_the_tool() {
    let call = probe(Path::new("/t"), Some(WASM), &["skew"]);
    let args: Vec<String> = call
        .get_args()
        .map(|a| a.to_string_lossy().into_owned())
        .collect();
    assert_eq!(args, [
        "run",
        "--quiet",
        "--manifest-path",
        "/t/probe/Cargo.toml",
        "--features",
        "skew",
        "--target",
        "wasm32-unknown-unknown",
        "--config",
        "target.wasm32-unknown-unknown.runner=['node', '/t/run.cjs']",
    ]);
    let host = probe(Path::new("/t"), None, &[]);
    assert_eq!(
        host.get_args().count(),
        4,
        "the host's build names no target"
    );
}

#[test]
fn a_target_nothing_here_runs_is_inconclusive_and_never_clean() {
    let crates = std::collections::BTreeSet::new();
    let dirs: Vec<std::path::PathBuf> = Vec::new();
    let args = ["riscv64gc-unknown-linux-gnu"];
    let registry = mockspace::RegistryView::default();
    let ctx = ToolContext {
        mock_dir:   Path::new("/nowhere"),
        repo_root:  Path::new("/nowhere"),
        all_crates: &crates,
        src_dirs:   &dirs,
        args:       &args,
        stdin:      None,
        registry:   &registry,
    };
    let report = CrossTarget.run(&ctx);
    let Outcome::Inconclusive {
        reason,
    } = report.outcome
    else {
        panic!("{:?}", report.outcome);
    };
    assert!(reason.contains("wasm32-unknown-unknown"), "{reason}");
    assert_eq!(
        CrossTarget.purpose(),
        Purpose::Check(NotALint::TakesAQuestion)
    );
    assert!(CrossTarget.args().iter().any(|a| a.required));
}

#[test]
fn the_host_and_wasm_leave_the_same_stream() {
    let (host, wasm) = both(tool(), WASM, &[]).unwrap_or_else(|why| panic!("{why}"));
    assert_eq!(host.len(), 8, "a digest for each frame");
    assert!(
        host.windows(2).all(|pair| pair[0] != pair[1]),
        "the frames differ from each other, so the stream is no constant: {host:x?}"
    );
    assert_eq!(differences(&host, &wasm, WASM.0), Vec::<String>::new());
}

#[test]
fn a_wasm_build_that_computes_otherwise_is_told_apart() {
    let (host, wasm) = both(tool(), WASM, &["skew"]).unwrap_or_else(|why| panic!("{why}"));
    let said = differences(&host, &wasm, WASM.0);
    assert_eq!(said.len(), 1, "{said:?}");
    assert!(said[0].starts_with("frame 8:"), "{said:?}");
}
