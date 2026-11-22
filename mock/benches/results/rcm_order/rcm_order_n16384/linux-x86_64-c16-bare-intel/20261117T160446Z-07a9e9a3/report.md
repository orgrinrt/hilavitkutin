# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 75.51 ms to slowest 75.91 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_adj shows alternating (throttle bounce) (autocorr -0.57)

rcm_adj's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (399.73 us) is smaller than the fastest variant's own run-to-run std-dev (966.07 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (75.51 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 75507249.2 ns median
- Spread: 1.01x (fastest 75507249.2 ns, slowest 75906981.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 2112757625 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 75796030ns | 75511353ns | 74836515ns | 75431067ns | 76823231ns | base |
| rcm_half | 75712583ns | 75616548ns | 75280849ns | 75601167ns | 76095575ns | -0.11% |
| rcm_rev | 75628063ns | 75541816ns | 75142266ns | 75504318ns | 76056577ns | -0.22% |
| rcm_scr | 76209160ns | 75911243ns | 75293317ns | 75797916ns | 77283949ns | +0.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 75791931ns | 74832520ns | 76819026ns | base | 0.000 |
| rcm_half | 75708366ns | 75276842ns | 76091645ns | -0.11% | 0.000 |
| rcm_rev | 75623833ns | 75138335ns | 76051796ns | -0.22% | 0.000 |
| rcm_scr | 76205030ns | 75289182ns | 77279820ns | +0.55% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 99.1% |
| rcm_half | 0.000 | 99.0% |
| rcm_rev | 0.000 | 99.1% |
| rcm_scr | 0.000 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 75796030ns | 75796030ns | base |
| rcm_half | 75712583ns | 75712583ns | -0.11% |
| rcm_rev | 75628063ns | 75628063ns | -0.22% |
| rcm_scr | 76209160ns | 76209160ns | +0.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 75507249ns | base | --- | [75231534, 75961342] | --- | --- | --- | --- |
| rcm_half | 75612322ns | no significant difference | [-202716, +403890]ns | [75547939, 75768342] | no | 0.3568 | 0.2379 | 0 |
| rcm_rev | 75537882ns | no significant difference | [-288078, +348405]ns | [75377136, 75844391] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 75906982ns | no significant difference | [-351215, +1243524]ns | [75445864, 76796373] | no | 0.3568 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 75672900ns | -0.1% | +0.2% | +0.6% |
| 1 | 2 | 76211237ns | -0.4% | -0.4% | +0.6% |
| 1 | 3 | 75444658ns | +0.3% | +0.0% | -0.1% |
| 2 | 1 | 74841044ns | +0.7% | +0.5% | +3.0% |
| 2 | 2 | 77017404ns | -1.6% | -1.9% | -1.4% |
| 2 | 3 | 75229265ns | +0.5% | -0.1% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.565 | HIGH- (thermal bounce) |
| rcm_half | -0.405 | moderate- |
| rcm_rev | 0.237 | moderate+ |
| rcm_scr | -0.498 | moderate- |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 3/6, lost 2/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 493132316.4ns | 75791931.4ns | 650.6% | HIGH |
| rcm_half | 493345660.3ns | 75708365.8ns | 651.6% | HIGH |
| rcm_rev | 491774382.2ns | 75623833.1ns | 650.3% | HIGH |
| rcm_scr | 492865552.9ns | 76205030.3ns | 646.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 74832520.0-76819026.0 ns)
  74832520.0 |########################################
  74931845.3 |
  75031170.6 |####################
  75130495.9 |####################
  75229821.2 |####################
  75329146.5 |########################################
  75428471.8 |####################
  75527797.1 |####################
  75627122.4 |####################
  75726447.7 |####################
  75825773.0 |####################
  75925098.3 |
  76024423.6 |####################
  76123748.9 |####################
  76223074.2 |
  76322399.5 |
  76421724.8 |
  76521050.1 |
  76620375.4 |
  76719700.7 |
  (1 below, 3 above range)

rcm_half (n=18, range 75276842.0-76091645.2 ns)
  75276842.0 |
  75317582.2 |#############
  75358322.3 |#############
  75399062.5 |
  75439802.7 |
  75480542.8 |
  75521283.0 |########################################
  75562023.1 |########################################
  75602763.3 |#############
  75643503.5 |#############
  75684243.6 |#############
  75724983.8 |#############
  75765723.9 |#############
  75806464.1 |
  75847204.2 |#############
  75887944.4 |#############
  75928684.6 |
  75969424.7 |
  76010164.9 |
  76050905.0 |#############
  (1 below, 1 above range)

rcm_rev (n=18, range 75138334.9-76051795.6 ns)
  75138334.9 |
  75184008.0 |########################################
  75229681.0 |
  75275354.0 |
  75321027.1 |####################
  75366700.1 |
  75412373.1 |########################################
  75458046.2 |####################
  75503719.2 |####################
  75549392.2 |####################
  75595065.3 |
  75640738.3 |
  75686411.3 |
  75732084.4 |########################################
  75777757.4 |
  75823430.4 |########################################
  75869103.5 |####################
  75914776.5 |####################
  75960449.5 |
  76006122.6 |
  (2 below, 2 above range)

rcm_scr (n=18, range 75289182.0-77279819.8 ns)
  75289182.0 |########################################
  75388713.9 |##########################
  75488245.8 |#############
  75587777.7 |
  75687309.5 |#############
  75786841.4 |
  75886373.3 |##########################
  75985905.2 |
  76085437.1 |#############
  76184969.0 |
  76284500.9 |
  76384032.8 |
  76483564.7 |
  76583096.5 |#############
  76682628.4 |#############
  76782160.3 |##########################
  76881692.2 |
  76981224.1 |#############
  77080756.0 |
  77180287.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=652.1% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=652.2% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=650.4% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=648.6% of algo (FFI overhead may distort results)
