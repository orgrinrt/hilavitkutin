//! Typed cache handle over a ResourceSnapshot.
//!
//! Consumers index via the access-set type param so a WU can only
//! read / write resources its AccessSet actually declares.

use core::marker::PhantomData;

use hilavitkutin_api::AccessSet;

use super::snapshot::{ResourceSnapshot, Slot};

pub struct ResourceCache<'a, R: AccessSet, const N: usize> {
    snapshot: &'a mut ResourceSnapshot<N>,
    _r: PhantomData<R>,
}

impl<'a, R: AccessSet, const N: usize> ResourceCache<'a, R, N> {
    #[inline(always)]
    pub fn new(snapshot: &'a mut ResourceSnapshot<N>) -> Self {
        Self {
            snapshot,
            _r: PhantomData,
        }
    }

    #[inline(always)]
    pub fn get(&self, i: usize) -> Slot {
        self.snapshot.get(i)
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, v: Slot) {
        self.snapshot.set(i, v);
    }
}
