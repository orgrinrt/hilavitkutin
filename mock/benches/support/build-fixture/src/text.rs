//! The Rust text of each fixture crate, generated from the seed.
//!
//! The generic items a contract offers follow the body shapes of a crate edge:
//! a trait with a defaulted method, a generic function, a generic type calling
//! it, and a call through a trait object. Downstream crates instantiate them
//! with their own types, so the monomorphised code lives in the crate that
//! names the type. An implementation crate carries `functions` plain functions
//! whose bodies come from the seed, each kept out of line and each called once
//! from the crate's `run`, so nothing folds away and the facade reaches all of
//! it.

use crate::graph::{Graph, Role, name};

/// splitmix64, the generator every constant of the text comes from.
pub struct Seeds(u64);

impl Seeds {
    pub const fn new(seed: u64) -> Self {
        Seeds(seed)
    }

    pub fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9E37_79B9_7F4A_7C15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        z ^ (z >> 31)
    }

    /// A rotation amount from 1 to 63.
    fn rot(&mut self) -> u64 {
        self.next() % 63 + 1
    }
}

/// The seed of one crate's text: the fixture seed mixed with the crate's place.
pub fn crate_seed(seed: u64, role: Role) -> u64 {
    let place = match role {
        Role::Contract {
            domain,
        } => (domain as u64) << 8,
        Role::Implementation {
            domain,
            index,
        } => ((domain as u64) << 8) | index as u64,
        Role::Facade => 0xFFFF,
    };
    let mut s = Seeds::new(seed ^ place.wrapping_mul(0x2545_F491_4F6C_DD1D));
    s.next()
}

/// The edit slot every crate carries: one function whose body holds `number`.
/// The pristine fixture has 0 there; an edit writes another number.
pub fn edit_slot(number: u64) -> String {
    format!(
        "//! The edit slot: the build benches change the number in this body.\n\n\
         pub(crate) fn mark(x: u64) -> u64 {{\n    x.rotate_left(7) ^ 0x{number:016x}\n}}\n"
    )
}

/// The contract crate of domain `d`.
pub fn contract(graph: &Graph, d: usize, generics: usize, seed: u64) -> String {
    let mut s = Seeds::new(crate_seed(seed, Role::Contract {
        domain: d,
    }));
    let mut t = String::new();
    t.push_str(&format!(
        "//! Fixture contract of domain {d}.\n\nmod edit;\n\n"
    ));
    t.push_str(&format!(
        "/// What an implementation of domain {d} does.\npub trait Op{d} {{\n    fn op(&self, x: u64) -> u64;\n\n    \
         fn twice(&self, x: u64) -> u64 {{\n        self.op(self.op(x)) ^ edit::mark(x)\n    }}\n}}\n\n"
    ));
    for j in 0 .. generics {
        let (c1, r1, c2, c3) = (s.next(), s.rot(), s.next(), s.next());
        t.push_str(&format!(
            "pub fn apply_{j}<T: Op{d}>(t: &T, x: u64) -> u64 {{\n    let mut a = x ^ 0x{c1:016x};\n    \
             let mut i = 0u64;\n    while i < (x & 3) + 1 {{\n        a = t.op(a).rotate_left({r1}) ^ i;\n        \
             i += 1;\n    }}\n    a\n}}\n\n\
             pub struct Wrap{j}<T>(pub T);\n\n\
             impl<T: Op{d}> Wrap{j}<T> {{\n    pub fn run(&self, x: u64) -> u64 {{\n        \
             apply_{j}(&self.0, x).wrapping_mul(0x{c2:016x} | 1)\n    }}\n}}\n\n\
             pub fn dyn_{j}(t: &dyn Op{d}, x: u64) -> u64 {{\n    t.twice(x ^ 0x{c3:016x})\n}}\n\n"
        ));
    }
    t.push_str("/// The contract's own non-generic code, which every edit of it changes.\n");
    let reads = graph.reads(d);
    if reads.is_empty() {
        t.push_str("pub fn edit_mark(x: u64) -> u64 {\n    edit::mark(x)\n}\n");
        return t;
    }
    t.push_str("pub fn edit_mark(x: u64) -> u64 {\n    let mut a = edit::mark(x);\n");
    for r in reads {
        let c = s.next();
        t.push_str(&format!(
            "    a ^= {}::edit_mark(a ^ 0x{c:016x});\n",
            name(Role::Contract {
                domain: r,
            })
        ));
    }
    t.push_str("    a\n}\n");
    t
}

