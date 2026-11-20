//! A model of loru's site shape and of a change mark carried along its two
//! cross-set reads, written to check shapes and corners, not the engine.
//!
//! Three record sets: strings (the text hash of each key), nodes (a key and a
//! kind per node), pages (twenty contiguous nodes each). Two derived stores:
//! the node hash, which gathers its string through the key column, and the
//! page hash, which folds the hashes of its twenty nodes in sequence.
//!
//! The model runs the two derivations incrementally under a configuration:
//! the grain of the mark (the whole store, a fixed window of records, or one
//! record), the direction it is carried in (pushed forward through the reverse
//! relation of each read, or pulled by each reader granule through the forward
//! relation and a generation compare), and the cutoff (a recomputed granule
//! counts as changed always, or only where a value differs). A demand frame
//! recomputes only what a chosen set of pages needs.
//!
//! Spike code: Vec, bools, no capacity discipline. Read it for what it checks.

pub const NODES_PER_PAGE: usize = 20;
const SHARED_KEYS: usize = 40;
const GLOBAL_SLOTS: usize = 3;
const SHARED_SLOTS: usize = 6;

pub struct Rng(u64);

impl Rng {
    pub fn new(seed: u64) -> Self {
        Rng(seed.wrapping_mul(0x9E37_79B9_7F4A_7C15) | 1)
    }

    pub fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }

    pub fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
}

