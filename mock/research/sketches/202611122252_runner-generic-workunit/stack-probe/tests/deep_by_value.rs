//! A by-value clone of the 64-deep registry product recurses with a frame per level
//! holding the rest of the product, and overflows a thread's default stack; the in-place
//! copy does not. The overflow holds for the debug profile; in release the clone runs
//! on the default stack, so that test is ignored there. Each case runs in a child process, since an overflow aborts it.

use std::hint::black_box;
use std::process::{Command, Output};
use std::thread;

use e2_unit::{Heat, Lag, Pos, Vel};
use rg_owed::Project;
use rg_plumb::{N, registry};

type Reg = registry!(Pos, Vel, Lag, Heat);

/// The stack a spawned thread gets unless it asks, and the one a test runs on.
const DEFAULT: usize = 2 << 20;
const LARGE: usize = 16 << 20;

fn on_stack(bytes: usize, f: fn()) {
    thread::Builder::new()
        .stack_size(bytes)
        .spawn(f)
        .unwrap()
        .join()
        .unwrap();
}

fn by_value() {
    let a = Reg::made::<N>(&Default::default());
    black_box(black_box(&a).clone());
}

fn in_place() {
    let a = Reg::made::<N>(&Default::default());
    let mut b = Reg::made::<N>(&Default::default());
    Reg::copy_from::<N>(&mut b, black_box(&a));
    black_box(&b);
}

#[test]
#[ignore = "run by the tests below, in a child process"]
fn child() {
    match std::env::var("STACK_PROBE").as_deref() {
        Ok("by_value") => on_stack(DEFAULT, by_value),
        Ok("by_value_large") => on_stack(LARGE, by_value),
        Ok("in_place") => on_stack(DEFAULT, in_place),
        other => panic!("no such case: {other:?}"),
    }
}

fn run(case: &str) -> Output {
    Command::new(std::env::current_exe().unwrap())
        .args(["child", "--ignored", "--exact", "--nocapture"])
        .env("STACK_PROBE", case)
        .output()
        .unwrap()
}

#[test]
#[cfg_attr(
    not(debug_assertions),
    ignore = "holds for the debug profile only: in release the clone runs on the default stack"
)]
fn a_by_value_clone_overflows_the_default_stack() {
    let o = run("by_value");
    let err = String::from_utf8_lossy(&o.stderr);
    assert!(!o.status.success(), "the clone ran on the default stack");
    assert!(err.contains("has overflowed its stack"), "{err}");
}

#[test]
fn a_by_value_clone_runs_on_a_large_stack() {
    let o = run("by_value_large");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}

#[test]
fn an_in_place_copy_runs_on_the_default_stack() {
    let o = run("in_place");
    assert!(o.status.success(), "{}", String::from_utf8_lossy(&o.stderr));
}
