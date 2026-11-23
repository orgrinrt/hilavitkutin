# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.7% (fastest 9.47 ms to slowest 9.54 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (68.21 us) is smaller than the fastest variant's own run-to-run std-dev (102.63 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_scr** at 9474964.9 ns median (-0.7% vs baseline)
- Spread: 1.01x (fastest 9474964.9 ns, slowest 9543178.9 ns)

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
| Counter frequency | 2112776360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 9584308ns | 9543471ns | 9250786ns | 9463052ns | 9932952ns | base |
| rcm_half | 9673602ns | 9547042ns | 9406110ns | 9515026ns | 10045212ns | +0.93% |
| rcm_rev | 9582806ns | 9543301ns | 9467528ns | 9527671ns | 9723148ns | -0.02% |
| rcm_scr | 9487977ns | 9478763ns | 9366742ns | 9444937ns | 9613156ns | -1.01% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 9580492ns | 9246911ns | 9929262ns | base | 0.000 |
| rcm_half | 9669777ns | 9402205ns | 10041472ns | +0.93% | 0.000 |
| rcm_rev | 9578844ns | 9463584ns | 9719129ns | -0.02% | 0.000 |
| rcm_scr | 9484246ns | 9362945ns | 9609406ns | -1.00% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 96.9% |
| rcm_half | 0.000 | 96.9% |
| rcm_rev | 0.000 | 96.9% |
| rcm_scr | 0.000 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 9584308ns | 9584308ns | base |
| rcm_half | 9673602ns | 9673602ns | +0.93% |
| rcm_rev | 9582806ns | 9582806ns | -0.02% |
| rcm_scr | 9487977ns | 9487977ns | -1.01% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 9539784ns | base | --- | [9370350, 9654105] | --- | --- | --- | --- |
| rcm_half | 9543179ns | no significant difference | [-69965, +189632]ns | [9430639, 9750953] | no | 0.3568 | 0.2379 | 0 |
| rcm_rev | 9539424ns | no significant difference | [-177794, +214760]ns | [9498745, 9572041] | no | 0.3568 | 0.2379 | 0 |
| rcm_scr | 9474965ns | no significant difference | [-224251, +85309]ns | [9401484, 9553435] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 9579053ns | +0.2% | +2.1% | +0.2% |
| 1 | 2 | 10006451ns | -4.9% | -4.4% | -5.1% |
| 1 | 3 | 9676123ns | +5.0% | -1.7% | -2.1% |
| 2 | 1 | 9400246ns | +4.6% | +1.4% | -0.5% |
| 2 | 2 | 9308564ns | +1.3% | +2.0% | +1.4% |
| 2 | 3 | 9268306ns | +1.6% | +2.3% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.411 | moderate+ |
| rcm_half | 0.092 | ok |
| rcm_rev | 0.147 | ok |
| rcm_scr | 0.205 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 1/6, lost 5/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 61605989.6ns | 9580491.6ns | 643.0% | HIGH |
| rcm_half | 61691254.2ns | 9669776.9ns | 638.0% | HIGH |
| rcm_rev | 61357634.0ns | 9578844.0ns | 640.6% | HIGH |
| rcm_scr | 61610452.9ns | 9484245.9ns | 649.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 9246911.0-9929261.6 ns)
  9246911.0 |#############
  9281028.6 |#############
  9315146.1 |#############
  9349263.6 |
  9383381.1 |########################################
  9417498.7 |
  9451616.2 |
  9485733.7 |
  9519851.2 |#############
  9553968.8 |##########################
  9588086.3 |#############
  9622203.8 |#############
  9656321.4 |#############
  9690438.9 |
  9724556.4 |
  9758673.9 |
  9792791.5 |
  9826909.0 |
  9861026.5 |
  9895144.0 |
  (2 below, 4 above range)

rcm_half (n=18, range 9402204.7-10041472.2 ns)
  9402204.7 |########################################
  9434168.0 |
  9466131.4 |####################
  9498094.8 |##########
  9530058.2 |
  9562021.5 |##########
  9593984.9 |##########
  9625948.3 |
  9657911.7 |####################
  9689875.0 |
  9721838.4 |
  9753801.8 |
  9785765.2 |
  9817728.5 |##########
  9849691.9 |
  9881655.3 |
  9913618.7 |
  9945582.0 |
  9977545.4 |
  10009508.8 |
  (2 below, 4 above range)

rcm_rev (n=18, range 9463584.2-9719129.0 ns)
  9463584.2 |#############
  9476361.5 |#############
  9489138.7 |##########################
  9501916.0 |########################################
  9514693.2 |
  9527470.4 |#############
  9540247.7 |#############
  9553024.9 |########################################
  9565802.2 |
  9578579.4 |#############
  9591356.6 |
  9604133.9 |
  9616911.1 |
  9629688.4 |
  9642465.6 |
  9655242.8 |
  9668020.1 |
  9680797.3 |
  9693574.6 |
  9706351.8 |
  (1 below, 4 above range)

rcm_scr (n=18, range 9362944.8-9609406.3 ns)
  9362944.8 |
  9375267.8 |##########################
  9387590.9 |#############
  9399914.0 |
  9412237.1 |#############
  9424560.2 |##########################
  9436883.2 |
  9449206.3 |
  9461529.4 |
  9473852.5 |########################################
  9486175.6 |#############
  9498498.6 |
  9510821.7 |
  9523144.8 |
  9535467.9 |#############
  9547791.0 |
  9560114.0 |#############
  9572437.1 |#############
  9584760.2 |#############
  9597083.3 |
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=643.5% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=643.4% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=643.0% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=647.9% of algo (FFI overhead may distort results)
