# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### atomic_relaxed dominates: 373% faster than the next best (atomic_acquire)

atomic_relaxed (34 ns) leads atomic_acquire (162 ns) by 373%, a clear separation rather than a photo finish. CV 9.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 79% (significant)

atomic_relaxed is -128 ns (79%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_seqcst is an outlier: 4.9x slower than the field

atomic_seqcst (167 ns) is 4.9x the fastest (34 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_relaxed is fastest but the noisiest (CV 9.5%)

atomic_relaxed wins on median (34 ns) yet has the highest variance (CV 9.5%), while atomic_seqcst is the steadiest (CV 2.3%, 167 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 4.9x the fastest

Fastest atomic_relaxed (34 ns) to slowest atomic_seqcst (167 ns): 4.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: atomic_relaxed** at 34.2 ns median (-78.8% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.87x (fastest 34.2 ns, slowest 166.7 ns)

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
| atomic_acquire | 2285ns | 2279ns | 2273ns | 2280ns | 2315ns | base |
| atomic_relaxed | 2139ns | 2126ns | 2118ns | 2129ns | 2189ns | -6.40% |
| atomic_seqcst | 2296ns | 2280ns | 2274ns | 2281ns | 2364ns | +0.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 162ns | 158ns | 167ns | base | 1.579 |
| atomic_relaxed | 34ns | 30ns | 39ns | -78.88% | 7.477 |
| atomic_seqcst | 167ns | 162ns | 172ns | +2.85% | 1.535 |

## Performance model

- Peak throughput: **8.600 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.583 | 18.4% |
| atomic_relaxed | 7.485 | 87.0% |
| atomic_seqcst | 1.536 | 17.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 2285ns | 2285ns | base |
| atomic_relaxed | 2139ns | 2139ns | -6.40% |
| atomic_seqcst | 2296ns | 2296ns | +0.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 162ns | base | --- | [161, 162] | --- | --- | --- | --- |
| atomic_relaxed | 34ns | -127.5ns (-78.8%) | [-128, -127]ns | [33, 35] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 167ns | +4.6ns (+2.8%) | [+4, +5]ns | [166, 167] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 161ns | -78.9% | +2.3% |
| 1 | 2 | 162ns | -78.2% | +2.9% |
| 1 | 3 | 162ns | -78.8% | +2.4% |
| 1 | 4 | 161ns | -78.9% | +3.4% |
| 1 | 5 | 163ns | -78.2% | +2.2% |
| 1 | 6 | 163ns | -79.8% | +5.6% |
| 1 | 7 | 161ns | -78.8% | +3.0% |
| 1 | 8 | 161ns | -78.6% | +2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.104 | ok |
| atomic_relaxed | -0.490 | moderate- |
| atomic_seqcst | -0.162 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.1ns | 162.1ns | 1.9% |  |
| atomic_relaxed | 4.7ns | 34.2ns | 13.7% | HIGH |
| atomic_seqcst | 2.9ns | 166.8ns | 1.7% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 157.6-167.5 ns)
    157.6 |#######
    158.1 |###############
    158.6 |##########
    159.1 |##########
    159.6 |###################################
    160.1 |##########
    160.6 |#########################
    161.0 |####################
    161.5 |###################################
    162.0 |########################################
    162.5 |######################
    163.0 |####################
    163.5 |############
    164.0 |##########
    164.5 |###############
    165.0 |########################################
    165.5 |#####
    166.0 |#####
    166.5 |############
    167.0 |
  (15 below, 5 above range)

atomic_relaxed (n=160, range 29.8-38.8 ns)
     29.8 |#################
     30.2 |#################
     30.7 |######################
     31.1 |###########
     31.6 |#################
     32.0 |#####
     32.5 |########################################
     32.9 |#####################################
     33.4 |##############
     33.9 |######################
     34.3 |##############
     34.8 |######################
     35.2 |##############
     35.7 |###############################
     36.1 |#########################
     36.6 |#########################
     37.0 |###########
     37.5 |####################
     37.9 |########
     38.4 |##
  (12 below, 14 above range)

atomic_seqcst (n=160, range 162.0-172.3 ns)
    162.0 |#####
    162.5 |#####################
    163.0 |########
    163.5 |##################
    164.0 |##################
    164.6 |##########################
    165.1 |#############
    165.6 |##################################
    166.1 |##################
    166.6 |########################################
    167.1 |########################
    167.7 |##########################
    168.2 |##################
    168.7 |##########################
    169.2 |##########################
    169.7 |#####
    170.2 |#####
    170.7 |##########
    171.3 |##
    171.8 |##
  (16 below, 11 above range)

```

## Diagnostics

- **atomic_relaxed**: bridge=13.5% of algo (FFI overhead may distort results)
