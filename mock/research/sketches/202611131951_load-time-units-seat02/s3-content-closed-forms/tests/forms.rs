use s3_content_closed_forms::*;

static ENV: Envelope = Envelope { names: &["heat", "fuel", "burn", "spread"], writable_from: 2 };

fn cols(n: usize) -> Vec<Vec<i64>> {
    vec![(0..n as i64).map(|x| x << 12).collect(), (0..n as i64).map(|x| 1000 - x).collect(), vec![0; n], vec![0; n]]
}

const PACK: &str = "
# a pack's rows: burn = clamp(heat * 0.75, 0, 9000); spread = burn + fuel
scale burn heat 49152
clamp burn burn 0 9000
sum spread burn fuel
";

#[test]
fn rows_compose_to_what_a_hand_written_unit_computes() {
    let rules = load(&ENV, PACK).expect("the pack loads");
    let mut c = cols(10_000);
    run_pass(&rules, &mut c, 1024);
    for i in 0..10_000usize {
        let heat = (i as i64) << 12;
        let burn = ((heat * 49152) >> 16).clamp(0, 9000);
        assert_eq!(c[2][i], burn, "burn at {i}");
        assert_eq!(c[3][i], burn + (1000 - i as i64), "spread at {i}");
    }
}

#[test]
fn a_row_writing_a_store_the_unit_only_reads_is_refused_at_load() {
    assert_eq!(load(&ENV, "offset heat fuel 3"), Err(LoadRefusal::StoreOutsideEnvelope("heat".into())));
}

#[test]
fn a_row_naming_a_store_the_unit_never_declared_is_refused_at_load() {
    assert_eq!(load(&ENV, "sum burn heat moisture"), Err(LoadRefusal::StoreOutsideEnvelope("moisture".into())));
}

/// The arm's wall. A mod that wants the heat of the neighbouring record, a new
/// form, has no row to write: the set of forms is the build's.
#[test]
fn a_row_asking_for_a_form_the_build_does_not_hold_is_refused() {
    assert_eq!(load(&ENV, "neighbour_mean burn heat 1"), Err(LoadRefusal::UnknownForm("neighbour_mean".into())));
}
