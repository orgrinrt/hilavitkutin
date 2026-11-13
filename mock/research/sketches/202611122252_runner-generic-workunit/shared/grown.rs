//! Included by each arm's growth check, which names the arm's crates `unit`, `vocab` and
//! `grow`. The sixth runner ends on the bytes and events of the model with the fourth
//! kernel's kick, and the kick moves the world.

use crate::model::{bytes, events, init, model, model_with, store};

fn kick(tick: u32, row: usize) -> i32 {
    (rg_plumb::draw(tick, row as u32) % 3) as i32 - 1
}

#[test]
fn the_sixth_runner_ends_on_the_grown_models_bytes_and_events() {
    let s = init();
    let m = model_with(&s, kick);
    let o = grow::runner::run(&s);
    assert!(
        bytes(&o.state) == bytes(&store(&s, &m)),
        "state bytes differ from the grown model"
    );
    assert_eq!(events(&o), m.events, "events differ from the grown model");
}

#[test]
fn the_fourth_kernel_moves_the_world() {
    let s = init();
    let grown = store(&s, &model_with(&s, kick));
    let base = store(&s, &model(&s));
    assert!(bytes(&grown) != bytes(&base), "the kick changes the state");
    let kicks: std::collections::BTreeSet<i32> = (0 .. 64).map(|r| kick(0, r)).collect();
    assert_eq!(kicks.len(), 3, "the draw reaches -1, 0 and 1");
}