/// One plain function of an implementation crate.
fn function(s: &mut Seeds, m: usize) -> String {
    let (c0, r0, c1, c2, c3, r1) = (s.next(), s.rot(), s.next(), s.next(), s.next(), s.rot());
    let arm = match s.next() % 3 {
        0 => format!("a.wrapping_mul(0x{c2:016x} | 1)"),
        1 => format!("a ^ (a >> {r1})"),
        _ => format!("a.wrapping_sub(0x{c2:016x})"),
    };
    format!(
        "#[inline(never)]\nfn f_{m}(x: u64) -> u64 {{\n    let mut a = x ^ 0x{c0:016x};\n    let mut i = 0u64;\n    \
         while i < (x & 7) + 1 {{\n        a = a.rotate_left({r0}).wrapping_add(0x{c1:016x}) ^ i;\n        i += 1;\n    }}\n    \
         match a % 4 {{\n        0 => {arm},\n        1 => a ^ 0x{c3:016x},\n        2 => a.rotate_right({r1}),\n        \
         _ => !a,\n    }}\n}}\n\n"
    )
}

/// Implementation `index` of domain `d`.
pub fn implementation(
    graph: &Graph,
    d: usize,
    index: usize,
    generics: usize,
    functions: usize,
    seed: u64,
) -> String {
    let mut s = Seeds::new(crate_seed(seed, Role::Implementation {
        domain: d,
        index,
    }));
    let own = name(Role::Contract {
        domain: d,
    });
    let reads = graph.reads(d);
    let mut t = String::new();
    t.push_str(&format!(
        "//! Fixture implementation {index} of domain {d}.\n\nmod edit;\n\n"
    ));
    let c = s.next();
    t.push_str(&format!(
        "pub struct Impl{index} {{\n    pub seed: u64,\n}}\n\n\
         impl {own}::Op{d} for Impl{index} {{\n    fn op(&self, x: u64) -> u64 {{\n        \
         (x ^ self.seed).wrapping_mul(0x{c:016x} | 1).rotate_left({})\n    }}\n}}\n\n",
        s.rot()
    ));
    for r in &reads {
        let c = s.next();
        t.push_str(&format!(
            "struct Local{r}(u64);\n\nimpl {}::Op{r} for Local{r} {{\n    fn op(&self, x: u64) -> u64 {{\n        \
             x.wrapping_mul(self.0 | 1) ^ 0x{c:016x}\n    }}\n}}\n\n",
            name(Role::Contract { domain: *r })
        ));
    }
    for m in 0 .. functions {
        t.push_str(&function(&mut s, m));
    }
    t.push_str(
        "/// Everything this crate does, folded into one value.\npub fn run(x: u64) -> u64 {\n",
    );
    t.push_str("    let mut a = x ^ edit::mark(x);\n");
    for m in 0 .. functions {
        t.push_str(&format!("    a = f_{m}(a);\n"));
    }
    for r in &reads {
        let other = name(Role::Contract {
            domain: *r,
        });
        for j in 0 .. generics {
            t.push_str(&format!("    a ^= {other}::Wrap{j}(Local{r}(a)).run(a);\n"));
            t.push_str(&format!("    a ^= {other}::dyn_{j}(&Local{r}(a), a);\n"));
        }
    }
    for j in 0 .. generics {
        t.push_str(&format!(
            "    a ^= {own}::Wrap{j}(Impl{index} {{ seed: a }}).run(a);\n"
        ));
    }
    t.push_str(&format!("    a ^ {own}::edit_mark(a)\n}}\n"));
    t
}

/// The facade: every implementation's `run` folded into the check line.
pub fn facade(graph: &Graph, seed: u64) -> String {
    let mut s = Seeds::new(crate_seed(seed, Role::Facade));
    let mut t = String::new();
    t.push_str(
        "//! The fixture's application facade, the one composer.\n\nmod edit;\n\nfn main() {\n",
    );
    t.push_str(&format!(
        "    let mut a = edit::mark(0x{:016x});\n",
        s.next()
    ));
    for role in graph.roles() {
        if let Role::Implementation {
            ..
        } = role
        {
            t.push_str(&format!("    a ^= {}::run(a);\n", name(role)));
        }
    }
    t.push_str("    println!(\"fixture check {a:016x}\");\n}\n");
    t
}
