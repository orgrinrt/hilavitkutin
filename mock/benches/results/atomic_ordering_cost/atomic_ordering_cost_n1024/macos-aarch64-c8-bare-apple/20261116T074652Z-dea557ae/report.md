# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 264% faster than the next best (atomic_acquire)

atomic_relaxed (231 ns) leads atomic_acquire (842 ns) by 264%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 73% (significant)

atomic_relaxed is -611 ns (73%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.7x slower than the field

atomic_seqcst (846 ns) is 3.7x the fastest (231 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.7x the fastest

Fastest atomic_relaxed (231 ns) to slowest atomic_seqcst (846 ns): 3.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (4 ns, 0.50%)

atomic_seqcst differs from baseline atomic_acquire by 4 ns (0.50%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 231.2 ns median (-72.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.66x (fastest 231.2 ns, slowest 846.2 ns)

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
| atomic_acquire | 3007ns | 2985ns | 2975ns | 2987ns | 3100ns | base |
| atomic_relaxed | 2350ns | 2351ns | 2313ns | 2349ns | 2392ns | -21.84% |
| atomic_seqcst | 2993ns | 2986ns | 2974ns | 2986ns | 3033ns | -0.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 848ns | 837ns | 875ns | base | 1.208 |
| atomic_relaxed | 232ns | 227ns | 240ns | -72.64% | 4.415 |
| atomic_seqcst | 849ns | 841ns | 863ns | +0.11% | 1.207 |

## Performance model

- Peak throughput: **4.516 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.216 | 26.9% |
| atomic_relaxed | 4.429 | 98.1% |
| atomic_seqcst | 1.210 | 26.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 3007ns | 3007ns | base |
| atomic_relaxed | 2350ns | 2350ns | -21.84% |
| atomic_seqcst | 2993ns | 2993ns | -0.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 842ns | base | --- | [842, 843] | --- | --- | --- | --- |
| atomic_relaxed | 231ns | -610.8ns (-72.5%) | [-611, -610]ns | [230, 232] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 846ns | +4.2ns (+0.5%) | [+4, +5]ns | [846, 847] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 841ns | -72.3% | +0.5% |
| 1 | 2 | 841ns | -72.3% | +0.6% |
| 1 | 3 | 842ns | -72.6% | +0.6% |
| 1 | 4 | 842ns | -72.3% | +0.5% |
| 1 | 5 | 842ns | -72.5% | +1.1% |
| 1 | 6 | 842ns | -72.6% | +0.8% |
| 1 | 7 | 885ns | -74.2% | -4.4% |
| 1 | 8 | 842ns | -72.9% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.135 | ok |
| atomic_relaxed | 0.399 | moderate+ |
| atomic_seqcst | 0.162 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.0ns | 847.7ns | 0.4% |  |
| atomic_relaxed | 2.7ns | 232.0ns | 1.1% |  |
| atomic_seqcst | 3.0ns | 848.7ns | 0.4% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 836.7-875.1 ns)
    836.7 |##########
    838.6 |############################
    840.5 |########################################
    842.4 |######################################
    844.3 |####################
    846.3 |####
    848.2 |
    850.1 |#
    852.0 |
    853.9 |
    855.9 |#
    857.8 |##
    859.7 |
    861.6 |
    863.6 |#
    865.5 |
    867.4 |#
    869.3 |
    871.2 |
    873.2 |
  (15 below, 18 above range)

atomic_relaxed (n=160, range 226.7-239.8 ns)
    226.7 |
    227.4 |##################
    228.0 |##########
    228.7 |#####################
    229.3 |########################################
    230.0 |#########
    230.7 |######################################
    231.3 |##############
    232.0 |#######################
    232.6 |##########
    233.3 |#########################
    233.9 |#########
    234.6 |#######
    235.2 |##########
    235.9 |
    236.6 |###
    237.2 |
    237.9 |#
    238.5 |
    239.2 |#
  (15 below, 9 above range)

atomic_seqcst (n=160, range 840.9-862.7 ns)
    840.9 |##########
    842.0 |#############
    843.1 |############
    844.2 |#############################
    845.3 |########################################
    846.4 |################
    847.4 |###########################
    848.5 |#####################
    849.6 |#########
    850.7 |#########
    851.8 |#########
    852.9 |######
    854.0 |#
    855.1 |#
    856.2 |###
    857.3 |
    858.4 |#
    859.4 |###
    860.5 |
    861.6 |
  (13 below, 6 above range)

```
