#![feature(const_trait_impl)]
//! evict_str / inject_str roundtrips for const + runtime handles.

use std::cell::RefCell;

use arvo::{Additive, Identity, USize};
use arvo_bits::Bits;
use arvo_hash::ContentHash;
use hilavitkutin_persistence::{
    BufferLen,
    BufferOffset,
    PersistenceError,
    StringTable,
    StringTableEntry,
    evict_str,
    inject_str,
};
use hilavitkutin_str::{ArenaInterner, Str, StringInterner, const_fnv1a, str_const};
use notko::{Maybe, Outcome};

struct VecInterner {
    strings: RefCell<Vec<String>>,
}

impl VecInterner {
    fn new() -> Self {
        Self {
            strings: RefCell::new(Vec::new()),
        }
    }
}

impl ArenaInterner for VecInterner {
    fn arena_intern(&self, s: &str) -> u32 {
        let mut v = self.strings.borrow_mut();
        for (i, existing) in v.iter().enumerate() {
            if existing == s {
                return i as u32;
            }
        }
        let id = v.len() as u32;
        v.push(s.to_string());
        id
    }

    fn arena_resolve(&self, id: u32) -> &str {
        let v = self.strings.borrow();
        // SAFETY: test-only. Vec does not reallocate within a single
        // test body that interns a bounded number of strings.
        let s: &str = &v[id as usize];
        unsafe { core::mem::transmute::<&str, &str>(s) }
    }
}

/// The key a string is stored under: the full FNV-1a hash of its bytes.
fn content_hash(s: &str) -> ContentHash {
    ContentHash::from_raw(const_fnv1a(s))
}

fn evicted(r: Outcome<ContentHash, PersistenceError>) -> ContentHash {
    match r {
        Outcome::Ok(h) => h,
        Outcome::Err(e) => panic!("evict failed: {e:?}"),
    }
}

#[test]
fn evict_const_hashes_the_string_behind_the_handle() {
    let interner = StringInterner::new(VecInterner::new());
    let h = str_const!("persist-const-evict");
    assert!(h.is_const().0);
    assert_eq!(
        evicted(evict_str(h, &interner)),
        content_hash("persist-const-evict")
    );
}

#[test]
fn evict_a_const_handle_nothing_registered_is_missing() {
    // A const handle holds 28 bits of a hash and no more. With no string
    // behind it there is no key to evict it to.
    let interner = StringInterner::new(VecInterner::new());
    let h = Str::__make(Bits::<28>::from_raw(0x0012_3456));
    assert!(h.is_const().0);
    match evict_str(h, &interner) {
        Outcome::Err(e) => assert_eq!(e, PersistenceError::Missing),
        Outcome::Ok(_) => panic!("a handle with no string behind it has no key"),
    }
}

#[test]
fn evict_runtime_hashes_bytes() {
    let interner = StringInterner::new(VecInterner::new());
    let h = interner.intern("runtime-evict-sample");
    assert!(h.is_runtime().0);
    assert_eq!(
        evicted(evict_str(h, &interner)),
        content_hash("runtime-evict-sample")
    );
}

#[test]
fn inject_a_const_key_returns_the_const_handle() {
    let interner = StringInterner::new(VecInterner::new());
    let h = str_const!("persist-const-inject");
    let key = evicted(evict_str(h, &interner));
    let table = StringTable::default();
    match inject_str(key, &interner, &table) {
        Outcome::Ok(back) => assert_eq!(back, h),
        Outcome::Err(_) => panic!("the const table knows this string"),
    }
}

