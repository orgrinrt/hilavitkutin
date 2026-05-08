//! Typed cache handle over a ResourceSnapshot.
//!
//! Consumers index via the access-set type param so a WU can only
//! read / write resources its AccessSet actually declares.

use core::marker::PhantomData;

use arvo::USize;
use hilavitkutin_api::AccessSet;

use super::snapshot::{ResourceSnapshot, Slot};

pub struct ResourceCache<'a, R: AccessSet, const N: usize> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    snapshot: &'a mut ResourceSnapshot<N>,
    _r: PhantomData<R>,
}

impl<'a, R: AccessSet, const N: usize> ResourceCache<'a, R, N> { // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: const-generic array size; rust grammar requires usize; tracked: #121
    #[inline(always)]
    pub fn new(snapshot: &'a mut ResourceSnapshot<N>) -> Self {
        Self {
            snapshot,
            _r: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self, i: USize) -> Slot {
        self.snapshot.get(i)
    }

    #[inline(always)]
    pub fn set(&mut self, i: USize, v: Slot) {
        self.snapshot.set(i, v);
    }
}
