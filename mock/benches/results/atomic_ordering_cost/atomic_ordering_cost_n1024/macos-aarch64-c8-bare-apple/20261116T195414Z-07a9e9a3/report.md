# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 260% faster than the next best (atomic_acquire)

atomic_relaxed (230 ns) leads atomic_acquire (826 ns) by 260%, a clear separation rather than a photo finish. CV 7.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 72% (significant)

atomic_relaxed is -597 ns (72%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.6x slower than the field

atomic_seqcst (830 ns) is 3.6x the fastest (230 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_relaxed is fastest but the noisiest (CV 7.2%)

atomic_relaxed wins on median (230 ns) yet has the highest variance (CV 7.2%), while atomic_acquire is the steadiest (CV 2.3%, 826 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 3.6x the fastest

Fastest atomic_relaxed (230 ns) to slowest atomic_seqcst (830 ns): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (4 ns, 0.45%)

atomic_seqcst differs from baseline atomic_acquire by 4 ns (0.45%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 229.6 ns median (-72.2% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.62x (fastest 229.6 ns, slowest 830.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 2968ns | 2941ns | 2933ns | 2946ns | 3067ns | base |
| atomic_relaxed | 2346ns | 2325ns | 2316ns | 2332ns | 2416ns | -20.96% |
| atomic_seqcst | 2956ns | 2942ns | 2935ns | 2942ns | 3020ns | -0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 834ns | 821ns | 864ns | base | 1.229 |
| atomic_relaxed | 232ns | 226ns | 248ns | -72.11% | 4.405 |
| atomic_seqcst | 834ns | 825ns | 853ns | +0.06% | 1.228 |

## Performance model

- Peak throughput: **4.540 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.239 | 27.3% |
| atomic_relaxed | 4.460 | 98.2% |
| atomic_seqcst | 1.233 | 27.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2968ns | 2968ns | base |
| atomic_relaxed | 2346ns | 2346ns | -20.96% |
| atomic_seqcst | 2956ns | 2956ns | -0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 826ns | base | --- | [826, 827] | --- | --- | --- | --- |
| atomic_relaxed | 230ns | -596.9ns (-72.2%) | [-598, -596]ns | [229, 230] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 830ns | +3.8ns (+0.5%) | [+3, +5]ns | [830, 831] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 826ns | -72.2% | +2.2% |
| 1 | 2 | 846ns | -72.7% | -1.9% |
| 1 | 3 | 836ns | -72.7% | -0.7% |
| 1 | 4 | 827ns | -71.8% | +0.4% |
| 1 | 5 | 826ns | -72.3% | +0.5% |
| 1 | 6 | 825ns | -72.2% | +0.6% |
| 1 | 7 | 826ns | -72.3% | +0.5% |
| 1 | 8 | 826ns | -72.4% | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.207 | moderate+ |
| atomic_relaxed | -0.466 | moderate- |
| atomic_seqcst | -0.022 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 2/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 2.9ns | 833.5ns | 0.4% |  |
| atomic_relaxed | 2.5ns | 232.4ns | 1.1% |  |
| atomic_seqcst | 2.8ns | 834.0ns | 0.3% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 821.4-864.4 ns)
    821.4 |########################
    823.6 |###########################
    825.7 |########################################
    827.9 |##################
    830.0 |####
    832.2 |####
    834.3 |#####
    836.4 |#
    838.6 |##
    840.7 |#####
    842.9 |##
    845.0 |##
    847.2 |
    849.3 |#
    851.5 |#
    853.6 |
    855.8 |##
    857.9 |##
    860.1 |
    862.2 |#
  (15 below, 13 above range)

atomic_relaxed (n=160, range 225.6-247.5 ns)
    225.6 |#######################
    226.7 |########################################
    227.8 |######################################
    228.9 |#######################
    230.0 |##################################
    231.1 |#####################
    232.2 |####################
    233.3 |################
    234.3 |#####
    235.4 |#########
    236.5 |#
    237.6 |
    238.7 |###
    239.8 |#
    240.9 |###
    242.0 |#
    243.1 |
    244.2 |#
    245.3 |
    246.4 |
  (14 below, 10 above range)

atomic_seqcst (n=160, range 825.4-853.2 ns)
    825.4 |##########
    826.8 |###################
    828.2 |###################
    829.6 |########################################
    831.0 |###############################
    832.4 |#######################
    833.7 |#########
    835.1 |###
    836.5 |#
    837.9 |
    839.3 |
    840.7 |
    842.1 |#
    843.5 |#
    844.9 |
    846.2 |#
    847.6 |#
    849.0 |
    850.4 |
    851.8 |##
  (15 below, 8 above range)

```
