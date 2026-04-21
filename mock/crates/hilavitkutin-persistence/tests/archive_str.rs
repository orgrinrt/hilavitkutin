//! evict_str / inject_str roundtrips for const + runtime handles.

use std::cell::RefCell;

use arvo::USize;
use arvo_bits::Bits;
use arvo_hash::ContentHash;
use hilavitkutin_persistence::{
    evict_str, inject_str, BufferLen, BufferOffset, PersistenceError, StringTable,
    StringTableEntry,
};
use hilavitkutin_str::{const_fnv1a, ArenaInterner, Str, StringInterner};
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

fn content_hash(s: &str) -> ContentHash {
    ContentHash::new(const_fnv1a(s) & Str::ID_MASK.bits())
}

#[test]
fn evict_const_is_identity() {
    let interner = StringInterner::new(VecInterner::new());
    let h = Str::__make(Bits::<28>::new(0x0012_3456));
    assert!(h.is_const());
    let evicted = evict_str(h, &interner);
    assert_eq!(evicted, ContentHash::new(0x0012_3456));
}

#[test]
fn evict_runtime_hashes_bytes() {
    let interner = StringInterner::new(VecInterner::new());
    let h = interner.intern("runtime-evict-sample");
    assert!(h.is_runtime());
    let evicted = evict_str(h, &interner);
    assert_eq!(evicted, content_hash("runtime-evict-sample"));
}

#[test]
fn inject_missing_hash_returns_missing() {
    let interner = StringInterner::new(VecInterner::new());
    let table = StringTable::default();
    let r = inject_str(ContentHash::new(0x0042_0042), &interner, &table);
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
        bytes_offset: BufferOffset(USize(0)),
        bytes_len: BufferLen(USize(payload.len())),
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
    assert!(injected.is_runtime());
    assert_eq!(interner.resolve(injected), Some("table-roundtrip"));
}

#[test]
fn evict_then_inject_runtime_roundtrips() {
    let interner = StringInterner::new(VecInterner::new());

    let original_bytes: &'static [u8] = b"roundtrip-string";
    let h = interner.intern("roundtrip-string");
    let evicted = evict_str(h, &interner);

    // Build a string-table entry that re-supplies the bytes.
    let entries: &'static [StringTableEntry] = Box::leak(Box::new([StringTableEntry {
        content_hash: evicted,
        bytes_offset: BufferOffset(USize(0)),
        bytes_len: BufferLen(USize(original_bytes.len())),
    }]));
    let table = StringTable {
        entries,
        buffer: original_bytes,
    };

    let reinjected = match inject_str(evicted, &interner, &table) {
        Outcome::Ok(h) => h,
        Outcome::Err(_) => panic!("inject ok"),
    };
    assert_eq!(interner.resolve(reinjected), Some("roundtrip-string"));
}

#[test]
fn string_table_lookup_misses_are_isnt() {
    let table = StringTable::default();
    assert!(table.lookup(ContentHash::new(0x1234_5678)).isnt());
}

#[test]
fn string_table_lookup_hits_return_bytes() {
    let payload: &'static [u8] = b"lookup-hit";
    let entries: &'static [StringTableEntry] = Box::leak(Box::new([StringTableEntry {
        content_hash: ContentHash::new(0xABCD),
        bytes_offset: BufferOffset(USize(0)),
        bytes_len: BufferLen(USize(payload.len())),
    }]));
    let table = StringTable {
        entries,
        buffer: payload,
    };
    match table.lookup(ContentHash::new(0xABCD)) {
        Maybe::Is(b) => assert_eq!(b, &payload[..]),
        Maybe::Isnt => panic!("expected hit"),
    }
}
