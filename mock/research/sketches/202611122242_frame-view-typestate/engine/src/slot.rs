//! The count a lane's place in the registry is read back in.

use notko_hlist::Cardinal;

/// How far into the registry a lane sits, as `notko-hlist`'s
/// `Position<Slot>::INDEX` reads it back.
///
/// The field is a bare `u32` because the sketch may depend on `notko` and
/// `notko-hlist` only; the production form takes arvo's count.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Slot(pub u32);

const impl Cardinal for Slot {
    const ZERO: Self = Slot(0);

    fn succ(self) -> Self {
        Slot(self.0 + 1)
    }
}
