//! The walks over a kernel's lists: checking a binding's roles and coefficients, minting handles
//! from them, and reading and writing a record through the handles.
//!
//! A kernel's roles are its `Roles` list, one per column it names. A member finds its role by the
//! place of its column in that list, which the compiler infers (`PlaceOf`): a member naming a
//! column the list does not hold, or a list holding one column twice, does not build. The places
//! of a list of kernels are carried as a type parameter `P` wherever the list is walked, inferred
//! where the list is first bound.
//!
//! Sealed: nothing outside this crate names these traits, so the only handles that exist are the
//! ones `mint` made from a binding `check` passed.

use notko::sink::Push;
use notko::{Maybe, Outcome};
use notko_hlist::{Here, PlaceOf, There};

use super::{BindRefusal, Binding, Col};
use crate::digest::{Absorb, Digest};
use crate::domain::{
    Cell,
    Column,
    Domain,
    FormatId,
    Near,
    Offset,
    Own,
    Param,
    Resource,
    Value,
    Word,
};
use crate::list::{Cons, Empty, List, Values};
use crate::num::{Bool, ColumnNo, Count, Index, Ordinal};
use crate::store::{Read, Stage};

/// What a walk needs of the store: the format of a stored column, or nothing past the last.
pub trait Formats {
    fn format(&self, column: ColumnNo) -> Maybe<FormatId>;
}

/// A place in a list, read as the ordinal of the role it names.
pub trait RoleAt {
    const AT: Ordinal;
}

impl RoleAt for Here {
    const AT: Ordinal = Ordinal::FIRST;
}

impl<P: RoleAt> RoleAt for There<P> {
    const AT: Ordinal = P::AT.next();
}

/// The walk over a kernel's role list: one role per column, each checked against its binding.
pub trait RoleList<D: Domain>: List {
    /// How many roles, which is how many a binding carries.
    const COUNT: Count;
    /// Checks the roles from `role` on: the binding's format is the column's, and the store holds
    /// the named column in that format.
    fn check<F: Formats>(
        binding: &Binding<'_>,
        at: Ordinal,
        role: Ordinal,
        formats: &F,
    ) -> Outcome<(), BindRefusal>;
}

impl<D: Domain> RoleList<D> for Empty {
    const COUNT: Count = Count::ZERO;

    fn check<F: Formats>(
        _binding: &Binding<'_>,
        _at: Ordinal,
        _role: Ordinal,
        _formats: &F,
    ) -> Outcome<(), BindRefusal> {
        Outcome::Ok(())
    }
}

impl<D: Domain, C: Column<Domain = D>, T: RoleList<D>> RoleList<D> for Cons<C, T> {
    const COUNT: Count = T::COUNT.next();

    fn check<F: Formats>(
        binding: &Binding<'_>,
        at: Ordinal,
        role: Ordinal,
        formats: &F,
    ) -> Outcome<(), BindRefusal> {
        let refused = BindRefusal::FormatMismatch {
            binding: at,
            role,
        };
        let Some(bound) = binding.roles.get(role.host()) else {
            return Outcome::Err(BindRefusal::RoleCount {
                binding: at,
            });
        };
        if bound.format != C::Value::FORMAT {
            return Outcome::Err(refused);
        }
        match formats.format(bound.column) {
            Maybe::Is(stored) if stored == C::Value::FORMAT => {
                T::check(binding, at, role.next(), formats)
            },
            Maybe::Is(_) => Outcome::Err(refused),
            Maybe::Isnt => {
                Outcome::Err(BindRefusal::UnknownColumn {
                    binding: at,
                    role,
                })
            },
        }
    }
}

