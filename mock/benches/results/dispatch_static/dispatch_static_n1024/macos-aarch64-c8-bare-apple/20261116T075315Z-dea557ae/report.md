# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (111 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_direct at 110 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.1% (fastest 110 ns to slowest 111 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (6 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader dispatch_direct vs stability leader dispatch_trait (+1% speed for 1.3x steadier)

dispatch_direct is fastest (110 ns, CV 5.4%); dispatch_trait gives up 0.7% median for 1.3x lower variance (CV 4.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: dispatch_direct** at 110.0 ns median (-1.1% vs baseline)
- Spread: 1.01x (fastest 110.0 ns, slowest 111.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| dispatch_const_ptr | 2258ns | 2201ns | 2188ns | 2214ns | 2461ns | base |
| dispatch_direct | 2223ns | 2198ns | 2187ns | 2201ns | 2325ns | -1.55% |
| dispatch_trait | 2228ns | 2199ns | 2187ns | 2205ns | 2336ns | -1.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 113ns | 106ns | 126ns | base | 9.065 |
| dispatch_direct | 111ns | 105ns | 120ns | -1.46% | 9.199 |
| dispatch_trait | 111ns | 105ns | 118ns | -1.62% | 9.214 |

## Performance model

- Peak throughput: **9.713 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 9.209 | 94.8% |
| dispatch_direct | 9.309 | 95.8% |
| dispatch_trait | 9.242 | 95.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2258ns | 2258ns | base |
| dispatch_direct | 2223ns | 2223ns | -1.55% |
| dispatch_trait | 2228ns | 2228ns | -1.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 111ns | base | --- | [110, 112] | --- | --- | --- | --- |
| dispatch_direct | 110ns | no significant difference | [-1, +0]ns | [110, 111] | no | 0.2907 | 0.1454 | 7 |
| dispatch_trait | 111ns | no significant difference | [-1, +0]ns | [110, 112] | no | 0.4625 | 0.4625 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 109ns | +0.2% | +0.2% |
| 1 | 2 | 109ns | +1.3% | +7.5% |
| 1 | 3 | 111ns | -1.5% | -0.9% |
| 1 | 4 | 117ns | +0.9% | -7.1% |
| 1 | 5 | 112ns | -2.8% | -1.0% |
| 1 | 6 | 110ns | +0.0% | +0.4% |
| 1 | 7 | 110ns | -1.5% | +0.2% |
| 1 | 8 | 117ns | -7.0% | -5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.047 | ok |
| dispatch_direct | -0.179 | ok |
| dispatch_trait | -0.256 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 4/8, lost 3/8
- **dispatch_trait**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 3.1ns | 113.0ns | 2.7% |  |
| dispatch_direct | 2.7ns | 111.3ns | 2.4% |  |
| dispatch_trait | 2.8ns | 111.1ns | 2.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 105.8-125.5 ns)
    105.8 |##################
    106.8 |#######################
    107.8 |###################################
    108.8 |##############
    109.7 |#############################
    110.7 |########################################
    111.7 |############
    112.7 |################
    113.7 |###################################
    114.7 |####
    115.7 |##############
    116.6 |##############
    117.6 |######
    118.6 |############
    119.6 |####
    120.6 |####
    121.6 |####
    122.6 |##
    123.5 |##
    124.5 |
  (16 below, 3 above range)

dispatch_direct (n=160, range 105.4-119.7 ns)
    105.4 |######
    106.2 |###########
    106.9 |##########################
    107.6 |############################
    108.3 |########################
    109.0 |##########################
    109.7 |########################################
    110.4 |###############
    111.2 |########
    111.9 |#################
    112.6 |###########
    113.3 |#################
    114.0 |#############
    114.7 |########
    115.4 |######
    116.1 |#############
    116.9 |####
    117.6 |######
    118.3 |########
    119.0 |##
  (16 below, 9 above range)

dispatch_trait (n=160, range 105.4-117.8 ns)
    105.4 |###############
    106.0 |##########
    106.7 |####################
    107.3 |############
    107.9 |######################
    108.5 |####################
    109.1 |#####################################
    109.7 |#####
    110.4 |#####################################
    111.0 |############
    111.6 |########################################
    112.2 |#######
    112.8 |######################
    113.4 |##
    114.1 |############
    114.7 |####################
    115.3 |###############
    115.9 |#######
    116.5 |#################
    117.1 |#######
  (13 below, 9 above range)

```
