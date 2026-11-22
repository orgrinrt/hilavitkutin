# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (9.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 2137138% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (8.55 us) by 2137138%, a clear separation rather than a photo finish. CV 87.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -9.77 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 24433.9x slower than the field

inc_cas_loop (9.77 us) is 24433.9x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 87.2%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 87.2%), while inc_cas_loop is the steadiest (CV 0.4%, 9.77 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 24433.9x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (9.77 us): 24433.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 24433.87x (fastest 0.4 ns, slowest 9773.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 11902ns | 11895ns | 11852ns | 11895ns | 11975ns | base |
| inc_fetch_add | 10727ns | 10679ns | 10644ns | 10685ns | 10935ns | -9.87% |
| inc_load_store | 2125ns | 2119ns | 2111ns | 2120ns | 2153ns | -82.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 9780ns | 9738ns | 9846ns | base | 0.419 |
| inc_fetch_add | 8592ns | 8518ns | 8776ns | -12.15% | 0.477 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 12800.000 |

## Performance model

- Peak throughput: **0.481 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.419 | 87.2% |
| inc_fetch_add | 0.479 | 99.6% |
| inc_load_store | 10240.000 | 2129550.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 11902ns | 11902ns | base |
| inc_fetch_add | 10727ns | 10727ns | -9.87% |
| inc_load_store | 2125ns | 2125ns | -82.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 9774ns | base | --- | [9765, 9780] | --- | --- | --- | --- |
| inc_fetch_add | 8549ns | -1215.4ns (-12.4%) | [-1224, -1209]ns | [8542, 8555] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -9772.9ns (-100.0%) | [-9779, -9765]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 9755ns | -12.5% | -100.0% |
| 1 | 2 | 9773ns | -12.6% | -100.0% |
| 1 | 3 | 9754ns | -12.4% | -100.0% |
| 1 | 4 | 9783ns | -12.6% | -100.0% |
| 1 | 5 | 9773ns | -12.6% | -100.0% |
| 1 | 6 | 9760ns | -10.1% | -100.0% |
| 1 | 7 | 9765ns | -12.6% | -100.0% |
| 1 | 8 | 9782ns | -12.6% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.356 | moderate- |
| inc_fetch_add | -0.194 | ok |
| inc_load_store | 0.189 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.6ns | 9780.4ns | 0.0% |  |
| inc_fetch_add | 5.1ns | 8592.1ns | 0.1% |  |
| inc_load_store | 2.5ns | 0.3ns | 784.0% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 9738.3-9846.2 ns)
   9738.3 |########
   9743.7 |########################################
   9749.1 |##################################
   9754.5 |########################################
   9759.9 |##################
   9765.3 |########
   9770.7 |#####################################
   9776.1 |##################################
   9781.4 |################################
   9786.8 |########################
   9792.2 |#############
   9797.6 |#####
   9803.0 |##########
   9808.4 |##########
   9813.8 |#############
   9819.2 |##
   9824.6 |##
   9830.0 |########
   9835.4 |################
   9840.8 |##
  (15 below, 9 above range)

inc_fetch_add (n=160, range 8518.2-8776.2 ns)
   8518.2 |###############################
   8531.1 |########################################
   8544.0 |###############################
   8556.9 |###########
   8569.8 |########
   8582.7 |########
   8595.6 |########
   8608.5 |###
   8621.4 |######
   8634.3 |#
   8647.2 |##
   8660.1 |###
   8673.0 |#
   8685.9 |##
   8698.8 |#
   8711.7 |
   8724.6 |#
   8737.5 |
   8750.4 |
   8763.3 |#####
  (14 below, 11 above range)

inc_load_store (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.3 |
      0.4 |###################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.7 |
      0.8 |##########
      0.8 |
  (0 below, 10 above range)

```

## Diagnostics

- **inc_load_store**: CV=109.0% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=625.0% of algo (FFI overhead may distort results)
