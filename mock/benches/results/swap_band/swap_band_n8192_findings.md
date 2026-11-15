# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 18% faster than the next best (sb_plan)

sb_none (12.92 us) leads sb_plan (15.24 us) by 18%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 196.7x slower than the field

sb_value (2.54 ms) is 196.7x the fastest (12.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sb_none is fastest but the noisiest (CV 5.1%)

sb_none wins on median (12.92 us) yet has the highest variance (CV 5.1%), while sb_value is the steadiest (CV 1.3%, 2.54 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (12.92 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 196.7x the fastest

Fastest sb_none (12.92 us) to slowest sb_value (2.54 ms): 196.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 12920.8 ns median
- 2 variants significantly slower than baseline
- Spread: 196.70x (fastest 12920.8 ns, slowest 2541525.0 ns)

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
| sb_none | 19281ns | 19231ns | 17421ns | 18775ns | 20969ns | base |
| sb_plan | 21460ns | 21262ns | 19828ns | 21055ns | 22883ns | +11.30% |
| sb_value | 2549407ns | 2548475ns | 2509732ns | 2536510ns | 2588589ns | +13122.52% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 12975ns | 12071ns | 13734ns | base | 0.631 |
| sb_plan | 15184ns | 14443ns | 15760ns | +17.03% | 0.539 |
| sb_value | 2542734ns | 2503185ns | 2582569ns | +19496.83% | 0.003 |

## Performance model

- Peak throughput: **0.679 Gops/s** (sb_none; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.634 | 93.4% |
| sb_plan | 0.538 | 79.2% |
| sb_value | 0.003 | 0.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 19281ns | 19281ns | base |
| sb_plan | 21460ns | 21460ns | +11.30% |
| sb_value | 2549407ns | 2549407ns | +13122.52% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 12921ns | base | --- | [12519, 13390] | --- | --- | --- | --- |
| sb_plan | 15240ns | +2241.7ns (+17.3%) | [+1983, +2685]ns | [14812, 15342] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 2541525ns | +2527720.8ns (+19563.2%) | [+2501452, +2548238]ns | [2514031, 2560869] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 12467ns | +17.6% | +20622.5% |
| 1 | 2 | 12500ns | +19.7% | +20012.5% |
| 1 | 3 | 12796ns | +23.2% | +19551.9% |
| 2 | 1 | 14129ns | +7.2% | +17861.8% |
| 2 | 2 | 13492ns | +17.2% | +18797.4% |
| 2 | 3 | 13067ns | +17.3% | +19227.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | 0.316 | moderate+ |
| sb_plan | -0.065 | ok |
| sb_value | -0.199 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 44891354.2ns | 12975.2ns | 345977.2% | HIGH |
| sb_plan | 45039674.1ns | 15184.5ns | 296616.2% | HIGH |
| sb_value | 44891168.5ns | 2542734.2ns | 1765.5% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 12070.8-13734.1 ns)
  12070.8 |####################
  12154.0 |
  12237.2 |
  12320.3 |####################
  12403.5 |####################
  12486.6 |########################################
  12569.8 |
  12653.0 |####################
  12736.1 |####################
  12819.3 |####################
  12902.4 |
  12985.6 |########################################
  13068.8 |####################
  13151.9 |
  13235.1 |####################
  13318.2 |
  13401.4 |####################
  13484.6 |####################
  13567.7 |
  13650.9 |
  (1 below, 3 above range)

sb_plan (n=18, range 14443.1-15759.8 ns)
  14443.1 |#############
  14508.9 |
  14574.7 |#############
  14640.6 |#############
  14706.4 |#############
  14772.2 |
  14838.1 |#############
  14903.9 |#############
  14969.7 |
  15035.6 |
  15101.4 |#############
  15167.2 |
  15233.1 |########################################
  15298.9 |########################################
  15364.7 |
  15430.6 |
  15496.4 |
  15562.2 |
  15628.1 |
  15693.9 |
  (1 below, 4 above range)

sb_value (n=18, range 2503184.7-2582568.8 ns)
  2503184.7 |
  2507153.9 |##########
  2511123.1 |########################################
  2515092.3 |
  2519061.5 |
  2523030.7 |##########
  2526999.9 |
  2530969.1 |
  2534938.3 |##########
  2538907.5 |
  2542876.7 |##########
  2546845.9 |##########
  2550815.1 |##########
  2554784.3 |##########
  2558753.5 |
  2562722.7 |##########
  2566691.9 |
  2570661.1 |
  2574630.3 |
  2578599.5 |
  (2 below, 4 above range)

```

## Diagnostics

- **sb_none**: bridge=346404.8% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=293753.2% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1762.3% of algo (FFI overhead may distort results)
