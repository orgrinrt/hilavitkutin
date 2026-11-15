# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 118% faster than the next best (mod_npow2_const)

mod_pow2_const (6.57 us) leads mod_npow2_const (14.32 us) by 118%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 54% (significant)

mod_pow2_const is -7.74 us (54%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (18.17 us) is 2.8x the fastest (6.57 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### mod_var shows warm-up / thermal drift (autocorr +0.61)

mod_var's per-pass series has lag-1 autocorrelation +0.61, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: mod_pow2_const** at 6566.6 ns median (-54.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.77x (fastest 6566.6 ns, slowest 18165.8 ns)

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
| mod_npow2_const | 16489ns | 16464ns | 16135ns | 16469ns | 16903ns | base |
| mod_pow2_const | 8683ns | 8727ns | 8442ns | 8638ns | 9060ns | -47.34% |
| mod_var | 20348ns | 20324ns | 19924ns | 20323ns | 20845ns | +23.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 14342ns | 14039ns | 14700ns | base | 0.286 |
| mod_pow2_const | 6528ns | 6347ns | 6811ns | -54.48% | 0.627 |
| mod_var | 18188ns | 17813ns | 18629ns | +26.81% | 0.225 |

## Performance model

- Peak throughput: **0.645 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.286 | 44.3% |
| mod_pow2_const | 0.624 | 96.7% |
| mod_var | 0.225 | 34.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 16489ns | 16489ns | base |
| mod_pow2_const | 8683ns | 8683ns | -47.34% |
| mod_var | 20348ns | 20348ns | +23.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 14317ns | base | --- | [14206, 14525] | --- | --- | --- | --- |
| mod_pow2_const | 6567ns | -7737.6ns (-54.0%) | [-7944, -7709]ns | [6393, 6570] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 18166ns | +3864.1ns (+27.0%) | [+3789, +3901]ns | [18055, 18336] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 14331ns | -55.7% | +26.1% |
| 1 | 2 | 14537ns | -54.8% | +27.0% |
| 1 | 3 | 14537ns | -54.7% | +26.9% |
| 1 | 4 | 14529ns | -56.3% | +27.0% |
| 1 | 5 | 14079ns | -53.8% | +29.1% |
| 1 | 6 | 14189ns | -52.1% | +25.6% |
| 1 | 7 | 14041ns | -54.8% | +27.0% |
| 1 | 8 | 14039ns | -51.6% | +27.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | 0.527 | HIGH+ (drift/warm-up) |
| mod_pow2_const | -0.463 | moderate- |
| mod_var | 0.612 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 2.8ns | 14342.3ns | 0.0% |  |
| mod_pow2_const | 2.6ns | 6528.4ns | 0.0% |  |
| mod_var | 3.2ns | 18188.1ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 14038.6-14700.1 ns)
  14038.6 |########################################
  14071.7 |#######
  14104.8 |
  14137.8 |##
  14170.9 |##########
  14204.0 |#########
  14237.1 |
  14270.1 |
  14303.2 |##########
  14336.3 |##
  14369.3 |
  14402.4 |
  14435.5 |
  14468.6 |#
  14501.6 |######################################
  14534.7 |############################
  14567.8 |####
  14600.9 |#
  14633.9 |
  14667.0 |#
  (16 below, 10 above range)

mod_pow2_const (n=160, range 6347.1-6810.6 ns)
   6347.1 |########################################
   6370.3 |######
   6393.5 |###
   6416.7 |
   6439.8 |
   6463.0 |
   6486.2 |
   6509.3 |
   6532.5 |
   6555.7 |#################################
   6578.9 |####
   6602.0 |
   6625.2 |
   6648.4 |
   6671.6 |
   6694.7 |#
   6717.9 |
   6741.1 |
   6764.3 |
   6787.4 |#####################
  (14 below, 9 above range)

mod_var (n=160, range 17813.4-18628.7 ns)
  17813.4 |############################
  17854.2 |###########
  17895.0 |##
  17935.7 |####
  17976.5 |##
  18017.3 |###
  18058.0 |##
  18098.8 |#
  18139.6 |###########
  18180.3 |###
  18221.1 |##
  18261.8 |####
  18302.6 |#
  18343.4 |
  18384.1 |##
  18424.9 |########################################
  18465.7 |############
  18506.4 |##
  18547.2 |##
  18587.9 |#
  (19 below, 9 above range)

```

## Diagnostics

- **mod_npow2_const**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **mod_var**: autocorrelation=0.61 (measurement drift or warm-up artifact)
