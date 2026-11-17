# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (111 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_trait at 110 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (4 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader dispatch_trait vs stability leader dispatch_direct (+1% speed for 1.0x steadier)

dispatch_trait is fastest (110 ns, CV 3.3%); dispatch_direct gives up 0.7% median for 1.0x lower variance (CV 3.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 1.1% of the fastest

All 3 variants sit between 110 ns and 111 ns - a 1.1% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### dispatch_trait's edge over baseline is significant but tiny (-1 ns, 1.08%)

dispatch_trait differs from baseline dispatch_const_ptr by -1 ns (1.08%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: dispatch_trait** at 110.0 ns median (-1.1% vs baseline)
- 1 variant significantly faster than baseline
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
| dispatch_const_ptr | 2249ns | 2244ns | 2202ns | 2237ns | 2330ns | base |
| dispatch_direct | 2244ns | 2247ns | 2206ns | 2244ns | 2281ns | -0.21% |
| dispatch_trait | 2225ns | 2212ns | 2204ns | 2221ns | 2258ns | -1.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 112ns | 107ns | 118ns | base | 9.170 |
| dispatch_direct | 111ns | 106ns | 116ns | -0.43% | 9.210 |
| dispatch_trait | 110ns | 106ns | 116ns | -1.24% | 9.286 |

## Performance model

- Peak throughput: **9.701 Gops/s** (dispatch_trait; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 9.209 | 94.9% |
| dispatch_direct | 9.242 | 95.3% |
| dispatch_trait | 9.309 | 96.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2249ns | 2249ns | base |
| dispatch_direct | 2244ns | 2244ns | -0.21% |
| dispatch_trait | 2225ns | 2225ns | -1.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 111ns | base | --- | [110, 112] | --- | --- | --- | --- |
| dispatch_direct | 111ns | no significant difference | [-1, +0]ns | [110, 111] | no | 1.0000 | 1.0000 | 8 |
| dispatch_trait | 110ns | -1.2ns (-1.1%) | [-2, -0]ns | [109, 111] | YES | 0.0161 | 0.0080 | 4 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 110ns | +0.2% | +0.0% |
| 1 | 2 | 111ns | -0.8% | -1.7% |
| 1 | 3 | 111ns | +0.6% | +0.6% |
| 1 | 4 | 109ns | +2.8% | +0.7% |
| 1 | 5 | 112ns | -2.4% | -1.2% |
| 1 | 6 | 110ns | +1.1% | -0.5% |
| 1 | 7 | 112ns | +1.2% | -2.3% |
| 1 | 8 | 116ns | -5.0% | -6.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | 0.060 | ok |
| dispatch_direct | -0.325 | moderate- |
| dispatch_trait | 0.045 | ok |

**Consistency summary:**

- **dispatch_direct**: won 3/8, lost 5/8
- **dispatch_trait**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 2.8ns | 111.7ns | 2.5% |  |
| dispatch_direct | 3.0ns | 111.2ns | 2.7% |  |
| dispatch_trait | 2.8ns | 110.3ns | 2.5% |  |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 106.6-117.8 ns)
    106.6 |##################
    107.2 |#########
    107.7 |##################
    108.3 |#######################
    108.9 |#######################
    109.4 |#######################
    110.0 |##################
    110.5 |#########
    111.1 |#########################
    111.7 |########################################
    112.2 |#########
    112.8 |################
    113.3 |##############
    113.9 |###########
    114.5 |###########
    115.0 |#########
    115.6 |##############
    116.2 |###########
    116.7 |#########
    117.3 |#######
  (11 below, 10 above range)

dispatch_direct (n=160, range 106.4-116.5 ns)
    106.4 |##################
    106.9 |#############
    107.4 |########################
    107.9 |#############
    108.4 |################
    108.9 |#####################
    109.4 |##################
    109.9 |########################################
    110.4 |##########################
    110.9 |#####################
    111.4 |#############
    111.9 |##################
    112.5 |##################
    113.0 |##########
    113.5 |#####
    114.0 |################
    114.5 |##################
    115.0 |#####################
    115.5 |########
    116.0 |##########
  (13 below, 14 above range)

dispatch_trait (n=160, range 105.6-115.7 ns)
    105.6 |#####
    106.1 |##################
    106.6 |########
    107.1 |########################################
    107.6 |########################
    108.1 |#############
    108.6 |##########################
    109.1 |#############################
    109.6 |#############
    110.1 |#############################
    110.6 |#####################
    111.1 |##################
    111.7 |########################
    112.2 |##########
    112.7 |#####
    113.2 |########
    113.7 |#############
    114.2 |##########################
    114.7 |#####
    115.2 |##########
  (14 below, 14 above range)

```
