# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (9.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -9.77 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Whole field within 0.0% of the fastest

All 3 variants sit between 0 ns and 9.77 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: inc_load_store** at 0.0 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline

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
| inc_cas_loop | 11986ns | 11896ns | 11652ns | 11895ns | 12591ns | base |
| inc_fetch_add | 10716ns | 10593ns | 10438ns | 10627ns | 11263ns | -10.59% |
| inc_load_store | 2196ns | 2156ns | 2086ns | 2168ns | 2388ns | -81.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 9847ns | 9575ns | 10351ns | base | 0.416 |
| inc_fetch_add | 8578ns | 8352ns | 9012ns | -12.89% | 0.478 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 16023.472 |

## Performance model

- Peak throughput: **0.490 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.419 | 85.5% |
| inc_fetch_add | 0.483 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 11986ns | 11986ns | base |
| inc_fetch_add | 10716ns | 10716ns | -10.59% |
| inc_load_store | 2196ns | 2196ns | -81.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 9771ns | base | --- | [9762, 9786] | --- | --- | --- | --- |
| inc_fetch_add | 8479ns | -1233.8ns (-12.6%) | [-1293, -1199]ns | [8439, 8525] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -9771.2ns (-100.0%) | [-9786, -9762]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 9776ns | -14.4% | -100.0% |
| 1 | 2 | 9573ns | -11.2% | -100.0% |
| 1 | 3 | 9944ns | -15.9% | -100.0% |
| 1 | 4 | 9991ns | -15.1% | -100.0% |
| 1 | 5 | 9727ns | -13.3% | -100.0% |
| 1 | 6 | 9772ns | -13.3% | -100.0% |
| 1 | 7 | 9743ns | -8.1% | -100.0% |
| 1 | 8 | 9771ns | -11.2% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.078 | ok |
| inc_fetch_add | 0.242 | moderate+ |
| inc_load_store | -0.420 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.7ns | 9847.2ns | 0.0% |  |
| inc_fetch_add | 4.7ns | 8578.0ns | 0.1% |  |
| inc_load_store | 2.9ns | 0.3ns | 1133.7% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 9574.7-10351.4 ns)
   9574.7 |###########################
   9613.5 |##############
   9652.3 |#######
   9691.2 |####
   9730.0 |###################################
   9768.8 |########################################
   9807.7 |############
   9846.5 |##
   9885.4 |#######
   9924.2 |#######
   9963.0 |########
  10001.9 |#
  10040.7 |##
  10079.5 |
  10118.4 |##
  10157.2 |
  10196.0 |#
  10234.9 |########
  10273.7 |#######
  10312.5 |##
  (16 below, 8 above range)

inc_fetch_add (n=160, range 8351.6-9012.3 ns)
   8351.6 |########################################
   8384.6 |##########################
   8417.7 |################
   8450.7 |###########
   8483.7 |###########
   8516.8 |########
   8549.8 |#####
   8582.8 |##
   8615.9 |#################
   8648.9 |###########
   8681.9 |#######
   8715.0 |
   8748.0 |##
   8781.1 |#####
   8814.1 |##
   8847.1 |##
   8880.2 |
   8913.2 |
   8946.2 |#################
   8979.3 |########
  (17 below, 6 above range)

inc_load_store (n=160, range 0.0-0.8 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.3 |
      0.4 |#######################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.7 |
  (0 below, 21 above range)

```

## Diagnostics

- **inc_load_store**: CV=132.5% (high variance, measurements may be unstable)
