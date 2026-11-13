//! The bind: bindings, plain data that bind kernels to stored columns, checked once.
//!
//! A unit is a kernel under one binding. A binding names the unit, the kernel by its declared id, a
//! stored column and a format for each of the kernel's roles, one role per column the kernel
//! names, in the order of its `Roles` list, and a coefficient for each `Param` member. [`bind`]
//! checks every binding against the kernel list and the store's declared formats and returns either
//! the checked table [`Bound`] or the first refusal as a value.
//!
//! `Bound` has no public constructor and holds the store it was checked against, borrowed
//! exclusively for as long as the table lives. So every runner takes a table that passed, a table
//! is never run over a store it was not checked against, and the store cannot be changed under it.
//! The typed column handle [`Col`] and every staged part of a step are made from a `Bound` and from
//! nothing else: the format check and the check for two writers of one column are made once, here,
//! and the staged parts a step hands out are disjoint because of it.

pub(crate) mod roles;
pub(crate) mod walk;

use core::marker::PhantomData;

use notko::{Maybe, Outcome};
use roles::{Formats, MintList, ReadRoles, WriteRoles};
pub(crate) use walk::{Each, Handles};

use crate::digest::Digest;
use crate::domain::{Domain, FormatId, Word};
use crate::kernel::{KernelId, Kernels, ValueKernel};
use crate::list::Values;
use crate::num::{Bool, ColumnNo, Count, Index, Ordinal};
use crate::store::{Opening, Read, Stage, Store, Writes};

/// A unit's declared identity: plain data in its binding, the unit of an order key. Unlocked.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub struct UnitId(crate::num::USize);

impl UnitId {
    /// The unit numbered `n`.
    pub const fn new(n: crate::num::USize) -> Self {
        Self(n)
    }
}

/// A role's binding: the stored column that fills one column a kernel names, and the format the
/// binding says it is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Role {
    /// The stored column that fills the role.
    pub column: ColumnNo,
    /// The format the binding says the column is in; the bind compares it with the kernel's and
    /// the store's.
    pub format: FormatId,
}

/// A `Param` member's binding: its value as a cell in a word, and the format the word is in.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Coefficient {
    /// The format the word is in; the bind compares it with the member's.
    pub format: FormatId,
    /// The value, as its cell in the low bytes of the word.
    pub word:   Word,
}

/// One binding: a unit, the kernel it runs, its roles and its coefficients.
#[derive(Clone, Copy, Debug)]
pub struct Binding<'t> {
    /// The unit's declared identity.
    pub unit:   UnitId,
    /// The kernel the unit runs, by its declared id.
    pub kernel: KernelId,
    /// One role for each column of the kernel's `Roles` list, in its order.
    pub roles:  &'t [Role],
    /// One coefficient for each `Param` member of the kernel's reads, in list order.
    pub params: &'t [Coefficient],
}

/// Why a table was not bound. The first refusal met is returned, in the order bindings are checked:
/// each binding in table order against its kernel (`UnknownKernel`, then `RoleCount`, then its
/// roles front to back, then its coefficients), then the units against each other, then the write
/// roles of the whole table.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum BindRefusal {
    /// Two bindings declare one unit.
    DuplicateUnit {
        /// The unit declared twice.
        unit: UnitId,
    },
    /// A role's format is not the kernel's, or not the stored column's.
    FormatMismatch {
        /// The binding's place in the table.
        binding: Ordinal,
        /// The role's place in the binding.
        role:    Ordinal,
    },
    /// Two writes of the table, in one binding or two, name one stored column.
    TwoWritersOfOneColumn {
        /// The stored column written twice.
        column: ColumnNo,
    },
    /// A binding names a kernel the list does not hold.
    UnknownKernel {
        /// The binding's place in the table.
        binding: Ordinal,
        /// The id it names.
        kernel:  KernelId,
    },
    /// A coefficient's format is not its `Param` member's.
    CoefficientMismatch {
        /// The binding's place in the table.
        binding: Ordinal,
        /// The coefficient's place in the binding.
        param:   Ordinal,
    },
    /// A role names a stored column past the store's last.
    UnknownColumn {
        /// The binding's place in the table.
        binding: Ordinal,
        /// The role's place in the binding.
        role:    Ordinal,
    },
    /// A binding carries more or fewer roles or coefficients than its kernel has.
    RoleCount {
        /// The binding's place in the table.
        binding: Ordinal,
    },
}

/// A typed column handle: a stored column of domain `D` whose format the bind compared with
/// `V::FORMAT`. Minted only from a [`Bound`].
pub struct Col<D, V> {
    column: ColumnNo,
    marker: PhantomData<fn() -> (D, V)>,
}

