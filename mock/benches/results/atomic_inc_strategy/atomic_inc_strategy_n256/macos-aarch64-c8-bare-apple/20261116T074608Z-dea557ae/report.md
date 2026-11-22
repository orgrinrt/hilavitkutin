# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (610 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 123950% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (496 ns) by 123950%, a clear separation rather than a photo finish. CV 94.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -610 ns (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 1525.0x slower than the field

inc_cas_loop (610 ns) is 1525.0x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 94.3%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 94.3%), while inc_cas_loop is the steadiest (CV 1.5%, 610 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 1525.0x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (610 ns): 1525.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1525.00x (fastest 0.4 ns, slowest 610.0 ns)

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
| inc_cas_loop | 2722ns | 2717ns | 2706ns | 2717ns | 2750ns | base |
| inc_fetch_add | 2621ns | 2614ns | 2607ns | 2615ns | 2651ns | -3.72% |
| inc_load_store | 2125ns | 2120ns | 2111ns | 2121ns | 2151ns | -21.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 610ns | 602ns | 620ns | base | 0.419 |
| inc_fetch_add | 499ns | 492ns | 513ns | -18.27% | 0.513 |
| inc_load_store | 0ns | 0ns | 1ns | -99.95% | 772.830 |

## Performance model

- Peak throughput: **0.520 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.420 | 80.7% |
| inc_fetch_add | 0.516 | 99.2% |
| inc_load_store | 640.000 | 123060.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 2722ns | 2722ns | base |
| inc_fetch_add | 2621ns | 2621ns | -3.72% |
| inc_load_store | 2125ns | 2125ns | -21.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 610ns | base | --- | [609, 611] | --- | --- | --- | --- |
| inc_fetch_add | 496ns | -113.4ns (-18.6%) | [-114, -113]ns | [495, 497] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -609.6ns (-99.9%) | [-610, -609]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 611ns | -18.5% | -100.0% |
| 1 | 2 | 610ns | -18.7% | -99.9% |
| 1 | 3 | 609ns | -18.3% | -99.9% |
| 1 | 4 | 610ns | -18.9% | -99.9% |
| 1 | 5 | 610ns | -18.7% | -100.0% |
| 1 | 6 | 600ns | -17.4% | -100.0% |
| 1 | 7 | 611ns | -18.6% | -99.9% |
| 1 | 8 | 609ns | -18.6% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.219 | moderate- |
| inc_fetch_add | -0.244 | moderate- |
| inc_load_store | 0.058 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.5ns | 610.4ns | 0.4% |  |
| inc_fetch_add | 4.8ns | 498.8ns | 1.0% |  |
| inc_load_store | 2.3ns | 0.3ns | 702.6% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 602.2-619.8 ns)
    602.2 |
    603.1 |####
    603.9 |####
    604.8 |#############
    605.7 |#############
    606.6 |#################
    607.5 |###################################
    608.3 |#################################
    609.2 |########################################
    610.1 |#################################
    611.0 |##########################
    611.9 |########################
    612.7 |##########################
    613.6 |######
    614.5 |###########
    615.4 |#############
    616.3 |####
    617.2 |##
    618.0 |####
    618.9 |
  (13 below, 5 above range)

inc_fetch_add (n=160, range 492.2-513.0 ns)
    492.2 |######################
    493.3 |################################
    494.3 |########################
    495.4 |########################################
    496.4 |########################
    497.4 |#################################
    498.5 |#################
    499.5 |###################
    500.6 |###
    501.6 |###
    502.6 |
    503.7 |#
    504.7 |#
    505.7 |#
    506.8 |
    507.8 |#
    508.9 |#
    509.9 |
    510.9 |#
    512.0 |
  (11 below, 6 above range)

inc_load_store (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |#################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |##########
      0.8 |
      0.8 |
      0.9 |
  (0 below, 11 above range)

```

## Diagnostics

- **inc_load_store**: CV=113.9% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=625.0% of algo (FFI overhead may distort results)
