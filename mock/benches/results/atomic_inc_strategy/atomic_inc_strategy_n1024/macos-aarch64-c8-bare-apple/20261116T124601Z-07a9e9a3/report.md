# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (2.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 523225% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (2.09 us) by 523225%, a clear separation rather than a photo finish. CV 86.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -2.41 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 6036.5x slower than the field

inc_cas_loop (2.41 us) is 6036.5x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 86.2%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 86.2%), while inc_cas_loop is the steadiest (CV 0.7%, 2.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6036.5x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (2.41 us): 6036.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.4 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6036.50x (fastest 0.4 ns, slowest 2414.6 ns)

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
| inc_cas_loop | 4553ns | 4540ns | 4529ns | 4542ns | 4612ns | base |
| inc_fetch_add | 4249ns | 4229ns | 4217ns | 4231ns | 4335ns | -6.68% |
| inc_load_store | 2139ns | 2132ns | 2125ns | 2133ns | 2173ns | -53.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2420ns | 2406ns | 2450ns | base | 0.423 |
| inc_fetch_add | 2104ns | 2087ns | 2147ns | -13.07% | 0.487 |
| inc_load_store | 0ns | 0ns | 1ns | -99.99% | 3270.259 |

## Performance model

- Peak throughput: **0.491 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.424 | 86.4% |
| inc_fetch_add | 0.489 | 99.7% |
| inc_load_store | 2560.000 | 521739.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 4553ns | 4553ns | base |
| inc_fetch_add | 4249ns | 4249ns | -6.68% |
| inc_load_store | 2139ns | 2139ns | -53.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 2415ns | base | --- | [2413, 2416] | --- | --- | --- | --- |
| inc_fetch_add | 2093ns | -320.0ns (-13.3%) | [-322, -318]ns | [2092, 2095] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -2414.2ns (-100.0%) | [-2416, -2413]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 2412ns | -13.2% | -100.0% |
| 1 | 2 | 2455ns | -13.3% | -100.0% |
| 1 | 3 | 2414ns | -13.4% | -100.0% |
| 1 | 4 | 2414ns | -12.6% | -100.0% |
| 1 | 5 | 2413ns | -13.3% | -100.0% |
| 1 | 6 | 2418ns | -13.6% | -100.0% |
| 1 | 7 | 2411ns | -13.3% | -100.0% |
| 1 | 8 | 2413ns | -13.3% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.186 | ok |
| inc_fetch_add | -0.233 | moderate- |
| inc_load_store | -0.258 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 3.0ns | 2420.3ns | 0.1% |  |
| inc_fetch_add | 4.8ns | 2104.0ns | 0.2% |  |
| inc_load_store | 3.5ns | 0.3ns | 1113.0% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 2406.2-2450.1 ns)
   2406.2 |####################
   2408.4 |###########################
   2410.6 |########################################
   2412.8 |##################################
   2415.0 |##########################
   2417.2 |###################
   2419.4 |##########
   2421.6 |##########
   2423.8 |############
   2426.0 |###
   2428.2 |#####
   2430.4 |
   2432.5 |
   2434.7 |#
   2436.9 |
   2439.1 |
   2441.3 |
   2443.5 |#
   2445.7 |#
   2447.9 |
  (15 below, 21 above range)

inc_fetch_add (n=160, range 2087.0-2146.9 ns)
   2087.0 |############################
   2090.0 |########################################
   2092.9 |#####################
   2095.9 |#######
   2098.9 |#######
   2101.9 |##
   2104.9 |########
   2107.9 |##
   2110.9 |######
   2113.9 |##
   2116.9 |##
   2119.9 |#####
   2122.9 |#####
   2125.9 |#
   2128.9 |##
   2131.9 |#
   2134.9 |##
   2137.9 |#
   2140.9 |
   2143.9 |
  (14 below, 8 above range)

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
      0.4 |################################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |#############
      0.8 |
  (0 below, 6 above range)

```

## Diagnostics

- **inc_load_store**: CV=110.2% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=825.0% of algo (FFI overhead may distort results)
