//! The one-thread reference runner: the oracle every other runner is held to.
//!
//! A frame is a step, a swap and a digest. [`step`] opens a step on the bound store and takes its
//! units in table order, each with the staged cells of exactly the columns it writes; it hands each
//! unit every record the visiting order gives, in its order: the unit's reads are gathered from the
//! committed view, its kernel is called, and its writes are scattered to its staged part. The
//! bound store's swap makes them committed. [`digest`] folds the frame's number and then, unit by
//! unit in table order, record by record in key order, each written column's value. [`run`] does
//! frames and keeps their digests in storage the caller lends, refusing as a value when it is full
//! or when the visiting order hands a position outside the store.
//!
//! The runner reaches the store through the contracts only, so a store is checked by running it
//! here against the model.

#![no_std]
#![deny(unsafe_op_in_unsafe_fn)]

use hilavitkutin_contracts::bind::{
    Bound,
    Outside,
    StagedFn,
    Staging,
    StepFn,
    Unit,
    UnitFn,
    Units,
};
use hilavitkutin_contracts::digest::{Absorb, Digest};
use hilavitkutin_contracts::domain::Domain;
use hilavitkutin_contracts::kernel::{Kernels, ValueKernel};
use hilavitkutin_contracts::num::{Count, Index};
use hilavitkutin_contracts::store::{Morsel, Read, Stage, Store, Visit, Visitor};
use hilavitkutin_contracts::{Maybe, Outcome};

struct Records<'p, 's, 'c, K: ValueKernel, R, W> {
    staging:   &'p mut Staging<'s, K, W>,
    committed: &'c R,
    outside:   &'p mut Maybe<Outside>,
}

impl<K: ValueKernel, R: Read<K::Domain>, W: Stage<K::Domain>> Visitor
    for Records<'_, '_, '_, K, R, W>
{
    fn morsel(&mut self, _len: Count) {}

    fn record(&mut self, at: Index) {
        let reads = self.staging.unit().gather(self.committed, at);
        if let Outcome::Err(outside) = self.staging.scatter(at, K::at(reads))
            && let Maybe::Isnt = self.outside
        {
            *self.outside = Maybe::Is(outside);
        }
    }
}

struct Stepper<'c, R, V> {
    committed: &'c R,
    visit:     &'c V,
    morsel:    Morsel,
    outside:   Maybe<Outside>,
}

impl<'s, D: Domain, R: Read<D>, W: Stage<D>, V: Visit> StagedFn<'s, D, W> for Stepper<'_, R, V> {
    fn unit<K: ValueKernel<Domain = D>>(&mut self, mut staging: Staging<'s, K, W>) {
        let extent = self.committed.extent();
        let mut records = Records {
            staging:   &mut staging,
            committed: self.committed,
            outside:   &mut self.outside,
        };
        self.visit.visit(extent, self.morsel, &mut records);
    }
}

struct OneThread<'v, V> {
    visit:   &'v V,
    morsel:  Morsel,
    outside: Maybe<Outside>,
}

impl<D: Domain, S: Store<D>, V: Visit> StepFn<D, S> for OneThread<'_, V> {
    fn step<'s, L: Kernels<D, P>, P>(
        &mut self,
        committed: &S::Committed<'s>,
        mut units: Units<'s, D, L, S, P>,
    ) where
        S: 's,
    {
        let mut stepper = Stepper {
            committed,
            visit: self.visit,
            morsel: self.morsel,
            outside: Maybe::Isnt,
        };
        while units.next(&mut stepper).get() {}
        self.outside = stepper.outside;
    }
}

struct Digester<'v, R> {
    committed: &'v R,
    into:      Digest,
}

impl<'t, D: Domain, R: Read<D>> UnitFn<'t, D> for Digester<'_, R> {
    fn unit<K: ValueKernel<Domain = D>>(&mut self, unit: Unit<'t, K>) {
        let extent = self.committed.extent();
        let mut at = Index::FIRST;
        while at.within(extent).get() {
            unit.absorb(self.committed, at, &mut self.into);
            at = at.next();
        }
    }
}

/// Runs every unit of `bound` once, visited by `visit` in morsels of `morsel` records: reads from
/// the committed view, writes to each unit's staged part. Refuses with the first write at a
/// position outside the store, which only a visiting order breaking its contract hands out.
pub fn step<D: Domain, L: Kernels<D, P>, S: Store<D>, P, V: Visit>(
    bound: &mut Bound<'_, D, L, S, P>,
    visit: &V,
    morsel: Morsel,
) -> Outcome<(), Outside> {
    let mut runner = OneThread {
        visit,
        morsel,
        outside: Maybe::Isnt,
    };
    bound.step(&mut runner);
    match runner.outside {
        Maybe::Is(outside) => Outcome::Err(outside),
        Maybe::Isnt => Outcome::Ok(()),
    }
}

/// The digest of frame `frame`: its number, then each unit's written columns record by record.
pub fn digest<D: Domain, L: Kernels<D, P>, S: Store<D>, P>(
    bound: &Bound<'_, D, L, S, P>,
    frame: Count,
) -> Digest {
    let mut into = Digest::new();
    frame.absorb(&mut into);
    let committed = bound.committed();
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
        /// The frame that did not run.
        frame: Count,
    },
    /// The visiting order handed frame `frame` a position outside the store; the frame was not
    /// swapped in.
    Outside {
        /// The frame that stopped.
        frame:   Count,
        /// The first write outside.
        outside: Outside,
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

/// Runs `frames` frames of `bound`, keeping each frame's digest in `out`.
pub fn run<D: Domain, L: Kernels<D, P>, S: Store<D>, P, V: Visit>(
    bound: &mut Bound<'_, D, L, S, P>,
    visit: &V,
    morsel: Morsel,
    frames: Count,
    out: &mut Run<'_>,
) -> Outcome<(), Refusal> {
    let mut left = frames;
    while let Maybe::Is(rest) = left.sub(Count::ONE) {
        let frame = out.frames;
        if frame.host() >= out.digests.len() {
            return Outcome::Err(Refusal::DigestsFull {
                frame,
            });
        }
        if let Outcome::Err(outside) = step(bound, visit, morsel) {
            return Outcome::Err(Refusal::Outside {
                frame,
                outside,
            });
        }
        bound.swap();
        out.digests[frame.host()] = digest(bound, frame);
        out.frames = frame.next();
        left = rest;
    }
    Outcome::Ok(())
}
