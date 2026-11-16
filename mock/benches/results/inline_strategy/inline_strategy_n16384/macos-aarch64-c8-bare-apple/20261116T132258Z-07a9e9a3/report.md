# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (54 ns) is smaller than the fastest variant's own run-to-run std-dev (240 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.3% of the fastest

All 3 variants sit between 20.41 us and 20.46 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (49 ns, 0.24%)

inline_never differs from baseline inline_always by 49 ns (0.24%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: inline_default** at 20406.9 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 20406.9 ns, slowest 20460.6 ns)

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
| inline_always | 22542ns | 22523ns | 22501ns | 22530ns | 22617ns | base |
| inline_default | 22602ns | 22530ns | 22499ns | 22536ns | 22906ns | +0.27% |
| inline_never | 22620ns | 22554ns | 22529ns | 22560ns | 22889ns | +0.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 20434ns | 20399ns | 20502ns | base | 0.802 |
| inline_default | 20489ns | 20399ns | 20769ns | +0.27% | 0.800 |
| inline_never | 20526ns | 20448ns | 20766ns | +0.45% | 0.798 |

## Performance model

- Peak throughput: **0.803 Gops/s** (inline_default; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.803 | 100.0% |
| inline_default | 0.803 | 100.0% |
| inline_never | 0.801 | 99.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 22542ns | 22542ns | base |
| inline_default | 22602ns | 22602ns | +0.27% |
| inline_never | 22620ns | 22620ns | +0.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 20408ns | base | --- | [20405, 20425] | --- | --- | --- | --- |
| inline_default | 20407ns | no significant difference | [-1, +1]ns | [20405, 20434] | no | 0.7481 | 0.7481 | 5 |
| inline_never | 20461ns | +49.0ns (+0.2%) | [+48, +51]ns | [20456, 20471] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 20448ns | -0.2% | +0.0% |
| 1 | 2 | 20447ns | -0.2% | +0.1% |
| 1 | 3 | 20407ns | -0.0% | +0.3% |
| 1 | 4 | 20406ns | -0.0% | +0.3% |
| 1 | 5 | 20434ns | -0.1% | +0.1% |
| 1 | 6 | 20405ns | +0.1% | +0.3% |
| 1 | 7 | 20404ns | -0.0% | +0.3% |
| 1 | 8 | 20403ns | +0.3% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.240 | moderate+ |
| inline_default | -0.107 | ok |
| inline_never | -0.130 | ok |

**Consistency summary:**

- **inline_default**: won 2/8, lost 1/8
- **inline_never**: won 0/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.1ns | 20434.0ns | 0.0% |  |
| inline_default | 3.1ns | 20489.2ns | 0.0% |  |
| inline_never | 3.7ns | 20525.9ns | 0.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 20399.1-20501.9 ns)
  20399.1 |########################################
  20404.3 |##################
  20409.4 |#
  20414.5 |#
  20419.7 |###
  20424.8 |
  20430.0 |
  20435.1 |
  20440.2 |###
  20445.4 |#######
  20450.5 |####
  20455.6 |###
  20460.8 |####
  20465.9 |###
  20471.0 |##
  20476.2 |
  20481.3 |######
  20486.5 |####
  20491.6 |#
  20496.7 |
  (11 below, 10 above range)

inline_default (n=160, range 20399.1-20769.2 ns)
  20399.1 |########################################
  20417.6 |##
  20436.1 |######
  20454.6 |########
  20473.1 |#####
  20491.6 |##
  20510.1 |##
  20528.6 |##
  20547.1 |#
  20565.6 |
  20584.2 |
  20602.7 |
  20621.2 |
  20639.7 |
  20658.2 |
  20676.7 |
  20695.2 |
  20713.7 |
  20732.2 |
  20750.7 |
  (13 below, 9 above range)

inline_never (n=160, range 20448.3-20766.3 ns)
  20448.3 |########################################
  20464.2 |##
  20480.1 |#####
  20496.0 |############
  20511.9 |###
  20527.8 |###
  20543.7 |#
  20559.6 |
  20575.5 |
  20591.4 |#
  20607.3 |
  20623.2 |#
  20639.1 |#
  20655.0 |
  20670.9 |
  20686.8 |
  20702.7 |
  20718.6 |
  20734.5 |
  20750.4 |
  (13 below, 9 above range)

```
