# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 437% faster than the next best (atomic_acquire)

atomic_relaxed (31 ns) leads atomic_acquire (168 ns) by 437%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 81% (significant)

atomic_relaxed is -136 ns (81%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 5.5x slower than the field

atomic_seqcst (170 ns) is 5.5x the fastest (31 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_relaxed is fastest but the noisiest (CV 7.9%)

atomic_relaxed wins on median (31 ns) yet has the highest variance (CV 7.9%), while atomic_seqcst is the steadiest (CV 2.6%, 170 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### atomic_relaxed shows alternating (throttle bounce) (autocorr -0.52)

atomic_relaxed's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.5x the fastest

Fastest atomic_relaxed (31 ns) to slowest atomic_seqcst (170 ns): 5.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### atomic_seqcst's edge over baseline is significant but tiny (3 ns, 1.97%)

atomic_seqcst differs from baseline atomic_acquire by 3 ns (1.97%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: atomic_relaxed** at 31.2 ns median (-81.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 5.46x (fastest 31.2 ns, slowest 170.4 ns)

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
| atomic_acquire | 2339ns | 2315ns | 2304ns | 2324ns | 2423ns | base |
| atomic_relaxed | 2147ns | 2148ns | 2107ns | 2147ns | 2186ns | -8.24% |
| atomic_seqcst | 2320ns | 2311ns | 2289ns | 2312ns | 2374ns | -0.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 168ns | 163ns | 176ns | base | 1.522 |
| atomic_relaxed | 31ns | 28ns | 34ns | -81.50% | 8.227 |
| atomic_seqcst | 171ns | 166ns | 177ns | +1.58% | 1.498 |

## Performance model

- Peak throughput: **9.266 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.528 | 16.5% |
| atomic_relaxed | 8.205 | 88.6% |
| atomic_seqcst | 1.502 | 16.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2339ns | 2339ns | base |
| atomic_relaxed | 2147ns | 2147ns | -8.24% |
| atomic_seqcst | 2320ns | 2320ns | -0.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 168ns | base | --- | [167, 168] | --- | --- | --- | --- |
| atomic_relaxed | 31ns | -136.2ns (-81.3%) | [-137, -135]ns | [31, 32] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 170ns | +3.3ns (+2.0%) | [+2, +4]ns | [170, 171] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 167ns | -81.4% | +1.6% |
| 1 | 2 | 171ns | -82.0% | -0.4% |
| 1 | 3 | 167ns | -81.0% | +0.8% |
| 1 | 4 | 166ns | -81.4% | +2.1% |
| 1 | 5 | 167ns | -80.7% | +2.2% |
| 1 | 6 | 171ns | -82.2% | +2.6% |
| 1 | 7 | 166ns | -81.3% | +2.0% |
| 1 | 8 | 166ns | -81.3% | +3.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.267 | moderate- |
| atomic_relaxed | -0.518 | HIGH- (thermal bounce) |
| atomic_seqcst | -0.174 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 1/8, lost 7/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 2.8ns | 168.2ns | 1.6% |  |
| atomic_relaxed | 6.4ns | 31.1ns | 20.6% | HIGH |
| atomic_seqcst | 2.5ns | 170.8ns | 1.5% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 162.7-175.7 ns)
    162.7 |##########
    163.3 |#################
    164.0 |###############
    164.6 |##########
    165.3 |########################################
    165.9 |##############################
    166.6 |###################################
    167.2 |######################
    167.9 |##############################
    168.5 |####################
    169.2 |#####################################
    169.8 |##############################
    170.5 |#######
    171.1 |#######
    171.8 |##
    172.4 |##########
    173.1 |##
    173.7 |
    174.4 |##
    175.0 |##
  (14 below, 13 above range)

atomic_relaxed (n=160, range 27.6-34.4 ns)
     27.6 |########
     28.0 |################
     28.3 |
     28.6 |##################
     29.0 |#############
     29.3 |##########################
     29.7 |
     30.0 |#############################
     30.3 |#####################################
     30.7 |##########################
     31.0 |#############################
     31.3 |
     31.7 |########################
     32.0 |########################
     32.3 |########################################
     32.7 |########################
     33.0 |#####################
     33.4 |
     33.7 |#####################
     34.0 |#############
  (11 below, 9 above range)

atomic_seqcst (n=160, range 165.7-177.2 ns)
    165.7 |###############
    166.3 |####################
    166.9 |####################
    167.4 |####################
    168.0 |####################
    168.6 |###############
    169.2 |##############################
    169.7 |######################
    170.3 |###################################
    170.9 |#######
    171.5 |#################
    172.0 |########################################
    172.6 |###############
    173.2 |############
    173.8 |###############
    174.3 |#################
    174.9 |############
    175.5 |##########
    176.1 |
    176.6 |##
  (12 below, 9 above range)

```

## Diagnostics

- **atomic_relaxed**: bridge=19.9% of algo (FFI overhead may distort results)
