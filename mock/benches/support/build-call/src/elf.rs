//! The `.comment` section of an ELF executable, where a linker leaves its
//! mark: lld writes `Linker: LLD <version>`, GNU ld nothing, and the plan names
//! mold and wild as writing their names and versions (a claim not yet shown).

use std::path::Path;

/// Every string in the `.comment` section of the 64-bit little-endian ELF
/// file at `path`, in order, or why none could be read.
///
/// FIXME(the_build_and_link_bench_assumes_an_elf_unix_host_and_fails): a
/// Mach-O, PE or 32-bit executable is refused here, so a link cell on such a
/// target fails on its missing mark instead of being left out by declaration.
pub fn comment(path: &Path) -> Result<Vec<String>, String> {
    let b = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    comment_of(&b).map_err(|e| format!("{}: {e}", path.display()))
}

/// [`comment`] over the bytes of a file. Every offset and size the file
/// states is checked against its length, so a truncated or corrupt file is an
/// error and never a panic.
pub fn comment_of(b: &[u8]) -> Result<Vec<String>, String> {
    if b.len() < 64 || &b[.. 4] != b"\x7fELF" || b[4] != 2 || b[5] != 1 {
        return Err("not a 64-bit little-endian ELF file".into());
    }
    let at = |o: usize, len: usize| b.get(o .. o.checked_add(len)?);
    let u16_at = |o: usize| at(o, 2).map(|s| u16::from_le_bytes([s[0], s[1]]) as usize);
    let u32_at = |o: usize| at(o, 4).map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as usize);
    let u64_at = |o: usize| {
        at(o, 8)
            .and_then(|s| s.try_into().ok())
            .and_then(|s| usize::try_from(u64::from_le_bytes(s)).ok())
    };
    let bad = |what: &str| format!("a truncated or corrupt ELF file: {what}");
    let shoff = u64_at(0x28).ok_or_else(|| bad("the section table offset"))?;
    let shentsize = u16_at(0x3A).ok_or_else(|| bad("the section entry size"))?;
    let shnum = u16_at(0x3C).ok_or_else(|| bad("the section count"))?;
    let shstrndx = u16_at(0x3E).ok_or_else(|| bad("the section name index"))?;
    let section = |i: usize| -> Option<(usize, usize, usize)> {
        let h = shoff.checked_add(i.checked_mul(shentsize)?)?;
        Some((
            u32_at(h)?,
            u64_at(h.checked_add(24)?)?,
            u64_at(h.checked_add(32)?)?,
        ))
    };
    let (_, names_off, names_size) =
        section(shstrndx).ok_or_else(|| bad("the section name table's header"))?;
    let names = at(names_off, names_size).ok_or_else(|| bad("the section name table"))?;
    for i in 0 .. shnum {
        let (name, off, size) = section(i).ok_or_else(|| bad("a section header"))?;
        let named = names
            .get(name ..)
            .ok_or_else(|| bad("a section name offset"))?;
        let end = named.iter().position(|&c| c == 0);
        if end.map(|e| &named[.. e]) != Some(b".comment".as_slice()) {
            continue;
        }
        let data = at(off, size).ok_or_else(|| bad("the `.comment` section"))?;
        return Ok(data
            .split(|&c| c == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect());
    }
    Ok(Vec::new())
}

/// The marks a compiler leaves in `.comment`: rustc's own, and those the C
/// runtime's objects carry from the compiler that built them.
const COMPILER_MARKS: [&str; 3] = ["rustc version ", "GCC: ", "clang version "];

/// What is left among the comment strings once every compiler's mark is
/// taken out, which is what the linker wrote, joined by ` | ` where it wrote
/// more than one string. `None` where the linker wrote nothing, as GNU ld does.
/// No linker is recognised by name here: what an arm's linker must write is
/// held by [`crate::linker`], and a string of any other linker is kept whole
/// so the record shows it.
pub fn linker_mark(comments: &[String]) -> Option<String> {
    let left: Vec<&str> = comments
        .iter()
        .map(String::as_str)
        .filter(|c| !COMPILER_MARKS.iter().any(|m| c.starts_with(m)))
        .collect();
    (!left.is_empty()).then(|| left.join(" | "))
}
