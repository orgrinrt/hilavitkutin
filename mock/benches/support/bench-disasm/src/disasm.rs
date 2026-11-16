//! Reading one symbol's disassembly, and parsing objdump's lines.
//!
//! The five checks used to match raw text, in forms objdump never prints on
//! this side: `\tcall\t*` where GNU objdump pads the mnemonic with spaces
//! (`call   *%rax`), Intel operands where its default is AT&T, and the
//! `--disassemble-symbols=` flag, which is llvm-objdump's spelling and which
//! GNU objdump refuses. So on an x86_64 Linux box the gate read nothing, and
//! check 1 would have counted zero indirect calls in any body it was handed.
//!
//! This module owns both halves. `disassemble` asks whichever objdump is on
//! the path for one symbol, in the flag it accepts. `Insn::parse` turns one
//! line into a mnemonic, its operands and objdump's trailing comment, for GNU
//! and llvm output on x86_64 (AT&T) and aarch64, so every check reads
//! instructions rather than substrings of lines.

use std::path::Path;
use std::process::Command;

/// One disassembled instruction: the mnemonic, its operands, and the comment
/// objdump appends (`# 64b40 <memset@GLIBC_2.2.5>` on x86_64, `// ...` on
/// aarch64), each trimmed.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Insn<'a> {
    pub mnemonic: &'a str,
    pub operands: &'a str,
    pub comment:  &'a str,
}

impl<'a> Insn<'a> {
    /// Parse one objdump line, or `None` for a line that is not an
    /// instruction (headers, labels, blank lines).
    ///
    /// The shape is `<addr>:\t[<bytes>\t]<mnemonic><space><operands>[ <comment>]`
    /// for GNU and llvm objdump both; the raw bytes column is present unless
    /// `--no-show-raw-insn` was given.
    pub fn parse(line: &'a str) -> Option<Self> {
        let mut fields = line.split('\t');
        let addr = fields.next()?.trim();
        if !addr.ends_with(':')
            || addr.len() < 2
            || !addr[.. addr.len() - 1]
                .bytes()
                .all(|b| b.is_ascii_hexdigit())
        {
            return None;
        }
        let mut rest: Vec<&str> = fields.map(str::trim).filter(|f| !f.is_empty()).collect();
        if rest.first().is_some_and(|f| is_raw_bytes(f)) {
            rest.remove(0);
        }
        // x86_64 pads the mnemonic with spaces; GNU's aarch64 output puts a tab
        // between mnemonic and operands. Either way the instruction is the line
        // from its first field on, and that field is a slice of `line`.
        let first = rest.first()?;
        let start = first.as_ptr() as usize - line.as_ptr() as usize;
        let text = line[start ..].trim();
        let (mnemonic, rest) = match text.find(char::is_whitespace) {
            Some(i) => (&text[.. i], text[i ..].trim_start()),
            None => (text, ""),
        };
        let (operands, comment) = split_comment(rest);
        Some(Self {
            mnemonic,
            operands,
            comment,
        })
    }

