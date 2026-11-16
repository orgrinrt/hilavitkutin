# replace_value blob install cost across value sizes

3 variants, 160 samples per variant.
Baseline: **sc_v1k**

## Highlights

Baseline for all deltas below: **sc_v1k**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sc_v64b dominates: 3994% faster than the next best (sc_v1k)

sc_v64b (2 ns) leads sc_v1k (70 ns) by 3994%, a clear separation rather than a photo finish. CV 52.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sc_v64b beats baseline by 97% (significant)

sc_v64b is -68 ns (97%) faster than baseline sc_v1k, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sc_v64k is an outlier: 985.4x slower than the field

sc_v64k (1.68 us) is 985.4x the fastest (2 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sc_v64b is fastest but the noisiest (CV 52.4%)

sc_v64b wins on median (2 ns) yet has the highest variance (CV 52.4%), while sc_v64k is the steadiest (CV 3.3%, 1.68 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 985.4x the fastest

Fastest sc_v64b (2 ns) to slowest sc_v64k (1.68 us): 985.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sc_v64b is inconsistent: worst-20% is 4.3x its best-20%

sc_v64b's best 20% of batches run at 1 ns but its worst 20% at 3 ns (4.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sc_v64b** at 1.7 ns median (-97.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 985.41x (fastest 1.7 ns, slowest 1675.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | replace_value blob install cost across value sizes |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sc_v1k | 2292ns | 2192ns | 2168ns | 2200ns | 2691ns | base |
| sc_v64b | 2152ns | 2139ns | 2130ns | 2141ns | 2209ns | -6.09% |
| sc_v64k | 3850ns | 3823ns | 3777ns | 3829ns | 3985ns | +68.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sc_v1k | 70ns | 66ns | 77ns | base | 0.911 |
| sc_v64b | 2ns | 1ns | 3ns | -97.30% | 33.751 |
| sc_v64k | 1694ns | 1644ns | 1783ns | +2311.76% | 0.038 |

## Performance model

- Peak throughput: **85.333 Gops/s** (sc_v64b; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sc_v1k | 0.920 | 1.1% |
| sc_v64b | 37.647 | 44.1% |
| sc_v64k | 0.038 | 0.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sc_v1k | 2292ns | 2292ns | base |
| sc_v64b | 2152ns | 2152ns | -6.09% |
| sc_v64k | 3850ns | 3850ns | +68.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sc_v1k | 70ns | base | --- | [69, 70] | --- | --- | --- | --- |
| sc_v64b | 2ns | -67.5ns (-97.0%) | [-68, -67]ns | [2, 2] | YES | 0.0000 | 0.0000 | 0 |
| sc_v64k | 1675ns | +1603.3ns (+2303.6%) | [+1594, +1619]ns | [1665, 1688] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sc_v1k | sc_v64b | sc_v64k |
|---|---|---|---|---|
| 1 | 1 | 68ns | -97.2% | +2331.5% |
| 1 | 2 | 68ns | -97.5% | +2365.9% |
| 1 | 3 | 72ns | -98.3% | +2216.0% |
| 1 | 4 | 69ns | -97.5% | +2307.6% |
| 1 | 5 | 69ns | -96.9% | +2323.3% |
| 1 | 6 | 69ns | -96.9% | +2400.6% |
| 1 | 7 | 69ns | -97.5% | +2343.3% |
| 1 | 8 | 71ns | -97.1% | +2282.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sc_v1k | -0.170 | ok |
| sc_v64b | 0.170 | ok |
| sc_v64k | -0.157 | ok |

**Consistency summary:**

- **sc_v64b**: won 8/8, lost 0/8
- **sc_v64k**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sc_v1k | 43.6ns | 70.3ns | 62.1% | HIGH |
| sc_v64b | 45.8ns | 1.9ns | 2415.3% | HIGH |
| sc_v64k | 41.8ns | 1694.3ns | 2.5% |  |

## Distribution (algo ns)

```
sc_v1k (n=160, range 65.6-76.9 ns)
     65.6 |######
     66.2 |###########################
     66.8 |#########################
     67.3 |##############
     67.9 |###############################
     68.5 |##########
     69.0 |##################
     69.6 |###########################
     70.2 |########
     70.7 |########################################
     71.3 |#########################
     71.9 |################
     72.4 |#########################
     73.0 |####
     73.5 |##########
     74.1 |####
     74.7 |
     75.2 |####
     75.8 |
     76.4 |
  (13 below, 4 above range)

sc_v64b (n=160, range 0.8-3.2 ns)
      0.8 |##################
      0.9 |
      1.0 |
      1.1 |##########################
      1.2 |
      1.4 |
      1.5 |
      1.6 |########################################
      1.7 |
      1.8 |
      2.0 |
      2.1 |########################################
      2.2 |
      2.3 |
      2.5 |######################
      2.6 |
      2.7 |
      2.8 |########
      2.9 |
      3.1 |
  (9 below, 15 above range)

sc_v64k (n=160, range 1644.3-1782.5 ns)
   1644.3 |########################################
   1651.2 |#######################
   1658.1 |############################
   1665.0 |###############
   1671.9 |###########
   1678.8 |##########
   1685.7 |###############
   1692.7 |#####
   1699.6 |###
   1706.5 |###############
   1713.4 |######
   1720.3 |###########
   1727.2 |#####
   1734.1 |###########
   1741.1 |###
   1748.0 |######
   1754.9 |###
   1761.8 |###
   1768.7 |#####
   1775.6 |#####
  (13 below, 10 above range)

```

## Diagnostics

- **sc_v1k**: bridge=60.5% of algo (FFI overhead may distort results)
- **sc_v64b**: CV=47.0% (high variance, measurements may be unstable)
- **sc_v64b**: worst_20/best_20 = 4.3x (possible bimodal distribution)
- **sc_v64b**: bridge=2670.6% of algo (FFI overhead may distort results)
