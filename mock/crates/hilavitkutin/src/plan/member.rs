//! What the plan reads of a member beside the store it names.
//!
//! A member's store gives the bit it sets in an access mask. Two more things
//! decide the edges of step 1 and the phase boundaries: whether a read through
//! the member waits for the store's writers of the pass, and whether it reads a
//! plain column at the records to either side of the one it stands on.

use arvo::{Bool, Cap};
use hilavitkutin_api::accum_overflow::AccumOverflow;
use hilavitkutin_api::builder_input::ExtensionSurface;
use hilavitkutin_api::member::Member;
use hilavitkutin_api::run_cfg::DefaultRunCfg;
use hilavitkutin_api::store::{Accum, BufferMode, Buffered, Column, Resource, Virtual};
use hilavitkutin_api::{Differs, Reach, Written};

/// The plan's reading of one member of an access set.
///
/// Implemented for every member the api has, each stating its own two facts.
/// The impls are of distinct concrete types, so none overlaps another.
pub trait PlanMember: Member {
    /// Whether a read through this member waits for the writers of its store
    /// in the same pass. False for every member over a buffered column, whose
    /// reads see the side the last boundary froze.
    const WAITS: Bool;

    /// Whether this member reads a plain column with a reach, which makes the
    /// read depend on records other than its own.
    const REACHES: Bool;
}

macro_rules! plan_member {
    ($([$($gen:tt)*] $member:ty => waits $waits:ident, reaches $reaches:ident;)*) => {
        $(
            impl<$($gen)*> PlanMember for $member {
                const WAITS: Bool = Bool::$waits;
                const REACHES: Bool = Bool::$reaches;
            }
        )*
    };
}

plan_member! {
    [T: 'static] Resource<T> => waits TRUE, reaches FALSE;
    [T: 'static] Column<T> => waits TRUE, reaches FALSE;
    [T: 'static] Accum<T> => waits TRUE, reaches FALSE;
    [T: 'static] Virtual<T> => waits TRUE, reaches FALSE;
    [T: ?Sized + 'static] ExtensionSurface<T> => waits TRUE, reaches FALSE;
    [O: AccumOverflow + 'static] DefaultRunCfg<O> => waits TRUE, reaches FALSE;
    [T: 'static, M: BufferMode] Buffered<T, M> => waits FALSE, reaches FALSE;
    [T: 'static, const N: Cap] Reach<Column<T>, N> => waits TRUE, reaches TRUE;
    [T: 'static, M: BufferMode, const N: Cap] Reach<Buffered<T, M>, N> => waits FALSE, reaches FALSE;
    [T: 'static] Written<Column<T>> => waits TRUE, reaches FALSE;
    [T: 'static, M: BufferMode] Written<Buffered<T, M>> => waits FALSE, reaches FALSE;
    [T: PartialEq + 'static] Differs<Column<T>> => waits TRUE, reaches FALSE;
    [T: PartialEq + 'static, M: BufferMode] Differs<Buffered<T, M>> => waits FALSE, reaches FALSE;
}
