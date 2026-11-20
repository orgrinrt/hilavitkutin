//! What one call of a build or link bench does.
//!
//! A call has three parts, the harness's `timed!` blocks: [`Call::ready`],
//! untimed, writes the fixture and puts the cell's build into the state its
//! scenario starts from; [`Call::run`], timed, is one cargo build or one run of
//! a recorded link line; [`Call::record`], untimed, reads what cargo and the
//! linker said, appends one line to the bench's record and answers with the
//! call's result. The scenario and the profile come in the input bytes the
//! routine of the cell builds ([`routine`]). How it builds is the arm's: a set
//! of hilavitkutin-build's pragmas and a quantity, both types ([`arm`]), so
//! every flag, key and mark of a call comes from that crate and nothing is
//! read from an arm's name.
//!
//! This file holds the vocabulary a call is described in: the scenario, the
//! profile, the quantity, the broken setups the controls use, and where a
//! cell's directories are. What a call does with them is [`call`].
//!
//! Every cell builds in a directory of its own under the build directory the
//! run inherits (`CARGO_TARGET_DIR`, or the system's temporary directory where
//! none is set): `build-link/<bench>/<arm>/n<point>/`, holding the fixture
//! tree `ws/`, the target directory `target/`, the temporary directory rustc
//! writes into, `tmp/`, and an empty cargo home, `home/`. A call at one point
//! removes its arm's directories of every other point, so one point per arm is
//! on the disk.

pub mod arm;
pub mod call;
pub mod cargo;
pub mod elf;
pub mod hooks;
pub mod linker;
pub mod linkline;
pub mod measured;
pub mod record;
pub mod routine;
pub mod space;
pub mod toolchain;

use std::path::PathBuf;

use build_fixture::Site;
pub use call::{Call, bench_call};

/// What a call does to the fixture.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scenario {
    /// Every crate built into an empty target directory.
    Clean,
    /// One leaf implementation crate edited, then the workspace built.
    Leaf,
    /// One mid contract crate edited, then the workspace built.
    Mid,
    /// One root contract crate edited, then the workspace built.
    Root,
    /// Every object fresh and the final executable removed, then linked again.
    Link,
}

impl Scenario {
    pub const ALL: [Scenario; 5] =
        [Scenario::Clean, Scenario::Leaf, Scenario::Mid, Scenario::Root, Scenario::Link];

    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn from_code(c: u8) -> Option<Scenario> {
        Scenario::ALL.get(usize::from(c)).copied()
    }

    pub const fn label(self) -> &'static str {
        match self {
            Scenario::Clean => "clean",
            Scenario::Leaf => "leaf",
            Scenario::Mid => "mid",
            Scenario::Root => "root",
            Scenario::Link => "link",
        }
    }

    /// The edit site, for an edit scenario.
    pub const fn site(self) -> Option<Site> {
        match self {
            Scenario::Leaf => Some(Site::Leaf),
            Scenario::Mid => Some(Site::Mid),
            Scenario::Root => Some(Site::Root),
            _ => None,
        }
    }

    /// What the untimed setup has to have done for the timed block to have work.
    pub const fn what_setup_undoes(self) -> &'static str {
        match self {
            Scenario::Clean => "the target directory was not empty",
            Scenario::Link => "the final executable was still there",
            _ => "the edit changed no content",
        }
    }
}

/// The fixture's own profile a call builds in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Profile {
    Dev,
    Release,
}

impl Profile {
    pub const fn code(self) -> u8 {
        self as u8
    }

    pub fn from_code(c: u8) -> Option<Profile> {
        match c {
            0 => Some(Profile::Dev),
            1 => Some(Profile::Release),
            _ => None,
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Profile::Dev => "dev",
            Profile::Release => "release",
        }
    }

    /// The same profile as hilavitkutin-build names it.
    pub const fn of_the_crate(self) -> hilavitkutin_build::Profile {
        match self {
            Profile::Dev => hilavitkutin_build::Profile::Dev,
            Profile::Release => hilavitkutin_build::Profile::Release,
        }
    }

