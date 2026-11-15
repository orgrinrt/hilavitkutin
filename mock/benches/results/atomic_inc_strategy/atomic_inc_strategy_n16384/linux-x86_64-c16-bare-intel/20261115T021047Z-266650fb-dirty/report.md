# Atomic increment strategy: fetch_add vs CAS-loop vs load+store

3 variants, 160 samples per variant.
Baseline: **inc_cas_loop**

## Highlights

Baseline for all deltas below: **inc_cas_loop**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (inc_cas_loop) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline inc_cas_loop has the worst median (146.50 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest inc_load_store at 10 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### inc_load_store dominates: 782481% faster than the next best (inc_fetch_add)

inc_load_store (10 ns) leads inc_fetch_add (82.17 us) by 782481%, a clear separation rather than a photo finish. CV 89.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### inc_load_store beats baseline by 100% (significant)

inc_load_store is -146.48 us (100%) faster than baseline inc_cas_loop, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### inc_cas_loop is an outlier: 13952.1x slower than the field

inc_cas_loop (146.50 us) is 13952.1x the fastest (10 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### inc_load_store is fastest but the noisiest (CV 89.0%)

inc_load_store wins on median (10 ns) yet has the highest variance (CV 89.0%), while inc_cas_loop is the steadiest (CV 8.8%, 146.50 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 13952.1x the fastest

Fastest inc_load_store (10 ns) to slowest inc_cas_loop (146.50 us): 13952.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### inc_load_store is inconsistent: worst-20% is 3.8x its best-20%

inc_load_store's best 20% of batches run at 9 ns but its worst 20% at 33 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: inc_load_store** at 10.5 ns median (-100.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 13952.07x (fastest 10.5 ns, slowest 146496.7 ns)

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
| inc_cas_loop | 138305ns | 148397ns | 121393ns | 140546ns | 148495ns | base |
| inc_fetch_add | 78390ns | 84059ns | 68767ns | 79620ns | 84324ns | -43.32% |
| inc_load_store | 2683ns | 1888ns | 1559ns | 1927ns | 6077ns | -98.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inc_cas_loop | 136527ns | 119835ns | 146577ns | base | 0.120 |
| inc_fetch_add | 76625ns | 67222ns | 82418ns | -43.88% | 0.214 |
| inc_load_store | 15ns | 9ns | 33ns | -99.99% | 1122.048 |

## Performance model

- Peak throughput: **1894.789 Gops/s** (inc_load_store; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inc_cas_loop | 0.112 | 0.0% |
| inc_fetch_add | 0.199 | 0.0% |
| inc_load_store | 1560.381 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inc_cas_loop | 138305ns | 138305ns | base |
| inc_fetch_add | 78390ns | 78390ns | -43.32% |
| inc_load_store | 2683ns | 2683ns | -98.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inc_cas_loop | 146497ns | base | --- | [146488, 146501] | --- | --- | --- | --- |
| inc_fetch_add | 82171ns | -64314.8ns (-43.9%) | [-64326, -64293]ns | [82164, 82175] | YES | 0.0000 | 0.0000 | 0 |
| inc_load_store | 10ns | -146478.9ns (-100.0%) | [-146487, -146472]ns | [10, 10] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inc_cas_loop | inc_fetch_add | inc_load_store |
|---|---|---|---|---|
| 1 | 1 | 146492ns | -43.9% | -100.0% |
| 1 | 2 | 119842ns | -31.4% | -100.0% |
| 1 | 3 | 119844ns | -43.9% | -100.0% |
| 1 | 4 | 146502ns | -54.1% | -100.0% |
| 1 | 5 | 119835ns | -31.4% | -100.0% |
| 1 | 6 | 146508ns | -43.9% | -100.0% |
| 1 | 7 | 146511ns | -43.9% | -100.0% |
| 1 | 8 | 146518ns | -54.1% | -100.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inc_cas_loop | -0.141 | ok |
| inc_fetch_add | 0.058 | ok |
| inc_load_store | -0.208 | moderate- |

**Consistency summary:**

- **inc_fetch_add**: won 8/8, lost 0/8
- **inc_load_store**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inc_cas_loop | 22.7ns | 136527.3ns | 0.0% |  |
| inc_fetch_add | 22.2ns | 76625.2ns | 0.0% |  |
| inc_load_store | 33.2ns | 14.6ns | 227.0% | HIGH |

## Distribution (algo ns)

```
inc_cas_loop (n=160, range 119835.3-146577.3 ns)
  119835.3 |###################
  121172.4 |
  122509.5 |
  123846.6 |
  125183.7 |
  126520.8 |
  127857.9 |
  129195.0 |
  130532.1 |
  131869.2 |
  133206.3 |
  134543.4 |
  135880.5 |
  137217.6 |
  138554.7 |
  139891.8 |
  141228.9 |
  142566.0 |
  143903.1 |
  145240.2 |########################################
  (14 below, 6 above range)

inc_fetch_add (n=160, range 67222.4-82417.7 ns)
  67222.4 |####################
  67982.1 |
  68741.9 |
  69501.7 |
  70261.4 |
  71021.2 |
  71781.0 |
  72540.7 |
  73300.5 |
  74060.3 |
  74820.0 |
  75579.8 |
  76339.6 |
  77099.3 |
  77859.1 |
  78618.9 |
  79378.6 |
  80138.4 |
  80898.2 |
  81657.9 |########################################
  (13 below, 6 above range)

inc_load_store (n=160, range 8.6-32.5 ns)
      8.6 |#
      9.8 |########################################
     11.0 |
     12.2 |
     13.4 |
     14.6 |
     15.8 |
     17.0 |
     18.2 |
     19.4 |
     20.6 |
     21.8 |
     23.0 |
     24.2 |
     25.4 |
     26.5 |
     27.7 |
     28.9 |##
     30.1 |
     31.3 |
  (28 below, 17 above range)

```

## Diagnostics

- **inc_load_store**: CV=64.0% (high variance, measurements may be unstable)
- **inc_load_store**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **inc_load_store**: bridge=225.7% of algo (FFI overhead may distort results)
