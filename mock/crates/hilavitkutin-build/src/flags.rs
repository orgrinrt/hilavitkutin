//! The rustc flags of a set, as the wrapper appends them.
//!
//! Every flag has exactly one owner pragma, which writes it
//! ([`Pragma::write_rustflags`]). This module puts a set's flags
//! together: one space between two pragmas that wrote any.

use core::fmt::Write;

use crate::pragma::Pragma;
use crate::set::{Set, Site, Visit};
use crate::wiring::Flags;

/// A writer that puts one space between what two pragmas wrote.
struct Spaced<'a, W: Write> {
    out:   &'a mut W,
    /// Whether anything has been written.
    any:   bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state, the crate takes no arvo dependency; tracked: #72
    /// Whether the next text is the first of a pragma after one that wrote.
    owing: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state, the crate takes no arvo dependency; tracked: #72
}

impl<W: Write> Write for Spaced<'_, W> {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn write_str(&mut self, part: &str) -> core::fmt::Result { // lint:allow(no-bare-string) reason: the signature of `core::fmt::Write`; tracked: #72
        if part.is_empty() {
            return Ok(());
        }
        if self.owing {
            self.out.write_char(' ')?;
            self.owing = false;
        }
        self.any = true;
        self.out.write_str(part)
    }
}

/// The walk that writes every member's flags.
struct Flagged<'a, 's, W: Write> {
    site: &'a Site<'s>,
    out:  Spaced<'a, W>,
}

impl<W: Write> Visit for Flagged<'_, '_, W> {
    fn pragma<P: Pragma>(&mut self) -> core::fmt::Result {
        self.out.owing = self.out.any;
        P::write_rustflags(&self.site.written::<P>(), &mut self.out)
    }
}

/// Write the rustc flags of every pragma of the set `S`, separated by
/// single spaces, with none before the first and none after the last.
///
/// This is the value of `HILAVITKUTIN_BUILD_RUSTFLAGS`, which the wrapper
/// splits on spaces; no flag contains one.
pub fn write_rustflags<S: Set, W: Write>(site: &Site<'_>, out: &mut W) -> core::fmt::Result {
    S::each(&mut Flagged {
        site,
        out: Spaced {
            out,
            any: false,
            owing: false,
        },
    })
}

/// A sink that keeps only whether anything was written to it.
struct Anything {
    wrote: bool, // lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: writer state, the crate takes no arvo dependency; tracked: #72
}

impl Write for Anything {
    #[rustfmt::skip] // keeps the allow on the signature it governs
    fn write_str(&mut self, part: &str) -> core::fmt::Result { // lint:allow(no-bare-string) reason: the signature of `core::fmt::Write`; tracked: #72
        self.wrote |= !part.is_empty();
        Ok(())
    }
}

/// Whether any pragma of the set `S` writes a rustc flag at `site`, so
/// the crate has something for the wrapper to hand over.
pub fn writes_flags<S: Set>(site: &Site<'_>) -> Flags {
    let mut sink = Anything {
        wrote: false,
    };
    if write_rustflags::<S, _>(site, &mut sink).is_ok() && sink.wrote {
        Flags::Written
    } else {
        Flags::Nothing
    }
}
