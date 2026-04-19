//! SIEVE-style cache with FIFO fallback semantics.
//!
//! This round ships FIFO ordering: slots fill head-to-tail; eviction
//! advances from `head` and pops the first occupied slot. The
//! `visited` bit is toggled on `get` so the surface is ready for the
//! weighted-SIEVE scan that lands with the weighted-eviction tuning
//! BACKLOG item.

/// A single cache slot. Occupied when `key` is `Some`.
struct Slot<K, V> {
    key: Option<K>,
    value: Option<V>,
    weight: u64,
    visited: bool,
}

impl<K, V> Slot<K, V> {
    const EMPTY: Self = Self {
        key: None,
        value: None,
        weight: 0,
        visited: false,
    };
}

impl<K, V> Default for Slot<K, V> {
    fn default() -> Self {
        Self::EMPTY
    }
}

/// Fixed-capacity cache. `CAP` is the total number of slots.
pub struct SieveCache<K, V, const CAP: usize> {
    slots: [Slot<K, V>; CAP],
    head: usize,
    count: usize,
}

impl<K: Copy + Eq, V, const CAP: usize> SieveCache<K, V, CAP> {
    /// Construct an empty cache.
    pub fn new() -> Self {
        Self {
            slots: [const { Slot::<K, V>::EMPTY }; CAP],
            head: 0,
            count: 0,
        }
    }

    /// Current number of occupied slots.
    pub fn len(&self) -> usize {
        self.count
    }

    /// `true` when no slots are occupied.
    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Total slot capacity.
    pub const fn capacity(&self) -> usize {
        CAP
    }

    /// Insert (or replace) an entry.
    ///
    /// If the key already exists, the old value is returned and the
    /// weight is updated in place. If the cache is full, the
    /// head-most slot is evicted first (FIFO fallback); the evicted
    /// value is discarded.
    pub fn insert(&mut self, key: K, value: V, weight: u64) -> Option<V> {
        // Replace path: key already present.
        let mut i = 0;
        while i < CAP {
            if let Some(k) = self.slots[i].key {
                if k == key {
                    let old = self.slots[i].value.take();
                    self.slots[i].value = Some(value);
                    self.slots[i].weight = weight;
                    self.slots[i].visited = false;
                    return old;
                }
            }
            i += 1;
        }

        // Room? Fill first empty slot.
        if self.count < CAP {
            let mut j = 0;
            while j < CAP {
                if self.slots[j].key.is_none() {
                    self.slots[j] = Slot {
                        key: Some(key),
                        value: Some(value),
                        weight,
                        visited: false,
                    };
                    self.count += 1;
                    return None;
                }
                j += 1;
            }
        }

        // Full: evict head-first, drop evicted value, install new
        // entry in the freed slot.
        let _ = self.evict();
        let mut j = 0;
        while j < CAP {
            if self.slots[j].key.is_none() {
                self.slots[j] = Slot {
                    key: Some(key),
                    value: Some(value),
                    weight,
                    visited: false,
                };
                self.count += 1;
                return None;
            }
            j += 1;
        }
        None
    }

    /// Look up an entry. Sets the visited bit on hit.
    pub fn get(&mut self, key: &K) -> Option<&V> {
        let mut i = 0;
        while i < CAP {
            if let Some(k) = self.slots[i].key {
                if k == *key {
                    self.slots[i].visited = true;
                    return self.slots[i].value.as_ref();
                }
            }
            i += 1;
        }
        None
    }

    /// Evict the head-most occupied slot.
    ///
    /// FIFO fallback: walks from `head`, skipping empty slots, and
    /// pops the first occupied slot it meets. The visited bit is
    /// cleared as slots are skipped so that the surface behaves like
    /// the weighted-SIEVE scan will once the BACKLOG item lands.
    pub fn evict(&mut self) -> Option<(K, V)> {
        if self.count == 0 {
            return None;
        }
        let mut scanned = 0;
        while scanned < CAP {
            let idx = self.head;
            self.head = (self.head + 1) % CAP;
            scanned += 1;
            let slot = &mut self.slots[idx];
            if slot.key.is_none() {
                continue;
            }
            if slot.visited {
                slot.visited = false;
                continue;
            }
            let key = slot.key.take();
            let value = slot.value.take();
            slot.weight = 0;
            self.count -= 1;
            if let (Some(k), Some(v)) = (key, value) {
                return Some((k, v));
            }
            return None;
        }
        None
    }
}

impl<K: Copy + Eq, V, const CAP: usize> Default for SieveCache<K, V, CAP> {
    fn default() -> Self {
        Self::new()
    }
}
