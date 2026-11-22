# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 121% faster than the next best (mod_npow2_const)

mod_pow2_const (25.55 us) leads mod_npow2_const (56.39 us) by 121%, a clear separation rather than a photo finish. CV 0.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -30.84 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (71.60 us) is 2.8x the fastest (25.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_var shows alternating (throttle bounce) (autocorr -0.68)

mod_var's per-pass series has lag-1 autocorrelation -0.68, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: mod_pow2_const** at 25555.0 ns median (-54.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.80x (fastest 25555.0 ns, slowest 71601.0 ns)

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
| mod_npow2_const | 58661ns | 58569ns | 58478ns | 58575ns | 59102ns | base |
| mod_pow2_const | 27732ns | 27673ns | 27626ns | 27673ns | 28015ns | -52.72% |
| mod_var | 73852ns | 73796ns | 73689ns | 73806ns | 74152ns | +25.90% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 56495ns | 56335ns | 56917ns | base | 0.290 |
| mod_pow2_const | 25611ns | 25526ns | 25858ns | -54.67% | 0.640 |
| mod_var | 71656ns | 71534ns | 71913ns | +26.83% | 0.229 |

## Performance model

- Peak throughput: **0.642 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.291 | 45.3% |
| mod_pow2_const | 0.641 | 99.9% |
| mod_var | 0.229 | 35.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 58661ns | 58661ns | base |
| mod_pow2_const | 27732ns | 27732ns | -52.72% |
| mod_var | 73852ns | 73852ns | +25.90% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 56391ns | base | --- | [56380, 56410] | --- | --- | --- | --- |
| mod_pow2_const | 25555ns | -30844.3ns (-54.7%) | [-30852, -30830]ns | [25532, 25575] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 71601ns | +15207.9ns (+27.0%) | [+15182, +15222]ns | [71587, 71614] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 56438ns | -54.7% | +26.9% |
| 1 | 2 | 56370ns | -54.7% | +27.0% |
| 1 | 3 | 56375ns | -54.7% | +27.0% |
| 1 | 4 | 56388ns | -54.7% | +27.1% |
| 1 | 5 | 56379ns | -54.7% | +27.0% |
| 1 | 6 | 56489ns | -54.8% | +26.8% |
| 1 | 7 | 56400ns | -54.6% | +27.0% |
| 1 | 8 | 56404ns | -54.7% | +26.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | -0.150 | ok |
| mod_pow2_const | -0.661 | HIGH- (thermal bounce) |
| mod_var | -0.679 | HIGH- (thermal bounce) |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.4ns | 56495.5ns | 0.0% |  |
| mod_pow2_const | 3.3ns | 25610.8ns | 0.0% |  |
| mod_var | 3.5ns | 71655.6ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 56334.9-56916.5 ns)
  56334.9 |########
  56364.0 |########################################
  56393.0 |################
  56422.1 |#########
  56451.2 |#######
  56480.3 |####
  56509.4 |########
  56538.5 |###
  56567.5 |#####
  56596.6 |#
  56625.7 |##
  56654.8 |#
  56683.9 |
  56712.9 |
  56742.0 |
  56771.1 |
  56800.2 |
  56829.3 |
  56858.4 |
  56887.4 |
  (25 below, 5 above range)

mod_pow2_const (n=160, range 25525.6-25858.0 ns)
  25525.6 |########################################
  25542.2 |###
  25558.8 |########
  25575.4 |##############
  25592.0 |#####
  25608.7 |###
  25625.3 |###
  25641.9 |#
  25658.5 |##
  25675.1 |#
  25691.8 |
  25708.4 |#
  25725.0 |
  25741.6 |
  25758.2 |
  25774.9 |
  25791.5 |
  25808.1 |
  25824.7 |
  25841.3 |
  (12 below, 8 above range)

mod_var (n=160, range 71533.6-71913.3 ns)
  71533.6 |####################
  71552.5 |#################################
  71571.5 |########################################
  71590.5 |####################
  71609.5 |###########################
  71628.5 |##############
  71647.5 |######
  71666.5 |########
  71685.4 |#########
  71704.4 |#
  71723.4 |######
  71742.4 |########
  71761.4 |
  71780.4 |####
  71799.3 |####
  71818.3 |#
  71837.3 |###
  71856.3 |#
  71875.3 |#
  71894.3 |###
  (13 below, 11 above range)

```
