//! The host-populated input column the two column pipelines read.

use hilavitkutin_api::access::{Cons, Empty};
use hilavitkutin_api::store::Column;

#[derive(Copy, Clone, Default)]
pub struct Inv(pub u32);

pub type OneIn = Cons<Column<Inv>, Empty>;