#[test]
fn inject_does_not_take_a_key_sharing_only_28_bits_for_the_const_string() {
    let interner = StringInterner::new(VecInterner::new());
    let h = str_const!("persist-const-lookalike");
    let key = content_hash("persist-const-lookalike");
    // Same low 28 bits, a different key: the const candidate resolves to a
    // string that does not hash back to it, so it is not that string.
    let lookalike = ContentHash::from_raw(const_fnv1a("persist-const-lookalike") ^ (1u64 << 40));
    assert_ne!(lookalike, key);
    let table = StringTable::default();
    match inject_str(lookalike, &interner, &table) {
        Outcome::Err(e) => assert_eq!(e, PersistenceError::Missing),
        Outcome::Ok(taken) => {
            assert_ne!(
                taken, h,
                "a key sharing 28 bits must not return the const handle"
            )
        },
    }
}

#[test]
fn inject_missing_hash_returns_missing() {
    let interner = StringInterner::new(VecInterner::new());
    let table = StringTable::default();
    let r = inject_str(ContentHash::from_raw(0x0042_0042), &interner, &table);
    match r {
        Outcome::Err(e) => assert_eq!(e, PersistenceError::Missing),
        Outcome::Ok(_) => panic!("expected missing"),
    }
}

#[test]
fn inject_runtime_via_string_table() {
    let interner = StringInterner::new(VecInterner::new());

    // Construct a fabricated string-table entry pointing at a known
    // byte payload; both the entry and the payload are leaked so we
    // can satisfy the `&'static [_]` contract from test code.
    let payload: &'static [u8] = b"table-roundtrip";
    let hash = content_hash("table-roundtrip");
    let entries: &'static [StringTableEntry] = Box::leak(Box::new([StringTableEntry {
        content_hash: hash,
        bytes_offset: BufferOffset(<USize as Identity<Additive>>::IDENTITY),
        bytes_len:    BufferLen(USize(payload.len())),
    }]));
    let table = StringTable {
        entries,
        buffer: payload,
    };

    let injected = match inject_str(hash, &interner, &table) {
        Outcome::Ok(h) => h,
        Outcome::Err(_) => panic!("inject ok"),
    };
    // Runtime handle bit set, id truncated to 28 bits.
    assert!(injected.is_runtime().0);
    assert_eq!(interner.resolve(injected), Maybe::Is("table-roundtrip"));
}

#[test]
fn evict_then_inject_runtime_roundtrips() {
    let interner = StringInterner::new(VecInterner::new());

    let original_bytes: &'static [u8] = b"roundtrip-string";
    let h = interner.intern("roundtrip-string");
    let evicted = evicted(evict_str(h, &interner));

    // Build a string-table entry that re-supplies the bytes.
    let entries: &'static [StringTableEntry] = Box::leak(Box::new([StringTableEntry {
        content_hash: evicted,
        bytes_offset: BufferOffset(<USize as Identity<Additive>>::IDENTITY),
        bytes_len:    BufferLen(USize(original_bytes.len())),
    }]));
    let table = StringTable {
        entries,
        buffer: original_bytes,
    };

    let reinjected = match inject_str(evicted, &interner, &table) {
        Outcome::Ok(h) => h,
        Outcome::Err(_) => panic!("inject ok"),
    };
    assert_eq!(interner.resolve(reinjected), Maybe::Is("roundtrip-string"));
}

#[test]
fn string_table_lookup_misses_are_isnt() {
    let table = StringTable::default();
    assert!(table.lookup(ContentHash::from_raw(0x1234_5678)).isnt());
}

#[test]
fn string_table_lookup_hits_return_bytes() {
    let payload: &'static [u8] = b"lookup-hit";
    let entries: &'static [StringTableEntry] = Box::leak(Box::new([StringTableEntry {
        content_hash: ContentHash::from_raw(0xABCD),
        bytes_offset: BufferOffset(<USize as Identity<Additive>>::IDENTITY),
        bytes_len:    BufferLen(USize(payload.len())),
    }]));
    let table = StringTable {
        entries,
        buffer: payload,
    };
    match table.lookup(ContentHash::from_raw(0xABCD)) {
        Maybe::Is(b) => assert_eq!(b, &payload[..]),
        Maybe::Isnt => panic!("expected hit"),
    }
}

