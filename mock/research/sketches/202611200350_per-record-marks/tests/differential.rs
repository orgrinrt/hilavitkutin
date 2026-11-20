//! Every configuration equals recomputation after every frame, over the whole
//! matrix of grain, carry and cutoff, at window widths chosen to straddle the
//! twenty-node page (under it, at it, past it, not a multiple of it, wider
//! than the store). Each mutant must be caught somewhere in the matrix it
//! applies to, or the differential is not shown able to fail.

use per_record_marks_probe::*;

const PAGES: [usize; 4] = [1, 3, 7, 13];
const SEEDS: u64 = 24;
const STEPS: usize = 300;

fn grains() -> Vec<Grain> {
    let mut g = vec![Grain::Store, Grain::Record];
    for w in [1, 2, 3, 7, 19, 20, 21, 40, 64, 1000] {
        g.push(Grain::Morsel(w));
    }
    g
}

fn matrix() -> Vec<Config> {
    let mut out = Vec::new();
    for grain in grains() {
        for carry in [Carry::Push, Carry::Pull] {
            for cutoff in [Cutoff::Structural, Cutoff::Differs] {
                out.push(Config::new(grain, carry, cutoff));
            }
        }
    }
    out
}

#[test]
fn every_configuration_equals_recomputation_after_every_frame() {
    let mut runs = 0;
    for cfg in matrix() {
        for pages in PAGES {
            for seed in 0 .. SEEDS {
                if let Err(p) = differential(cfg, pages, seed, STEPS) {
                    panic!("{cfg:?} at {pages} pages parted: {p:?}");
                }
                runs += 1;
            }
        }
    }
    assert_eq!(runs, 12 * 2 * 2 * PAGES.len() * SEEDS as usize);
}

/// The configurations a mutant applies to, and whether any of them catches it.
fn caught(mutant: Mutant, carries: &[Carry], grains: &[Grain]) -> Option<(Config, usize, Parted)> {
    for &grain in grains {
        for &carry in carries {
            for cutoff in [Cutoff::Structural, Cutoff::Differs] {
                let cfg = Config {
                    grain,
                    carry,
                    cutoff,
                    mutant,
                };
                for pages in PAGES {
                    for seed in 0 .. SEEDS {
                        if let Err(p) = differential(cfg, pages, seed, STEPS) {
                            return Some((cfg, pages, p));
                        }
                    }
                }
            }
        }
    }
    None
}

fn finer() -> Vec<Grain> {
    vec![Grain::Record, Grain::Morsel(3), Grain::Morsel(7), Grain::Morsel(21)]
}

#[test]
fn a_stale_inverse_after_a_rekey_is_caught_under_push() {
    let m = Mutant {
        stale_inverse: true,
        ..Mutant::default()
    };
    let hit = caught(m, &[Carry::Push], &finer());
    println!("stale_inverse: {hit:?}");
    assert!(hit.is_some());
}

#[test]
fn a_stale_inverse_is_harmless_at_store_grain_and_under_pull() {
    // Store grain has one granule, so its reverse relation is all to all and
    // never goes stale. Pull reads the forward index and keeps no inverse.
    let m = Mutant {
        stale_inverse: true,
        ..Mutant::default()
    };
    assert!(caught(m, &[Carry::Push, Carry::Pull], &[Grain::Store]).is_none());
    assert!(caught(m, &[Carry::Pull], &finer()).is_none());
}

#[test]
fn a_demand_frame_that_forgets_what_it_did_not_run_is_caught() {
    let m = Mutant {
        demand_clears_all: true,
        ..Mutant::default()
    };
    let push = caught(m, &[Carry::Push], &finer());
    let pull = caught(m, &[Carry::Pull], &finer());
    println!("demand_clears_all: push {push:?}, pull {pull:?}");
    assert!(push.is_some() && pull.is_some());
}

#[test]
fn a_fold_reverse_that_misses_the_last_page_is_caught_under_push() {
    let m = Mutant {
        fold_edge: true,
        ..Mutant::default()
    };
    let hit = caught(m, &[Carry::Push], &finer());
    println!("fold_edge: {hit:?}");
    assert!(hit.is_some());
}

#[test]
fn a_cutoff_that_drops_every_change_is_caught_at_every_grain() {
    let m = Mutant {
        cutoff_drops: true,
        ..Mutant::default()
    };
    for grain in grains() {
        for carry in [Carry::Push, Carry::Pull] {
            assert!(
                caught(m, &[carry], &[grain]).is_some(),
                "{grain:?} {carry:?}"
            );
        }
    }
}
