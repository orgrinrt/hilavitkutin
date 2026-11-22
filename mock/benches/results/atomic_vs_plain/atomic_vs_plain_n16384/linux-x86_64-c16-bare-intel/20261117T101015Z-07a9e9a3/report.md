# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 18.26 us to slowest 18.27 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### acc_atomic_relaxed is fastest but the noisiest (CV 53.9%)

acc_atomic_relaxed wins on median (18.26 us) yet has the highest variance (CV 53.9%), while acc_atomic_seqcst is the steadiest (CV 7.9%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (12 ns) is smaller than the fastest variant's own run-to-run std-dev (9.84 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (18.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader acc_atomic_relaxed vs stability leader acc_atomic_seqcst (+0% speed for 6.8x steadier)

acc_atomic_relaxed is fastest (18.26 us, CV 53.9%); acc_atomic_seqcst gives up 0.1% median for 6.8x lower variance (CV 7.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 18256.3 ns median
- Spread: 1.00x (fastest 18256.3 ns, slowest 18268.0 ns)

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
| Counter frequency | 2112846992 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 19252ns | 20148ns | 16483ns | 18429ns | 24489ns | base |
| acc_atomic_seqcst | 19248ns | 20155ns | 16486ns | 19852ns | 20198ns | -0.02% |
| acc_plain | 18852ns | 20153ns | 16486ns | 19170ns | 20263ns | -2.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 17444ns | 14936ns | 22183ns | base | 0.939 |
| acc_atomic_seqcst | 17441ns | 14943ns | 18291ns | -0.02% | 0.939 |
| acc_plain | 17083ns | 14938ns | 18363ns | -2.07% | 0.959 |

## Performance model

- Peak throughput: **1.097 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.897 | 81.8% |
| acc_atomic_seqcst | 0.897 | 81.8% |
| acc_plain | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 19252ns | 19252ns | base |
| acc_atomic_seqcst | 19248ns | 19248ns | -0.02% |
| acc_plain | 18852ns | 18852ns | -2.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 18256ns | base | --- | [14953, 18266] | --- | --- | --- | --- |
| acc_atomic_seqcst | 18268ns | no significant difference | [-0, +11]ns | [18267, 18268] | no | 0.4087 | 0.2043 | 1 |
| acc_plain | 18267ns | no significant difference | [-1, +2]ns | [18257, 18268] | no | 0.5777 | 0.5777 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 18268ns | -18.2% | -0.0% |
| 1 | 2 | 18273ns | -0.0% | -18.2% |
| 1 | 3 | 14948ns | -0.0% | -0.1% |
| 1 | 4 | 14947ns | +22.2% | -0.0% |
| 1 | 5 | 14947ns | +22.2% | +22.2% |
| 1 | 6 | 18267ns | +0.0% | +0.0% |
| 1 | 7 | 14939ns | +22.3% | +22.3% |
| 1 | 8 | 18270ns | -0.0% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.126 | ok |
| acc_atomic_seqcst | -0.209 | moderate- |
| acc_plain | 0.393 | moderate+ |

**Consistency summary:**

- **acc_atomic_seqcst**: won 1/8, lost 3/8
- **acc_plain**: won 1/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 22.9ns | 17443.7ns | 0.1% |  |
| acc_atomic_seqcst | 22.8ns | 17440.7ns | 0.1% |  |
| acc_plain | 22.6ns | 17083.2ns | 0.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 14936.4-22183.1 ns)
  14936.4 |###################################
  15298.8 |
  15661.1 |
  16023.4 |
  16385.7 |
  16748.1 |
  17110.4 |
  17472.7 |
  17835.1 |
  18197.4 |########################################
  18559.7 |#
  18922.1 |
  19284.4 |
  19646.7 |
  20009.1 |
  20371.4 |
  20733.7 |
  21096.1 |
  21458.4 |
  21820.7 |
  (6 below, 1 above range)

acc_atomic_seqcst (n=160, range 14942.9-18291.2 ns)
  14942.9 |########
  15110.3 |
  15277.7 |
  15445.2 |
  15612.6 |
  15780.0 |
  15947.4 |
  16114.8 |
  16282.2 |
  16449.6 |
  16617.0 |
  16784.5 |
  16951.9 |
  17119.3 |
  17286.7 |
  17454.1 |
  17621.5 |
  17788.9 |
  17956.3 |
  18123.8 |########################################
  (15 below, 6 above range)

acc_plain (n=160, range 14937.9-18363.1 ns)
  14937.9 |################
  15109.2 |
  15280.4 |
  15451.7 |
  15622.9 |
  15794.2 |
  15965.5 |
  16136.7 |
  16308.0 |
  16479.2 |
  16650.5 |
  16821.7 |
  16993.0 |
  17164.3 |
  17335.5 |
  17506.8 |
  17678.0 |
  17849.3 |
  18020.6 |
  18191.8 |########################################
  (16 below, 2 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=56.4% (high variance, measurements may be unstable)
