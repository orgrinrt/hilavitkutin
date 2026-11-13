use s2_extension_surface_unit::*;

const A: u64 = store_id("kaski.probe.a");
const B: u64 = store_id("kaski.probe.b");
const OUT: u64 = store_id("kaski.probe.out");
const OTHER: u64 = store_id("kaski.probe.other");

static HOST_READS: [u64; 2] = [A, B];
static HOST_WRITES: [u64; 1] = [OUT];
static ENV: HostEnvelope = HostEnvelope { reads: &HOST_READS, writes: &HOST_WRITES };

fn columns(n: usize) -> Columns {
    Columns {
        ids: vec![A, B, OUT, OTHER],
        data: vec![
            (0..n as i64).collect(),
            (0..n as i64).map(|x| 7 - x).collect(),
            vec![0; n],
            vec![0; n],
        ],
    }
}

fn expected(n: usize) -> Vec<i64> {
    (0..n as i64).map(|x| x * 3 + (7 - x)).collect()
}

static R_AB: [u64; 2] = [A, B];
static W_OUT: [u64; 1] = [OUT];
static W_OTHER: [u64; 1] = [OTHER];
static R_A: [u64; 1] = [A];

fn decl(reads: &'static [u64], writes: &'static [u64], run: MorselFn) -> UnitDecl {
    UnitDecl { reads: reads.as_ptr(), reads_len: reads.len() as u32, writes: writes.as_ptr(), writes_len: writes.len() as u32, run }
}

#[test]
fn a_conforming_extension_is_admitted_and_computes_the_function() {
    let ad = admit(&ENV, &decl(&R_AB, &W_OUT, s2_ext_compiled)).expect("admitted");
    let mut c = columns(10_000);
    ad.run_pass(&mut c, 1024, None);
    assert_eq!(c.data[2], expected(10_000));
}

#[test]
fn a_write_outside_the_host_units_envelope_is_refused_at_load() {
    match admit(&ENV, &decl(&R_AB, &W_OTHER, s2_ext_compiled)) {
        Err(Refusal::WriteOutsideEnvelope(id)) => assert_eq!(id, OTHER),
        Err(e) => panic!("refused for the wrong reason: {e:?}"),
        Ok(_) => panic!("admitted a write the host unit never declared"),
    }
}

#[test]
fn a_read_of_a_store_the_host_unit_never_declared_is_refused_at_load() {
    static R_OTHER: [u64; 1] = [OTHER];
    match admit(&ENV, &decl(&R_OTHER, &W_OUT, s2_ext_compiled)) {
        Err(Refusal::ReadOutsideEnvelope(id)) => assert_eq!(id, OTHER),
        other => panic!("expected a read refusal, got {:?}", other.err()),
    }
}

#[test]
fn swapping_the_variant_between_morsels_changes_nothing_in_the_result() {
    for k in [0usize, 1, 3, 9] {
        let ad = admit(&ENV, &decl(&R_AB, &W_OUT, s2_ext_interpreted)).expect("admitted");
        let mut c = columns(10_000);
        ad.run_pass(&mut c, 1024, Some((k, s2_ext_compiled)));
        assert_eq!(c.data[2], expected(10_000), "swap after morsel {k}");
    }
}

/// The perimeter. Admission saw a declaration; the code behind it is native and
/// full trust (the extensions design, non-scope: "V1 is full-trust native code; no
/// isolation"). This test states the property a sandbox would give and is red
/// against this arm on purpose.
#[test]
fn an_extension_that_writes_what_it_declared_as_read_is_caught() {
    let ad = admit(&ENV, &decl(&R_A, &W_OUT, s2_ext_liar)).expect("its declaration conforms, so it is admitted");
    let mut c = columns(4096);
    let before = c.data[0].clone();
    ad.run_pass(&mut c, 1024, None);
    let changed = c.data[0].iter().zip(before.iter()).filter(|(x, y)| x != y).count();
    assert_eq!(changed, 0, "records of the read-only store written with nothing at the boundary noticing");
}
