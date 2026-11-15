# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (popcount_handrolled) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline popcount_handrolled has the worst median (42 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest popcount_std at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### popcount_std dominates: 867% faster than the next best (popcount_swar)

popcount_std (4 ns) leads popcount_swar (41 ns) by 867%, a clear separation rather than a photo finish. CV 29.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 90% (significant)

popcount_std is -38 ns (90%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_handrolled is an outlier: 10.0x slower than the field

popcount_handrolled (42 ns) is 10.0x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_std is fastest but the noisiest (CV 29.5%)

popcount_std wins on median (4 ns) yet has the highest variance (CV 29.5%), while popcount_handrolled is the steadiest (CV 13.9%, 42 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### popcount_swar shows alternating (throttle bounce) (autocorr -0.88)

popcount_swar's per-pass series has lag-1 autocorrelation -0.88, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 10.0x the fastest

Fastest popcount_std (4 ns) to slowest popcount_handrolled (42 ns): 10.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_std is inconsistent: worst-20% is 2.4x its best-20%

popcount_std's best 20% of batches run at 3 ns but its worst 20% at 6 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: popcount_std** at 4.2 ns median (-90.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 10.02x (fastest 4.2 ns, slowest 42.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 2284ns | 2260ns | 2102ns | 2291ns | 2442ns | base |
| popcount_std | 2285ns | 2248ns | 2102ns | 2304ns | 2410ns | +0.06% |
| popcount_swar | 2205ns | 2131ns | 2098ns | 2166ns | 2429ns | -3.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 42ns | 35ns | 51ns | base | 6.028 |
| popcount_std | 4ns | 3ns | 6ns | -90.03% | 60.449 |
| popcount_swar | 42ns | 34ns | 51ns | -2.19% | 6.164 |

## Performance model

- Peak throughput: **102.017 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 6.081 | 6.0% |
| popcount_std | 60.952 | 59.7% |
| popcount_swar | 6.305 | 6.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 2284ns | 2284ns | base |
| popcount_std | 2285ns | 2285ns | +0.06% |
| popcount_swar | 2205ns | 2205ns | -3.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 42ns | base | --- | [41, 43] | --- | --- | --- | --- |
| popcount_std | 4ns | -37.9ns (-90.0%) | [-39, -37]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 41ns | no significant difference | [-2, +0]ns | [40, 42] | no | 0.0635 | 0.0635 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 43ns | -89.7% | -5.4% |
| 1 | 2 | 43ns | -92.3% | -3.4% |
| 1 | 3 | 41ns | -89.8% | -6.1% |
| 1 | 4 | 42ns | -88.1% | +2.4% |
| 1 | 5 | 42ns | -88.7% | -12.7% |
| 1 | 6 | 40ns | -90.8% | +5.7% |
| 1 | 7 | 45ns | -90.7% | -17.0% |
| 1 | 8 | 37ns | -87.5% | +14.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.559 | HIGH- (thermal bounce) |
| popcount_std | -0.003 | ok |
| popcount_swar | -0.882 | HIGH- (thermal bounce) |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.6ns | 42.5ns | 8.4% | HIGH |
| popcount_std | 3.4ns | 4.2ns | 79.9% | HIGH |
| popcount_swar | 3.5ns | 41.5ns | 8.5% | HIGH |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 34.6-51.3 ns)
     34.6 |#################
     35.5 |#################
     36.3 |#################
     37.1 |##############
     38.0 |##############
     38.8 |############################
     39.6 |###############################
     40.5 |#########################
     41.3 |########################################
     42.1 |#########################
     43.0 |###########
     43.8 |############################
     44.6 |####################
     45.5 |######################
     46.3 |##############
     47.1 |########
     47.9 |###########
     48.8 |########
     49.6 |#################
     50.4 |#################
  (14 below, 9 above range)

popcount_std (n=160, range 2.5-6.0 ns)
      2.5 |
      2.7 |
      2.9 |###########
      3.0 |
      3.2 |###################
      3.4 |
      3.6 |########################################
      3.7 |
      3.9 |
      4.1 |###################################
      4.2 |
      4.4 |
      4.6 |####################
      4.8 |
      4.9 |#############################
      5.1 |
      5.3 |#################
      5.5 |
      5.6 |################
      5.8 |
  (20 below, 11 above range)

popcount_swar (n=160, range 34.2-51.3 ns)
     34.2 |######################
     35.1 |#########################
     35.9 |##################################
     36.8 |#################
     37.6 |##############
     38.5 |###############################
     39.3 |########################################
     40.2 |####################
     41.1 |####################
     41.9 |############################
     42.8 |######################
     43.6 |##############
     44.5 |#################
     45.3 |######################
     46.2 |##############
     47.0 |##
     47.9 |#################
     48.8 |#####
     49.6 |#####
     50.5 |###########
  (14 below, 10 above range)

```

## Diagnostics

- **popcount_std**: CV=29.3% (high variance, measurements may be unstable)
- **popcount_std**: bridge=78.6% of algo (FFI overhead may distort results)
