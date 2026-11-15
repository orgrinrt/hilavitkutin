//! The platform-aware disassembly checker shared by the bench tree.
//!
//! The hooks library runs the five checks after every `dispatch_*` cell, and
//! the ASM-dispatch gate in `asm_gate_fixtures` runs them over its fixtures, so
//! both import one implementation of the aarch64 and x86_64 patterns rather
//! than each carrying its own.

pub mod disasm;
pub mod disasm_5check;
