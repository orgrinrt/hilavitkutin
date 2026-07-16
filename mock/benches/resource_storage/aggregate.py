#!/usr/bin/env python3
"""Cross-run aggregation for the six-variant resource-storage bench.

Reads two (or more) runs of the rsb_* CSV matrix (each run = one dir of
rsb_<section>_n<N>.csv files produced by the orchestrator) and emits a
markdown summary: per (section, n, variant) median/min algo_ns per run,
ratio vs the section baseline, run-to-run agreement, plus a focused
V1-vs-V4 block (the tiebreak axis from round 202606210600).

Usage:
    python3 aggregate.py <run1-dir> <run2-dir> [more-run-dirs...]

Baselines per section: v0_blob where present, else wide_blob (bwide),
else seq_snapshot (seqd; snapshot-copy is the hypothesis-under-test
there, live-stream the challenger).
"""

import csv
import re
import statistics
import sys
from pathlib import Path

FNAME = re.compile(r"rsb_(?P<section>.+)_n(?P<n>\d+)\.csv$")
BASELINES = ("v0_blob", "wide_blob", "seq_snapshot")


def load_run(run_dir: Path):
    """{(section, n): {variant: [algo_ns...]}} for warm-mode samples."""
    out = {}
    for f in sorted(run_dir.glob("rsb_*.csv")):
        m = FNAME.search(f.name)
        if not m:
            continue
        key = (m.group("section"), int(m.group("n")))
        arms = out.setdefault(key, {})
        with f.open() as fh:
            for row in csv.DictReader(fh):
                if row.get("mode") != "warm":
                    continue
                try:
                    v = float(row["algo_ns"])
                except (KeyError, ValueError):
                    continue
                arms.setdefault(row["variant"], []).append(v)
    return out


def stats(samples):
    return statistics.median(samples), min(samples), len(samples)


def pick_baseline(variants):
    for b in BASELINES:
        if b in variants:
            return b
    return sorted(variants)[0]


def main(argv):
    if len(argv) < 3:
        print(__doc__, file=sys.stderr)
        return 2
    run_dirs = [Path(a) for a in argv[1:]]
    runs = [load_run(d) for d in run_dirs]
    labels = [d.name or str(d) for d in run_dirs]

    all_keys = sorted(set().union(*(r.keys() for r in runs)))
    print("# Cross-run aggregation: resource-storage bench\n")
    print(f"Runs compared: {', '.join(str(d) for d in run_dirs)}\n")

    v1v4 = []  # (section, n, [ratio-per-run])
    for key in all_keys:
        section, n = key
        per_run = [r.get(key, {}) for r in runs]
        variants = sorted(set().union(*(pr.keys() for pr in per_run)))
        if not variants:
            continue
        base = pick_baseline(variants)
        print(f"## {section} n={n}  (baseline: {base})\n")
        hdr = ["variant"]
        for lb in labels:
            hdr += [f"med {lb}", f"vs base", f"min {lb}"]
        hdr += ["run2/run1 med"]
        print("| " + " | ".join(hdr) + " |")
        print("|" + "---|" * len(hdr))
        meds = {}
        for var in variants:
            cells = [var]
            row_meds = []
            for pr in per_run:
                if var not in pr or base not in pr:
                    cells += ["-", "-", "-"]
                    row_meds.append(None)
                    continue
                med, mn, _cnt = stats(pr[var])
                bmed = statistics.median(pr[base])
                cells += [f"{med:.1f}", f"{med / bmed:.3f}x", f"{mn:.1f}"]
                row_meds.append(med)
            if all(m is not None for m in row_meds) and len(row_meds) >= 2:
                cells.append(f"{row_meds[1] / row_meds[0] - 1:+.1%}")
            else:
                cells.append("-")
            meds[var] = row_meds
            print("| " + " | ".join(cells) + " |")
        print()
        if "v1_snapshot" in meds and "v4_erased" in meds:
            ratios = []
            for i in range(len(runs)):
                a, b = meds["v1_snapshot"][i], meds["v4_erased"][i]
                ratios.append(b / a if a and b else None)
            v1v4.append((section, n, ratios))

    if v1v4:
        print("## Tiebreak axis: V4 (erased) vs V1 (snapshot), median ratio\n")
        print("Ratio > 1.0 means V4 slower than V1 by that factor.\n")
        hdr = ["section", "n"] + [f"v4/v1 {lb}" for lb in labels]
        print("| " + " | ".join(hdr) + " |")
        print("|" + "---|" * len(hdr))
        for section, n, ratios in v1v4:
            cells = [section, str(n)]
            cells += [f"{r:.3f}x" if r else "-" for r in ratios]
            print("| " + " | ".join(cells) + " |")
        print()
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv))
