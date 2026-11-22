# Popcount lowering: std count_ones vs Kernighan loop vs SWAR

3 variants, 160 samples per variant.
Baseline: **popcount_handrolled**

## Highlights

Baseline for all deltas below: **popcount_handrolled**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### popcount_std dominates: 619% faster than the next best (popcount_handrolled)

popcount_std (276 ns) leads popcount_handrolled (1.99 us) by 619%, a clear separation rather than a photo finish. CV 1.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### popcount_std beats baseline by 86% (significant)

popcount_std is -1.71 us (86%) faster than baseline popcount_handrolled, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### popcount_swar is an outlier: 7.2x slower than the field

popcount_swar (1.99 us) is 7.2x the fastest (276 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### popcount_swar shows alternating (throttle bounce) (autocorr -0.52)

popcount_swar's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 7.2x the fastest

Fastest popcount_std (276 ns) to slowest popcount_swar (1.99 us): 7.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### popcount_swar's edge over baseline is significant but tiny (2 ns, 0.13%)

popcount_swar differs from baseline popcount_handrolled by 2 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: popcount_std** at 276.2 ns median (-86.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 7.20x (fastest 276.2 ns, slowest 1987.9 ns)

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
| popcount_handrolled | 4107ns | 4089ns | 4078ns | 4092ns | 4178ns | base |
| popcount_std | 2381ns | 2379ns | 2371ns | 2379ns | 2394ns | -42.03% |
| popcount_swar | 4097ns | 4090ns | 4083ns | 4090ns | 4131ns | -0.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| popcount_handrolled | 1995ns | 1979ns | 2037ns | base | 8.211 |
| popcount_std | 277ns | 271ns | 285ns | -86.11% | 59.133 |
| popcount_swar | 1991ns | 1980ns | 2014ns | -0.19% | 8.227 |

## Performance model

- Peak throughput: **60.479 Gops/s** (popcount_std; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| popcount_handrolled | 8.252 | 13.6% |
| popcount_std | 59.319 | 98.1% |
| popcount_swar | 8.242 | 13.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| popcount_handrolled | 4107ns | 4107ns | base |
| popcount_std | 2381ns | 2381ns | -42.03% |
| popcount_swar | 4097ns | 4097ns | -0.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| popcount_handrolled | 1985ns | base | --- | [1984, 1988] | --- | --- | --- | --- |
| popcount_std | 276ns | -1708.0ns (-86.0%) | [-1709, -1708]ns | [275, 277] | YES | 0.0000 | 0.0000 | 0 |
| popcount_swar | 1988ns | +2.5ns (+0.1%) | [+1, +3]ns | [1986, 1989] | YES | 0.0012 | 0.0012 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | popcount_handrolled | popcount_std | popcount_swar |
|---|---|---|---|---|
| 1 | 1 | 1987ns | -86.1% | +0.0% |
| 1 | 2 | 1984ns | -86.1% | +0.2% |
| 1 | 3 | 2015ns | -86.3% | -1.4% |
| 1 | 4 | 1987ns | -86.0% | +0.3% |
| 1 | 5 | 1985ns | -86.2% | -0.1% |
| 1 | 6 | 1985ns | -86.1% | +0.2% |
| 1 | 7 | 1981ns | -86.0% | +0.4% |
| 1 | 8 | 1984ns | -86.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| popcount_handrolled | -0.082 | ok |
| popcount_std | -0.210 | moderate- |
| popcount_swar | -0.520 | HIGH- (thermal bounce) |

**Consistency summary:**

- **popcount_std**: won 8/8, lost 0/8
- **popcount_swar**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| popcount_handrolled | 3.3ns | 1995.3ns | 0.2% |  |
| popcount_std | 2.8ns | 277.1ns | 1.0% |  |
| popcount_swar | 3.0ns | 1991.4ns | 0.2% |  |

## Distribution (algo ns)

```
popcount_handrolled (n=160, range 1978.8-2037.3 ns)
   1978.8 |####################################
   1981.7 |########################################
   1984.6 |###############
   1987.5 |##########################
   1990.5 |######
   1993.4 |#########
   1996.3 |##
   1999.2 |###
   2002.2 |###
   2005.1 |##
   2008.0 |#
   2010.9 |######
   2013.9 |####
   2016.8 |##
   2019.7 |#
   2022.6 |#
   2025.6 |###
   2028.5 |
   2031.4 |#
   2034.3 |##
  (13 below, 6 above range)

popcount_std (n=160, range 270.9-284.8 ns)
    270.9 |###
    271.6 |#################################
    272.3 |#################################
    273.0 |#####################
    273.7 |#################################
    274.4 |#####################
    275.1 |########################
    275.8 |########################################
    276.5 |########################
    277.2 |############
    277.9 |##################
    278.5 |###########################
    279.2 |############
    279.9 |########################
    280.6 |###############
    281.3 |#########
    282.0 |###############
    282.7 |##################
    283.4 |###
    284.1 |###############
  (15 below, 12 above range)

popcount_swar (n=160, range 1979.6-2013.6 ns)
   1979.6 |###########################
   1981.3 |########################################
   1983.0 |################################
   1984.7 |###################################
   1986.4 |#####################################
   1988.1 |###################################
   1989.8 |########################################
   1991.5 |##########
   1993.2 |##########
   1994.9 |###############
   1996.6 |############
   1998.3 |##########
   2000.0 |##
   2001.7 |#######
   2003.4 |##
   2005.1 |#####
   2006.8 |
   2008.5 |
   2010.2 |#####
   2011.9 |#####
  (15 below, 12 above range)

```
