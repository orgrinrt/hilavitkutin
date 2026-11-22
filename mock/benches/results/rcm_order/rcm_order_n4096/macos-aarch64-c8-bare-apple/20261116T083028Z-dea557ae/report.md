# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 8.58 ms to slowest 8.63 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_scr shows alternating (throttle bounce) (autocorr -0.57)

rcm_scr's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (44.80 us) is smaller than the fastest variant's own run-to-run std-dev (137.87 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_half** at 8580364.6 ns median (-0.2% vs baseline)
- Spread: 1.01x (fastest 8580364.6 ns, slowest 8625166.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 8654545ns | 8600131ns | 8521990ns | 8588685ns | 8819613ns | base |
| rcm_half | 8647061ns | 8586512ns | 8515033ns | 8586759ns | 8803528ns | -0.09% |
| rcm_rev | 8665732ns | 8631050ns | 8510926ns | 8620506ns | 8810974ns | +0.13% |
| rcm_scr | 8635252ns | 8591667ns | 8489522ns | 8583690ns | 8785459ns | -0.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 8648600ns | 8515904ns | 8813783ns | base | 0.000 |
| rcm_half | 8641164ns | 8509250ns | 8797556ns | -0.09% | 0.000 |
| rcm_rev | 8660029ns | 8505153ns | 8805367ns | +0.13% | 0.000 |
| rcm_scr | 8629225ns | 8483556ns | 8779454ns | -0.22% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.7% |
| rcm_half | 0.000 | 98.9% |
| rcm_rev | 0.000 | 98.4% |
| rcm_scr | 0.000 | 98.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 8654545ns | 8654545ns | base |
| rcm_half | 8647061ns | 8647061ns | -0.09% |
| rcm_rev | 8665732ns | 8665732ns | +0.13% |
| rcm_scr | 8635252ns | 8635252ns | -0.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 8593850ns | base | --- | [8558962, 8648310] | --- | --- | --- | --- |
| rcm_half | 8580365ns | no significant difference | [-40321, +68867]ns | [8561858, 8660812] | no | 1.0000 | 1.0000 | 0 |
| rcm_rev | 8625167ns | no significant difference | [-29300, +126921]ns | [8604902, 8737088] | no | 1.0000 | 0.4807 | 0 |
| rcm_scr | 8585883ns | no significant difference | [-89125, +52352]ns | [8547435, 8647831] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 8658896ns | -0.9% | +0.2% | -1.9% |
| 1 | 2 | 8593588ns | -0.2% | -0.8% | +1.6% |
| 1 | 3 | 8577804ns | +0.0% | +0.6% | +0.2% |
| 2 | 1 | 8559100ns | +1.3% | +0.7% | +0.8% |
| 2 | 2 | 8569846ns | -0.1% | +0.8% | -0.1% |
| 2 | 3 | 8601442ns | +0.4% | +0.2% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.184 | ok |
| rcm_half | -0.508 | HIGH- (thermal bounce) |
| rcm_rev | -0.513 | HIGH- (thermal bounce) |
| rcm_scr | -0.573 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 2/6
- **rcm_rev**: won 1/6, lost 5/6
- **rcm_scr**: won 1/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 179113411.1ns | 8648600.0ns | 2071.0% | HIGH |
| rcm_half | 179124490.5ns | 8641163.9ns | 2072.9% | HIGH |
| rcm_rev | 179108051.6ns | 8660028.9ns | 2068.2% | HIGH |
| rcm_scr | 179201785.4ns | 8629224.8ns | 2076.7% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 8515904.2-8813783.3 ns)
  8515904.2 |#############
  8530798.1 |#############
  8545692.1 |########################################
  8560586.0 |#############
  8575480.0 |#############
  8590374.0 |########################################
  8605267.9 |
  8620161.9 |#############
  8635055.8 |#############
  8649949.8 |#############
  8664843.8 |#############
  8679737.7 |
  8694631.7 |
  8709525.6 |
  8724419.6 |
  8739313.5 |
  8754207.5 |
  8769101.5 |
  8783995.4 |
  8798889.4 |
  (1 below, 3 above range)

rcm_half (n=18, range 8509250.0-8797556.3 ns)
  8509250.0 |##########
  8523665.3 |
  8538080.7 |
  8552496.0 |########################################
  8566911.3 |########################################
  8581326.6 |
  8595741.9 |##########
  8610157.2 |
  8624572.5 |##########
  8638987.8 |##########
  8653403.2 |
  8667818.5 |##########
  8682233.8 |
  8696649.1 |
  8711064.4 |##########
  8725479.7 |
  8739895.0 |
  8754310.3 |
  8768725.6 |
  8783141.0 |
  (1 below, 3 above range)

rcm_rev (n=18, range 8505152.8-8805366.7 ns)
  8505152.8 |##########
  8520163.5 |####################
  8535174.2 |
  8550184.9 |
  8565195.5 |
  8580206.2 |
  8595216.9 |##########
  8610227.6 |########################################
  8625238.3 |##########
  8640249.0 |##########
  8655259.7 |
  8670270.4 |##########
  8685281.1 |
  8700291.8 |
  8715302.5 |
  8730313.2 |##########
  8745323.9 |##########
  8760334.6 |
  8775345.3 |
  8790356.0 |##########
  (1 below, 3 above range)

rcm_scr (n=18, range 8483555.5-8779454.2 ns)
  8483555.5 |########################################
  8498350.5 |
  8513145.4 |
  8527940.3 |####################
  8542735.3 |########################################
  8557530.2 |########################################
  8572325.1 |####################
  8587120.1 |####################
  8601915.0 |
  8616709.9 |####################
  8631504.8 |########################################
  8646299.8 |####################
  8661094.7 |
  8675889.6 |
  8690684.6 |
  8705479.5 |
  8720274.4 |####################
  8735069.4 |
  8749864.3 |
  8764659.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2082.5% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2086.1% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2075.3% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2085.2% of algo (FFI overhead may distort results)
