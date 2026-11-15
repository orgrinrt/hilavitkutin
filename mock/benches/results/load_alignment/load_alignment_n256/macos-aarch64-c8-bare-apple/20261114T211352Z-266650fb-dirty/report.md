# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.6% (fastest 15 ns to slowest 16 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_aligned is fastest but the noisiest (CV 23.7%)

load_aligned wins on median (15 ns) yet has the highest variance (CV 23.7%), while load_byte_pack is the steadiest (CV 18.6%, 16 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (load_aligned)

The baseline load_aligned is the fastest (15 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader load_aligned vs stability leader load_byte_pack (+3% speed for 1.3x steadier)

load_aligned is fastest (15 ns, CV 23.7%); load_byte_pack gives up 2.6% median for 1.3x lower variance (CV 18.6%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### load_aligned is inconsistent: worst-20% is 2.1x its best-20%

load_aligned's best 20% of batches run at 10 ns but its worst 20% at 20 ns (2.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (load_aligned) is the fastest** at 15.4 ns median
- Spread: 1.03x (fastest 15.4 ns, slowest 15.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2232ns | 2180ns | 2092ns | 2215ns | 2423ns | base |
| load_byte_pack | 2280ns | 2252ns | 2099ns | 2293ns | 2424ns | +2.19% |
| load_unaligned | 2239ns | 2181ns | 2093ns | 2226ns | 2427ns | +0.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 15ns | 10ns | 20ns | base | 16.884 |
| load_byte_pack | 16ns | 12ns | 20ns | +3.97% | 16.239 |
| load_unaligned | 15ns | 10ns | 20ns | +0.22% | 16.847 |

## Performance model

- Peak throughput: **26.131 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 16.623 | 63.6% |
| load_byte_pack | 16.203 | 62.0% |
| load_unaligned | 16.623 | 63.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2232ns | 2232ns | base |
| load_byte_pack | 2280ns | 2280ns | +2.19% |
| load_unaligned | 2239ns | 2239ns | +0.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 15ns | base | --- | [15, 16] | --- | --- | --- | --- |
| load_byte_pack | 16ns | no significant difference | [-0, +1]ns | [15, 17] | no | 0.6255 | 0.6255 | 9 |
| load_unaligned | 15ns | no significant difference | [-0, +1]ns | [15, 16] | no | 0.5896 | 0.2948 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 10ns | +48.1% | +18.3% |
| 1 | 2 | 17ns | -0.3% | +2.4% |
| 1 | 3 | 16ns | -4.9% | -1.2% |
| 1 | 4 | 16ns | +9.5% | -3.8% |
| 1 | 5 | 15ns | +11.1% | +12.5% |
| 1 | 6 | 18ns | -9.7% | -3.4% |
| 1 | 7 | 11ns | +52.7% | +35.7% |
| 1 | 8 | 17ns | -29.2% | -36.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.446 | moderate- |
| load_byte_pack | -0.263 | moderate- |
| load_unaligned | -0.014 | ok |

**Consistency summary:**

- **load_byte_pack**: won 4/8, lost 4/8
- **load_unaligned**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.7ns | 15.2ns | 17.7% | HIGH |
| load_byte_pack | 2.8ns | 15.8ns | 17.8% | HIGH |
| load_unaligned | 2.8ns | 15.2ns | 18.6% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 9.8-20.2 ns)
      9.8 |##############
     10.3 |###########
     10.8 |######################
     11.4 |##############
     11.9 |#####
     12.4 |#########################
     12.9 |###########
     13.4 |####################
     13.9 |####################
     14.5 |#########################
     15.0 |##################################
     15.5 |#################
     16.0 |##################################
     16.5 |####################
     17.0 |########################################
     17.6 |#################
     18.1 |##############
     18.6 |##
     19.1 |############################
     19.6 |###########
  (11 below, 12 above range)

load_byte_pack (n=160, range 11.5-19.8 ns)
     11.5 |############
     11.9 |#################
     12.3 |#######
     12.7 |#######
     13.2 |####################
     13.6 |#################
     14.0 |##########
     14.4 |######################
     14.8 |#########################
     15.2 |######################
     15.6 |####################
     16.1 |##########
     16.5 |######################
     16.9 |###########################
     17.3 |########################################
     17.7 |##########
     18.1 |##
     18.5 |#################
     19.0 |#################
     19.4 |#######
  (11 below, 14 above range)

load_unaligned (n=160, range 10.4-19.6 ns)
     10.4 |##########
     10.9 |########
     11.3 |#####
     11.8 |########
     12.3 |######
     12.7 |#####
     13.2 |###
     13.6 |############
     14.1 |######
     14.6 |########################################
     15.0 |#############
     15.5 |###################
     15.9 |#############
     16.4 |###############
     16.9 |#############
     17.3 |############
     17.8 |#
     18.3 |##########
     18.7 |#############
     19.2 |######
  (15 below, 13 above range)

```

## Diagnostics

- **load_aligned**: CV=24.1% (high variance, measurements may be unstable)
- **load_aligned**: bridge=16.2% of algo (FFI overhead may distort results)
- **load_byte_pack**: bridge=15.8% of algo (FFI overhead may distort results)
- **load_unaligned**: CV=21.2% (high variance, measurements may be unstable)
- **load_unaligned**: bridge=16.2% of algo (FFI overhead may distort results)
