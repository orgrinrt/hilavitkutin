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

    /// Whether this instruction may write registers it does not name: a call
    /// (the callee may clobber every caller-saved register) and the x86_64
    /// instructions with implicit destinations.
    pub fn clobbers_implicitly(&self) -> bool {
        self.is_call()
            || self.mnemonic.starts_with("rep")
            || matches!(
                self.mnemonic,
                "mul"
                    | "div"
                    | "idiv"
                    | "cqto"
                    | "cltq"
                    | "cltd"
                    | "cpuid"
                    | "rdtsc"
                    | "rdtscp"
                    | "syscall"
                    | "xchg"
                    | "cmpxchg"
                    | "xadd"
            )
            || self.mnemonic.starts_with("lock")
            || self.mnemonic == "imul" && !self.operands.contains(',')
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
        if insn.clobbers_implicitly() {
            holds.retain(|(r, _)| !caller_saved(r));
        }
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
mod tests {
    use super::*;

    /// Lines as GNU objdump 2.4x prints them on x86_64, raw bytes shown.
    const GNU_X86: &str = "\
   1fa70:\t4c 8b 3d d9 50 04 00 \tmov    0x450d9(%rip),%r15        # 64b50 <syscall@GLIBC_2.2.5>
   1fa80:\tbf ca 00 00 00       \tmov    $0xca,%edi
   1fa92:\t41 ff d7             \tcall   *%r15
   11ab8:\tff 15 72 30 05 00    \tcall   *0x53072(%rip)        # 64b30 <clock_gettime@GLIBC_2.17>
   20f6d:\t49 8b 07             \tmov    (%r15),%rax
   20f70:\t48 85 c0             \ttest   %rax,%rax
   20f78:\tff d0                \tcall   *%rax
";

    #[test]
    fn a_gnu_line_parses_into_its_parts() {
        let line = GNU_X86.lines().next().unwrap();
        let insn = Insn::parse(line).unwrap();
        assert_eq!(insn.mnemonic, "mov");
        assert_eq!(insn.operands, "0x450d9(%rip),%r15");
        assert_eq!(insn.comment, "64b50 <syscall@GLIBC_2.2.5>");
        assert_eq!(insn.comment_symbol(), Some("syscall"));
        assert_eq!(insn.written_register().as_deref(), Some("%r15"));
    }

    #[test]
    fn sub_registers_belong_to_their_family() {
        for (sub, family) in [
            ("%eax", "%rax"),
            ("%al", "%rax"),
            ("%r13d", "%r13"),
            ("%r9b", "%r9"),
            ("%sil", "%rsi"),
            ("w8", "x8"),
            ("%xmm0", "%xmm0"),
        ] {
            assert_eq!(register_family(sub), family, "{sub}");
        }
    }

    #[test]
    fn a_narrow_write_and_a_call_each_forget_the_import() {
        let narrowed = "\
   1c997:\tmov    0x481a2(%rip),%r13        # 64b40 <memset@GLIBC_2.2.5>
   1c288:\tmov    $0x1,%r13d
   1c9a4:\tcall   *%r13
";
        assert_eq!(indirect_calls(narrowed).unresolved, ["1c9a4"]);
        // rax is caller-saved, so the first call may have replaced it; r13 is
        // callee-saved and survives.
        let across_call = "\
   1c990:\tmov    0x481a2(%rip),%rax        # 64b40 <memset@GLIBC_2.2.5>
   1c997:\tmov    0x481a2(%rip),%r13        # 64b40 <memset@GLIBC_2.2.5>
   1c99e:\tcall   *%r13
   1c9b0:\tcall   *%rax
";
        let calls = indirect_calls(across_call);
        assert_eq!(calls.to_imports, ["memset"]);
        assert_eq!(calls.unresolved, ["1c9b0"]);
    }

    #[test]
    fn headers_and_labels_are_not_instructions() {
        assert_eq!(Insn::parse(""), None);
        assert_eq!(Insn::parse("Disassembly of section .text:"), None);
        assert_eq!(
            Insn::parse("000000000001f500 <asm_gate_column_chain>:"),
            None
        );
        assert_eq!(Insn::parse("   1fa92:\t41 ff d7 "), None);
    }

    #[test]
    fn a_padded_indirect_call_is_seen() {
        // The form the old `\tcall\t*` match never matched.
        let insn = Insn::parse("   1fa92:\t41 ff d7             \tcall   *%r15").unwrap();
        assert!(insn.is_indirect_call());
        assert!(
            !Insn::parse("   1cab2:\te8 59 e3 ff ff \tcall   1ae10 <f>")
                .unwrap()
                .is_indirect_call()
        );
    }

    #[test]
    fn import_calls_resolve_and_a_loaded_pointer_does_not() {
        let calls = indirect_calls(GNU_X86);
        assert_eq!(calls.to_imports, ["syscall", "clock_gettime"]);
        assert_eq!(calls.unresolved, ["20f78"]);
        // Under a symbol header the location is an offset into the symbol.
        let headed = format!("0000000000020f00 <asm_gate_x>:\n{GNU_X86}");
        assert_eq!(indirect_calls(&headed).unresolved, ["+0x78"]);
    }

    #[test]
    fn a_copy_keeps_the_import_and_an_overwrite_forgets_it() {
        let copied = "\
   1c997:\tmov    0x481a2(%rip),%r13        # 64b40 <memset@GLIBC_2.2.5>
   1caee:\tmov    %r13,%rax
   1cb4b:\tcall   *%rax
";
        assert_eq!(indirect_calls(copied).to_imports, ["memset"]);
        let overwritten = "\
   1c997:\tmov    0x481a2(%rip),%r13        # 64b40 <memset@GLIBC_2.2.5>
   1c9a0:\tadd    $0x8,%r13
   1c9a4:\tcall   *%r13
";
        assert_eq!(indirect_calls(overwritten).unresolved, ["1c9a4"]);
    }

    #[test]
    fn an_aarch64_blr_is_indirect_and_unresolved() {
        let a64 = "   8f0:\tf9400108 \tldr\tx8, [x8, #16]\n   8f4:\td63f0100 \tblr\tx8\n";
        let calls = indirect_calls(a64);
        assert!(calls.to_imports.is_empty());
        assert_eq!(calls.unresolved, ["8f4"]);
        let imm = Insn::parse("   8f8:\td2802000 \tmov\tx0, #0x100").unwrap();
        assert_eq!(imm.operands, "x0, #0x100");
        assert_eq!(imm.written_register().as_deref(), Some("x0"));
        // A mnemonic that is also hex digits, behind raw bytes that contain it.
        let add = Insn::parse("   900:\t8add0000 \tadd\tx8, x8, #0x1").unwrap();
        assert_eq!((add.mnemonic, add.operands), ("add", "x8, x8, #0x1"));
        let bare = Insn::parse("   904:\tfadd\td0, d1, d2").unwrap();
        assert_eq!((bare.mnemonic, bare.operands), ("fadd", "d0, d1, d2"));
    }
}
