//! Smoke tests for Encoder + Decoder round-trip.

use core::mem::MaybeUninit;

use arvo::USize;
use hilavitkutin_api::{
    BulkPush, DecodeError, Decoder, DecoderExt, Encoder, EncoderExt, Len, Push,
};
use notko::Outcome;

/// Bounded byte buffer: implements `Push<u8>` + `BulkPush<u8>` so it
/// satisfies `ByteEmitter`.
struct ByteBuf<const N: usize> {
    bytes: [u8; N],
    cursor: usize,
}

impl<const N: usize> ByteBuf<N> {
    fn new() -> Self {
        Self {
            bytes: [0u8; N],
            cursor: 0,
        }
    }

    fn written(&self) -> &[u8] {
        &self.bytes[..self.cursor]
    }
}

impl<const N: usize> Push<u8> for ByteBuf<N> {
    fn push(&mut self, b: u8) {
        assert!(self.cursor < N, "ByteBuf overflow");
        self.bytes[self.cursor] = b;
        self.cursor += 1;
    }
}

impl<const N: usize> BulkPush<u8> for ByteBuf<N> {
    fn push_bulk(&mut self, items: &[u8]) {
        assert!(self.cursor + items.len() <= N, "ByteBuf overflow");
        self.bytes[self.cursor..self.cursor + items.len()].copy_from_slice(items);
        self.cursor += items.len();
    }
}

/// Bounded item sink for decoded values.
struct ItemSink<T: Copy, const N: usize> {
    items: [MaybeUninit<T>; N],
    count: usize,
}

impl<T: Copy, const N: usize> ItemSink<T, N> {
    fn new() -> Self {
        Self {
            items: [MaybeUninit::uninit(); N],
            count: 0,
        }
    }

    fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self.items.as_ptr().cast::<T>(), self.count) }
    }
}

impl<T: Copy, const N: usize> Push<T> for ItemSink<T, N> {
    fn push(&mut self, item: T) {
        assert!(self.count < N, "ItemSink overflow");
        self.items[self.count].write(item);
        self.count += 1;
    }
}

impl<T: Copy, const N: usize> Len for ItemSink<T, N> {
    fn len(&self) -> USize {
        USize(self.count)
    }
}

/// Minimal little-endian u32 codec.
#[derive(Default)]
struct U32Le;

impl Encoder<u32> for U32Le {
    fn feed<B: hilavitkutin_api::ByteEmitter>(&mut self, v: &u32, out: &mut B) {
        out.push_bulk(&v.to_le_bytes());
    }

    fn finish<B: hilavitkutin_api::ByteEmitter>(self, _out: &mut B) {}
}

impl Decoder<u32> for U32Le {
    fn feed<'a, S: Push<u32>>(
        &mut self,
        chunk: &'a [u8],
        out: &mut S,
    ) -> Outcome<&'a [u8], DecodeError> {
        let mut rest = chunk;
        while rest.len() >= 4 {
            let mut buf = [0u8; 4];
            buf.copy_from_slice(&rest[..4]);
            out.push(u32::from_le_bytes(buf));
            rest = &rest[4..];
        }
        Outcome::Ok(rest)
    }

    fn finish(self) -> Outcome<(), DecodeError> {
        Outcome::Ok(())
    }
}

#[test]
fn u32_le_round_trip() {
    let inputs = [5u32, 42, 7];
    let mut buf = ByteBuf::<64>::new();
    for v in &inputs {
        U32Le.encode_one(v, &mut buf);
    }
    assert_eq!(buf.written().len(), 12);

    let mut out = ItemSink::<u32, 8>::new();
    let r = U32Le.decode_all(buf.written(), &mut out);
    assert!(matches!(r, Outcome::Ok(())));
    assert_eq!(out.as_slice(), &[5, 42, 7]);
}

#[test]
fn decode_all_overlength_on_trailing_byte() {
    // 4 bytes make one u32; 5 bytes means one trailing byte unconsumed.
    let bytes = [1u8, 0, 0, 0, 99];
    let mut out = ItemSink::<u32, 4>::new();
    let r = U32Le.decode_all(&bytes, &mut out);
    assert!(matches!(r, Outcome::Err(DecodeError::OverLength)));
    // Partial decode still populated the sink before OverLength fired.
    assert_eq!(out.as_slice(), &[1u32]);
}

#[test]
fn decode_all_overlength_on_partial_frame() {
    // 2 bytes: no full u32 yet. U32Le returns Ok with 2 leftover,
    // decode_all then reports OverLength.
    let bytes = [1u8, 2];
    let mut out = ItemSink::<u32, 4>::new();
    let r = U32Le.decode_all(&bytes, &mut out);
    assert!(matches!(r, Outcome::Err(DecodeError::OverLength)));
    assert_eq!(out.as_slice(), &[] as &[u32]);
}
