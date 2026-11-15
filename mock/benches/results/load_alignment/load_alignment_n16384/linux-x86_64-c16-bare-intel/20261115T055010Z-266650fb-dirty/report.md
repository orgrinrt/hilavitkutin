# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.29 us to slowest 2.29 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (203 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (2.29 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### load_byte_pack's comparison is tie-heavy (12% tied pairs)

12% of paired samples for load_byte_pack are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

## Key findings

- **Baseline (load_aligned) is the fastest** at 2287.2 ns median
- Spread: 1.00x (fastest 2287.2 ns, slowest 2287.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 2112776509 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 3891ns | 4161ns | 3406ns | 3949ns | 4203ns | base |
| load_byte_pack | 3900ns | 4163ns | 3405ns | 3950ns | 4244ns | +0.22% |
| load_unaligned | 4016ns | 4165ns | 3408ns | 4106ns | 4355ns | +3.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2135ns | 1871ns | 2302ns | base | 7.675 |
| load_byte_pack | 2139ns | 1871ns | 2323ns | +0.21% | 7.658 |
| load_unaligned | 2203ns | 1871ns | 2381ns | +3.17% | 7.439 |

## Performance model

- Peak throughput: **8.755 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 7.163 | 81.8% |
| load_byte_pack | 7.163 | 81.8% |
| load_unaligned | 7.163 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 3891ns | 3891ns | base |
| load_byte_pack | 3900ns | 3900ns | +0.22% |
| load_unaligned | 4016ns | 4016ns | +3.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 2287ns | base | --- | [2287, 2287] | --- | --- | --- | --- |
| load_byte_pack | 2287ns | no significant difference | [-0, +0]ns | [2287, 2287] | no | 0.8663 | 0.8663 | **19** (12%, HIGH) |
| load_unaligned | 2287ns | no significant difference | [+0, +0]ns | [2287, 2287] | no | 0.3675 | 0.1837 | 15 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 2288ns | -0.0% | -0.0% |
| 1 | 2 | 1872ns | -0.0% | +0.0% |
| 1 | 3 | 1872ns | -0.0% | -0.0% |
| 1 | 4 | 1872ns | -0.0% | +22.2% |
| 1 | 5 | 2287ns | +0.0% | -0.0% |
| 1 | 6 | 2287ns | +0.0% | +0.0% |
| 1 | 7 | 2287ns | +0.0% | +0.0% |
| 1 | 8 | 2287ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.391 | moderate+ |
| load_byte_pack | 0.392 | moderate+ |
| load_unaligned | 0.292 | moderate+ |

**Consistency summary:**

- **load_byte_pack**: won 0/8, lost 0/8
- **load_unaligned**: won 0/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 22.2ns | 2134.8ns | 1.0% |  |
| load_byte_pack | 22.2ns | 2139.4ns | 1.0% |  |
| load_unaligned | 22.9ns | 2202.5ns | 1.0% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 1871.4-2301.8 ns)
   1871.4 |##################
   1893.0 |
   1914.5 |
   1936.0 |
   1957.5 |
   1979.0 |
   2000.5 |
   2022.0 |
   2043.6 |
   2065.1 |
   2086.6 |
   2108.1 |
   2129.6 |
   2151.1 |
   2172.7 |
   2194.2 |
   2215.7 |
   2237.2 |
   2258.7 |
   2280.2 |########################################
  (15 below, 3 above range)

load_byte_pack (n=160, range 1871.4-2323.1 ns)
   1871.4 |####################
   1894.0 |
   1916.5 |
   1939.1 |
   1961.7 |
   1984.3 |
   2006.9 |
   2029.5 |
   2052.1 |
   2074.7 |
   2097.3 |
   2119.8 |
   2142.4 |
   2165.0 |
   2187.6 |
   2210.2 |
   2232.8 |
   2255.4 |
   2278.0 |########################################
   2300.6 |#
  (11 below, 2 above range)

load_unaligned (n=160, range 1871.5-2381.1 ns)
   1871.5 |########
   1897.0 |
   1922.4 |
   1947.9 |
   1973.4 |
   1998.9 |
   2024.4 |
   2049.8 |
   2075.3 |
   2100.8 |
   2126.3 |
   2151.8 |
   2177.2 |
   2202.7 |
   2228.2 |
   2253.7 |
   2279.1 |########################################
   2304.6 |
   2330.1 |
   2355.6 |
  (16 below, 5 above range)

```