    /// The symbol a trailing `<name>` comment names, without any `@version`
    /// or `+offset`.
    pub fn comment_symbol(&self) -> Option<&'a str> {
        symbol_in(self.comment).or_else(|| symbol_in(self.operands))
    }

    /// Whether this is a call, direct or indirect.
    pub fn is_call(&self) -> bool {
        matches!(
            self.mnemonic,
            "call" | "callq" | "bl" | "blr" | "blraa" | "blraaz" | "blrab" | "blrabz"
        )
    }

    /// Whether this is a call through a register or memory rather than to an
    /// address fixed in the instruction.
    pub fn is_indirect_call(&self) -> bool {
        match self.mnemonic {
            "call" | "callq" => self.operands.starts_with('*'),
            "blr" | "blraa" | "blraaz" | "blrab" | "blrabz" => true,
            _ => false,
        }
    }

    /// The register an instruction names as its destination, as its 64-bit
    /// family (`%r13d` is `%r13`, `w8` is `x8`): the last operand on x86_64
    /// AT&T, the first on aarch64. Compares, tests, pushes, stores and
    /// branches name none.
    pub fn written_register(&self) -> Option<String> {
        let m = self.mnemonic;
        if m.starts_with("cmp")
            || m.starts_with("test")
            || m.starts_with("push")
            || m.starts_with("st")
            || m == "bt"
            || m.starts_with('j')
            || matches!(m, "b" | "br" | "ret" | "cbz" | "cbnz" | "tbz" | "tbnz")
            || m.starts_with("b.")
            || self.is_call()
        {
            return None;
        }
        let ops = self.operands;
        if ops.contains('%') {
            let last = ops.rsplit(',').next()?.trim();
            return last.starts_with('%').then(|| register_family(last));
        }
        let first = ops.split(',').next()?.trim();
        let is_reg = first.len() > 1
            && first.starts_with(['x', 'w'])
            && first[1 ..].bytes().all(|b| b.is_ascii_digit());
        is_reg.then(|| register_family(first))
    }

    /// Whether this is a direct call to a function that never returns: one
    /// whose last name segment is in [`NO_RETURN`] or starts with
    /// `panic_const_`. The match is on the exact segment, so a function that
    /// merely lives in a `panicking` module and returns
    /// (`std::panicking::panic_count::is_zero_slow_path`,
    /// `std::panicking::panicking`) is a call that returns.
    ///
    /// Keeping register state across such a call assumes the instruction after
    /// it is reached only by a jump from code that never called it, which is
    /// how the compiler lays out a cold panic block. That is a guess about
    /// layout, not something the disassembly proves: if a jump lands after the
    /// call from a path where the register was overwritten, the import is
    /// still reported as resolved and the gate passes silently.
    pub fn calls_no_return(&self) -> bool {
        self.is_call()
            && !self.operands.starts_with(['*', '%', 'x'])
            && symbol_in(self.operands).is_some_and(|s| {
                let name = last_segment(s);
                NO_RETURN.contains(&name) || name.starts_with("panic_const_")
            })
    }

    /// Whether this instruction may write register `family` without naming it
    /// as its destination: a call that returns writes every caller-saved
    /// register; an x86_64 instruction with implicit destinations writes those
    /// (`div` writes `%rax` and `%rdx`, `syscall` `%rax`, `%rcx` and `%r11`); an
    /// exchange writes every register it names. Anything else writes only what
    /// [`Self::written_register`] reports, so a `lock add` to memory or a `div`
    /// leaves `%r10` as it was.
    pub fn clobbers(&self, family: &str) -> bool {
        if self.is_call() {
            return !self.calls_no_return() && caller_saved(family);
        }
        // `lock` and `rep` print as the mnemonic, the instruction they prefix
        // as the first word of the operands.
        let (m, ops) = match self.mnemonic {
            "lock" | "rep" | "repz" | "repe" | "repnz" | "repne" => {
                let rest = self.operands.trim_start();
                rest.split_once(char::is_whitespace).unwrap_or((rest, ""))
            },
            m => (m, self.operands),
        };
        let implicit: &[&str] = match m {
            _ if self.mnemonic.starts_with("rep") => &["%rax", "%rcx", "%rdi", "%rsi"],
            "mul" | "div" | "idiv" => &["%rax", "%rdx"],
            "imul" if !ops.contains(',') => &["%rax", "%rdx"],
            "cqto" | "cltd" | "cwtd" => &["%rdx"],
            "cltq" | "cwtl" | "cbtw" => &["%rax"],
            "cpuid" => &["%rax", "%rbx", "%rcx", "%rdx"],
            "rdtsc" => &["%rax", "%rdx"],
            "rdtscp" => &["%rax", "%rcx", "%rdx"],
            "syscall" => &["%rax", "%rcx", "%r11"],
            _ if m.starts_with("cmpxchg") => &["%rax", "%rdx"],
            _ => &[],
        };
        if implicit.contains(&family) {
            return true;
        }
        let exchanges = m.starts_with("xchg") || m.starts_with("xadd") || m.starts_with("cmpxchg");
        exchanges
            && ops
                .split(',')
                .map(str::trim)
                .any(|o| o.starts_with('%') && register_family(o) == family)
    }
}

