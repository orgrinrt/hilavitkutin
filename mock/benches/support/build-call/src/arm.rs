//! An arm: a set of hilavitkutin-build's pragmas and a quantity, both types.
//!
//! An arm names its set and its quantity once, where it calls
//! [`bench_call`](crate::bench_call). Everything a call builds with is read
//! from the set through the crate: the lines of cargo configuration, the rustc
//! flags, whether the build is incremental, and the linker whose mark the
//! output must carry. The arm's name is a directory and a label of the record
//! and says nothing a call reads.

use hilavitkutin_build::{
    Group,
    Incremental,
    IncrementalGroup,
    Linker,
    Pragma,
    PragmaSet,
    Set,
    Site,
    Source,
    Target,
    Visit,
    checked,
    each_tool,
    linker_of,
    write_rustflags,
    write_set_config,
};

use crate::{Kind, Profile, Quantity};

/// What an arm's set writes for one profile and one target.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Written {
    /// Lines of cargo configuration, each a dotted key and its value, which
    /// is what cargo's `--config` takes.
    pub config:    Vec<String>,
    /// The rustc flags, one to an element.
    pub rustflags: Vec<String>,
}

/// How an arm builds and what it measures.
#[derive(Clone, Debug)]
pub struct Arm {
    kind:        Kind,
    linker:      Linker,
    /// `None` where the set leaves incremental compilation to cargo.
    incremental: Option<bool>,
    /// The tools the set needs that have a source, by name.
    tools:       Vec<&'static str>,
    written:     fn(Profile, &str) -> Written,
    unadmitted:  fn(&Target) -> Vec<String>,
}

impl Arm {
    /// The arm of the set `S` measuring the quantity `Q`. A set with
    /// something wrong in it, two linkers or a requirement unmet, does not
    /// compile here.
    pub fn of<S: Set, Q: Quantity>() -> Arm {
        const { checked::<S>() };
        let mut tools = Vec::new();
        each_tool::<S>(|tool| {
            if !matches!(tool.source, Source::Awaited) {
                tools.push(tool.name);
            }
        });
        let holds_one = S::GROUPS & <IncrementalGroup as Group>::BIT != 0;
        Arm {
            kind: Q::KIND,
            linker: linker_of::<S>(),
            incremental: holds_one.then(|| PragmaSet::of::<S>().contains::<Incremental>()),
            tools,
            written: written::<S>,
            unadmitted: unadmitted::<S>,
        }
    }

    /// What the arm's timed block is.
    pub const fn kind(&self) -> Kind {
        self.kind
    }

    /// The linker of what the arm links.
    pub const fn linker(&self) -> Linker {
        self.linker
    }

    /// Whether the arm's cargo calls build with incremental compilation on.
    pub fn incremental(&self) -> bool {
        self.incremental == Some(true)
    }

    /// What the arm's set writes for `profile` on the target `triple`.
    pub fn written(&self, profile: Profile, triple: &str) -> Written {
        (self.written)(profile, triple)
    }

    /// Why no call of this arm can run on this host, or `None` where one can.
    ///
    /// A set that holds neither value of incremental is refused: cargo's own
    /// choice differs by profile, so the cell would not say what it measured.
    /// So is a set with a pragma the host target does not admit.
    pub fn refusal(&self) -> Option<String> {
        if self.incremental.is_none() {
            return Some(
                "the arm's set holds neither Incremental nor NoIncremental, and cargo's own choice differs by \
                 profile"
                    .into(),
            );
        }
        let no = (self.unadmitted)(&host());
        if !no.is_empty() {
            return Some(no.join("; "));
        }
        // FIXME(a_bench_arm_whose_set_needs_a_tool_finds_it_as_a_script_does):
        // a set that needs a tool is refused whole. Finding one is inside the
        // expansion of hilavitkutin-build's `script!` and no caller but a
        // build script reaches it, so an arm naming mold, wild, a link driver
        // or cranelift waits on that ticket.
        if !self.tools.is_empty() {
            return Some(format!(
                "the arm's set needs {}, and this bench does not find a tool yet",
                self.tools.join(", ")
            ));
        }
        None
    }
}

/// The host as hilavitkutin-build reads a target. The fixture is built for
/// the host, which this crate is built for too.
fn host() -> Target {
    Target::from_cargo(
        Some(std::env::consts::OS).into(),
        None.into(),
        Some(std::env::consts::ARCH).into(),
        None.into(),
    )
}

fn written<S: Set>(profile: Profile, triple: &str) -> Written {
    let site = Site::new(profile.of_the_crate(), triple, &[]);
    let mut config = String::new();
    write_set_config::<S, _>(&site, &mut config).expect("a String takes every write");
    let mut flags = String::new();
    write_rustflags::<S, _>(&site, &mut flags).expect("a String takes every write");
    Written {
        config:    config.lines().map(str::to_owned).collect(),
        rustflags: flags.split_whitespace().map(str::to_owned).collect(),
    }
}

/// The walk that keeps what each member the target does not admit says.
struct Unadmitted<'a> {
    target: &'a Target,
    said:   Vec<String>,
}

impl Visit for Unadmitted<'_> {
    fn pragma<P: Pragma>(&mut self) -> std::fmt::Result {
        if let Some(why) = Option::<&'static str>::from(P::admitted(self.target)) {
            self.said.push(format!("{} {why}", P::NAME));
        }
        Ok(())
    }
}

fn unadmitted<S: Set>(target: &Target) -> Vec<String> {
    let mut walk = Unadmitted {
        target,
        said: Vec::new(),
    };
    S::each(&mut walk).expect("the walk writes nothing");
    walk.said
}
