# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.7% (fastest 5.28 us to slowest 5.42 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (140 ns) is smaller than the fastest variant's own run-to-run std-dev (207 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (acc_atomic_relaxed)

The baseline acc_atomic_relaxed is the fastest (5.28 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader acc_atomic_relaxed vs stability leader acc_plain (+3% speed for 1.1x steadier)

acc_atomic_relaxed is fastest (5.28 us, CV 3.9%); acc_plain gives up 2.7% median for 1.1x lower variance (CV 3.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (acc_atomic_relaxed) is the fastest** at 5280.6 ns median
- Spread: 1.03x (fastest 5280.6 ns, slowest 5421.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 7495ns | 7447ns | 7157ns | 7481ns | 7875ns | base |
| acc_atomic_seqcst | 7542ns | 7603ns | 7157ns | 7501ns | 8050ns | +0.63% |
| acc_plain | 7522ns | 7654ns | 7156ns | 7547ns | 7816ns | +0.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 5302ns | 5062ns | 5577ns | base | 0.772 |
| acc_atomic_seqcst | 5334ns | 5060ns | 5694ns | +0.60% | 0.768 |
| acc_plain | 5323ns | 5061ns | 5536ns | +0.38% | 0.770 |

## Performance model

- Peak throughput: **0.809 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.776 | 95.8% |
| acc_atomic_seqcst | 0.762 | 94.1% |
| acc_plain | 0.756 | 93.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 7495ns | 7495ns | base |
| acc_atomic_seqcst | 7542ns | 7542ns | +0.63% |
| acc_plain | 7522ns | 7522ns | +0.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 5281ns | base | --- | [5243, 5410] | --- | --- | --- | --- |
| acc_atomic_seqcst | 5376ns | no significant difference | [+0, +52]ns | [5255, 5425] | no | 0.1328 | 0.0788 | 3 |
| acc_plain | 5421ns | no significant difference | [-2, +65]ns | [5312, 5425] | no | 0.1328 | 0.1328 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 5425ns | -6.0% | -0.0% |
| 1 | 2 | 5427ns | +0.0% | -1.0% |
| 1 | 3 | 5093ns | +6.4% | +3.5% |
| 1 | 4 | 5429ns | -0.0% | -6.4% |
| 1 | 5 | 5238ns | +2.5% | +3.6% |
| 1 | 6 | 5093ns | +6.7% | +6.5% |
| 1 | 7 | 5268ns | -3.9% | +3.1% |
| 1 | 8 | 5106ns | +6.3% | +6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.189 | ok |
| acc_atomic_seqcst | -0.294 | moderate- |
| acc_plain | 0.165 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 2/8, lost 4/8
- **acc_plain**: won 2/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.6ns | 5302.3ns | 0.0% |  |
| acc_atomic_seqcst | 2.7ns | 5334.2ns | 0.0% |  |
| acc_plain | 2.8ns | 5322.7ns | 0.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 5062.4-5577.4 ns)
   5062.4 |#####################
   5088.2 |######
   5113.9 |#
   5139.7 |####
   5165.4 |##
   5191.2 |
   5216.9 |##########
   5242.7 |######
   5268.4 |#########
   5294.2 |#
   5319.9 |#
   5345.7 |#
   5371.4 |
   5397.2 |#########
   5422.9 |########################################
   5448.7 |##
   5474.4 |
   5500.2 |
   5525.9 |
   5551.7 |
  (17 below, 11 above range)

acc_atomic_seqcst (n=160, range 5060.5-5693.8 ns)
   5060.5 |#####################
   5092.1 |##
   5123.8 |#####
   5155.5 |
   5187.1 |#
   5218.8 |#######
   5250.5 |###
   5282.1 |#
   5313.8 |
   5345.4 |##
   5377.1 |#
   5408.8 |########################################
   5440.4 |#
   5472.1 |
   5503.8 |
   5535.4 |
   5567.1 |
   5598.8 |
   5630.4 |
   5662.1 |
  (15 below, 18 above range)

acc_plain (n=160, range 5061.2-5535.8 ns)
   5061.2 |############
   5084.9 |#
   5108.6 |##
   5132.4 |
   5156.1 |
   5179.8 |#
   5203.6 |
   5227.3 |#####
   5251.0 |#####
   5274.7 |##
   5298.5 |####
   5322.2 |
   5345.9 |
   5369.7 |#
   5393.4 |#
   5417.1 |########################################
   5440.9 |#
   5464.6 |###
   5488.3 |#
   5512.1 |#
  (17 below, 8 above range)

```
