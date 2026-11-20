//! A timed build that waited on another process's lock is seen in its record
//! and refused: the cell's build locks are held here while the call starts,
//! and let go once cargo says it is waiting.

mod common;

use std::fs::File;
use std::io::{BufRead, BufReader, Read};
use std::process::Output;
use std::sync::mpsc;
use std::time::Duration;

use build_call::{Call, Dirs, Profile, Scenario, record, root};
use common::Removed;

const BENCH: &str = "test-lock";
const ARM: &str = "control";
const N: usize = 25;

#[test]
fn a_call_that_waited_on_a_lock_is_recorded_as_waiting_and_refused() {
    let _gone = Removed::bench(BENCH);
    let input = [Scenario::Leaf.code(), Profile::Dev.code(), 0, 0, 0, 0, 0, 0];
    let mut call = Call::ready(BENCH, ARM, common::off(), &input, N);

    let profile_dir = Dirs::of(BENCH, ARM, N).target().join(Profile::Dev.dir());
    let held: Vec<File> = [".cargo-lock", ".cargo-build-lock", ".cargo-artifact-lock"]
        .iter()
        .filter_map(|name| {
            let f = File::options()
                .read(true)
                .write(true)
                .create(true)
                .truncate(false)
                .open(profile_dir.join(name))
                .ok()?;
            f.lock().ok()?;
            Some(f)
        })
        .collect();
    assert!(
        !held.is_empty(),
        "no build lock of {} could be held",
        profile_dir.display()
    );

    let mut child = call.spawn().expect("the timed command starts");
    let mut stdout = child.stdout.take().expect("piped stdout");
    let stderr = child.stderr.take().expect("piped stderr");
    let out_reader = std::thread::spawn(move || {
        let mut text = Vec::new();
        let _ = stdout.read_to_end(&mut text);
        text
    });
    let (seen, waiting) = mpsc::channel();
    let err_reader = std::thread::spawn(move || {
        let mut text = Vec::new();
        for line in BufReader::new(stderr).split(b'\n').map_while(Result::ok) {
            if String::from_utf8_lossy(&line).contains("Blocking") {
                let _ = seen.send(());
            }
            text.extend_from_slice(&line);
            text.push(b'\n');
        }
        text
    });
    let blocked = waiting.recv_timeout(Duration::from_secs(120)).is_ok();
    drop(held);
    let status = child.wait().expect("the build ends");
    let out = Output {
        status,
        stdout: out_reader.join().expect("stdout read"),
        stderr: err_reader.join().expect("stderr read"),
    };
    assert!(
        blocked,
        "cargo never said it was waiting while the locks were held: {}",
        String::from_utf8_lossy(&out.stderr)
    );
    call.record(&out);
    let (lines, _) = record::read(&root(), BENCH);
    let line = lines.last().expect("the call appended a line");
    assert!(line.waited, "the record missed the wait: {line:?}");
    let why = record::check(
        std::slice::from_ref(line),
        BENCH,
        Scenario::Leaf,
        Profile::Dev,
        N,
    )
    .expect_err("a call that waited on a lock must be refused");
    assert!(why.contains("cargo waited on a lock"), "{why}");
}