/// The 64-bit register a register operand belongs to: `%eax`, `%ax` and `%al`
/// are `%rax`, `%r13d` is `%r13`, `w8` is `x8`.
pub fn register_family(reg: &str) -> String {
    if let Some(w) = reg.strip_prefix('w') {
        return format!("x{w}");
    }
    let Some(name) = reg.strip_prefix('%') else { return reg.to_string() };
    if let Some(num) = name
        .strip_prefix('r')
        .filter(|n| n.starts_with(|c: char| c.is_ascii_digit()))
    {
        let digits: String = num.chars().take_while(char::is_ascii_digit).collect();
        return format!("%r{digits}");
    }
    const LEGACY: [(&str, [&str; 5]); 8] = [
        ("rax", ["rax", "eax", "ax", "al", "ah"]),
        ("rbx", ["rbx", "ebx", "bx", "bl", "bh"]),
        ("rcx", ["rcx", "ecx", "cx", "cl", "ch"]),
        ("rdx", ["rdx", "edx", "dx", "dl", "dh"]),
        ("rsi", ["rsi", "esi", "si", "sil", "sil"]),
        ("rdi", ["rdi", "edi", "di", "dil", "dil"]),
        ("rsp", ["rsp", "esp", "sp", "spl", "spl"]),
        ("rbp", ["rbp", "ebp", "bp", "bpl", "bpl"]),
    ];
    let family = LEGACY
        .iter()
        .find(|(_, names)| names.contains(&name))
        .map_or(name, |(f, _)| f);
    format!("%{family}")
}

/// Whether a register's value is lost across a call, by the C calling
/// convention of the platform the listing is from: rax, rcx, rdx, rsi, rdi
/// and r8 to r11 on x86_64 System V, x0 to x18 on aarch64.
fn caller_saved(reg: &str) -> bool {
    if let Some(n) = reg.strip_prefix('x') {
        return n.parse::<u8>().is_ok_and(|n| n <= 18);
    }
    if let Some(n) = reg.strip_prefix("%r").and_then(|n| n.parse::<u8>().ok()) {
        return (8 ..= 11).contains(&n);
    }
    matches!(reg, "%rax" | "%rcx" | "%rdx" | "%rsi" | "%rdi")
}

/// Whether a field is objdump's raw-bytes column: two-digit hex bytes on
/// x86_64, one eight-digit word on aarch64. A mnemonic is neither (`fadd` is
/// four hex digits, and no mnemonic is eight).
fn is_raw_bytes(field: &str) -> bool {
    let hex = |t: &str| t.bytes().all(|b| b.is_ascii_hexdigit());
    let tokens: Vec<&str> = field.split_whitespace().collect();
    match tokens.as_slice() {
        [word] if word.len() == 8 => hex(word),
        ts => !ts.is_empty() && ts.iter().all(|t| t.len() == 2 && hex(t)),
    }
}

/// Split operands from objdump's comment: ` # ` on x86_64 (an aarch64
/// immediate is `#imm`, with no space after the hash), `//` on aarch64.
fn split_comment(rest: &str) -> (&str, &str) {
    if let Some(i) = rest.find(" # ") {
        return (rest[.. i].trim_end(), rest[i + 3 ..].trim());
    }
    if let Some(i) = rest.find("//") {
        return (rest[.. i].trim_end(), rest[i + 2 ..].trim());
    }
    (rest, "")
}

