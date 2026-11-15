# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (227.84 us) is smaller than the fastest variant's own run-to-run std-dev (241.30 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 1.2% of the fastest

All 4 variants sit between 18.79 ms and 19.01 ms - a 1.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: rcm_scr** at 18786023.8 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 18786023.8 ns, slowest 19013867.9 ns)

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
| rcm_adj | 19007076ns | 18925732ns | 18779018ns | 18909900ns | 19266869ns | base |
| rcm_half | 18841449ns | 18835005ns | 18739176ns | 18811502ns | 18937507ns | -0.87% |
| rcm_rev | 19235982ns | 19017871ns | 18754765ns | 18985099ns | 19852916ns | +1.20% |
| rcm_scr | 18870656ns | 18789852ns | 18701589ns | 18777463ns | 19094977ns | -0.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 19003228ns | 18775130ns | 19263024ns | base | 0.000 |
| rcm_half | 18837555ns | 18735197ns | 18933577ns | -0.87% | 0.000 |
| rcm_rev | 19231957ns | 18750953ns | 19848696ns | +1.20% | 0.000 |
| rcm_scr | 18866795ns | 18697624ns | 19091138ns | -0.72% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.8% |
| rcm_half | 0.000 | 99.3% |
| rcm_rev | 0.000 | 98.3% |
| rcm_scr | 0.000 | 99.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 19007076ns | 19007076ns | base |
| rcm_half | 18841449ns | 18841449ns | -0.87% |
| rcm_rev | 19235982ns | 19235982ns | +1.20% |
| rcm_scr | 18870656ns | 18870656ns | -0.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 18921916ns | base | --- | [18882674, 18987108] | --- | --- | --- | --- |
| rcm_half | 18831108ns | -93401.2ns (-0.5%) | [-167913, -47248]ns | [18779342, 18848141] | YES | 0.0020 | 0.0013 | 0 |
| rcm_rev | 19013868ns | +97810.2ns (+0.5%) | [+51511, +288500]ns | [18939718, 19335787] | YES | 0.0020 | 0.0013 | 0 |
| rcm_scr | 18786024ns | no significant difference | [-222779, +2705]ns | [18721344, 18899334] | no | 0.2379 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 18856039ns | -0.6% | -0.9% | -0.8% |
| 1 | 2 | 18892256ns | -0.3% | +0.4% | -1.0% |
| 1 | 3 | 18989787ns | -0.7% | +3.4% | -1.3% |
| 2 | 1 | 18884157ns | -0.4% | +0.2% | +0.5% |
| 2 | 2 | 19109693ns | -1.2% | +1.9% | -1.2% |
| 2 | 3 | 18940303ns | -0.9% | +0.5% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.258 | moderate- |
| rcm_half | -0.290 | moderate- |
| rcm_rev | -0.344 | moderate- |
| rcm_scr | 0.351 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 6/6, lost 0/6
- **rcm_rev**: won 1/6, lost 5/6
- **rcm_scr**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 123472897.2ns | 19003227.7ns | 649.7% | HIGH |
| rcm_half | 122821647.9ns | 18837554.6ns | 652.0% | HIGH |
| rcm_rev | 125747664.0ns | 19231957.2ns | 653.8% | HIGH |
| rcm_scr | 122644592.6ns | 18866794.9ns | 650.1% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 18775129.6-19263024.5 ns)
  18775129.6 |
  18799524.3 |
  18823919.1 |
  18848313.8 |####################
  18872708.6 |########################################
  18897103.3 |##########
  18921498.1 |##########
  18945892.8 |####################
  18970287.5 |####################
  18994682.3 |
  19019077.0 |
  19043471.8 |
  19067866.5 |
  19092261.3 |##########
  19116656.0 |##########
  19141050.8 |
  19165445.5 |##########
  19189840.2 |
  19214235.0 |
  19238629.7 |
  (2 below, 1 above range)

rcm_half (n=18, range 18735197.5-18933576.8 ns)
  18735197.5 |##########################
  18745116.5 |
  18755035.4 |#############
  18764954.4 |#############
  18774873.4 |
  18784792.3 |##########################
  18794711.3 |
  18804630.2 |#############
  18814549.2 |
  18824468.2 |##########################
  18834387.1 |########################################
  18844306.1 |#############
  18854225.1 |#############
  18864144.0 |#############
  18874063.0 |
  18883982.0 |#############
  18893900.9 |
  18903819.9 |
  18913738.9 |
  18923657.8 |
  (1 below, 1 above range)

rcm_rev (n=18, range 18750952.6-19848696.2 ns)
  18750952.6 |
  18805839.8 |
  18860727.0 |##########################
  18915614.2 |########################################
  18970501.4 |########################################
  19025388.5 |##########################
  19080275.7 |
  19135162.9 |
  19190050.1 |
  19244937.3 |#############
  19299824.4 |
  19354711.6 |
  19409598.8 |
  19464486.0 |#############
  19519373.2 |
  19574260.3 |
  19629147.5 |#############
  19684034.7 |
  19738921.9 |
  19793809.1 |#############
  (2 below, 2 above range)

rcm_scr (n=18, range 18697623.7-19091138.5 ns)
  18697623.7 |########################################
  18717299.4 |##########################
  18736975.1 |##########################
  18756650.9 |#############
  18776326.6 |
  18796002.4 |#############
  18815678.1 |
  18835353.9 |
  18855029.6 |#############
  18874705.3 |##########################
  18894381.1 |#############
  18914056.8 |
  18933732.6 |
  18953408.3 |#############
  18973084.0 |#############
  18992759.8 |
  19012435.5 |
  19032111.3 |#############
  19051787.0 |
  19071462.7 |
  (1 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: bridge=648.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=651.2% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=645.6% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=652.2% of algo (FFI overhead may distort results)
