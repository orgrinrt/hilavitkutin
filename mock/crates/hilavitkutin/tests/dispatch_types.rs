//! Dispatch-stage type tests.
//!
//! Only the behaviour a dispatch type derives from its inputs is pinned
//! here: a morsel range's end and emptiness are computed, not stored.

use arvo::{Additive, Bool, Identity, USize};
use hilavitkutin::dispatch::MorselRange;

#[test]
fn morsel_range_new_end_is_empty() {
    let r = MorselRange::new(USize(100), USize(16)); // lint:allow(no-bare-numeric) reason: morsel range literals; tracked: #399
    assert_eq!(r.end(), USize(116)); // lint:allow(no-bare-numeric) reason: end-offset check; tracked: #399
    assert_eq!(r.is_empty(), Bool::FALSE);

    let empty = MorselRange::new(
        <USize as Identity<Additive>>::IDENTITY,
        <USize as Identity<Additive>>::IDENTITY,
    );
    assert_eq!(empty.is_empty(), Bool::TRUE);
    assert_eq!(empty.end(), <USize as Identity<Additive>>::IDENTITY);
}
