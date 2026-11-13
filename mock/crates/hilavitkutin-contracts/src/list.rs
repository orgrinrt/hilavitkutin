//! The list vocabulary: notko-hlist's type-level list, and the value projection over a list of
//! members.
//!
//! `Values` is local until notko step NK1 lands a projection upstream, and is removed here then.

pub use notko_hlist::{Cons, Empty, Here, List, There};

use crate::domain::Member;

/// The projection from a list of members to a nested pair of their values, `(A, (B, ()))`,
/// which an author destructures and never names by position.
///
/// Local until notko step NK1; removed here when it lands upstream.
// FIXME(33d72d28dd71): a local stand-in for the projection notko owes at NK1.
pub trait Values: List {
    /// The nested pair of the members' values, in list order.
    type Of: Copy + Send + Sync + 'static;
}

impl Values for Empty {
    type Of = ();
}

impl<H: Member, T: Values> Values for Cons<H, T> {
    type Of = (H::Value, T::Of);
}