/// How many of `keys` repeat an earlier one once reduced to `bits` bits.
fn collisions_at(bits: u32, keys: &[u64]) -> usize {
    let mask = if bits >= 64 { u64::MAX } else { (1u64 << bits) - 1 };
    let mut reduced: Vec<u64> = keys.iter().map(|k| k & mask).collect();
    reduced.sort_unstable();
    reduced.windows(2).filter(|w| w[0] == w[1]).count()
}

/// The keys `evict_str` hands back for `count` distinct interned strings.
fn evicted_keys(interner: &StringInterner<VecInterner>, count: usize) -> Vec<u64> {
    (0 .. count)
        .map(|i| {
            let text = format!("string-{i}");
            let key = evicted(evict_str(interner.intern(&text), interner));
            assert_eq!(key, content_hash(&text), "evict keys a string by its hash");
            key.to_raw()
        })
        .collect()
}

/// Ten thousand strings evicted through `evict_str` take ten thousand distinct
/// keys. The same keys are counted again at 16 bits first, where they collide
/// by the hundreds, so a count of zero at the stored width is a measurement
/// and not a counter that cannot see one. A key cut to 28 bits would show up
/// in the stored count only by luck, which is why the next test searches for
/// the collision instead of hoping for it.
#[test]
fn ten_thousand_strings_evict_to_distinct_keys() {
    let interner = StringInterner::new(VecInterner::new());
    let keys = evicted_keys(&interner, 10_000);

    assert!(
        collisions_at(16, &keys) > 100,
        "the control: 16 bits must collide heavily, or the counter is blind"
    );
    assert_eq!(
        collisions_at(64, &keys),
        0,
        "the stored key is the width the evict path returns"
    );
}

/// Two strings that agree on their low 28 bits, found by search. A key cut to
/// 28 bits keys them the same and returns the wrong string on inject; the full
/// key tells them apart.
#[test]
fn strings_sharing_28_bits_evict_to_different_keys() {
    let mask = (1u64 << 28) - 1;
    let mut seen: std::collections::HashMap<u64, String> = std::collections::HashMap::new();
    let mut pair: Option<(String, String)> = None;
    for i in 0 .. 2_000_000u64 {
        let text = format!("probe-{i}");
        let low = const_fnv1a(&text) & mask;
        if let Some(earlier) = seen.insert(low, text.clone()) {
            pair = Some((earlier, text));
            break;
        }
    }
    let (a, b) = pair.expect("two probes sharing their low 28 bits exist well inside the search");

    let interner = StringInterner::new(VecInterner::new());
    let ka = evicted(evict_str(interner.intern(&a), &interner)).to_raw();
    let kb = evicted(evict_str(interner.intern(&b), &interner)).to_raw();

    assert_ne!(a, b);
    assert_eq!(
        ka & mask,
        kb & mask,
        "the premise: they share their low 28 bits"
    );
    assert_ne!(ka, kb, "evict keys them apart at the full width");

    // The same through inject: the key of one resolves to that string.
    let payload_a: &'static [u8] = Box::leak(a.clone().into_bytes().into_boxed_slice());
    let entries: &'static [StringTableEntry] = Box::leak(Box::new([StringTableEntry {
        content_hash: ContentHash::from_raw(ka),
        bytes_offset: BufferOffset(<USize as Identity<Additive>>::IDENTITY),
        bytes_len:    BufferLen(USize(payload_a.len())),
    }]));
    let table = StringTable {
        entries,
        buffer: payload_a,
    };
    match inject_str(ContentHash::from_raw(kb), &interner, &table) {
        Outcome::Ok(h) => {
            assert_ne!(
                interner.resolve(h),
                Maybe::Is(a.as_str()),
                "the other key must not return the first string"
            )
        },
        Outcome::Err(e) => assert_eq!(e, PersistenceError::Missing),
    }
}
