# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 618% faster than the next best (popcount_handrolled)

popcount_std (271 ns) leads popcount_handrolled (1.95 us) by 618%, a clear separation rather than a photo finish. CV 1.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 86% (significant)

popcount_std is -1.68 us (86%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 7.2x slower than the field

popcount_swar (1.95 us) is 7.2x the fastest (271 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 7.2x the fastest

Fastest popcount_std (271 ns) to slowest popcount_swar (1.95 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (1 ns, 0.06%)

popcount_swar differs from baseline popcount_handrolled by 1 ns (0.06%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 271.2 ns median (-86.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.18x (fastest 271.2 ns, slowest 1948.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Popcount lowering: std count_ones vs Kernighan loop vs SWAR |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x24680123456789ab |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| popcount_handrolled | 4033ns | 4019ns | 4014ns | 4021ns | 4090ns | base |
| popcount_std | 2350ns | 2345ns | 2338ns | 2345ns | 2377ns | -41.73% |
| popcount_swar | 4052ns | 4021ns | 4014ns | 4023ns | 4178ns | +0.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 1953ns | 1941ns | 1982ns | base | 8.388 |
| popcount_std | 272ns | 265ns | 279ns | -86.10% | 60.326 |
| popcount_swar | 1961ns | 1941ns | 2018ns | +0.40% | 8.354 |

## Performance model

- Peak throughput: **61.861 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 8.415 | 13.6% |
| popcount_std | 60.413 | 97.7% |
| popcount_swar | 8.410 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 4033ns | 4033ns | base |
| popcount_std | 2350ns | 2350ns | -41.73% |
| popcount_swar | 4052ns | 4052ns | +0.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 1947ns | base | --- | [1946, 1949] | --- | --- | --- | --- |
| popcount_std | 271ns | -1675.7ns (-86.1%) | [-1677, -1675]ns | [270, 272] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 1948ns | +1.2ns (+0.1%) | [+0, +2]ns | [1947, 1950] | YES | 0.0022 | 0.0022 | 5 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1981ns | -86.4% | +2.5% |
| 1 | 2 | 1946ns | -86.1% | +0.0% |
| 1 | 3 | 1946ns | -85.9% | -0.0% |
| 1 | 4 | 1945ns | -86.1% | +0.0% |
| 1 | 5 | 1946ns | -86.1% | +0.0% |
| 1 | 6 | 1946ns | -86.1% | +0.2% |
| 1 | 7 | 1947ns | -86.0% | +0.1% |
| 1 | 8 | 1946ns | -86.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.012 | ok |
| popcount_std | -0.108 | ok |
| popcount_swar | -0.025 | ok |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 0/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.1ns | 1953.3ns | 0.2% |  |
| popcount_std | 2.8ns | 271.6ns | 1.0% |  |
| popcount_swar | 3.1ns | 1961.2ns | 0.2% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1940.8-1982.4 ns)
   1940.8 |########################################
   1942.8 |####################
   1944.9 |################################
   1947.0 |############################
   1949.1 |#################
   1951.2 |##########
   1953.3 |########
   1955.4 |####
   1957.4 |#####
   1959.5 |#
   1961.6 |##
   1963.7 |
   1965.8 |
   1967.9 |
   1969.9 |#
   1972.0 |#
   1974.1 |##
   1976.2 |####
   1978.3 |#####
   1980.4 |##
  (12 below, 15 above range)

popcount_std (n=160, range 264.8-279.0 ns)
    264.8 |#########
    265.6 |##############################
    266.3 |#########
    267.0 |###############
    267.7 |#####################
    268.4 |#####################
    269.1 |###########################
    269.8 |########################################
    270.5 |########################
    271.2 |#####################
    271.9 |########################
    272.7 |#################################
    273.4 |############
    274.1 |###########################
    274.8 |########################
    275.5 |########################
    276.2 |######
    276.9 |############
    277.6 |#########
    278.3 |######
  (16 below, 13 above range)

popcount_swar (n=160, range 1940.9-2017.9 ns)
   1940.9 |################################
   1944.7 |########################################
   1948.6 |########################
   1952.4 |################
   1956.3 |######
   1960.1 |###
   1964.0 |#
   1967.8 |#
   1971.7 |#####
   1975.5 |#
   1979.4 |
   1983.2 |
   1987.1 |
   1990.9 |#
   1994.8 |#
   1998.6 |
   2002.5 |#
   2006.3 |
   2010.2 |#
   2014.0 |#
  (19 below, 17 above range)

```
