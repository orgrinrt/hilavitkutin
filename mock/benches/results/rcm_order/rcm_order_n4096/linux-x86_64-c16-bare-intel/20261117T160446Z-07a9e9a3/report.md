# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (rcm_adj) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline rcm_adj has the worst median (19.35 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rcm_scr at 19.09 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 1.4% of the fastest

All 4 variants sit between 19.09 ms and 19.35 ms - a 1.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_scr** at 19090859.2 ns median (-1.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 19090859.2 ns, slowest 19350870.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 2112757625 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 19384408ns | 19354900ns | 18937159ns | 19244026ns | 19818606ns | base |
| rcm_half | 19185431ns | 19126988ns | 18771872ns | 19103403ns | 19515252ns | -1.03% |
| rcm_rev | 19218683ns | 19130448ns | 18894566ns | 19091019ns | 19572236ns | -0.85% |
| rcm_scr | 19104854ns | 19094866ns | 18809450ns | 19069355ns | 19305804ns | -1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 19380471ns | 18933221ns | 19814605ns | base | 0.000 |
| rcm_half | 19181603ns | 18768193ns | 19511383ns | -1.03% | 0.000 |
| rcm_rev | 19214645ns | 18890524ns | 19568223ns | -0.86% | 0.000 |
| rcm_scr | 19100953ns | 18805650ns | 19301907ns | -1.44% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 97.0% |
| rcm_half | 0.000 | 98.1% |
| rcm_rev | 0.000 | 98.1% |
| rcm_scr | 0.000 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 19384408ns | 19384408ns | base |
| rcm_half | 19185431ns | 19185431ns | -1.03% |
| rcm_rev | 19218683ns | 19218683ns | -0.85% |
| rcm_scr | 19104854ns | 19104854ns | -1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 19350871ns | base | --- | [19117263, 19467891] | --- | --- | --- | --- |
| rcm_half | 19123107ns | no significant difference | [-402324, +253702]ns | [19048925, 19318210] | no | 0.3568 | 0.2379 | 0 |
| rcm_rev | 19126274ns | no significant difference | [-257930, +43388]ns | [19044624, 19175574] | no | 0.4807 | 0.4807 | 0 |
| rcm_scr | 19090859ns | -132093.8ns (-0.7%) | [-434102, -31417]ns | [18981204, 19240113] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 18873320ns | +1.5% | -0.1% | +1.9% |
| 1 | 2 | 19494543ns | -4.1% | -2.1% | -3.5% |
| 1 | 3 | 19118402ns | -0.5% | -0.4% | -0.3% |
| 2 | 1 | 19356596ns | -0.3% | -1.0% | -2.0% |
| 2 | 2 | 19470112ns | -2.0% | +1.4% | -0.6% |
| 2 | 3 | 19087333ns | +1.4% | +0.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.473 | moderate- |
| rcm_half | -0.066 | ok |
| rcm_rev | 0.054 | ok |
| rcm_scr | -0.187 | ok |

**Consistency summary:**

- **rcm_half**: won 4/6, lost 2/6
- **rcm_rev**: won 4/6, lost 2/6
- **rcm_scr**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 123467040.7ns | 19380471.5ns | 637.1% | HIGH |
| rcm_half | 123351761.5ns | 19181603.1ns | 643.1% | HIGH |
| rcm_rev | 124143732.8ns | 19214644.7ns | 646.1% | HIGH |
| rcm_scr | 123121202.5ns | 19100953.1ns | 644.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 18933221.2-19814604.9 ns)
  18933221.2 |
  18977290.4 |
  19021359.6 |##########
  19065428.8 |##########
  19109498.0 |########################################
  19153567.2 |
  19197636.3 |
  19241705.5 |
  19285774.7 |
  19329843.9 |####################
  19373913.1 |
  19417982.3 |####################
  19462051.5 |##############################
  19506120.6 |
  19550189.8 |
  19594259.0 |##########
  19638328.2 |
  19682397.4 |
  19726466.6 |
  19770535.8 |
  (2 below, 2 above range)

rcm_half (n=18, range 18768193.5-19511383.4 ns)
  18768193.5 |
  18805353.0 |
  18842512.5 |
  18879672.0 |
  18916831.5 |
  18953991.0 |
  18991150.5 |##########################
  19028310.0 |##########################
  19065469.5 |########################################
  19102629.0 |
  19139788.5 |##########################
  19176948.0 |#############
  19214107.5 |
  19251267.0 |
  19288426.5 |#############
  19325585.9 |##########################
  19362745.4 |
  19399904.9 |#############
  19437064.4 |#############
  19474223.9 |
  (2 below, 1 above range)

rcm_rev (n=18, range 18890524.4-19568223.1 ns)
  18890524.4 |
  18924409.4 |
  18958294.3 |#############
  18992179.2 |#############
  19026064.2 |########################################
  19059949.1 |#############
  19093834.0 |##########################
  19127719.0 |##########################
  19161603.9 |##########################
  19195488.8 |
  19229373.8 |
  19263258.7 |
  19297143.6 |#############
  19331028.6 |
  19364913.5 |
  19398798.4 |
  19432683.4 |
  19466568.3 |
  19500453.2 |#############
  19534338.2 |
  (2 below, 2 above range)

rcm_scr (n=18, range 18805650.0-19301907.4 ns)
  18805650.0 |####################
  18830462.9 |
  18855275.7 |
  18880088.6 |
  18904901.5 |
  18929714.3 |
  18954527.2 |########################################
  18979340.1 |####################
  19004153.0 |
  19028965.8 |####################
  19053778.7 |########################################
  19078591.6 |
  19103404.4 |####################
  19128217.3 |
  19153030.2 |####################
  19177843.0 |
  19202655.9 |####################
  19227468.8 |########################################
  19252281.6 |
  19277094.5 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=636.8% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=642.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=643.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=642.7% of algo (FFI overhead may distort results)
