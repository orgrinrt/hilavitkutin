# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 19% faster than the next best (sb_plan)

sb_none (12.66 us) leads sb_plan (15.09 us) by 19%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 229.6x slower than the field

sb_value (2.91 ms) is 229.6x the fastest (12.66 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_none is fastest but the noisiest (CV 5.1%)

sb_none wins on median (12.66 us) yet has the highest variance (CV 5.1%), while sb_value is the steadiest (CV 3.7%, 2.91 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (12.66 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 229.6x the fastest

Fastest sb_none (12.66 us) to slowest sb_value (2.91 ms): 229.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 12664.6 ns median
- 2 variants significantly slower than baseline
- Spread: 229.60x (fastest 12664.6 ns, slowest 2907839.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | next-frame asymmetry: clean skip vs value-swap cone vs plan band |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000002 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 18816ns | 18819ns | 17338ns | 18571ns | 19922ns | base |
| sb_plan | 21984ns | 21202ns | 19494ns | 20928ns | 24813ns | +16.84% |
| sb_value | 2887974ns | 2914190ns | 2736257ns | 2861578ns | 3003425ns | +15248.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 12764ns | 11988ns | 13468ns | base | 0.642 |
| sb_plan | 15133ns | 14071ns | 15790ns | +18.56% | 0.541 |
| sb_value | 2881123ns | 2728993ns | 2996304ns | +22472.03% | 0.003 |

## Performance model

- Peak throughput: **0.683 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.647 | 94.7% |
| sb_plan | 0.543 | 79.4% |
| sb_value | 0.003 | 0.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 18816ns | 18816ns | base |
| sb_plan | 21984ns | 21984ns | +16.84% |
| sb_value | 2887974ns | 2887974ns | +15248.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 12665ns | base | --- | [12342, 12992] | --- | --- | --- | --- |
| sb_plan | 15094ns | +2393.8ns (+18.9%) | [+1900, +2742]ns | [14846, 15490] | YES | 0.0001 | 0.0001 | 0 |
| sb_value | 2907840ns | +2895070.9ns (+22859.6%) | [+2773842, +2933046]ns | [2779200, 2946219] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 12629ns | +17.9% | +21548.1% |
| 1 | 2 | 12888ns | +19.9% | +22661.7% |
| 1 | 3 | 12804ns | +20.9% | +22805.7% |
| 2 | 1 | 12375ns | +22.6% | +23710.1% |
| 2 | 2 | 12133ns | +17.0% | +22860.6% |
| 2 | 3 | 12917ns | +20.0% | +22707.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.035 | ok |
| sb_plan | -0.259 | moderate- |
| sb_value | -0.312 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 45528369.2ns | 12764.1ns | 356690.0% | HIGH |
| sb_plan | 44954062.7ns | 15132.9ns | 297062.4% | HIGH |
| sb_value | 44969600.7ns | 2881123.4ns | 1560.8% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 11987.5-13468.1 ns)
  11987.5 |####################
  12061.5 |####################
  12135.6 |
  12209.6 |####################
  12283.6 |####################
  12357.6 |####################
  12431.7 |####################
  12505.7 |
  12579.7 |####################
  12653.8 |########################################
  12727.8 |
  12801.8 |####################
  12875.8 |########################################
  12949.9 |
  13023.9 |####################
  13097.9 |
  13172.0 |
  13246.0 |
  13320.0 |####################
  13394.0 |########################################
  (1 below, 1 above range)

sb_plan (n=18, range 14070.8-15789.6 ns)
  14070.8 |
  14156.7 |####################
  14242.7 |
  14328.6 |
  14414.6 |
  14500.5 |
  14586.4 |
  14672.4 |##########
  14758.3 |##########
  14844.3 |####################
  14930.2 |##########
  15016.1 |##########
  15102.1 |##########
  15188.0 |
  15273.9 |
  15359.9 |##########
  15445.8 |########################################
  15531.8 |##########
  15617.7 |
  15703.6 |
  (1 below, 2 above range)

sb_value (n=18, range 2728993.1-2996304.2 ns)
  2728993.1 |########################################
  2742358.6 |
  2755724.2 |
  2769089.7 |#############
  2782455.3 |#############
  2795820.8 |
  2809186.4 |
  2822551.9 |
  2835917.5 |
  2849283.1 |#############
  2862648.6 |#############
  2876014.2 |
  2889379.7 |
  2902745.3 |##########################
  2916110.8 |
  2929476.4 |##########################
  2942841.9 |##########################
  2956207.5 |
  2969573.0 |
  2982938.6 |
  (1 below, 4 above range)

```

## Diagnostics

- **sb_none**: bridge=355561.5% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=297089.5% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1542.5% of algo (FFI overhead may distort results)
