# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (38.13 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 8424012% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (33.70 us) by 8424012%, a clear separation rather than a photo finish. CV 97.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -38.13 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 95323.5x slower than the field

inc_cas_loop (38.13 us) is 95323.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 97.3%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 97.3%), while inc_cas_loop is the steadiest (CV 0.9%, 38.13 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### inc_cas_loop shows alternating (throttle bounce) (autocorr -0.54)

inc_cas_loop's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 95323.5x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (38.13 us): 95323.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 95323.50x (fastest 0.4 ns, slowest 38129.4 ns)

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
| inc_cas_loop | 40363ns | 40245ns | 40092ns | 40252ns | 40966ns | base |
| inc_fetch_add | 35924ns | 35829ns | 35624ns | 35826ns | 36519ns | -11.00% |
| inc_load_store | 2226ns | 2233ns | 2097ns | 2221ns | 2371ns | -94.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 38239ns | 37977ns | 38810ns | base | 0.428 |
| inc_fetch_add | 33805ns | 33512ns | 34379ns | -11.60% | 0.485 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 44658.262 |

## Performance model

- Peak throughput: **0.489 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.430 | 87.9% |
| inc_fetch_add | 0.486 | 99.5% |
| inc_load_store | 40960.000 | 8378101.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 40363ns | 40363ns | base |
| inc_fetch_add | 35924ns | 35924ns | -11.00% |
| inc_load_store | 2226ns | 2226ns | -94.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 38129ns | base | --- | [38103, 38158] | --- | --- | --- | --- |
| inc_fetch_add | 33696ns | -4421.2ns (-11.6%) | [-4462, -4390]ns | [33676, 33743] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -38129.2ns (-100.0%) | [-38157, -38103]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 38151ns | -10.3% | -100.0% |
| 1 | 2 | 38101ns | -11.3% | -100.0% |
| 1 | 3 | 38144ns | -11.6% | -100.0% |
| 1 | 4 | 38068ns | -11.8% | -100.0% |
| 1 | 5 | 38211ns | -11.4% | -100.0% |
| 1 | 6 | 38112ns | -11.7% | -100.0% |
| 1 | 7 | 38123ns | -11.9% | -100.0% |
| 1 | 8 | 38076ns | -11.6% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.543 | HIGH- (thermal bounce) |
| inc_fetch_add | 0.087 | ok |
| inc_load_store | 0.188 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 3.4ns | 38239.4ns | 0.0% |  |
| inc_fetch_add | 5.5ns | 33804.6ns | 0.0% |  |
| inc_load_store | 3.2ns | 0.4ns | 869.8% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 37977.2-38810.1 ns)
  37977.2 |#####################################
  38018.8 |########################################
  38060.5 |########################################
  38102.1 |###################################
  38143.8 |#################################
  38185.4 |###########################
  38227.1 |#############################
  38268.7 |########
  38310.3 |##
  38352.0 |##
  38393.6 |######
  38435.3 |##
  38476.9 |##
  38518.6 |####
  38560.2 |##
  38601.9 |
  38643.5 |##
  38685.1 |##
  38726.8 |
  38768.4 |####
  (11 below, 15 above range)

inc_fetch_add (n=160, range 33512.4-34379.2 ns)
  33512.4 |################################
  33555.7 |##############################
  33599.1 |#####################################
  33642.4 |########################################
  33685.8 |#####################################
  33729.1 |##############################
  33772.4 |############################
  33815.8 |#########################
  33859.1 |###########
  33902.5 |####
  33945.8 |
  33989.1 |##
  34032.5 |#########
  34075.8 |##
  34119.1 |####
  34162.5 |####
  34205.8 |
  34249.2 |####
  34292.5 |####
  34335.8 |#######
  (13 below, 11 above range)

inc_load_store (n=160, range 0.0-1.0 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |#####################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |###############
      0.8 |
      0.9 |
      0.9 |
  (0 below, 11 above range)

```

## Diagnostics

- **inc_load_store**: CV=106.1% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=725.0% of algo (FFI overhead may distort results)
