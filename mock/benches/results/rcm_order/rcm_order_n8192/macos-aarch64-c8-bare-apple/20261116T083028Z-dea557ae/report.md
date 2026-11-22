# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.9% (fastest 17.32 ms to slowest 17.48 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (160.90 us) is smaller than the fastest variant's own run-to-run std-dev (317.83 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_scr** at 17322733.4 ns median (-0.2% vs baseline)
- Spread: 1.01x (fastest 17322733.4 ns, slowest 17483631.2 ns)

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
| rcm_adj | 17530815ns | 17367917ns | 17248675ns | 17356498ns | 17933360ns | base |
| rcm_half | 17476471ns | 17393715ns | 17220319ns | 17377137ns | 17753548ns | -0.31% |
| rcm_rev | 17490873ns | 17491469ns | 17231753ns | 17411774ns | 17739082ns | -0.23% |
| rcm_scr | 17462188ns | 17333750ns | 17199743ns | 17317886ns | 17809865ns | -0.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 17522765ns | 17241096ns | 17925051ns | base | 0.000 |
| rcm_half | 17467985ns | 17211051ns | 17745657ns | -0.31% | 0.000 |
| rcm_rev | 17483142ns | 17224615ns | 17730903ns | -0.23% | 0.000 |
| rcm_scr | 17454263ns | 17192679ns | 17802216ns | -0.39% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 99.0% |
| rcm_half | 0.000 | 98.9% |
| rcm_rev | 0.000 | 98.3% |
| rcm_scr | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 17530815ns | 17530815ns | base |
| rcm_half | 17476471ns | 17476471ns | -0.31% |
| rcm_rev | 17490873ns | 17490873ns | -0.23% |
| rcm_scr | 17462188ns | 17462188ns | -0.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 17360552ns | base | --- | [17317244, 17626125] | --- | --- | --- | --- |
| rcm_half | 17386023ns | no significant difference | [-125548, +236973]ns | [17286819, 17619150] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 17483631ns | no significant difference | [-116121, +215342]ns | [17293260, 17588206] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 17322733ns | no significant difference | [-289815, +122881]ns | [17275535, 17439608] | no | 0.8145 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 17343683ns | +1.8% | +0.1% | -0.4% |
| 1 | 2 | 17366196ns | -0.4% | -0.6% | -0.5% |
| 1 | 3 | 17315583ns | +1.3% | +1.7% | +0.6% |
| 2 | 1 | 17602517ns | -1.0% | -0.4% | -1.8% |
| 2 | 2 | 17367400ns | -0.5% | +0.9% | +0.1% |
| 2 | 3 | 17255212ns | +0.5% | +1.1% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.192 | ok |
| rcm_half | -0.298 | moderate- |
| rcm_rev | 0.064 | ok |
| rcm_scr | -0.307 | moderate- |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 2/6, lost 3/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 358568324.3ns | 17522764.8ns | 2046.3% | HIGH |
| rcm_half | 358587267.6ns | 17467984.7ns | 2052.8% | HIGH |
| rcm_rev | 358718257.4ns | 17483142.4ns | 2051.8% | HIGH |
| rcm_scr | 358559906.7ns | 17454263.4ns | 2054.3% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 17241095.8-17925051.4 ns)
  17241095.8 |####################
  17275293.6 |##########
  17309491.4 |########################################
  17343689.2 |##############################
  17377886.9 |
  17412084.7 |##########
  17446282.5 |
  17480480.3 |
  17514678.1 |
  17548875.8 |
  17583073.6 |##########
  17617271.4 |##########
  17651469.2 |##########
  17685666.9 |##########
  17719864.7 |
  17754062.5 |
  17788260.3 |
  17822458.1 |
  17856655.8 |
  17890853.6 |
  (1 below, 2 above range)

rcm_half (n=18, range 17211051.4-17745656.9 ns)
  17211051.4 |
  17237781.7 |#############
  17264512.0 |##########################
  17291242.2 |########################################
  17317972.5 |#############
  17344702.8 |
  17371433.1 |
  17398163.3 |
  17424893.6 |#############
  17451623.9 |
  17478354.2 |
  17505084.5 |
  17531814.7 |##########################
  17558545.0 |
  17585275.3 |#############
  17612005.6 |#############
  17638735.8 |#############
  17665466.1 |
  17692196.4 |#############
  17718926.7 |
  (2 below, 2 above range)

rcm_rev (n=18, range 17224615.3-17730902.8 ns)
  17224615.3 |#############
  17249929.6 |#############
  17275244.0 |##########################
  17300558.4 |
  17325872.8 |
  17351187.1 |##########################
  17376501.5 |
  17401815.9 |
  17427130.3 |#############
  17452444.6 |
  17477759.0 |
  17503073.4 |#############
  17528387.8 |#############
  17553702.1 |
  17579016.5 |########################################
  17604330.9 |#############
  17629645.3 |
  17654959.6 |
  17680274.0 |
  17705588.4 |
  (2 below, 3 above range)

rcm_scr (n=18, range 17192679.2-17802216.0 ns)
  17192679.2 |
  17223156.0 |########
  17253632.8 |########################################
  17284109.7 |
  17314586.5 |################
  17345063.4 |########
  17375540.2 |########
  17406017.1 |########
  17436493.9 |########
  17466970.7 |
  17497447.6 |
  17527924.4 |
  17558401.3 |
  17588878.1 |
  17619354.9 |########
  17649831.8 |
  17680308.6 |
  17710785.5 |
  17741262.3 |
  17771739.1 |########
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2064.4% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2061.6% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2051.4% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2069.2% of algo (FFI overhead may distort results)
