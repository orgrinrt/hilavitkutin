//! The tools a pragma waits on, and what a script does about each.
//!
//! A pragma that needs something the toolchain does not bring names a row
//! of [`TOOLS`] in its [`TOOL`](crate::Pragma::TOOL). The script looks for
//! the tool and asks [`provision`], which answers what to do. The looking,
//! the building and the removing stay the script's; this module holds the
//! rows, the decision and the text.

use core::fmt::{self, Display, Write};

use notko::Maybe;

use crate::target::Os;

/// Where a tool comes from.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Source {
    /// A package of a trusted package manager, by the package's name.
    Package(&'static str),
    /// A component of the toolchain, by the name rustup has for it.
    Component(&'static str),
    /// A build on this machine, by the crate cargo installs.
    LocalBuild(&'static str),
    /// No source yet. A pragma that waits on such a tool writes what it
    /// can without it and is not refused.
    Awaited,
}

/// What a tool's version line has to show.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Line {
    /// The line starts with these words, and a word ends where they end.
    Starts(&'static str),
    /// The line holds these words.
    Holds(&'static str),
    /// There is no line: the tool is a file in the toolchain's sysroot,
    /// and finding it there is the proof.
    Sysroot,
    /// Nothing proves the tool yet.
    Nothing,
}

/// How a script tells that what it found is the tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Proof {
    /// What the tool's version line has to show.
    pub line: Line,
    /// The mark a linker leaves in the `.comment` section of what it links.
    pub mark: Maybe<&'static str>,
}

impl Proof {
    /// Whether `line`, the first line a tool printed when asked for its
    /// version, is this tool's.
    #[rustfmt::skip] // keeps the allow on the signature it governs
    pub fn holds(&self, line: &str) -> bool { // lint:allow(no-bare-string) lint:allow(no-bare-numeric) lint:allow(arvo-types-only) reason: a version line as the build script read it; tracked: #72
        match self.line {
            Line::Starts(words) => match line.strip_prefix(words) {
                Some(rest) => words.ends_with(' ') || rest.chars().next().is_none_or(char::is_whitespace),
                None => false,
            },
            Line::Holds(words) => line.contains(words),
            Line::Sysroot => true,
            Line::Nothing => false,
        }
    }
}

/// One tool a pragma waits on.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Tool {
    /// The tool's name, which is also the program's.
    pub name:    &'static str,
    /// The version the crate pins, where it pins one.
    pub version: Maybe<&'static str>,
    /// Where the tool comes from.
    pub source:  Source,
    /// How a script tells it has the tool.
    pub proof:   Proof,
}

/// A tool with no source yet.
const fn awaited(name: &'static str) -> Tool {
    Tool {
        name,
        version: Maybe::Isnt,
        source: Source::Awaited,
        proof: Proof {
            line: Line::Nothing,
            mark: Maybe::Isnt,
        },
    }
}

/// The mold linker, from the machine's packages.
pub const MOLD: Tool = Tool {
    name:    "mold",
    version: Maybe::Isnt,
    source:  Source::Package("mold"),
    proof:   Proof {
        line: Line::Starts("mold "),
        mark: Maybe::Is("mold "),
    },
};

/// The wild linker, built on the machine at its pin.
pub const WILD: Tool = Tool {
    name:    "wild",
    version: Maybe::Is("0.10.0"),
    source:  Source::LocalBuild("wild-linker"),
    proof:   Proof {
        line: Line::Starts("Wild 0.10.0"),
        mark: Maybe::Is("Linker: Wild"),
    },
};

/// clang, as a link driver and as a C compiler.
pub const CLANG: Tool = Tool {
    name:    "clang",
    version: Maybe::Isnt,
    source:  Source::Package("clang"),
    proof:   Proof {
        line: Line::Holds("clang version"),
        mark: Maybe::Isnt,
    },
};

/// gcc, as a link driver and as a C compiler.
pub const GCC: Tool = Tool {
    name:    "gcc",
    version: Maybe::Isnt,
    source:  Source::Package("gcc"),
    proof:   Proof {
        line: Line::Starts("gcc "),
        mark: Maybe::Isnt,
    },
};

/// The cranelift codegen backend of the pinned toolchain.
pub const CRANELIFT: Tool = Tool {
    name:    "cranelift",
    version: Maybe::Isnt,
    source:  Source::Component("rustc-codegen-cranelift-preview"),
    proof:   Proof {
        line: Line::Sysroot,
        mark: Maybe::Isnt,
    },
};

/// The loop pass plugin `LoopOptimization` and `LoopFusion` wait on.
pub const POLKA_PASSES: Tool = awaited("polka-passes");
/// The float peephole plugin `MathPeephole` waits on.
pub const MATH_PEEPHOLE: Tool = awaited("math-peephole");
/// The Polly-enabled LLVM `Polly` waits on.
pub const POLLY: Tool = awaited("polly");
/// The profile merge tool `Pgo` and `Profiling` wait on.
pub const PROFDATA: Tool = awaited("llvm-profdata");
/// The post-link rewriter `Bolt` waits on.
pub const LLVM_BOLT: Tool = awaited("llvm-bolt");
/// The allocator crate `MimallocAllocator` recommends.
pub const MIMALLOC: Tool = awaited("mimalloc");

/// Every tool a pragma of the crate waits on.
pub const TOOLS: &[Tool] = &[
    MOLD,
    WILD,
    CLANG,
    GCC,
    CRANELIFT,
    POLKA_PASSES,
    MATH_PEEPHOLE,
    POLLY,
    PROFDATA,
    LLVM_BOLT,
    MIMALLOC,
];

/// Where locally built tools live: a directory and what is under it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Home<'a> {
    base:  &'a str, // lint:allow(no-bare-string) reason: a path as the build script read it; tracked: #72
    under: &'static str,
}

