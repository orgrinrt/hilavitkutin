//! Records recomputed and relation entries touched per scenario, per
//! configuration, on loru's site shape. Counts from the model, no timer.
//! Each row is checked against recomputation before it is printed.

use per_record_marks_probe::*;

fn configs() -> Vec<(String, Config)> {
    let mut out = Vec::new();
    let mut grains = vec![("store".to_string(), Grain::Store)];
    for w in [64, 256, 2304, 4608, 8192] {
        grains.push((format!("morsel{w}"), Grain::Morsel(w)));
    }
    grains.push(("record".to_string(), Grain::Record));
    for (gname, grain) in grains {
        for (cname, carry) in [("push", Carry::Push), ("pull", Carry::Pull)] {
            for (xname, cutoff) in
                [("structural", Cutoff::Structural), ("differs", Cutoff::Differs)]
            {
                out.push((
                    format!("{gname}\t{cname}\t{xname}"),
                    Config::new(grain, carry, cutoff),
                ));
            }
        }
    }
    out
}

fn check(e: &Engine) {
    let (n, p) = scratch(&e.inp);
    assert_eq!(e.node, n);
    assert_eq!(e.page, p);
}

fn main() {
    println!(
        "pages\tscenario\tgrain\tcarry\tcutoff\tnode_records\tpage_records\trelation_reads\tinverse_writes"
    );
    for pages in [300, 3000] {
        let mid = pages / 2;
        for (name, cfg) in configs() {
            let scenarios: Vec<(&str, Box<dyn Fn(&mut Engine)>)> = vec![
                (
                    "unique_edit",
                    Box::new(move |e: &mut Engine| {
                        let k = unique_key(&e.inp, mid, 10);
                        e.set_text(k, 0xDEAD);
                        e.frame(None);
                    }),
                ),
                (
                    "global_edit",
                    Box::new(|e: &mut Engine| {
                        e.set_text(0, 0xBEEF);
                        e.frame(None);
                    }),
                ),
                (
                    "rekey",
                    Box::new(move |e: &mut Engine| {
                        let k = unique_key(&e.inp, 7, 12);
                        e.rekey(mid * NODES_PER_PAGE + 10, k);
                        e.frame(None);
                    }),
                ),
                (
                    "noop_edit",
                    Box::new(move |e: &mut Engine| {
                        let k = unique_key(&e.inp, mid, 10);
                        let v = e.inp.text[k];
                        e.set_text(k, v);
                        e.frame(None);
                    }),
                ),
                (
                    "demand_other_page",
                    Box::new(move |e: &mut Engine| {
                        let k = unique_key(&e.inp, 7, 10);
                        e.set_text(k, 0xF00D);
                        e.frame(Some(&[mid]));
                    }),
                ),
                (
                    "demand_edited_page",
                    Box::new(move |e: &mut Engine| {
                        let k = unique_key(&e.inp, 7, 10);
                        e.set_text(k, 0xF00D);
                        e.frame(Some(&[7]));
                    }),
                ),
            ];
            for (sname, scenario) in scenarios {
                let mut rng = Rng::new(7);
                let inp = site(pages, &mut rng);
                let mut e = Engine::new(cfg, inp);
                e.frame(None);
                check(&e);
                e.work = Work::default();
                scenario(&mut e);
                let w = e.work;
                if sname.starts_with("demand") {
                    let (_, sp) = scratch(&e.inp);
                    let p = if sname == "demand_other_page" { mid } else { 7 };
                    assert_eq!(e.page[p], sp[p]);
                    e.frame(None);
                    check(&e);
                } else {
                    check(&e);
                }
                println!(
                    "{pages}\t{sname}\t{name}\t{}\t{}\t{}\t{}",
                    w.node_records, w.page_records, w.relation_reads, w.inverse_writes
                );
            }
        }
    }
}
