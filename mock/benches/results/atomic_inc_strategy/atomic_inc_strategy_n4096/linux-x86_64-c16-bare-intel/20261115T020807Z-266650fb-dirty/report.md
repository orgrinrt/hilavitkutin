# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (29.96 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 161645% faster than the next best (inc_fetch_add)

inc_load_store (10 ns) leads inc_fetch_add (16.82 us) by 161645%, a clear separation rather than a photo finish. CV 94.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -29.95 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 2880.8x slower than the field

inc_cas_loop (29.96 us) is 2880.8x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 94.5%)

inc_load_store wins on median (10 ns) yet has the highest variance (CV 94.5%), while inc_cas_loop is the steadiest (CV 11.5%, 29.96 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### inc_fetch_add shows alternating (throttle bounce) (autocorr -0.63)

inc_fetch_add's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 2880.8x the fastest

Fastest inc_load_store (10 ns) to slowest inc_cas_loop (29.96 us): 2880.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.9x its best-20%

inc_load_store's best 20% of batches run at 9 ns but its worst 20% at 33 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 2880.83x (fastest 10.4 ns, slowest 29960.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic increment strategy: fetch_add vs CAS-loop vs load+store |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x18c492ea6f3ba570 |
| Counter frequency | 2112741386 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 34082ns | 31503ns | 31218ns | 33552ns | 38537ns | base |
| inc_fetch_add | 20284ns | 18362ns | 18353ns | 19591ns | 24293ns | -40.49% |
| inc_load_store | 2728ns | 1882ns | 1541ns | 2044ns | 5969ns | -92.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 32232ns | 28800ns | 36644ns | base | 0.127 |
| inc_fetch_add | 18478ns | 16812ns | 21742ns | -42.67% | 0.222 |
| inc_load_store | 15ns | 9ns | 33ns | -99.95% | 269.829 |

## Performance model

- Peak throughput: **480.469 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.137 | 0.0% |
| inc_fetch_add | 0.243 | 0.1% |
| inc_load_store | 393.846 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 34082ns | 34082ns | base |
| inc_fetch_add | 20284ns | 20284ns | -40.49% |
| inc_load_store | 2728ns | 2728ns | -92.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 29961ns | base | --- | [29960, 29968] | --- | --- | --- | --- |
| inc_fetch_add | 16822ns | -13146.3ns (-43.9%) | [-13149, -13139]ns | [16820, 16824] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 10ns | -29950.5ns (-100.0%) | [-29957, -29949]ns | [9, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 36624ns | -54.1% | -100.0% |
| 1 | 2 | 29958ns | -31.4% | -100.0% |
| 1 | 3 | 29958ns | -43.9% | -100.0% |
| 1 | 4 | 29959ns | -43.9% | -100.0% |
| 1 | 5 | 36625ns | -43.8% | -100.0% |
| 1 | 6 | 36626ns | -54.1% | -100.0% |
| 1 | 7 | 28112ns | -20.2% | -99.9% |
| 1 | 8 | 29958ns | -43.9% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | 0.010 | ok |
| inc_fetch_add | -0.634 | HIGH- (thermal bounce) |
| inc_load_store | -0.293 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 23.2ns | 32232.2ns | 0.1% |  |
| inc_fetch_add | 22.3ns | 18478.1ns | 0.1% |  |
| inc_load_store | 33.9ns | 15.2ns | 223.1% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 28800.2-36643.6 ns)
  28800.2 |
  29192.4 |
  29584.5 |########################################
  29976.7 |#
  30368.9 |
  30761.0 |
  31153.2 |
  31545.4 |
  31937.5 |
  32329.7 |
  32721.9 |
  33114.1 |
  33506.2 |
  33898.4 |
  34290.6 |
  34682.7 |
  35074.9 |
  35467.1 |
  35859.2 |
  36251.4 |##############################
  (20 below, 2 above range)

inc_fetch_add (n=160, range 16812.3-21741.6 ns)
  16812.3 |########################################
  17058.8 |
  17305.2 |
  17551.7 |
  17798.2 |
  18044.6 |
  18291.1 |
  18537.5 |
  18784.0 |
  19030.5 |
  19276.9 |
  19523.4 |
  19769.9 |
  20016.3 |
  20262.8 |
  20509.3 |#####################
  20755.7 |
  21002.2 |
  21248.6 |
  21495.1 |
  (22 below, 20 above range)

inc_load_store (n=160, range 8.5-32.9 ns)
      8.5 |########################################
      9.7 |######################
     11.0 |
     12.2 |
     13.4 |
     14.6 |
     15.9 |##############
     17.1 |
     18.3 |
     19.5 |
     20.7 |
     22.0 |
     23.2 |
     24.4 |###
     25.6 |#
     26.8 |##
     28.1 |##
     29.3 |##
     30.5 |##
     31.7 |
  (24 below, 16 above range)

```

## Diagnostics

- **inc_load_store**: CV=64.7% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **inc_load_store**: bridge=225.0% of algo (FFI overhead may distort results)
