# Static dispatch shapes: direct vs sealed-trait vs const fn pointer

3 variants, 160 samples per variant.
Baseline: **dispatch_const_ptr**

## Highlights

Baseline for all deltas below: **dispatch_const_ptr**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (dispatch_const_ptr) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline dispatch_const_ptr has the worst median (15 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest dispatch_trait at 14 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### dispatch_direct is inconsistent: worst-20% is 1.9x its best-20%

dispatch_direct's best 20% of batches run at 10 ns but its worst 20% at 19 ns (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: dispatch_trait** at 14.2 ns median (-7.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.08x (fastest 14.2 ns, slowest 15.4 ns)

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
| dispatch_const_ptr | 2278ns | 2351ns | 2091ns | 2289ns | 2430ns | base |
| dispatch_direct | 2298ns | 2408ns | 2090ns | 2323ns | 2431ns | +0.88% |
| dispatch_trait | 2244ns | 2190ns | 2088ns | 2235ns | 2425ns | -1.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| dispatch_const_ptr | 16ns | 12ns | 21ns | base | 16.125 |
| dispatch_direct | 14ns | 10ns | 19ns | -10.88% | 18.093 |
| dispatch_trait | 14ns | 10ns | 19ns | -10.45% | 18.008 |

## Performance model

- Peak throughput: **26.477 Gops/s** (dispatch_direct; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| dispatch_const_ptr | 16.623 | 62.8% |
| dispatch_direct | 17.778 | 67.1% |
| dispatch_trait | 18.028 | 68.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| dispatch_const_ptr | 2278ns | 2278ns | base |
| dispatch_direct | 2298ns | 2298ns | +0.88% |
| dispatch_trait | 2244ns | 2244ns | -1.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| dispatch_const_ptr | 15ns | base | --- | [15, 16] | --- | --- | --- | --- |
| dispatch_direct | 14ns | -1.3ns (-8.4%) | [-2, -1]ns | [13, 15] | YES | 0.0000 | 0.0000 | 9 |
| dispatch_trait | 14ns | -1.2ns (-7.8%) | [-2, -0]ns | [14, 15] | YES | 0.0010 | 0.0010 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | dispatch_const_ptr | dispatch_direct | dispatch_trait |
|---|---|---|---|---|
| 1 | 1 | 15ns | -19.9% | +9.6% |
| 1 | 2 | 16ns | +1.2% | -35.0% |
| 1 | 3 | 15ns | -17.1% | -1.4% |
| 1 | 4 | 15ns | -21.3% | -24.0% |
| 1 | 5 | 17ns | -4.2% | -10.2% |
| 1 | 6 | 16ns | -10.3% | -5.1% |
| 1 | 7 | 16ns | +3.1% | -12.3% |
| 1 | 8 | 15ns | -1.4% | -4.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| dispatch_const_ptr | -0.283 | moderate- |
| dispatch_direct | -0.266 | moderate- |
| dispatch_trait | -0.477 | moderate- |

**Consistency summary:**

- **dispatch_direct**: won 6/8, lost 2/8
- **dispatch_trait**: won 7/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| dispatch_const_ptr | 4.4ns | 15.9ns | 27.7% | HIGH |
| dispatch_direct | 4.1ns | 14.1ns | 29.3% | HIGH |
| dispatch_trait | 3.9ns | 14.2ns | 27.8% | HIGH |

## Distribution (algo ns)

```
dispatch_const_ptr (n=160, range 12.1-21.1 ns)
     12.1 |#################
     12.6 |##################################
     13.0 |##############
     13.5 |##################################
     13.9 |############################
     14.4 |########################################
     14.8 |######################
     15.3 |#################
     15.7 |######################
     16.2 |#####################################
     16.6 |######################
     17.1 |########################################
     17.5 |###########
     18.0 |###########
     18.4 |####################
     18.9 |########
     19.3 |##############
     19.8 |#####
     20.2 |###########
     20.7 |
  (12 below, 3 above range)

dispatch_direct (n=160, range 9.7-18.6 ns)
      9.7 |##########
     10.1 |#############
     10.6 |##############################
     11.0 |################
     11.5 |#######################
     11.9 |####################
     12.4 |##########################
     12.8 |################
     13.3 |#######################
     13.7 |####################
     14.2 |#################################
     14.6 |##############################
     15.1 |########################################
     15.5 |####################################
     15.9 |####################
     16.4 |#######################
     16.8 |####################
     17.3 |####################
     17.7 |##########
     18.2 |######
  (15 below, 13 above range)

dispatch_trait (n=160, range 10.0-18.6 ns)
     10.0 |#################
     10.4 |########
     10.9 |########
     11.3 |#################
     11.7 |######################
     12.2 |#########################
     12.6 |############################
     13.0 |#################
     13.5 |####################
     13.9 |####################
     14.3 |########################################
     14.7 |######################
     15.2 |#################
     15.6 |###############################
     16.0 |#################
     16.5 |#####
     16.9 |#################
     17.3 |######################
     17.8 |###########
     18.2 |
  (16 below, 14 above range)

```

## Diagnostics

- **dispatch_const_ptr**: CV=34.4% (high variance, measurements may be unstable)
- **dispatch_const_ptr**: bridge=27.3% of algo (FFI overhead may distort results)
- **dispatch_direct**: CV=23.0% (high variance, measurements may be unstable)
- **dispatch_direct**: bridge=29.2% of algo (FFI overhead may distort results)
- **dispatch_trait**: CV=21.4% (high variance, measurements may be unstable)
- **dispatch_trait**: bridge=26.1% of algo (FFI overhead may distort results)