impl Display for Home<'_> {
    fn fmt(&self, out: &mut fmt::Formatter<'_>) -> core::fmt::Result {
        out.write_str(self.base)?;
        out.write_str(self.under)
    }
}

/// A value the script read, where it holds anything.
#[rustfmt::skip] // keeps the allow on the signature it governs
fn given(value: Maybe<&str>) -> Maybe<&str> { // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    match value {
        Maybe::Is(text) if !text.is_empty() => Maybe::Is(text),
        _ => Maybe::Isnt,
    }
}

/// Decide the home of locally built tools.
///
/// `named` is the value of `HILAVITKUTIN_BUILD_TOOLS_HOME`, which the
/// builder sets to choose the place. Without it the home is the per-user
/// cache: on macOS `Library/Caches` under `home`, elsewhere `xdg`, the
/// value of `XDG_CACHE_HOME`, or `.cache` under `home`. A variable that is
/// set and empty counts as unset. Where nothing is given there is no home.
pub fn tools_home<'a>(
    named: Maybe<&'a str>, // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    xdg: Maybe<&'a str>, // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    home: Maybe<&'a str>, // lint:allow(no-bare-string) reason: env value as the build script read it; tracked: #72
    host: Os,
) -> Maybe<Home<'a>> {
    if let Maybe::Is(base) = given(named) {
        return Maybe::Is(Home {
            base,
            under: "",
        });
    }
    let mac = matches!(host, Os::MacOs);
    if let (false, Maybe::Is(base)) = (mac, given(xdg)) {
        return Maybe::Is(Home {
            base,
            under: "/hilavitkutin-build/tools",
        });
    }
    match given(home) {
        Maybe::Is(base) => {
            Maybe::Is(Home {
                base,
                under: if mac {
                    "/Library/Caches/hilavitkutin-build/tools"
                } else {
                    "/.cache/hilavitkutin-build/tools"
                },
            })
        },
        Maybe::Isnt => Maybe::Isnt,
    }
}

/// Write the directory of `tool` under a tools home:
/// `<home>/<host triple>/<name>-<version>`, or `<name>` alone for a tool
/// with no pin.
///
/// The host triple is in the path so one home serves machines of several
/// kinds, and the version so a pin that moves gets a directory of its own.
pub fn write_tool_dir<W: Write, H: Display>(
    home: H,
    host: &str, // lint:allow(no-bare-string) reason: the host triple as the build script read it; tracked: #72
    tool: &Tool,
    out: &mut W,
) -> core::fmt::Result {
    write!(out, "{home}/{host}/{}", tool.name)?;
    match tool.version {
        Maybe::Is(version) => write!(out, "-{version}"),
        Maybe::Isnt => Ok(()),
    }
}

/// What a script does about one tool.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Provision {
    /// Use the one on the path.
    Path,
    /// Use the one in the tools home.
    Home,
    /// Build it into the tools home, then use that one.
    Build,
    /// Refuse the build, with the command that brings the tool.
    Refuse,
    /// Go on without it: the tool has no source yet.
    Awaited,
}

/// Decide what a script does about `tool`.
///
/// `on_path` is the first line the program of that name on the path
/// printed when asked for its version, and `in_home` the same of the one
/// in the tool's directory under the tools home, each `Maybe::Isnt` where
/// there is no such program. A tool proved by the sysroot has no line, and
/// the script hands an empty one where it found the file. The one on the
/// path comes first, since a machine set up by hand is the builder's
/// choice. Only a local build is built: a package and a component are a
/// machine's setup, which a build script does not do.
pub fn provision(
    tool: &Tool,
    on_path: Maybe<&str>, // lint:allow(no-bare-string) reason: a version line as the build script read it; tracked: #72
    in_home: Maybe<&str>, // lint:allow(no-bare-string) reason: a version line as the build script read it; tracked: #72
) -> Provision {
    if matches!(tool.source, Source::Awaited) {
        return Provision::Awaited;
    }
    let proved = |line: Maybe<&str>| matches!(line, Maybe::Is(line) if tool.proof.holds(line)); // lint:allow(no-bare-string) reason: a version line as the build script read it; tracked: #72
    if proved(on_path) {
        return Provision::Path;
    }
    match tool.source {
        Source::LocalBuild(_) if proved(in_home) => Provision::Home,
        Source::LocalBuild(_) => Provision::Build,
        Source::Package(_) | Source::Component(_) | Source::Awaited => Provision::Refuse,
    }
}

/// Write the `cargo::error` line of a tool the machine lacks and the
/// script does not build, with the command that brings it. Nothing is
/// written for a local build or a tool with no source.
pub fn write_refusal<W: Write>(tool: &Tool, out: &mut W) -> core::fmt::Result {
    let name = tool.name;
    match tool.source {
        Source::Package(package) => {
            writeln!(
                out,
                "cargo::error=hilavitkutin-build: a pragma of this build needs {name}, which this machine \
                 lacks: install the package `{package}` (`apt-get install {package}` on Debian and Ubuntu)"
            )
        },
        Source::Component(component) => {
            writeln!(
                out,
                "cargo::error=hilavitkutin-build: a pragma of this build needs {name}, which this toolchain \
                 lacks: run `rustup component add {component}`"
            )
        },
        Source::LocalBuild(_) | Source::Awaited => Ok(()),
    }
}