impl<D, V> Clone for Col<D, V> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<D, V> Copy for Col<D, V> {}

impl<D, V> Col<D, V> {
    pub(crate) const fn mint(column: ColumnNo) -> Self {
        Self {
            column,
            marker: PhantomData,
        }
    }

    /// The stored column the handle names, for the store to find it by.
    pub const fn column(self) -> ColumnNo {
        self.column
    }
}

/// A unit of a checked table, its kernel known as a type, with the committed view of the store the
/// table holds. Only [`Bound::each`] and [`Units::next`] make one, so a unit's handles are only
/// ever applied to the store they were checked against.
pub struct Unit<'v, K: ValueKernel, R> {
    handles:   Handles<'v, K>,
    committed: &'v R,
}

impl<K: ValueKernel, R> Clone for Unit<'_, K, R> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<K: ValueKernel, R> Copy for Unit<'_, K, R> {}

impl<'v, K: ValueKernel, R: Read<K::Domain>> Unit<'v, K, R> {
    /// The unit's declared identity.
    pub const fn id(&self) -> UnitId {
        self.handles.binding.unit
    }

    /// How many records the committed view holds.
    pub fn extent(&self) -> Count {
        self.committed.extent()
    }

    /// The values of the kernel's reads at the record at `at` of the committed view.
    pub fn gather(&self, at: Index) -> <K::Reads as Values>::Of {
        K::Reads::gather(self.committed, at, &self.handles.reads)
    }

    /// Folds the values of the unit's written columns at `at` of the committed view into `into`,
    /// in write-list order.
    pub fn absorb(&self, at: Index, into: &mut Digest) {
        K::Writes::absorb(self.committed, at, &self.handles.writes, into);
    }

    /// Pushes the words of the unit's written columns at `at` of the committed view, in
    /// write-list order.
    pub fn words<P: notko::sink::Push<Word>>(&self, at: Index, out: &mut P) {
        K::Writes::words(self.committed, at, &self.handles.writes, out);
    }
}

/// A write at a position the staged part does not hold.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Outside {
    /// The unit that wrote.
    pub unit: UnitId,
    /// The position it wrote at.
    pub at:   Index,
}

/// One unit's staged part of a step: the cells of exactly the stored columns the unit writes, at
/// the positions from `low` up to `high`, with the unit and its store's committed view. `Send`
/// when the store's part is `Send` and its committed view `Sync`, which every store's are.
pub struct Staging<'v, K: ValueKernel, W, R> {
    unit: Unit<'v, K, R>,
    part: W,
    low:  Index,
    high: Index,
}

impl<'v, K: ValueKernel, W: Stage<K::Domain>, R: Read<K::Domain>> Staging<'v, K, W, R> {
    /// The unit whose part this is, with the committed view it reads.
    pub const fn unit(&self) -> &Unit<'v, K, R> {
        &self.unit
    }

    /// The first position the part holds.
    pub const fn low(&self) -> Index {
        self.low
    }

    /// One past the last position the part holds.
    pub const fn high(&self) -> Index {
        self.high
    }

    /// Cuts the part at `at`, held to the part's own positions: the positions below it, then
    /// `at` and those after.
    pub fn split(self, at: Index) -> (Self, Self) {
        let at = at.max(self.low).min(self.high);
        let (below, from) = self.part.split(at);
        (
            Self {
                unit: self.unit,
                part: below,
                low:  self.low,
                high: at,
            },
            Self {
                unit: self.unit,
                part: from,
                low:  at,
                high: self.high,
            },
        )
    }

    /// Writes the kernel's write values to the record at `at`, or refuses a position the part
    /// does not hold.
    pub fn scatter(
        &mut self,
        at: Index,
        values: <K::Writes as Values>::Of,
    ) -> Outcome<(), Outside> {
        if at.lt(self.low).get() || at.ge(self.high).get() {
            return Outcome::Err(Outside {
                unit: self.unit.id(),
                at,
            });
        }
        K::Writes::scatter(&mut self.part, at, &self.unit.handles.writes, values);
        Outcome::Ok(())
    }
}

/// What a runner hands [`Bound::each`]: called once per unit, with the unit's kernel as a type
/// and `R`, the committed view of the bound store.
pub trait UnitFn<D: Domain, R> {
    /// One unit.
    fn unit<'v, K: ValueKernel<Domain = D>>(&mut self, unit: Unit<'v, K, R>)
    where
        R: 'v;
}

/// What a runner hands [`Units::next`] for each unit of the step, with its staged part `W` and
/// `R`, the committed view the step reads.
pub trait StagedFn<'v, D: Domain, W: Stage<D>, R> {
    /// One unit's staged part, its kernel as a type.
    fn unit<K: ValueKernel<Domain = D>>(&mut self, staging: Staging<'v, K, W, R>);
}

