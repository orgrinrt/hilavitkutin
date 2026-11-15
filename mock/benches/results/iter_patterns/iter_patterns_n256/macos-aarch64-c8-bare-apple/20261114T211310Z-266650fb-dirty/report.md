# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (47 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 1 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 2038% faster than the next best (iter_seq)

iter_strided (1 ns) leads iter_seq (17 ns) by 2038%, a clear separation rather than a photo finish. CV 81.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 98% (significant)

iter_strided is -46 ns (98%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 58.4x slower than the field

iter_gather (47 ns) is 58.4x the fastest (1 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 81.4%)

iter_strided wins on median (1 ns) yet has the highest variance (CV 81.4%), while iter_gather is the steadiest (CV 10.4%, 47 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_seq shows alternating (throttle bounce) (autocorr -0.57)

iter_seq's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 58.4x the fastest

Fastest iter_strided (1 ns) to slowest iter_gather (47 ns): 58.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 7.5x its best-20%

iter_strided's best 20% of batches run at 0 ns but its worst 20% at 2 ns (7.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 0.8 ns median (-98.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 58.38x (fastest 0.8 ns, slowest 46.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2328ns | 2368ns | 2124ns | 2346ns | 2477ns | base |
| iter_seq | 2273ns | 2249ns | 2098ns | 2276ns | 2439ns | -2.35% |
| iter_strided | 2312ns | 2401ns | 2140ns | 2334ns | 2418ns | -0.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 47ns | 40ns | 53ns | base | 5.486 |
| iter_seq | 18ns | 13ns | 24ns | -61.70% | 14.323 |
| iter_strided | 1ns | 0ns | 2ns | -97.75% | 244.246 |

## Performance model

- Peak throughput: **930.909 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 5.482 | 0.6% |
| iter_seq | 14.971 | 1.6% |
| iter_strided | 320.000 | 34.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2328ns | 2328ns | base |
| iter_seq | 2273ns | 2273ns | -2.35% |
| iter_strided | 2312ns | 2312ns | -0.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 47ns | base | --- | [46, 48] | --- | --- | --- | --- |
| iter_seq | 17ns | -29.6ns (-63.4%) | [-30, -28]ns | [17, 18] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 1ns | -45.8ns (-98.1%) | [-47, -45]ns | [1, 1] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 45ns | -62.6% | -98.2% |
| 1 | 2 | 49ns | -63.8% | -98.4% |
| 1 | 3 | 43ns | -60.0% | -97.2% |
| 1 | 4 | 48ns | -66.4% | -97.9% |
| 1 | 5 | 51ns | -63.8% | -98.4% |
| 1 | 6 | 49ns | -67.9% | -98.0% |
| 1 | 7 | 45ns | -57.7% | -98.2% |
| 1 | 8 | 46ns | -61.7% | -97.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.149 | ok |
| iter_seq | -0.574 | HIGH- (thermal bounce) |
| iter_strided | -0.230 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.7ns | 46.7ns | 5.9% | HIGH |
| iter_seq | 3.0ns | 17.9ns | 16.6% | HIGH |
| iter_strided | 2.7ns | 1.0ns | 260.6% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 39.8-53.4 ns)
     39.8 |################
     40.5 |########
     41.2 |#####
     41.9 |##########################
     42.6 |##################
     43.2 |#############
     43.9 |#####################
     44.6 |##########################
     45.3 |##################
     45.9 |################
     46.6 |########################################
     47.3 |################
     48.0 |##########
     48.7 |################
     49.3 |########################
     50.0 |################
     50.7 |#############################
     51.4 |#####
     52.1 |################
     52.7 |########
  (13 below, 15 above range)

iter_seq (n=160, range 13.4-23.9 ns)
     13.4 |####
     13.9 |################
     14.5 |#######
     15.0 |########################################
     15.5 |#######################
     16.0 |##############
     16.6 |################################
     17.1 |############################
     17.6 |##############
     18.1 |###################################
     18.6 |##############
     19.2 |##############################
     19.7 |##############
     20.2 |####
     20.7 |##############
     21.3 |##############
     21.8 |#######
     22.3 |####
     22.8 |#######
     23.4 |#######
  (15 below, 3 above range)

iter_strided (n=160, range 0.3-2.1 ns)
      0.3 |
      0.4 |###############################
      0.5 |
      0.5 |
      0.6 |
      0.7 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |
      1.2 |#######################################
      1.3 |
      1.3 |
      1.4 |
      1.5 |
      1.6 |
      1.7 |##################
      1.8 |
      1.9 |
      2.0 |
  (10 below, 18 above range)

```

## Diagnostics

- **iter_seq**: CV=34.8% (high variance, measurements may be unstable)
- **iter_seq**: bridge=17.0% of algo (FFI overhead may distort results)
- **iter_strided**: CV=62.1% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 7.5x (possible bimodal distribution)
- **iter_strided**: bridge=312.5% of algo (FFI overhead may distort results)
