# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (240.76 us) is smaller than the fastest variant's own run-to-run std-dev (585.88 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.6% of the fastest

All 4 variants sit between 37.54 ms and 37.79 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_scr** at 37544966.0 ns median (-0.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.01x (fastest 37544966.0 ns, slowest 37785728.5 ns)

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
| rcm_adj | 38077884ns | 37752806ns | 37615048ns | 37733413ns | 38826008ns | base |
| rcm_half | 37888208ns | 37789605ns | 37599759ns | 37766502ns | 38214990ns | -0.50% |
| rcm_rev | 37714515ns | 37629929ns | 37358991ns | 37607348ns | 38053029ns | -0.95% |
| rcm_scr | 37689601ns | 37548834ns | 37144437ns | 37544038ns | 38180528ns | -1.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 38073892ns | 37611223ns | 38821992ns | base | 0.000 |
| rcm_half | 37884217ns | 37595796ns | 38210912ns | -0.50% | 0.000 |
| rcm_rev | 37710458ns | 37354801ns | 38048970ns | -0.95% | 0.000 |
| rcm_scr | 37685382ns | 37140336ns | 38175926ns | -1.02% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.4% |
| rcm_half | 0.000 | 98.3% |
| rcm_rev | 0.000 | 98.7% |
| rcm_scr | 0.000 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 38077884ns | 38077884ns | base |
| rcm_half | 37888208ns | 37888208ns | -0.50% |
| rcm_rev | 37714515ns | 37714515ns | -0.95% |
| rcm_scr | 37689601ns | 37689601ns | -1.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 37748855ns | base | --- | [37708942, 37793142] | --- | --- | --- | --- |
| rcm_half | 37785728ns | no significant difference | [-96155, +267961]ns | [37706461, 37949595] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 37626010ns | no significant difference | [-273715, +165338]ns | [37531783, 37931096] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 37544966ns | -197901.0ns (-0.5%) | [-252990, -95767]ns | [37514036, 37611520] | YES | 0.0226 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 37612239ns | +0.3% | -0.2% | +0.5% |
| 1 | 2 | 37808523ns | -0.3% | -1.2% | -1.8% |
| 1 | 3 | 37772871ns | -0.2% | -0.5% | -0.6% |
| 2 | 1 | 37650365ns | +0.4% | +1.0% | -0.3% |
| 2 | 2 | 37721606ns | +1.1% | -0.2% | -0.3% |
| 2 | 3 | 37768231ns | +1.4% | +0.4% | -0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.298 | moderate- |
| rcm_half | 0.485 | moderate+ |
| rcm_rev | 0.043 | ok |
| rcm_scr | -0.492 | moderate- |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 4/6, lost 2/6
- **rcm_scr**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 247542425.4ns | 38073892.2ns | 650.2% | HIGH |
| rcm_half | 247297346.2ns | 37884217.2ns | 652.8% | HIGH |
| rcm_rev | 246970584.3ns | 37710458.2ns | 654.9% | HIGH |
| rcm_scr | 246226197.1ns | 37685382.4ns | 653.4% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 37611222.6-38821991.9 ns)
  37611222.6 |####################
  37671761.1 |####################
  37732299.6 |########################################
  37792838.0 |####################
  37853376.5 |######
  37913914.9 |
  37974453.4 |
  38034991.9 |
  38095530.3 |
  38156068.8 |
  38216607.3 |
  38277145.7 |
  38337684.2 |
  38398222.6 |
  38458761.1 |
  38519299.6 |
  38579838.0 |
  38640376.5 |
  38700915.0 |
  38761453.4 |
  (1 below, 1 above range)

rcm_half (n=18, range 37595795.5-38210911.9 ns)
  37595795.5 |#############
  37626551.4 |
  37657307.2 |#############
  37688063.0 |########################################
  37718818.8 |#############
  37749574.6 |#############
  37780330.5 |##########################
  37811086.3 |#############
  37841842.1 |##########################
  37872597.9 |
  37903353.7 |
  37934109.6 |
  37964865.4 |
  37995621.2 |
  38026377.0 |#############
  38057132.8 |
  38087888.7 |
  38118644.5 |#############
  38149400.3 |
  38180156.1 |
  (1 below, 3 above range)

rcm_rev (n=18, range 37354801.5-38048969.8 ns)
  37354801.5 |########################################
  37389509.9 |
  37424218.3 |
  37458926.7 |
  37493635.1 |########################################
  37528343.6 |####################
  37563052.0 |####################
  37597760.4 |########################################
  37632468.8 |####################
  37667177.2 |####################
  37701885.6 |
  37736594.1 |####################
  37771302.5 |
  37806010.9 |
  37840719.3 |
  37875427.7 |
  37910136.1 |####################
  37944844.6 |####################
  37979553.0 |####################
  38014261.4 |####################
  (1 below, 2 above range)

rcm_scr (n=18, range 37140335.5-38175926.1 ns)
  37140335.5 |######
  37192115.1 |
  37243894.6 |
  37295674.1 |
  37347453.7 |
  37399233.2 |
  37451012.7 |######
  37502792.2 |########################################
  37554571.8 |
  37606351.3 |##########################
  37658130.8 |######
  37709910.4 |
  37761689.9 |######
  37813469.4 |
  37865249.0 |
  37917028.5 |
  37968808.0 |
  38020587.6 |
  38072367.1 |
  38124146.6 |
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=651.5% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=654.6% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=655.0% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=655.2% of algo (FFI overhead may distort results)
