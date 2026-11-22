# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (5.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_atomic_seqcst at 5.06 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### acc_atomic_relaxed shows alternating (throttle bounce) (autocorr -0.53)

acc_atomic_relaxed's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 1.8% of the fastest

All 3 variants sit between 5.06 us and 5.15 us - a 1.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_atomic_seqcst's edge over baseline is significant but tiny (-14 ns, 0.27%)

acc_atomic_seqcst differs from baseline acc_atomic_relaxed by -14 ns (0.27%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_atomic_seqcst** at 5064.6 ns median (-1.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.02x (fastest 5064.6 ns, slowest 5154.4 ns)

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
| acc_atomic_relaxed | 7261ns | 7283ns | 7152ns | 7250ns | 7402ns | base |
| acc_atomic_seqcst | 7192ns | 7158ns | 7147ns | 7170ns | 7305ns | -0.95% |
| acc_plain | 7254ns | 7280ns | 7150ns | 7244ns | 7391ns | -0.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 5136ns | 5060ns | 5238ns | base | 0.798 |
| acc_atomic_seqcst | 5089ns | 5058ns | 5170ns | -0.91% | 0.805 |
| acc_plain | 5132ns | 5059ns | 5232ns | -0.08% | 0.798 |

## Performance model

- Peak throughput: **0.810 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.795 | 98.1% |
| acc_atomic_seqcst | 0.809 | 99.9% |
| acc_plain | 0.795 | 98.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 7261ns | 7261ns | base |
| acc_atomic_seqcst | 7192ns | 7192ns | -0.95% |
| acc_plain | 7254ns | 7254ns | -0.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 5154ns | base | --- | [5133, 5156] | --- | --- | --- | --- |
| acc_atomic_seqcst | 5065ns | -14.0ns (-0.3%) | [-52, -3]ns | [5063, 5067] | YES | 0.0000 | 0.0000 | 3 |
| acc_plain | 5151ns | no significant difference | [-14, +2]ns | [5101, 5155] | no | 0.4741 | 0.4741 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 5076ns | -0.2% | +1.7% |
| 1 | 2 | 5102ns | -0.7% | -0.1% |
| 1 | 3 | 5159ns | -1.4% | -1.8% |
| 1 | 4 | 5134ns | -1.4% | +0.5% |
| 1 | 5 | 5065ns | +0.2% | +1.9% |
| 1 | 6 | 5158ns | -1.8% | -0.1% |
| 1 | 7 | 5064ns | -0.0% | -0.0% |
| 1 | 8 | 5165ns | -2.0% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.527 | HIGH- (thermal bounce) |
| acc_atomic_seqcst | -0.315 | moderate- |
| acc_plain | 0.143 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 6/8, lost 1/8
- **acc_plain**: won 3/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.7ns | 5135.7ns | 0.1% |  |
| acc_atomic_seqcst | 2.7ns | 5088.9ns | 0.1% |  |
| acc_plain | 2.7ns | 5131.6ns | 0.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 5059.9-5237.6 ns)
   5059.9 |########################################
   5068.8 |#####
   5077.7 |####
   5086.6 |
   5095.5 |##
   5104.4 |###
   5113.2 |##
   5122.1 |
   5131.0 |
   5139.9 |
   5148.8 |#################################
   5157.6 |############################
   5166.5 |####
   5175.4 |###
   5184.3 |###
   5193.2 |##
   5202.1 |#
   5210.9 |#
   5219.8 |#
   5228.7 |#
  (14 below, 17 above range)

acc_atomic_seqcst (n=160, range 5057.5-5169.7 ns)
   5057.5 |########################################
   5063.1 |#########################
   5068.8 |##
   5074.4 |#######
   5080.0 |####
   5085.6 |
   5091.2 |
   5096.8 |
   5102.4 |###
   5108.0 |
   5113.6 |
   5119.2 |
   5124.8 |
   5130.5 |
   5136.1 |#
   5141.7 |
   5147.3 |
   5152.9 |########
   5158.5 |######
   5164.1 |#
  (18 below, 9 above range)

acc_plain (n=160, range 5059.3-5231.5 ns)
   5059.3 |########################################
   5067.9 |####
   5076.5 |###
   5085.1 |##
   5093.7 |######
   5102.4 |#
   5111.0 |##
   5119.6 |#
   5128.2 |###
   5136.8 |##
   5145.4 |##########
   5154.0 |###################################
   5162.6 |###########
   5171.2 |#
   5179.8 |##
   5188.5 |
   5197.1 |###
   5205.7 |###
   5214.3 |####
   5222.9 |#
  (17 below, 14 above range)

```
