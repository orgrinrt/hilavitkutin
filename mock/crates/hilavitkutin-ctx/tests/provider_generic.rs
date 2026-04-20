//! Parameterised provider macro — accessor trait with a type
//! parameter carries through generic-method calls.

#![no_std]

use hilavitkutin_ctx::{provider_generic, provider_generic2};

// --- Single-parameter variant ------------------------------------------

/// Toy sealed trait standing in for AccessSet.
trait Tag: 'static {}
struct A;
struct B;
impl Tag for A {}
impl Tag for B {}

/// Toy API trait parameterised over a Tag.
trait Labeller<R: Tag> {
    fn label(&self) -> u8;
}

provider_generic!(<R: Tag> Labeller as HasLabel => label);

struct LabelA;
impl Labeller<A> for LabelA {
    fn label(&self) -> u8 {
        0xA
    }
}

struct LabelB;
impl Labeller<B> for LabelB {
    fn label(&self) -> u8 {
        0xB
    }
}

/// Consumer-defined Ctx type (newtype over the provider tuple).
struct LabelCtx<L: Labeller<A>>(L);
impl<L: Labeller<A>> HasLabel<A> for LabelCtx<L> {
    type Provider = L;
    fn label(&self) -> &L {
        &self.0
    }
}

#[test]
fn parameterised_provider_dispatches_via_accessor() {
    let ctx = LabelCtx(LabelA);
    // HasLabel::<A>::label returns &LabelA; LabelA::label returns 0xA.
    assert_eq!(HasLabel::<A>::label(&ctx).label(), 0xA);
}

#[test]
fn parameterised_provider_covers_second_tag() {
    struct BCtx<L: Labeller<B>>(L);
    impl<L: Labeller<B>> HasLabel<B> for BCtx<L> {
        type Provider = L;
        fn label(&self) -> &L {
            &self.0
        }
    }

    let ctx = BCtx(LabelB);
    assert_eq!(HasLabel::<B>::label(&ctx).label(), 0xB);
}

// --- Two-parameter variant ---------------------------------------------

trait Pair<R: Tag, W: Tag> {
    fn pair(&self) -> (u8, u8);
}

provider_generic2!(<R: Tag, W: Tag> Pair as HasPair => pair);

struct PairAB;
impl Pair<A, B> for PairAB {
    fn pair(&self) -> (u8, u8) {
        (0xA, 0xB)
    }
}

struct PairCtx<P: Pair<A, B>>(P);
impl<P: Pair<A, B>> HasPair<A, B> for PairCtx<P> {
    type Provider = P;
    fn pair(&self) -> &P {
        &self.0
    }
}

#[test]
fn two_parameter_provider_dispatches_via_accessor() {
    let ctx = PairCtx(PairAB);
    assert_eq!(HasPair::<A, B>::pair(&ctx).pair(), (0xA, 0xB));
}
