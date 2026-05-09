//! ConvergenceBuffer + AccumulatorSlot combine tests.

use arvo::{Identity, USize};
use hilavitkutin::resource::{AccumulatorSlot, ConvergenceBuffer};

fn add(a: u32, b: u32) -> u32 { // lint:allow(no-bare-numeric) reason: combine fn signature uses bare u32 payload; tracked: #399
    a.wrapping_add(b)
}

fn max(a: u32, b: u32) -> u32 { // lint:allow(no-bare-numeric) reason: combine fn signature uses bare u32 payload; tracked: #399
    if a > b { a } else { b }
}

#[test]
fn accumulator_slot_constructs() {
    let s = AccumulatorSlot::new(42u32); // lint:allow(no-bare-numeric) reason: payload literal; tracked: #399
    assert_eq!(s.value, 42); // lint:allow(no-bare-numeric) reason: roundtrip check; tracked: #399
}

#[test]
fn convergence_buffer_default_is_zero() {
    let b: ConvergenceBuffer<u32, 4> = ConvergenceBuffer::new(0); // lint:allow(no-bare-numeric) reason: payload zero literal; tracked: #399
    for i in 0..4 {
        assert_eq!(b.get(USize(i)), 0); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    }
}

#[test]
fn convergence_buffer_combine_addition() {
    let mut b: ConvergenceBuffer<u32, 4> = ConvergenceBuffer::new(0); // lint:allow(no-bare-numeric) reason: payload zero literal; tracked: #399
    b.set(USize(0), 1); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    b.set(USize(1), 2); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    b.set(USize(2), 3); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    b.set(USize(3), 4); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    assert_eq!(b.combine(0, add), 10); // lint:allow(no-bare-numeric) reason: payload literal; tracked: #399
}

#[test]
fn convergence_buffer_combine_max() {
    let mut b: ConvergenceBuffer<u32, 4> = ConvergenceBuffer::new(0); // lint:allow(no-bare-numeric) reason: payload zero literal; tracked: #399
    b.set(USize(0), 5); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    b.set(USize(1), 2); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    b.set(USize(2), 9); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    b.set(USize(3), 1); // lint:allow(no-bare-numeric) reason: index + payload literal; tracked: #399
    assert_eq!(b.combine(0, max), 9); // lint:allow(no-bare-numeric) reason: payload literal; tracked: #399
}

#[allow(dead_code)]
const _IDENTITY_USED: USize = USize::ZERO;
