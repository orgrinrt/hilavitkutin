//! The `.comment` reader on a small ELF file built here, whole, cut short at
//! every length, and with each offset it trusts pointed outside the file:
//! every broken form is an error, none a panic, and a cut past every byte it
//! reads changes nothing.

use build_call::elf::comment_of;

const SHOFF: usize = 104;
const NAME_OF_COMMENT: u32 = 11;
const COMMENT_AT: u64 = 84;

/// A 64-bit little-endian ELF file with a null section, its section name
/// table and a `.comment` section holding `Linker: LLD 1`.
fn elf() -> Vec<u8> {
    let mut b = vec![0u8; SHOFF + 3 * 64];
    b[.. 4].copy_from_slice(b"\x7fELF");
    b[4] = 2;
    b[5] = 1;
    b[0x28 .. 0x30].copy_from_slice(&(SHOFF as u64).to_le_bytes());
    b[0x3A .. 0x3C].copy_from_slice(&64u16.to_le_bytes());
    b[0x3C .. 0x3E].copy_from_slice(&3u16.to_le_bytes());
    b[0x3E .. 0x40].copy_from_slice(&1u16.to_le_bytes());
    let names = b"\0.shstrtab\0.comment\0";
    b[64 .. 64 + names.len()].copy_from_slice(names);
    let comment = b"Linker: LLD 1\0";
    b[84 .. 84 + comment.len()].copy_from_slice(comment);
    header(&mut b, 1, 1, 64, names.len() as u64);
    header(&mut b, 2, NAME_OF_COMMENT, COMMENT_AT, comment.len() as u64);
    b
}

fn header(b: &mut [u8], i: usize, name: u32, off: u64, size: u64) {
    let h = SHOFF + i * 64;
    b[h .. h + 4].copy_from_slice(&name.to_le_bytes());
    b[h + 24 .. h + 32].copy_from_slice(&off.to_le_bytes());
    b[h + 32 .. h + 40].copy_from_slice(&size.to_le_bytes());
}

#[test]
fn a_whole_file_gives_its_comment() {
    assert_eq!(comment_of(&elf()), Ok(vec!["Linker: LLD 1".to_string()]));
}

/// The end of the last bytes the reader reads: the offset and size fields of
/// the last section header. What follows them in the header is never read.
const READ_END: usize = SHOFF + 2 * 64 + 40;

#[test]
fn a_file_cut_short_of_what_the_reader_reads_is_an_error_and_past_it_is_not() {
    let whole = elf();
    for len in 0 .. READ_END {
        assert!(comment_of(&whole[.. len]).is_err(), "cut at {len}");
    }
    for len in READ_END ..= whole.len() {
        assert_eq!(
            comment_of(&whole[.. len]),
            comment_of(&whole),
            "cut at {len}, past every byte the reader reads"
        );
    }
}

#[test]
fn an_offset_or_size_outside_the_file_is_an_error() {
    let mut name_out = elf();
    header(&mut name_out, 2, 10_000, COMMENT_AT, 14);
    let mut data_past_the_end = elf();
    header(&mut data_past_the_end, 2, NAME_OF_COMMENT, u64::MAX - 4, 14);
    let mut size_past_the_end = elf();
    header(
        &mut size_past_the_end,
        2,
        NAME_OF_COMMENT,
        COMMENT_AT,
        u64::MAX,
    );
    let mut table_past_the_end = elf();
    table_past_the_end[0x28 .. 0x30].copy_from_slice(&u64::MAX.to_le_bytes());
    let mut names_out = elf();
    header(&mut names_out, 1, 1, u64::MAX, 20);
    for (what, b) in [
        ("a section name past its table", name_out),
        (
            "a section offset near the top of the address space",
            data_past_the_end,
        ),
        ("a section size past the file", size_past_the_end),
        ("a section table past the file", table_past_the_end),
        ("a name table past the file", names_out),
    ] {
        let said = comment_of(&b);
        assert!(said.is_err(), "{what}: {said:?}");
    }
}
