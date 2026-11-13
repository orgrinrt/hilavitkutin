//! `AccessSet`: a list of members, its count, and what each member stands for.

#![no_std]

use arvo::Cap;
use hilavitkutin_api::{
    AccessSet,
    Accum,
    Buffered,
    Column,
    Differs,
    Exchange,
    Member,
    Reach,
    Resource,
    Snapshot,
    Virtual,
    Written,
    read,
    write,
};

#[derive(Clone, Copy, PartialEq)]
struct A;
#[derive(Clone, Copy, PartialEq)]
struct B;
#[derive(Clone, Copy)]
struct C;
struct D;
struct E;
struct F;

const TWO: Cap = arvo_tensor::cap(2);

#[test]
fn len_arity_0() {
    assert_eq!(<read![]>::LEN.0, 0);
}

#[test]
fn len_arity_1() {
    assert_eq!(<read![Column<A>]>::LEN.0, 1);
}

#[test]
fn len_arity_2() {
    assert_eq!(<read![Column<A>, Column<B>]>::LEN.0, 2);
}

#[test]
fn len_arity_3() {
    assert_eq!(<read![Column<A>, Column<B>, Column<C>]>::LEN.0, 3);
}

#[test]
fn len_counts_every_store_kind() {
    assert_eq!(
        <read![Resource<D>, Column<A>, Buffered<B>, Buffered<C, Exchange>, Accum<A>, Virtual<E>]>::LEN.0,
        6
    );
}

#[test]
fn len_counts_a_wrapper_as_one_member() {
    assert_eq!(
        <read![Reach<Column<A>, TWO>, Reach<Buffered<B>, TWO>, Resource<F>]>::LEN.0,
        3
    );
    assert_eq!(
        <write![
            Written<Column<C>>,
            Differs<Column<A>>,
            Written<Buffered<C>>,
            Differs<Buffered<B>>
        ]>::LEN
            .0,
        4
    );
}

// `M: StandsFor<S>` holds only where the member `M` stands for the store `S`.
trait StandsFor<S> {}
impl<M: Member<Store = S>, S> StandsFor<S> for M {}

fn stands_for<M: StandsFor<S>, S>() {}

#[test]
fn a_store_marker_stands_for_itself() {
    stands_for::<Resource<D>, Resource<D>>();
    stands_for::<Column<A>, Column<A>>();
    stands_for::<Buffered<A>, Buffered<A, Snapshot>>();
    stands_for::<Buffered<A, Exchange>, Buffered<A, Exchange>>();
    stands_for::<Accum<A>, Accum<A>>();
    stands_for::<Virtual<E>, Virtual<E>>();
}

#[test]
fn a_wrapper_stands_for_the_store_it_wraps() {
    stands_for::<Reach<Column<A>, TWO>, Column<A>>();
    stands_for::<Reach<Buffered<A, Exchange>, TWO>, Buffered<A, Exchange>>();
    stands_for::<Written<Column<C>>, Column<C>>();
    stands_for::<Written<Buffered<C>>, Buffered<C>>();
    stands_for::<Differs<Column<A>>, Column<A>>();
    stands_for::<Differs<Buffered<B>>, Buffered<B>>();
}

// `Written` asks nothing of the value type: `C` has no `PartialEq`.
#[test]
fn written_takes_a_value_type_with_no_comparison() {
    fn member<M: Member>() {}
    member::<Written<Column<C>>>();
    member::<Written<Buffered<C, Exchange>>>();
}
