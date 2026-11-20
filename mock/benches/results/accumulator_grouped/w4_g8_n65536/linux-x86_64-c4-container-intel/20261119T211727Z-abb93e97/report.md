# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 27% faster than the next best (grouped)

per_core (132.25 us) leads grouped (168.34 us) by 27%, a clear separation rather than a photo finish. CV 17.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.3x slower than the field

sequential (307.79 us) is 2.3x the fastest (132.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped shows alternating (throttle bounce) (autocorr -0.65)

grouped's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 2.0x its best-20%

grouped's best 20% of batches run at 111.93 us but its worst 20% at 221.96 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 132250.6 ns median (-21.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.33x (fastest 132250.6 ns, slowest 307786.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 172621ns | 169472ns | 112407ns | 159288ns | 222729ns | base |
| per_core | 143112ns | 132872ns | 116114ns | 132069ns | 173174ns | -17.10% |
| sequential | 306554ns | 309318ns | 275925ns | 301554ns | 329369ns | +77.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 171816ns | 111930ns | 221956ns | base | 0.381 |
| per_core | 142425ns | 115293ns | 172480ns | -17.11% | 0.460 |
| sequential | 305354ns | 274908ns | 327932ns | +77.72% | 0.215 |

## Performance model

- Peak throughput: **0.586 Gops/s** (grouped; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.389 | 66.5% |
| per_core | 0.496 | 84.6% |
| sequential | 0.213 | 36.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 172621ns | 172621ns | base |
| per_core | 143112ns | 143112ns | -17.10% |
| sequential | 306554ns | 306554ns | +77.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 168338ns | base | --- | [145981, 188955] | --- | --- | --- | --- |
| per_core | 132251ns | no significant difference | [-41935, +3545]ns | [126570, 166804] | no | 0.4807 | 0.4807 | 0 |
| sequential | 307786ns | +125938.1ns (+74.8%) | [+107828, +166992]ns | [288150, 320846] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 188955ns | -31.3% | +65.6% |
| 1 | 2 | 121266ns | +10.5% | +165.7% |
| 1 | 3 | 177526ns | -5.4% | +60.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.650 | HIGH- (thermal bounce) |
| per_core | -0.113 | ok |
| sequential | -0.319 | moderate- |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 237326.7ns | 171815.6ns | 138.1% | HIGH |
| per_core | 149322.5ns | 142424.8ns | 104.8% | HIGH |
| sequential | 306489.4ns | 305354.1ns | 100.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 111929.5-221956.2 ns)
  111929.5 |##########################
  117430.9 |
  122932.2 |
  128433.5 |#############
  133934.9 |
  139436.2 |
  144937.5 |########################################
  150438.9 |#############
  155940.2 |
  161441.5 |#############
  166942.9 |#############
  172444.2 |
  177945.5 |#############
  183446.9 |##########################
  188948.2 |##########################
  194449.5 |##########################
  199950.9 |
  205452.2 |
  210953.5 |
  216454.9 |
  (1 below, 1 above range)

per_core (n=18, range 115293.0-172479.6 ns)
  115293.0 |####################
  118152.3 |##########
  121011.7 |##########
  123871.0 |
  126730.3 |
  129589.7 |########################################
  132449.0 |####################
  135308.3 |
  138167.7 |
  141027.0 |
  143886.3 |
  146745.6 |##########
  149605.0 |
  152464.3 |
  155323.6 |
  158183.0 |
  161042.3 |
  163901.6 |##########
  166761.0 |####################
  169620.3 |##########
  (1 below, 2 above range)

sequential (n=18, range 274907.9-327932.1 ns)
  274907.9 |
  277559.1 |
  280210.3 |
  282861.5 |########################################
  285512.7 |####################
  288163.9 |####################
  290815.1 |####################
  293466.4 |
  296117.6 |
  298768.8 |
  301420.0 |####################
  304071.2 |####################
  306722.4 |
  309373.6 |####################
  312024.8 |
  314676.0 |####################
  317327.2 |####################
  319978.4 |########################################
  322629.6 |########################################
  325280.9 |####################
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=32.2% (high variance, measurements may be unstable)
- **grouped**: bridge=138.1% of algo (FFI overhead may distort results)
- **per_core**: bridge=110.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.7% of algo (FFI overhead may distort results)
