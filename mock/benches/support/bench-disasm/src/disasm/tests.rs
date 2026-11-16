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
fn a_call_that_never_returns_keeps_the_import() {
    // The shape the accumulator fixture's body takes: the GOT slot is
    // loaded, a cold bounds-check panic sits between, and the slot's
    // register is called after it.
    let across_panic = "\
   23a3a:\tmov    0x44c1f(%rip),%r10        # 68660 <memset@GLIBC_2.2.5>
   23b07:\tcall   2c933 <_RNvNtCslfAf2Mv2Tp5_4core9panicking18panic_bounds_check>
   23bcd:\tmov    %r10,%r12
   23bd0:\tcall   *%r10
   23be2:\tcall   *%r12
";
    assert_eq!(indirect_calls(across_panic).to_imports, [
        "memset", "memset"
    ]);
    // The case that must still fail: a call that returns forgets r10.
    let across_return = "\
   23a3a:\tmov    0x44c1f(%rip),%r10        # 68660 <memset@GLIBC_2.2.5>
   23b07:\tcall   2c933 <_RNvNtCslfAf2Mv2Tp5_4core3fmt5write>
   23bd0:\tcall   *%r10
";
    assert_eq!(indirect_calls(across_return).unresolved, ["23bd0"]);
}

#[test]
fn a_returning_function_under_panicking_forgets_the_import() {
    // `std::panicking::panic_count::is_zero_slow_path` sits in the
    // `panicking` module and returns: r10 does not survive it.
    let across_panic_count = "\
   23a3a:\tmov    0x44c1f(%rip),%r10        # 68660 <memset@GLIBC_2.2.5>
   23b07:\tcall   2c933 <_RNvNtNtCs1_3std9panicking11panic_count17is_zero_slow_path>
   23bd0:\tcall   *%r10
";
    let calls = indirect_calls(across_panic_count);
    assert!(calls.to_imports.is_empty());
    assert_eq!(calls.unresolved, ["23bd0"]);
    // `std::panicking::panicking` returns too, in either mangling.
    for callee in [
        "_RNvNtCs1_3std9panicking9panicking",
        "_ZN3std9panicking9panicking17h0123456789abcdefE",
        "std::panicking::panicking",
    ] {
        let text = format!(
            "   23a3a:\tmov    0x44c1f(%rip),%r10        # 68660 \
             <memset@GLIBC_2.2.5>\n   23b07:\tcall   2c933 <{callee}>\n   \
             23bd0:\tcall   *%r10\n"
        );
        assert_eq!(indirect_calls(&text).unresolved, ["23bd0"], "{callee}");
    }
    // The never-returning names keep it, in each mangling.
    for callee in [
        "_ZN4core9panicking9panic_fmt17h0123456789abcdefE",
        "core::panicking::panic_nounwind",
        "_RNvNtCs1_4core9panicking5panic",
        "_RNvNtCs1_3std9panicking11begin_panic",
        "_RNvNtCs1_4core9panicking24panic_const_add_overflow",
        "abort",
    ] {
        let text = format!(
            "   23a3a:\tmov    0x44c1f(%rip),%r10        # 68660 \
             <memset@GLIBC_2.2.5>\n   23b07:\tcall   2c933 <{callee}>\n   \
             23bd0:\tcall   *%r10\n"
        );
        assert_eq!(indirect_calls(&text).to_imports, ["memset"], "{callee}");
    }
}

#[test]
fn an_implicit_destination_forgets_only_the_registers_it_writes() {
    // The run_parallel worker body's shape: a memset slot in r10 across a
    // `lock add`, a `lock xadd` and a `div`, none of which write r10.
    let kept = "\
   1eeb7:\tmov    0x497a2(%rip),%r10        # 68660 <memset@GLIBC_2.2.5>
   1eee6:\tlock add %rcx,0x14120(%rbx,%r14,8)
   1eefe:\tlock xadd %eax,(%rcx)
   1ef28:\tdiv    %ecx
   1efca:\tmov    %r10,%r15
   1efcd:\tcall   *%r10
   1efd0:\tcall   *%r15
";
    assert_eq!(indirect_calls(kept).to_imports, ["memset", "memset"]);
    // The cases that must still fail: each instruction forgets the register
    // it does write.
    for (between, reg) in [
        ("div    %ecx", "%rdx"),
        ("lock xadd %r10d,(%rcx)", "%r10"),
        ("syscall", "%r11"),
        ("rep stos %rax,%es:(%rdi)", "%rdi"),
    ] {
        let disasm = format!(
            "   1000:\tmov    0x497a2(%rip),{reg}        # 68660 <memset@GLIBC_2.2.5>\n   \
             1004:\t{between}\n   1008:\tcall   *{reg}\n"
        );
        assert_eq!(
            indirect_calls(&disasm).unresolved,
            ["1008"],
            "{between} keeps {reg}"
        );
    }
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