/// The name inside the first `<...>` of `s`, cut at `@` or `+`.
/// The last name segments of the functions that never return: the panics of
/// `core` and `std`, the unwinding entry points, and `abort`.
const NO_RETURN: &[&str] = &[
    "panic",
    "panic_fmt",
    "panic_nounwind",
    "panic_nounwind_fmt",
    "panic_nounwind_nobacktrace",
    "panic_bounds_check",
    "panic_explicit",
    "panic_display",
    "panic_str_2015",
    "panic_cannot_unwind",
    "panic_in_cleanup",
    "panic_misaligned_pointer_dereference",
    "panic_null_pointer_dereference",
    "begin_panic",
    "begin_panic_handler",
    "rust_begin_unwind",
    "rust_panic",
    "__rust_start_panic",
    "rust_panic_with_hook",
    "unwrap_failed",
    "expect_failed",
    "assert_failed",
    "slice_index_order_fail",
    "slice_start_index_len_fail",
    "slice_end_index_len_fail",
    "str_index_overflow_fail",
    "handle_alloc_error",
    "capacity_overflow",
    "_Unwind_Resume",
    "abort",
];

/// The last name segment of a symbol, in any of the three spellings objdump
/// prints: v0 mangled (`_RNvNtCs1_4core9panicking5panic`), legacy mangled
/// (`_ZN4core9panicking5panic17h<hash>E`) or demangled
/// (`core::panicking::panic`, with or without `::h<hash>`). A plain C name is
/// its own segment.
fn last_segment(sym: &str) -> &str {
    let is_hash = |s: &str| {
        s.len() == 17 && s.starts_with('h') && s[1 ..].bytes().all(|b| b.is_ascii_hexdigit())
    };
    if sym.contains("::") {
        let mut parts = sym.rsplit("::");
        let last = parts.next().unwrap_or(sym);
        return if is_hash(last) { parts.next().unwrap_or(last) } else { last };
    }
    let mut s = sym;
    if let Some(body) = s.strip_prefix("_ZN").and_then(|b| b.strip_suffix('E')) {
        s = body;
        if s.len() > 19 && s.ends_with(|c: char| c.is_ascii_hexdigit()) {
            let tail = &s[s.len() - 19 ..];
            if tail.starts_with("17") && is_hash(&tail[2 ..]) {
                s = &s[.. s.len() - 19];
            }
        }
    } else if !sym.starts_with("_R") {
        return sym;
    }
    // The segment is the suffix `<len><ident>` whose decimal length matches.
    let bytes = s.as_bytes();
    for i in 1 .. bytes.len() {
        if bytes[i].is_ascii_digit() || !bytes[i - 1].is_ascii_digit() {
            continue;
        }
        let start = s[.. i]
            .rfind(|c: char| !c.is_ascii_digit())
            .map_or(0, |p| p + 1);
        if s[start .. i].parse::<usize>().ok() == Some(bytes.len() - i) {
            return &s[i ..];
        }
    }
    sym
}

fn symbol_in(s: &str) -> Option<&str> {
    let open = s.find('<')?;
    let close = open + s[open ..].find('>')?;
    let inner = &s[open + 1 .. close];
    let end = inner.find(['@', '+']).unwrap_or(inner.len());
    (end > 0).then_some(&inner[.. end])
}

/// Whether a `<name>` comment points into the global offset table of an
/// import: `name@GLIBC_2.2.5` and the like, or a `@plt` stub.
fn names_import(s: &str) -> bool {
    let Some(open) = s.find('<') else { return false };
    s[open ..]
        .split('>')
        .next()
        .is_some_and(|inner| inner.contains('@'))
}

/// The indirect calls of one body, split by what they reach.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct IndirectCalls {
    /// Calls whose target was loaded from an import's GOT slot, directly
    /// (`call *off(%rip)  # <memset@...>`) or through a register that slot was
    /// loaded into and copied between. These are the dynamic linker's, not a
    /// stored function pointer.
    pub to_imports: Vec<String>,
    /// Every other indirect call, by address: a target the body read from
    /// data, which is what check 1 exists to refuse.
    pub unresolved: Vec<String>,
}

