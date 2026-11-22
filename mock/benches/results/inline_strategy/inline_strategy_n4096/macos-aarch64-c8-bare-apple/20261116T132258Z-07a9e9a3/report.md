# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (5.06 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.0% of the fastest

All 3 variants sit between 5.06 us and 5.11 us - a 1.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### inline_never's edge over baseline is significant but tiny (49 ns, 0.98%)

inline_never differs from baseline inline_always by 49 ns (0.98%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (inline_always) is the fastest** at 5062.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 5062.1 ns, slowest 5111.9 ns)

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
| inline_always | 7170ns | 7163ns | 7157ns | 7163ns | 7205ns | base |
| inline_default | 7195ns | 7163ns | 7157ns | 7165ns | 7325ns | +0.35% |
| inline_never | 7254ns | 7194ns | 7185ns | 7199ns | 7487ns | +1.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 5066ns | 5057ns | 5089ns | base | 0.808 |
| inline_default | 5086ns | 5057ns | 5188ns | +0.40% | 0.805 |
| inline_never | 5151ns | 5105ns | 5301ns | +1.66% | 0.795 |

## Performance model

- Peak throughput: **0.810 Gops/s** (inline_always; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.809 | 99.9% |
| inline_default | 0.809 | 99.9% |
| inline_never | 0.801 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 7170ns | 7170ns | base |
| inline_default | 7195ns | 7195ns | +0.35% |
| inline_never | 7254ns | 7254ns | +1.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 5062ns | base | --- | [5061, 5063] | --- | --- | --- | --- |
| inline_default | 5062ns | no significant difference | [-0, +1]ns | [5061, 5063] | no | 0.5179 | 0.5179 | 7 |
| inline_never | 5112ns | +49.4ns (+1.0%) | [+49, +51]ns | [5111, 5113] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 5061ns | +0.0% | +1.0% |
| 1 | 2 | 5062ns | -0.0% | +1.0% |
| 1 | 3 | 5062ns | -0.0% | +1.0% |
| 1 | 4 | 5063ns | -0.0% | +0.9% |
| 1 | 5 | 5063ns | +0.0% | +4.5% |
| 1 | 6 | 5062ns | +0.0% | +1.0% |
| 1 | 7 | 5062ns | -0.0% | +1.0% |
| 1 | 8 | 5061ns | +0.1% | +1.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.131 | ok |
| inline_default | 0.195 | ok |
| inline_never | -0.168 | ok |

**Consistency summary:**

- **inline_default**: won 0/8, lost 0/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.5ns | 5066.4ns | 0.1% |  |
| inline_default | 2.6ns | 5086.4ns | 0.1% |  |
| inline_never | 3.1ns | 5150.5ns | 0.1% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 5057.2-5089.1 ns)
   5057.2 |####################
   5058.8 |#################################
   5060.4 |############################
   5062.0 |########################################
   5063.6 |########################
   5065.2 |###########
   5066.8 |######
   5068.4 |###
   5070.0 |
   5071.6 |
   5073.1 |
   5074.7 |
   5076.3 |#
   5077.9 |
   5079.5 |
   5081.1 |
   5082.7 |
   5084.3 |##
   5085.9 |
   5087.5 |#
  (15 below, 10 above range)

inline_default (n=160, range 5057.2-5187.7 ns)
   5057.2 |########################################
   5063.8 |#############
   5070.3 |
   5076.8 |
   5083.3 |#
   5089.9 |
   5096.4 |
   5102.9 |#
   5109.4 |
   5115.9 |
   5122.5 |
   5129.0 |
   5135.5 |
   5142.0 |
   5148.6 |
   5155.1 |
   5161.6 |
   5168.1 |
   5174.6 |
   5181.2 |
  (16 below, 13 above range)

inline_never (n=160, range 5105.0-5301.3 ns)
   5105.0 |########################################
   5114.8 |#####
   5124.6 |##
   5134.5 |
   5144.3 |
   5154.1 |
   5163.9 |
   5173.7 |
   5183.5 |
   5193.3 |
   5203.2 |
   5213.0 |
   5222.8 |
   5232.6 |
   5242.4 |
   5252.2 |
   5262.0 |#
   5271.9 |
   5281.7 |###
   5291.5 |#
  (12 below, 12 above range)

```
