//! The target's facts, as cargo hands them to a build script.
//!
//! The script reads `CARGO_CFG_TARGET_OS`, `CARGO_CFG_TARGET_ENV`,
//! `CARGO_CFG_TARGET_ARCH` and `CARGO_CFG_TARGET_FEATURE`, and
//! [`Target::from_cargo`] names what they say. What a target admits is
//! asked of it by [`Pragma::admitted`](crate::Pragma::admitted).

use notko::Maybe;

/// The operating system of the target.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Os {
    Linux,
    Android,
    FreeBsd,
    NetBsd,
    OpenBsd,
    MacOs,
    Ios,
    Windows,
    Wasi,
    /// A value the crate has no name for, or none given.
    #[default]
    Other,
}

/// The environment of the target, its C library or ABI.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Env {
    Gnu,
    Musl,
    Msvc,
    /// A value the crate has no name for, or none given.
    #[default]
    Other,
}

/// The architecture of the target.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Arch {
    X86_64,
    Aarch64,
    Riscv64,
    Wasm32,
    Wasm64,
    /// A value the crate has no name for, or none given.
    #[default]
    Other,
}

/// The format of the target's object files.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum ObjectFormat {
    Elf,
    MachO,
    Coff,
    Wasm,
    /// A target the crate cannot place.
    #[default]
    Other,
}

/// The widest instruction set among the target's features.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub enum Isa {
    /// The baseline 64-bit instruction set, with no SIMD assumed.
    #[default]
    Iss64,
    /// x86_64 with AVX2.
    Avx2,
    /// x86_64 with AVX-512.
    Avx512,
    /// AArch64 with NEON.
    Neon,
    /// AArch64 with SVE.
    Sve,
}

/// The target a crate is built for.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Target {
    pub os:     Os,
    pub env:    Env,
    pub arch:   Arch,
    pub format: ObjectFormat,
    pub isa:    Isa,
}

impl Target {
    /// The target from the four values a build script read. A variable
    /// cargo did not set is `Maybe::Isnt`, and reads as `Other`, or as
    /// the baseline instruction set.
    pub fn from_cargo(
        os: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env values as the build script read them; tracked: #72
        env: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env values as the build script read them; tracked: #72
        arch: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env values as the build script read them; tracked: #72
        features: Maybe<&str>, // lint:allow(no-bare-string) reason: cargo env values as the build script read them; tracked: #72
    ) -> Self {
        let os = match os {
            Maybe::Is("linux") => Os::Linux,
            Maybe::Is("android") => Os::Android,
            Maybe::Is("freebsd") => Os::FreeBsd,
            Maybe::Is("netbsd") => Os::NetBsd,
            Maybe::Is("openbsd") => Os::OpenBsd,
            Maybe::Is("macos") => Os::MacOs,
            Maybe::Is("ios") => Os::Ios,
            Maybe::Is("windows") => Os::Windows,
            Maybe::Is("wasi") => Os::Wasi,
            _ => Os::Other,
        };
        let env = match env {
            Maybe::Is("gnu") => Env::Gnu,
            Maybe::Is("musl") => Env::Musl,
            Maybe::Is("msvc") => Env::Msvc,
            _ => Env::Other,
        };
        let arch = match arch {
            Maybe::Is("x86_64") => Arch::X86_64,
            Maybe::Is("aarch64") => Arch::Aarch64,
            Maybe::Is("riscv64") => Arch::Riscv64,
            Maybe::Is("wasm32") => Arch::Wasm32,
            Maybe::Is("wasm64") => Arch::Wasm64,
            _ => Arch::Other,
        };
        let isa = match features {
            Maybe::Is(features) => widest_isa(features),
            Maybe::Isnt => Isa::Iss64,
        };
        Target {
            os,
            env,
            arch,
            format: object_format(os, arch),
            isa,
        }
    }
}

/// The object format that follows from the system and the architecture.
/// A wasm architecture is wasm on any system.
const fn object_format(os: Os, arch: Arch) -> ObjectFormat {
    match (arch, os) {
        (Arch::Wasm32 | Arch::Wasm64, _) => ObjectFormat::Wasm,
        (_, Os::Linux | Os::Android | Os::FreeBsd | Os::NetBsd | Os::OpenBsd) => ObjectFormat::Elf,
        (_, Os::MacOs | Os::Ios) => ObjectFormat::MachO,
        (_, Os::Windows) => ObjectFormat::Coff,
        (_, Os::Wasi | Os::Other) => ObjectFormat::Other,
    }
}

/// The widest instruction set in a comma-separated feature list, richest
/// first: AVX-512, AVX2, SVE, NEON, then the baseline.
fn widest_isa(
    features: &str, // lint:allow(no-bare-string) reason: cargo env `CARGO_CFG_TARGET_FEATURE` value; tracked: #72
) -> Isa {
    let has = |needle: &str| features.split(',').any(|f| f.trim() == needle); // lint:allow(no-bare-string) reason: feature-list closure operand; tracked: #72

    if has("avx512f") || has("avx512") {
        Isa::Avx512
    } else if has("avx2") {
        Isa::Avx2
    } else if has("sve") {
        Isa::Sve
    } else if has("neon") {
        Isa::Neon
    } else {
        Isa::Iss64
    }
}
