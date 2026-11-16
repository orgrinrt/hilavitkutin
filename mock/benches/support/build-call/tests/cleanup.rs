//! A test bench's state is removed when its test ends, whether it passes or
//! panics.

mod common;

use build_call::root;
use common::Removed;

fn fill(bench: &str) {
    let dir = root().join(bench).join("arm/n25/target");
    std::fs::create_dir_all(&dir).expect("the directory is made");
    std::fs::write(dir.join("left"), b"x").expect("the file is written");
}

#[test]
fn a_bench_is_removed_when_its_guard_ends_and_when_its_test_panics() {
    let ended = "test-cleanup-ended";
    {
        let _gone = Removed::bench(ended);
        fill(ended);
        assert!(root().join(ended).exists());
    }
    assert!(!root().join(ended).exists(), "left after the guard ended");

    let panicked = "test-cleanup-panicked";
    let caught = std::panic::catch_unwind(|| {
        let _gone = Removed::bench(panicked);
        fill(panicked);
        panic!("the test fails");
    });
    assert!(caught.is_err());
    assert!(
        !root().join(panicked).exists(),
        "left after the test panicked"
    );

    let earlier = "test-cleanup-earlier";
    fill(earlier);
    let _gone = Removed::bench(earlier);
    assert!(
        !root().join(earlier).exists(),
        "a run before left its state"
    );
}
