//! SieveCache insert / get / evict ordering (FIFO fallback).
//!
//! Every insert of a key not yet present returns `Isnt`, including the
//! one that evicts to make room, because an evicted value is discarded
//! rather than handed back; the tests assert that at each such insert.

use arvo::{Bool, USize};
use hilavitkutin_persistence::{EvictionWeight, SieveCache};
use notko::Maybe;

#[test]
fn insert_and_get_roundtrip() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    let prev = c.insert(1, 100, EvictionWeight::new(10));
    assert!(prev.isnt());
    assert_eq!(c.len(), USize(1));
    match c.get(&1) {
        Maybe::Is(v) => assert_eq!(*v, 100),
        Maybe::Isnt => panic!("expected value"),
    }
}

#[test]
fn insert_replaces_existing_key() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.insert(1, 100, EvictionWeight::new(10)).isnt());
    let prev = c.insert(1, 200, EvictionWeight::new(20));
    assert_eq!(prev, Maybe::Is(100));
    assert_eq!(c.len(), USize(1));
    match c.get(&1) {
        Maybe::Is(v) => assert_eq!(*v, 200),
        Maybe::Isnt => panic!("expected value"),
    }
}

#[test]
fn evict_empty_is_isnt() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.evict().isnt());
}

#[test]
fn evict_fifo_ordering_when_no_visits() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.insert(1, 10, EvictionWeight::new(1)).isnt());
    assert!(c.insert(2, 20, EvictionWeight::new(1)).isnt());
    assert!(c.insert(3, 30, EvictionWeight::new(1)).isnt());

    match c.evict() {
        Maybe::Is((k, v)) => assert_eq!((k, v), (1, 10)),
        Maybe::Isnt => panic!("non-empty"),
    }
    assert_eq!(c.len(), USize(2));
}

#[test]
fn evict_skips_visited_and_clears_bit() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.insert(1, 10, EvictionWeight::new(1)).isnt());
    assert!(c.insert(2, 20, EvictionWeight::new(1)).isnt());
    assert!(c.insert(3, 30, EvictionWeight::new(1)).isnt());

    // Visit 1. Eviction should skip it the first pass, land on 2.
    assert!(c.get(&1).is());

    match c.evict() {
        Maybe::Is((k, _)) => assert_eq!(k, 2),
        Maybe::Isnt => panic!("non-empty"),
    }

    // After the first eviction, head has advanced past slot 0 (where
    // 1 lives); its visited bit got cleared on the skip. The next
    // eviction continues from the head position and picks up 3.
    match c.evict() {
        Maybe::Is((k, _)) => assert_eq!(k, 3),
        Maybe::Isnt => panic!("non-empty"),
    }

    // 1 is the last survivor; evict it explicitly.
    match c.evict() {
        Maybe::Is((k, _)) => assert_eq!(k, 1),
        Maybe::Isnt => panic!("non-empty"),
    }
    assert_eq!(c.is_empty(), Bool(true));
}

#[test]
fn insert_evicts_head_when_full() {
    let mut c: SieveCache<u32, u32, 2> = SieveCache::new();
    assert!(c.insert(1, 10, EvictionWeight::new(1)).isnt());
    assert!(c.insert(2, 20, EvictionWeight::new(1)).isnt());
    // Cache full. Inserting 3 evicts head (1), installs 3, and hands
    // nothing back.
    assert!(c.insert(3, 30, EvictionWeight::new(1)).isnt());
    assert_eq!(c.len(), USize(2));
    assert!(c.get(&1).isnt());
    match c.get(&2) {
        Maybe::Is(v) => assert_eq!(*v, 20),
        Maybe::Isnt => panic!("expected value"),
    }
    match c.get(&3) {
        Maybe::Is(v) => assert_eq!(*v, 30),
        Maybe::Isnt => panic!("expected value"),
    }
}

#[test]
fn get_missing_is_isnt() {
    let mut c: SieveCache<u32, u32, 4> = SieveCache::new();
    assert!(c.insert(1, 10, EvictionWeight::new(1)).isnt());
    assert!(c.get(&99).isnt());
}
