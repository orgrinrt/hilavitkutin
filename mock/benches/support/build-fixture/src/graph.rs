//! The crate graph of the fixture: domains in three tiers, a contract crate
//! and its implementation crates per domain, and one composer over them.
//!
//! Every domain offers a contract and implementations beside it. A contract
//! takes the contracts of the domains it reads, an implementation takes its
//! own contract and the contracts its domain reads, and only the composer, the
//! facade binary, takes implementations. The root tier is read by every other
//! domain, the mid tier by the leaf tier, and the leaf tier by nothing but the
//! composer.

/// The tier a domain sits in.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Root,
    Mid,
    Leaf,
}

/// What one crate of the fixture is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// The contract crate of a domain.
    Contract {
        domain: usize,
    },
    /// Implementation `index` (from 1) of a domain.
    Implementation {
        domain: usize,
        index:  usize,
    },
    /// The application facade, the one composer.
    Facade,
}

/// The shape of the graph: how many domains and how many crates each has.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Graph {
    /// Domains, a multiple of three: a third in each tier.
    pub domains:    usize,
    /// Crates per domain: one contract and `per_domain - 1` implementations.
    pub per_domain: usize,
}

impl Graph {
    /// The tier of domain `d`.
    pub const fn tier(&self, d: usize) -> Tier {
        let third = self.domains / 3;
        if d < third {
            Tier::Root
        } else if d < 2 * third {
            Tier::Mid
        } else {
            Tier::Leaf
        }
    }

    /// The domains domain `d` reads, in ascending order. A mid domain reads
    /// every root domain; a leaf domain reads every root domain and two mid
    /// domains, its own position's and the next one's.
    pub fn reads(&self, d: usize) -> Vec<usize> {
        let third = self.domains / 3;
        match self.tier(d) {
            Tier::Root => Vec::new(),
            Tier::Mid => (0 .. third).collect(),
            Tier::Leaf => {
                let pos = d - 2 * third;
                let mut r: Vec<usize> = (0 .. third).collect();
                let a = third + pos % third;
                let b = third + (pos + 1) % third;
                r.push(a.min(b));
                if a != b {
                    r.push(a.max(b));
                }
                r
            },
        }
    }

    /// Every crate, contracts and implementations domain by domain, then the facade.
    pub fn roles(&self) -> Vec<Role> {
        let mut roles = Vec::with_capacity(self.domains * self.per_domain + 1);
        for d in 0 .. self.domains {
            roles.push(Role::Contract {
                domain: d,
            });
            for index in 1 .. self.per_domain {
                roles.push(Role::Implementation {
                    domain: d,
                    index,
                });
            }
        }
        roles.push(Role::Facade);
        roles
    }

    /// The crate count: every domain's crates and the facade.
    pub const fn crate_count(&self) -> usize {
        self.domains * self.per_domain + 1
    }

    /// The crates `role` depends on, by role.
    pub fn deps(&self, role: Role) -> Vec<Role> {
        match role {
            Role::Contract {
                domain,
            } => {
                self.reads(domain)
                    .into_iter()
                    .map(|r| {
                        Role::Contract {
                            domain: r,
                        }
                    })
                    .collect()
            },
            Role::Implementation {
                domain,
                ..
            } => {
                let mut deps = vec![Role::Contract {
                    domain,
                }];
                deps.extend(self.reads(domain).into_iter().map(|r| {
                    Role::Contract {
                        domain: r,
                    }
                }));
                deps
            },
            Role::Facade => {
                self.roles()
                    .into_iter()
                    .filter(|r| matches!(r, Role::Implementation { .. }))
                    .collect()
            },
        }
    }

    /// The crates a change to `role` makes cargo compile again: itself and
    /// every crate that depends on it, directly or through others.
    pub fn reach(&self, role: Role) -> Vec<Role> {
        let roles = self.roles();
        let mut reached = vec![role];
        let mut grew = true;
        while grew {
            grew = false;
            for r in &roles {
                if reached.contains(r) {
                    continue;
                }
                if self.deps(*r).iter().any(|d| reached.contains(d)) {
                    reached.push(*r);
                    grew = true;
                }
            }
        }
        reached
    }
}

/// The package name of a crate.
pub fn name(role: Role) -> String {
    match role {
        Role::Contract {
            domain,
        } => format!("fx_d{domain:02}_contract"),
        Role::Implementation {
            domain,
            index,
        } => format!("fx_d{domain:02}_impl{index:02}"),
        Role::Facade => "fx_facade".to_string(),
    }
}

/// The directory of a crate in the fixture workspace.
pub fn dir(role: Role) -> String {
    match role {
        Role::Facade => "facade".to_string(),
        other => format!("crates/{}", name(other)),
    }
}
