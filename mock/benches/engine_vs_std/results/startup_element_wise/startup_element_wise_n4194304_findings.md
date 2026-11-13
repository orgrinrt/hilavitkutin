# element_wise, get-ready

2 variants, 8 samples per variant.
Baseline: **engine_startup**

## Highlights

Baseline for all deltas below: **engine_startup**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine_startup has the worst median (1.82 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_startup at 1.72 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std_startup is fastest but the noisiest (CV 26.6%)

std_startup wins on median (1.72 ms) yet has the highest variance (CV 26.6%), while engine_startup is the steadiest (CV 17.8%, 1.82 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_startup shows alternating (throttle bounce) (autocorr -0.50)

std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (98.09 us) is smaller than the fastest variant's own run-to-run std-dev (458.77 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader std_startup vs stability leader engine_startup (+6% speed for 1.5x steadier)

std_startup is fastest (1.72 ms, CV 26.6%); engine_startup gives up 5.7% median for 1.5x lower variance (CV 17.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### std_startup is inconsistent: worst-20% is 1.5x its best-20%

std_startup's best 20% of batches run at 1.53 ms but its worst 20% at 2.29 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_startup** at 1723240.9 ns median (-5.4% vs baseline)
- Spread: 1.06x (fastest 1723240.9 ns, slowest 1821334.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010001 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine_startup | 1912869ns | 1830082ns | 1517717ns | 1717067ns | 2158509ns | base |
| std_startup | 1961094ns | 1728844ns | 1534658ns | 1650791ns | 2300430ns | +2.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine_startup | 1906347ns | 1512486ns | 2150669ns | base | 2.200 |
| std_startup | 1955337ns | 1529494ns | 2294666ns | +2.57% | 2.145 |

## Performance model

- Peak throughput: **2.773 Gops/s** (engine_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine_startup | 2.303 | 83.0% |
| std_startup | 2.434 | 87.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine_startup | 1912869ns | 1912869ns | base |
| std_startup | 1961094ns | 1961094ns | +2.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine_startup | 1821334ns | base | --- | [1639003, 2075380] | --- | --- | --- | --- |
| std_startup | 1723241ns | no significant difference | [-340416, +574764]ns | [1595678, 2282930] | no | 0.7266 | 0.7266 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine_startup | std_startup |
|---|---|---|---|
| 1 | 1 | 1821334ns | -8.3% |
| 1 | 2 | 1882245ns | +6.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine_startup | -0.500 | moderate- |
| std_startup | -0.500 | moderate- |

**Consistency summary:**

- **std_startup**: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine_startup | 43349340.3ns | 1906347.4ns | 2273.9% | HIGH |
| std_startup | 44853612.7ns | 1955336.8ns | 2293.9% | HIGH |

## Distribution (algo ns)

```
engine_startup (n=8, range 1512486.4-2150669.2 ns)
  1512486.4 |####################
  1544395.5 |
  1576304.7 |
  1608213.8 |####################
  1640123.0 |
  1672032.1 |
  1703941.2 |####################
  1735850.4 |
  1767759.5 |####################
  1799668.7 |
  1831577.8 |####################
  1863486.9 |
  1895396.1 |
  1927305.2 |
  1959214.4 |
  1991123.5 |
  2023032.6 |
  2054941.8 |########################################
  2086850.9 |
  2118760.1 |
  (0 below, 1 above range)

std_startup (n=8, range 1529494.4-2294665.5 ns)
  1529494.4 |########################################
  1567753.0 |########################################
  1606011.5 |########################################
  1644270.1 |
  1682528.6 |########################################
  1720787.2 |########################################
  1759045.7 |
  1797304.3 |
  1835562.9 |
  1873821.4 |
  1912080.0 |
  1950338.5 |
  1988597.1 |
  2026855.6 |
  2065114.2 |
  2103372.8 |
  2141631.3 |
  2179889.9 |########################################
  2218148.4 |
  2256407.0 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **engine_startup**: bridge=2427.4% of algo (FFI overhead may distort results)
- **std_startup**: CV=23.5% (high variance, measurements may be unstable)
- **std_startup**: bridge=2609.1% of algo (FFI overhead may distort results)
