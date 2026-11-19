# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.7x slower than the field

sequential (4.67 ms) is 2.7x the fastest (1.70 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core is fastest but the noisiest (CV 6.8%)

per_core wins on median (1.70 ms) yet has the highest variance (CV 6.8%), while sequential is the steadiest (CV 3.4%, 4.67 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 1703087.0 ns median (-7.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.74x (fastest 1703087.0 ns, slowest 4671886.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 3, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 1897238ns | 1847016ns | 1763286ns | 1841791ns | 2047385ns | base |
| per_core | 1725887ns | 1705663ns | 1588170ns | 1685082ns | 1855953ns | -9.03% |
| sequential | 4742433ns | 4674083ns | 4573541ns | 4668729ns | 4937436ns | +149.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 1895330ns | 1761439ns | 2045477ns | base | 0.553 |
| per_core | 1723983ns | 1586288ns | 1854190ns | -9.04% | 0.608 |
| sequential | 4739938ns | 4570823ns | 4934897ns | +150.09% | 0.221 |

## Performance model

- Peak throughput: **0.661 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.568 | 86.0% |
| per_core | 0.616 | 93.1% |
| sequential | 0.224 | 34.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 1897238ns | 1897238ns | base |
| per_core | 1725887ns | 1725887ns | -9.03% |
| sequential | 4742433ns | 4742433ns | +149.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 1844629ns | base | --- | [1806685, 1944446] | --- | --- | --- | --- |
| per_core | 1703087ns | -162994.5ns (-8.8%) | [-258792, -69451]ns | [1653626, 1782521] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 4671887ns | +2817314.1ns (+152.7%) | [+2707419, +2969287]ns | [4631633, 4841738] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 1817575ns | -8.2% | +154.8% |
| 1 | 2 | 1957164ns | -10.3% | +137.4% |
| 1 | 3 | 1833959ns | -6.8% | +166.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.659 | HIGH- (thermal bounce) |
| per_core | -0.529 | HIGH- (thermal bounce) |
| sequential | -0.139 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 2047308.9ns | 1895330.4ns | 108.0% | HIGH |
| per_core | 1735597.6ns | 1723982.9ns | 100.7% | HIGH |
| sequential | 4735998.4ns | 4739938.2ns | 99.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 1761438.9-2045477.2 ns)
  1761438.9 |########################################
  1775640.8 |####################
  1789842.8 |####################
  1804044.7 |####################
  1818246.6 |####################
  1832448.5 |########################################
  1846650.4 |####################
  1860852.3 |
  1875054.3 |####################
  1889256.2 |
  1903458.1 |
  1917660.0 |########################################
  1931861.9 |
  1946063.8 |####################
  1960265.8 |
  1974467.7 |####################
  1988669.6 |
  2002871.5 |
  2017073.4 |
  2031275.3 |
  (1 below, 3 above range)

per_core (n=18, range 1586288.0-1854190.5 ns)
  1586288.0 |
  1599683.1 |
  1613078.2 |
  1626473.3 |########################################
  1639868.5 |#############
  1653263.6 |##########################
  1666658.7 |
  1680053.8 |#############
  1693449.0 |##########################
  1706844.1 |#############
  1720239.2 |#############
  1733634.4 |
  1747029.5 |#############
  1760424.6 |
  1773819.7 |
  1787214.9 |
  1800610.0 |#############
  1814005.1 |
  1827400.2 |##########################
  1840795.4 |
  (1 below, 2 above range)

sequential (n=18, range 4570823.3-4934897.3 ns)
  4570823.3 |
  4589027.0 |
  4607230.7 |##########################
  4625434.4 |########################################
  4643638.1 |#############
  4661841.8 |########################################
  4680045.5 |
  4698249.2 |
  4716452.9 |
  4734656.6 |
  4752860.3 |
  4771064.0 |
  4789267.7 |#############
  4807471.4 |#############
  4825675.1 |
  4843878.8 |
  4862082.5 |#############
  4880286.2 |#############
  4898489.9 |#############
  4916693.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=108.4% of algo (FFI overhead may distort results)
- **per_core**: bridge=101.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
