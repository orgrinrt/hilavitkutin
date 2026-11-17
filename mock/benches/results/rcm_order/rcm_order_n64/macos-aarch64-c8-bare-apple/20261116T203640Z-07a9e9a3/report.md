# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 80.39 us to slowest 80.75 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (367 ns) is smaller than the fastest variant's own run-to-run std-dev (1.12 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_rev** at 80385.4 ns median (-0.2% vs baseline)
- Spread: 1.00x (fastest 80385.4 ns, slowest 80752.1 ns)

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
| rcm_adj | 84576ns | 84660ns | 83622ns | 84448ns | 85246ns | base |
| rcm_half | 85515ns | 84469ns | 83528ns | 84270ns | 88375ns | +1.11% |
| rcm_rev | 84473ns | 84298ns | 83106ns | 84212ns | 85547ns | -0.12% |
| rcm_scr | 84274ns | 83981ns | 83107ns | 83865ns | 85470ns | -0.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 80822ns | 80182ns | 81592ns | base | 0.001 |
| rcm_half | 81843ns | 80353ns | 84413ns | +1.26% | 0.001 |
| rcm_rev | 80822ns | 80176ns | 81857ns | -0.00% | 0.001 |
| rcm_scr | 81137ns | 80456ns | 82133ns | +0.39% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 99.6% |
| rcm_half | 0.001 | 99.4% |
| rcm_rev | 0.001 | 99.7% |
| rcm_scr | 0.001 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 84576ns | 84576ns | base |
| rcm_half | 85515ns | 85515ns | +1.11% |
| rcm_rev | 84473ns | 84473ns | -0.12% |
| rcm_scr | 84274ns | 84274ns | -0.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 80525ns | base | --- | [80415, 81040] | --- | --- | --- | --- |
| rcm_half | 80667ns | no significant difference | [-254, +360]ns | [80521, 80962] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 80385ns | no significant difference | [-577, +146]ns | [80271, 80567] | no | 0.7137 | 0.2379 | 0 |
| rcm_scr | 80752ns | no significant difference | [-271, +354]ns | [80585, 80952] | no | 0.7210 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 81329ns | -0.6% | -1.3% | -0.2% |
| 1 | 2 | 80862ns | +9.5% | +2.3% | +0.1% |
| 1 | 3 | 80721ns | -0.2% | -0.5% | +0.1% |
| 2 | 1 | 80571ns | +0.1% | -0.5% | +0.5% |
| 2 | 2 | 80392ns | +0.1% | +0.2% | +0.2% |
| 2 | 3 | 80479ns | +0.4% | +0.1% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.374 | moderate+ |
| rcm_half | -0.215 | moderate- |
| rcm_rev | -0.315 | moderate- |
| rcm_scr | 0.132 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 3/6
- **rcm_rev**: won 3/6, lost 3/6
- **rcm_scr**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 2788872.5ns | 80822.5ns | 3450.6% | HIGH |
| rcm_half | 2801068.7ns | 81843.1ns | 3422.5% | HIGH |
| rcm_rev | 2792531.0ns | 80822.2ns | 3455.2% | HIGH |
| rcm_scr | 2789721.1ns | 81137.0ns | 3438.3% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 80181.9-81592.4 ns)
  80181.9 |#############
  80252.5 |#############
  80323.0 |#############
  80393.5 |#############
  80464.0 |########################################
  80534.5 |#############
  80605.1 |
  80675.6 |#############
  80746.1 |
  80816.6 |#############
  80887.1 |#############
  80957.7 |
  81028.2 |
  81098.7 |#############
  81169.2 |
  81239.8 |
  81310.3 |#############
  81380.8 |
  81451.3 |#############
  81521.8 |
  (2 below, 2 above range)

rcm_half (n=18, range 80352.8-84413.2 ns)
  80352.8 |########################################
  80555.8 |################################
  80758.8 |########################
  80961.8 |########
  81164.9 |########
  81367.9 |
  81570.9 |
  81773.9 |
  81976.9 |
  82180.0 |
  82383.0 |########
  82586.0 |
  82789.0 |
  82992.0 |
  83195.1 |
  83398.1 |
  83601.1 |
  83804.1 |
  84007.2 |
  84210.2 |
  (1 below, 2 above range)

rcm_rev (n=18, range 80176.4-81856.9 ns)
  80176.4 |##############################
  80260.4 |##############################
  80344.5 |########################################
  80428.5 |
  80512.5 |##############################
  80596.5 |##########
  80680.6 |
  80764.6 |
  80848.6 |
  80932.6 |
  81016.7 |
  81100.7 |
  81184.7 |
  81268.8 |
  81352.8 |
  81436.8 |
  81520.8 |
  81604.9 |
  81688.9 |
  81772.9 |
  (1 below, 3 above range)

rcm_scr (n=18, range 80455.5-82132.6 ns)
  80455.5 |########
  80539.4 |########################################
  80623.2 |
  80707.1 |################
  80791.0 |################
  80874.8 |################
  80958.7 |########
  81042.5 |
  81126.4 |########
  81210.2 |
  81294.1 |
  81377.9 |
  81461.8 |
  81545.6 |########
  81629.5 |
  81713.4 |
  81797.2 |
  81881.1 |
  81964.9 |
  82048.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3460.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3455.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3466.3% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3450.2% of algo (FFI overhead may distort results)
