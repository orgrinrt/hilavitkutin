//! The build scripts the crate offers, by name.
//!
//! A consumer's script names one in [`script!`](crate::script) and writes
//! nothing of what it does, so no consumer holds a copy that can drift.

use core::fmt::Write;

use notko::Outcome;

use crate::bootstrap::{Env, resolve, write_directives};
use crate::config::BuildConfig;
use crate::pragma::{Pragma, PragmaSet};

/// A build script the crate offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Prototype {
    /// The profile's default pragmas, as [`configure`](crate::configure)
    /// takes them.
    Default,
    /// The default pragmas without `FastMath`, for a crate whose own code
    /// must give the same floats on every machine.
    Deterministic,
}

impl Prototype {
    /// The profile's default pragmas as this prototype takes them.
    pub const fn tune(self, pragmas: PragmaSet) -> PragmaSet {
        match self {
            Prototype::Default => pragmas,
            Prototype::Deterministic => pragmas.without(Pragma::FastMath),
        }
    }

    /// Resolve the build from cargo's environment and write the directives.
    ///
    /// The resolved build comes back, so the script knows whether there
    /// are flags to hand over. No warning is written here: `script!` says
    /// the one thing there is to say after it has looked at the wiring.
    pub fn configure<S: AsRef<str>, W: Write>(
        self,
        mut env: impl Env<S>,
        out: &mut W,
    ) -> Outcome<BuildConfig, core::fmt::Error> {
        let cfg = resolve(&mut env, |pragmas| self.tune(pragmas));
        match write_directives(&cfg, out) {
            Ok(()) => Outcome::Ok(cfg),
            Err(refused) => Outcome::Err(refused),
        }
    }
}
