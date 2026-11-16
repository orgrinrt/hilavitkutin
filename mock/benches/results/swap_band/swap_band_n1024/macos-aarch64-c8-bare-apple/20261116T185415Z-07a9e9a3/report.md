# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 22% faster than the next best (sb_plan)

sb_none (10.30 us) leads sb_plan (12.55 us) by 22%, a clear separation rather than a photo finish. CV 1.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 15.5x slower than the field

sb_value (160.02 us) is 15.5x the fastest (10.30 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (10.30 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.5x the fastest

Fastest sb_none (10.30 us) to slowest sb_value (160.02 us): 15.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 10302.0 ns median
- 2 variants significantly slower than baseline
- Spread: 15.53x (fastest 10302.0 ns, slowest 160018.8 ns)

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
| sb_none | 14075ns | 14056ns | 13356ns | 13931ns | 14651ns | base |
| sb_plan | 16240ns | 16250ns | 15636ns | 16085ns | 16774ns | +15.38% |
| sb_value | 164102ns | 164229ns | 162132ns | 163886ns | 165410ns | +1065.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10342ns | 10211ns | 10485ns | base | 0.099 |
| sb_plan | 12617ns | 12521ns | 12767ns | +22.00% | 0.081 |
| sb_value | 159873ns | 157512ns | 161291ns | +1445.88% | 0.006 |

## Performance model

- Peak throughput: **0.100 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.099 | 99.1% |
| sb_plan | 0.082 | 81.3% |
| sb_value | 0.006 | 6.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 14075ns | 14075ns | base |
| sb_plan | 16240ns | 16240ns | +15.38% |
| sb_value | 164102ns | 164102ns | +1065.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 10302ns | base | --- | [10275, 10367] | --- | --- | --- | --- |
| sb_plan | 12554ns | +2264.5ns (+22.0%) | [+2233, +2350]ns | [12535, 12590] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 160019ns | +149479.1ns (+1451.0%) | [+148904, +150383]ns | [159340, 160648] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10296ns | +22.1% | +1456.9% |
| 1 | 2 | 10496ns | +19.8% | +1417.0% |
| 1 | 3 | 10233ns | +23.1% | +1467.6% |
| 2 | 1 | 10312ns | +21.7% | +1462.7% |
| 2 | 2 | 10229ns | +22.4% | +1437.4% |
| 2 | 3 | 10338ns | +21.3% | +1444.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.416 | moderate- |
| sb_plan | 0.433 | moderate+ |
| sb_value | -0.341 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 5697934.5ns | 10341.9ns | 55095.6% | HIGH |
| sb_plan | 5696067.4ns | 12616.7ns | 45147.2% | HIGH |
| sb_value | 5696461.3ns | 159873.2ns | 3563.1% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 10211.1-10484.7 ns)
  10211.1 |#############
  10224.8 |##########################
  10238.5 |
  10252.2 |
  10265.9 |########################################
  10279.5 |
  10293.2 |##########################
  10306.9 |##########################
  10320.6 |
  10334.3 |#############
  10347.9 |#############
  10361.6 |
  10375.3 |#############
  10389.0 |
  10402.7 |
  10416.3 |#############
  10430.0 |
  10443.7 |
  10457.4 |
  10471.1 |
  (1 below, 3 above range)

sb_plan (n=18, range 12520.8-12766.7 ns)
  12520.8 |##############################
  12533.1 |########################################
  12545.4 |##########
  12557.7 |##########
  12570.0 |####################
  12582.3 |##########
  12594.6 |##########
  12606.9 |
  12619.2 |
  12631.5 |
  12643.8 |
  12656.0 |
  12668.3 |
  12680.6 |
  12692.9 |
  12705.2 |
  12717.5 |
  12729.8 |
  12742.1 |
  12754.4 |
  (1 below, 4 above range)

sb_value (n=18, range 157512.5-161291.0 ns)
  157512.5 |
  157701.4 |
  157890.3 |####################
  158079.3 |
  158268.2 |
  158457.1 |
  158646.0 |####################
  158835.0 |
  159023.9 |
  159212.8 |####################
  159401.7 |####################
  159590.7 |########################################
  159779.6 |####################
  159968.5 |####################
  160157.4 |########################################
  160346.4 |####################
  160535.3 |
  160724.2 |####################
  160913.1 |
  161102.1 |####################
  (2 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=55327.9% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=45385.4% of algo (FFI overhead may distort results)
- **sb_value**: bridge=3560.5% of algo (FFI overhead may distort results)
