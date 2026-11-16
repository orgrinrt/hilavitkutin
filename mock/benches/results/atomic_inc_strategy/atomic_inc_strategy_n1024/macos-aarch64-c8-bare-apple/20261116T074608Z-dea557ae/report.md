# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (2.42 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 1049900% faster than the next best (inc_fetch_add)

inc_load_store (0 ns) leads inc_fetch_add (2.10 us) by 1049900%, a clear separation rather than a photo finish. CV 164.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -2.42 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 12104.0x slower than the field

inc_cas_loop (2.42 us) is 12104.0x the fastest (0 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 164.3%)

inc_load_store wins on median (0 ns) yet has the highest variance (CV 164.3%), while inc_cas_loop is the steadiest (CV 1.0%, 2.42 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 12104.0x the fastest

Fastest inc_load_store (0 ns) to slowest inc_cas_loop (2.42 us): 12104.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: inc_load_store** at 0.2 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 12104.00x (fastest 0.2 ns, slowest 2420.8 ns)

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
| inc_cas_loop | 4545ns | 4538ns | 4524ns | 4538ns | 4587ns | base |
| inc_fetch_add | 4230ns | 4222ns | 4210ns | 4223ns | 4270ns | -6.94% |
| inc_load_store | 2126ns | 2120ns | 2113ns | 2121ns | 2157ns | -53.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2426ns | 2409ns | 2458ns | base | 0.422 |
| inc_fetch_add | 2104ns | 2090ns | 2132ns | -13.26% | 0.487 |
| inc_load_store | 0ns | 0ns | 1ns | -99.99% | 3690.090 |

## Performance model

- Peak throughput: **0.490 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.423 | 86.3% |
| inc_fetch_add | 0.488 | 99.5% |
| inc_load_store | 5120.000 | 1045096.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 4545ns | 4545ns | base |
| inc_fetch_add | 4230ns | 4230ns | -6.94% |
| inc_load_store | 2126ns | 2126ns | -53.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 2421ns | base | --- | [2419, 2423] | --- | --- | --- | --- |
| inc_fetch_add | 2100ns | -321.9ns (-13.3%) | [-325, -320]ns | [2099, 2101] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -2420.6ns (-100.0%) | [-2422, -2418]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 2422ns | -13.6% | -100.0% |
| 1 | 2 | 2429ns | -13.4% | -100.0% |
| 1 | 3 | 2411ns | -12.8% | -100.0% |
| 1 | 4 | 2419ns | -13.4% | -100.0% |
| 1 | 5 | 2428ns | -13.4% | -100.0% |
| 1 | 6 | 2414ns | -12.9% | -100.0% |
| 1 | 7 | 2418ns | -13.1% | -100.0% |
| 1 | 8 | 2425ns | -13.4% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.374 | moderate- |
| inc_fetch_add | -0.356 | moderate- |
| inc_load_store | -0.386 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.8ns | 2426.0ns | 0.1% |  |
| inc_fetch_add | 5.0ns | 2104.4ns | 0.2% |  |
| inc_load_store | 2.6ns | 0.3ns | 919.1% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 2409.1-2458.0 ns)
   2409.1 |#####################
   2411.5 |#############################
   2413.9 |#############################
   2416.4 |########################################
   2418.8 |###########################
   2421.3 |#################################
   2423.7 |#########################
   2426.2 |#########################
   2428.6 |############
   2431.1 |##############
   2433.5 |########
   2436.0 |########
   2438.4 |
   2440.9 |##
   2443.3 |######
   2445.8 |
   2448.2 |##
   2450.7 |
   2453.1 |
   2455.6 |
  (13 below, 11 above range)

inc_fetch_add (n=160, range 2090.2-2131.9 ns)
   2090.2 |################
   2092.3 |################
   2094.4 |########################################
   2096.4 |####################################
   2098.5 |########################################
   2100.6 |################################
   2102.7 |########################
   2104.8 |####################################
   2106.9 |############
   2109.0 |######
   2111.0 |##
   2113.1 |######
   2115.2 |
   2117.3 |####
   2119.4 |
   2121.5 |####
   2123.5 |
   2125.6 |##
   2127.7 |
   2129.8 |
  (14 below, 8 above range)

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
      0.4 |
      0.4 |###########################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.7 |
  (0 below, 25 above range)

```

## Diagnostics

- **inc_load_store**: CV=118.4% (high variance, measurements may be unstable)
- **inc_load_store**: bridge=1250.0% of algo (FFI overhead may distort results)
