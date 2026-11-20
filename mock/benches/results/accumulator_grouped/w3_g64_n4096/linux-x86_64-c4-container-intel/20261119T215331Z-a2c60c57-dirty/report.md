# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 46% faster than the next best (grouped)

sequential (27.93 us) leads grouped (40.75 us) by 46%, a clear separation rather than a photo finish. CV 13.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 38% (significant)

sequential is -15.35 us (38%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### per_core shows alternating (throttle bounce) (autocorr -0.65)

per_core's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### per_core is inconsistent: worst-20% is 6.2x its best-20%

per_core's best 20% of batches run at 35.12 us but its worst 20% at 216.60 us (6.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 27929.4 ns median (-31.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.70x (fastest 27929.4 ns, slowest 47543.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 3, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 42379ns | 41191ns | 36907ns | 40566ns | 47834ns | base |
| per_core | 100228ns | 47999ns | 35756ns | 43720ns | 217227ns | +136.51% |
| sequential | 28743ns | 28364ns | 23543ns | 27592ns | 33070ns | -32.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 41793ns | 36465ns | 47178ns | base | 0.098 |
| per_core | 99644ns | 35117ns | 216602ns | +138.42% | 0.041 |
| sequential | 28166ns | 22983ns | 32380ns | -32.61% | 0.145 |

## Performance model

- Peak throughput: **0.178 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.101 | 56.4% |
| per_core | 0.086 | 48.3% |
| sequential | 0.147 | 82.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 42379ns | 42379ns | base |
| per_core | 100228ns | 100228ns | +136.51% |
| sequential | 28743ns | 28743ns | -32.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 40747ns | base | --- | [38112, 44066] | --- | --- | --- | --- |
| per_core | 47544ns | no significant difference | [-1693, +35584]ns | [37092, 82676] | no | 0.4807 | 0.4807 | 0 |
| sequential | 27929ns | -15351.7ns (-37.7%) | [-17195, -9770]ns | [25741, 30138] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 37190ns | +8.2% | -16.1% |
| 1 | 2 | 46376ns | +7.2% | -40.6% |
| 1 | 3 | 39745ns | +5.5% | -40.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.618 | HIGH- (thermal bounce) |
| per_core | -0.648 | HIGH- (thermal bounce) |
| sequential | -0.000 | ok |

**Consistency summary:**

- **per_core**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 41605.7ns | 41793.2ns | 99.6% | HIGH |
| per_core | 48908.0ns | 99644.4ns | 49.1% | HIGH |
| sequential | 29327.9ns | 28166.4ns | 104.1% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 36465.1-47177.6 ns)
  36465.1 |########################################
  37000.7 |
  37536.3 |########################################
  38072.0 |####################
  38607.6 |####################
  39143.2 |####################
  39678.8 |
  40214.5 |####################
  40750.1 |####################
  41285.7 |
  41821.3 |
  42357.0 |
  42892.6 |########################################
  43428.2 |####################
  43963.8 |
  44499.4 |####################
  45035.1 |####################
  45570.7 |
  46106.3 |
  46641.9 |
  (1 below, 3 above range)

per_core (n=18, range 35117.3-216601.9 ns)
  35117.3 |########################################
  44191.5 |##########################
  53265.8 |
  62340.0 |
  71414.2 |######
  80488.4 |######
  89562.7 |
  98636.9 |
  107711.1 |
  116785.3 |
  125859.6 |
  134933.8 |
  144008.0 |
  153082.3 |
  162156.5 |
  171230.7 |
  180304.9 |
  189379.2 |
  198453.4 |
  207527.6 |
  (2 below, 4 above range)

sequential (n=18, range 22983.2-32380.3 ns)
  22983.2 |####################
  23453.1 |########################################
  23922.9 |
  24392.8 |
  24862.6 |####################
  25332.5 |
  25802.3 |
  26272.2 |####################
  26742.0 |####################
  27211.9 |####################
  27681.7 |########################################
  28151.6 |####################
  28621.5 |
  29091.3 |
  29561.2 |####################
  30031.0 |########################################
  30500.9 |
  30970.7 |
  31440.6 |
  31910.4 |####################
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=98.8% of algo (FFI overhead may distort results)
- **per_core**: CV=103.9% (high variance, measurements may be unstable)
- **per_core**: worst_20/best_20 = 6.2x (possible bimodal distribution)
- **per_core**: bridge=89.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.2% of algo (FFI overhead may distort results)
