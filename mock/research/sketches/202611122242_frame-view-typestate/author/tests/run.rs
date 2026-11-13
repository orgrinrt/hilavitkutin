//! The kernels and the close unit run on the reference runner, and the view is
//! read from two scoped threads at once.

use author::{Cool, Count, Drift, Heat, Landings, Pos, Vel, World};
use engine::{Block, BlockKernel, Frame, Staged, Value, View};
use notko_hlist::{Cons, Empty};

#[test]
fn a_block_and_a_value_kernel_step_together_and_the_close_counts() {
    let mut frame: Frame<World, 4> = Frame::new();
    frame.step2(Block(Drift), Value(Cool));
    frame.close(Count);
    frame.step2(Block(Drift), Value(Cool));
    frame.close(Count);

    assert_eq!(frame.committed(Pos), &[6, 6, 6, 6]);
    assert_eq!(frame.committed(Vel), &[3, 3, 3, 3]);
    assert_eq!(frame.committed(Heat), &[16, 16, 16, 16]);
    assert_eq!(frame.committed(Landings), &[2, 2, 2, 2]);
}

#[test]
fn what_a_step_staged_is_committed_and_the_next_step_reads_it() {
    // Drift reads Pos and stages Pos: the first step reads the initial 0, the
    // second reads the 3 the first committed.
    let mut frame: Frame<World, 2> = Frame::new();
    frame.step(Block(Drift));
    assert_eq!(frame.committed(Pos), &[3, 3]);
    frame.step(Block(Drift));
    assert_eq!(frame.committed(Pos), &[6, 6]);
}

/// Reads the committed view from two scoped threads at once and stages the
/// sum, so the test sees both reads happened.
struct ReadTwice;

impl BlockKernel<World> for ReadTwice {
    type Reads = Cons<Heat, Cons<Vel, Empty>>;
    type Writes = Cons<Pos, Empty>;

    fn run<'s>(view: View<'s, World, Self::Reads>, out: Staged<'s, Self::Writes>) {
        let (mut pos, ()) = out;
        for r in view.rows() {
            let (heat, vel) = std::thread::scope(|s| {
                let a = s.spawn(|| view.get(Heat, r));
                let b = s.spawn(|| view.get(Vel, r));
                (a.join().unwrap(), b.join().unwrap())
            });
            pos.set(r, heat + vel);
        }
    }
}

#[test]
fn the_committed_view_is_read_from_two_scoped_threads_at_once() {
    let mut frame: Frame<World, 3> = Frame::new();
    frame.step(Block(ReadTwice));
    assert_eq!(frame.committed(Pos), &[67, 67, 67]);
}
