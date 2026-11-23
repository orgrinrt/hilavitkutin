# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (9.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 49399% faster than the next best (inc_fetch_add)

inc_load_store (10 ns) leads inc_fetch_add (5.15 us) by 49399%, a clear separation rather than a photo finish. CV 91.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -9.16 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 882.0x slower than the field

inc_cas_loop (9.17 us) is 882.0x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 91.2%)

inc_load_store wins on median (10 ns) yet has the highest variance (CV 91.2%), while inc_fetch_add is the steadiest (CV 6.1%, 5.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 882.0x the fastest

Fastest inc_load_store (10 ns) to slowest inc_cas_loop (9.17 us): 882.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.9x its best-20%

inc_load_store's best 20% of batches run at 8 ns but its worst 20% at 33 ns (3.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 881.96x (fastest 10.4 ns, slowest 9172.4 ns)

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
| inc_cas_loop | 10568ns | 11057ns | 9046ns | 10897ns | 11103ns | base |
| inc_fetch_add | 6885ns | 7039ns | 6236ns | 7037ns | 7078ns | -34.85% |
| inc_load_store | 2706ns | 1891ns | 1547ns | 1973ns | 6063ns | -74.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 8761ns | 7503ns | 9196ns | base | 0.117 |
| inc_fetch_add | 5038ns | 4565ns | 5170ns | -42.49% | 0.203 |
| inc_load_store | 15ns | 8ns | 33ns | -99.83% | 69.453 |

## Performance model

- Peak throughput: **121.004 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.112 | 0.1% |
| inc_fetch_add | 0.199 | 0.2% |
| inc_load_store | 98.462 | 81.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 10568ns | 10568ns | base |
| inc_fetch_add | 6885ns | 6885ns | -34.85% |
| inc_load_store | 2706ns | 2706ns | -74.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 9172ns | base | --- | [9172, 9173] | --- | --- | --- | --- |
| inc_fetch_add | 5148ns | -4014.5ns (-43.8%) | [-4015, -4014]ns | [5148, 5158] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 10ns | -9160.2ns (-99.9%) | [-9162, -9151]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 9173ns | -43.8% | -99.9% |
| 1 | 2 | 9173ns | -43.8% | -99.9% |
| 1 | 3 | 7505ns | -43.9% | -99.9% |
| 1 | 4 | 7505ns | -31.3% | -99.9% |
| 1 | 5 | 9172ns | -43.8% | -99.9% |
| 1 | 6 | 9173ns | -43.8% | -99.9% |
| 1 | 7 | 9172ns | -43.8% | -99.9% |
| 1 | 8 | 9174ns | -43.9% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | 0.292 | moderate+ |
| inc_fetch_add | -0.167 | ok |
| inc_load_store | -0.455 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 22.7ns | 8761.3ns | 0.3% |  |
| inc_fetch_add | 23.4ns | 5038.2ns | 0.5% |  |
| inc_load_store | 33.4ns | 14.7ns | 226.7% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 7502.7-9196.3 ns)
   7502.7 |##########
   7587.4 |
   7672.1 |
   7756.7 |
   7841.4 |
   7926.1 |
   8010.8 |
   8095.5 |
   8180.1 |
   8264.8 |
   8349.5 |
   8434.2 |
   8518.9 |
   8603.6 |
   8688.2 |
   8772.9 |
   8857.6 |
   8942.3 |
   9027.0 |
   9111.6 |########################################
  (9 below, 2 above range)

inc_fetch_add (n=160, range 4564.8-5169.9 ns)
   4564.8 |
   4595.1 |
   4625.3 |
   4655.6 |
   4685.9 |
   4716.1 |
   4746.4 |
   4776.6 |
   4806.9 |
   4837.1 |
   4867.4 |
   4897.6 |
   4927.9 |
   4958.1 |
   4988.4 |
   5018.7 |
   5048.9 |
   5079.2 |
   5109.4 |
   5139.7 |########################################
  (20 below, 4 above range)

inc_load_store (n=160, range 8.5-32.8 ns)
      8.5 |#################
      9.7 |########################################
     10.9 |
     12.1 |
     13.3 |
     14.5 |
     15.8 |
     17.0 |
     18.2 |
     19.4 |
     20.6 |
     21.8 |
     23.1 |
     24.3 |#
     25.5 |#
     26.7 |#
     27.9 |
     29.1 |
     30.4 |#
     31.6 |#
  (11 below, 19 above range)

```

## Diagnostics

- **inc_load_store**: CV=64.4% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **inc_load_store**: bridge=226.0% of algo (FFI overhead may distort results)
