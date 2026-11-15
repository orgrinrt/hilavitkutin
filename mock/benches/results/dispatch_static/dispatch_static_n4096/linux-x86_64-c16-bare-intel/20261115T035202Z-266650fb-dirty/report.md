# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### dispatch_const_ptr is fastest but the noisiest (CV 70.3%)

dispatch_const_ptr wins on median (578 ns) yet has the highest variance (CV 70.3%), while dispatch_trait is the steadiest (CV 61.2%, 578 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (406 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (dispatch_const_ptr)

The baseline dispatch_const_ptr is the fastest (578 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### dispatch_trait's comparison is tie-heavy (13% tied pairs)

13% of paired samples for dispatch_trait are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader dispatch_const_ptr vs stability leader dispatch_trait (+0% speed for 1.1x steadier)

dispatch_const_ptr is fastest (578 ns, CV 70.3%); dispatch_trait gives up 0.0% median for 1.1x lower variance (CV 61.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### dispatch_const_ptr is inconsistent: worst-20% is 2.9x its best-20%

dispatch_const_ptr's best 20% of batches run at 473 ns but its worst 20% at 1.38 us (2.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### Whole field within 0.0% of the fastest

All 3 variants sit between 578 ns and 578 ns - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_trait's edge over baseline is significant but tiny (0 ns, 0.03%)

dispatch_trait differs from baseline dispatch_const_ptr by 0 ns (0.03%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (dispatch_const_ptr) is the fastest** at 577.5 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 577.5 ns, slowest 577.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Static dispatch shapes: direct vs sealed-trait vs const fn pointer |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x7eedfacedeadbeef |
| Counter frequency | 2112812250 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 3054ns | 2455ns | 2010ns | 2451ns | 5908ns | base |
| dispatch_direct | 2977ns | 2455ns | 2256ns | 2456ns | 5257ns | -2.55% |
| dispatch_trait | 2974ns | 2456ns | 2448ns | 2458ns | 5047ns | -2.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 716ns | 473ns | 1380ns | base | 5.717 |
| dispatch_direct | 699ns | 532ns | 1230ns | -2.41% | 5.858 |
| dispatch_trait | 698ns | 577ns | 1179ns | -2.59% | 5.869 |

## Performance model

- Peak throughput: **8.668 Gops/s** (dispatch_const_ptr; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 7.093 | 81.8% |
| dispatch_direct | 7.091 | 81.8% |
| dispatch_trait | 7.091 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 3054ns | 3054ns | base |
| dispatch_direct | 2977ns | 2977ns | -2.55% |
| dispatch_trait | 2974ns | 2974ns | -2.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 578ns | base | --- | [577, 578] | --- | --- | --- | --- |
| dispatch_direct | 578ns | no significant difference | [+0, +0]ns | [578, 578] | no (adj: YES) | 0.0181 | 0.0181 | **19** (12%, HIGH) |
| dispatch_trait | 578ns | +0.2ns (+0.0%) | [+0, +0]ns | [578, 578] | YES | 0.0001 | 0.0000 | **21** (13%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 578ns | +0.0% | +0.0% |
| 1 | 2 | 578ns | +0.0% | +0.0% |
| 1 | 3 | 473ns | +22.2% | +22.2% |
| 1 | 4 | 578ns | +0.0% | +0.0% |
| 1 | 5 | 578ns | -0.0% | -0.0% |
| 1 | 6 | 472ns | +22.2% | +22.2% |
| 1 | 7 | 578ns | +0.0% | +0.0% |
| 1 | 8 | 578ns | -18.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.376 | moderate- |
| dispatch_direct | -0.018 | ok |
| dispatch_trait | 0.311 | moderate+ |

**Consistency summary:**

- **dispatch_direct**: won 1/8, lost 2/8
- **dispatch_trait**: won 0/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 29.5ns | 716.5ns | 4.1% |  |
| dispatch_direct | 28.9ns | 699.2ns | 4.1% |  |
| dispatch_trait | 28.8ns | 697.9ns | 4.1% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 472.6-1380.3 ns)
    472.6 |#####
    517.9 |
    563.3 |########################################
    608.7 |
    654.1 |
    699.5 |
    744.9 |
    790.3 |
    835.7 |
    881.0 |
    926.4 |
    971.8 |
   1017.2 |
   1062.6 |
   1108.0 |
   1153.4 |
   1198.8 |
   1244.1 |
   1289.5 |
   1334.9 |
  (18 below, 18 above range)

dispatch_direct (n=160, range 531.9-1230.5 ns)
    531.9 |
    566.8 |########################################
    601.7 |
    636.7 |
    671.6 |
    706.5 |
    741.5 |
    776.4 |
    811.3 |
    846.2 |
    881.2 |
    916.1 |
    951.0 |
    986.0 |
   1020.9 |
   1055.8 |
   1090.8 |
   1125.7 |
   1160.6 |
   1195.5 |
  (14 below, 18 above range)

dispatch_trait (n=160, range 577.4-1179.4 ns)
    577.4 |########################################
    607.5 |
    637.6 |
    667.7 |
    697.8 |
    727.9 |
    758.0 |
    788.1 |
    818.2 |
    848.3 |
    878.4 |
    908.5 |
    938.6 |
    968.7 |
    998.8 |
   1028.9 |
   1059.0 |
   1089.1 |
   1119.2 |
   1149.3 |
  (6 below, 16 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=56.7% (high variance, measurements may be unstable)
- **dispatch_direct**: CV=52.5% (high variance, measurements may be unstable)
- **dispatch_trait**: CV=50.6% (high variance, measurements may be unstable)
