//! The one-thread reference runner: the oracle every other runner is held to.
//!
//! A frame is a step, a swap and a digest. [`step`] hands every unit of a checked table, in table
//! order, every record the visiting order gives, in its order: the unit's reads are gathered from
//! the committed view, its kernel is called, and its writes are scattered to the staged columns.
//! [`swap`] makes them committed. [`digest`] folds the frame's number and then, unit by unit in
//! table order, record by record in key order, each written column's value. [`run`] does frames
//! and keeps their digests in storage the caller lends, refusing as a value when it is full.
//!
//! The runner reaches the store through the contracts only, so a store is checked by running it
//! here against the model.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use hilavitkutin_contracts::Outcome;
use hilavitkutin_contracts::bind::{Bound, Unit, UnitFn};
use hilavitkutin_contracts::digest::{Absorb, Digest};
use hilavitkutin_contracts::domain::Domain;
use hilavitkutin_contracts::kernel::{Kernels, ValueKernel};
use hilavitkutin_contracts::num::{Count, Index};
use hilavitkutin_contracts::store::{Read, Store, Visit, Visitor, Write};

struct Records<'u, 'v, K: ValueKernel, R, W> {
    unit:      &'u Unit<'u, K>,
    committed: &'v R,
    staged:    &'v mut W,
}

impl<K: ValueKernel, R: Read<K::Domain>, W: Write<K::Domain>> Visitor for Records<'_, '_, K, R, W> {
    fn morsel(&mut self, _len: Count) {}

    fn record(&mut self, at: Index) {
        let reads = self.unit.gather(self.committed, at);
        self.unit.scatter(self.staged, at, K::at(reads));
    }
}

struct Stepper<'v, R, W, V> {
    committed: &'v R,
    staged:    &'v mut W,
    visit:     &'v V,
    morsel:    Count,
}

impl<D: Domain, R: Read<D>, W: Write<D>, V: Visit> UnitFn<D> for Stepper<'_, R, W, V> {
    fn unit<K: ValueKernel<Domain = D>>(&mut self, unit: Unit<'_, K>) {
        let extent = self.committed.extent();
        let mut records = Records {
            unit:      &unit,
            committed: self.committed,
            staged:    &mut *self.staged,
        };
        self.visit.visit(extent, self.morsel, &mut records);
    }
}

struct Digester<'v, R> {
    committed: &'v R,
    into:      Digest,
}

impl<D: Domain, R: Read<D>> UnitFn<D> for Digester<'_, R> {
    fn unit<K: ValueKernel<Domain = D>>(&mut self, unit: Unit<'_, K>) {
        let extent = self.committed.extent();
        let mut at = Index::FIRST;
        while at.within(extent).get() {
            unit.absorb(self.committed, at, &mut self.into);
            at = at.next();
        }
    }
}

/// Runs every unit of `bound` once over `store`, visited by `visit` in morsels of `morsel`
/// records: reads from the committed view, writes to the staged columns.
pub fn step<D: Domain, L: Kernels<D>, S: Store<D>, V: Visit>(
    bound: &Bound<'_, D, L>,
    store: &mut S,
    visit: &V,
    morsel: Count,
) {
    let (committed, mut staged) = store.step();
    bound.each(&mut Stepper {
        committed: &committed,
        staged: &mut staged,
        visit,
        morsel,
    });
}

/// Makes the staged columns of `store` committed.
pub fn swap<D: Domain, S: Store<D>>(store: &mut S) {
    store.swap();
}

/// The digest of frame `frame`: its number, then each unit's written columns record by record.
pub fn digest<D: Domain, L: Kernels<D>, S: Store<D>>(
    bound: &Bound<'_, D, L>,
    store: &S,
    frame: Count,
) -> Digest {
    let mut into = Digest::new();
    frame.absorb(&mut into);
    let committed = store.committed();
    let mut digester = Digester {
        committed: &committed,
        into,
    };
    bound.each(&mut digester);
    digester.into
}

/// Why a run stopped short.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Refusal {
    /// The lent digest storage was full before frame `frame` ran; the store is as frame `frame - 1`
    /// left it.
    DigestsFull {
        frame: Count,
    },
}

/// The digests of a run's frames, in storage the caller lends.
pub struct Run<'d> {
    digests: &'d mut [Digest],
    frames:  Count,
}

impl<'d> Run<'d> {
    /// A run keeping its digests in `digests`.
    pub const fn new(digests: &'d mut [Digest]) -> Self {
        Self {
            digests,
            frames: Count::ZERO,
        }
    }

    /// How many frames have run.
    pub const fn frames(&self) -> Count {
        self.frames
    }

    /// The digests of the frames that ran, in frame order.
    pub fn digests(&self) -> &[Digest] {
        match self.digests.get(.. self.frames.host()) {
            Some(done) => done,
            None => &[],
        }
    }
}

/// Runs `frames` frames of `bound` over `store`, keeping each frame's digest in `out`.
pub fn run<D: Domain, L: Kernels<D>, S: Store<D>, V: Visit>(
    bound: &Bound<'_, D, L>,
    store: &mut S,
    visit: &V,
    morsel: Count,
    frames: Count,
    out: &mut Run<'_>,
) -> Outcome<(), Refusal> {
    let mut left = frames;
    while left.gt(Count::ZERO).get() {
        let frame = out.frames;
        if frame.host() >= out.digests.len() {
            return Outcome::Err(Refusal::DigestsFull {
                frame,
            });
        }
        step(bound, store, visit, morsel);
        swap(store);
        out.digests[frame.host()] = digest(bound, store, frame);
        out.frames = frame.next();
        left = left.sub(Count::ONE);
    }
    Outcome::Ok(())
}
