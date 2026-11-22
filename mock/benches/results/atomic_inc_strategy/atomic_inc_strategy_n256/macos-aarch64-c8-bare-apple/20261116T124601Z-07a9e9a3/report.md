# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (610 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 123325% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (494 ns) by 123325%, a clear separation rather than a photo finish. CV 93.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -610 ns (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 1526.0x slower than the field

inc_cas_loop (610 ns) is 1526.0x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 93.2%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 93.2%), while inc_cas_loop is the steadiest (CV 0.7%, 610 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 1526.0x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (610 ns): 1526.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1526.00x (fastest 0.4 ns, slowest 610.4 ns)

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
| inc_cas_loop | 2731ns | 2728ns | 2721ns | 2728ns | 2753ns | base |
| inc_fetch_add | 2625ns | 2622ns | 2615ns | 2622ns | 2646ns | -3.88% |
| inc_load_store | 2137ns | 2133ns | 2127ns | 2133ns | 2158ns | -21.77% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 610ns | 604ns | 615ns | base | 0.420 |
| inc_fetch_add | 494ns | 490ns | 500ns | -19.01% | 0.518 |
| inc_load_store | 0ns | 0ns | 1ns | -99.95% | 820.842 |

## Performance model

- Peak throughput: **0.523 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.419 | 80.2% |
| inc_fetch_add | 0.519 | 99.2% |
| inc_load_store | 640.000 | 122428.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 2731ns | 2731ns | base |
| inc_fetch_add | 2625ns | 2625ns | -3.88% |
| inc_load_store | 2137ns | 2137ns | -21.77% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 610ns | base | --- | [610, 612] | --- | --- | --- | --- |
| inc_fetch_add | 494ns | -116.7ns (-19.1%) | [-117, -116]ns | [493, 494] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -610.0ns (-99.9%) | [-611, -610]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 610ns | -19.1% | -99.9% |
| 1 | 2 | 610ns | -19.0% | -99.9% |
| 1 | 3 | 611ns | -19.2% | -99.9% |
| 1 | 4 | 610ns | -19.1% | -99.9% |
| 1 | 5 | 611ns | -19.2% | -100.0% |
| 1 | 6 | 611ns | -19.2% | -100.0% |
| 1 | 7 | 610ns | -19.2% | -100.0% |
| 1 | 8 | 611ns | -19.1% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.165 | ok |
| inc_fetch_add | 0.218 | moderate+ |
| inc_load_store | 0.392 | moderate+ |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.5ns | 610.1ns | 0.4% |  |
| inc_fetch_add | 4.3ns | 494.1ns | 0.9% |  |
| inc_load_store | 2.4ns | 0.3ns | 769.9% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 604.0-615.4 ns)
    604.0 |#####
    604.5 |############
    605.1 |##
    605.7 |############
    606.2 |###############
    606.8 |###############
    607.4 |######################
    608.0 |############
    608.5 |###############
    609.1 |########################################
    609.7 |############
    610.2 |################################
    610.8 |#####
    611.4 |######################
    612.0 |#####################################
    612.5 |############
    613.1 |#######
    613.7 |###########################
    614.2 |##########
    614.8 |#################
  (11 below, 14 above range)

inc_fetch_add (n=160, range 489.7-499.5 ns)
    489.7 |#########
    490.2 |####
    490.7 |##############
    491.2 |#########
    491.7 |#####################
    492.2 |#######################
    492.7 |#######################
    493.1 |#####################################
    493.6 |############################
    494.1 |########################################
    494.6 |#####################
    495.1 |#####################
    495.6 |################
    496.1 |###########
    496.6 |###########
    497.1 |##############
    497.6 |##
    498.1 |#########
    498.5 |
    499.0 |
  (15 below, 9 above range)

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
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |##########
      0.8 |
      0.9 |
  (0 below, 8 above range)

```

## Diagnostics

- **inc_load_store**: CV=119.5% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=625.0% of algo (FFI overhead may distort results)
