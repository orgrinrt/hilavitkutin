//! Runtime string table header + bytes pool.
//!
//! Header entries carry a content hash and a slice into the bytes
//! pool. Lookup is an O(n) linear scan this round; perfect-hash
//! lookup lands once the hot path is measured.

/// A single entry in the string-table header.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct StringTableEntry {
    /// 28-bit FNV content hash (stored in a u32).
    pub content_hash: u32,
    /// Byte offset into the string-table buffer.
    pub bytes_offset: u32,
    /// Byte length at that offset.
    pub bytes_len: u32,
}

/// Runtime string table, as referenced from a loaded cold store.
///
/// `entries` and `buffer` are borrowed from the mmap-backed cold
/// store. Skeleton defaults to empty static slices.
pub struct StringTable {
    /// Header slice.
    pub entries: &'static [StringTableEntry],
    /// Bytes pool.
    pub buffer: &'static [u8],
}

impl StringTable {
    /// Construct an empty string table (no entries, no bytes).
    pub const fn empty() -> Self {
        Self {
            entries: &[],
            buffer: &[],
        }
    }

    /// Look up bytes for a content hash. O(n) linear scan this
    /// skeleton round; a perfect-hash lookup lands in the follow-up
    /// round when the lookup hot path is measured.
    pub fn lookup(&self, content_hash: u32) -> Option<&[u8]> {
        let mut i = 0;
        while i < self.entries.len() {
            let e = &self.entries[i];
            if e.content_hash == content_hash {
                let start = e.bytes_offset as usize;
                let end = start + e.bytes_len as usize;
                if end <= self.buffer.len() {
                    return Some(&self.buffer[start..end]);
                }
                return None;
            }
            i += 1;
        }
        None
    }
}

impl Default for StringTable {
    fn default() -> Self {
        Self::empty()
    }
}