    /// The directory cargo writes this profile's outputs to.
    pub const fn dir(self) -> &'static str {
        match self {
            Profile::Dev => "debug",
            Profile::Release => "release",
        }
    }
}

/// What an arm's timed block is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    /// A whole-workspace build by cargo.
    Build,
    /// Quantity A: the facade built again by cargo, every rlib fresh.
    Relink,
    /// Quantity B: the facade's recorded link line run again as a process.
    Replay,
}

impl Kind {
    pub const ALL: [Kind; 3] = [Kind::Build, Kind::Relink, Kind::Replay];

    /// The quantity as a record's line has it.
    pub const fn label(self) -> &'static str {
        match self {
            Kind::Build => "build",
            Kind::Relink => "relink",
            Kind::Replay => "replay",
        }
    }

    pub fn from_label(label: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|k| k.label() == label)
    }
}

/// The quantity an arm measures, as a type the arm names.
pub trait Quantity {
    const KIND: Kind;
}

/// A whole-workspace build by cargo.
#[derive(Clone, Copy, Debug)]
pub struct Build;

/// Quantity A: the facade built again by cargo, every rlib fresh.
#[derive(Clone, Copy, Debug)]
pub struct Relink;

/// Quantity B: the facade's recorded link line run again as a process.
#[derive(Clone, Copy, Debug)]
pub struct Replay;

impl Quantity for Build {
    const KIND: Kind = Kind::Build;
}

impl Quantity for Relink {
    const KIND: Kind = Kind::Relink;
}

impl Quantity for Replay {
    const KIND: Kind = Kind::Replay;
}

pub use arm::Arm;

/// A deliberately broken setup, which the checks must refuse. Only the
/// controls in this crate's tests use it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Control {
    /// A clean scenario that removes nothing.
    SkipClean,
    /// An edit that moves the slot's time and leaves its content.
    TimeOnly,
    /// A link that leaves the final executable in place.
    KeepOutput,
    /// A disk read as having no free space, which the setup must refuse.
    NoSpace,
}

/// Where a bench's state lives: `build-link/` under the inherited build directory.
pub fn root() -> PathBuf {
    std::env::var_os("CARGO_TARGET_DIR")
        .map_or_else(std::env::temp_dir, PathBuf::from)
        .join("build-link")
}

/// The directories of one cell: `<root>/<bench>/<arm>/n<point>/`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Dirs {
    dir: PathBuf,
}

impl Dirs {
    /// The cell of `arm` at `n` crates in `bench`.
    pub fn of(bench: &str, arm: &str, n: usize) -> Dirs {
        Dirs {
            dir: root().join(bench).join(arm).join(format!("n{n}")),
        }
    }

    /// The cell's own directory.
    pub fn dir(&self) -> PathBuf {
        self.dir.clone()
    }

    /// The fixture tree.
    pub fn ws(&self) -> PathBuf {
        self.dir.join("ws")
    }

    /// The target directory, which is also the build directory.
    pub fn target(&self) -> PathBuf {
        self.dir.join("target")
    }

    /// The temporary directory rustc writes into.
    pub fn tmp(&self) -> PathBuf {
        self.dir.join("tmp")
    }

    /// The cargo home, kept empty so no configuration of the machine's own
    /// cargo home reaches the build.
    pub fn home(&self) -> PathBuf {
        self.dir.join("home")
    }

    /// Where a replay keeps the link line rustc printed for `profile`. One
    /// file per profile, since both profiles of a point share the cell.
    pub fn link_line(&self, profile: Profile) -> PathBuf {
        self.dir.join(format!("link-line-{}.txt", profile.label()))
    }

    /// Every directory a call needs, made where missing.
    pub fn make(&self) -> Result<(), String> {
        for d in [self.ws(), self.tmp(), self.home()] {
            std::fs::create_dir_all(&d).map_err(|e| format!("{}: {e}", d.display()))?;
        }
        Ok(())
    }
}
