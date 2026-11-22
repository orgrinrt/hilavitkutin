# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (9.60 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 2092400% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (8.37 us) by 2092400%, a clear separation rather than a photo finish. CV 94.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -9.60 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 23990.5x slower than the field

inc_cas_loop (9.60 us) is 23990.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 94.6%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 94.6%), while inc_cas_loop is the steadiest (CV 0.7%, 9.60 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 23990.5x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (9.60 us): 23990.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 23990.50x (fastest 0.4 ns, slowest 9596.2 ns)

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
| inc_cas_loop | 11718ns | 11689ns | 11654ns | 11695ns | 11848ns | base |
| inc_fetch_add | 10523ns | 10476ns | 10435ns | 10497ns | 10688ns | -10.19% |
| inc_load_store | 2122ns | 2129ns | 2092ns | 2123ns | 2148ns | -81.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 9619ns | 9566ns | 9729ns | base | 0.426 |
| inc_fetch_add | 8409ns | 8341ns | 8540ns | -12.58% | 0.487 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 13457.084 |

## Performance model

- Peak throughput: **0.491 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.427 | 86.9% |
| inc_fetch_add | 0.489 | 99.7% |
| inc_load_store | 10240.000 | 2085353.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 11718ns | 11718ns | base |
| inc_fetch_add | 10523ns | 10523ns | -10.19% |
| inc_load_store | 2122ns | 2122ns | -81.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 9596ns | base | --- | [9591, 9601] | --- | --- | --- | --- |
| inc_fetch_add | 8370ns | -1220.7ns (-12.7%) | [-1228, -1213]ns | [8361, 8388] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -9596.0ns (-100.0%) | [-9601, -9591]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 9631ns | -13.2% | -100.0% |
| 1 | 2 | 9615ns | -12.0% | -100.0% |
| 1 | 3 | 9581ns | -12.0% | -100.0% |
| 1 | 4 | 9595ns | -12.4% | -100.0% |
| 1 | 5 | 9596ns | -12.0% | -100.0% |
| 1 | 6 | 9690ns | -13.7% | -100.0% |
| 1 | 7 | 9592ns | -12.8% | -100.0% |
| 1 | 8 | 9592ns | -12.9% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.188 | ok |
| inc_fetch_add | 0.142 | ok |
| inc_load_store | 0.058 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.7ns | 9619.0ns | 0.0% |  |
| inc_fetch_add | 4.7ns | 8409.1ns | 0.1% |  |
| inc_load_store | 3.0ns | 0.3ns | 1001.0% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 9565.8-9729.2 ns)
   9565.8 |#####################
   9573.9 |########################################
   9582.1 |#######################
   9590.3 |######################################
   9598.4 |#####################
   9606.6 |########
   9614.8 |#############
   9623.0 |########
   9631.1 |########
   9639.3 |#####
   9647.5 |######
   9655.6 |######
   9663.8 |###
   9672.0 |#
   9680.1 |
   9688.3 |
   9696.5 |#
   9704.7 |###
   9712.8 |#
   9721.0 |#
  (14 below, 17 above range)

inc_fetch_add (n=160, range 8341.4-8540.2 ns)
   8341.4 |########################################
   8351.4 |##############################
   8361.3 |################
   8371.2 |########
   8381.2 |######
   8391.1 |###
   8401.1 |###########
   8411.0 |#
   8420.9 |##
   8430.9 |######
   8440.8 |###
   8450.8 |###
   8460.7 |#
   8470.7 |#
   8480.6 |##
   8490.5 |######
   8500.5 |##
   8510.4 |########
   8520.4 |########
   8530.3 |#####
  (13 below, 11 above range)

inc_load_store (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |#########################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |#############
      0.8 |
      0.9 |
  (0 below, 5 above range)

```

## Diagnostics

- **inc_load_store**: CV=124.3% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=725.0% of algo (FFI overhead may distort results)
