# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (1.88 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 8 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 12416% faster than the next best (inc_fetch_add)

inc_load_store (8 ns) leads inc_fetch_add (1.06 us) by 12416%, a clear separation rather than a photo finish. CV 105.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -1.87 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 221.1x slower than the field

inc_cas_loop (1.88 us) is 221.1x the fastest (8 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 105.1%)

inc_load_store wins on median (8 ns) yet has the highest variance (CV 105.1%), while inc_cas_loop is the steadiest (CV 27.9%, 1.88 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 221.1x the fastest

Fastest inc_load_store (8 ns) to slowest inc_cas_loop (1.88 us): 221.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.5x its best-20%

inc_load_store's best 20% of batches run at 8 ns but its worst 20% at 29 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 8.5 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 221.12x (fastest 8.5 ns, slowest 1879.5 ns)

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
| Counter frequency | 2112741386 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 3971ns | 3427ns | 3415ns | 3692ns | 5361ns | base |
| inc_fetch_add | 3298ns | 2609ns | 2600ns | 2762ns | 5605ns | -16.93% |
| inc_load_store | 2746ns | 1553ns | 1542ns | 2271ns | 5376ns | -30.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2073ns | 1818ns | 2706ns | base | 0.124 |
| inc_fetch_add | 1344ns | 1064ns | 2272ns | -35.14% | 0.190 |
| inc_load_store | 15ns | 8ns | 29ns | -99.28% | 17.113 |

## Performance model

- Peak throughput: **30.522 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.136 | 0.4% |
| inc_fetch_add | 0.241 | 0.8% |
| inc_load_store | 30.118 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 3971ns | 3971ns | base |
| inc_fetch_add | 3298ns | 3298ns | -16.93% |
| inc_load_store | 2746ns | 2746ns | -30.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 1880ns | base | --- | [1880, 1880] | --- | --- | --- | --- |
| inc_fetch_add | 1064ns | -815.7ns (-43.4%) | [-816, -816]ns | [1064, 1064] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 8ns | -1870.9ns (-99.5%) | [-1871, -1871]ns | [8, 9] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 1880ns | -43.4% | -99.5% |
| 1 | 2 | 1774ns | -40.0% | -99.5% |
| 1 | 3 | 1880ns | -43.4% | -99.5% |
| 1 | 4 | 2297ns | -53.7% | -99.6% |
| 1 | 5 | 1879ns | -30.8% | -99.5% |
| 1 | 6 | 2297ns | -43.4% | -99.6% |
| 1 | 7 | 1879ns | -43.4% | -99.4% |
| 1 | 8 | 1879ns | -43.4% | -99.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.255 | moderate- |
| inc_fetch_add | 0.292 | moderate+ |
| inc_load_store | -0.161 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 23.9ns | 2072.7ns | 1.2% |  |
| inc_fetch_add | 24.7ns | 1344.3ns | 1.8% |  |
| inc_load_store | 34.1ns | 15.0ns | 228.0% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 1818.1-2705.9 ns)
   1818.1 |
   1862.5 |########################################
   1906.9 |
   1951.3 |
   1995.7 |
   2040.1 |
   2084.5 |
   2128.9 |
   2173.3 |
   2217.7 |
   2262.0 |#############
   2306.4 |###
   2350.8 |
   2395.2 |
   2439.6 |
   2484.0 |
   2528.4 |
   2572.8 |
   2617.2 |
   2661.6 |
  (19 below, 6 above range)

inc_fetch_add (n=160, range 1063.5-2272.5 ns)
   1063.5 |########################################
   1124.0 |
   1184.4 |
   1244.9 |#################
   1305.3 |#
   1365.8 |
   1426.2 |
   1486.7 |
   1547.1 |
   1607.6 |
   1668.0 |
   1728.5 |
   1788.9 |
   1849.3 |
   1909.8 |
   1970.2 |
   2030.7 |
   2091.1 |
   2151.6 |
   2212.0 |
  (21 below, 16 above range)

inc_load_store (n=160, range 8.4-29.1 ns)
      8.4 |########################################
      9.4 |#####
     10.5 |
     11.5 |
     12.5 |
     13.6 |
     14.6 |
     15.6 |
     16.7 |
     17.7 |
     18.8 |
     19.8 |
     20.8 |
     21.9 |#
     22.9 |#
     23.9 |##
     25.0 |
     26.0 |######
     27.1 |#
     28.1 |###
  (4 below, 15 above range)

```

## Diagnostics

- **inc_cas_loop**: CV=25.3% (high variance, measurements may be unstable)
- **inc_fetch_add**: CV=47.9% (high variance, measurements may be unstable)
- **inc_load_store**: CV=59.7% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **inc_load_store**: bridge=227.1% of algo (FFI overhead may distort results)
