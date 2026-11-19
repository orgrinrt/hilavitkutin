//! Every row of the two tables in `measured.rs`, read back from the run
//! directory its comment cites: the first warm pass of the slower arm rounded
//! up to a second, and the largest target directory a cell left. A row that no
//! run directory gives, or a run directory no row accounts for, fails here.

use build_call::measured::{FIRST_CALL_S, Row, TARGET_BYTES, call_key, disk_key};

const DEV: u8 = 0;
const RELEASE: u8 = 1;
const CLEAN: u8 = 0;
const LEAF: u8 = 1;
const MID: u8 = 2;
const ROOT: u8 = 3;
const LINK: u8 = 4;
const INCREMENTAL: &str = "llvm_incremental";
const NON_INCREMENTAL: &str = "llvm_non_incremental";

/// The samples and the call record of one cell's run directory.
struct Cell {
    samples: &'static str,
    calls:   &'static str,
}

macro_rules! cell {
    ($bench:literal, $sweep:literal, $class:literal, $run:literal) => {
        Cell {
            samples: include_str!(concat!(
                "../../../results/",
                $bench,
                "/",
                $sweep,
                "/",
                $class,
                "/",
                $run,
                "/samples.csv"
            )),
            calls:   include_str!(concat!(
                "../../../results/",
                $bench,
                "/",
                $sweep,
                "/",
                $class,
                "/",
                $run,
                "/calls.tsv"
            )),
        }
    };
}

const CLASS: &str = "linux-x86_64-c4-container-intel";

fn build_at_25(sweep: &str) -> Cell {
    match sweep {
        "clean" => {
            cell!(
                "build-static",
                "clean_n25",
                "linux-x86_64-c4-container-intel",
                "20261119T150158Z-ffb57e5a"
            )
        },
        "leaf" => {
            cell!(
                "build-static",
                "leaf_n25",
                "linux-x86_64-c4-container-intel",
                "20261119T150158Z-ffb57e5a"
            )
        },
        "mid" => {
            cell!(
                "build-static",
                "mid_n25",
                "linux-x86_64-c4-container-intel",
                "20261119T150158Z-ffb57e5a"
            )
        },
        "root" => {
            cell!(
                "build-static",
                "root_n25",
                "linux-x86_64-c4-container-intel",
                "20261119T150158Z-ffb57e5a"
            )
        },
        _ => unreachable!("four sweeps"),
    }
}

fn build_at_97(sweep: &str) -> Cell {
    match sweep {
        "clean" => {
            cell!(
                "build-static",
                "clean_n97",
                "linux-x86_64-c4-container-intel",
                "20261119T171604Z-fe400621"
            )
        },
        "leaf" => {
            cell!(
                "build-static",
                "leaf_n97",
                "linux-x86_64-c4-container-intel",
                "20261119T203124Z-4228372a"
            )
        },
        "mid" => {
            cell!(
                "build-static",
                "mid_n97",
                "linux-x86_64-c4-container-intel",
                "20261119T203124Z-4228372a"
            )
        },
        "root" => {
            cell!(
                "build-static",
                "root_n97",
                "linux-x86_64-c4-container-intel",
                "20261119T203124Z-4228372a"
            )
        },
        _ => unreachable!("four sweeps"),
    }
}

const SWEEPS: [(&str, u8); 4] = [("clean", CLEAN), ("leaf", LEAF), ("mid", MID), ("root", ROOT)];

/// The warm samples of the first pass, in nanoseconds, of the arm named or of
/// every arm: the largest.
fn first_call_ns(samples: &str, arm: Option<&str>) -> f64 {
    let mut slowest: f64 = 0.0;
    for line in samples.lines().skip(1) {
        let f: Vec<&str> = line.split(',').collect();
        if f[1] == "1" && f[3] == "warm" && arm.is_none_or(|a| f[4] == a) {
            slowest = slowest.max(f[6].parse::<f64>().expect("e2e_ns is a number"));
        }
    }
    assert!(slowest > 0.0, "no warm sample of the first pass");
    slowest
}

