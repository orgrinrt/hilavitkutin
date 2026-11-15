# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (687 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -686 ns (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Whole field within 0.0% of the fastest

All 3 variants sit between 0 ns and 687 ns - a 0.0% band - though some paired differences are still significant.

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
| inc_cas_loop | 2951ns | 3067ns | 2691ns | 2980ns | 3122ns | base |
| inc_fetch_add | 2814ns | 2802ns | 2616ns | 2827ns | 2975ns | -4.63% |
| inc_load_store | 2326ns | 2402ns | 2120ns | 2363ns | 2424ns | -21.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 663ns | 603ns | 707ns | base | 0.386 |
| inc_fetch_add | 532ns | 493ns | 565ns | -19.72% | 0.481 |
| inc_load_store | 0ns | 0ns | 1ns | -99.96% | 894.323 |

## Performance model

- Peak throughput: **0.519 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.373 | 71.8% |
| inc_fetch_add | 0.479 | 92.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 2951ns | 2951ns | base |
| inc_fetch_add | 2814ns | 2814ns | -4.63% |
| inc_load_store | 2326ns | 2326ns | -21.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 687ns | base | --- | [683, 689] | --- | --- | --- | --- |
| inc_fetch_add | 535ns | -132.5ns (-19.3%) | [-136, -130]ns | [520, 557] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -686.3ns (-99.9%) | [-689, -683]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 618ns | -14.8% | -99.9% |
| 1 | 2 | 644ns | -21.8% | -99.9% |
| 1 | 3 | 692ns | -19.7% | -99.9% |
| 1 | 4 | 693ns | -27.0% | -100.0% |
| 1 | 5 | 689ns | -28.2% | -100.0% |
| 1 | 6 | 691ns | -18.9% | -99.9% |
| 1 | 7 | 685ns | -18.4% | -100.0% |
| 1 | 8 | 615ns | -8.5% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | 0.231 | moderate+ |
| inc_fetch_add | 0.061 | ok |
| inc_load_store | 0.125 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.7ns | 663.1ns | 0.4% |  |
| inc_fetch_add | 4.6ns | 532.3ns | 0.9% |  |
| inc_load_store | 2.7ns | 0.3ns | 935.2% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 602.8-706.8 ns)
    602.8 |#######
    608.0 |########
    613.2 |########
    618.4 |#####
    623.6 |########
    628.8 |##
    634.0 |#
    639.2 |####
    644.4 |####
    649.6 |##
    654.8 |#
    660.0 |
    665.2 |
    670.4 |
    675.6 |
    680.8 |############
    686.0 |#################################
    691.2 |########################################
    696.4 |###
    701.6 |##
  (16 below, 7 above range)

inc_fetch_add (n=160, range 493.4-564.7 ns)
    493.4 |#####################
    497.0 |######
    500.6 |#########
    504.1 |##########
    507.7 |#
    511.3 |##
    514.8 |##
    518.4 |############
    522.0 |######
    525.5 |#
    529.1 |###
    532.7 |
    536.2 |#
    539.8 |
    543.3 |###
    546.9 |#
    550.5 |#
    554.0 |##########
    557.6 |########################################
    561.2 |#########################
  (16 below, 11 above range)

inc_load_store (n=160, range 0.0-0.9 ns)
      0.0 |########################################
      0.0 |
      0.1 |
      0.1 |
      0.2 |
      0.2 |
      0.3 |
      0.3 |
      0.4 |#####################
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
      0.8 |
  (0 below, 7 above range)

```

## Diagnostics

- **inc_load_store**: CV=128.9% (high variance, measurements may be unstable)
