# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.2% (fastest 9.50 ms to slowest 9.62 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (116.38 us) is smaller than the fastest variant's own run-to-run std-dev (348.94 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader rcm_scr vs stability leader rcm_rev (+1% speed for 4.1x steadier)

rcm_scr is fastest (9.50 ms, CV 3.7%); rcm_rev gives up 1.2% median for 4.1x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_scr** at 9499898.3 ns median (-0.7% vs baseline)
- Spread: 1.01x (fastest 9499898.3 ns, slowest 9616278.9 ns)

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
| Counter frequency | 2112746224 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 9667508ns | 9570535ns | 9410988ns | 9535092ns | 9994391ns | base |
| rcm_half | 9592691ns | 9618308ns | 9405068ns | 9549743ns | 9750924ns | -0.77% |
| rcm_rev | 9601785ns | 9620256ns | 9479638ns | 9584499ns | 9688789ns | -0.68% |
| rcm_scr | 9714725ns | 9503718ns | 9418159ns | 9495581ns | 10191724ns | +0.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 9663579ns | 9407161ns | 9990332ns | base | 0.000 |
| rcm_half | 9588753ns | 9401213ns | 9746927ns | -0.77% | 0.000 |
| rcm_rev | 9597765ns | 9475423ns | 9684800ns | -0.68% | 0.000 |
| rcm_scr | 9710892ns | 9414350ns | 10187825ns | +0.49% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.3% |
| rcm_half | 0.000 | 97.8% |
| rcm_rev | 0.000 | 97.8% |
| rcm_scr | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 9667508ns | 9667508ns | base |
| rcm_half | 9592691ns | 9592691ns | -0.77% |
| rcm_rev | 9601785ns | 9601785ns | -0.68% |
| rcm_scr | 9714725ns | 9714725ns | +0.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 9566648ns | base | --- | [9476119, 9708879] | --- | --- | --- | --- |
| rcm_half | 9614393ns | no significant difference | [-246660, +228980]ns | [9469160, 9698583] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 9616279ns | no significant difference | [-61345, +62011]ns | [9514476, 9649877] | no | 0.8145 | 0.4807 | 0 |
| rcm_scr | 9499898ns | no significant difference | [-64142, +350144]ns | [9433521, 10069540] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 9593207ns | +0.4% | +0.7% | +0.5% |
| 1 | 2 | 9527871ns | +2.6% | +1.1% | +7.4% |
| 1 | 3 | 9721880ns | -2.8% | -1.1% | +4.9% |
| 2 | 1 | 10325138ns | -8.3% | -6.7% | -8.5% |
| 2 | 2 | 9411634ns | +3.2% | +0.7% | +0.2% |
| 2 | 3 | 9450337ns | -0.7% | +0.4% | -0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.132 | ok |
| rcm_half | -0.340 | moderate- |
| rcm_rev | 0.381 | moderate+ |
| rcm_scr | 0.319 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 61501761.6ns | 9663579.3ns | 636.4% | HIGH |
| rcm_half | 61326910.0ns | 9588753.1ns | 639.6% | HIGH |
| rcm_rev | 61276776.8ns | 9597764.7ns | 638.4% | HIGH |
| rcm_scr | 61577287.8ns | 9710891.7ns | 634.1% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 9407160.9-9990332.0 ns)
  9407160.9 |##########################
  9436319.5 |##########################
  9465478.0 |
  9494636.6 |##########################
  9523795.1 |##########################
  9552953.7 |
  9582112.2 |########################################
  9611270.8 |
  9640429.3 |
  9669587.9 |#############
  9698746.4 |#############
  9727905.0 |#############
  9757063.5 |
  9786222.1 |
  9815380.7 |
  9844539.2 |
  9873697.8 |
  9902856.3 |
  9932014.9 |
  9961173.4 |
  (1 below, 3 above range)

rcm_half (n=18, range 9401213.0-9746927.0 ns)
  9401213.0 |
  9418498.7 |
  9435784.4 |########################################
  9453070.1 |########################################
  9470355.8 |
  9487641.5 |####################
  9504927.2 |
  9522212.9 |####################
  9539498.6 |
  9556784.3 |
  9574070.0 |
  9591355.7 |####################
  9608641.4 |
  9625927.1 |####################
  9643212.8 |########################################
  9660498.5 |
  9677784.2 |####################
  9695069.9 |
  9712355.6 |####################
  9729641.3 |####################
  (2 below, 3 above range)

rcm_rev (n=18, range 9475422.6-9684800.3 ns)
  9475422.6 |#############
  9485891.5 |#############
  9496360.3 |#############
  9506829.2 |
  9517298.1 |
  9527767.0 |#############
  9538235.9 |
  9548704.8 |
  9559173.7 |
  9569642.6 |
  9580111.5 |
  9590580.3 |#############
  9601049.2 |
  9611518.1 |########################################
  9621987.0 |##########################
  9632455.9 |
  9642924.8 |#############
  9653393.7 |##########################
  9663862.6 |#############
  9674331.4 |#############
  (2 below, 1 above range)

rcm_scr (n=18, range 9414349.5-10187825.2 ns)
  9414349.5 |########################################
  9453023.3 |######
  9491697.1 |
  9530370.9 |######
  9569044.7 |######
  9607718.5 |######
  9646392.2 |
  9685066.0 |
  9723739.8 |
  9762413.6 |
  9801087.4 |
  9839761.2 |
  9878434.9 |
  9917108.7 |######
  9955782.5 |
  9994456.3 |
  10033130.1 |
  10071803.9 |
  10110477.6 |
  10149151.4 |
  (2 below, 5 above range)

```

## Diagnostics

- **rcm_adj**: bridge=640.4% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=637.6% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=637.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=645.3% of algo (FFI overhead may distort results)
