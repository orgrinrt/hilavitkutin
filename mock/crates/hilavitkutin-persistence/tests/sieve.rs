//! SieveCache insert / get / evict ordering (FIFO fallback).

use hilavitkutin_persistence::SieveCache;

#[test]
fn new_cache_is_empty() {
    let c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.is_empty());
    assert_eq!(c.len(), 0);
    assert_eq!(c.capacity(), 4);
}

#[test]
fn insert_and_get_roundtrip() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    let prev = c.insert(1, 100, 10);
    assert!(prev.is_none());
    assert_eq!(c.len(), 1);
    assert_eq!(c.get(&1), Some(&100));
}

#[test]
fn insert_replaces_existing_key() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    c.insert(1, 100, 10);
    let prev = c.insert(1, 200, 20);
    assert_eq!(prev, Some(100));
    assert_eq!(c.len(), 1);
    assert_eq!(c.get(&1), Some(&200));
}

#[test]
fn evict_empty_is_none() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.evict().is_none());
}

#[test]
fn evict_fifo_ordering_when_no_visits() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    c.insert(1, 10, 1);
    c.insert(2, 20, 1);
    c.insert(3, 30, 1);

    let (k, v) = c.evict().expect("non-empty");
    assert_eq!((k, v), (1, 10));
    assert_eq!(c.len(), 2);
}

#[test]
fn evict_skips_visited_and_clears_bit() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    c.insert(1, 10, 1);
    c.insert(2, 20, 1);
    c.insert(3, 30, 1);

    // Visit 1 — eviction should skip it the first pass, land on 2.
    let _ = c.get(&1);

    let (k, _) = c.evict().expect("non-empty");
    assert_eq!(k, 2);

    // After the first eviction, head has advanced past slot 0 (where
    // 1 lives) — its visited bit got cleared on the skip. The next
    // eviction continues from the head position and picks up 3.
    let (k2, _) = c.evict().expect("non-empty");
    assert_eq!(k2, 3);

    // 1 is the last survivor; evict it explicitly.
    let (k3, _) = c.evict().expect("non-empty");
    assert_eq!(k3, 1);
    assert!(c.is_empty());
}

#[test]
fn insert_evicts_head_when_full() {
    let mut c: SieveCache<u32, u32, 2> = SieveCache::new();
    c.insert(1, 10, 1);
    c.insert(2, 20, 1);
    // Cache full; inserting 3 evicts head (1) and installs 3.
    c.insert(3, 30, 1);
    assert_eq!(c.len(), 2);
    assert!(c.get(&1).is_none());
    assert_eq!(c.get(&2), Some(&20));
    assert_eq!(c.get(&3), Some(&30));
}

#[test]
fn get_missing_is_none() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    c.insert(1, 10, 1);
    assert_eq!(c.get(&99), None);
}
