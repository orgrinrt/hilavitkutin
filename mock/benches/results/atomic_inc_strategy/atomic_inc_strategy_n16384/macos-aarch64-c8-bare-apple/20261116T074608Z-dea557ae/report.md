# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (38.84 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 8589275% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (34.36 us) by 8589275%, a clear separation rather than a photo finish. CV 85.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -38.84 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 97092.1x slower than the field

inc_cas_loop (38.84 us) is 97092.1x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 85.8%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 85.8%), while inc_cas_loop is the steadiest (CV 1.1%, 38.84 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### inc_fetch_add shows alternating (throttle bounce) (autocorr -0.50)

inc_fetch_add's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 97092.1x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (38.84 us): 97092.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 97092.13x (fastest 0.4 ns, slowest 38836.9 ns)

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
| inc_cas_loop | 41107ns | 40988ns | 40831ns | 41002ns | 41697ns | base |
| inc_fetch_add | 36586ns | 36502ns | 36143ns | 36528ns | 37203ns | -11.00% |
| inc_load_store | 2123ns | 2118ns | 2112ns | 2118ns | 2148ns | -94.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 38951ns | 38693ns | 39518ns | base | 0.421 |
| inc_fetch_add | 34433ns | 34021ns | 35020ns | -11.60% | 0.476 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 50027.481 |

## Performance model

- Peak throughput: **0.482 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.422 | 87.6% |
| inc_fetch_add | 0.477 | 99.0% |
| inc_load_store | 40960.000 | 8505139.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 41107ns | 41107ns | base |
| inc_fetch_add | 36586ns | 36586ns | -11.00% |
| inc_load_store | 2123ns | 2123ns | -94.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 38837ns | base | --- | [38818, 38866] | --- | --- | --- | --- |
| inc_fetch_add | 34358ns | -4475.6ns (-11.5%) | [-4523, -4438]ns | [34339, 34379] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -38836.7ns (-100.0%) | [-38866, -38819]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 39080ns | -12.1% | -100.0% |
| 1 | 2 | 38795ns | -11.4% | -100.0% |
| 1 | 3 | 38847ns | -11.6% | -100.0% |
| 1 | 4 | 38893ns | -10.3% | -100.0% |
| 1 | 5 | 38815ns | -12.4% | -100.0% |
| 1 | 6 | 38824ns | -11.6% | -100.0% |
| 1 | 7 | 38807ns | -11.1% | -100.0% |
| 1 | 8 | 38847ns | -11.6% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.175 | ok |
| inc_fetch_add | -0.504 | HIGH- (thermal bounce) |
| inc_load_store | 0.190 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.8ns | 38951.4ns | 0.0% |  |
| inc_fetch_add | 5.0ns | 34432.5ns | 0.0% |  |
| inc_load_store | 2.6ns | 0.3ns | 788.2% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 38693.4-39518.0 ns)
  38693.4 |#################################
  38734.6 |###############################
  38775.9 |#################################
  38817.1 |########################################
  38858.3 |##########################
  38899.6 |####################
  38940.8 |###############
  38982.0 |###
  39023.3 |#################
  39064.5 |#
  39105.7 |#
  39146.9 |###
  39188.2 |#
  39229.4 |#
  39270.6 |
  39311.9 |#####
  39353.1 |#
  39394.3 |#
  39435.6 |###
  39476.8 |
  (11 below, 9 above range)

inc_fetch_add (n=160, range 34020.6-35020.2 ns)
  34020.6 |
  34070.5 |
  34120.5 |##
  34170.5 |####
  34220.5 |###################
  34270.5 |###################
  34320.4 |########################################
  34370.4 |###################
  34420.4 |######
  34470.4 |############
  34520.4 |#####
  34570.3 |###
  34620.3 |
  34670.3 |####
  34720.3 |###
  34770.3 |#
  34820.3 |########
  34870.2 |##
  34920.2 |#
  34970.2 |#
  (14 below, 10 above range)

inc_load_store (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |
      0.4 |#################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |
      0.8 |###############
      0.8 |
  (0 below, 7 above range)

```

## Diagnostics

- **inc_load_store**: CV=104.8% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=625.0% of algo (FFI overhead may distort results)
