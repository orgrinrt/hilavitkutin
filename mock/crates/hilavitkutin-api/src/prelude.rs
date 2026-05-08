//! Convenient re-exports for hilavitkutin-api consumers.
//!
//! `use hilavitkutin_api::prelude::*;` brings in the cons-list
//! typestate primitives, the construction macros, the schedule
//! plus WorkUnit traits, the store markers, and the membership
//! witnesses needed by typical consumer code.
//!
//! Provider-side and platform-contract traits stay out of the
//! prelude; consumers that need them name them directly to keep
//! their import set self-documenting.

pub use crate::access::{AccessSet, Concat, Cons, Contains, ContainsAll, Empty};
pub use crate::store::{Column, Replaceable, Resource, StoreBundle, Virtual};
pub use crate::work_unit::{Always, On, WorkUnit, WorkUnitBundle};
pub use crate::{read, write};
