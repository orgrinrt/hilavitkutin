//! The member-wise walk of a kernel's lists: checking a binding's roles, minting handles from them,
//! and reading and writing a record through the handles.
//!
//! Sealed: nothing outside this crate names these traits, so the only handles that exist are the
//! ones `mint` made from a binding `check` passed.

use notko::sink::Push;
use notko::{Maybe, Outcome};

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
use crate::list::{Cons, Empty, Values};
use crate::num::{Count, Index, USize};
use crate::store::{Read, Write};

/// Where a walk is in a binding's roles and coefficients, and which binding it is.
pub struct Cursor {
    pub binding: Index,
    pub role:    Index,
    pub param:   Index,
}

impl Cursor {
    pub const fn at(binding: Index) -> Self {
        Self {
            binding,
            role: Index::FIRST,
            param: Index::FIRST,
        }
    }
}

/// What a walk needs of the store: the format of a stored column, or nothing past the last.
pub trait Formats {
    fn format(&self, column: USize) -> Maybe<FormatId>;
}

fn role_format<F: Formats, V: Value>(
    binding: &Binding<'_>,
    cur: &mut Cursor,
    formats: &F,
) -> Outcome<(), BindRefusal> {
    let at = cur.role;
    cur.role = at.next();
    let Some(role) = binding.roles.get(at.host()) else {
        return Outcome::Err(BindRefusal::RoleCount {
            binding: cur.binding,
        });
    };
    if role.format != V::FORMAT {
        return Outcome::Err(BindRefusal::FormatMismatch {
            binding: cur.binding,
            role:    at,
        });
    }
    match formats.format(role.column) {
        Maybe::Is(stored) if stored == V::FORMAT => Outcome::Ok(()),
        Maybe::Is(_) => {
            Outcome::Err(BindRefusal::FormatMismatch {
                binding: cur.binding,
                role:    at,
            })
        },
        Maybe::Isnt => {
            Outcome::Err(BindRefusal::UnknownColumn {
                binding: cur.binding,
                role:    at,
            })
        },
    }
}

fn role_col<D: Domain, V: Value>(binding: &Binding<'_>, cur: &mut Cursor) -> Col<D, V> {
    let at = cur.role;
    cur.role = at.next();
    // `check` passed this binding, so the role is there.
    Col::mint(binding.roles[at.host()].column)
}

/// One member's part of a walk.
pub trait ReadRole<D: Domain> {
    /// What the member is read through: a column handle, or the coefficient's value.
    type Handle: Copy + Send + Sync + 'static;
    /// Stored-column roles the member takes: one for `Own` and `Near`, none for `Param`.
    const ROLES: Count;
    fn check<F: Formats>(
        binding: &Binding<'_>,
        cur: &mut Cursor,
        formats: &F,
    ) -> Outcome<(), BindRefusal>;
    fn mint(binding: &Binding<'_>, cur: &mut Cursor) -> Self::Handle;
}

/// One member's read of a record, and for `Own`, its write.
pub trait Gather<D: Domain>: ReadRole<D> + crate::domain::Member {
    fn read<R: Read<D>>(c: &R, at: Index, h: Self::Handle) -> Self::Value;
}

/// A member that is written: `Own`.
pub trait WriteRole<D: Domain>: Gather<D> {
    fn write<W: Write<D>>(s: &mut W, at: Index, h: Self::Handle, v: Self::Value);
}

impl<D: Domain, C: Column<Domain = D>> ReadRole<D> for Own<C> {
    type Handle = Col<D, C::Value>;

    const ROLES: Count = Count::ONE;

    fn check<F: Formats>(
        binding: &Binding<'_>,
        cur: &mut Cursor,
        formats: &F,
    ) -> Outcome<(), BindRefusal> {
        role_format::<F, C::Value>(binding, cur, formats)
    }

    fn mint(binding: &Binding<'_>, cur: &mut Cursor) -> Self::Handle {
        role_col(binding, cur)
    }
}

impl<D: Domain, C: Column<Domain = D>> Gather<D> for Own<C> {
    fn read<R: Read<D>>(c: &R, at: Index, h: Self::Handle) -> C::Value {
        c.get(h, at)
    }
}

impl<D: Domain, C: Column<Domain = D>> WriteRole<D> for Own<C> {
    fn write<W: Write<D>>(s: &mut W, at: Index, h: Self::Handle, v: C::Value) {
        s.put(h, at, v);
    }
}

impl<D: Domain, C: Column<Domain = D>, O: Offset<Domain = D>> ReadRole<D> for Near<C, O> {
    type Handle = Col<D, C::Value>;

    const ROLES: Count = Count::ONE;

    fn check<F: Formats>(
        binding: &Binding<'_>,
        cur: &mut Cursor,
        formats: &F,
    ) -> Outcome<(), BindRefusal> {
        role_format::<F, C::Value>(binding, cur, formats)
    }

    fn mint(binding: &Binding<'_>, cur: &mut Cursor) -> Self::Handle {
        role_col(binding, cur)
    }
}

impl<D: Domain, C: Column<Domain = D>, O: Offset<Domain = D>> Gather<D> for Near<C, O> {
    fn read<R: Read<D>>(c: &R, at: Index, h: Self::Handle) -> C::Value {
        match c.near(at, O::STEP) {
            Maybe::Is(there) => c.get(h, there),
            Maybe::Isnt => C::OUTSIDE,
        }
    }
}

impl<D: Domain, R: Resource> ReadRole<D> for Param<R> {
    type Handle = R::Value;

