# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (2.54 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 552600% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (2.21 us) by 552600%, a clear separation rather than a photo finish. CV 78.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -2.54 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 6345.3x slower than the field

inc_cas_loop (2.54 us) is 6345.3x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 78.9%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 78.9%), while inc_cas_loop is the steadiest (CV 3.6%, 2.54 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6345.3x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (2.54 us): 6345.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6345.25x (fastest 0.4 ns, slowest 2538.1 ns)

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
| inc_cas_loop | 4712ns | 4759ns | 4482ns | 4719ns | 4923ns | base |
| inc_fetch_add | 4527ns | 4449ns | 4310ns | 4510ns | 4797ns | -3.93% |
| inc_load_store | 2228ns | 2222ns | 2124ns | 2219ns | 2359ns | -52.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2513ns | 2389ns | 2626ns | base | 0.407 |
| inc_fetch_add | 2248ns | 2140ns | 2384ns | -10.55% | 0.456 |
| inc_load_store | 0ns | 0ns | 1ns | -99.99% | 3657.143 |

## Performance model

- Peak throughput: **0.479 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.403 | 84.3% |
| inc_fetch_add | 0.463 | 96.8% |
| inc_load_store | 2560.000 | 535002.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 4712ns | 4712ns | base |
| inc_fetch_add | 4527ns | 4527ns | -3.93% |
| inc_load_store | 2228ns | 2228ns | -52.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 2538ns | base | --- | [2536, 2541] | --- | --- | --- | --- |
| inc_fetch_add | 2211ns | -223.1ns (-8.8%) | [-287, -208]ns | [2209, 2217] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -2537.5ns (-100.0%) | [-2540, -2535]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 2424ns | -9.1% | -100.0% |
| 1 | 2 | 2541ns | -6.6% | -100.0% |
| 1 | 3 | 2542ns | -13.1% | -100.0% |
| 1 | 4 | 2549ns | -16.2% | -100.0% |
| 1 | 5 | 2374ns | -6.8% | -100.0% |
| 1 | 6 | 2556ns | -13.5% | -100.0% |
| 1 | 7 | 2541ns | -12.7% | -100.0% |
| 1 | 8 | 2533ns | -6.2% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.294 | moderate- |
| inc_fetch_add | -0.080 | ok |
| inc_load_store | 0.058 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 3.0ns | 2513.2ns | 0.1% |  |
| inc_fetch_add | 5.4ns | 2248.0ns | 0.2% |  |
| inc_load_store | 3.3ns | 0.3ns | 1187.7% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 2388.6-2625.9 ns)
   2388.6 |###
   2400.4 |
   2412.3 |#############
   2424.2 |#####
   2436.0 |
   2447.9 |###
   2459.8 |
   2471.6 |
   2483.5 |
   2495.4 |##
   2507.2 |#
   2519.1 |#
   2531.0 |########################################
   2542.9 |#########################
   2554.7 |#####
   2566.6 |####
   2578.5 |
   2590.3 |##
   2602.2 |
   2614.1 |
  (18 below, 10 above range)

inc_fetch_add (n=160, range 2140.0-2383.6 ns)
   2140.0 |####
   2152.2 |###
   2164.4 |
   2176.5 |##
   2188.7 |##################
   2200.9 |########################################
   2213.1 |###################
   2225.3 |##
   2237.4 |##
   2249.6 |###
   2261.8 |#
   2274.0 |#
   2286.1 |#
   2298.3 |#
   2310.5 |
   2322.7 |#
   2334.9 |##
   2347.0 |
   2359.2 |#####
   2371.4 |##############################
  (16 below, 9 above range)

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
      0.3 |
      0.4 |############################
      0.4 |
      0.5 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.7 |
  (0 below, 27 above range)

```

## Diagnostics

- **inc_load_store**: CV=112.7% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=825.0% of algo (FFI overhead may distort results)
