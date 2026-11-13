//! The bind: bindings, plain data that bind kernels to stored columns, checked once.
//!
//! A unit is a kernel under one binding. A binding names the unit, the kernel by its declared id, a
//! stored column and a format for each of the kernel's column roles (its reads' `Own` and `Near`
//! members, then its writes'), and a coefficient for each `Param` member. [`bind`] checks every binding
//! against the kernel list and the store's declared formats and returns either the checked table
//! [`Bound`] or the first refusal as a value. `Bound` has no public constructor, so every runner
//! takes a table that passed, and the typed column handle [`Col`] is minted from one and from
//! nothing else: the format check and the check for two writers of one column are made once,
//! here.

pub(crate) mod roles;

use core::marker::PhantomData;

use notko::{Maybe, Outcome};
use roles::{Cursor, Formats, ReadRoles, WriteRoles};

use crate::digest::Digest;
use crate::domain::{Domain, FormatId, Word};
use crate::kernel::{KernelId, Kernels, ValueKernel};
use crate::list::Values;
use crate::num::{Count, Index, USize};
use crate::store::{Read, Store, Write};

/// A unit's declared identity: plain data in its binding, the unit of an order key. Unlocked.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UnitId(USize);

impl UnitId {
    /// The unit numbered `n`.
    pub const fn new(n: USize) -> Self {
        Self(n)
    }
}

/// A column role's binding: the stored column that fills it and the format the binding says it is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Role {
    pub column: USize,
    pub format: FormatId,
}

/// A `Param` member's binding: its value as a cell in a word, and the format the word is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coefficient {
    pub format: FormatId,
    pub word:   Word,
}

/// One binding: a unit, the kernel it runs, its column roles and its coefficients.
#[derive(Clone, Copy, Debug)]
pub struct Binding<'t> {
    pub unit:   UnitId,
    pub kernel: KernelId,
    pub roles:  &'t [Role],
    pub params: &'t [Coefficient],
}

/// Why a table was not bound. The first refusal met is returned, in the order bindings are checked:
/// each binding in table order against its kernel (`UnknownKernel`, then its roles front to back),
/// then the units against each other, then the write roles of the whole table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindRefusal {
    /// Two bindings declare one unit.
    DuplicateUnit {
        unit: UnitId,
    },
    /// A role's format is not the kernel's, or not the stored column's.
    FormatMismatch {
        binding: Index,
        role:    Index,
    },
    /// Two write roles of the table, in one binding or two, name one stored column.
    TwoWritersOfOneColumn {
        column: USize,
    },
    /// A binding names a kernel the list does not hold.
    UnknownKernel {
        binding: Index,
        kernel:  KernelId,
    },
    /// A coefficient's format is not its `Param` member's.
    CoefficientMismatch {
        binding: Index,
        param:   Index,
    },
    /// A role names a stored column past the store's last.
    UnknownColumn {
        binding: Index,
        role:    Index,
    },
    /// A binding carries more or fewer roles or coefficients than its kernel has.
    RoleCount {
        binding: Index,
    },
}

/// A typed column handle: a stored column of domain `D` whose format the bind compared with
/// `V::FORMAT`. Minted only from a [`Bound`].
pub struct Col<D, V> {
    column: USize,
    marker: PhantomData<fn() -> (D, V)>,
}

impl<D, V> Clone for Col<D, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D, V> Copy for Col<D, V> {}

impl<D, V> Col<D, V> {
    pub(crate) const fn mint(column: USize) -> Self {
        Self {
            column,
            marker: PhantomData,
        }
    }

    /// The stored column the handle names, for the store to find it by.
    pub const fn column(self) -> USize {
        self.column
    }
}

/// The checked binding table: every binding passed the bind against the kernel list `L` and a store
/// of domain `D`. No public field and no public constructor.
pub struct Bound<'t, D, L> {
    bindings: &'t [Binding<'t>],
    marker:   PhantomData<fn() -> (D, L)>,
}

/// A unit of a checked table, its kernel known as a type: the handles of its roles.
pub struct Unit<'t, K: ValueKernel> {
    binding: &'t Binding<'t>,
    reads:   <K::Reads as ReadRoles<K::Domain>>::Handles,
    writes:  <K::Writes as ReadRoles<K::Domain>>::Handles,
}

impl<'t, K: ValueKernel> Unit<'t, K> {
    pub(crate) const fn new(
        binding: &'t Binding<'t>,
        reads: <K::Reads as ReadRoles<K::Domain>>::Handles,
        writes: <K::Writes as ReadRoles<K::Domain>>::Handles,
    ) -> Self {
        Self {
            binding,
            reads,
            writes,
        }
    }

