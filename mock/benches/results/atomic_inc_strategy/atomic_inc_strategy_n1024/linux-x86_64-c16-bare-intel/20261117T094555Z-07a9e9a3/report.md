# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (7.51 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 9 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 50909% faster than the next best (inc_fetch_add)

inc_load_store (9 ns) leads inc_fetch_add (4.39 us) by 50909%, a clear separation rather than a photo finish. CV 106.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -7.50 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 872.9x slower than the field

inc_cas_loop (7.51 us) is 872.9x the fastest (9 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 106.0%)

inc_load_store wins on median (9 ns) yet has the highest variance (CV 106.0%), while inc_cas_loop is the steadiest (CV 11.0%, 7.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 872.9x the fastest

Fastest inc_load_store (9 ns) to slowest inc_cas_loop (7.51 us): 872.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.5x its best-20%

inc_load_store's best 20% of batches run at 8 ns but its worst 20% at 30 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 8.6 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 872.86x (fastest 8.6 ns, slowest 7506.6 ns)

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
| Counter frequency | 2112789647 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 9603ns | 9050ns | 9043ns | 9264ns | 11180ns | base |
| inc_fetch_add | 6499ns | 6003ns | 5752ns | 6304ns | 7832ns | -32.32% |
| inc_load_store | 2440ns | 1563ns | 1549ns | 1702ns | 5542ns | -74.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 7967ns | 7499ns | 9298ns | base | 0.129 |
| inc_fetch_add | 4753ns | 4211ns | 5727ns | -40.35% | 0.215 |
| inc_load_store | 13ns | 8ns | 30ns | -99.83% | 77.247 |

## Performance model

- Peak throughput: **121.408 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.136 | 0.1% |
| inc_fetch_add | 0.233 | 0.2% |
| inc_load_store | 119.070 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 9603ns | 9603ns | base |
| inc_fetch_add | 6499ns | 6499ns | -32.32% |
| inc_load_store | 2440ns | 2440ns | -74.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 7507ns | base | --- | [7506, 7508] | --- | --- | --- | --- |
| inc_fetch_add | 4387ns | -3285.0ns (-43.8%) | [-3294, -3272]ns | [4223, 4877] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 9ns | -7497.5ns (-99.9%) | [-7498, -7497]ns | [9, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 9173ns | -54.1% | -99.9% |
| 1 | 2 | 9173ns | -43.8% | -99.9% |
| 1 | 3 | 7506ns | -31.3% | -99.9% |
| 1 | 4 | 7506ns | -31.3% | -99.9% |
| 1 | 5 | 7506ns | -40.7% | -99.9% |
| 1 | 6 | 7506ns | -43.1% | -99.9% |
| 1 | 7 | 7507ns | -43.9% | -99.9% |
| 1 | 8 | 7506ns | -43.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | 0.458 | moderate+ |
| inc_fetch_add | 0.426 | moderate+ |
| inc_load_store | -0.239 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 20.6ns | 7967.2ns | 0.3% |  |
| inc_fetch_add | 22.0ns | 4752.7ns | 0.5% |  |
| inc_load_store | 30.3ns | 13.3ns | 228.7% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 7498.6-9298.4 ns)
   7498.6 |########################################
   7588.6 |##
   7678.6 |
   7768.6 |
   7858.6 |
   7948.6 |
   8038.6 |
   8128.6 |
   8218.6 |
   8308.5 |
   8398.5 |
   8488.5 |
   8578.5 |
   8668.5 |
   8758.5 |
   8848.5 |
   8938.5 |
   9028.4 |
   9118.4 |##################
   9208.4 |
  (24 below, 1 above range)

inc_fetch_add (n=160, range 4211.4-5726.5 ns)
   4211.4 |#######################################
   4287.2 |###
   4363.0 |####
   4438.7 |##
   4514.5 |
   4590.2 |
   4666.0 |#
   4741.7 |
   4817.5 |
   4893.2 |
   4969.0 |
   5044.7 |
   5120.5 |########################################
   5196.2 |#
   5272.0 |
   5347.8 |
   5423.5 |
   5499.3 |
   5575.0 |
   5650.8 |
  (17 below, 4 above range)

inc_load_store (n=160, range 8.4-29.9 ns)
      8.4 |########################################
      9.5 |########################
     10.6 |
     11.7 |
     12.7 |
     13.8 |
     14.9 |
     15.9 |
     17.0 |
     18.1 |
     19.2 |
     20.2 |#
     21.3 |
     22.4 |
     23.5 |#
     24.5 |#
     25.6 |
     26.7 |#
     27.8 |
     28.8 |
  (21 below, 18 above range)

```

## Diagnostics

- **inc_load_store**: CV=68.8% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **inc_load_store**: bridge=226.7% of algo (FFI overhead may distort results)
