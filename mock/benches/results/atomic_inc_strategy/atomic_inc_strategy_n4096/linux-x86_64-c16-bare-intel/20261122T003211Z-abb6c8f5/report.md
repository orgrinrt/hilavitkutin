# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (36.63 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 178187% faster than the next best (inc_fetch_add)

inc_load_store (10 ns) leads inc_fetch_add (18.72 us) by 178187%, a clear separation rather than a photo finish. CV 89.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -36.61 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 3488.2x slower than the field

inc_cas_loop (36.63 us) is 3488.2x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 89.5%)

inc_load_store wins on median (10 ns) yet has the highest variance (CV 89.5%), while inc_cas_loop is the steadiest (CV 6.0%, 36.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3488.2x the fastest

Fastest inc_load_store (10 ns) to slowest inc_cas_loop (36.63 us): 3488.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.7x its best-20%

inc_load_store's best 20% of batches run at 9 ns but its worst 20% at 32 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.5 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3488.16x (fastest 10.5 ns, slowest 36625.7 ns)

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
| inc_cas_loop | 37653ns | 38513ns | 34131ns | 38513ns | 38594ns | base |
| inc_fetch_add | 20411ns | 20455ns | 18358ns | 20409ns | 22471ns | -45.79% |
| inc_load_store | 2755ns | 1887ns | 1543ns | 2127ns | 5851ns | -92.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 35802ns | 32455ns | 36682ns | base | 0.114 |
| inc_fetch_add | 18691ns | 16812ns | 20571ns | -47.79% | 0.219 |
| inc_load_store | 15ns | 9ns | 32ns | -99.96% | 268.370 |

## Performance model

- Peak throughput: **477.494 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.112 | 0.0% |
| inc_fetch_add | 0.219 | 0.0% |
| inc_load_store | 390.095 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 37653ns | 37653ns | base |
| inc_fetch_add | 20411ns | 20411ns | -45.79% |
| inc_load_store | 2755ns | 2755ns | -92.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 36626ns | base | --- | [36625, 36626] | --- | --- | --- | --- |
| inc_fetch_add | 18720ns | -18133.5ns (-49.5%) | [-19797, -16070]ns | [16822, 20557] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 10ns | -36610.6ns (-100.0%) | [-36615, -36605]ns | [10, 11] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 36627ns | -43.9% | -100.0% |
| 1 | 2 | 36626ns | -54.1% | -100.0% |
| 1 | 3 | 36627ns | -43.9% | -100.0% |
| 1 | 4 | 36625ns | -43.9% | -100.0% |
| 1 | 5 | 29959ns | -31.4% | -100.0% |
| 1 | 6 | 36625ns | -54.1% | -100.0% |
| 1 | 7 | 36626ns | -54.1% | -100.0% |
| 1 | 8 | 36627ns | -54.1% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.160 | ok |
| inc_fetch_add | 0.125 | ok |
| inc_load_store | -0.106 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 23.0ns | 35801.5ns | 0.1% |  |
| inc_fetch_add | 21.1ns | 18690.6ns | 0.1% |  |
| inc_load_store | 34.2ns | 15.3ns | 223.8% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 32455.1-36681.7 ns)
  32455.1 |
  32666.4 |
  32877.7 |
  33089.1 |
  33300.4 |
  33511.7 |
  33723.0 |
  33934.4 |
  34145.7 |
  34357.0 |
  34568.4 |
  34779.7 |
  34991.0 |
  35202.4 |
  35413.7 |
  35625.0 |
  35836.3 |
  36047.7 |
  36259.0 |
  36470.3 |########################################
  (20 below, 6 above range)

inc_fetch_add (n=160, range 16812.5-20571.1 ns)
  16812.5 |##############################
  17000.4 |
  17188.4 |
  17376.3 |
  17564.2 |
  17752.1 |
  17940.1 |
  18128.0 |
  18315.9 |
  18503.9 |
  18691.8 |
  18879.7 |
  19067.7 |
  19255.6 |
  19443.5 |
  19631.5 |
  19819.4 |
  20007.3 |
  20195.3 |
  20383.2 |########################################
  (25 below, 7 above range)

inc_load_store (n=160, range 8.6-32.2 ns)
      8.6 |###############################
      9.8 |########################################
     10.9 |
     12.1 |
     13.3 |
     14.5 |
     15.7 |
     16.8 |
     18.0 |#
     19.2 |
     20.4 |
     21.6 |
     22.7 |
     23.9 |#
     25.1 |##
     26.3 |##
     27.4 |#
     28.6 |####
     29.8 |#
     31.0 |######
  (7 below, 12 above range)

```

## Diagnostics

- **inc_load_store**: CV=61.6% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **inc_load_store**: bridge=223.8% of algo (FFI overhead may distort results)
