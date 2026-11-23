# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 33% faster than the next best (sb_plan)

sb_none (9.25 us) leads sb_plan (12.35 us) by 33%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 25.1x slower than the field

sb_value (231.96 us) is 25.1x the fastest (9.25 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_plan shows alternating (throttle bounce) (autocorr -0.54)

sb_plan's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (9.25 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 25.1x the fastest

Fastest sb_none (9.25 us) to slowest sb_value (231.96 us): 25.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 9254.2 ns median
- 2 variants significantly slower than baseline
- Spread: 25.06x (fastest 9254.2 ns, slowest 231955.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | next-frame asymmetry: clean skip vs value-swap cone vs plan band |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000002 |
| Counter frequency | 2112749428 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 11248ns | 11273ns | 10764ns | 11245ns | 11495ns | base |
| sb_plan | 14463ns | 14372ns | 13946ns | 14311ns | 14950ns | +28.58% |
| sb_value | 249223ns | 234081ns | 223396ns | 232298ns | 287524ns | +2115.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 9203ns | 8757ns | 9430ns | base | 0.111 |
| sb_plan | 12411ns | 11953ns | 12825ns | +34.85% | 0.083 |
| sb_value | 246989ns | 221314ns | 285005ns | +2583.71% | 0.004 |

## Performance model

- Peak throughput: **0.117 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.111 | 94.6% |
| sb_plan | 0.083 | 70.9% |
| sb_value | 0.004 | 3.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 11248ns | 11248ns | base |
| sb_plan | 14463ns | 14463ns | +28.58% |
| sb_value | 249223ns | 249223ns | +2115.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 9254ns | base | --- | [9083, 9362] | --- | --- | --- | --- |
| sb_plan | 12345ns | +3075.5ns (+33.2%) | [+2922, +3414]ns | [12196, 12532] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 231955ns | +222534.4ns (+2404.7%) | [+217564, +231521]ns | [226792, 239004] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 9519ns | +32.2% | +2329.4% |
| 1 | 2 | 9281ns | +28.2% | +2676.3% |
| 1 | 3 | 9364ns | +32.5% | +2350.7% |
| 2 | 1 | 8759ns | +40.8% | +2663.0% |
| 2 | 2 | 9238ns | +36.2% | +2380.3% |
| 2 | 3 | 9085ns | +33.1% | +2374.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.154 | ok |
| sb_plan | -0.543 | HIGH- (thermal bounce) |
| sb_value | -0.327 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 3852557.5ns | 9203.2ns | 41860.9% | HIGH |
| sb_plan | 3850233.1ns | 12410.8ns | 31023.3% | HIGH |
| sb_value | 3899918.2ns | 246988.6ns | 1579.0% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 8757.2-9430.0 ns)
   8757.2 |#############
   8790.8 |#############
   8824.4 |
   8858.1 |
   8891.7 |
   8925.4 |
   8959.0 |
   8992.6 |
   9026.3 |
   9059.9 |########################################
   9093.6 |
   9127.2 |
   9160.8 |
   9194.5 |
   9228.1 |########################################
   9261.8 |##########################
   9295.4 |#############
   9329.0 |#############
   9362.7 |##########################
   9396.3 |#############
  (1 below, 2 above range)

sb_plan (n=18, range 11953.5-12825.1 ns)
  11953.5 |
  11997.1 |
  12040.6 |#############
  12084.2 |#############
  12127.8 |#############
  12171.4 |#############
  12215.0 |
  12258.6 |
  12302.1 |########################################
  12345.7 |#############
  12389.3 |#############
  12432.9 |#############
  12476.5 |#############
  12520.1 |
  12563.6 |########################################
  12607.2 |
  12650.8 |
  12694.4 |#############
  12738.0 |
  12781.6 |
  (2 below, 1 above range)

sb_value (n=18, range 221313.7-285005.0 ns)
  221313.7 |
  224498.3 |########################################
  227682.8 |####################
  230867.4 |##############################
  234052.0 |##########
  237236.5 |##########
  240421.1 |####################
  243605.7 |
  246790.2 |
  249974.8 |
  253159.4 |
  256343.9 |##########
  259528.5 |
  262713.1 |
  265897.6 |
  269082.2 |
  272266.8 |
  275451.3 |
  278635.9 |
  281820.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=41608.6% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=31190.7% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1650.0% of algo (FFI overhead may distort results)