/// The units of an open step, each handed out once with its staged part and the step's
/// committed view.
pub struct Units<'v, 's, D: Domain, L, S: Store<D> + 's, P> {
    bindings:  &'v [Binding<'v>],
    committed: &'v S::Committed<'s>,
    next:      usize,
    rest:      Maybe<S::Staged<'s>>,
    extent:    Count,
    marker:    PhantomData<fn() -> (D, L, P)>,
}

struct Handing<'f, 'v, D: Domain, W, R, F> {
    committed: &'v R,
    rest:      &'f mut Maybe<W>,
    extent:    Count,
    f:         &'f mut F,
    marker:    PhantomData<fn() -> D>,
}

impl<'v, D: Domain, W: Stage<D>, R: Read<D>, F: StagedFn<'v, D, W, R>> Each<'v, D>
    for Handing<'_, 'v, D, W, R, F>
{
    fn handles<K: ValueKernel<Domain = D>>(&mut self, handles: Handles<'v, K>) {
        let Maybe::Is(rest) = core::mem::replace(self.rest, Maybe::Isnt) else {
            return;
        };
        let (mine, others) = rest.take(&Writes(&handles.writes));
        *self.rest = Maybe::Is(others);
        self.f.unit(Staging {
            unit: Unit {
                handles,
                committed: self.committed,
            },
            part: mine,
            low:  Index::FIRST,
            high: self.extent.end(),
        });
    }
}

impl<'v, 's, D: Domain, L: Kernels<D, P>, S: Store<D> + 's, P> Units<'v, 's, D, L, S, P> {
    /// How many records the step's store holds.
    pub const fn extent(&self) -> Count {
        self.extent
    }

    /// Hands the next unit and its staged part, holding every position, to `f`; false when every
    /// unit has been handed out.
    pub fn next<F: StagedFn<'v, D, S::Staged<'s>, S::Committed<'s>>>(&mut self, f: &mut F) -> Bool {
        let Some(binding) = self.bindings.get(self.next) else {
            return Bool::FALSE;
        };
        self.next += 1;
        L::dispatch(binding, &mut Handing {
            committed: self.committed,
            rest: &mut self.rest,
            extent: self.extent,
            f,
            marker: PhantomData,
        });
        Bool::TRUE
    }
}

/// What a runner hands [`Bound::step`]: called once per step with the units, which it takes from
/// as it likes. Each unit comes with the step's committed view, and no other.
pub trait StepFn<D: Domain, S: Store<D>> {
    /// One step.
    fn step<'v, 's, L: Kernels<D, P>, P>(&mut self, units: Units<'v, 's, D, L, S, P>)
    where
        S: 's;
}

struct Viewing<'f, 'v, R, F> {
    committed: &'v R,
    f:         &'f mut F,
}

impl<'v, D: Domain, R: Read<D>, F: UnitFn<D, R>> Each<'v, D> for Viewing<'_, 'v, R, F> {
    fn handles<K: ValueKernel<Domain = D>>(&mut self, handles: Handles<'v, K>) {
        self.f.unit(Unit {
            handles,
            committed: self.committed,
        });
    }
}

/// The checked binding table, holding the store it was checked against. No public field and no
/// public constructor.
pub struct Bound<'b, D: Domain, L, S: Store<D>, P> {
    bindings: &'b [Binding<'b>],
    store:    &'b mut S,
    marker:   PhantomData<fn() -> (D, L, P)>,
}

impl<'b, D: Domain, L: Kernels<D, P>, S: Store<D>, P> Bound<'b, D, L, S, P> {
    /// How many units the table holds.
    pub const fn units(&self) -> Count {
        Count::from_host(self.bindings.len())
    }

    /// The store, to read; it cannot be changed while the table holds it.
    pub fn store(&self) -> &S {
        self.store
    }

    /// The committed view.
    pub fn committed(&self) -> S::Committed<'_> {
        self.store.committed()
    }

    /// Hands every unit to `f` in table order, each with the store's committed view.
    pub fn each<'a, F: UnitFn<D, S::Committed<'a>>>(&'a self, f: &mut F) {
        let committed = self.store.committed();
        let mut viewing = Viewing {
            committed: &committed,
            f,
        };
        let mut i = 0;
        while i < self.bindings.len() {
            L::dispatch(&self.bindings[i], &mut viewing);
            i += 1;
        }
    }

    /// Opens a step on the store and hands `f` the units, each with the step's committed view and
    /// the staged cells of the columns it writes.
    pub fn step<F: StepFn<D, S>>(&mut self, f: &mut F) {
        let extent = self.store.extent();
        let bindings: &[Binding<'_>] = self.bindings;
        let (committed, staged) = self.store.step(Opening::new());
        f.step::<L, P>(Units {
            bindings,
            committed: &committed,
            next: 0,
            rest: Maybe::Is(staged),
            extent,
            marker: PhantomData,
        });
    }

    /// Makes the step's staged cells committed.
    pub fn swap(&mut self) {
        self.store.swap();
    }
}

