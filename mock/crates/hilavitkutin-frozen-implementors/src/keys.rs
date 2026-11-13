//! The key table both stores answer a neighbour read from: every key of the shape, in key order,
//! a record's position being its place in the table.

use hilavitkutin_contracts::Maybe;
use hilavitkutin_contracts::domain::Domain;
use hilavitkutin_contracts::num::{Count, Index};

pub struct Keys<D: Domain, const CAP: usize> {
    keys:   [Maybe<D::Key>; CAP],
    extent: Count,
    shape:  D::Shape,
}

impl<D: Domain, const CAP: usize> Keys<D, CAP> {
    /// The table of `shape`, or nothing where it holds more than `CAP` records.
    pub fn of(shape: D::Shape) -> Maybe<Self> {
        let mut keys = [const { Maybe::Isnt }; CAP];
        let mut extent = Count::ZERO;
        let mut walk = D::first(shape);
        while let Maybe::Is(key) = walk {
            if extent.host() == CAP {
                return Maybe::Isnt;
            }
            keys[extent.host()] = Maybe::Is(key);
            extent = extent.next();
            walk = D::next(shape, key);
        }
        Maybe::Is(Self {
            keys,
            extent,
            shape,
        })
    }

    pub const fn extent(&self) -> Count {
        self.extent
    }

    pub const fn shape(&self) -> D::Shape {
        self.shape
    }

    fn key(&self, at: Index) -> Maybe<D::Key> {
        match self.keys.get(at.host()) {
            Some(key) => *key,
            None => Maybe::Isnt,
        }
    }

    /// The position of `key`, by halving the table.
    fn position(&self, key: D::Key) -> Maybe<Index> {
        let mut low = 0;
        let mut high = self.extent.host();
        while low < high {
            let mid = low + (high - low) / 2;
            match self.keys[mid] {
                Maybe::Is(there) if there < key => low = mid + 1,
                Maybe::Is(there) if there > key => high = mid,
                Maybe::Is(_) => return Maybe::Is(Index::from_host(mid)),
                Maybe::Isnt => return Maybe::Isnt,
            }
        }
        Maybe::Isnt
    }

    pub fn near(&self, at: Index, step: D::Step) -> Maybe<Index> {
        match self.key(at) {
            Maybe::Is(key) => {
                match D::reach(self.shape, key, step) {
                    Maybe::Is(there) => self.position(there),
                    Maybe::Isnt => Maybe::Isnt,
                }
            },
            Maybe::Isnt => Maybe::Isnt,
        }
    }
}
