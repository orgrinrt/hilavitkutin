//! Smoke tests for sink composition + combinators.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin_api::{Collector, CountingSink, Len, NullSink, Push, TeeSink};

/// Bounded in-test sink. Stores up to `N` `Copy` items on the stack.
struct VecSink<T: Copy, const N: usize> {
    items: [MaybeUninit<T>; N],
    count: usize,
}

impl<T: Copy, const N: usize> VecSink<T, N> {
    fn new() -> Self {
        Self {
            items: [MaybeUninit::uninit(); N],
            count: 0,
        }
    }

    fn as_slice(&self) -> &[T] {
        // Safety: indices `0..count` were written by `push`.
        unsafe { core::slice::from_raw_parts(self.items.as_ptr().cast::<T>(), self.count) }
    }
}

impl<T: Copy, const N: usize> Push<T> for VecSink<T, N> {
    fn push(&mut self, item: T) {
        assert!(self.count < N, "VecSink overflow");
        self.items[self.count].write(item);
        self.count += 1;
    }
}

impl<T: Copy, const N: usize> Len for VecSink<T, N> {
    fn len(&self) -> USize {
        USize(self.count)
    }
}

#[test]
fn null_sink_accepts_pushes() {
    let mut s = NullSink;
    s.push(1u32);
    s.push(2u32);
    s.push(3u32);
    // No state; no panic; success.
}

#[test]
fn counting_sink_tracks_count() {
    let mut s = CountingSink::<u32>::new();
    assert_eq!(*s.len(), 0);
    for i in 0..5u32 {
        s.push(i);
    }
    assert_eq!(*s.len(), 5);
}

#[test]
fn tee_sink_fans_to_both() {
    let mut a = VecSink::<u32, 8>::new();
    let mut b = CountingSink::<u32>::new();
    {
        let mut tee = TeeSink {
            a: &mut a,
            b: &mut b,
        };
        for i in 1..=3u32 {
            tee.push(i);
        }
    }
    assert_eq!(a.as_slice(), &[1, 2, 3]);
    assert_eq!(*b.len(), 3);
}

#[test]
fn collector_blanket_covers_push_t() {
    fn drain<S: Collector<u32>>(s: &mut S, items: &[u32]) {
        for &x in items {
            s.push(x);
        }
    }

    drain(&mut NullSink, &[1, 2, 3]);
    drain(&mut CountingSink::<u32>::new(), &[1, 2, 3]);

    let mut v = VecSink::<u32, 4>::new();
    drain(&mut v, &[10, 20, 30]);
    assert_eq!(v.as_slice(), &[10, 20, 30]);
}
