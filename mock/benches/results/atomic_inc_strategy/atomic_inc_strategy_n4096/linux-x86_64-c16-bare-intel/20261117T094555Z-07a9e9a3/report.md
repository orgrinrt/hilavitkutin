# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (30.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 9 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 193261% faster than the next best (inc_fetch_add)

inc_load_store (9 ns) leads inc_fetch_add (16.82 us) by 193261%, a clear separation rather than a photo finish. CV 108.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -30.56 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 3513.4x slower than the field

inc_cas_loop (30.57 us) is 3513.4x the fastest (9 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 108.3%)

inc_load_store wins on median (9 ns) yet has the highest variance (CV 108.3%), while inc_cas_loop is the steadiest (CV 25.4%, 30.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3513.4x the fastest

Fastest inc_load_store (9 ns) to slowest inc_cas_loop (30.57 us): 3513.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.7x its best-20%

inc_load_store's best 20% of batches run at 9 ns but its worst 20% at 31 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 8.7 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3513.44x (fastest 8.7 ns, slowest 30567.0 ns)

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
| inc_cas_loop | 35547ns | 32179ns | 31501ns | 34573ns | 42512ns | base |
| inc_fetch_add | 21188ns | 18369ns | 18358ns | 18916ns | 30832ns | -40.39% |
| inc_load_store | 2626ns | 1555ns | 1548ns | 1876ns | 5953ns | -92.61% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 33790ns | 29956ns | 40411ns | base | 0.121 |
| inc_fetch_add | 19400ns | 16812ns | 28245ns | -42.59% | 0.211 |
| inc_load_store | 14ns | 9ns | 31ns | -99.96% | 287.174 |

## Performance model

- Peak throughput: **478.016 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.134 | 0.0% |
| inc_fetch_add | 0.243 | 0.1% |
| inc_load_store | 470.805 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 35547ns | 35547ns | base |
| inc_fetch_add | 21188ns | 21188ns | -40.39% |
| inc_load_store | 2626ns | 2626ns | -92.61% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 30567ns | base | --- | [29984, 35666] | --- | --- | --- | --- |
| inc_fetch_add | 16822ns | -13158.7ns (-43.0%) | [-18442, -13141]ns | [16822, 16829] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 9ns | -30555.2ns (-100.0%) | [-35657, -29975]ns | [9, 9] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 36625ns | -54.1% | -100.0% |
| 1 | 2 | 29959ns | -31.4% | -100.0% |
| 1 | 3 | 29974ns | -43.9% | -100.0% |
| 1 | 4 | 29970ns | -31.4% | -100.0% |
| 1 | 5 | 35180ns | -52.1% | -100.0% |
| 1 | 6 | 36624ns | -54.1% | -100.0% |
| 1 | 7 | 29959ns | -43.9% | -100.0% |
| 1 | 8 | 36627ns | -54.1% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.157 | ok |
| inc_fetch_add | -0.374 | moderate- |
| inc_load_store | -0.115 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 21.6ns | 33789.7ns | 0.1% |  |
| inc_fetch_add | 22.2ns | 19400.3ns | 0.1% |  |
| inc_load_store | 32.4ns | 14.3ns | 227.4% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 29956.1-40410.7 ns)
  29956.1 |########################################
  30478.8 |##
  31001.5 |#
  31524.3 |
  32047.0 |
  32569.7 |
  33092.5 |
  33615.2 |
  34137.9 |
  34660.6 |
  35183.4 |####
  35706.1 |#
  36228.8 |#################################
  36751.6 |
  37274.3 |#
  37797.0 |
  38319.8 |
  38842.5 |
  39365.2 |
  39888.0 |
  (8 below, 2 above range)

inc_fetch_add (n=160, range 16811.8-28245.2 ns)
  16811.8 |########################################
  17383.5 |
  17955.1 |
  18526.8 |
  19098.5 |
  19670.1 |
  20241.8 |###############
  20813.5 |
  21385.2 |
  21956.8 |
  22528.5 |
  23100.2 |
  23671.8 |
  24243.5 |
  24815.2 |
  25386.9 |
  25958.5 |
  26530.2 |
  27101.9 |
  27673.5 |
  (13 below, 3 above range)

inc_load_store (n=160, range 8.6-31.3 ns)
      8.6 |########################################
      9.7 |######
     10.8 |
     12.0 |#
     13.1 |
     14.3 |
     15.4 |
     16.5 |
     17.7 |
     18.8 |
     20.0 |
     21.1 |
     22.2 |#
     23.4 |#
     24.5 |##
     25.7 |
     26.8 |###
     27.9 |
     29.1 |##
     30.2 |
  (10 below, 15 above range)

```

## Diagnostics

- **inc_cas_loop**: CV=22.9% (high variance, measurements may be unstable)
- **inc_fetch_add**: CV=64.5% (high variance, measurements may be unstable)
- **inc_load_store**: CV=66.1% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **inc_load_store**: bridge=221.8% of algo (FFI overhead may distort results)
