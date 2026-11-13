//! Two growable column shapes whose records never move.
//!
//! `Paged<T, P>`: fixed pages of `P` records, each its own allocation, plus a
//! page table. Growth appends pages. A morsel that stays inside one page is a
//! plain contiguous stride loop from that page's base, so the hot loop pays
//! one table load per morsel, never one per record.
//!
//! `Reserved<T>`: one address range reserved up front with no access, made
//! readable and writable a page at a time as records are added. The base
//! never moves and the column stays one contiguous run, which is what the
//! trunk's one-base-pointer `ColumnStorage` contract needs. Linux only here;
//! it needs virtual memory, which wasm32 does not offer.

use std::alloc::{Layout, alloc_zeroed};
use std::ptr::NonNull;

pub struct Paged<T: Copy, const P: usize> {
    pages: Vec<NonNull<T>>,
    len:   usize,
}

impl<T: Copy, const P: usize> Default for Paged<T, P> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Copy, const P: usize> Paged<T, P> {
    pub fn new() -> Self {
        Self {
            pages: Vec::new(),
            len:   0,
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }

    /// Append one record, adding a page when the last is full. Pages already
    /// handed out are never touched.
    pub fn push(&mut self, v: T) {
        if self.len == self.pages.len() * P {
            let layout = Layout::array::<T>(P).unwrap().align_to(64).unwrap();
            let p = unsafe { alloc_zeroed(layout) } as *mut T;
            self.pages.push(NonNull::new(p).expect("oom"));
        }
        let (pg, off) = (self.len / P, self.len % P);
        unsafe { self.pages[pg].as_ptr().add(off).write(v) };
        self.len += 1;
    }

    pub fn get(&self, i: usize) -> T {
        assert!(i < self.len);
        unsafe { self.pages[i / P].as_ptr().add(i % P).read() }
    }

    pub fn ptr(&self, i: usize) -> *const T {
        unsafe { self.pages[i / P].as_ptr().add(i % P) }
    }

    /// Base of the page holding record `start` and the offset into it, if
    /// the morsel `[start, start + len)` stays inside that page.
    pub fn morsel_base(&self, start: usize, len: usize) -> Option<*const T> {
        if start / P != (start + len - 1) / P {
            return None;
        }
        Some(self.ptr(start))
    }
}

pub struct Reserved<T: Copy> {
    base:            *mut T,
    reserved_bytes:  usize,
    committed_bytes: usize,
    len:             usize,
}

const PAGE: usize = 4096;

impl<T: Copy> Reserved<T> {
    /// Reserve room for `cap` records without committing any of it.
    pub fn reserve(cap: usize) -> Self {
        let bytes = (cap * size_of::<T>()).next_multiple_of(PAGE);
        let p = unsafe {
            libc::mmap(
                std::ptr::null_mut(),
                bytes,
                libc::PROT_NONE,
                libc::MAP_PRIVATE | libc::MAP_ANONYMOUS | libc::MAP_NORESERVE,
                -1,
                0,
            )
        };
        assert_ne!(p, libc::MAP_FAILED, "reserve failed");
        Self {
            base:            p as *mut T,
            reserved_bytes:  bytes,
            committed_bytes: 0,
            len:             0,
        }
    }

    pub fn base(&self) -> *const T {
        self.base
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn committed_bytes(&self) -> usize {
        self.committed_bytes
    }

    /// Grow to `n` records, committing whole pages as needed.
    pub fn grow_to(&mut self, n: usize) {
        let need = (n * size_of::<T>()).next_multiple_of(PAGE);
        assert!(need <= self.reserved_bytes, "past the reservation");
        if need > self.committed_bytes {
            let r = unsafe {
                libc::mprotect(
                    (self.base as *mut u8).add(self.committed_bytes) as *mut libc::c_void,
                    need - self.committed_bytes,
                    libc::PROT_READ | libc::PROT_WRITE,
                )
            };
            assert_eq!(r, 0, "commit failed");
            self.committed_bytes = need;
        }
        self.len = n;
    }

    pub fn write(&mut self, i: usize, v: T) {
        assert!(i < self.len);
        unsafe { self.base.add(i).write(v) }
    }

    pub fn read(&self, i: usize) -> T {
        assert!(i < self.len);
        unsafe { self.base.add(i).read() }
    }
}

impl<T: Copy> Drop for Reserved<T> {
    fn drop(&mut self) {
        unsafe { libc::munmap(self.base as *mut libc::c_void, self.reserved_bytes) };
    }
}

/// Number of morsels of width `w` over `n` records that cross a page of `p`
/// records, walking from 0 as the trunk's morsel loop does
/// (hilavitkutin/src/scheduler/worker.rs:369-385: `start += len`).
pub fn straddling_morsels(n: usize, w: usize, p: usize) -> usize {
    let mut start = 0;
    let mut count = 0;
    while start < n {
        let len = w.min(n - start);
        if start / p != (start + len - 1) / p {
            count += 1;
        }
        start += len;
    }
    count
}
