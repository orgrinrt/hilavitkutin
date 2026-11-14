//! The three drivers of the engine plan's job 4, run on every arm: two
//! consecutive frames; a frame, `replace_value`, a frame; a frame,
//! `replace_resource`, a frame. Natively each holds the outputs; under Miri
//! each says whether the arm stays sound across what happens between frames.

use core::pin::pin;

use parked_worker_plane::{Arm, Frame, Narrow, RECORDS, Today, plane, program};

const COLD: Frame = Frame {
    out:  [51; RECORDS],
    band: 1,
};

fn two_frames<A: Arm>() {
    let mut s = pin!(A::build());
    assert_eq!(s.as_mut().run_parallel(), COLD);
    assert_eq!(s.as_mut().run_parallel(), Frame {
        out:  [50; RECORDS],
        band: 0,
    });
}

fn frame_replace_value_frame<A: Arm>() {
    let mut s = pin!(A::build());
    assert_eq!(s.as_mut().run_parallel(), COLD);
    // SAFETY: the scheduler is not moved out of its pin.
    unsafe { s.as_mut().get_unchecked_mut() }.replace_value(6);
    assert_eq!(s.as_mut().run_parallel(), Frame {
        out:  [60; RECORDS],
        band: 0,
    });
}

fn frame_replace_resource_frame<A: Arm>() {
    let mut s = pin!(A::build());
    assert_eq!(s.as_mut().run_parallel(), COLD);
    // SAFETY: as above.
    unsafe { s.as_mut().get_unchecked_mut() }.replace_resource(7);
    assert_eq!(s.as_mut().run_parallel(), Frame {
        out:  [71; RECORDS],
        band: 1,
    });
}

macro_rules! drivers {
    ($arm:ident, $ty:ty) => {
        mod $arm {
            #[test]
            fn two_frames() {
                super::two_frames::<$ty>();
            }
            #[test]
            fn frame_replace_value_frame() {
                super::frame_replace_value_frame::<$ty>();
            }
            #[test]
            fn frame_replace_resource_frame() {
                super::frame_replace_resource_frame::<$ty>();
            }
        }
    };
}

drivers!(today, crate::Today);
drivers!(narrow, crate::Narrow);
drivers!(plane_arm, crate::plane::Sched);
drivers!(program_arm, crate::program::Sched);
