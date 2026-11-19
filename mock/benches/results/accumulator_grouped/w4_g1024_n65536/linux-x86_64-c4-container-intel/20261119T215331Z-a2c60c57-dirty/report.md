# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.2x slower than the field

sequential (289.81 us) is 2.2x the fastest (131.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped is fastest but the noisiest (CV 36.6%)

grouped wins on median (131.57 us) yet has the highest variance (CV 36.6%), while sequential is the steadiest (CV 10.8%, 289.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (grouped)

The baseline grouped is the fastest (131.57 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### grouped is inconsistent: worst-20% is 1.7x its best-20%

grouped's best 20% of batches run at 107.85 us but its worst 20% at 179.72 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (grouped) is the fastest** at 131571.5 ns median
- 1 variant significantly slower than baseline
- Spread: 2.20x (fastest 131571.5 ns, slowest 289813.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 1024 |
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
| grouped | 142729ns | 131994ns | 108314ns | 129088ns | 180399ns | base |
| per_core | 145232ns | 144113ns | 111471ns | 138725ns | 171874ns | +1.75% |
| sequential | 295326ns | 290516ns | 260708ns | 281280ns | 333703ns | +106.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 142063ns | 107854ns | 179720ns | base | 0.461 |
| per_core | 144617ns | 110778ns | 171187ns | +1.80% | 0.453 |
| sequential | 294457ns | 259939ns | 332726ns | +107.27% | 0.223 |

## Performance model

- Peak throughput: **0.608 Gops/s** (grouped; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.498 | 82.0% |
| per_core | 0.456 | 75.1% |
| sequential | 0.226 | 37.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 142729ns | 142729ns | base |
| per_core | 145232ns | 145232ns | +1.75% |
| sequential | 295326ns | 295326ns | +106.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 131572ns | base | --- | [122074, 142249] | --- | --- | --- | --- |
| per_core | 143631ns | no significant difference | [-5803, +32206]ns | [129317, 163205] | no | 0.2379 | 0.2379 | 0 |
| sequential | 289813ns | +156235.5ns (+118.7%) | [+138580, +184128]ns | [265115, 317197] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 115206ns | +28.5% | +128.0% |
| 1 | 2 | 133202ns | +23.0% | +110.5% |
| 1 | 3 | 140353ns | -10.2% | +137.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.039 | ok |
| per_core | -0.443 | moderate- |
| sequential | -0.051 | ok |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 142552.0ns | 142063.3ns | 100.3% | HIGH |
| per_core | 145171.3ns | 144616.8ns | 100.4% | HIGH |
| sequential | 293008.2ns | 294457.2ns | 99.5% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 107854.1-179720.0 ns)
  107854.1 |##########################
  111447.4 |#############
  115040.7 |
  118634.0 |#############
  122227.3 |#############
  125820.6 |##########################
  129413.9 |##########################
  133007.2 |
  136600.5 |##########################
  140193.8 |########################################
  143787.1 |
  147380.4 |
  150973.7 |#############
  154567.0 |
  158160.3 |
  161753.6 |
  165346.9 |
  168940.1 |#############
  172533.4 |
  176126.7 |
  (1 below, 1 above range)

per_core (n=18, range 110777.9-171186.7 ns)
  110777.9 |
  113798.3 |####################
  116818.7 |####################
  119839.2 |####################
  122859.6 |####################
  125880.1 |
  128900.5 |####################
  131921.0 |
  134941.4 |####################
  137961.8 |####################
  140982.3 |####################
  144002.7 |########################################
  147023.2 |
  150043.6 |
  153064.0 |####################
  156084.5 |####################
  159104.9 |
  162125.4 |
  165145.8 |
  168166.3 |########################################
  (1 below, 3 above range)

sequential (n=18, range 259938.5-332726.5 ns)
  259938.5 |########################################
  263577.9 |##########
  267217.3 |##########
  270856.7 |
  274496.1 |##########
  278135.5 |
  281774.9 |
  285414.3 |##########
  289053.7 |
  292693.1 |##########
  296332.5 |
  299971.9 |##########
  303611.3 |##########
  307250.7 |
  310890.1 |##########
  314529.5 |
  318168.9 |##########
  321808.3 |
  325447.7 |
  329087.1 |
  (1 below, 4 above range)

```

## Diagnostics

- **grouped**: CV=33.9% (high variance, measurements may be unstable)
- **grouped**: bridge=106.3% of algo (FFI overhead may distort results)
- **per_core**: bridge=94.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.8% of algo (FFI overhead may distort results)
