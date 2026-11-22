# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (7.74 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_rowm at 7.66 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### riv_spec shows warm-up / thermal drift (autocorr +0.53)

riv_spec's per-pass series has lag-1 autocorrelation +0.53, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (76.36 us) is smaller than the fastest variant's own run-to-run std-dev (78.47 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.0% of the fastest

All 5 variants sit between 7.66 ms and 7.74 ms - a 1.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_rowm** at 7659679.2 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 7659679.2 ns, slowest 7736043.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 7776170ns | 7742348ns | 7653453ns | 7732547ns | 7902963ns | base |
| riv_rowm | 7664873ns | 7665867ns | 7582136ns | 7637698ns | 7747005ns | -1.43% |
| riv_scr | 7743765ns | 7667435ns | 7621828ns | 7669630ns | 7915935ns | -0.42% |
| riv_snake | 7736759ns | 7700710ns | 7566626ns | 7678311ns | 7909497ns | -0.51% |
| riv_spec | 7748285ns | 7723615ns | 7584051ns | 7704298ns | 7896384ns | -0.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 7769648ns | 7647004ns | 7896074ns | base | 0.000 |
| riv_rowm | 7658262ns | 7575664ns | 7740324ns | -1.43% | 0.000 |
| riv_scr | 7737384ns | 7615758ns | 7909076ns | -0.42% | 0.000 |
| riv_snake | 7730057ns | 7560193ns | 7902232ns | -0.51% | 0.000 |
| riv_spec | 7741600ns | 7577676ns | 7889248ns | -0.36% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.7% |
| riv_rowm | 0.000 | 98.7% |
| riv_scr | 0.000 | 98.7% |
| riv_snake | 0.000 | 98.3% |
| riv_spec | 0.000 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 7776170ns | 7776170ns | base |
| riv_rowm | 7664873ns | 7664873ns | -1.43% |
| riv_scr | 7743765ns | 7743765ns | -0.42% |
| riv_snake | 7736759ns | 7736759ns | -0.51% |
| riv_spec | 7748285ns | 7748285ns | -0.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 7736044ns | base | --- | [7710492, 7834133] | --- | --- | --- | --- |
| riv_rowm | 7659679ns | -85989.6ns (-1.1%) | [-171383, -58844]ns | [7599417, 7687127] | YES | 0.0052 | 0.0013 | 0 |
| riv_scr | 7661300ns | no significant difference | [-117452, +39256]ns | [7633015, 7814804] | no | 0.3172 | 0.2379 | 0 |
| riv_snake | 7694202ns | no significant difference | [-136633, +11792]ns | [7623771, 7800762] | no | 0.3172 | 0.2379 | 0 |
| riv_spec | 7717017ns | no significant difference | [-137987, +63967]ns | [7636867, 7845979] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 7658412ns | -1.0% | -0.3% | +0.1% | +0.8% |
| 1 | 2 | 7775000ns | -1.5% | -1.1% | -1.8% | -1.6% |
| 1 | 3 | 7841558ns | -1.9% | -2.6% | -1.5% | -2.8% |
| 2 | 1 | 7899083ns | -3.0% | -3.2% | -2.7% | -1.9% |
| 2 | 2 | 7715538ns | -0.0% | -0.6% | +0.6% | +2.0% |
| 2 | 3 | 7723233ns | -1.7% | +0.7% | +1.4% | +1.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.120 | ok |
| riv_rowm | -0.196 | ok |
| riv_scr | -0.019 | ok |
| riv_snake | 0.278 | moderate+ |
| riv_spec | 0.531 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 0/6
- **riv_scr**: won 5/6, lost 1/6
- **riv_snake**: won 3/6, lost 3/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 134118538.2ns | 7769647.7ns | 1726.2% | HIGH |
| riv_rowm | 133970198.6ns | 7658261.8ns | 1749.4% | HIGH |
| riv_scr | 133980363.4ns | 7737384.3ns | 1731.6% | HIGH |
| riv_snake | 134054368.5ns | 7730056.9ns | 1734.2% | HIGH |
| riv_spec | 134002919.7ns | 7741600.0ns | 1730.9% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 7647004.2-7896073.6 ns)
  7647004.2 |####################
  7659457.6 |####################
  7671911.1 |
  7684364.6 |
  7696818.1 |####################
  7709271.5 |####################
  7721725.0 |########################################
  7734178.5 |########################################
  7746631.9 |####################
  7759085.4 |
  7771538.9 |####################
  7783992.4 |
  7796445.8 |
  7808899.3 |
  7821352.8 |####################
  7833806.2 |####################
  7846259.7 |
  7858713.2 |
  7871166.7 |
  7883620.1 |####################
  (2 below, 3 above range)

riv_rowm (n=18, range 7575663.9-7740324.3 ns)
  7575663.9 |
  7583896.9 |########################################
  7592129.9 |########################################
  7600363.0 |
  7608596.0 |####################
  7616829.0 |
  7625062.0 |####################
  7633295.0 |
  7641528.1 |
  7649761.1 |####################
  7657994.1 |########################################
  7666227.1 |####################
  7674460.2 |
  7682693.2 |########################################
  7690926.2 |
  7699159.2 |
  7707392.2 |########################################
  7715625.3 |
  7723858.3 |
  7732091.3 |####################
  (2 below, 1 above range)

riv_scr (n=18, range 7615758.3-7909075.7 ns)
  7615758.3 |########################################
  7630424.2 |####################
  7645090.1 |####################
  7659755.9 |##########
  7674421.8 |##########
  7689087.7 |
  7703753.5 |
  7718419.4 |
  7733085.3 |
  7747751.1 |
  7762417.0 |
  7777082.9 |##########
  7791748.8 |##########
  7806414.6 |
  7821080.5 |##########
  7835746.4 |
  7850412.2 |##########
  7865078.1 |
  7879744.0 |
  7894409.8 |
  (1 below, 3 above range)

riv_snake (n=18, range 7560193.1-7902231.9 ns)
  7560193.1 |####################
  7577295.0 |
  7594397.0 |
  7611498.9 |########################################
  7628600.8 |########################################
  7645702.8 |
  7662804.7 |####################
  7679906.7 |####################
  7697008.6 |####################
  7714110.6 |####################
  7731212.5 |
  7748314.5 |####################
  7765416.4 |####################
  7782518.3 |
  7799620.3 |
  7816722.2 |####################
  7833824.2 |
  7850926.1 |
  7868028.1 |
  7885130.0 |
  (2 below, 4 above range)

riv_spec (n=18, range 7577676.4-7889247.9 ns)
  7577676.4 |##########################
  7593255.0 |
  7608833.6 |##########################
  7624412.1 |
  7639990.7 |#############
  7655569.3 |#############
  7671147.9 |
  7686726.4 |
  7702305.0 |########################################
  7717883.6 |
  7733462.2 |#############
  7749040.7 |
  7764619.3 |
  7780197.9 |
  7795776.5 |
  7811355.0 |
  7826933.6 |##########################
  7842512.2 |
  7858090.8 |##########################
  7873669.3 |#############
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1732.7% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1748.3% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1747.8% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1741.6% of algo (FFI overhead may distort results)
- **riv_spec**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **riv_spec**: bridge=1735.5% of algo (FFI overhead may distort results)
