# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (38.78 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 8551712% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (34.21 us) by 8551712%, a clear separation rather than a photo finish. CV 90.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -38.78 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 96954.2x slower than the field

inc_cas_loop (38.78 us) is 96954.2x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 90.2%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 90.2%), while inc_cas_loop is the steadiest (CV 0.6%, 38.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 96954.2x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (38.78 us): 96954.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 96954.25x (fastest 0.4 ns, slowest 38781.7 ns)

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
| inc_cas_loop | 40985ns | 40945ns | 40789ns | 40944ns | 41303ns | base |
| inc_fetch_add | 36510ns | 36364ns | 36254ns | 36379ns | 37159ns | -10.92% |
| inc_load_store | 2136ns | 2130ns | 2125ns | 2131ns | 2163ns | -94.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 38824ns | 38638ns | 39122ns | base | 0.422 |
| inc_fetch_add | 34343ns | 34102ns | 34964ns | -11.54% | 0.477 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 50606.950 |

## Performance model

- Peak throughput: **0.480 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.422 | 87.9% |
| inc_fetch_add | 0.479 | 99.7% |
| inc_load_store | 40960.000 | 8525511.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 40985ns | 40985ns | base |
| inc_fetch_add | 36510ns | 36510ns | -10.92% |
| inc_load_store | 2136ns | 2136ns | -94.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 38782ns | base | --- | [38766, 38798] | --- | --- | --- | --- |
| inc_fetch_add | 34207ns | -4553.1ns (-11.7%) | [-4580, -4534]ns | [34185, 34232] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -38781.5ns (-100.0%) | [-38798, -38766]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 38819ns | -11.9% | -100.0% |
| 1 | 2 | 38730ns | -11.6% | -100.0% |
| 1 | 3 | 38763ns | -11.9% | -100.0% |
| 1 | 4 | 38769ns | -11.8% | -100.0% |
| 1 | 5 | 38751ns | -11.6% | -100.0% |
| 1 | 6 | 38941ns | -11.8% | -100.0% |
| 1 | 7 | 38816ns | -10.8% | -100.0% |
| 1 | 8 | 38742ns | -11.8% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.060 | ok |
| inc_fetch_add | 0.030 | ok |
| inc_load_store | -0.011 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 3.4ns | 38823.7ns | 0.0% |  |
| inc_fetch_add | 5.3ns | 34343.1ns | 0.0% |  |
| inc_load_store | 3.4ns | 0.3ns | 1045.2% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 38637.6-39121.6 ns)
  38637.6 |######
  38661.8 |######################
  38686.0 |##############
  38710.2 |################################
  38734.4 |##################
  38758.6 |########################################
  38782.8 |##################################
  38807.0 |##########
  38831.2 |####################
  38855.4 |##################
  38879.6 |####################
  38903.8 |############
  38928.0 |######
  38952.2 |######
  38976.4 |####
  39000.6 |####
  39024.8 |##
  39049.0 |##
  39073.2 |##
  39097.4 |##
  (16 below, 7 above range)

inc_fetch_add (n=160, range 34102.0-34964.4 ns)
  34102.0 |#########################
  34145.2 |########################################
  34188.3 |#########################
  34231.4 |#######################
  34274.5 |################
  34317.6 |#########
  34360.7 |###
  34403.9 |
  34447.0 |##
  34490.1 |####
  34533.2 |
  34576.3 |##
  34619.4 |
  34662.6 |
  34705.7 |
  34748.8 |
  34791.9 |
  34835.0 |
  34878.1 |#
  34921.3 |
  (13 below, 15 above range)

inc_load_store (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |##############################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |###############
      0.8 |
      0.9 |
  (0 below, 6 above range)

```

## Diagnostics

- **inc_load_store**: CV=111.5% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=825.0% of algo (FFI overhead may distort results)
