# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (38.10 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 8395837% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (33.58 us) by 8395837%, a clear separation rather than a photo finish. CV 100.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -38.10 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 95255.8x slower than the field

inc_cas_loop (38.10 us) is 95255.8x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 100.8%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 100.8%), while inc_fetch_add is the steadiest (CV 0.3%, 33.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### inc_load_store shows alternating (throttle bounce) (autocorr -0.62)

inc_load_store's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 95255.8x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (38.10 us): 95255.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 95255.75x (fastest 0.4 ns, slowest 38102.3 ns)

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
| inc_cas_loop | 40242ns | 40223ns | 40062ns | 40220ns | 40488ns | base |
| inc_fetch_add | 35715ns | 35699ns | 35574ns | 35704ns | 35890ns | -11.25% |
| inc_load_store | 2121ns | 2128ns | 2089ns | 2123ns | 2145ns | -94.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 38121ns | 37947ns | 38352ns | base | 0.430 |
| inc_fetch_add | 33593ns | 33466ns | 33747ns | -11.88% | 0.488 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 49742.694 |

## Performance model

- Peak throughput: **0.490 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.430 | 87.8% |
| inc_fetch_add | 0.488 | 99.7% |
| inc_load_store | 40960.000 | 8366555.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 40242ns | 40242ns | base |
| inc_fetch_add | 35715ns | 35715ns | -11.25% |
| inc_load_store | 2121ns | 2121ns | -94.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 38102ns | base | --- | [38083, 38124] | --- | --- | --- | --- |
| inc_fetch_add | 33584ns | -4502.6ns (-11.8%) | [-4526, -4482]ns | [33562, 33608] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -38101.7ns (-100.0%) | [-38123, -38083]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 38038ns | -11.9% | -100.0% |
| 1 | 2 | 38131ns | -11.8% | -100.0% |
| 1 | 3 | 38165ns | -11.9% | -100.0% |
| 1 | 4 | 38037ns | -11.7% | -100.0% |
| 1 | 5 | 38096ns | -11.8% | -100.0% |
| 1 | 6 | 38119ns | -11.8% | -100.0% |
| 1 | 7 | 38089ns | -11.8% | -100.0% |
| 1 | 8 | 38064ns | -12.0% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.238 | moderate- |
| inc_fetch_add | 0.002 | ok |
| inc_load_store | -0.619 | HIGH- (thermal bounce) |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 3.5ns | 38120.9ns | 0.0% |  |
| inc_fetch_add | 5.6ns | 33593.4ns | 0.0% |  |
| inc_load_store | 3.1ns | 0.3ns | 951.0% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 37946.5-38352.1 ns)
  37946.5 |#############
  37966.8 |##########################
  37987.1 |#################################
  38007.4 |################
  38027.6 |#################################
  38047.9 |##############################
  38068.2 |########################################
  38088.5 |####################################
  38108.8 |####################################
  38129.1 |####################################
  38149.3 |####################################
  38169.6 |####################################
  38189.9 |#############
  38210.2 |##########
  38230.5 |##########
  38250.7 |##########
  38271.0 |###
  38291.3 |################
  38311.6 |######
  38331.9 |###
  (14 below, 11 above range)

inc_fetch_add (n=160, range 33466.2-33747.0 ns)
  33466.2 |#####
  33480.3 |#################
  33494.3 |########################################
  33508.3 |######################
  33522.4 |###########
  33536.4 |##############
  33550.5 |########################################
  33564.5 |#################
  33578.5 |############################
  33592.6 |#################
  33606.6 |#########################
  33620.6 |#################
  33634.7 |##################################
  33648.7 |########
  33662.8 |#########################
  33676.8 |###########
  33690.8 |###########
  33704.9 |########
  33718.9 |#####
  33732.9 |########
  (17 below, 13 above range)

inc_load_store (n=160, range 0.0-1.0 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |############################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |########
      0.8 |
      0.9 |
      0.9 |
  (0 below, 14 above range)

```

## Diagnostics

- **inc_load_store**: CV=122.5% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=725.0% of algo (FFI overhead may distort results)
