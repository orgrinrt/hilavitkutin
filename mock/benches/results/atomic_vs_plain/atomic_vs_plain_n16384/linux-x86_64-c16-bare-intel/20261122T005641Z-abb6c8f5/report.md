# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### acc_plain dominates: 10% faster than the next best (acc_atomic_relaxed)

acc_plain (16.60 us) leads acc_atomic_relaxed (18.27 us) by 10%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### acc_plain is fastest but the noisiest (CV 10.1%)

acc_plain wins on median (16.60 us) yet has the highest variance (CV 10.1%), while acc_atomic_seqcst is the steadiest (CV 7.8%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.66 us) is smaller than the fastest variant's own run-to-run std-dev (1.67 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### acc_atomic_seqcst's edge over baseline is significant but tiny (1 ns, 0.00%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by 1 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_plain** at 16603.3 ns median (-9.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.10x (fastest 16603.3 ns, slowest 18267.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 2112780760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 18827ns | 20148ns | 16480ns | 19088ns | 20391ns | base |
| acc_atomic_seqcst | 19289ns | 20150ns | 16485ns | 19859ns | 20382ns | +2.45% |
| acc_plain | 18337ns | 18363ns | 16482ns | 18326ns | 20227ns | -2.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 17062ns | 14938ns | 18469ns | base | 0.960 |
| acc_atomic_seqcst | 17445ns | 14942ns | 18281ns | +2.25% | 0.939 |
| acc_plain | 16613ns | 14937ns | 18316ns | -2.63% | 0.986 |

## Performance model

- Peak throughput: **1.097 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.897 | 81.8% |
| acc_atomic_seqcst | 0.897 | 81.8% |
| acc_plain | 0.987 | 90.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 18827ns | 18827ns | base |
| acc_atomic_seqcst | 19289ns | 19289ns | +2.45% |
| acc_plain | 18337ns | 18337ns | -2.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 18268ns | base | --- | [18257, 18268] | --- | --- | --- | --- |
| acc_atomic_seqcst | 18268ns | +0.6ns (+0.0%) | [+0, +1]ns | [18268, 18268] | YES | 0.0325 | 0.0325 | 0 |
| acc_plain | 16603ns | -1.2ns (-0.0%) | [-2, -0]ns | [14948, 18257] | YES | 0.0012 | 0.0006 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 18268ns | +0.0% | -0.0% |
| 1 | 2 | 18269ns | -0.0% | -0.0% |
| 1 | 3 | 14938ns | +0.1% | +0.0% |
| 1 | 4 | 14947ns | +22.2% | -0.1% |
| 1 | 5 | 18268ns | -0.0% | +0.0% |
| 1 | 6 | 18269ns | +0.0% | -0.0% |
| 1 | 7 | 18268ns | -18.2% | -18.2% |
| 1 | 8 | 14948ns | +22.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.059 | ok |
| acc_atomic_seqcst | -0.376 | moderate- |
| acc_plain | 0.125 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 1/8, lost 2/8
- **acc_plain**: won 1/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 22.3ns | 17061.8ns | 0.1% |  |
| acc_atomic_seqcst | 22.8ns | 17445.2ns | 0.1% |  |
| acc_plain | 22.0ns | 16613.4ns | 0.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 14938.4-18468.9 ns)
  14938.4 |################
  15114.9 |
  15291.4 |
  15468.0 |
  15644.5 |
  15821.0 |
  15997.5 |
  16174.1 |
  16350.6 |
  16527.1 |
  16703.6 |
  16880.2 |
  17056.7 |
  17233.2 |
  17409.7 |
  17586.3 |
  17762.8 |
  17939.3 |
  18115.8 |########################################
  18292.4 |
  (19 below, 2 above range)

acc_atomic_seqcst (n=160, range 14942.1-18280.8 ns)
  14942.1 |######
  15109.0 |
  15276.0 |
  15442.9 |
  15609.8 |
  15776.8 |
  15943.7 |
  16110.6 |
  16277.6 |
  16444.5 |
  16611.4 |
  16778.4 |
  16945.3 |
  17112.2 |
  17279.2 |
  17446.1 |
  17613.0 |
  17780.0 |
  17946.9 |
  18113.8 |########################################
  (18 below, 1 above range)

acc_plain (n=160, range 14936.9-18316.1 ns)
  14936.9 |########################################
  15105.9 |
  15274.8 |
  15443.8 |
  15612.8 |
  15781.7 |
  15950.7 |
  16119.6 |
  16288.6 |
  16457.6 |
  16626.5 |
  16795.5 |
  16964.5 |
  17133.4 |
  17302.4 |
  17471.3 |
  17640.3 |
  17809.3 |
  17978.2 |
  18147.2 |#######################################
  (3 below, 4 above range)

```
