# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 361% faster than the next best (atomic_acquire)

atomic_relaxed (36 ns) leads atomic_acquire (165 ns) by 361%, a clear separation rather than a photo finish. CV 8.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 78% (significant)

atomic_relaxed is -129 ns (78%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 4.7x slower than the field

atomic_seqcst (169 ns) is 4.7x the fastest (36 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_relaxed is fastest but the noisiest (CV 8.5%)

atomic_relaxed wins on median (36 ns) yet has the highest variance (CV 8.5%), while atomic_acquire is the steadiest (CV 1.9%, 165 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### atomic_seqcst shows alternating (throttle bounce) (autocorr -0.58)

atomic_seqcst's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.7x the fastest

Fastest atomic_relaxed (36 ns) to slowest atomic_seqcst (169 ns): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: atomic_relaxed** at 35.8 ns median (-78.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.73x (fastest 35.8 ns, slowest 169.2 ns)

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
| atomic_acquire | 2323ns | 2320ns | 2314ns | 2320ns | 2338ns | base |
| atomic_relaxed | 2168ns | 2164ns | 2156ns | 2164ns | 2189ns | -6.68% |
| atomic_seqcst | 2326ns | 2322ns | 2316ns | 2322ns | 2348ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 165ns | 160ns | 168ns | base | 1.555 |
| atomic_relaxed | 35ns | 31ns | 39ns | -78.51% | 7.237 |
| atomic_seqcst | 169ns | 163ns | 174ns | +2.56% | 1.516 |

## Performance model

- Peak throughput: **8.267 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.552 | 18.8% |
| atomic_relaxed | 7.151 | 86.5% |
| atomic_seqcst | 1.513 | 18.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2323ns | 2323ns | base |
| atomic_relaxed | 2168ns | 2168ns | -6.68% |
| atomic_seqcst | 2326ns | 2326ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 165ns | base | --- | [165, 166] | --- | --- | --- | --- |
| atomic_relaxed | 36ns | -129.2ns (-78.3%) | [-130, -129]ns | [35, 36] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 169ns | +4.1ns (+2.5%) | [+3, +5]ns | [168, 170] | YES | 0.0000 | 0.0000 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 166ns | -78.7% | +1.8% |
| 1 | 2 | 164ns | -78.2% | +2.9% |
| 1 | 3 | 165ns | -79.1% | +2.1% |
| 1 | 4 | 165ns | -78.2% | +2.8% |
| 1 | 5 | 165ns | -78.8% | +2.5% |
| 1 | 6 | 166ns | -78.9% | +1.9% |
| 1 | 7 | 164ns | -77.8% | +3.3% |
| 1 | 8 | 165ns | -78.3% | +2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.508 | HIGH- (thermal bounce) |
| atomic_relaxed | -0.354 | moderate- |
| atomic_seqcst | -0.579 | HIGH- (thermal bounce) |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.1ns | 164.6ns | 1.9% |  |
| atomic_relaxed | 4.6ns | 35.4ns | 13.1% | HIGH |
| atomic_seqcst | 3.0ns | 168.8ns | 1.8% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 159.8-168.4 ns)
    159.8 |####
    160.2 |###########
    160.6 |####
    161.1 |#########
    161.5 |##############
    161.9 |#######
    162.4 |#######
    162.8 |#########
    163.2 |################
    163.7 |##############
    164.1 |##################
    164.5 |#########################
    165.0 |########################################
    165.4 |#########################
    165.8 |##############
    166.3 |#######################
    166.7 |##############################
    167.1 |################
    167.6 |##############
    168.0 |###########
  (14 below, 10 above range)

atomic_relaxed (n=160, range 31.0-39.5 ns)
     31.0 |##
     31.4 |#############
     31.8 |################
     32.2 |##################
     32.7 |##################
     33.1 |#############
     33.5 |##########
     33.9 |##########
     34.4 |########################
     34.8 |#####################
     35.2 |################
     35.6 |########################################
     36.1 |#############
     36.5 |#############################
     36.9 |################
     37.3 |#############
     37.8 |################
     38.2 |##################
     38.6 |##################
     39.0 |##################
  (16 below, 13 above range)

atomic_seqcst (n=160, range 163.4-174.0 ns)
    163.4 |###########
    164.0 |########
    164.5 |###########
    165.0 |####
    165.5 |########
    166.1 |########
    166.6 |###############
    167.1 |#############
    167.6 |#################
    168.2 |########################################
    168.7 |######################
    169.2 |###############################
    169.8 |#################
    170.3 |###################################
    170.8 |#################
    171.3 |#############
    171.9 |##
    172.4 |########################
    172.9 |####
    173.4 |####
  (14 below, 5 above range)

```

## Diagnostics

- **atomic_relaxed**: bridge=12.8% of algo (FFI overhead may distort results)
