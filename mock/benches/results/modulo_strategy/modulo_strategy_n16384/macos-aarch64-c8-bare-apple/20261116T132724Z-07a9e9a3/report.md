# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 121% faster than the next best (mod_npow2_const)

mod_pow2_const (25.53 us) leads mod_npow2_const (56.41 us) by 121%, a clear separation rather than a photo finish. CV 0.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -30.85 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (71.60 us) is 2.8x the fastest (25.53 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_npow2_const shows alternating (throttle bounce) (autocorr -0.50)

mod_npow2_const's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: mod_pow2_const** at 25532.3 ns median (-54.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.80x (fastest 25532.3 ns, slowest 71596.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4dd582c1e7fa3b96 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mod_npow2_const | 58739ns | 58574ns | 58478ns | 58584ns | 59465ns | base |
| mod_pow2_const | 27680ns | 27659ns | 27626ns | 27664ns | 27781ns | -52.88% |
| mod_var | 73853ns | 73806ns | 73714ns | 73811ns | 74121ns | +25.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 56561ns | 56332ns | 57236ns | base | 0.290 |
| mod_pow2_const | 25564ns | 25525ns | 25657ns | -54.80% | 0.641 |
| mod_var | 71637ns | 71520ns | 71871ns | +26.66% | 0.229 |

## Performance model

- Peak throughput: **0.642 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.290 | 45.3% |
| mod_pow2_const | 0.642 | 100.0% |
| mod_var | 0.229 | 35.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 58739ns | 58739ns | base |
| mod_pow2_const | 27680ns | 27680ns | -52.88% |
| mod_var | 73853ns | 73853ns | +25.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 56408ns | base | --- | [56393, 56423] | --- | --- | --- | --- |
| mod_pow2_const | 25532ns | -30854.4ns (-54.7%) | [-30872, -30841]ns | [25530, 25549] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 71597ns | +15190.2ns (+26.9%) | [+15175, +15212]ns | [71580, 71607] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 56402ns | -54.7% | +26.9% |
| 1 | 2 | 56405ns | -54.7% | +26.9% |
| 1 | 3 | 56435ns | -54.8% | +26.9% |
| 1 | 4 | 56420ns | -54.8% | +26.9% |
| 1 | 5 | 56381ns | -54.6% | +27.0% |
| 1 | 6 | 56448ns | -54.8% | +26.9% |
| 1 | 7 | 56402ns | -54.7% | +27.0% |
| 1 | 8 | 56406ns | -54.7% | +26.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.504 | HIGH- (thermal bounce) |
| mod_pow2_const | -0.024 | ok |
| mod_var | 0.139 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.2ns | 56560.6ns | 0.0% |  |
| mod_pow2_const | 3.2ns | 25564.2ns | 0.0% |  |
| mod_var | 3.4ns | 71637.3ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 56331.7-57235.7 ns)
  56331.7 |################################
  56376.9 |########################################
  56422.1 |###########################
  56467.3 |########
  56512.5 |########
  56557.7 |#######
  56602.9 |#####
  56648.1 |#
  56693.3 |#
  56738.5 |
  56783.7 |
  56828.9 |
  56874.1 |#
  56919.3 |
  56964.5 |
  57009.7 |
  57054.9 |
  57100.1 |
  57145.3 |
  57190.5 |
  (20 below, 10 above range)

mod_pow2_const (n=160, range 25525.0-25656.9 ns)
  25525.0 |########################################
  25531.6 |########
  25538.2 |#
  25544.8 |##
  25551.4 |#
  25558.0 |##
  25564.6 |####
  25571.2 |######
  25577.8 |######
  25584.4 |#
  25591.0 |##
  25597.6 |
  25604.2 |
  25610.8 |#
  25617.4 |##
  25624.0 |#
  25630.6 |#
  25637.1 |##
  25643.7 |
  25650.3 |#
  (17 below, 11 above range)

mod_var (n=160, range 71520.2-71870.6 ns)
  71520.2 |#################
  71537.7 |#####################################
  71555.3 |###################################
  71572.8 |#####################################
  71590.3 |########################################
  71607.8 |######################
  71625.3 |##########################
  71642.8 |#############
  71660.4 |####
  71677.9 |########################
  71695.4 |########
  71712.9 |####
  71730.4 |###########
  71748.0 |
  71765.5 |
  71783.0 |##
  71800.5 |####
  71818.0 |
  71835.5 |####
  71853.1 |##
  (18 below, 8 above range)

```
