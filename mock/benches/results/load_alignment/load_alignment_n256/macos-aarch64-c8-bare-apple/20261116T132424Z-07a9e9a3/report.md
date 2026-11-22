# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (16 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_byte_pack at 15 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.6% (fastest 15 ns to slowest 16 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### load_aligned is inconsistent: worst-20% is 1.5x its best-20%

load_aligned's best 20% of batches run at 12 ns but its worst 20% at 19 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: load_byte_pack** at 15.4 ns median (-2.5% vs baseline)
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
| load_aligned | 2122ns | 2107ns | 2101ns | 2107ns | 2188ns | base |
| load_byte_pack | 2124ns | 2106ns | 2101ns | 2106ns | 2200ns | +0.09% |
| load_unaligned | 2108ns | 2106ns | 2099ns | 2105ns | 2125ns | -0.66% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 16ns | 12ns | 19ns | base | 16.212 |
| load_byte_pack | 15ns | 12ns | 19ns | -1.92% | 16.529 |
| load_unaligned | 16ns | 12ns | 19ns | -1.28% | 16.422 |

## Performance model

- Peak throughput: **20.661 Gops/s** (load_aligned; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 16.203 | 78.4% |
| load_byte_pack | 16.623 | 80.5% |
| load_unaligned | 16.623 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2122ns | 2122ns | base |
| load_byte_pack | 2124ns | 2124ns | +0.09% |
| load_unaligned | 2108ns | 2108ns | -0.66% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 16ns | base | --- | [15, 16] | --- | --- | --- | --- |
| load_byte_pack | 15ns | no significant difference | [-1, +0]ns | [15, 16] | no | 0.3627 | 0.3223 | 13 |
| load_unaligned | 15ns | no significant difference | [-0, +0]ns | [15, 16] | no | 0.3627 | 0.3627 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 16ns | -4.9% | -4.9% |
| 1 | 2 | 16ns | -3.8% | -6.3% |
| 1 | 3 | 16ns | -2.6% | +1.3% |
| 1 | 4 | 16ns | -6.4% | -1.3% |
| 1 | 5 | 16ns | +0.0% | -2.5% |
| 1 | 6 | 16ns | +1.3% | -1.3% |
| 1 | 7 | 15ns | +2.6% | +2.6% |
| 1 | 8 | 16ns | -5.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.054 | ok |
| load_byte_pack | 0.016 | ok |
| load_unaligned | -0.232 | moderate- |

**Consistency summary:**

- **load_byte_pack**: won 5/8, lost 2/8
- **load_unaligned**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.6ns | 15.8ns | 16.6% | HIGH |
| load_byte_pack | 2.6ns | 15.5ns | 17.1% | HIGH |
| load_unaligned | 2.7ns | 15.6ns | 17.0% | HIGH |

## Distribution (algo ns)

```
load_aligned (n=160, range 12.4-19.2 ns)
     12.4 |########
     12.7 |#################
     13.1 |########
     13.4 |
     13.7 |#########################
     14.1 |#####################################
     14.4 |##############
     14.8 |##################################
     15.1 |#####################################
     15.4 |
     15.8 |########################################
     16.1 |############################
     16.5 |########################################
     16.8 |#################
     17.1 |
     17.5 |#################
     17.8 |#################
     18.2 |####################
     18.5 |##############
     18.8 |
  (11 below, 17 above range)

load_byte_pack (n=160, range 12.4-18.6 ns)
     12.4 |############
     12.7 |##########
     13.0 |#################
     13.3 |
     13.7 |#########################
     14.0 |#########################
     14.3 |
     14.6 |########################################
     14.9 |################################
     15.2 |###########################
     15.5 |#########################
     15.8 |
     16.1 |###########################
     16.4 |##############################
     16.7 |
     17.0 |####################
     17.3 |###########################
     17.7 |############
     18.0 |
     18.3 |############
  (10 below, 12 above range)

load_unaligned (n=160, range 12.5-18.9 ns)
     12.5 |##############
     12.8 |#########
     13.1 |##################
     13.4 |
     13.8 |#######################
     14.1 |#####################
     14.4 |############################
     14.7 |#########################
     15.0 |
     15.4 |#######################
     15.7 |########################################
     16.0 |#######################
     16.3 |
     16.6 |#####################
     17.0 |###########
     17.3 |#########
     17.6 |#######################
     17.9 |
     18.2 |##############
     18.6 |##################
  (11 below, 10 above range)

```

## Diagnostics

- **load_aligned**: bridge=15.8% of algo (FFI overhead may distort results)
- **load_byte_pack**: bridge=16.2% of algo (FFI overhead may distort results)
- **load_unaligned**: bridge=16.2% of algo (FFI overhead may distort results)
