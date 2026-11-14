//! Each arm runs in a child process with a body that panics on a spawned
//! thread, and the parent asserts the child died by abort within a bound. The
//! control is an unguarded entry: its thread dies, the join reports it, and the
//! child exits normally, which is the case the engine cannot survive.

use std::os::unix::process::ExitStatusExt;
use std::process::Command;
use std::time::{Duration, Instant};

use no_unwind_worker_boundary::{Ctx, entry_extern_c, entry_guard};

fn panics(_: &u32) {
    panic!("a unit body panicked");
}

fn unguarded(ctx: *const Ctx<u32>) {
    // SAFETY: as the arms.
    let ctx = unsafe { &*ctx };
    (ctx.body)(&ctx.value);
}

/// In the child: run the entry `which` names on a worker thread and join it.
fn child(which: &str) {
    let ctx = Box::leak(Box::new(Ctx {
        body:  panics,
        value: 7u32,
    })) as *const Ctx<u32> as usize;
    let entry: fn(usize) = match which {
        "extern-c" => |p| entry_extern_c::<u32>(p as *const _),
        "guard" => |p| entry_guard::<u32>(p as *const _),
        _ => |p| unguarded(p as *const _),
    };
    let joined = std::thread::spawn(move || entry(ctx)).join();
    println!("joined: {}", joined.is_ok());
    std::process::exit(0);
}

/// Run this test binary as a child on `which`: its exit code, whether it died
/// of SIGABRT, and how long it took.
fn run(which: &str) -> (Option<i32>, bool, Duration) {
    let started = Instant::now();
    let out = Command::new(std::env::current_exe().unwrap())
        .args(["--exact", "child_entry", "--nocapture", "--test-threads=1"])
        .env("BOUNDARY_ARM", which)
        .output()
        .unwrap();
    (
        out.status.code(),
        out.status.signal() == Some(6),
        started.elapsed(),
    )
}

#[test]
fn child_entry() {
    if let Ok(which) = std::env::var("BOUNDARY_ARM") {
        child(&which);
    }
}

#[test]
fn the_extern_c_entry_aborts() {
    let (code, aborted, took) = run("extern-c");
    assert!(aborted && code.is_none(), "{code:?} {aborted}");
    assert!(took < Duration::from_secs(10));
}

#[test]
fn the_guard_entry_aborts() {
    let (code, aborted, took) = run("guard");
    assert!(aborted && code.is_none(), "{code:?} {aborted}");
    assert!(took < Duration::from_secs(10));
}

#[test]
fn an_unguarded_entry_unwinds_and_the_child_exits_normally() {
    let (code, aborted, _) = run("unguarded");
    assert!(!aborted && code == Some(0), "{code:?} {aborted}");
}
