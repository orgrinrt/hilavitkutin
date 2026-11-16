# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (9.77 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 2128450% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (8.51 us) by 2128450%, a clear separation rather than a photo finish. CV 80.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -9.77 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 24428.0x slower than the field

inc_cas_loop (9.77 us) is 24428.0x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 80.0%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 80.0%), while inc_fetch_add is the steadiest (CV 0.4%, 8.51 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 24428.0x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (9.77 us): 24428.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 24428.00x (fastest 0.4 ns, slowest 9771.2 ns)

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
| inc_cas_loop | 11908ns | 11898ns | 11871ns | 11899ns | 11970ns | base |
| inc_fetch_add | 10657ns | 10651ns | 10628ns | 10651ns | 10704ns | -10.50% |
| inc_load_store | 2133ns | 2132ns | 2125ns | 2132ns | 2147ns | -82.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 9780ns | 9747ns | 9838ns | base | 0.419 |
| inc_fetch_add | 8520ns | 8495ns | 8563ns | -12.89% | 0.481 |
| inc_load_store | 0ns | 0ns | 1ns | -100.00% | 13768.067 |

## Performance model

- Peak throughput: **0.482 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.419 | 86.9% |
| inc_fetch_add | 0.481 | 99.8% |
| inc_load_store | 10240.000 | 2123816.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 11908ns | 11908ns | base |
| inc_fetch_add | 10657ns | 10657ns | -10.50% |
| inc_load_store | 2133ns | 2133ns | -82.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 9771ns | base | --- | [9768, 9775] | --- | --- | --- | --- |
| inc_fetch_add | 8514ns | -1256.7ns (-12.9%) | [-1262, -1250]ns | [8511, 8518] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -9770.6ns (-100.0%) | [-9775, -9768]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 9768ns | -12.8% | -100.0% |
| 1 | 2 | 9780ns | -12.9% | -100.0% |
| 1 | 3 | 9774ns | -12.9% | -100.0% |
| 1 | 4 | 9771ns | -12.9% | -100.0% |
| 1 | 5 | 9770ns | -12.9% | -100.0% |
| 1 | 6 | 9774ns | -12.9% | -100.0% |
| 1 | 7 | 9774ns | -12.9% | -100.0% |
| 1 | 8 | 9768ns | -12.8% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.257 | moderate- |
| inc_fetch_add | -0.183 | ok |
| inc_load_store | -0.279 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.6ns | 9780.4ns | 0.0% |  |
| inc_fetch_add | 4.7ns | 8520.1ns | 0.1% |  |
| inc_load_store | 3.1ns | 0.3ns | 1056.9% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 9747.3-9838.2 ns)
   9747.3 |#######################
   9751.9 |############################
   9756.4 |##############################
   9761.0 |#########################
   9765.5 |########################################
   9770.1 |###################################
   9774.6 |#####################################
   9779.1 |##################
   9783.7 |#######################
   9788.2 |##############
   9792.8 |##############
   9797.3 |###########
   9801.9 |##
   9806.4 |
   9810.9 |####
   9815.5 |#######
   9820.0 |
   9824.6 |##
   9829.1 |##
   9833.7 |##
  (11 below, 11 above range)

inc_fetch_add (n=160, range 8495.3-8562.6 ns)
   8495.3 |#########################
   8498.6 |##############################
   8502.0 |################################
   8505.4 |#########################
   8508.7 |########################################
   8512.1 |##############################
   8515.5 |##############################
   8518.8 |################################
   8522.2 |##############################
   8525.6 |####################
   8528.9 |##########
   8532.3 |##
   8535.7 |############
   8539.0 |
   8542.4 |
   8545.8 |##
   8549.1 |
   8552.5 |
   8555.9 |###############
   8559.2 |#####
  (12 below, 11 above range)

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
      0.4 |################################
      0.4 |
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |#############
  (0 below, 4 above range)

```

## Diagnostics

- **inc_load_store**: CV=107.6% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=725.0% of algo (FFI overhead may distort results)