/// Whole seconds, rounded up.
fn secs_up(ns: f64) -> u64 {
    (ns / 1e9).ceil() as u64
}

/// The largest `target_bytes` an arm's calls recorded.
fn target_max(calls: &str, arm: &str) -> u64 {
    calls
        .lines()
        .skip(1)
        .map(|l| l.split('\t').collect::<Vec<&str>>())
        .filter(|f| f[1] == arm)
        .map(|f| f[13].parse::<u64>().expect("target_bytes is a number"))
        .max()
        .expect("the arm has calls")
}

#[test]
fn the_rounding_goes_up_and_leaves_a_whole_second_alone() {
    assert_eq!(secs_up(123_171_044_654.9), 124);
    assert_eq!(secs_up(123_000_000_000.0), 123);
    assert_eq!(secs_up(512_923_605.4), 1);
    let csv = "run,pass,cooldown_ms,mode,variant,batch_idx,e2e_ns\n\
               1,1,0,warm,a,0,5.0\n1,1,0,cold,a,0,90.0\n1,2,0,warm,a,0,70.0\n1,1,0,warm,b,0,8.0\n";
    assert_eq!(
        first_call_ns(csv, None),
        8.0,
        "the slower arm, the first pass, warm"
    );
    assert_eq!(first_call_ns(csv, Some("a")), 5.0);
}

#[test]
fn every_row_of_the_first_call_table_is_the_reading_of_the_run_it_cites() {
    let mut expected: Vec<Row> = Vec::new();
    for (sweep, scenario) in SWEEPS {
        for (n, cell) in [(25, build_at_25(sweep)), (97, build_at_97(sweep))] {
            expected.push((
                call_key(scenario, DEV),
                n,
                secs_up(first_call_ns(cell.samples, None)),
            ));
        }
    }
    let dev_25 = cell!(
        "link-static",
        "dev_n25",
        "linux-x86_64-c4-container-intel",
        "20261119T134226Z-07a9e9a3"
    );
    let release_25 = cell!(
        "link-static",
        "release_n25",
        "linux-x86_64-c4-container-intel",
        "20261119T134226Z-07a9e9a3"
    );
    expected.push((
        call_key(LINK, DEV),
        25,
        secs_up(first_call_ns(dev_25.samples, None)),
    ));
    expected.push((
        call_key(LINK, RELEASE),
        25,
        secs_up(first_call_ns(release_25.samples, None)),
    ));
    // dev 97 is not a timed call: the untimed build of the 97 crates plus the
    // relink, which the harness's probe counts (ticket
    // `the_bench_harness_probe_counts_the_untimed_setup_of_a_build`)
    let link_97 = cell!(
        "link-static",
        "dev_n97",
        "linux-x86_64-c4-container-intel",
        "20261119T222847Z-2eb71914"
    );
    let build = first_call_ns(build_at_97("clean").samples, Some(NON_INCREMENTAL));
    let relink = first_call_ns(link_97.samples, Some("default_relink"));
    expected.push((call_key(LINK, DEV), 97, secs_up(build + relink)));

    let mut table = FIRST_CALL_S.to_vec();
    table.sort_unstable();
    expected.sort_unstable();
    assert_eq!(table, expected, "{CLASS}");
}

#[test]
fn every_row_of_the_disk_table_is_the_largest_target_of_the_run_it_cites() {
    let mut expected: Vec<Row> = Vec::new();
    for (n, cells) in [
        (25, SWEEPS.map(|(s, _)| build_at_25(s))),
        (97, SWEEPS.map(|(s, _)| build_at_97(s))),
    ] {
        for (incremental, arm) in [(true, INCREMENTAL), (false, NON_INCREMENTAL)] {
            let largest = cells
                .iter()
                .map(|c| target_max(c.calls, arm))
                .max()
                .unwrap();
            expected.push((disk_key(incremental), n, largest));
        }
    }
    let mut table = TARGET_BYTES.to_vec();
    table.sort_unstable();
    expected.sort_unstable();
    assert_eq!(table, expected);
}
