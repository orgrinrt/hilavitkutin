# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (1.88 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 9 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 12271% faster than the next best (inc_fetch_add)

inc_load_store (9 ns) leads inc_fetch_add (1.06 us) by 12271%, a clear separation rather than a photo finish. CV 110.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -1.87 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 218.5x slower than the field

inc_cas_loop (1.88 us) is 218.5x the fastest (9 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 110.5%)

inc_load_store wins on median (9 ns) yet has the highest variance (CV 110.5%), while inc_cas_loop is the steadiest (CV 20.8%, 1.88 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 218.5x the fastest

Fastest inc_load_store (9 ns) to slowest inc_cas_loop (1.88 us): 218.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.8x its best-20%

inc_load_store's best 20% of batches run at 8 ns but its worst 20% at 32 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 8.6 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 218.55x (fastest 8.6 ns, slowest 1879.5 ns)

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
| Counter frequency | 2112744494 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 3694ns | 3427ns | 3417ns | 3509ns | 4524ns | base |
| inc_fetch_add | 2977ns | 2608ns | 2600ns | 2689ns | 4219ns | -19.40% |
| inc_load_store | 2611ns | 1558ns | 1546ns | 1873ns | 5892ns | -29.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2027ns | 1879ns | 2477ns | base | 0.126 |
| inc_fetch_add | 1212ns | 1064ns | 1703ns | -40.22% | 0.211 |
| inc_load_store | 14ns | 8ns | 32ns | -99.30% | 17.973 |

## Performance model

- Peak throughput: **30.341 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.136 | 0.4% |
| inc_fetch_add | 0.241 | 0.8% |
| inc_load_store | 29.767 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 3694ns | 3694ns | base |
| inc_fetch_add | 2977ns | 2977ns | -19.40% |
| inc_load_store | 2611ns | 2611ns | -29.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 1880ns | base | --- | [1880, 1888] | --- | --- | --- | --- |
| inc_fetch_add | 1064ns | -815.7ns (-43.4%) | [-816, -816]ns | [1064, 1064] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 9ns | -1871.0ns (-99.5%) | [-1871, -1871]ns | [8, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 2297ns | -43.4% | -99.6% |
| 1 | 2 | 1880ns | -43.4% | -99.5% |
| 1 | 3 | 2297ns | -53.7% | -99.6% |
| 1 | 4 | 1879ns | -30.8% | -99.4% |
| 1 | 5 | 1879ns | -43.4% | -99.4% |
| 1 | 6 | 1879ns | -43.4% | -99.5% |
| 1 | 7 | 1879ns | -43.4% | -99.5% |
| 1 | 8 | 1880ns | -43.4% | -99.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.208 | moderate- |
| inc_fetch_add | -0.208 | moderate- |
| inc_load_store | 0.292 | moderate+ |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 21.0ns | 2027.3ns | 1.0% |  |
| inc_fetch_add | 22.2ns | 1211.8ns | 1.8% |  |
| inc_load_store | 32.3ns | 14.2ns | 226.7% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 1879.2-2477.2 ns)
   1879.2 |########################################
   1909.1 |
   1939.0 |
   1968.9 |
   1998.8 |
   2028.7 |
   2058.6 |
   2088.5 |
   2118.4 |
   2148.3 |
   2178.2 |
   2208.1 |
   2238.0 |
   2267.9 |###########
   2297.8 |####
   2327.7 |
   2357.6 |
   2387.5 |
   2417.4 |
   2447.3 |
  (18 below, 2 above range)

inc_fetch_add (n=160, range 1063.6-1702.6 ns)
   1063.6 |########################################
   1095.5 |
   1127.5 |
   1159.4 |
   1191.4 |
   1223.3 |
   1255.3 |
   1287.2 |#############
   1319.2 |
   1351.1 |
   1383.1 |
   1415.0 |
   1447.0 |
   1478.9 |
   1510.9 |
   1542.8 |
   1574.8 |
   1606.7 |
   1638.7 |
   1670.6 |
  (8 below, 8 above range)

inc_load_store (n=160, range 8.4-31.9 ns)
      8.4 |########################################
      9.6 |################
     10.8 |#
     12.0 |
     13.1 |
     14.3 |
     15.5 |
     16.7 |
     17.8 |
     19.0 |
     20.2 |
     21.3 |
     22.5 |
     23.7 |#
     24.9 |#
     26.0 |##
     27.2 |
     28.4 |###
     29.6 |
     30.7 |###
  (20 below, 14 above range)

```

## Diagnostics

- **inc_fetch_add**: CV=34.1% (high variance, measurements may be unstable)
- **inc_load_store**: CV=66.7% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **inc_load_store**: bridge=225.0% of algo (FFI overhead may distort results)
