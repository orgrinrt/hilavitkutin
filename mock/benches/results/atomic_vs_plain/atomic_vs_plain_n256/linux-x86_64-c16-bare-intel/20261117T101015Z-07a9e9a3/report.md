# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (acc_plain, acc_atomic_relaxed) are a dead heat (<1%)

acc_plain (250 ns) and acc_atomic_relaxed (250 ns) differ by 0.22%, inside the noise, even though the wider field spreads 63.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (158 ns) is smaller than the fastest variant's own run-to-run std-dev (524 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### acc_atomic_relaxed is inconsistent: worst-20% is 5.4x its best-20%

acc_atomic_relaxed's best 20% of batches run at 239 ns but its worst 20% at 1.30 us (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: acc_plain** at 249.5 ns median (-0.2% vs baseline)
- Spread: 1.63x (fastest 249.5 ns, slowest 407.8 ns)

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
| acc_atomic_relaxed | 3794ns | 1843ns | 1780ns | 2544ns | 9560ns | base |
| acc_atomic_seqcst | 3503ns | 3106ns | 1780ns | 3121ns | 6372ns | -7.68% |
| acc_plain | 3828ns | 1806ns | 1779ns | 2642ns | 9431ns | +0.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 513ns | 239ns | 1300ns | base | 0.499 |
| acc_atomic_seqcst | 471ns | 239ns | 862ns | -8.02% | 0.543 |
| acc_plain | 513ns | 239ns | 1262ns | +0.01% | 0.499 |

## Performance model

- Peak throughput: **1.071 Gops/s** (acc_atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 1.024 | 95.6% |
| acc_atomic_seqcst | 0.628 | 58.6% |
| acc_plain | 1.026 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 3794ns | 3794ns | base |
| acc_atomic_seqcst | 3503ns | 3503ns | -7.68% |
| acc_plain | 3828ns | 3828ns | +0.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 250ns | base | --- | [239, 316] | --- | --- | --- | --- |
| acc_atomic_seqcst | 408ns | no significant difference | [-8, +30]ns | [331, 481] | no | 1.0000 | 1.0000 | 7 |
| acc_plain | 250ns | no significant difference | [-5, +0]ns | [240, 347] | no | 0.3521 | 0.1760 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 239ns | +0.0% | +0.1% |
| 1 | 2 | 453ns | -39.2% | -47.2% |
| 1 | 3 | 803ns | -28.5% | -14.2% |
| 1 | 4 | 618ns | -61.2% | -61.3% |
| 1 | 5 | 239ns | +101.2% | +70.0% |
| 1 | 6 | 239ns | +142.9% | +157.0% |
| 1 | 7 | 242ns | -1.0% | -0.7% |
| 1 | 8 | 239ns | +144.4% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | 0.428 | moderate+ |
| acc_atomic_seqcst | -0.440 | moderate- |
| acc_plain | -0.305 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 4/8, lost 3/8
- **acc_plain**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 41.2ns | 512.5ns | 8.0% | HIGH |
| acc_atomic_seqcst | 38.2ns | 471.4ns | 8.1% | HIGH |
| acc_plain | 41.6ns | 512.6ns | 8.1% | HIGH |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 239.1-1299.9 ns)
    239.1 |########################################
    292.1 |###
    345.1 |###
    398.2 |##
    451.2 |##
    504.3 |##
    557.3 |#
    610.4 |#
    663.4 |##
    716.5 |#####
    769.5 |###
    822.5 |#
    875.6 |#
    928.6 |#
    981.7 |#
   1034.7 |
   1087.8 |
   1140.8 |
   1193.8 |
   1246.9 |
  (12 below, 8 above range)

acc_atomic_seqcst (n=160, range 239.1-861.8 ns)
    239.1 |########################################
    270.2 |##############
    301.4 |
    332.5 |###
    363.6 |###
    394.8 |#####
    425.9 |##
    457.0 |###########
    488.2 |#
    519.3 |##
    550.4 |####
    581.6 |#####
    612.7 |#########
    643.8 |########
    675.0 |#######
    706.1 |##
    737.2 |###
    768.4 |#
    799.5 |
    830.6 |
  (6 below, 8 above range)

acc_plain (n=160, range 239.3-1262.4 ns)
    239.3 |########################################
    290.4 |#
    341.6 |###
    392.7 |##
    443.9 |###
    495.1 |##
    546.2 |#
    597.4 |##
    648.5 |####
    699.7 |###
    750.8 |##
    802.0 |##
    853.2 |#
    904.3 |##
    955.5 |#
   1006.6 |
   1057.8 |
   1108.9 |
   1160.1 |
   1211.2 |
  (11 below, 8 above range)

```

## Diagnostics

- **acc_atomic_relaxed**: CV=107.0% (high variance, measurements may be unstable)
- **acc_atomic_relaxed**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **acc_atomic_seqcst**: CV=61.8% (high variance, measurements may be unstable)
- **acc_atomic_seqcst**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **acc_plain**: CV=102.3% (high variance, measurements may be unstable)
- **acc_plain**: worst_20/best_20 = 5.3x (possible bimodal distribution)
