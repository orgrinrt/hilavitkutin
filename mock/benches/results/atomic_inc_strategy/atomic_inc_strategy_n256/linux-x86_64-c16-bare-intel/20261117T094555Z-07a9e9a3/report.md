# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (1.88 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 8 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 15193% faster than the next best (inc_fetch_add)

inc_load_store (8 ns) leads inc_fetch_add (1.30 us) by 15193%, a clear separation rather than a photo finish. CV 108.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -1.87 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 221.1x slower than the field

inc_cas_loop (1.88 us) is 221.1x the fastest (8 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 108.2%)

inc_load_store wins on median (8 ns) yet has the highest variance (CV 108.2%), while inc_cas_loop is the steadiest (CV 21.5%, 1.88 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 221.1x the fastest

Fastest inc_load_store (8 ns) to slowest inc_cas_loop (1.88 us): 221.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.6x its best-20%

inc_load_store's best 20% of batches run at 8 ns but its worst 20% at 30 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 8.5 ns median (-99.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 221.13x (fastest 8.5 ns, slowest 1879.6 ns)

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
| Counter frequency | 2112789647 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inc_cas_loop | 3731ns | 3429ns | 3419ns | 3523ns | 4665ns | base |
| inc_fetch_add | 3316ns | 3180ns | 2604ns | 2942ns | 5152ns | -11.11% |
| inc_load_store | 2648ns | 1554ns | 1547ns | 2016ns | 5645ns | -29.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 2046ns | 1879ns | 2546ns | base | 0.125 |
| inc_fetch_add | 1351ns | 1064ns | 2092ns | -33.94% | 0.189 |
| inc_load_store | 14ns | 8ns | 30ns | -99.30% | 17.798 |

## Performance model

- Peak throughput: **30.397 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.136 | 0.4% |
| inc_fetch_add | 0.197 | 0.6% |
| inc_load_store | 30.118 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 3731ns | 3731ns | base |
| inc_fetch_add | 3316ns | 3316ns | -11.11% |
| inc_load_store | 2648ns | 2648ns | -29.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 1880ns | base | --- | [1880, 1889] | --- | --- | --- | --- |
| inc_fetch_add | 1300ns | -815.4ns (-43.4%) | [-816, -644]ns | [1074, 1300] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 8ns | -1871.0ns (-99.5%) | [-1871, -1871]ns | [8, 9] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 1880ns | -43.4% | -99.4% |
| 1 | 2 | 1879ns | -30.8% | -99.5% |
| 1 | 3 | 1880ns | -30.8% | -99.5% |
| 1 | 4 | 2297ns | -53.7% | -99.6% |
| 1 | 5 | 1880ns | -43.4% | -99.5% |
| 1 | 6 | 1880ns | -30.8% | -99.5% |
| 1 | 7 | 2297ns | -43.4% | -99.6% |
| 1 | 8 | 1879ns | -43.4% | -99.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.375 | moderate- |
| inc_fetch_add | -0.125 | ok |
| inc_load_store | -0.018 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 21.3ns | 2045.7ns | 1.0% |  |
| inc_fetch_add | 24.7ns | 1351.3ns | 1.8% |  |
| inc_load_store | 33.0ns | 14.4ns | 229.4% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 1879.2-2545.8 ns)
   1879.2 |########################################
   1912.5 |
   1945.9 |
   1979.2 |
   2012.5 |
   2045.9 |
   2079.2 |
   2112.5 |
   2145.8 |
   2179.2 |
   2212.5 |
   2245.8 |
   2279.2 |###############
   2312.5 |
   2345.8 |
   2379.1 |
   2412.5 |
   2445.8 |
   2479.1 |
   2512.5 |
  (20 below, 5 above range)

inc_fetch_add (n=160, range 1063.6-2091.7 ns)
   1063.6 |#################################
   1115.0 |
   1166.4 |
   1217.8 |
   1269.2 |########################################
   1320.6 |
   1372.0 |
   1423.4 |
   1474.8 |
   1526.2 |
   1577.6 |
   1629.0 |
   1680.5 |
   1731.9 |
   1783.3 |
   1834.7 |
   1886.1 |
   1937.5 |
   1988.9 |
   2040.3 |
  (12 below, 11 above range)

inc_load_store (n=160, range 8.4-30.4 ns)
      8.4 |########################################
      9.5 |########
     10.6 |
     11.7 |
     12.8 |
     13.9 |
     15.0 |
     16.1 |
     17.2 |
     18.3 |
     19.4 |
     20.5 |
     21.6 |
     22.7 |
     23.8 |#
     24.9 |#
     26.0 |####
     27.1 |#
     28.2 |####
     29.3 |##
  (25 below, 14 above range)

```

## Diagnostics

- **inc_fetch_add**: CV=44.6% (high variance, measurements may be unstable)
- **inc_load_store**: CV=63.9% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **inc_load_store**: bridge=228.2% of algo (FFI overhead may distort results)
