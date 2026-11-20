# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential is an outlier: 2.6x slower than the field

sequential (4.59 ms) is 2.6x the fastest (1.78 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core is fastest but the noisiest (CV 10.3%)

per_core wins on median (1.78 ms) yet has the highest variance (CV 10.3%), while sequential is the steadiest (CV 4.4%, 4.59 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 1778113.8 ns median (-1.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 2.58x (fastest 1778113.8 ns, slowest 4588444.2 ns)

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
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 1842714ns | 1811355ns | 1697788ns | 1785413ns | 2001127ns | base |
| per_core | 1756865ns | 1780023ns | 1465238ns | 1724278ns | 1951558ns | -4.66% |
| sequential | 4590214ns | 4590487ns | 4325572ns | 4540575ns | 4796993ns | +149.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 1840912ns | 1696055ns | 1998993ns | base | 0.570 |
| per_core | 1754999ns | 1463448ns | 1949434ns | -4.67% | 0.597 |
| sequential | 4587911ns | 4323527ns | 4794515ns | +149.22% | 0.229 |

## Performance model

- Peak throughput: **0.717 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.579 | 80.9% |
| per_core | 0.590 | 82.3% |
| sequential | 0.229 | 31.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 1842714ns | 1842714ns | base |
| per_core | 1756865ns | 1756865ns | -4.66% |
| sequential | 4590214ns | 4590214ns | +149.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 1809681ns | base | --- | [1743828, 1898655] | --- | --- | --- | --- |
| per_core | 1778114ns | no significant difference | [-125928, +98685]ns | [1623422, 1922179] | no | 0.8145 | 0.8145 | 0 |
| sequential | 4588444ns | +2776707.0ns (+153.4%) | [+2663952, +2821392]ns | [4462118, 4657128] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 1836569ns | -3.2% | +153.6% |
| 1 | 2 | 1778300ns | +8.1% | +159.9% |
| 1 | 3 | 1842279ns | -17.3% | +138.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.662 | HIGH- (thermal bounce) |
| per_core | -0.402 | moderate- |
| sequential | -0.102 | ok |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 1969288.2ns | 1840912.4ns | 107.0% | HIGH |
| per_core | 1785042.4ns | 1754999.4ns | 101.7% | HIGH |
| sequential | 4578656.9ns | 4587910.6ns | 99.8% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 1696055.4-1998992.6 ns)
  1696055.4 |########################################
  1711202.3 |
  1726349.1 |####################
  1741496.0 |########################################
  1756642.9 |########################################
  1771789.7 |
  1786936.6 |####################
  1802083.4 |
  1817230.3 |####################
  1832377.1 |####################
  1847524.0 |########################################
  1862670.9 |
  1877817.7 |
  1892964.6 |
  1908111.4 |
  1923258.3 |
  1938405.1 |####################
  1953552.0 |
  1968698.9 |
  1983845.7 |
  (1 below, 4 above range)

per_core (n=18, range 1463448.3-1949433.8 ns)
  1463448.3 |##########
  1487747.6 |##########
  1512046.9 |
  1536346.2 |##########
  1560645.4 |
  1584944.7 |##########
  1609244.0 |
  1633543.3 |####################
  1657842.5 |
  1682141.8 |
  1706441.1 |
  1730740.3 |
  1755039.6 |####################
  1779338.9 |##########
  1803638.2 |
  1827937.4 |##########
  1852236.7 |
  1876536.0 |##########
  1900835.3 |##########
  1925134.5 |########################################
  (1 below, 1 above range)

sequential (n=18, range 4323527.0-4794515.3 ns)
  4323527.0 |
  4347076.4 |
  4370625.9 |####################
  4394175.3 |####################
  4417724.7 |####################
  4441274.1 |
  4464823.5 |
  4488372.9 |####################
  4511922.3 |
  4535471.8 |########################################
  4559021.2 |
  4582570.6 |########################################
  4606120.0 |########################################
  4629669.4 |####################
  4653218.8 |
  4676768.2 |########################################
  4700317.6 |
  4723867.1 |####################
  4747416.5 |
  4770965.9 |
  (2 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=107.2% of algo (FFI overhead may distort results)
- **per_core**: bridge=104.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.9% of algo (FFI overhead may distort results)
