//! ConvergenceBuffer + AccumulatorSlot combine tests.

use hilavitkutin::resource::{AccumulatorSlot, ConvergenceBuffer};

fn add(a: u32, b: u32) -> u32 {
    a.wrapping_add(b)
}

fn max(a: u32, b: u32) -> u32 {
    if a > b { a } else { b }
}

#[test]
fn accumulator_slot_constructs() {
    let s = AccumulatorSlot::new(42u32);
    assert_eq!(s.value, 42);
}

#[test]
fn convergence_buffer_default_is_zero() {
    let b: ConvergenceBuffer<u32, 4> = ConvergenceBuffer::new(0);
    for i in 0..4 {
        assert_eq!(b.get(i), 0);
    }
}

#[test]
fn convergence_buffer_combine_addition() {
    let mut b: ConvergenceBuffer<u32, 4> = ConvergenceBuffer::new(0);
    b.set(0, 1);
    b.set(1, 2);
    b.set(2, 3);
    b.set(3, 4);
    assert_eq!(b.combine(0, add), 10);
}

#[test]
fn convergence_buffer_combine_max() {
    let mut b: ConvergenceBuffer<u32, 4> = ConvergenceBuffer::new(0);
    b.set(0, 5);
    b.set(1, 2);
    b.set(2, 9);
    b.set(3, 1);
    assert_eq!(b.combine(0, max), 9);
}
