# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (3 ns) is smaller than the fastest variant's own run-to-run std-dev (68 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.0% of the fastest

All 3 variants sit between 18.27 us and 18.27 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (3 ns, 0.02%)

inline_never differs from baseline inline_always by 3 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 18267.5 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 18267.5 ns, slowest 18270.7 ns)

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
| Counter frequency | 2112795995 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 20154ns | 20147ns | 20141ns | 20147ns | 20188ns | base |
| inline_default | 20168ns | 20147ns | 20141ns | 20148ns | 20254ns | +0.07% |
| inline_never | 20811ns | 20154ns | 17872ns | 20154ns | 25723ns | +3.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 18269ns | 18261ns | 18282ns | base | 0.897 |
| inline_default | 18280ns | 18256ns | 18342ns | +0.06% | 0.896 |
| inline_never | 18874ns | 16199ns | 23360ns | +3.31% | 0.868 |

## Performance model

- Peak throughput: **1.011 Gops/s** (inline_never; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.897 | 88.7% |
| inline_default | 0.897 | 88.7% |
| inline_never | 0.897 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 20154ns | 20154ns | base |
| inline_default | 20168ns | 20168ns | +0.07% |
| inline_never | 20811ns | 20811ns | +3.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 18268ns | base | --- | [18267, 18268] | --- | --- | --- | --- |
| inline_default | 18267ns | no significant difference | [-1, +1]ns | [18267, 18268] | no | 1.0000 | 1.0000 | 1 |
| inline_never | 18271ns | +3.2ns (+0.0%) | [+2, +4]ns | [18270, 18271] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 18270ns | -0.0% | +0.0% |
| 1 | 2 | 18267ns | +0.0% | +0.0% |
| 1 | 3 | 18266ns | +0.0% | +0.0% |
| 1 | 4 | 18268ns | -0.0% | +0.0% |
| 1 | 5 | 18266ns | +0.0% | -18.2% |
| 1 | 6 | 18268ns | +0.0% | +0.0% |
| 1 | 7 | 18268ns | -0.0% | +63.7% |
| 1 | 8 | 18268ns | -0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.197 | ok |
| inline_default | -0.221 | moderate- |
| inline_never | -0.071 | ok |

**Consistency summary:**

- **inline_default**: won 0/8, lost 0/8
- **inline_never**: won 1/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 23.8ns | 18269.1ns | 0.1% |  |
| inline_default | 23.8ns | 18280.2ns | 0.1% |  |
| inline_never | 24.8ns | 18874.2ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 18260.6-18282.2 ns)
  18260.6 |
  18261.6 |
  18262.7 |
  18263.8 |
  18264.9 |#########
  18266.0 |##########################
  18267.0 |########################################
  18268.1 |###########################
  18269.2 |############
  18270.3 |#######
  18271.4 |###
  18272.4 |
  18273.5 |
  18274.6 |
  18275.7 |
  18276.8 |
  18277.8 |
  18278.9 |
  18280.0 |
  18281.1 |
  (18 below, 2 above range)

inline_default (n=160, range 18256.4-18342.4 ns)
  18256.4 |########
  18260.7 |
  18265.0 |########################################
  18269.3 |#########
  18273.6 |
  18277.9 |##
  18282.2 |
  18286.5 |
  18290.8 |
  18295.1 |
  18299.4 |
  18303.7 |
  18308.0 |
  18312.3 |
  18316.6 |
  18320.9 |
  18325.2 |
  18329.5 |
  18333.8 |
  18338.1 |
  (15 below, 7 above range)

inline_never (n=160, range 16198.6-23360.0 ns)
  16198.6 |
  16556.7 |
  16914.7 |
  17272.8 |
  17630.9 |
  17989.0 |########################################
  18347.0 |
  18705.1 |
  19063.2 |
  19421.2 |
  19779.3 |
  20137.4 |
  20495.5 |
  20853.5 |
  21211.6 |
  21569.7 |
  21927.7 |
  22285.8 |
  22643.9 |
  23002.0 |
  (20 below, 14 above range)

```
