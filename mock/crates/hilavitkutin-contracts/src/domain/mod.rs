//! What an author names: a domain of records, its columns and resources, and the members of a
//! read list.

mod member;
pub(crate) mod value;

pub use member::{Member, Near, Own, Param, ReadMember, Reads, Roles, WriteMember, Writes};
use notko::Maybe;
pub use value::{Cell, Cell8, Cell16, Cell32, Cell64, FormatId, Value, Word};

/// A set of records, each with a key, in a key order.
///
/// The shape is the domain's run-time extent, and the store owns it: one store holds one shape.
/// The domain says how to walk its keys in order from a shape and where a step from a key lands.
/// Everything a store needs to lay out the records and answer a neighbour read by key comes from
/// these three functions, so a store is written once for every domain.
pub trait Domain: 'static {
    /// The canonical record key; key order is its `Ord`.
    type Key: Copy + Ord + Send + Sync + 'static;
    /// An offset between keys, one component per axis.
    type Step: Copy + Send + Sync + 'static;
    /// The run-time extent a store holds, per axis.
    type Shape: Copy + Send + Sync + 'static;

    /// The first key of `shape` in key order, or nothing for an empty shape.
    fn first(shape: Self::Shape) -> Maybe<Self::Key>;
    /// The key after `key` in key order, or nothing after the last.
    fn next(shape: Self::Shape, key: Self::Key) -> Maybe<Self::Key>;
    /// The key `step` away from `key`, or nothing outside `shape`.
    fn reach(shape: Self::Shape, key: Self::Key, step: Self::Step) -> Maybe<Self::Key>;
}

/// A named offset in a domain, declared by the domain's author, read by a `Near` member.
pub trait Offset: 'static {
    /// The domain the offset is in.
    type Domain: Domain;
    /// Its components, plain data.
    const STEP: <Self::Domain as Domain>::Step;
}

/// A column of a domain: a value per record. Named in a kernel, it is a role; which stored column
/// fills the role is data, supplied by a binding.
pub trait Column: 'static {
    /// The domain whose records it holds a value for.
    type Domain: Domain;
    /// What it holds.
    type Value: Value;
    /// What a neighbour read outside the shape reads. The first boundary policy, unlocked.
    const OUTSIDE: Self::Value;
}

/// A resource: one value, constant over a step. Read as a `Param` member, its value supplied per
/// binding.
pub trait Resource: 'static {
    /// What it holds.
    type Value: Value;
}
