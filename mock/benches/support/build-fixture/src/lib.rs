//! The generated fixture of the build and link benches.
//!
//! A deterministic cargo workspace of `12 * c + 1` crates: twelve domains in
//! three tiers, each a contract crate and `c - 1` implementation crates behind
//! it, and one facade binary that composes every implementation. The same seed
//! and point give the same tree to the byte; [`Fixture::digest`] names it, and
//! [`committed_digest`] holds the digest of each point as it was when the
//! benches were first run, so a generator that changed the workload is refused
//! rather than measured as though nothing had moved.
//!
//! The generated tree is never committed. It is written by [`Fixture::write`]
//! into a directory the caller owns, with every crate's edit slot at the number
//! the caller asks, which is how an edit scenario changes a function body.

pub mod graph;
pub mod manifest;
pub mod text;

use std::path::Path;

use graph::{Graph, Role};

/// The seed of every fixture the benches build.
pub const SEED: u64 = 0x6876_626C_6669_7831;

/// Domains in every point: four per tier.
pub const DOMAINS: usize = 12;

/// Generic items each contract offers across its edge.
pub const GENERICS: usize = 4;

/// Plain functions in each implementation crate, the code size per crate.
/// Calibrated once and never retuned: at 2000 the 25-crate point's `dev`
/// executable was 8680112 bytes for its 12 implementation crates, which puts
/// 156 implementation crates near 113 MB; 2230 aims the 169-crate point at
/// the 126 MB of the trial's `kaski-sim` test binary.
pub const FUNCTIONS: usize = 2230;

/// The points the benches run at, by crate count: small, mid and large.
pub const POINTS: [usize; 3] = [25, 97, 169];

/// The digest of each point's pristine tree, as committed with the benches.
pub const COMMITTED: [(usize, u64); 3] = [
    (25, 0x4266_67DB_7541_99ED),
    (97, 0x6053_CAB8_9E05_FB7A),
    (169, 0xC5BB_68A5_0A2C_7A46),
];

/// The committed digest of point `n`, if `n` is one of [`POINTS`].
pub fn committed_digest(n: usize) -> Option<u64> {
    COMMITTED.iter().find(|(p, _)| *p == n).map(|(_, d)| *d)
}

/// Where an edit scenario changes a function body.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Site {
    /// The first implementation of the first leaf domain.
    Leaf,
    /// The contract of the first mid domain.
    Mid,
    /// The contract of the first root domain.
    Root,
}

impl Site {
    pub const ALL: [Site; 3] = [Site::Leaf, Site::Mid, Site::Root];

    pub const fn label(self) -> &'static str {
        match self {
            Site::Leaf => "leaf",
            Site::Mid => "mid",
            Site::Root => "root",
        }
    }
}

/// One fixture: a point's graph, the generic and function counts, the seed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Fixture {
    pub graph:     Graph,
    pub generics:  usize,
    pub functions: usize,
    pub seed:      u64,
}

impl Fixture {
    /// The fixture of `n` crates, or `None` where `n` is not `12 * c + 1` with `c >= 2`.
    pub fn at(n: usize) -> Option<Fixture> {
        if n < 2 * DOMAINS + 1 || !(n - 1).is_multiple_of(DOMAINS) {
            return None;
        }
        Some(Fixture {
            graph:     Graph {
                domains:    DOMAINS,
                per_domain: (n - 1) / DOMAINS,
            },
            generics:  GENERICS,
            functions: FUNCTIONS,
            seed:      SEED,
        })
    }

    /// The crate an edit at `site` changes.
    pub const fn site_role(&self, site: Site) -> Role {
        let third = self.graph.domains / 3;
        match site {
            Site::Leaf => {
                Role::Implementation {
                    domain: 2 * third,
                    index:  1,
                }
            },
            Site::Mid => {
                Role::Contract {
                    domain: third,
                }
            },
            Site::Root => {
                Role::Contract {
                    domain: 0,
                }
            },
        }
    }

    /// How many crates cargo compiles again after an edit at `site`.
    pub fn reach(&self, site: Site) -> usize {
        self.graph.reach(self.site_role(site)).len()
    }

    /// The path of the edit slot of `role`, relative to the workspace root.
    pub fn edit_path(role: Role) -> String {
        format!("{}/src/edit.rs", graph::dir(role))
    }

    /// Every file of the pristine tree, by path relative to the workspace root, sorted.
    pub fn files(&self) -> Vec<(String, String)> {
        let g = &self.graph;
        let mut files = vec![
            ("Cargo.toml".to_string(), manifest::workspace()),
            ("Cargo.lock".to_string(), manifest::lockfile(g)),
        ];
        for role in g.roles() {
            let d = graph::dir(role);
            files.push((format!("{d}/Cargo.toml"), manifest::package(g, role)));
            files.push((Fixture::edit_path(role), text::edit_slot(0)));
            let (path, body) = match role {
                Role::Contract {
                    domain,
                } => {
                    (
                        "src/lib.rs",
                        text::contract(g, domain, self.generics, self.seed),
                    )
                },
                Role::Implementation {
                    domain,
                    index,
                } => {
                    (
                        "src/lib.rs",
                        text::implementation(
                            g,
                            domain,
                            index,
                            self.generics,
                            self.functions,
                            self.seed,
                        ),
                    )
                },
                Role::Facade => ("src/main.rs", text::facade(g, self.seed)),
            };
            files.push((format!("{d}/{path}"), body));
        }
        files.sort();
        files
    }

    /// The digest of the pristine tree: FNV-1a over every path and its text, in path order.
    pub fn digest(&self) -> u64 {
        digest_of(&self.files())
    }

    /// Write the tree into `root`, with the edit slot of `edited` holding
    /// `number` where one is given. A file already holding the right text is
    /// left alone, so its time does not move. Refuses a tree whose pristine
    /// digest is not the committed one of its point.
    pub fn write(&self, root: &Path, edited: Option<(Role, u64)>) -> Result<(), String> {
        let files = self.files();
        let n = self.graph.crate_count();
        let found = digest_of(&files);
        match committed_digest(n) {
            Some(want) if want == found => {},
            Some(want) => {
                return Err(format!(
                    "fixture n={n}: the generator writes digest {found:016x}, and {want:016x} is committed. \
                     The workload moved; a changed fixture is a new workload, so commit it as one"
                ));
            },
            None => {
                return Err(format!(
                    "fixture n={n}: no digest is committed for this point"
                ));
            },
        }
        for (path, body) in files {
            let body = match edited {
                Some((role, number)) if path == Fixture::edit_path(role) => text::edit_slot(number),
                _ => body,
            };
            put(&root.join(&path), &body)?;
        }
        Ok(())
    }
}

/// FNV-1a over each path, a zero byte, its text and a zero byte.
pub fn digest_of(files: &[(String, String)]) -> u64 {
    let mut h: u64 = 0xCBF2_9CE4_8422_2325;
    for (path, body) in files {
        for b in path.bytes().chain([0u8]).chain(body.bytes()).chain([0u8]) {
            h ^= u64::from(b);
            h = h.wrapping_mul(0x0000_0100_0000_01B3);
        }
    }
    h
}

/// Write `body` to `path` unless it already holds exactly that.
fn put(path: &Path, body: &str) -> Result<(), String> {
    if std::fs::read_to_string(path).is_ok_and(|have| have == body) {
        return Ok(());
    }
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
    }
    std::fs::write(path, body).map_err(|e| format!("{}: {e}", path.display()))
}
