//! The `.comment` section of an ELF executable, where a linker leaves its
//! mark: lld writes `Linker: LLD <version>`, mold and wild their names and
//! versions, and GNU ld nothing.

use std::path::Path;

/// Every string in the `.comment` section of the 64-bit little-endian ELF
/// file at `path`, in order, or why none could be read.
pub fn comment(path: &Path) -> Result<Vec<String>, String> {
    let b = std::fs::read(path).map_err(|e| format!("{}: {e}", path.display()))?;
    if b.len() < 64 || &b[.. 4] != b"\x7fELF" || b[4] != 2 || b[5] != 1 {
        return Err(format!(
            "{}: not a 64-bit little-endian ELF file",
            path.display()
        ));
    }
    let u16_at = |o: usize| {
        b.get(o .. o + 2)
            .map(|s| u16::from_le_bytes([s[0], s[1]]) as usize)
    };
    let u32_at = |o: usize| {
        b.get(o .. o + 4)
            .map(|s| u32::from_le_bytes([s[0], s[1], s[2], s[3]]) as usize)
    };
    let u64_at = |o: usize| {
        b.get(o .. o + 8)
            .map(|s| u64::from_le_bytes(s.try_into().unwrap_or([0; 8])) as usize)
    };
    let bad = || format!("{}: a truncated ELF header", path.display());
    let shoff = u64_at(0x28).ok_or_else(bad)?;
    let shentsize = u16_at(0x3A).ok_or_else(bad)?;
    let shnum = u16_at(0x3C).ok_or_else(bad)?;
    let shstrndx = u16_at(0x3E).ok_or_else(bad)?;
    let section = |i: usize| -> Option<(usize, usize, usize)> {
        let h = shoff + i * shentsize;
        Some((u32_at(h)?, u64_at(h + 24)?, u64_at(h + 32)?))
    };
    let (_, names_off, names_size) = section(shstrndx).ok_or_else(bad)?;
    let names = b.get(names_off .. names_off + names_size).ok_or_else(bad)?;
    for i in 0 .. shnum {
        let (name, off, size) = section(i).ok_or_else(bad)?;
        let end = names[name ..]
            .iter()
            .position(|&c| c == 0)
            .map(|p| name + p);
        if end.map(|e| &names[name .. e]) != Some(b".comment".as_slice()) {
            continue;
        }
        let data = b.get(off .. off + size).ok_or_else(bad)?;
        return Ok(data
            .split(|&c| c == 0)
            .filter(|s| !s.is_empty())
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .collect());
    }
    Ok(Vec::new())
}

/// The linker's own mark among the comment strings: the one naming a linker.
pub fn linker_mark(comments: &[String]) -> Option<String> {
    comments
        .iter()
        .find(|c| c.starts_with("Linker:") || c.starts_with("mold ") || c.contains("Wild"))
        .cloned()
}
