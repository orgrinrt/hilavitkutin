//! Named sink traits + combinators built on capability atoms.
//!
//! `Collector<T>`, `DiagnosticSink<E>`, `ByteEmitter` read as intent
//! at call sites; blanket impls make each a zero-overhead shorthand
//! for its capability bound.
//!
//! `NullSink`, `CountingSink`, `TeeSink` are the standard combinators
//! consumers pass where a sink is expected. Tuple blanket impls are
//! not provided in this round; callers taking disjoint sinks take
//! them as separate parameters or wrap via `TeeSink`.

use core::marker::PhantomData;

use arvo::USize;
use arvo::strategy::Identity;

use crate::capability::{BulkPush, Len, Push};

/// Intent-signalling alias: "this position receives items of type T".
///
/// Any `Push<T>` implementor is a `Collector<T>` via the blanket
/// below. Prefer this trait in call-site bounds when the semantics
/// are "accepts items" rather than "overflow-aware push" etc.
pub trait Collector<T>: Push<T> {}
impl<T, S: Push<T> + ?Sized> Collector<T> for S {}

/// Push-with-count: sinks that track how many items arrived.
///
/// Suitable for diagnostic streams where the caller wants to branch
/// on "were any errors emitted".
pub trait DiagnosticSink<E>: Push<E> + Len {}
impl<E, S: Push<E> + Len + ?Sized> DiagnosticSink<E> for S {}

/// Byte-stream: per-byte push + bulk-write capability.
///
/// Codec `Encoder<T>` and `Decoder<T>` write through this trait.
pub trait ByteEmitter: Push<u8> + BulkPush<u8> {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: byte-stream trait bound; bytes are the 8-bit I/O unit of the contract; tracked: #72
impl<S: Push<u8> + BulkPush<u8> + ?Sized> ByteEmitter for S {} // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: matches ByteEmitter bound above; tracked: #72

/// Discards everything pushed to it.
///
/// For tests and dry-run code-paths that need a sink parameter but
/// do not care about stored output.
#[derive(Debug, Clone, Copy, Default)]
pub struct NullSink;

impl<T> Push<T> for NullSink {
    #[inline(always)]
    fn push(&mut self, _: T) {}
}

impl<T> BulkPush<T> for NullSink {
    #[inline(always)]
    fn push_bulk(&mut self, _: &[T])
    where
        T: Copy,
    {
    }
}

/// Counts pushes but stores nothing.
///
/// Useful when a caller only needs "how many items were emitted"
/// without paying for storage.
#[derive(Debug, Clone, Copy)]
pub struct CountingSink<T> {
    count: USize,
    _m: PhantomData<fn(T)>,
}

impl<T> CountingSink<T> {
    /// Fresh sink starting at count 0.
    pub const fn new() -> Self {
        Self {
            count: USize::ZERO,
            _m: PhantomData,
        }
    }
}

impl<T> Default for CountingSink<T> {
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Push<T> for CountingSink<T> {
    #[inline(always)]
    fn push(&mut self, _: T) {
        self.count = USize(*self.count + 1);
    }
}

impl<T> Len for CountingSink<T> {
    #[inline(always)]
    fn len(&self) -> USize {
        self.count
    }
}

/// Fans each push out to two downstream sinks.
///
/// `T: Clone` required because both downstreams receive the item.
/// `Copy` types clone for free; non-`Copy` consumers pay one clone
/// per push.
pub struct TeeSink<'a, A: ?Sized, B: ?Sized> {
    pub a: &'a mut A,
    pub b: &'a mut B,
}

impl<'a, T, A, B> Push<T> for TeeSink<'a, A, B>
where
    T: Clone,
    A: Push<T> + ?Sized,
    B: Push<T> + ?Sized,
{
    #[inline(always)]
    fn push(&mut self, item: T) {
        self.a.push(item.clone());
        self.b.push(item);
    }
}
