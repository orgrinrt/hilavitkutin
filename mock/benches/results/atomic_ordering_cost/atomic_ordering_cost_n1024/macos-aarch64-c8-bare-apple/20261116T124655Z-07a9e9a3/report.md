# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 261% faster than the next best (atomic_acquire)

atomic_relaxed (233 ns) leads atomic_acquire (842 ns) by 261%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 72% (significant)

atomic_relaxed is -608 ns (72%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 3.6x slower than the field

atomic_seqcst (846 ns) is 3.6x the fastest (233 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 3.6x the fastest

Fastest atomic_relaxed (233 ns) to slowest atomic_seqcst (846 ns): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (4 ns, 0.49%)

atomic_seqcst differs from baseline atomic_acquire by 4 ns (0.49%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 233.3 ns median (-72.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.63x (fastest 233.3 ns, slowest 845.8 ns)

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
| atomic_acquire | 3012ns | 2994ns | 2988ns | 2994ns | 3092ns | base |
| atomic_relaxed | 2367ns | 2363ns | 2356ns | 2364ns | 2387ns | -21.43% |
| atomic_seqcst | 2999ns | 2997ns | 2990ns | 2997ns | 3013ns | -0.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 845ns | 837ns | 861ns | base | 1.212 |
| atomic_relaxed | 236ns | 229ns | 249ns | -72.05% | 4.338 |
| atomic_seqcst | 847ns | 841ns | 856ns | +0.25% | 1.209 |

## Performance model

- Peak throughput: **4.463 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.216 | 27.2% |
| atomic_relaxed | 4.389 | 98.3% |
| atomic_seqcst | 1.211 | 27.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 3012ns | 3012ns | base |
| atomic_relaxed | 2367ns | 2367ns | -21.43% |
| atomic_seqcst | 2999ns | 2999ns | -0.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 842ns | base | --- | [841, 842] | --- | --- | --- | --- |
| atomic_relaxed | 233ns | -607.9ns (-72.2%) | [-609, -607]ns | [233, 234] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 846ns | +4.1ns (+0.5%) | [+4, +4]ns | [845, 846] | YES | 0.0000 | 0.0000 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 840ns | -72.3% | +0.6% |
| 1 | 2 | 842ns | -72.5% | +0.3% |
| 1 | 3 | 842ns | -71.4% | +0.4% |
| 1 | 4 | 842ns | -72.3% | +0.4% |
| 1 | 5 | 841ns | -72.3% | +0.6% |
| 1 | 6 | 844ns | -72.3% | +0.5% |
| 1 | 7 | 840ns | -72.3% | +0.8% |
| 1 | 8 | 841ns | -72.4% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.468 | moderate- |
| atomic_relaxed | -0.239 | moderate- |
| atomic_seqcst | 0.329 | moderate+ |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 2.7ns | 844.6ns | 0.3% |  |
| atomic_relaxed | 2.7ns | 236.1ns | 1.1% |  |
| atomic_seqcst | 2.8ns | 846.8ns | 0.3% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 836.9-860.8 ns)
    836.9 |###########################
    838.1 |####################
    839.3 |############################
    840.5 |###################
    841.7 |######################################
    842.9 |########################################
    844.1 |########################
    845.3 |###################
    846.5 |########
    847.7 |###
    848.8 |
    850.0 |#
    851.2 |
    852.4 |
    853.6 |#
    854.8 |
    856.0 |
    857.2 |
    858.4 |
    859.6 |
  (10 below, 5 above range)

atomic_relaxed (n=160, range 229.4-249.3 ns)
    229.4 |##########################
    230.4 |################
    231.4 |##########################
    232.4 |########################################
    233.4 |#####################
    234.4 |########################
    235.4 |#########
    236.4 |####
    237.4 |######
    238.4 |###
    239.4 |###
    240.4 |############
    241.4 |####
    242.4 |#
    243.4 |
    244.4 |###
    245.4 |#######
    246.4 |
    247.4 |
    248.4 |
  (11 below, 12 above range)

atomic_seqcst (n=160, range 840.7-855.8 ns)
    840.7 |########
    841.4 |#############
    842.2 |######################
    843.0 |#############
    843.7 |###################################
    844.5 |##########################
    845.2 |###################################
    846.0 |########################################
    846.7 |############################
    847.5 |########
    848.3 |############################
    849.0 |#################
    849.8 |###############
    850.5 |####
    851.3 |##
    852.1 |####
    852.8 |##
    853.6 |
    854.3 |##
    855.1 |
  (14 below, 6 above range)

```
