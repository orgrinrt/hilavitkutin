//! Convenient re-exports for hilavitkutin-api consumers.
//!
//! `use hilavitkutin_api::prelude::*;` brings in the list types and the
//! membership bounds, the store markers and the members over them, the
//! schedule and WorkUnit traits, and the construction macros a typical
//! unit or kit author needs.
//!
//! Provider-side and platform-contract traits stay out of the
//! prelude; consumers that need them name them directly to keep
//! their import set self-documenting.

pub use crate::access::{AccessSet, Concat, Cons, Contains, ContainsAll, Empty, Holds, HoldsAll};
pub use crate::builder_input::ExtensionSurface;
pub use crate::member::{Differs, Member, Reach, Written};
pub use crate::store::{
    Accum,
    Buffered,
    Column,
    Exchange,
    Replaceable,
    Resource,
    Snapshot,
    StartsFromDefault,
    StoreBundle,
    Virtual,
};
pub use crate::work_unit::{Always, On, OnMeta, WorkUnit, WorkUnitBundle};
pub use crate::{read, rec, write};
