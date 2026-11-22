# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (110 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_byte_pack at 109 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.1% (fastest 109 ns to slowest 110 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_unaligned shows alternating (throttle bounce) (autocorr -0.64)

load_unaligned's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: load_byte_pack** at 109.2 ns median (-1.1% vs baseline)
- Spread: 1.01x (fastest 109.2 ns, slowest 110.4 ns)

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
| load_aligned | 2308ns | 2247ns | 2233ns | 2252ns | 2552ns | base |
| load_byte_pack | 2260ns | 2243ns | 2232ns | 2245ns | 2335ns | -2.07% |
| load_unaligned | 2254ns | 2238ns | 2230ns | 2242ns | 2317ns | -2.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 113ns | 106ns | 131ns | base | 9.033 |
| load_byte_pack | 109ns | 105ns | 114ns | -3.41% | 9.352 |
| load_unaligned | 111ns | 105ns | 120ns | -2.44% | 9.259 |

## Performance model

- Peak throughput: **9.735 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 9.275 | 95.3% |
| load_byte_pack | 9.377 | 96.3% |
| load_unaligned | 9.377 | 96.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2308ns | 2308ns | base |
| load_byte_pack | 2260ns | 2260ns | -2.07% |
| load_unaligned | 2254ns | 2254ns | -2.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 110ns | base | --- | [109, 111] | --- | --- | --- | --- |
| load_byte_pack | 109ns | no significant difference | [-1, +0]ns | [108, 110] | no | 0.0523 | 0.0261 | 12 |
| load_unaligned | 109ns | no significant difference | [-1, +0]ns | [108, 110] | no | 0.2576 | 0.2576 | 7 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 110ns | -1.6% | +0.1% |
| 1 | 2 | 110ns | -1.4% | -2.3% |
| 1 | 3 | 108ns | +0.4% | +1.6% |
| 1 | 4 | 111ns | -1.5% | -2.4% |
| 1 | 5 | 114ns | -4.4% | -4.0% |
| 1 | 6 | 110ns | +2.3% | -0.2% |
| 1 | 7 | 109ns | -0.2% | -0.8% |
| 1 | 8 | 108ns | -0.4% | +1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.157 | ok |
| load_byte_pack | 0.035 | ok |
| load_unaligned | -0.642 | HIGH- (thermal bounce) |

**Consistency summary:**

- **load_byte_pack**: won 6/8, lost 2/8
- **load_unaligned**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.7ns | 113.4ns | 2.4% |  |
| load_byte_pack | 2.9ns | 109.5ns | 2.7% |  |
| load_unaligned | 2.8ns | 110.6ns | 2.5% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 105.6-130.8 ns)
    105.6 |########################
    106.8 |############################
    108.1 |########################################
    109.3 |########################
    110.6 |#################################
    111.9 |############################
    113.1 |####################
    114.4 |#################
    115.7 |####
    116.9 |####
    118.2 |#
    119.4 |
    120.7 |
    122.0 |
    123.2 |
    124.5 |
    125.8 |
    127.0 |
    128.3 |
    129.5 |
  (14 below, 3 above range)

load_byte_pack (n=160, range 105.2-114.3 ns)
    105.2 |##########
    105.6 |###
    106.1 |########
    106.6 |########
    107.0 |#############
    107.5 |########################################
    107.9 |#############
    108.4 |#############
    108.8 |###################
    109.3 |##########
    109.8 |############
    110.2 |##########
    110.7 |##########
    111.1 |######
    111.6 |###
    112.0 |########
    112.5 |####################
    113.0 |#####
    113.4 |########
    113.9 |######
  (11 below, 13 above range)

load_unaligned (n=160, range 105.2-120.1 ns)
    105.2 |########################
    106.0 |###############################
    106.7 |######################
    107.5 |#####################################
    108.2 |########################################
    109.0 |########################
    109.7 |#####################################
    110.5 |###############
    111.2 |######################
    111.9 |####################
    112.7 |###########
    113.4 |######
    114.2 |###########
    114.9 |###############
    115.7 |####
    116.4 |##
    117.1 |
    117.9 |##
    118.6 |
    119.4 |##
  (9 below, 2 above range)

```

## Diagnostics

- **load_aligned**: CV=24.2% (high variance, measurements may be unstable)
