# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (2.40 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 0 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -2.40 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_fetch_add shows alternating (throttle bounce) (autocorr -0.61)

inc_fetch_add's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 0.0% of the fastest

All 3 variants sit between 0 ns and 2.40 us - a 0.0% band - though some paired differences are still significant.

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
| inc_cas_loop | 4511ns | 4508ns | 4446ns | 4501ns | 4602ns | base |
| inc_fetch_add | 4201ns | 4218ns | 4144ns | 4200ns | 4263ns | -6.85% |
| inc_load_store | 2132ns | 2133ns | 2111ns | 2133ns | 2151ns | -52.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2398ns | 2363ns | 2444ns | base | 0.427 |
| inc_fetch_add | 2078ns | 2049ns | 2108ns | -13.34% | 0.493 |
| inc_load_store | 0ns | 0ns | 1ns | -99.99% | 3690.090 |

## Performance model

- Peak throughput: **0.500 Gops/s** (inc_fetch_add; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.427 | 85.4% |
| inc_fetch_add | 0.491 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 4511ns | 4511ns | base |
| inc_fetch_add | 4201ns | 4201ns | -6.85% |
| inc_load_store | 2132ns | 2132ns | -52.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 2400ns | base | --- | [2377, 2408] | --- | --- | --- | --- |
| inc_fetch_add | 2086ns | -319.0ns (-13.3%) | [-322, -318]ns | [2080, 2089] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 0ns | -2399.4ns (-100.0%) | [-2408, -2377]ns | [0, 0] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 2369ns | -11.7% | -100.0% |
| 1 | 2 | 2376ns | -13.5% | -100.0% |
| 1 | 3 | 2417ns | -13.5% | -100.0% |
| 1 | 4 | 2370ns | -13.3% | -100.0% |
| 1 | 5 | 2411ns | -13.2% | -100.0% |
| 1 | 6 | 2396ns | -13.0% | -100.0% |
| 1 | 7 | 2374ns | -12.2% | -100.0% |
| 1 | 8 | 2417ns | -14.0% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.449 | moderate- |
| inc_fetch_add | -0.614 | HIGH- (thermal bounce) |
| inc_load_store | 0.187 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 2.9ns | 2397.8ns | 0.1% |  |
| inc_fetch_add | 4.8ns | 2077.9ns | 0.2% |  |
| inc_load_store | 3.3ns | 0.3ns | 1196.8% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 2363.3-2443.5 ns)
   2363.3 |####################
   2367.3 |########################################
   2371.3 |########
   2375.3 |########
   2379.3 |##
   2383.3 |#
   2387.3 |##
   2391.4 |#
   2395.4 |####
   2399.4 |######
   2403.4 |########
   2407.4 |#############
   2411.4 |###################
   2415.4 |########
   2419.4 |#################
   2423.5 |###############
   2427.5 |##
   2431.5 |##
   2435.5 |#
   2439.5 |
  (14 below, 11 above range)

inc_fetch_add (n=160, range 2049.1-2107.8 ns)
   2049.1 |##############################
   2052.0 |####################################
   2055.0 |###################
   2057.9 |#######
   2060.8 |#
   2063.8 |###
   2066.7 |
   2069.7 |
   2072.6 |###
   2075.5 |#
   2078.5 |###
   2081.4 |#####
   2084.3 |##########################
   2087.3 |################################
   2090.2 |########################################
   2093.2 |####################
   2096.1 |#############
   2099.0 |#######
   2102.0 |#######
   2104.9 |
  (12 below, 10 above range)

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
      0.4 |####################
      0.4 |
      0.5 |
      0.5 |
      0.6 |
      0.6 |
      0.6 |
      0.7 |
      0.7 |
      0.8 |###########
      0.8 |
  (0 below, 6 above range)

```

## Diagnostics

- **inc_load_store**: CV=124.8% (high variance, measurements may be unstable)
