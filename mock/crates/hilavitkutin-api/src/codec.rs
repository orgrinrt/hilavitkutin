//! Stateful chunked codec contracts + oneshot adapters.
//!
//! Primary form is chunked: `feed` drives one value (Encoder) or a
//! byte chunk (Decoder); `finish` flushes framing bytes / finalises.
//! Streaming consumers (clause bytecode loader, paradox save parser)
//! use the primary form directly.
//!
//! Oneshot adapters (`EncoderExt`, `DecoderExt`) ride blanket impls
//! so single-value consumers can write `codec.encode_one(...)` /
//! `codec.decode_all(...)` without managing feed / finish manually.

use notko::Outcome;

use crate::capability::Push;
use crate::sink::ByteEmitter;

/// Chunked encoder.
///
/// `feed` consumes one value and writes its encoded bytes through
/// the byte emitter. `finish` consumes the encoder and may flush a
/// trailing framing sequence.
pub trait Encoder<T> {
    /// Encode one value; write bytes through `out`.
    fn feed<B: ByteEmitter>(&mut self, v: &T, out: &mut B);

    /// Finalise the stream; writes any trailing framing bytes.
    fn finish<B: ByteEmitter>(self, out: &mut B);
}

/// Chunked decoder.
///
/// `feed` consumes a byte chunk, emits decoded values to `out`,
/// returns the unconsumed tail (which the caller either retains for
/// the next `feed` call or treats as an error via `DecoderExt`).
/// `finish` errors if the decoder carries an unfinished frame.
pub trait Decoder<T> {
    /// Feed a byte chunk. Decoded values arrive via `out`; returns
    /// unconsumed bytes.
    fn feed<'a, S: Push<T>>(
        &mut self,
        chunk: &'a [u8],
        out: &mut S,
    ) -> Outcome<&'a [u8], DecodeError>;

    /// Finalise the decoder. Errors if an in-progress frame remains.
    fn finish(self) -> Outcome<(), DecodeError>;
}

/// Decode-side failure modes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DecodeError {
    /// Input ended mid-value.
    Truncated,
    /// Input bytes did not match the expected encoding.
    Invalid,
    /// Decoder consumed a valid frame but bytes remain.
    OverLength,
}

/// Oneshot encode helper.
///
/// Consumes the encoder, feeds one value, then finishes. Blanket
/// impl makes this available on every `Encoder<T>` without explicit
/// opt-in from the implementor.
pub trait EncoderExt<T>: Encoder<T> + Sized {
    /// All-at-once encode: feed `v` then finish.
    fn encode_one<B: ByteEmitter>(self, v: &T, out: &mut B) {
        let mut s = self;
        s.feed(v, out);
        s.finish(out);
    }
}
impl<T, E: Encoder<T>> EncoderExt<T> for E {}

/// Oneshot decode helper.
///
/// Consumes the decoder, feeds one complete input, then finishes.
/// Errors with `DecodeError::OverLength` if `feed` leaves unconsumed
/// bytes.
pub trait DecoderExt<T>: Decoder<T> + Sized {
    /// All-at-once decode: feed `bytes`, errors on trailing bytes,
    /// then finish.
    fn decode_all<S: Push<T>>(
        self,
        bytes: &[u8],
        out: &mut S,
    ) -> Outcome<(), DecodeError> {
        let mut d = self;
        let rest = match d.feed(bytes, out) {
            Outcome::Ok(r) => r,
            Outcome::Err(e) => return Outcome::Err(e),
        };
        if !rest.is_empty() {
            return Outcome::Err(DecodeError::OverLength);
        }
        d.finish()
    }
}
impl<T, D: Decoder<T>> DecoderExt<T> for D {}