struct StoreFormats<'s, D, S>(&'s S, PhantomData<fn() -> D>);

impl<D: Domain, S: Store<D>> Formats for StoreFormats<'_, D, S> {
    fn format(&self, column: ColumnNo) -> Maybe<FormatId> {
        Store::format(self.0, column)
    }
}

fn writes_of<D: Domain, L: Kernels<D, P>, P>(binding: &Binding<'_>, column: ColumnNo) -> Count {
    let mut n = Count::ZERO;
    L::writes(binding, &mut |c| {
        if c == column {
            n = n.next();
        }
    });
    n
}

/// Checks `bindings` against the kernel list `L` and the stored formats of `store`, and returns the
/// checked table holding `store`, or the first refusal. Two kernels of `L` with one id, or an id
/// of 128 or more, fail the build here; `P`, the places of the kernels' members in their role
/// lists, is inferred.
pub fn bind<'b, D: Domain, L: Kernels<D, P>, S: Store<D>, P>(
    bindings: &'b [Binding<'b>],
    store: &'b mut S,
) -> Outcome<Bound<'b, D, L, S, P>, BindRefusal> {
    const {
        assert!(
            L::FITS.get(),
            "a KernelId of a bound list is below 128 while the id check folds a 128-bit word"
        );
        assert!(
            L::DISTINCT.get(),
            "two kernels of one list declare one KernelId"
        )
    };
    let formats = StoreFormats::<D, S>(store, PhantomData);
    let mut i = 0;
    while i < bindings.len() {
        if let Outcome::Err(refusal) = L::check(&bindings[i], Ordinal::from_host(i), &formats) {
            return Outcome::Err(refusal);
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
    let mut twice = Maybe::Isnt;
    let mut i = 0;
    while i < bindings.len() {
        L::writes(&bindings[i], &mut |column| {
            if matches!(twice, Maybe::Is(_)) {
                return;
            }
            let mut n = writes_of::<D, L, P>(&bindings[i], column);
            let mut j = i + 1;
            while j < bindings.len() {
                n = n.add(writes_of::<D, L, P>(&bindings[j], column));
                j += 1;
            }
            if n.gt(Count::ONE).get() {
                twice = Maybe::Is(column);
            }
        });
        if let Maybe::Is(column) = twice {
            return Outcome::Err(BindRefusal::TwoWritersOfOneColumn {
                column,
            });
        }
        i += 1;
    }
    Outcome::Ok(Bound {
        bindings,
        store,
        marker: PhantomData,
    })
}

/// The walks a kernel list does over one binding, typed by the kernel the binding names.
pub(crate) fn check_kernel<D: Domain, K: ValueKernel<Domain = D>, PR, PW>(
    binding: &Binding<'_>,
    at: Ordinal,
    formats: &impl Formats,
) -> Outcome<(), BindRefusal>
where
    K::Reads: MintList<D, K::Roles, PR>,
    K::Writes: MintList<D, K::Roles, PW>,
{
    use roles::RoleList;
    let params = <K::Reads as ReadRoles<D>>::PARAMS;
    if binding.roles.len() != <K::Roles as RoleList<D>>::COUNT.host()
        || binding.params.len() != params.host()
    {
        return Outcome::Err(BindRefusal::RoleCount {
            binding: at,
        });
    }
    match <K::Roles as RoleList<D>>::check(binding, at, Ordinal::FIRST, formats) {
        Outcome::Ok(()) => K::Reads::check_params(binding, at, Ordinal::FIRST),
        refused => refused,
    }
}

/// The handles of `binding`'s unit, minted.
pub(crate) fn unit_of<'t, D: Domain, K: ValueKernel<Domain = D>, PR, PW>(
    binding: &'t Binding<'t>,
) -> Handles<'t, K>
where
    K::Reads: MintList<D, K::Roles, PR>,
    K::Writes: MintList<D, K::Roles, PW>,
{
    let reads = <K::Reads as MintList<D, K::Roles, PR>>::mint(binding, Ordinal::FIRST);
    let writes = <K::Writes as MintList<D, K::Roles, PW>>::mint(binding, Ordinal::FIRST);
    Handles::new(binding, reads, writes)
}
