//! The members of a kernel's read and write lists.
//!
//! `Own<C>` is this record's value of column `C`; `Near<C, O>` the value at offset `O` from this
//! record's key, or `C::OUTSIDE` where that is outside the shape; `Param<R>` a resource's value,
//! constant over the step. A write list holds `Own` members only. Every member is a role: which
//! stored column, or which value, fills it comes from a binding.

use core::marker::PhantomData;

use super::{Column, Domain, Offset, Resource, Value};
use crate::bind::roles::{Gather, ReadRoles, WriteRole, WriteRoles};
use crate::list::{Cons, Empty, Values};

/// This record's value of column `C`.
pub struct Own<C>(PhantomData<fn() -> C>);

/// The value of column `C` at offset `O` from this record's key; `C::OUTSIDE` outside the shape.
pub struct Near<C, O>(PhantomData<fn() -> (C, O)>);

/// A resource's value, constant over the step, supplied by the binding.
pub struct Param<R>(PhantomData<fn() -> R>);

/// A member of a read or write list: something with a value.
pub trait Member: 'static {
    /// The member's value.
    type Value: Value;
}

impl<C: Column> Member for Own<C> {
    type Value = C::Value;
}

impl<C: Column, O: Offset<Domain = C::Domain>> Member for Near<C, O> {
    type Value = C::Value;
}

impl<R: Resource> Member for Param<R> {
    type Value = R::Value;
}

/// A member a kernel of domain `D` may read: `Own` and `Near` of `D`'s columns, and `Param`.
#[allow(private_bounds)]
pub trait ReadMember<D: Domain>: Member + Gather<D> {}

impl<D: Domain, C: Column<Domain = D>> ReadMember<D> for Own<C> {}
impl<D: Domain, C: Column<Domain = D>, O: Offset<Domain = D>> ReadMember<D> for Near<C, O> {}
impl<D: Domain, R: Resource> ReadMember<D> for Param<R> {}

/// A member a kernel of domain `D` may write: `Own` of `D`'s columns.
#[allow(private_bounds)]
pub trait WriteMember<D: Domain>: Member + WriteRole<D> {}

impl<D: Domain, C: Column<Domain = D>> WriteMember<D> for Own<C> {}

/// A read list of domain `D`: every member a `ReadMember<D>`.
#[allow(private_bounds)]
pub trait Reads<D: Domain>: Values + ReadRoles<D> {}

impl<D: Domain> Reads<D> for Empty {}
impl<D: Domain, H: ReadMember<D>, T: Reads<D>> Reads<D> for Cons<H, T> {}

/// A write list of domain `D`: every member a `WriteMember<D>`.
#[allow(private_bounds)]
pub trait Writes<D: Domain>: Values + WriteRoles<D> {}

impl<D: Domain> Writes<D> for Empty {}
impl<D: Domain, H: WriteMember<D>, T: Writes<D>> Writes<D> for Cons<H, T> {}