    /// The unit's declared identity.
    pub const fn id(&self) -> UnitId {
        self.binding.unit
    }

    /// The values of the kernel's reads at the record at `at` of the committed view.
    pub fn gather<R: Read<K::Domain>>(&self, committed: &R, at: Index) -> <K::Reads as Values>::Of {
        K::Reads::gather(committed, at, &self.reads)
    }

    /// Writes the kernel's write values to the record at `at` of the staged columns.
    pub fn scatter<W: Write<K::Domain>>(
        &self,
        staged: &mut W,
        at: Index,
        values: <K::Writes as Values>::Of,
    ) {
        K::Writes::scatter(staged, at, &self.writes, values);
    }

    /// Folds the values of the unit's written columns at `at` into `into`, in role order.
    pub fn absorb<R: Read<K::Domain>>(&self, committed: &R, at: Index, into: &mut Digest) {
        K::Writes::absorb(committed, at, &self.writes, into);
    }

    /// Pushes the words of the unit's written columns at `at`, in role order.
    pub fn words<R: Read<K::Domain>, P: notko::sink::Push<Word>>(
        &self,
        committed: &R,
        at: Index,
        out: &mut P,
    ) {
        K::Writes::words(committed, at, &self.writes, out);
    }
}

/// What a runner hands [`Bound::each`]: called once per unit, with the unit's kernel as a type.
pub trait UnitFn<D: Domain> {
    /// One unit.
    fn unit<K: ValueKernel<Domain = D>>(&mut self, unit: Unit<'_, K>);
}

impl<'t, D: Domain, L: Kernels<D>> Bound<'t, D, L> {
    /// How many units the table holds.
    pub const fn units(&self) -> Count {
        Count::from_host(self.bindings.len())
    }

    /// Hands every unit to `f`, in table order.
    pub fn each<F: UnitFn<D>>(&self, f: &mut F) {
        let mut i = 0;
        while i < self.bindings.len() {
            L::dispatch(&self.bindings[i], f);
            i += 1;
        }
    }
}

struct StoreFormats<'s, D, S>(&'s S, PhantomData<fn() -> D>);

impl<D: Domain, S: Store<D>> Formats for StoreFormats<'_, D, S> {
    fn format(&self, column: USize) -> Maybe<FormatId> {
        Store::format(self.0, column)
    }
}

/// Checks `bindings` against the kernel list `L` and the stored formats of `store`, and returns the
/// checked table or the first refusal. Two kernels of `L` with one id fail the build here.
pub fn bind<'t, D: Domain, L: Kernels<D>, S: Store<D>>(
    bindings: &'t [Binding<'t>],
    store: &S,
) -> Outcome<Bound<'t, D, L>, BindRefusal> {
    const {
        assert!(
            L::DISTINCT.get(),
            "two kernels of one list declare one KernelId"
        )
    };
    let formats = StoreFormats::<D, S>(store, PhantomData);
    let mut i = 0;
    while i < bindings.len() {
        let mut cur = Cursor::at(Index::from_host(i));
        if let Outcome::Err(refusal) = L::check(&bindings[i], &mut cur, &formats) {
            return Outcome::Err(refusal);
        }
        if cur.role.host() != bindings[i].roles.len()
            || cur.param.host() != bindings[i].params.len()
        {
            return Outcome::Err(BindRefusal::RoleCount {
                binding: cur.binding,
            });
        }
        i += 1;
    }
    let mut i = 0;
    while i < bindings.len() {
        let mut j = i + 1;
        while j < bindings.len() {
            if bindings[i].unit == bindings[j].unit {
                return Outcome::Err(BindRefusal::DuplicateUnit {
                    unit: bindings[i].unit,
                });
            }
            j += 1;
        }
        i += 1;
    }
    let mut i = 0;
    while i < bindings.len() {
        let mine = roles::write_columns(&bindings[i], L::read_roles(&bindings[i]));
        let mut a = 0;
        while a < mine.len() {
            let column = mine[a].column;
            let mut b = a + 1;
            while b < mine.len() {
                if mine[b].column == column {
                    return Outcome::Err(BindRefusal::TwoWritersOfOneColumn {
                        column,
                    });
                }
                b += 1;
            }
            let mut j = i + 1;
            while j < bindings.len() {
                let theirs = roles::write_columns(&bindings[j], L::read_roles(&bindings[j]));
                let mut c = 0;
                while c < theirs.len() {
                    if theirs[c].column == column {
                        return Outcome::Err(BindRefusal::TwoWritersOfOneColumn {
                            column,
                        });
                    }
                    c += 1;
                }
                j += 1;
            }
            a += 1;
        }
        i += 1;
    }
    Outcome::Ok(Bound {
        bindings,
        marker: PhantomData,
    })
}
