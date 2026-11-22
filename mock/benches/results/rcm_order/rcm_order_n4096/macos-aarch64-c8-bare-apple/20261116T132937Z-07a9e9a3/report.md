# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.8% (fastest 8.39 ms to slowest 8.45 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_rev shows alternating (throttle bounce) (autocorr -0.84)

rcm_rev's per-pass series has lag-1 autocorrelation -0.84, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (66.26 us) is smaller than the fastest variant's own run-to-run std-dev (125.23 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_rev** at 8388464.6 ns median (-0.5% vs baseline)
- Spread: 1.01x (fastest 8388464.6 ns, slowest 8454720.8 ns)

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
| rcm_adj | 8573184ns | 8433017ns | 8338024ns | 8422096ns | 8917395ns | base |
| rcm_half | 8499856ns | 8461112ns | 8354746ns | 8439595ns | 8662804ns | -0.86% |
| rcm_rev | 8427771ns | 8395598ns | 8315726ns | 8376464ns | 8560753ns | -1.70% |
| rcm_scr | 8550858ns | 8408085ns | 8329217ns | 8395904ns | 8894109ns | -0.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 8566960ns | 8332017ns | 8910871ns | base | 0.000 |
| rcm_half | 8493816ns | 8349315ns | 8656739ns | -0.85% | 0.000 |
| rcm_rev | 8421680ns | 8310181ns | 8554630ns | -1.70% | 0.000 |
| rcm_scr | 8544673ns | 8323472ns | 8887924ns | -0.26% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.6% |
| rcm_half | 0.000 | 98.3% |
| rcm_rev | 0.000 | 99.1% |
| rcm_scr | 0.000 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 8573184ns | 8573184ns | base |
| rcm_half | 8499856ns | 8499856ns | -0.86% |
| rcm_rev | 8427771ns | 8427771ns | -1.70% |
| rcm_scr | 8550858ns | 8550858ns | -0.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 8427308ns | base | --- | [8385727, 8480894] | --- | --- | --- | --- |
| rcm_half | 8454721ns | no significant difference | [-91556, +136075]ns | [8395646, 8553465] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 8388465ns | no significant difference | [-120356, +38279]ns | [8334385, 8447965] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 8401458ns | no significant difference | [-101900, +54704]ns | [8357927, 8577840] | no | 0.8145 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 8375725ns | +0.7% | +0.7% | +0.8% |
| 1 | 2 | 8416654ns | -0.4% | -0.7% | -0.6% |
| 1 | 3 | 8471262ns | -0.4% | -0.0% | -1.4% |
| 2 | 1 | 8459604ns | +1.2% | -1.5% | -1.1% |
| 2 | 2 | 8472333ns | -0.4% | -0.5% | +8.7% |
| 2 | 3 | 8665429ns | -2.2% | -3.6% | -2.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.115 | ok |
| rcm_half | -0.102 | ok |
| rcm_rev | -0.838 | HIGH- (thermal bounce) |
| rcm_scr | -0.183 | ok |

**Consistency summary:**

- **rcm_half**: won 4/6, lost 2/6
- **rcm_rev**: won 4/6, lost 1/6
- **rcm_scr**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 179299876.8ns | 8566960.4ns | 2092.9% | HIGH |
| rcm_half | 178608035.0ns | 8493816.4ns | 2102.8% | HIGH |
| rcm_rev | 178541460.4ns | 8421679.9ns | 2120.0% | HIGH |
| rcm_scr | 178823195.4ns | 8544672.7ns | 2092.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 8332016.7-8910870.8 ns)
  8332016.7 |#############
  8360959.4 |########################################
  8389902.1 |##########################
  8418844.8 |##########################
  8447787.5 |########################################
  8476730.2 |#############
  8505672.9 |
  8534615.6 |
  8563558.3 |
  8592501.0 |
  8621443.8 |
  8650386.5 |#############
  8679329.2 |
  8708271.9 |
  8737214.6 |#############
  8766157.3 |
  8795100.0 |#############
  8824042.7 |
  8852985.4 |
  8881928.1 |
  (2 below, 1 above range)

rcm_half (n=18, range 8349315.3-8656738.9 ns)
  8349315.3 |##########################
  8364686.4 |#############
  8380057.6 |
  8395428.8 |#############
  8410800.0 |
  8426171.2 |########################################
  8441542.4 |
  8456913.5 |#############
  8472284.7 |#############
  8487655.9 |
  8503027.1 |#############
  8518398.3 |
  8533769.4 |#############
  8549140.6 |#############
  8564511.8 |
  8579883.0 |
  8595254.2 |#############
  8610625.3 |
  8625996.5 |
  8641367.7 |#############
  (2 below, 2 above range)

rcm_rev (n=18, range 8310180.6-8554629.9 ns)
  8310180.6 |#############
  8322403.0 |########################################
  8334625.5 |#############
  8346848.0 |##########################
  8359070.4 |#############
  8371292.9 |
  8383515.4 |
  8395737.8 |
  8407960.3 |#############
  8420182.8 |########################################
  8432405.2 |
  8444627.7 |
  8456850.1 |##########################
  8469072.6 |
  8481295.1 |
  8493517.5 |#############
  8505740.0 |
  8517962.5 |
  8530184.9 |
  8542407.4 |
  (1 below, 2 above range)

rcm_scr (n=18, range 8323472.2-8887924.3 ns)
  8323472.2 |#############
  8351694.8 |########################################
  8379917.4 |
  8408140.0 |######
  8436362.6 |#############
  8464585.2 |
  8492807.9 |
  8521030.5 |
  8549253.1 |######
  8577475.7 |######
  8605698.3 |
  8633920.9 |
  8662143.5 |
  8690366.1 |######
  8718588.7 |
  8746811.3 |
  8775033.9 |
  8803256.5 |
  8831479.1 |######
  8859701.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2118.3% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2111.2% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2128.0% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2126.9% of algo (FFI overhead may distort results)
