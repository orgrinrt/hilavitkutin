// A consumer implements `Replaceable`, the trait `Scheduler::replace_value`
// bounds on, for a `Resource` it registered. Both the trait and `Resource` are
// defined outside the consumer, so the orphan rule refuses the impl that the
// diagnostic of `replace_value_unreachable_fail.rs` suggests. The twin
// `replace_value_orphan_pass.rs` differs only in the trait: a local marker.

use hilavitkutin_api::store::{Replaceable, Resource};

#[derive(Copy, Clone)]
struct InA(u32);

impl Replaceable for Resource<InA> {}

fn main() {}
