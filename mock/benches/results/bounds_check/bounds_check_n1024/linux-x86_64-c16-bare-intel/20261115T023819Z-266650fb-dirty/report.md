# Bounds-check elision: iter() vs explicit index vs get_unchecked

3 variants, 160 samples per variant.
Baseline: **bc_index**

## Highlights

Baseline for all deltas below: **bc_index**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### bc_iter is fastest but the noisiest (CV 92.8%)

bc_iter wins on median (152 ns) yet has the highest variance (CV 92.8%), while bc_unchecked is the steadiest (CV 86.8%, 153 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (141 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bc_iter's comparison is tie-heavy (17% tied pairs)

17% of paired samples for bc_iter are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader bc_iter vs stability leader bc_unchecked (+0% speed for 1.1x steadier)

bc_iter is fastest (152 ns, CV 92.8%); bc_unchecked gives up 0.2% median for 1.1x lower variance (CV 86.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### bc_index is inconsistent: worst-20% is 3.8x its best-20%

bc_index's best 20% of batches run at 125 ns but its worst 20% at 475 ns (3.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.2% of the fastest

All 3 variants sit between 152 ns and 153 ns - a 0.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bc_unchecked's edge over baseline is significant but tiny (0 ns, 0.13%)

bc_unchecked differs from baseline bc_index by 0 ns (0.13%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bc_iter** at 152.3 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 152.3 ns, slowest 152.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Bounds-check elision: iter() vs explicit index vs get_unchecked |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5a3ec07491bf2d68 |
| Counter frequency | 2112761440 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bc_index | 2837ns | 2030ns | 1661ns | 2034ns | 6422ns | base |
| bc_iter | 2771ns | 2029ns | 1658ns | 1968ns | 6291ns | -2.33% |
| bc_unchecked | 2961ns | 2033ns | 1821ns | 2205ns | 6369ns | +4.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bc_index | 212ns | 125ns | 475ns | base | 4.839 |
| bc_iter | 208ns | 125ns | 472ns | -1.77% | 4.926 |
| bc_unchecked | 213ns | 137ns | 458ns | +0.87% | 4.797 |

## Performance model

- Peak throughput: **8.221 Gops/s** (bc_iter; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bc_index | 6.721 | 81.8% |
| bc_iter | 6.724 | 81.8% |
| bc_unchecked | 6.710 | 81.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bc_index | 2837ns | 2837ns | base |
| bc_iter | 2771ns | 2771ns | -2.33% |
| bc_unchecked | 2961ns | 2961ns | +4.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bc_index | 152ns | base | --- | [152, 152] | --- | --- | --- | --- |
| bc_iter | 152ns | no significant difference | [-0, +0]ns | [152, 152] | no (adj: YES) | 0.0370 | 0.0370 | **27** (17%, HIGH) |
| bc_unchecked | 153ns | +0.2ns (+0.1%) | [+0, +0]ns | [152, 153] | YES | 0.0000 | 0.0000 | 8 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bc_index | bc_iter | bc_unchecked |
|---|---|---|---|---|
| 1 | 1 | 152ns | -18.2% | +18.3% |
| 1 | 2 | 152ns | +0.0% | +0.1% |
| 1 | 3 | 125ns | -0.1% | +0.1% |
| 1 | 4 | 152ns | +0.0% | +0.1% |
| 1 | 5 | 125ns | +0.1% | +22.5% |
| 1 | 6 | 152ns | -0.0% | +0.1% |
| 1 | 7 | 172ns | -11.2% | -9.8% |
| 1 | 8 | 152ns | -0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bc_index | -0.107 | ok |
| bc_iter | -0.478 | moderate- |
| bc_unchecked | 0.004 | ok |

**Consistency summary:**

- **bc_iter**: won 2/8, lost 0/8
- **bc_unchecked**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bc_index | 33.2ns | 211.6ns | 15.7% | HIGH |
| bc_iter | 32.9ns | 207.9ns | 15.8% | HIGH |
| bc_unchecked | 35.0ns | 213.5ns | 16.4% | HIGH |

## Distribution (algo ns)

```
bc_index (n=160, range 124.7-475.3 ns)
    124.7 |#####
    142.2 |########################################
    159.7 |
    177.3 |
    194.8 |
    212.3 |
    229.8 |
    247.4 |
    264.9 |
    282.4 |
    300.0 |
    317.5 |
    335.0 |
    352.6 |
    370.1 |
    387.6 |
    405.2 |
    422.7 |
    440.2 |
    457.8 |
  (20 below, 20 above range)

bc_iter (n=160, range 124.6-472.1 ns)
    124.6 |##################
    141.9 |########################################
    159.3 |#
    176.7 |
    194.1 |#
    211.4 |
    228.8 |
    246.2 |
    263.6 |
    280.9 |
    298.3 |
    315.7 |
    333.1 |
    350.5 |
    367.8 |
    385.2 |
    402.6 |#
    420.0 |
    437.3 |#
    454.7 |#
  (15 below, 19 above range)

bc_unchecked (n=160, range 136.9-457.8 ns)
    136.9 |########################################
    152.9 |####
    169.0 |######
    185.0 |
    201.1 |
    217.1 |
    233.1 |
    249.2 |
    265.2 |
    281.3 |
    297.3 |
    313.4 |
    329.4 |
    345.5 |
    361.5 |
    377.5 |
    393.6 |
    409.6 |#
    425.7 |
    441.7 |
  (18 below, 19 above range)

```

## Diagnostics

- **bc_index**: CV=65.0% (high variance, measurements may be unstable)
- **bc_index**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bc_index**: bridge=15.6% of algo (FFI overhead may distort results)
- **bc_iter**: CV=68.0% (high variance, measurements may be unstable)
- **bc_iter**: worst_20/best_20 = 3.8x (possible bimodal distribution)
- **bc_iter**: bridge=15.6% of algo (FFI overhead may distort results)
- **bc_unchecked**: CV=62.0% (high variance, measurements may be unstable)
- **bc_unchecked**: worst_20/best_20 = 3.3x (possible bimodal distribution)
- **bc_unchecked**: bridge=15.8% of algo (FFI overhead may distort results)
