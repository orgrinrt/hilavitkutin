# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (10.25 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 5.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 94% faster than the next best (sat_saturate)

sat_wrap (5.27 us) leads sat_saturate (10.24 us) by 94%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 49% (significant)

sat_wrap is -5.06 us (49%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap shows alternating (throttle bounce) (autocorr -0.64)

sat_wrap's per-pass series has lag-1 autocorrelation -0.64, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sat_wrap** at 5270.8 ns median (-48.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.94x (fastest 5270.8 ns, slowest 10247.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 12483ns | 12340ns | 12319ns | 12374ns | 12975ns | base |
| sat_saturate | 12413ns | 12327ns | 12317ns | 12356ns | 12682ns | -0.56% |
| sat_wrap | 7432ns | 7405ns | 7217ns | 7403ns | 7732ns | -40.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 10373ns | 10236ns | 10789ns | base | 1.579 |
| sat_saturate | 10316ns | 10236ns | 10540ns | -0.56% | 1.588 |
| sat_wrap | 5289ns | 5137ns | 5512ns | -49.02% | 3.098 |

## Performance model

- Peak throughput: **3.189 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.599 | 50.1% |
| sat_saturate | 1.599 | 50.1% |
| sat_wrap | 3.108 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 12483ns | 12483ns | base |
| sat_saturate | 12413ns | 12413ns | -0.56% |
| sat_wrap | 7432ns | 7432ns | -40.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 10247ns | base | --- | [10245, 10270] | --- | --- | --- | --- |
| sat_saturate | 10245ns | no significant difference | [-5, +2]ns | [10242, 10247] | no | 0.3816 | 0.3816 | 2 |
| sat_wrap | 5271ns | -5061.8ns (-49.4%) | [-5097, -5015]ns | [5206, 5304] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 10630ns | -3.7% | -51.6% |
| 1 | 2 | 10244ns | -0.0% | -47.0% |
| 1 | 3 | 10241ns | +1.5% | -49.8% |
| 1 | 4 | 10245ns | -0.0% | -48.4% |
| 1 | 5 | 10244ns | -0.0% | -48.2% |
| 1 | 6 | 10242ns | +0.0% | -47.7% |
| 1 | 7 | 10238ns | +1.7% | -49.8% |
| 1 | 8 | 10610ns | -3.5% | -48.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.046 | ok |
| sat_saturate | -0.364 | moderate- |
| sat_wrap | -0.642 | HIGH- (thermal bounce) |

**Consistency summary:**

- **sat_saturate**: won 2/8, lost 2/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 3.0ns | 10373.2ns | 0.0% |  |
| sat_saturate | 3.1ns | 10315.6ns | 0.0% |  |
| sat_wrap | 2.9ns | 5288.7ns | 0.1% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 10235.6-10789.3 ns)
  10235.6 |########################################
  10263.3 |##########
  10290.9 |#
  10318.6 |#
  10346.3 |##
  10374.0 |#
  10401.7 |
  10429.4 |#
  10457.1 |
  10484.8 |
  10512.5 |#
  10540.1 |
  10567.8 |#
  10595.5 |##
  10623.2 |##
  10650.9 |
  10678.6 |
  10706.3 |
  10734.0 |
  10761.7 |#
  (17 below, 14 above range)

sat_saturate (n=160, range 10235.6-10540.1 ns)
  10235.6 |########################################
  10250.9 |##
  10266.1 |
  10281.3 |####
  10296.5 |##
  10311.8 |##
  10327.0 |#
  10342.2 |##
  10357.4 |
  10372.7 |
  10387.9 |###
  10403.1 |###
  10418.3 |#
  10433.5 |
  10448.8 |#
  10464.0 |
  10479.2 |
  10494.4 |
  10509.7 |
  10524.9 |
  (18 below, 13 above range)

sat_wrap (n=160, range 5137.0-5512.2 ns)
   5137.0 |########################################
   5155.8 |#####
   5174.5 |##########
   5193.3 |#######
   5212.0 |##
   5230.8 |####
   5249.6 |####
   5268.3 |#########
   5287.1 |#####
   5305.8 |#####
   5324.6 |########
   5343.3 |#########
   5362.1 |##
   5380.9 |###
   5399.6 |#######
   5418.4 |###########
   5437.1 |###
   5455.9 |####
   5474.6 |###
   5493.4 |###
  (15 below, 14 above range)

```
