# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.2% (fastest 8.51 ms to slowest 8.62 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_adj shows alternating (throttle bounce) (autocorr -0.58)

rcm_adj's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (100.98 us) is smaller than the fastest variant's own run-to-run std-dev (137.00 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (8.51 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 8514593.8 ns median
- Spread: 1.01x (fastest 8514593.8 ns, slowest 8615570.8 ns)

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
| rcm_adj | 8574643ns | 8523262ns | 8409075ns | 8519926ns | 8739501ns | base |
| rcm_half | 8617678ns | 8589427ns | 8448694ns | 8562163ns | 8785441ns | +0.50% |
| rcm_rev | 8632820ns | 8622125ns | 8370162ns | 8554455ns | 8881698ns | +0.68% |
| rcm_scr | 8656025ns | 8602510ns | 8388297ns | 8551119ns | 8947249ns | +0.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 8567264ns | 8402326ns | 8731749ns | base | 0.000 |
| rcm_half | 8610610ns | 8440701ns | 8778491ns | +0.51% | 0.000 |
| rcm_rev | 8625977ns | 8363283ns | 8874979ns | +0.69% | 0.000 |
| rcm_scr | 8648724ns | 8380838ns | 8939756ns | +0.95% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.2% |
| rcm_half | 0.000 | 97.4% |
| rcm_rev | 0.000 | 97.1% |
| rcm_scr | 0.000 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 8574643ns | 8574643ns | base |
| rcm_half | 8617678ns | 8617678ns | +0.50% |
| rcm_rev | 8632820ns | 8632820ns | +0.68% |
| rcm_scr | 8656025ns | 8656025ns | +0.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 8514594ns | base | --- | [8481115, 8635238] | --- | --- | --- | --- |
| rcm_half | 8582690ns | no significant difference | [-62427, +195829]ns | [8507762, 8711979] | no | 1.0000 | 0.8145 | 0 |
| rcm_rev | 8615571ns | no significant difference | [-53088, +190290]ns | [8467008, 8755315] | no | 1.0000 | 0.8145 | 0 |
| rcm_scr | 8595429ns | no significant difference | [-95996, +156833]ns | [8483892, 8690002] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 8502046ns | -0.4% | -0.9% | +0.3% |
| 1 | 2 | 8580171ns | -0.2% | -2.5% | -1.3% |
| 1 | 3 | 8494621ns | +3.1% | +2.4% | +2.3% |
| 2 | 1 | 8482004ns | +1.9% | -0.1% | +4.7% |
| 2 | 2 | 8687346ns | -1.0% | -0.5% | -3.5% |
| 2 | 3 | 8447038ns | +2.6% | +2.4% | +1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.581 | HIGH- (thermal bounce) |
| rcm_half | 0.072 | ok |
| rcm_rev | -0.148 | ok |
| rcm_scr | -0.223 | moderate- |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 3/6, lost 2/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 179340489.8ns | 8567263.7ns | 2093.3% | HIGH |
| rcm_half | 178872929.9ns | 8610609.9ns | 2077.4% | HIGH |
| rcm_rev | 179508352.5ns | 8625976.9ns | 2081.0% | HIGH |
| rcm_scr | 180827063.2ns | 8648723.8ns | 2090.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 8402326.4-8731749.3 ns)
  8402326.4 |#############
  8418797.5 |
  8435268.7 |#############
  8451739.8 |
  8468211.0 |########################################
  8484682.1 |##########################
  8501153.2 |#############
  8517624.4 |#############
  8534095.5 |
  8550566.7 |
  8567037.8 |##########################
  8583509.0 |
  8599980.1 |#############
  8616451.3 |
  8632922.4 |
  8649393.6 |
  8665864.7 |#############
  8682335.8 |#############
  8698807.0 |
  8715278.1 |
  (1 below, 3 above range)

rcm_half (n=18, range 8440701.4-8778491.0 ns)
  8440701.4 |
  8457590.8 |########################################
  8474480.3 |####################
  8491369.8 |
  8508259.3 |
  8525148.8 |########################################
  8542038.3 |####################
  8558927.7 |####################
  8575817.2 |
  8592706.7 |####################
  8609596.2 |####################
  8626485.7 |
  8643375.1 |####################
  8660264.6 |####################
  8677154.1 |
  8694043.6 |
  8710933.1 |
  8727822.5 |
  8744712.0 |####################
  8761601.5 |####################
  (2 below, 3 above range)

rcm_rev (n=18, range 8363283.3-8874979.2 ns)
  8363283.3 |####################
  8388868.1 |
  8414452.9 |########################################
  8440037.7 |####################
  8465622.5 |########################################
  8491207.3 |
  8516792.1 |
  8542376.9 |####################
  8567961.7 |####################
  8593546.4 |
  8619131.2 |########################################
  8644716.0 |####################
  8670300.8 |
  8695885.6 |####################
  8721470.4 |
  8747055.2 |
  8772640.0 |
  8798224.8 |####################
  8823809.6 |
  8849394.4 |
  (1 below, 4 above range)

rcm_scr (n=18, range 8380837.5-8939755.5 ns)
  8380837.5 |########################################
  8408783.4 |
  8436729.3 |
  8464675.2 |####################
  8492621.1 |####################
  8520567.0 |####################
  8548512.9 |####################
  8576458.8 |####################
  8604404.7 |########################################
  8632350.6 |####################
  8660296.5 |
  8688242.4 |########################################
  8716188.3 |
  8744134.2 |
  8772080.1 |
  8800026.0 |
  8827971.9 |
  8855917.8 |########################################
  8883863.7 |####################
  8911809.6 |
  (2 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2101.2% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2083.4% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2075.6% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2080.4% of algo (FFI overhead may distort results)
