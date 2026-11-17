# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (48 ns) is smaller than the fastest variant's own run-to-run std-dev (106 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.9% of the fastest

All 3 variants sit between 5.06 us and 5.11 us - a 0.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (47 ns, 0.93%)

inline_never differs from baseline inline_always by 47 ns (0.93%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 5064.0 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 5064.0 ns, slowest 5111.7 ns)

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
| inline_always | 7238ns | 7166ns | 7158ns | 7186ns | 7476ns | base |
| inline_default | 7243ns | 7168ns | 7159ns | 7181ns | 7511ns | +0.06% |
| inline_never | 7208ns | 7196ns | 7188ns | 7198ns | 7258ns | -0.42% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 5116ns | 5059ns | 5286ns | base | 0.801 |
| inline_default | 5116ns | 5058ns | 5305ns | +0.01% | 0.801 |
| inline_never | 5120ns | 5106ns | 5160ns | +0.09% | 0.800 |

## Performance model

- Peak throughput: **0.810 Gops/s** (inline_default; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.809 | 99.9% |
| inline_default | 0.809 | 99.9% |
| inline_never | 0.801 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 7238ns | 7238ns | base |
| inline_default | 7243ns | 7243ns | +0.06% |
| inline_never | 7208ns | 7208ns | -0.42% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 5064ns | base | --- | [5063, 5067] | --- | --- | --- | --- |
| inline_default | 5064ns | no significant difference | [-1, +1]ns | [5063, 5065] | no | 1.0000 | 1.0000 | 7 |
| inline_never | 5112ns | +47.1ns (+0.9%) | [+45, +49]ns | [5111, 5112] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 5062ns | +0.0% | +1.0% |
| 1 | 2 | 5259ns | -0.2% | -2.8% |
| 1 | 3 | 5064ns | -0.0% | +0.9% |
| 1 | 4 | 5063ns | -0.0% | +0.9% |
| 1 | 5 | 5072ns | -0.2% | +0.8% |
| 1 | 6 | 5064ns | -0.0% | +0.9% |
| 1 | 7 | 5062ns | +0.0% | +1.0% |
| 1 | 8 | 5062ns | +4.4% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.178 | ok |
| inline_default | -0.189 | ok |
| inline_never | -0.065 | ok |

**Consistency summary:**

- **inline_default**: won 2/8, lost 1/8
- **inline_never**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.6ns | 5115.7ns | 0.1% |  |
| inline_default | 2.7ns | 5116.4ns | 0.1% |  |
| inline_never | 3.2ns | 5120.4ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 5058.5-5285.6 ns)
   5058.5 |########################################
   5069.9 |#
   5081.2 |#
   5092.6 |##
   5103.9 |##
   5115.3 |#
   5126.6 |
   5138.0 |
   5149.4 |#
   5160.7 |
   5172.1 |
   5183.4 |
   5194.8 |
   5206.1 |
   5217.5 |
   5228.8 |#
   5240.2 |###
   5251.5 |#
   5262.9 |
   5274.3 |#
  (13 below, 10 above range)

inline_default (n=160, range 5058.0-5305.0 ns)
   5058.0 |########################################
   5070.3 |##
   5082.7 |##
   5095.0 |##
   5107.4 |#
   5119.7 |
   5132.1 |
   5144.4 |##
   5156.8 |
   5169.1 |
   5181.5 |
   5193.8 |
   5206.2 |
   5218.5 |
   5230.9 |
   5243.2 |##
   5255.6 |#
   5267.9 |
   5280.3 |
   5292.6 |
  (13 below, 13 above range)

inline_never (n=160, range 5105.6-5160.5 ns)
   5105.6 |####################
   5108.3 |########################################
   5111.1 |######################################
   5113.8 |#################
   5116.6 |#####
   5119.3 |#
   5122.1 |#
   5124.8 |
   5127.5 |##
   5130.3 |###
   5133.0 |
   5135.8 |
   5138.5 |##
   5141.3 |##
   5144.0 |
   5146.7 |###
   5149.5 |
   5152.2 |#
   5155.0 |#
   5157.7 |#
  (16 below, 14 above range)

```
