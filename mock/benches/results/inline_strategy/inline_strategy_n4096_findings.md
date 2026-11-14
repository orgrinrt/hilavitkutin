# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (111 ns) is smaller than the fastest variant's own run-to-run std-dev (240 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader inline_never vs stability leader inline_always (+2% speed for 1.3x steadier)

inline_never is fastest (5.32 us, CV 4.5%); inline_always gives up 2.0% median for 1.3x lower variance (CV 3.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 2.1% of the fastest

All 3 variants sit between 5.32 us and 5.43 us - a 2.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_default's edge over baseline is significant but tiny (6 ns, 0.12%)

inline_default differs from baseline inline_always by 6 ns (0.12%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_never** at 5315.6 ns median (-2.0% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.02x (fastest 5315.6 ns, slowest 5426.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 7514ns | 7660ns | 7155ns | 7523ns | 7843ns | base |
| inline_default | 7579ns | 7665ns | 7164ns | 7575ns | 8004ns | +0.87% |
| inline_never | 7525ns | 7465ns | 7184ns | 7471ns | 8029ns | +0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 5317ns | 5062ns | 5546ns | base | 0.770 |
| inline_default | 5361ns | 5068ns | 5656ns | +0.82% | 0.764 |
| inline_never | 5352ns | 5109ns | 5707ns | +0.66% | 0.765 |

## Performance model

- Peak throughput: **0.809 Gops/s** (inline_always; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.755 | 93.3% |
| inline_default | 0.755 | 93.3% |
| inline_never | 0.771 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 7514ns | 7514ns | base |
| inline_default | 7579ns | 7579ns | +0.87% |
| inline_never | 7525ns | 7525ns | +0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 5424ns | base | --- | [5312, 5426] | --- | --- | --- | --- |
| inline_default | 5426ns | +6.5ns (+0.1%) | [+1, +25]ns | [5423, 5428] | YES | 0.0218 | 0.0109 | 1 |
| inline_never | 5316ns | no significant difference | [-3, +53]ns | [5258, 5406] | no | 0.0687 | 0.0687 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 5216ns | +4.0% | -2.0% |
| 1 | 2 | 5428ns | +0.2% | -2.0% |
| 1 | 3 | 5165ns | +2.2% | +6.0% |
| 1 | 4 | 5063ns | +7.2% | +8.2% |
| 1 | 5 | 5387ns | +0.7% | +9.6% |
| 1 | 6 | 5428ns | -1.4% | -5.6% |
| 1 | 7 | 5427ns | +0.1% | -5.7% |
| 1 | 8 | 5427ns | -6.2% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.164 | ok |
| inline_default | -0.207 | moderate- |
| inline_never | -0.037 | ok |

**Consistency summary:**

- **inline_default**: won 2/8, lost 5/8
- **inline_never**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.8ns | 5317.1ns | 0.1% |  |
| inline_default | 2.8ns | 5360.8ns | 0.1% |  |
| inline_never | 3.2ns | 5352.1ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 5062.1-5545.5 ns)
   5062.1 |##############
   5086.2 |###
   5110.4 |#
   5134.6 |####
   5158.8 |##
   5182.9 |
   5207.1 |##
   5231.3 |######
   5255.4 |
   5279.6 |
   5303.8 |##
   5328.0 |##
   5352.1 |
   5376.3 |##
   5400.5 |######
   5424.7 |########################################
   5448.8 |
   5473.0 |
   5497.2 |#
   5521.4 |
  (15 below, 10 above range)

inline_default (n=160, range 5068.2-5656.1 ns)
   5068.2 |####
   5097.6 |####
   5127.0 |##
   5156.4 |
   5185.8 |##
   5215.2 |#
   5244.6 |##
   5274.0 |#
   5303.3 |
   5332.7 |
   5362.1 |
   5391.5 |##
   5420.9 |########################################
   5450.3 |##
   5479.7 |
   5509.1 |
   5538.5 |
   5567.9 |
   5597.3 |
   5626.7 |#
  (24 below, 16 above range)

inline_never (n=160, range 5109.3-5706.9 ns)
   5109.3 |##################################
   5139.2 |#######
   5169.1 |######
   5199.0 |##
   5228.8 |###
   5258.7 |###
   5288.6 |############
   5318.5 |#####
   5348.4 |##
   5378.2 |###
   5408.1 |##
   5438.0 |##
   5467.9 |########################################
   5497.8 |##
   5527.7 |###
   5557.5 |
   5587.4 |##
   5617.3 |###
   5647.2 |
   5677.1 |
  (16 below, 14 above range)

```
