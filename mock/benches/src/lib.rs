//! Shared bench infrastructure for the hilavitkutin runtime megaround.
//!
//! The only shared item today is the platform-aware disassembly checker. It is
//! a lib module so both the reporting bench bin (`src/main.rs`) and the standing
//! ASM-dispatch gate bin (`src/bin/asm_gate.rs`) import one implementation of
//! the five checks rather than duplicating the aarch64 / x86_64 patterns.

pub mod disasm_5check;
