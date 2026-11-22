# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (608 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 123225% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (493 ns) by 123225%, a clear separation rather than a photo finish. CV 91.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -608 ns (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 1519.7x slower than the field

inc_cas_loop (608 ns) is 1519.7x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 91.0%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 91.0%), while inc_fetch_add is the steadiest (CV 1.5%, 493 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 1519.7x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (608 ns): 1519.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1519.75x (fastest 0.4 ns, slowest 607.9 ns)

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
| inc_cas_loop | 2728ns | 2728ns | 2677ns | 2723ns | 2792ns | base |
| inc_fetch_add | 2626ns | 2624ns | 2597ns | 2624ns | 2664ns | -3.71% |
| inc_load_store | 2139ns | 2133ns | 2098ns | 2132ns | 2198ns | -21.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 609ns | 597ns | 625ns | base | 0.420 |
| inc_fetch_add | 494ns | 487ns | 502ns | -18.90% | 0.518 |
| inc_load_store | 0ns | 0ns | 1ns | -99.95% | 809.486 |

## Performance model

- Peak throughput: **0.526 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.421 | 80.1% |
| inc_fetch_add | 0.519 | 98.7% |
| inc_load_store | 640.000 | 121721.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 2728ns | 2728ns | base |
| inc_fetch_add | 2626ns | 2626ns | -3.71% |
| inc_load_store | 2139ns | 2139ns | -21.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 608ns | base | --- | [607, 609] | --- | --- | --- | --- |
| inc_fetch_add | 493ns | -114.6ns (-18.9%) | [-115, -113]ns | [492, 494] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -607.5ns (-99.9%) | [-609, -606]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 606ns | -18.5% | -99.9% |
| 1 | 2 | 609ns | -18.9% | -100.0% |
| 1 | 3 | 609ns | -19.0% | -100.0% |
| 1 | 4 | 609ns | -18.9% | -100.0% |
| 1 | 5 | 611ns | -19.0% | -99.9% |
| 1 | 6 | 616ns | -20.0% | -99.9% |
| 1 | 7 | 601ns | -18.7% | -99.9% |
| 1 | 8 | 597ns | -17.6% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | 0.199 | ok |
| inc_fetch_add | 0.212 | moderate+ |
| inc_load_store | 0.286 | moderate+ |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.4ns | 608.9ns | 0.4% |  |
| inc_fetch_add | 4.3ns | 493.8ns | 0.9% |  |
| inc_load_store | 2.3ns | 0.3ns | 720.4% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 596.8-624.5 ns)
    596.8 |##########
    598.2 |#####
    599.6 |####################
    601.0 |##########
    602.4 |##########
    603.8 |################################
    605.1 |##############################
    606.5 |#####################################
    607.9 |#####################################
    609.3 |##############################
    610.7 |####################
    612.1 |########################################
    613.4 |##########
    614.8 |####################
    616.2 |#####
    617.6 |############
    619.0 |##
    620.4 |##
    621.8 |
    623.1 |
  (19 below, 7 above range)

inc_fetch_add (n=160, range 486.9-502.4 ns)
    486.9 |#####
    487.7 |###
    488.4 |###
    489.2 |###
    490.0 |#########
    490.8 |###########################
    491.5 |########################################
    492.3 |#########################
    493.1 |#############################
    493.9 |###########################
    494.6 |#########################
    495.4 |#########
    496.2 |##############
    497.0 |##########
    497.7 |###
    498.5 |#####
    499.3 |###
    500.1 |#####
    500.8 |
    501.6 |#
  (14 below, 6 above range)

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
      0.4 |##################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.7 |
      0.8 |###########
      0.8 |
  (0 below, 7 above range)

```

## Diagnostics

- **inc_load_store**: CV=115.1% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=525.0% of algo (FFI overhead may distort results)
