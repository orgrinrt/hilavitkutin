//! Included by each arm's check, which names the arm's crates `unit`, `vocab` and
//! `ordered` .. `registry`. Every runner ends on the same bytes and the same ordered
//! events, and those are the ones the plain model of the three kernels computes.

use rg_plumb::{ArrOf, N, Outcome};
use unit::{All, Crossed, SmoothReach};
use vocab::Reach;

use crate::model::{bytes, events, init, model, offset, store};

fn runs() -> Vec<(&'static str, Outcome<ArrOf<All, N>, Crossed>)> {
    let s = init();
    vec![
        ("ordered", ordered::run(&s)),
        ("permuted", permuted::run(&s)),
        ("fused", fused::run(&s)),
        ("threaded", threaded::run(&s)),
        ("registry", registry::run(&s)),
    ]
}

#[test]
fn every_runner_ends_on_the_models_bytes_and_events() {
    let s = init();
    let m = model(&s);
    let want = store(&s, &m);
    for (name, o) in runs() {
        assert!(
            bytes(&o.state) == bytes(&want),
            "{name}: state bytes differ from the model"
        );
        assert_eq!(events(&o), m.events, "{name}: events differ from the model");
    }
}

#[test]
fn the_world_exercises_every_kernel() {
    let m = model(&init());
    assert!(
        m.events.len() > 4,
        "the emitter fires: {} events",
        m.events.len()
    );
    assert!(
        m.events.windows(2).any(|w| w[0].0 != w[1].0),
        "events span ticks"
    );
    let offs: std::collections::BTreeSet<(i32, i32)> = m.lag.iter().map(|l| offset(*l)).collect();
    assert_eq!(
        offs.len(),
        ((2 * SmoothReach::EAST + 1) * (2 * SmoothReach::NORTH + 1)) as usize,
        "the window reaches every offset in its reach, on both axes"
    );
}

#[test]
fn a_wrong_kernel_is_caught() {
    let s = init();
    let o = ordered::run(&s);
    let mut m = model(&s);
    m.heat[7] ^= 1;
    let want = store(&s, &m);
    assert!(
        bytes(&o.state) != bytes(&want),
        "the byte comparison sees a one-bit difference"
    );
}
