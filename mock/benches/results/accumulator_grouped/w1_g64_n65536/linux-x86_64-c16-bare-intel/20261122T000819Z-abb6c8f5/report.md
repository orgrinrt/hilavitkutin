# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (186.21 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 176.70 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential is fastest but the noisiest (CV 24.8%)

sequential wins on median (176.70 us) yet has the highest variance (CV 24.8%), while per_core is the steadiest (CV 13.2%, 177.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (sequential, per_core) are a dead heat (<1%)

sequential (176.70 us) and per_core (177.09 us) differ by 0.22%, inside the noise, even though the wider field spreads 5.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (9.51 us) is smaller than the fastest variant's own run-to-run std-dev (43.75 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader sequential vs stability leader per_core (+0% speed for 1.9x steadier)

sequential is fastest (176.70 us, CV 24.8%); per_core gives up 0.2% median for 1.9x lower variance (CV 13.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: sequential** at 176696.0 ns median (-5.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.05x (fastest 176696.0 ns, slowest 186210.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112760367 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 206885ns | 186386ns | 182122ns | 185534ns | 251295ns | base |
| per_core | 186382ns | 177550ns | 175850ns | 177314ns | 205250ns | -9.91% |
| sequential | 196944ns | 177221ns | 175548ns | 177184ns | 237282ns | -4.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 206406ns | 181861ns | 250281ns | base | 0.318 |
| per_core | 185581ns | 175500ns | 203756ns | -10.09% | 0.353 |
| sequential | 196004ns | 175146ns | 235433ns | -5.04% | 0.334 |

## Performance model

- Peak throughput: **0.374 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.352 | 94.1% |
| per_core | 0.370 | 98.9% |
| sequential | 0.371 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 206885ns | 206885ns | base |
| per_core | 186382ns | 186382ns | -9.91% |
| sequential | 196944ns | 196944ns | -4.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 186210ns | base | --- | [183522, 203078] | --- | --- | --- | --- |
| per_core | 177093ns | -8829.2ns (-4.7%) | [-18461, -6668]ns | [176147, 181256] | YES | 0.0001 | 0.0001 | 0 |
| sequential | 176696ns | -8269.8ns (-4.4%) | [-12390, -6305]ns | [175971, 178564] | YES | 0.0001 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 182830ns | -0.5% | -2.9% |
| 1 | 2 | 186733ns | -5.0% | -5.6% |
| 1 | 3 | 187713ns | -5.8% | -5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.071 | ok |
| per_core | -0.111 | ok |
| sequential | -0.662 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 216279.8ns | 206406.3ns | 104.8% | HIGH |
| per_core | 187801.3ns | 185581.2ns | 101.2% | HIGH |
| sequential | 197663.2ns | 196003.6ns | 100.8% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 181861.2-250281.1 ns)
  181861.2 |########################################
  185282.2 |############################
  188703.2 |
  192124.2 |
  195545.2 |
  198966.2 |
  202387.2 |
  205808.2 |
  209229.1 |
  212650.1 |
  216071.1 |#####
  219492.1 |
  222913.1 |
  226334.1 |#####
  229755.1 |#####
  233176.1 |
  236597.1 |
  240018.1 |
  243439.1 |
  246860.1 |
  (1 below, 2 above range)

per_core (n=18, range 175499.9-203755.6 ns)
  175499.9 |########################################
  176912.7 |######################
  178325.5 |
  179738.3 |#####
  181151.1 |#####
  182563.8 |#####
  183976.6 |#####
  185389.4 |
  186802.2 |
  188215.0 |
  189627.8 |
  191040.5 |
  192453.3 |
  193866.1 |
  195278.9 |
  196691.7 |
  198104.4 |
  199517.2 |
  200930.0 |
  202342.8 |
  (1 below, 2 above range)

sequential (n=18, range 175145.7-235432.5 ns)
  175145.7 |########################################
  178160.0 |#############
  181174.4 |
  184188.7 |####
  187203.1 |
  190217.4 |
  193231.8 |
  196246.1 |
  199260.4 |
  202274.8 |
  205289.1 |
  208303.5 |
  211317.8 |
  214332.1 |
  217346.5 |
  220360.8 |
  223375.2 |
  226389.5 |
  229403.9 |
  232418.2 |
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=20.7% (high variance, measurements may be unstable)
- **grouped**: bridge=103.0% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: CV=22.3% (high variance, measurements may be unstable)
- **sequential**: bridge=100.6% of algo (FFI overhead may distort results)
