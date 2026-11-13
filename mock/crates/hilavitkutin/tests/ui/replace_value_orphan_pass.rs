// A consumer opts its resource into replacement with a local marker trait, which
// the orphan rule allows. The twin `replace_value_orphan_fail.rs` differs only
// in the trait: `Replaceable`, the one `Scheduler::replace_value` bounds on,
// which is foreign to the consumer as `Resource` is, so the impl its
// diagnostic suggests is refused.

use hilavitkutin_api::store::Resource;

trait LocalReplaceable {}

#[derive(Copy, Clone)]
struct InA(u32);

impl LocalReplaceable for Resource<InA> {}

fn main() {}