pub fn mix(a: u64, b: u64) -> u64 {
    let mut x = a ^ b.rotate_left(29) ^ 0x9E37_79B9_7F4A_7C15;
    x = (x ^ (x >> 31)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94D0_49BB_1331_11EB);
    x ^ (x >> 31)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Inputs {
    pub text: Vec<u64>,
    pub key:  Vec<u32>,
    pub kind: Vec<u8>,
}

impl Inputs {
    pub fn pages(&self) -> usize {
        self.key.len() / NODES_PER_PAGE
    }
}

/// loru-bench-data's layout: slots 0..3 name a key every page carries, slots
/// 3..6 rotate through the shared pool, slots 6..20 name a key of their own.
pub fn site(pages: usize, rng: &mut Rng) -> Inputs {
    let mut text: Vec<u64> = (0 .. SHARED_KEYS).map(|_| rng.next()).collect();
    let mut key = Vec::new();
    let mut kind = Vec::new();
    for p in 0 .. pages {
        for slot in 0 .. NODES_PER_PAGE {
            let k = if slot < GLOBAL_SLOTS {
                slot
            } else if slot < SHARED_SLOTS {
                GLOBAL_SLOTS + (p * 3 + slot) % (SHARED_KEYS - GLOBAL_SLOTS)
            } else {
                text.push(rng.next());
                text.len() - 1
            };
            key.push(k as u32);
            kind.push(rng.below(4) as u8);
        }
    }
    Inputs {
        text,
        key,
        kind,
    }
}

/// The key a page-unique slot of page `p` names.
pub fn unique_key(inp: &Inputs, page: usize, slot: usize) -> usize {
    assert!(slot >= SHARED_SLOTS);
    inp.key[page * NODES_PER_PAGE + slot] as usize
}

pub fn node_hash(kind: u8, text: u64) -> u64 {
    mix(kind as u64 + 1, text)
}

pub fn page_hash(page: usize, nodes: &[u64]) -> u64 {
    nodes.iter().fold(mix(page as u64, 0xA5), |a, h| mix(a, *h))
}

/// The from-scratch answer every incremental run is held to.
pub fn scratch(inp: &Inputs) -> (Vec<u64>, Vec<u64>) {
    let nodes: Vec<u64> = (0 .. inp.key.len())
        .map(|n| node_hash(inp.kind[n], inp.text[inp.key[n] as usize]))
        .collect();
    let pages = (0 .. inp.pages())
        .map(|p| page_hash(p, &nodes[p * NODES_PER_PAGE .. (p + 1) * NODES_PER_PAGE]))
        .collect();
    (nodes, pages)
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Grain {
    Store,
    Morsel(usize),
    Record,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Carry {
    Push,
    Pull,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cutoff {
    Structural,
    Differs,
}

/// Deliberate defects the differential must catch.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Mutant {
    /// A rekey does not update the reverse relation of the gather.
    pub stale_inverse:     bool,
    /// A demand frame forgets the marks of what it did not run.
    pub demand_clears_all: bool,
    /// The reverse of the fold misses the last page a node granule touches.
    pub fold_edge:         bool,
    /// A recomputed node granule never counts as changed.
    pub cutoff_drops:      bool,
}

#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub grain:  Grain,
    pub carry:  Carry,
    pub cutoff: Cutoff,
    pub mutant: Mutant,
}

impl Config {
    pub fn new(grain: Grain, carry: Carry, cutoff: Cutoff) -> Self {
        Config {
            grain,
            carry,
            cutoff,
            mutant: Mutant::default(),
        }
    }
}

/// Work done, in records and relation entries. Counts, not time.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Work {
    pub node_records:   usize,
    pub page_records:   usize,
    /// Entries of a relation read to find what to rerun: key records read by a
    /// pull, inverse links or bitmatrix cells walked by a push.
    pub relation_reads: usize,
    /// Entries of the reverse relation written to keep it current.
    pub inverse_writes: usize,
}

#[derive(Clone, Copy)]
struct Tiling {
    w: usize,
    n: usize,
}

impl Tiling {
    fn new(grain: Grain, n: usize) -> Self {
        let w = match grain {
            Grain::Store => n.max(1),
            Grain::Morsel(w) => w.max(1),
            Grain::Record => 1,
        };
        Tiling {
            w,
            n,
        }
    }

    fn count(self) -> usize {
        self.n.div_ceil(self.w)
    }

    fn of(self, r: usize) -> usize {
        r / self.w
    }

    fn range(self, g: usize) -> (usize, usize) {
        (g * self.w, ((g + 1) * self.w).min(self.n))
    }
}

const NIL: u32 = u32::MAX;

pub struct Engine {
    cfg:       Config,
    pub inp:   Inputs,
    pub node:  Vec<u64>,
    pub page:  Vec<u64>,
    tt:        Tiling,
    tn:        Tiling,
    tp:        Tiling,
    clock:     u64,
    gen_text:  Vec<u64>,
    gen_key:   Vec<u64>,
    gen_kind:  Vec<u64>,
    gen_node:  Vec<u64>,
    last_node: Vec<u64>,
    last_page: Vec<u64>,
    pend_node: Vec<bool>,
    pend_page: Vec<bool>,
    // Record grain: the inverse of the key column as intrusive lists, a head
    // per string and a link pair per node. Three columns, fixed at build.
    head:      Vec<u32>,
    next:      Vec<u32>,
    prev:      Vec<u32>,
    // Coarser grains: string granule by node granule, one bit per pair.
    rev:       Vec<Vec<bool>>,
    pub work:  Work,
}

impl Engine {
    /// Build: every granule counts as changed since build, so the first frame,
    /// demand or full, runs what it needs.
    pub fn new(cfg: Config, inp: Inputs) -> Self {
        let tt = Tiling::new(cfg.grain, inp.text.len());
        let tn = Tiling::new(cfg.grain, inp.key.len());
        let tp = Tiling::new(cfg.grain, inp.pages());
        let mut e = Engine {
            cfg,
            node: vec![0; inp.key.len()],
            page: vec![0; inp.pages()],
            tt,
            tn,
            tp,
            clock: 1,
            gen_text: vec![1; tt.count()],
            gen_key: vec![1; tn.count()],
            gen_kind: vec![1; tn.count()],
            gen_node: vec![1; tn.count()],
            last_node: vec![0; tn.count()],
            last_page: vec![0; tp.count()],
            pend_node: vec![true; tn.count()],
            pend_page: vec![true; tp.count()],
            head: vec![NIL; inp.text.len()],
            next: vec![NIL; inp.key.len()],
            prev: vec![NIL; inp.key.len()],
            rev: vec![vec![false; tn.count()]; tt.count()],
            work: Work::default(),
            inp,
        };
        for n in 0 .. e.inp.key.len() {
            let k = e.inp.key[n] as usize;
            e.link(n, k);
            let (tg, ng) = (e.tt.of(k), e.tn.of(n));
            e.rev[tg][ng] = true;
        }
        e.work = Work::default();
        e
    }

    fn link(&mut self, n: usize, k: usize) {
        self.next[n] = self.head[k];
        self.prev[n] = NIL;
        if self.head[k] != NIL {
            self.prev[self.head[k] as usize] = n as u32;
        }
        self.head[k] = n as u32;
        self.work.inverse_writes += 1;
    }

    fn unlink(&mut self, n: usize, k: usize) {
        let (p, x) = (self.prev[n], self.next[n]);
        if p == NIL {
            self.head[k] = x;
        } else {
            self.next[p as usize] = x;
        }
        if x != NIL {
            self.prev[x as usize] = p;
        }
        self.work.inverse_writes += 1;
    }

    /// Push: the reader granules a changed string granule reaches.
    fn push_from_text(&mut self, k: usize) {
        if self.cfg.carry != Carry::Push {
            return;
        }
        if self.cfg.grain == Grain::Record {
            let mut n = self.head[k];
            while n != NIL {
                self.work.relation_reads += 1;
                self.pend_node[n as usize] = true;
                n = self.next[n as usize];
            }
        } else {
            let tg = self.tt.of(k);
            for ng in 0 .. self.tn.count() {
                self.work.relation_reads += 1;
                if self.rev[tg][ng] {
                    self.pend_node[ng] = true;
                }
            }
        }
    }

    /// The host writes a string. It says it wrote, whatever the value.
    pub fn set_text(&mut self, k: usize, v: u64) {
        self.inp.text[k] = v;
        self.clock += 1;
        let tg = self.tt.of(k);
        self.gen_text[tg] = self.clock;
        self.push_from_text(k);
    }

    pub fn set_kind(&mut self, n: usize, v: u8) {
        self.inp.kind[n] = v;
        self.clock += 1;
        let ng = self.tn.of(n);
        self.gen_kind[ng] = self.clock;
        self.pend_node[ng] = true;
    }

    /// The host points a node at another string: the gather's index changes.
    pub fn rekey(&mut self, n: usize, k: usize) {
        let old = self.inp.key[n] as usize;
        self.inp.key[n] = k as u32;
        self.clock += 1;
        let ng = self.tn.of(n);
        self.gen_key[ng] = self.clock;
        self.pend_node[ng] = true;
        if self.cfg.mutant.stale_inverse {
            return;
        }
        if self.cfg.grain == Grain::Record {
            self.unlink(n, old);
            self.link(n, k);
        } else {
            // Recompute the node granule's column of the bitmatrix from its keys.
            for tg in 0 .. self.tt.count() {
                self.rev[tg][ng] = false;
            }
            let (a, b) = self.tn.range(ng);
            for m in a .. b {
                let tg = self.tt.of(self.inp.key[m] as usize);
                self.rev[tg][ng] = true;
                self.work.inverse_writes += 1;
            }
        }
    }

    /// Page granules a node granule's records fold into (reverse of the fold).
    fn fold_reverse(&self, ng: usize) -> (usize, usize) {
        let (a, b) = self.tn.range(ng);
        let first = a / NODES_PER_PAGE;
        let last_excl = if self.cfg.mutant.fold_edge {
            b / NODES_PER_PAGE
        } else {
            (b - 1) / NODES_PER_PAGE + 1
        };
        if last_excl <= first {
            return (0, 0);
        }
        (self.tp.of(first), self.tp.of(last_excl - 1) + 1)
    }

    /// Node granules a page granule folds (forward of the fold).
    fn fold_forward(&self, pg: usize) -> (usize, usize) {
        let (pa, pb) = self.tp.range(pg);
        (
            self.tn.of(pa * NODES_PER_PAGE),
            self.tn.of(pb * NODES_PER_PAGE - 1) + 1,
        )
    }

    fn pull_node_stale(&mut self, ng: usize) -> bool {
        let last = self.last_node[ng];
        if self.gen_key[ng] > last || self.gen_kind[ng] > last {
            return true;
        }
        let (a, b) = self.tn.range(ng);
        for n in a .. b {
            self.work.relation_reads += 1;
            if self.gen_text[self.tt.of(self.inp.key[n] as usize)] > last {
                return true;
            }
        }
        false
    }

    fn pull_page_stale(&self, pg: usize) -> bool {
        let (a, b) = self.fold_forward(pg);
        (a .. b).any(|ng| self.gen_node[ng] > self.last_page[pg])
    }

    /// One frame. `demand` names the pages the frame must make current; `None`
    /// is a full frame.
    pub fn frame(&mut self, demand: Option<&[usize]>) {
        self.clock += 1;
        let now = self.clock;
        let mut want_page = vec![demand.is_none(); self.tp.count()];
        let mut want_node = vec![demand.is_none(); self.tn.count()];
        if let Some(pages) = demand {
            for &p in pages {
                let pg = self.tp.of(p);
                want_page[pg] = true;
            }
            for pg in 0 .. self.tp.count() {
                if want_page[pg] {
                    let (a, b) = self.fold_forward(pg);
                    for w in &mut want_node[a .. b] {
                        *w = true;
                    }
                }
            }
        }
        for ng in 0 .. self.tn.count() {
            if !want_node[ng] {
                continue;
            }
            let stale = match self.cfg.carry {
                Carry::Push => self.pend_node[ng],
                Carry::Pull => self.pull_node_stale(ng),
            };
            if !stale {
                continue;
            }
            let (a, b) = self.tn.range(ng);
            let mut differs = false;
            for n in a .. b {
                self.work.node_records += 1;
                let v = node_hash(self.inp.kind[n], self.inp.text[self.inp.key[n] as usize]);
                differs |= v != self.node[n];
                self.node[n] = v;
            }
            self.last_node[ng] = now;
            self.pend_node[ng] = false;
            let changed = match self.cfg.cutoff {
                Cutoff::Structural => true,
                Cutoff::Differs => differs,
            } && !self.cfg.mutant.cutoff_drops;
            if changed {
                self.gen_node[ng] = now;
                let (a, b) = self.fold_reverse(ng);
                for pg in a .. b {
                    self.pend_page[pg] = true;
                }
            }
        }
        for pg in 0 .. self.tp.count() {
            if !want_page[pg] {
                continue;
            }
            let stale = match self.cfg.carry {
                Carry::Push => self.pend_page[pg],
                Carry::Pull => self.pull_page_stale(pg),
            };
            if !stale {
                continue;
            }
            let (a, b) = self.tp.range(pg);
            for p in a .. b {
                self.work.page_records += 1;
                self.page[p] = page_hash(
                    p,
                    &self.node[p * NODES_PER_PAGE .. (p + 1) * NODES_PER_PAGE],
                );
            }
            self.last_page[pg] = now;
            self.pend_page[pg] = false;
        }
        if demand.is_some() && self.cfg.mutant.demand_clears_all {
            self.pend_node.iter_mut().for_each(|x| *x = false);
            self.pend_page.iter_mut().for_each(|x| *x = false);
            self.last_node.iter_mut().for_each(|x| *x = now);
            self.last_page.iter_mut().for_each(|x| *x = now);
        }
    }
}

/// Where an incremental run first parts from recomputation.
#[derive(Debug, PartialEq)]
pub struct Parted {
    pub seed: u64,
    pub step: usize,
    pub what: &'static str,
}

/// Drive one configuration through a random stream of nudges and frames,
/// holding it to from-scratch recomputation after every frame: every record
/// after a full frame, the demanded pages after a demand frame.
pub fn differential(cfg: Config, pages: usize, seed: u64, steps: usize) -> Result<(), Parted> {
    let mut rng = Rng::new(seed);
    let inp = site(pages, &mut rng);
    let mut e = Engine::new(cfg, inp);
    for step in 0 .. steps {
        match rng.below(10) {
            0 ..= 2 => {
                let k = rng.below(e.inp.text.len());
                let v = if rng.below(5) == 0 { e.inp.text[k] } else { rng.next() };
                e.set_text(k, v);
            },
            3 => {
                let n = rng.below(e.inp.key.len());
                let k = rng.below(e.inp.text.len());
                e.rekey(n, k);
            },
            4 => {
                let n = rng.below(e.inp.key.len());
                e.set_kind(n, rng.below(4) as u8);
            },
            5 | 6 => {
                let count = 1 + rng.below(3);
                let want: Vec<usize> = (0 .. count).map(|_| rng.below(pages)).collect();
                e.frame(Some(&want));
                let (_, sp) = scratch(&e.inp);
                if want.iter().any(|&p| e.page[p] != sp[p]) {
                    return Err(Parted {
                        seed,
                        step,
                        what: "a demanded page",
                    });
                }
            },
            _ => {
                e.frame(None);
                let (sn, sp) = scratch(&e.inp);
                if e.node != sn {
                    return Err(Parted {
                        seed,
                        step,
                        what: "a node hash",
                    });
                }
                if e.page != sp {
                    return Err(Parted {
                        seed,
                        step,
                        what: "a page hash",
                    });
                }
            },
        }
    }
    e.frame(None);
    let (sn, sp) = scratch(&e.inp);
    if e.node != sn || e.page != sp {
        return Err(Parted {
            seed,
            step: steps,
            what: "the closing full frame",
        });
    }
    Ok(())
}
