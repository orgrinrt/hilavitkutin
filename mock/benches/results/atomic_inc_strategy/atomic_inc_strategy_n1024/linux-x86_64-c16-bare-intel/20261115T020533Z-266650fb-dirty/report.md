# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (8.34 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 45173% faster than the next best (inc_fetch_add)

inc_load_store (10 ns) leads inc_fetch_add (4.69 us) by 45173%, a clear separation rather than a photo finish. CV 90.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -8.31 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 805.3x slower than the field

inc_cas_loop (8.34 us) is 805.3x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 90.8%)

inc_load_store wins on median (10 ns) yet has the highest variance (CV 90.8%), while inc_cas_loop is the steadiest (CV 10.0%, 8.34 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 805.3x the fastest

Fastest inc_load_store (10 ns) to slowest inc_cas_loop (8.34 us): 805.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.8x its best-20%

inc_load_store's best 20% of batches run at 9 ns but its worst 20% at 33 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.4 ns median (-99.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 805.34x (fastest 10.4 ns, slowest 8335.2 ns)

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
| inc_cas_loop | 10055ns | 10088ns | 9041ns | 10053ns | 11074ns | base |
| inc_fetch_add | 6406ns | 6440ns | 5749ns | 6399ns | 7084ns | -36.28% |
| inc_load_store | 2758ns | 1893ns | 1579ns | 2041ns | 6087ns | -72.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 8338ns | 7499ns | 9178ns | base | 0.123 |
| inc_fetch_add | 4689ns | 4211ns | 5182ns | -43.77% | 0.218 |
| inc_load_store | 15ns | 9ns | 33ns | -99.82% | 68.346 |

## Performance model

- Peak throughput: **118.639 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.123 | 0.1% |
| inc_fetch_add | 0.219 | 0.2% |
| inc_load_store | 98.937 | 83.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 10055ns | 10055ns | base |
| inc_fetch_add | 6406ns | 6406ns | -36.28% |
| inc_load_store | 2758ns | 2758ns | -72.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 8335ns | base | --- | [7506, 9162] | --- | --- | --- | --- |
| inc_fetch_add | 4686ns | -3509.3ns (-42.1%) | [-4013, -3293]ns | [4221, 5147] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 10ns | -8311.0ns (-99.7%) | [-9141, -7497]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 7505ns | -31.4% | -99.9% |
| 1 | 2 | 9173ns | -54.1% | -99.9% |
| 1 | 3 | 9172ns | -54.1% | -99.9% |
| 1 | 4 | 9173ns | -43.8% | -99.9% |
| 1 | 5 | 7505ns | -31.4% | -99.9% |
| 1 | 6 | 9172ns | -43.8% | -99.9% |
| 1 | 7 | 7504ns | -43.9% | -99.9% |
| 1 | 8 | 7505ns | -43.8% | -99.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.125 | ok |
| inc_fetch_add | 0.123 | ok |
| inc_load_store | -0.207 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 21.7ns | 8337.7ns | 0.3% |  |
| inc_fetch_add | 21.7ns | 4688.6ns | 0.5% |  |
| inc_load_store | 34.1ns | 15.0ns | 227.6% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 7499.4-9178.1 ns)
   7499.4 |#################################
   7583.3 |
   7667.3 |
   7751.2 |
   7835.1 |
   7919.1 |
   8003.0 |
   8087.0 |
   8170.9 |
   8254.8 |
   8338.8 |
   8422.7 |
   8506.6 |
   8590.6 |
   8674.5 |
   8758.4 |
   8842.4 |
   8926.3 |
   9010.2 |
   9094.2 |########################################
  (19 below, 8 above range)

inc_fetch_add (n=160, range 4211.3-5181.6 ns)
   4211.3 |####################################
   4259.8 |
   4308.3 |
   4356.8 |
   4405.4 |
   4453.9 |
   4502.4 |
   4550.9 |
   4599.4 |
   4647.9 |
   4696.5 |
   4745.0 |
   4793.5 |
   4842.0 |
   4890.5 |
   4939.0 |
   4987.6 |
   5036.1 |
   5084.6 |
   5133.1 |########################################
  (11 below, 4 above range)

inc_load_store (n=160, range 8.6-32.8 ns)
      8.6 |
      9.8 |########################################
     11.0 |
     12.3 |
     13.5 |
     14.7 |
     15.9 |
     17.1 |
     18.3 |
     19.5 |
     20.7 |
     21.9 |
     23.1 |
     24.3 |
     25.5 |#
     26.7 |
     27.9 |##
     29.2 |#
     30.4 |##
     31.6 |
  (29 below, 16 above range)

```

## Diagnostics

- **inc_load_store**: CV=62.7% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **inc_load_store**: bridge=227.1% of algo (FFI overhead may distort results)