/// Classify every indirect call in `disasm`.
///
/// The tracking is straight-line over the listing and deliberately
/// conservative: a register holds an import only after a load from an import
/// slot or a copy from a register that does, and any other write forgets it.
/// A branch into a block where the register was set elsewhere therefore reads
/// as unresolved, never as an import, so the error is on the side of failing.
pub fn indirect_calls(disasm: &str) -> IndirectCalls {
    let mut holds: Vec<(String, String)> = Vec::new();
    let mut out = IndirectCalls::default();
    let mut base: Option<u64> = None;
    for line in disasm.lines() {
        let Some(insn) = Insn::parse(line) else {
            // `0000000000020bc0 <name>:` opens a symbol; offsets count from it.
            if let Some((addr, _)) = line
                .trim_end()
                .strip_suffix(">:")
                .and_then(|l| l.split_once(" <"))
            {
                base = u64::from_str_radix(addr.trim(), 16).ok();
            }
            continue;
        };
        if insn.is_indirect_call() {
            let target = insn
                .operands
                .trim_start_matches('*')
                .split_whitespace()
                .next()
                .unwrap_or("");
            let through_slot = !target.starts_with('%') && names_import(insn.comment);
            let family = register_family(target);
            let held = holds
                .iter()
                .find(|(r, _)| *r == family)
                .map(|(_, s)| s.clone());
            match (through_slot, held) {
                (true, _) => {
                    out.to_imports
                        .push(insn.comment_symbol().unwrap_or("?").to_string())
                },
                (false, Some(sym)) => out.to_imports.push(sym),
                (false, None) => out.unresolved.push(location(line, base)),
            }
        }
        holds.retain(|(r, _)| !insn.clobbers(r));
        let Some(reg) = insn.written_register() else { continue };
        let source = if insn.mnemonic == "mov"
            && names_import(insn.comment)
            && insn.operands.contains("(%rip)")
        {
            insn.comment_symbol().map(str::to_string)
        } else if insn.mnemonic == "mov" {
            let from = register_family(insn.operands.split(',').next().unwrap_or("").trim());
            holds
                .iter()
                .find(|(r, _)| *r == from)
                .map(|(_, s)| s.clone())
        } else {
            None
        };
        holds.retain(|(r, _)| *r != reg);
        if let Some(sym) = source {
            holds.push((reg, sym));
        }
    }
    out
}

/// Where an instruction is: `+0x<offset>` into the symbol whose header came
/// before it, which stays the same when the layout of the rest of the binary
/// moves, or its absolute address when no header was seen.
fn location(line: &str, base: Option<u64>) -> String {
    let addr = line.split(':').next().unwrap_or("").trim();
    match (base, u64::from_str_radix(addr, 16)) {
        (Some(b), Ok(a)) if a >= b => format!("+{:#x}", a - b),
        _ => addr.to_string(),
    }
}

/// One symbol's disassembly from the objdump on the path, or `None` when no
/// form of the per-symbol flag gave back that symbol's instructions.
///
/// GNU objdump takes `--disassemble=<sym>`; llvm-objdump (macOS's `objdump`)
/// takes `--disassemble-symbols=<sym>`. Each form is tried, as the name given
/// and with the leading underscore Mach-O adds, and a result counts only if
/// the symbol's own `<name>:` header came back, since both exit zero with a
/// bare file header when the symbol is absent.
pub fn disassemble(dylib: &Path, symbol: &str) -> Option<String> {
    for flag in ["--disassemble", "--disassemble-symbols"] {
        for name in [symbol.to_string(), format!("_{symbol}")] {
            let Ok(out) = Command::new("objdump")
                .arg("-d")
                .arg("--no-show-raw-insn")
                .arg(format!("{flag}={name}"))
                .arg(dylib)
                .output()
            else {
                return None;
            };
            if !out.status.success() {
                continue;
            }
            let text = String::from_utf8_lossy(&out.stdout).into_owned();
            if text.contains(&format!("<{name}>:")) {
                return Some(text);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests;
