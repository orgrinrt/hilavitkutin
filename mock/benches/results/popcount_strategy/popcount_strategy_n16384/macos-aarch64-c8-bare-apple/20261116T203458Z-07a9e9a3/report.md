# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 619% faster than the next best (popcount_handrolled)

popcount_std (271 ns) leads popcount_handrolled (1.95 us) by 619%, a clear separation rather than a photo finish. CV 3.1%.

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

### popcount_swar's edge over baseline is significant but tiny (2 ns, 0.13%)

popcount_swar differs from baseline popcount_handrolled by 2 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 270.8 ns median (-86.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.20x (fastest 270.8 ns, slowest 1948.8 ns)

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
| popcount_handrolled | 4026ns | 4019ns | 4013ns | 4019ns | 4060ns | base |
| popcount_std | 2362ns | 2346ns | 2340ns | 2348ns | 2426ns | -41.34% |
| popcount_swar | 4036ns | 4022ns | 4015ns | 4022ns | 4097ns | +0.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 1949ns | 1941ns | 1963ns | base | 8.408 |
| popcount_std | 273ns | 265ns | 284ns | -86.01% | 60.086 |
| popcount_swar | 1955ns | 1942ns | 1984ns | +0.32% | 8.382 |

## Performance model

- Peak throughput: **61.859 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 8.418 | 13.6% |
| popcount_std | 60.502 | 97.8% |
| popcount_swar | 8.407 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 4026ns | 4026ns | base |
| popcount_std | 2362ns | 2362ns | -41.34% |
| popcount_swar | 4036ns | 4036ns | +0.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 1946ns | base | --- | [1945, 1947] | --- | --- | --- | --- |
| popcount_std | 271ns | -1675.4ns (-86.1%) | [-1676, -1675]ns | [270, 272] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 1949ns | +2.5ns (+0.1%) | [+2, +3]ns | [1948, 1951] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1949ns | -86.2% | +0.1% |
| 1 | 2 | 1946ns | -86.1% | +0.0% |
| 1 | 3 | 1945ns | -86.1% | +0.2% |
| 1 | 4 | 1946ns | -86.1% | +0.0% |
| 1 | 5 | 1945ns | -86.1% | +2.0% |
| 1 | 6 | 1947ns | -86.1% | +0.1% |
| 1 | 7 | 1945ns | -86.1% | +0.0% |
| 1 | 8 | 1946ns | -85.6% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.178 | ok |
| popcount_std | -0.041 | ok |
| popcount_swar | -0.208 | moderate- |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 0/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.3ns | 1948.6ns | 0.2% |  |
| popcount_std | 2.7ns | 272.7ns | 1.0% |  |
| popcount_swar | 3.3ns | 1954.7ns | 0.2% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1940.7-1963.1 ns)
   1940.7 |######################
   1941.8 |##################
   1942.9 |######################
   1944.0 |########################################
   1945.2 |########################################
   1946.3 |##################
   1947.4 |############################
   1948.5 |########################
   1949.7 |######
   1950.8 |########
   1951.9 |##########
   1953.0 |##
   1954.2 |##########
   1955.3 |########
   1956.4 |####
   1957.5 |####
   1958.7 |##
   1959.8 |######
   1960.9 |
   1962.0 |
  (15 below, 9 above range)

popcount_std (n=160, range 264.9-284.1 ns)
    264.9 |################################
    265.8 |######################
    266.8 |#################
    267.8 |####################
    268.7 |########################################
    269.7 |#########################
    270.6 |#################
    271.6 |########################################
    272.6 |#################
    273.5 |#######
    274.5 |######################
    275.5 |#######
    276.4 |############
    277.4 |#########################
    278.4 |##########
    279.3 |############
    280.3 |##########
    281.3 |##
    282.2 |##
    283.2 |##
  (12 below, 9 above range)

popcount_swar (n=160, range 1942.0-1983.5 ns)
   1942.0 |######################
   1944.1 |########################################
   1946.2 |###################################
   1948.3 |######################
   1950.3 |################################
   1952.4 |#################
   1954.5 |############
   1956.6 |#########
   1958.6 |####
   1960.7 |####
   1962.8 |####
   1964.9 |
   1966.9 |###
   1969.0 |
   1971.1 |#
   1973.2 |###
   1975.2 |
   1977.3 |
   1979.4 |
   1981.5 |
  (13 below, 13 above range)

```
