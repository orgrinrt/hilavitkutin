# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 20.82 us to slowest 20.83 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### memcpy_intrinsic shows alternating (throttle bounce) (autocorr -0.62)

memcpy_intrinsic's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (11 ns) is smaller than the fastest variant's own run-to-run std-dev (62 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: memcpy_unrolled** at 20818.7 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 20818.7 ns, slowest 20829.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 22986ns | 22965ns | 22921ns | 22967ns | 23107ns | base |
| memcpy_naive | 22994ns | 22972ns | 22924ns | 22975ns | 23121ns | +0.04% |
| memcpy_unrolled | 22984ns | 22967ns | 22923ns | 22966ns | 23100ns | -0.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 20844ns | 20791ns | 20956ns | base | 0.786 |
| memcpy_naive | 20849ns | 20791ns | 20962ns | +0.02% | 0.786 |
| memcpy_unrolled | 20842ns | 20791ns | 20949ns | -0.01% | 0.786 |

## Performance model

- Peak throughput: **0.788 Gops/s** (memcpy_unrolled; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.787 | 99.9% |
| memcpy_naive | 0.787 | 99.8% |
| memcpy_unrolled | 0.787 | 99.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 22986ns | 22986ns | base |
| memcpy_naive | 22994ns | 22994ns | +0.04% |
| memcpy_unrolled | 22984ns | 22984ns | -0.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 20821ns | base | --- | [20809, 20836] | --- | --- | --- | --- |
| memcpy_naive | 20830ns | no significant difference | [-1, +2]ns | [20815, 20838] | no | 0.8732 | 0.6343 | 1 |
| memcpy_unrolled | 20819ns | no significant difference | [-2, +2]ns | [20809, 20833] | no | 0.8732 | 0.8732 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 20831ns | -0.1% | -0.1% |
| 1 | 2 | 20810ns | +0.1% | +0.1% |
| 1 | 3 | 20810ns | +0.2% | +0.1% |
| 1 | 4 | 20838ns | -0.1% | -0.1% |
| 1 | 5 | 20805ns | +0.1% | +0.0% |
| 1 | 6 | 20827ns | -0.1% | -0.1% |
| 1 | 7 | 20819ns | +0.1% | +0.1% |
| 1 | 8 | 20816ns | +0.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.620 | HIGH- (thermal bounce) |
| memcpy_naive | -0.469 | moderate- |
| memcpy_unrolled | -0.021 | ok |

**Consistency summary:**

- **memcpy_naive**: won 1/8, lost 2/8
- **memcpy_unrolled**: won 0/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 3.8ns | 20844.0ns | 0.0% |  |
| memcpy_naive | 4.0ns | 20848.9ns | 0.0% |  |
| memcpy_unrolled | 4.0ns | 20842.3ns | 0.0% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 20790.8-20955.8 ns)
  20790.8 |########################################
  20799.0 |#####
  20807.3 |#########
  20815.5 |#####
  20823.8 |#####
  20832.0 |######
  20840.3 |###########
  20848.5 |###
  20856.8 |###
  20865.0 |
  20873.3 |#
  20881.5 |
  20889.8 |##
  20898.0 |#######
  20906.3 |
  20914.5 |#
  20922.8 |###
  20931.0 |##
  20939.3 |##
  20947.5 |
  (13 below, 12 above range)

memcpy_naive (n=160, range 20790.9-20961.6 ns)
  20790.9 |########################################
  20799.5 |########
  20808.0 |#############
  20816.5 |####
  20825.1 |############
  20833.6 |################
  20842.1 |######
  20850.7 |############
  20859.2 |########
  20867.7 |
  20876.2 |
  20884.8 |##
  20893.3 |###
  20901.8 |###
  20910.4 |###
  20918.9 |#
  20927.4 |####
  20936.0 |####
  20944.5 |#
  20953.0 |#####
  (15 below, 12 above range)

memcpy_unrolled (n=160, range 20790.7-20949.1 ns)
  20790.7 |########################################
  20798.6 |##############
  20806.5 |############
  20814.4 |#####
  20822.4 |######
  20830.3 |##########
  20838.2 |###########
  20846.1 |########
  20854.0 |##
  20862.0 |###
  20869.9 |####
  20877.8 |#
  20885.7 |##
  20893.6 |####
  20901.6 |##
  20909.5 |##
  20917.4 |###
  20925.3 |####
  20933.2 |##
  20941.1 |#
  (15 below, 13 above range)

```
