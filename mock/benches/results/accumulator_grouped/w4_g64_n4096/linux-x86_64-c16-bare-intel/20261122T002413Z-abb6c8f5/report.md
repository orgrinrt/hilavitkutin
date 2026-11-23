# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (per_core, grouped) are a dead heat (<1%)

per_core (17.75 us) and grouped (17.75 us) differ by 0.00%, inside the noise, even though the wider field spreads 26.9%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (4.77 us) is smaller than the fastest variant's own run-to-run std-dev (7.60 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### per_core is inconsistent: worst-20% is 1.7x its best-20%

per_core's best 20% of batches run at 15.75 us but its worst 20% at 27.41 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 17754.9 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.27x (fastest 17754.9 ns, slowest 22524.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112755559 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 22108ns | 19003ns | 17094ns | 18552ns | 29950ns | base |
| per_core | 21568ns | 18972ns | 16452ns | 18603ns | 28574ns | -2.44% |
| sequential | 23950ns | 24032ns | 18560ns | 21767ns | 29918ns | +8.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 21182ns | 16556ns | 28758ns | base | 0.193 |
| per_core | 20561ns | 15752ns | 27408ns | -2.93% | 0.199 |
| sequential | 22988ns | 18088ns | 28459ns | +8.53% | 0.178 |

## Performance model

- Peak throughput: **0.260 Gops/s** (per_core; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.231 | 88.7% |
| per_core | 0.231 | 88.7% |
| sequential | 0.182 | 69.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 22108ns | 22108ns | base |
| per_core | 21568ns | 21568ns | -2.44% |
| sequential | 23950ns | 23950ns | +8.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 17755ns | base | --- | [17378, 18515] | --- | --- | --- | --- |
| per_core | 17755ns | no significant difference | [-1349, +1530]ns | [17020, 20633] | no | 0.4807 | 0.4807 | 0 |
| sequential | 22524ns | +2076.7ns (+11.7%) | [+51, +7749]ns | [18753, 25527] | YES (adj: no) | 0.1925 | 0.0963 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 17623ns | -9.4% | +46.0% |
| 1 | 2 | 16893ns | +3.7% | +28.2% |
| 1 | 3 | 18614ns | +11.7% | -1.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.447 | moderate- |
| per_core | -0.028 | ok |
| sequential | -0.003 | ok |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 21945.2ns | 21181.8ns | 103.6% | HIGH |
| per_core | 20065.0ns | 20560.9ns | 97.6% | HIGH |
| sequential | 23059.4ns | 22988.4ns | 100.3% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 16556.4-28757.6 ns)
  16556.4 |########################
  17166.4 |########################################
  17776.5 |################
  18386.6 |########################################
  18996.6 |
  19606.7 |
  20216.7 |
  20826.8 |########
  21436.9 |
  22046.9 |
  22657.0 |
  23267.0 |
  23877.1 |
  24487.2 |
  25097.2 |
  25707.3 |
  26317.4 |
  26927.4 |
  27537.5 |
  28147.5 |
  (1 below, 1 above range)

per_core (n=18, range 15751.9-27408.3 ns)
  15751.9 |##############################
  16334.7 |##########
  16917.5 |##############################
  17500.3 |####################
  18083.2 |##########
  18666.0 |
  19248.8 |
  19831.6 |##########
  20414.4 |########################################
  20997.3 |
  21580.1 |
  22162.9 |
  22745.7 |
  23328.6 |
  23911.4 |
  24494.2 |
  25077.0 |
  25659.8 |
  26242.7 |
  26825.5 |
  (1 below, 2 above range)

sequential (n=18, range 18087.5-28459.1 ns)
  18087.5 |##########
  18606.1 |########################################
  19124.7 |
  19643.3 |##########
  20161.8 |
  20680.4 |##########
  21199.0 |
  21717.6 |
  22236.2 |
  22754.7 |
  23273.3 |
  23791.9 |####################
  24310.5 |##########
  24829.1 |##########
  25347.6 |
  25866.2 |##########
  26384.8 |####################
  26903.4 |##########
  27422.0 |
  27940.5 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=63.8% (high variance, measurements may be unstable)
- **grouped**: bridge=106.2% of algo (FFI overhead may distort results)
- **per_core**: CV=36.9% (high variance, measurements may be unstable)
- **per_core**: bridge=102.0% of algo (FFI overhead may distort results)
- **sequential**: CV=22.7% (high variance, measurements may be unstable)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
