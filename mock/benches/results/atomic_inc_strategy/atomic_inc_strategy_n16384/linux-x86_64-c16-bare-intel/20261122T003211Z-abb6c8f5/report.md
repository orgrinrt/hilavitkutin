# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (134.14 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 640193% faster than the next best (inc_fetch_add)

inc_load_store (10 ns) leads inc_fetch_add (67.23 us) by 640193%, a clear separation rather than a photo finish. CV 90.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -134.13 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 12775.4x slower than the field

inc_cas_loop (134.14 us) is 12775.4x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 90.6%)

inc_load_store wins on median (10 ns) yet has the highest variance (CV 90.6%), while inc_fetch_add is the steadiest (CV 9.6%, 67.23 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### inc_cas_loop shows alternating (throttle bounce) (autocorr -0.63)

inc_cas_loop's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 12775.4x the fastest

Fastest inc_load_store (10 ns) to slowest inc_cas_loop (134.14 us): 12775.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.7x its best-20%

inc_load_store's best 20% of batches run at 9 ns but its worst 20% at 32 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.5 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 12775.36x (fastest 10.5 ns, slowest 134141.3 ns)

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
| Counter frequency | 2112744494 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 135023ns | 135878ns | 121407ns | 134975ns | 148786ns | base |
| inc_fetch_add | 72620ns | 68779ns | 68770ns | 70081ns | 84087ns | -46.22% |
| inc_load_store | 2575ns | 1888ns | 1543ns | 1837ns | 5818ns | -98.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 133285ns | 119848ns | 146861ns | base | 0.123 |
| inc_fetch_add | 70980ns | 67221ns | 82188ns | -46.75% | 0.231 |
| inc_load_store | 14ns | 9ns | 32ns | -99.99% | 1146.937 |

## Performance model

- Peak throughput: **1903.042 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.122 | 0.0% |
| inc_fetch_add | 0.244 | 0.0% |
| inc_load_store | 1560.381 | 82.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 135023ns | 135023ns | base |
| inc_fetch_add | 72620ns | 72620ns | -46.22% |
| inc_load_store | 2575ns | 2575ns | -98.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 134141ns | base | --- | [119883, 146496] | --- | --- | --- | --- |
| inc_fetch_add | 67231ns | -59371.4ns (-44.3%) | [-64318, -52656]ns | [67229, 67233] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 10ns | -134125.8ns (-100.0%) | [-146468, -119864]ns | [10, 11] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 119858ns | -43.9% | -100.0% |
| 1 | 2 | 146496ns | -43.9% | -100.0% |
| 1 | 3 | 146512ns | -54.1% | -100.0% |
| 1 | 4 | 119856ns | -43.9% | -100.0% |
| 1 | 5 | 146501ns | -43.9% | -100.0% |
| 1 | 6 | 119855ns | -43.9% | -100.0% |
| 1 | 7 | 146507ns | -54.1% | -100.0% |
| 1 | 8 | 119856ns | -43.9% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.625 | HIGH- (thermal bounce) |
| inc_fetch_add | -0.375 | moderate- |
| inc_load_store | -0.126 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 22.4ns | 133284.8ns | 0.0% |  |
| inc_fetch_add | 20.6ns | 70980.4ns | 0.0% |  |
| inc_load_store | 32.3ns | 14.3ns | 225.8% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 119848.4-146860.5 ns)
  119848.4 |####################################
  121199.0 |
  122549.6 |
  123900.2 |
  125250.8 |
  126601.4 |
  127952.0 |
  129302.6 |
  130653.2 |
  132003.8 |
  133354.4 |
  134705.0 |
  136055.6 |
  137406.3 |
  138756.9 |
  140107.5 |
  141458.1 |
  142808.7 |
  144159.3 |
  145509.9 |########################################
  (13 below, 7 above range)

inc_fetch_add (n=160, range 67220.9-82188.4 ns)
  67220.9 |########################################
  67969.3 |
  68717.7 |
  69466.0 |
  70214.4 |
  70962.8 |
  71711.2 |
  72459.5 |
  73207.9 |
  73956.3 |
  74704.7 |
  75453.0 |
  76201.4 |
  76949.8 |
  77698.2 |
  78446.5 |
  79194.9 |
  79943.3 |
  80691.7 |
  81440.0 |#############
  (17 below, 6 above range)

inc_load_store (n=160, range 8.6-32.0 ns)
      8.6 |#########
      9.8 |########################################
     11.0 |
     12.1 |
     13.3 |
     14.5 |
     15.6 |
     16.8 |
     18.0 |
     19.1 |
     20.3 |
     21.5 |
     22.7 |
     23.8 |
     25.0 |
     26.2 |
     27.3 |
     28.5 |#
     29.7 |
     30.9 |##
  (27 below, 16 above range)

```

## Diagnostics

- **inc_load_store**: CV=66.6% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **inc_load_store**: bridge=226.7% of algo (FFI overhead may distort results)
