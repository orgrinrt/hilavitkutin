# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (119.86 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 11 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 634264% faster than the next best (inc_fetch_add)

inc_load_store (11 ns) leads inc_fetch_add (67.24 us) by 634264%, a clear separation rather than a photo finish. CV 86.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -119.85 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 11307.7x slower than the field

inc_cas_loop (119.86 us) is 11307.7x the fastest (11 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 86.7%)

inc_load_store wins on median (11 ns) yet has the highest variance (CV 86.7%), while inc_fetch_add is the steadiest (CV 9.7%, 67.24 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 11307.7x the fastest

Fastest inc_load_store (11 ns) to slowest inc_cas_loop (119.86 us): 11307.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.4x its best-20%

inc_load_store's best 20% of batches run at 10 ns but its worst 20% at 33 ns (3.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.6 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 11307.66x (fastest 10.6 ns, slowest 119861.1 ns)

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
| inc_cas_loop | 129144ns | 121423ns | 121403ns | 123776ns | 152992ns | base |
| inc_fetch_add | 72645ns | 68790ns | 68771ns | 70095ns | 84169ns | -43.75% |
| inc_load_store | 2765ns | 1897ns | 1743ns | 2038ns | 5966ns | -97.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 127478ns | 119845ns | 151009ns | base | 0.129 |
| inc_fetch_add | 71006ns | 67225ns | 82266ns | -44.30% | 0.231 |
| inc_load_store | 15ns | 10ns | 33ns | -99.99% | 1070.281 |

## Performance model

- Peak throughput: **1692.343 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.137 | 0.0% |
| inc_fetch_add | 0.244 | 0.0% |
| inc_load_store | 1545.660 | 91.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 129144ns | 129144ns | base |
| inc_fetch_add | 72645ns | 72645ns | -43.75% |
| inc_load_store | 2765ns | 2765ns | -97.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 119861ns | base | --- | [119856, 119866] | --- | --- | --- | --- |
| inc_fetch_add | 67243ns | -52619.9ns (-43.9%) | [-52627, -52615]ns | [67236, 67246] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 11ns | -119846.2ns (-100.0%) | [-119853, -119843]ns | [11, 11] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 119858ns | -43.9% | -100.0% |
| 1 | 2 | 119859ns | -43.9% | -100.0% |
| 1 | 3 | 119846ns | -43.9% | -100.0% |
| 1 | 4 | 119863ns | -31.4% | -100.0% |
| 1 | 5 | 119850ns | -43.9% | -100.0% |
| 1 | 6 | 119848ns | -43.9% | -100.0% |
| 1 | 7 | 146533ns | -43.9% | -100.0% |
| 1 | 8 | 146537ns | -54.1% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | 0.458 | moderate+ |
| inc_fetch_add | -0.375 | moderate- |
| inc_load_store | -0.025 | ok |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 21.4ns | 127478.2ns | 0.0% |  |
| inc_fetch_add | 20.7ns | 71006.2ns | 0.0% |  |
| inc_load_store | 34.8ns | 15.3ns | 227.5% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 119844.8-151009.1 ns)
  119844.8 |########################################
  121403.0 |
  122961.2 |
  124519.4 |
  126077.6 |
  127635.8 |
  129194.1 |
  130752.3 |
  132310.5 |
  133868.7 |
  135426.9 |
  136985.1 |
  138543.3 |
  140101.6 |
  141659.8 |
  143218.0 |
  144776.2 |
  146334.4 |##############
  147892.6 |
  149450.8 |
  (12 below, 1 above range)

inc_fetch_add (n=160, range 67224.7-82265.9 ns)
  67224.7 |########################################
  67976.8 |
  68728.8 |
  69480.9 |
  70232.9 |
  70985.0 |
  71737.1 |
  72489.1 |
  73241.2 |
  73993.3 |
  74745.3 |
  75497.4 |
  76249.4 |
  77001.5 |
  77753.6 |
  78505.6 |
  79257.7 |
  80009.7 |
  80761.8 |
  81513.9 |###############
  (14 below, 1 above range)

inc_load_store (n=160, range 9.7-32.9 ns)
      9.7 |########################################
     10.8 |
     12.0 |
     13.2 |
     14.3 |
     15.5 |
     16.6 |
     17.8 |
     19.0 |
     20.1 |
     21.3 |
     22.4 |
     23.6 |
     24.7 |
     25.9 |
     27.1 |
     28.2 |#
     29.4 |#
     30.5 |#
     31.7 |#
  (14 below, 16 above range)

```

## Diagnostics

- **inc_load_store**: CV=60.0% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.4x (possible bimodal distribution)
- **inc_load_store**: bridge=225.5% of algo (FFI overhead may distort results)
