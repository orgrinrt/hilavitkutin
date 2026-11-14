//! The pieces a bench member's arms share when every arm reads one column of
//! u32 records and is compared by a digest of what it writes.
//!
//! A member defines its workload, its arms and its routine. What it does not
//! define again is how the input image is laid out and seeded (`image`), how a
//! digest is taken (`Fnv1a`), and how an arm holding a built scheduler stays
//! alive across the calls of one worker process (`arm`). Those are the same for
//! every such member, and a second copy of any of them is one more place for
//! two members to drift apart while both still pass.
//!
//! std and alloc are allowed here: this is bench tooling outside the no_std
//! crates, as every bench member is.

pub mod arm;
pub mod findings;
pub mod image;

pub use arm::{Arm, StateArm, call};
pub use image::{
    Fnv1a,
    MAX_CALL_US_FLOOR,
    TaggedRecords,
    fnv1a_by,
    fnv1a_u32,
    tagged_bytes,
    tagged_records,
};