    const ROLES: Count = Count::ZERO;

    fn check<F: Formats>(
        binding: &Binding<'_>,
        cur: &mut Cursor,
        _formats: &F,
    ) -> Outcome<(), BindRefusal> {
        let at = cur.param;
        cur.param = at.next();
        match binding.params.get(at.host()) {
            Some(p) if p.format == R::Value::FORMAT => Outcome::Ok(()),
            Some(_) => {
                Outcome::Err(BindRefusal::CoefficientMismatch {
                    binding: cur.binding,
                    param:   at,
                })
            },
            None => {
                Outcome::Err(BindRefusal::RoleCount {
                    binding: cur.binding,
                })
            },
        }
    }

    fn mint(binding: &Binding<'_>, cur: &mut Cursor) -> R::Value {
        let at = cur.param;
        cur.param = at.next();
        let word = match binding.params.get(at.host()) {
            Some(p) => p.word,
            None => Word::ZERO,
        };
        R::Value::from_cell(<<R::Value as Value>::Cell as Cell>::from_word(word))
    }
}

impl<D: Domain, R: Resource> Gather<D> for Param<R> {
    fn read<Rd: Read<D>>(_c: &Rd, _at: Index, h: R::Value) -> R::Value {
        h
    }
}

/// The walk over a read list.
pub trait ReadRoles<D: Domain>: Values {
    type Handles: Copy + Send + Sync + 'static;
    const ROLES: Count;
    fn check<F: Formats>(
        binding: &Binding<'_>,
        cur: &mut Cursor,
        formats: &F,
    ) -> Outcome<(), BindRefusal>;
    fn mint(binding: &Binding<'_>, cur: &mut Cursor) -> Self::Handles;
    fn gather<R: Read<D>>(c: &R, at: Index, h: &Self::Handles) -> Self::Of;
}

impl<D: Domain> ReadRoles<D> for Empty {
    type Handles = ();

    const ROLES: Count = Count::ZERO;

    fn check<F: Formats>(
        _row: &Binding<'_>,
        _cur: &mut Cursor,
        _formats: &F,
    ) -> Outcome<(), BindRefusal> {
        Outcome::Ok(())
    }

    fn mint(_row: &Binding<'_>, _cur: &mut Cursor) {}

    fn gather<R: Read<D>>(_c: &R, _at: Index, _h: &()) {}
}

impl<D: Domain, H: Gather<D>, T: ReadRoles<D>> ReadRoles<D> for Cons<H, T> {
    type Handles = (H::Handle, T::Handles);

    const ROLES: Count = H::ROLES.add(T::ROLES);

    fn check<F: Formats>(
        binding: &Binding<'_>,
        cur: &mut Cursor,
        formats: &F,
    ) -> Outcome<(), BindRefusal> {
        match H::check(binding, cur, formats) {
            Outcome::Ok(()) => T::check(binding, cur, formats),
            refused => refused,
        }
    }

    fn mint(binding: &Binding<'_>, cur: &mut Cursor) -> Self::Handles {
        let head = H::mint(binding, cur);
        (head, T::mint(binding, cur))
    }

    fn gather<R: Read<D>>(c: &R, at: Index, h: &Self::Handles) -> Self::Of {
        (H::read(c, at, h.0), T::gather(c, at, &h.1))
    }
}

/// The walk over a write list, which adds the write and the two outputs a runner reads back.
pub trait WriteRoles<D: Domain>: ReadRoles<D> {
    fn scatter<W: Write<D>>(s: &mut W, at: Index, h: &Self::Handles, v: Self::Of);
    fn absorb<R: Read<D>>(c: &R, at: Index, h: &Self::Handles, into: &mut Digest);
    fn words<R: Read<D>, P: Push<Word>>(c: &R, at: Index, h: &Self::Handles, out: &mut P);
}

impl<D: Domain> WriteRoles<D> for Empty {
    fn scatter<W: Write<D>>(_s: &mut W, _at: Index, _h: &(), _v: ()) {}

    fn absorb<R: Read<D>>(_c: &R, _at: Index, _h: &(), _into: &mut Digest) {}

    fn words<R: Read<D>, P: Push<Word>>(_c: &R, _at: Index, _h: &(), _out: &mut P) {}
}

impl<D: Domain, H: WriteRole<D>, T: WriteRoles<D>> WriteRoles<D> for Cons<H, T> {
    fn scatter<W: Write<D>>(s: &mut W, at: Index, h: &Self::Handles, v: Self::Of) {
        H::write(s, at, h.0, v.0);
        T::scatter(s, at, &h.1, v.1);
    }

    fn absorb<R: Read<D>>(c: &R, at: Index, h: &Self::Handles, into: &mut Digest) {
        H::read(c, at, h.0).absorb(into);
        T::absorb(c, at, &h.1, into);
    }

    fn words<R: Read<D>, P: Push<Word>>(c: &R, at: Index, h: &Self::Handles, out: &mut P) {
        out.push(H::read(c, at, h.0).cell().word());
        T::words(c, at, &h.1, out);
    }
}

/// The stored columns a binding's write roles name: the roles after its read roles.
pub fn write_columns<'r>(binding: &'r Binding<'r>, reads: Count) -> &'r [super::Role] {
    match binding.roles.get(reads.host() ..) {
        Some(w) => w,
        None => &[],
    }
}
