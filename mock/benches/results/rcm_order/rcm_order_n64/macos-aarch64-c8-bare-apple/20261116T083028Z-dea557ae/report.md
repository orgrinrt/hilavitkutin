# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 80.88 us to slowest 81.25 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_scr shows alternating (throttle bounce) (autocorr -0.52)

rcm_scr's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (369 ns) is smaller than the fastest variant's own run-to-run std-dev (645 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (80.88 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 80883.3 ns median
- Spread: 1.00x (fastest 80883.3 ns, slowest 81252.1 ns)

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
| rcm_adj | 84545ns | 84665ns | 83558ns | 84341ns | 85344ns | base |
| rcm_half | 84716ns | 84750ns | 83881ns | 84524ns | 85422ns | +0.20% |
| rcm_rev | 84614ns | 84356ns | 83397ns | 84231ns | 85796ns | +0.08% |
| rcm_scr | 84412ns | 84502ns | 83199ns | 84299ns | 85188ns | -0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 81136ns | 80583ns | 81797ns | base | 0.001 |
| rcm_half | 81376ns | 80660ns | 82119ns | +0.30% | 0.001 |
| rcm_rev | 81446ns | 80440ns | 82565ns | +0.38% | 0.001 |
| rcm_scr | 81069ns | 80635ns | 81524ns | -0.08% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 99.5% |
| rcm_half | 0.001 | 99.0% |
| rcm_rev | 0.001 | 99.1% |
| rcm_scr | 0.001 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 84545ns | 84545ns | base |
| rcm_half | 84716ns | 84716ns | +0.20% |
| rcm_rev | 84614ns | 84614ns | +0.08% |
| rcm_scr | 84412ns | 84412ns | -0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 80883ns | base | --- | [80727, 81317] | --- | --- | --- | --- |
| rcm_half | 81252ns | no significant difference | [-194, +731]ns | [80896, 81665] | no | 0.2888 | 0.0963 | 0 |
| rcm_rev | 81196ns | no significant difference | [-187, +544]ns | [80806, 81354] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 81033ns | no significant difference | [-400, +288]ns | [80725, 81262] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 80804ns | +1.3% | +0.8% | +0.2% |
| 1 | 2 | 80812ns | +0.6% | +0.5% | +0.2% |
| 1 | 3 | 81508ns | -0.9% | -0.7% | -0.5% |
| 2 | 1 | 81421ns | -0.1% | -0.4% | +0.0% |
| 2 | 2 | 80688ns | +0.0% | -0.1% | +0.0% |
| 2 | 3 | 80725ns | +1.4% | +0.6% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.165 | ok |
| rcm_half | -0.281 | moderate- |
| rcm_rev | -0.120 | ok |
| rcm_scr | -0.521 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rcm_half**: won 1/6, lost 3/6
- **rcm_rev**: won 2/6, lost 3/6
- **rcm_scr**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 2795901.2ns | 81136.3ns | 3445.9% | HIGH |
| rcm_half | 2795577.5ns | 81375.7ns | 3435.4% | HIGH |
| rcm_rev | 2801048.6ns | 81445.6ns | 3439.2% | HIGH |
| rcm_scr | 2792138.9ns | 81069.4ns | 3444.1% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 80583.3-81796.5 ns)
  80583.3 |
  80644.0 |#############
  80704.7 |########################################
  80765.3 |##########################
  80826.0 |#############
  80886.6 |#############
  80947.3 |
  81007.9 |
  81068.6 |
  81129.3 |#############
  81189.9 |##########################
  81250.6 |
  81311.2 |
  81371.9 |#############
  81432.6 |
  81493.2 |#############
  81553.9 |#############
  81614.5 |
  81675.2 |#############
  81735.9 |
  (2 below, 1 above range)

rcm_half (n=18, range 80659.7-82118.8 ns)
  80659.7 |########################################
  80732.7 |########################################
  80805.6 |
  80878.6 |
  80951.5 |####################
  81024.5 |####################
  81097.4 |####################
  81170.4 |####################
  81243.3 |####################
  81316.3 |####################
  81389.2 |
  81462.2 |####################
  81535.1 |####################
  81608.1 |
  81681.0 |
  81754.0 |####################
  81826.9 |########################################
  81899.9 |
  81972.8 |
  82045.8 |
  (1 below, 2 above range)

rcm_rev (n=18, range 80440.3-82565.3 ns)
  80440.3 |
  80546.5 |##########################
  80652.8 |#############
  80759.0 |
  80865.3 |#############
  80971.5 |
  81077.8 |########################################
  81184.0 |########################################
  81290.3 |#############
  81396.5 |##########################
  81502.8 |#############
  81609.0 |
  81715.3 |#############
  81821.5 |
  81927.8 |
  82034.0 |
  82140.3 |
  82246.5 |
  82352.8 |
  82459.0 |
  (2 below, 1 above range)

rcm_scr (n=18, range 80634.7-81524.3 ns)
  80634.7 |
  80679.2 |########################################
  80723.7 |#############
  80768.1 |#############
  80812.6 |
  80857.1 |
  80901.6 |
  80946.1 |##########################
  80990.5 |
  81035.0 |
  81079.5 |##########################
  81124.0 |#############
  81168.5 |
  81212.9 |#############
  81257.4 |#############
  81301.9 |#############
  81346.4 |
  81390.9 |
  81435.3 |#############
  81479.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3453.9% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3436.8% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3438.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3445.7% of algo (FFI overhead may distort results)
