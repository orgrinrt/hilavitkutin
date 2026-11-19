# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 21% faster than the next best (grouped)

per_core (138.29 us) leads grouped (166.92 us) by 21%, a clear separation rather than a photo finish. CV 19.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.1x slower than the field

sequential (296.03 us) is 2.1x the fastest (138.29 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped shows alternating (throttle bounce) (autocorr -0.55)

grouped's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 1.7x its best-20%

grouped's best 20% of batches run at 125.06 us but its worst 20% at 217.39 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 138285.9 ns median (-17.2% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.14x (fastest 138285.9 ns, slowest 296026.8 ns)

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
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 176984ns | 167983ns | 125572ns | 166509ns | 218404ns | base |
| per_core | 146358ns | 138837ns | 111994ns | 137346ns | 177058ns | -17.30% |
| sequential | 307272ns | 297290ns | 268931ns | 291676ns | 349837ns | +73.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 176132ns | 125060ns | 217395ns | base | 0.372 |
| per_core | 145635ns | 111459ns | 176318ns | -17.31% | 0.450 |
| sequential | 306170ns | 267705ns | 348495ns | +73.83% | 0.214 |

## Performance model

- Peak throughput: **0.588 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.393 | 66.8% |
| per_core | 0.474 | 80.6% |
| sequential | 0.221 | 37.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 176984ns | 176984ns | base |
| per_core | 146358ns | 146358ns | -17.30% |
| sequential | 307272ns | 307272ns | +73.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 166915ns | base | --- | [158442, 188862] | --- | --- | --- | --- |
| per_core | 138286ns | no significant difference | [-53516, +5172]ns | [124615, 165225] | no | 0.2379 | 0.2379 | 0 |
| sequential | 296027ns | +145626.3ns (+87.2%) | [+99765, +168964]ns | [284028, 332367] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 164657ns | -12.2% | +75.6% |
| 1 | 2 | 176175ns | -29.4% | +62.0% |
| 1 | 3 | 169541ns | +4.3% | +105.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.548 | HIGH- (thermal bounce) |
| per_core | -0.419 | moderate- |
| sequential | -0.198 | ok |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 186351.0ns | 176132.0ns | 105.8% | HIGH |
| per_core | 143281.9ns | 145635.2ns | 98.4% | HIGH |
| sequential | 305800.0ns | 306169.7ns | 99.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 125060.1-217394.6 ns)
  125060.1 |#############
  129676.9 |
  134293.6 |
  138910.3 |
  143527.0 |#############
  148143.8 |
  152760.5 |##########################
  157377.2 |#############
  161993.9 |########################################
  166610.7 |#############
  171227.4 |
  175844.1 |#############
  180460.8 |#############
  185077.5 |#############
  189694.3 |#############
  194311.0 |
  198927.7 |#############
  203544.4 |
  208161.2 |
  212777.9 |##########################
  (1 below, 1 above range)

per_core (n=18, range 111458.7-176317.6 ns)
  111458.7 |
  114701.6 |
  117944.6 |########################################
  121187.5 |####################
  124430.5 |####################
  127673.4 |####################
  130916.4 |########################################
  134159.3 |
  137402.3 |
  140645.2 |
  143888.2 |####################
  147131.1 |
  150374.0 |
  153617.0 |
  156859.9 |####################
  160102.9 |
  163345.8 |########################################
  166588.8 |########################################
  169831.7 |
  173074.7 |
  (2 below, 3 above range)

sequential (n=18, range 267704.7-348495.0 ns)
  267704.7 |
  271744.2 |##########################
  275783.8 |#############
  279823.3 |
  283862.8 |
  287902.3 |########################################
  291941.8 |#############
  295981.3 |##########################
  300020.8 |#############
  304060.4 |
  308099.9 |
  312139.4 |
  316178.9 |
  320218.4 |
  324257.9 |
  328297.4 |
  332336.9 |##########################
  336376.5 |
  340416.0 |
  344455.5 |#############
  (2 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=22.4% (high variance, measurements may be unstable)
- **grouped**: bridge=111.6% of algo (FFI overhead may distort results)
- **per_core**: bridge=93.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