/// One member's read: what it is read through, and the read of a record.
pub trait Gather<D: Domain>: crate::domain::Member {
    /// A column handle, or the coefficient's value.
    type Handle: Copy + Send + Sync + 'static;
    /// Coefficients the member takes: one for `Param`, none for `Own` and `Near`.
    const PARAMS: Count;
    /// Checks the member's coefficient, if it takes one, at `param`.
    fn check_param(binding: &Binding<'_>, at: Ordinal, param: Ordinal) -> Outcome<(), BindRefusal>;
    fn read<R: Read<D>>(c: &R, at: Index, h: Self::Handle) -> Self::Value;
}

/// A member that is written: `Own`.
pub trait WriteRole<D: Domain>: Gather<D> {
    fn write<W: Stage<D>>(s: &mut W, at: Index, h: Self::Handle, v: Self::Value);
}

/// One member's handle, minted from its binding: its column's role found at the place `P` in the
/// role list `R`, or for `Param`, its coefficient.
pub trait Mint<D: Domain, R, P>: Gather<D> {
    fn mint(binding: &Binding<'_>, param: Ordinal) -> Self::Handle;
    /// The stored column the member names, if it names one.
    fn column(binding: &Binding<'_>) -> Maybe<ColumnNo>;
}

fn param_check(
    binding: &Binding<'_>,
    at: Ordinal,
    param: Ordinal,
    format: FormatId,
) -> Outcome<(), BindRefusal> {
    match binding.params.get(param.host()) {
        Some(p) if p.format == format => Outcome::Ok(()),
        Some(_) => {
            Outcome::Err(BindRefusal::CoefficientMismatch {
                binding: at,
                param,
            })
        },
        None => {
            Outcome::Err(BindRefusal::RoleCount {
                binding: at,
            })
        },
    }
}

fn role_column<P: RoleAt>(binding: &Binding<'_>) -> ColumnNo {
    // `check` passed this binding, so it carries every role of its kernel.
    binding.roles[P::AT.host()].column
}

impl<D: Domain, C: Column<Domain = D>> Gather<D> for Own<C> {
    type Handle = Col<D, C::Value>;

    const PARAMS: Count = Count::ZERO;

    fn check_param(
        _binding: &Binding<'_>,
        _at: Ordinal,
        _param: Ordinal,
    ) -> Outcome<(), BindRefusal> {
        Outcome::Ok(())
    }

    fn read<R: Read<D>>(c: &R, at: Index, h: Self::Handle) -> C::Value {
        c.get(h, at)
    }
}

impl<D: Domain, C: Column<Domain = D>> WriteRole<D> for Own<C> {
    fn write<W: Stage<D>>(s: &mut W, at: Index, h: Self::Handle, v: C::Value) {
        s.put(h, at, v);
    }
}

impl<D: Domain, C: Column<Domain = D>, R: PlaceOf<C, P>, P: RoleAt + notko_hlist::Place>
    Mint<D, R, P> for Own<C>
{
    fn mint(binding: &Binding<'_>, _param: Ordinal) -> Self::Handle {
        Col::mint(role_column::<P>(binding))
    }

    fn column(binding: &Binding<'_>) -> Maybe<ColumnNo> {
        Maybe::Is(role_column::<P>(binding))
    }
}

impl<D: Domain, C: Column<Domain = D>, O: Offset<Domain = D>> Gather<D> for Near<C, O> {
    type Handle = Col<D, C::Value>;

    const PARAMS: Count = Count::ZERO;

    fn check_param(
        _binding: &Binding<'_>,
        _at: Ordinal,
        _param: Ordinal,
    ) -> Outcome<(), BindRefusal> {
        Outcome::Ok(())
    }

    fn read<R: Read<D>>(c: &R, at: Index, h: Self::Handle) -> C::Value {
        match c.near(at, O::STEP) {
            Maybe::Is(there) => c.get(h, there),
            Maybe::Isnt => C::OUTSIDE,
        }
    }
}

impl<
    D: Domain,
    C: Column<Domain = D>,
    O: Offset<Domain = D>,
    R: PlaceOf<C, P>,
    P: RoleAt + notko_hlist::Place,
> Mint<D, R, P> for Near<C, O>
{
    fn mint(binding: &Binding<'_>, _param: Ordinal) -> Self::Handle {
        Col::mint(role_column::<P>(binding))
    }

    fn column(binding: &Binding<'_>) -> Maybe<ColumnNo> {
        Maybe::Is(role_column::<P>(binding))
    }
}

impl<D: Domain, X: Resource> Gather<D> for Param<X> {
    type Handle = X::Value;

    const PARAMS: Count = Count::ONE;

    fn check_param(binding: &Binding<'_>, at: Ordinal, param: Ordinal) -> Outcome<(), BindRefusal> {
        param_check(binding, at, param, X::Value::FORMAT)
    }

    fn read<Rd: Read<D>>(_c: &Rd, _at: Index, h: X::Value) -> X::Value {
        h
    }
}

impl<D: Domain, R, X: Resource> Mint<D, R, ()> for Param<X> {
    fn mint(binding: &Binding<'_>, param: Ordinal) -> X::Value {
        let word = match binding.params.get(param.host()) {
            Some(p) => p.word,
            None => Word::ZERO,
        };
        X::Value::from_cell(<<X::Value as Value>::Cell as Cell>::from_word(word))
    }

    fn column(_binding: &Binding<'_>) -> Maybe<ColumnNo> {
        Maybe::Isnt
    }
}

/// What a set of write handles says about the stored columns it names.
pub trait Columns {
    /// Whether one of the handles names `column`.
    fn holds(&self, column: ColumnNo) -> Bool;
}

impl Columns for () {
    fn holds(&self, _column: ColumnNo) -> Bool {
        Bool::FALSE
    }
}

impl<D, V, T: Columns> Columns for (Col<D, V>, T) {
    fn holds(&self, column: ColumnNo) -> Bool {
        if self.0.column() == column { Bool::TRUE } else { self.1.holds(column) }
    }
}

/// The walk over a read list.
pub trait ReadRoles<D: Domain>: Values {
    type Handles: Copy + Send + Sync + 'static;
    /// How many coefficients the list takes.
    const PARAMS: Count;
    /// Checks the coefficients of the list's `Param` members, from `param` on.
    fn check_params(binding: &Binding<'_>, at: Ordinal, param: Ordinal)
    -> Outcome<(), BindRefusal>;
    fn gather<R: Read<D>>(c: &R, at: Index, h: &Self::Handles) -> Self::Of;
}

impl<D: Domain> ReadRoles<D> for Empty {
    type Handles = ();

    const PARAMS: Count = Count::ZERO;

    fn check_params(
        _binding: &Binding<'_>,
        _at: Ordinal,
        _param: Ordinal,
    ) -> Outcome<(), BindRefusal> {
        Outcome::Ok(())
    }

    fn gather<R: Read<D>>(_c: &R, _at: Index, _h: &()) {}
}

impl<D: Domain, H: Gather<D>, T: ReadRoles<D>> ReadRoles<D> for Cons<H, T> {
    type Handles = (H::Handle, T::Handles);

    const PARAMS: Count = H::PARAMS.add(T::PARAMS);

    fn check_params(
        binding: &Binding<'_>,
        at: Ordinal,
        param: Ordinal,
    ) -> Outcome<(), BindRefusal> {
        match H::check_param(binding, at, param) {
            Outcome::Ok(()) => T::check_params(binding, at, param.after(H::PARAMS)),
            refused => refused,
        }
    }

    fn gather<R: Read<D>>(c: &R, at: Index, h: &Self::Handles) -> Self::Of {
        (H::read(c, at, h.0), T::gather(c, at, &h.1))
    }
}

/// The walk over a list minting its handles, its members' roles found in `R` at the places `P`.
pub trait MintList<D: Domain, R, P>: ReadRoles<D> {
    fn mint(binding: &Binding<'_>, param: Ordinal) -> Self::Handles;
    /// Hands `f` the stored column of each member that names one, in list order.
    fn columns<F: FnMut(ColumnNo)>(binding: &Binding<'_>, f: &mut F);
}

impl<D: Domain, R> MintList<D, R, ()> for Empty {
    fn mint(_binding: &Binding<'_>, _param: Ordinal) {}

    fn columns<F: FnMut(ColumnNo)>(_binding: &Binding<'_>, _f: &mut F) {}
}

impl<D: Domain, R, PH, PT, H: Mint<D, R, PH>, T: MintList<D, R, PT>> MintList<D, R, (PH, PT)>
    for Cons<H, T>
{
    fn mint(binding: &Binding<'_>, param: Ordinal) -> Self::Handles {
        let head = H::mint(binding, param);
        (head, T::mint(binding, param.after(H::PARAMS)))
    }

    fn columns<F: FnMut(ColumnNo)>(binding: &Binding<'_>, f: &mut F) {
        if let Maybe::Is(column) = H::column(binding) {
            f(column);
        }
        T::columns(binding, f);
    }
}

/// The walk over a write list, which adds the write and the two outputs a runner reads back.
pub trait WriteRoles<D: Domain>: ReadRoles<D, Handles: Columns> {
    fn scatter<W: Stage<D>>(s: &mut W, at: Index, h: &Self::Handles, v: Self::Of);
    fn absorb<R: Read<D>>(c: &R, at: Index, h: &Self::Handles, into: &mut Digest);
    fn words<R: Read<D>, P: Push<Word>>(c: &R, at: Index, h: &Self::Handles, out: &mut P);
}

impl<D: Domain> WriteRoles<D> for Empty {
    fn scatter<W: Stage<D>>(_s: &mut W, _at: Index, _h: &(), _v: ()) {}

    fn absorb<R: Read<D>>(_c: &R, _at: Index, _h: &(), _into: &mut Digest) {}

    fn words<R: Read<D>, P: Push<Word>>(_c: &R, _at: Index, _h: &(), _out: &mut P) {}
}

impl<D: Domain, C: Column<Domain = D>, T: WriteRoles<D>> WriteRoles<D> for Cons<Own<C>, T> {
    fn scatter<W: Stage<D>>(s: &mut W, at: Index, h: &Self::Handles, v: Self::Of) {
        <Own<C> as WriteRole<D>>::write(s, at, h.0, v.0);
        T::scatter(s, at, &h.1, v.1);
    }

    fn absorb<R: Read<D>>(c: &R, at: Index, h: &Self::Handles, into: &mut Digest) {
        <Own<C> as Gather<D>>::read(c, at, h.0).absorb(into);
        T::absorb(c, at, &h.1, into);
    }

    fn words<R: Read<D>, P: Push<Word>>(c: &R, at: Index, h: &Self::Handles, out: &mut P) {
        out.push(<Own<C> as Gather<D>>::read(c, at, h.0).cell().word());
        T::words(c, at, &h.1, out);
    }
}
