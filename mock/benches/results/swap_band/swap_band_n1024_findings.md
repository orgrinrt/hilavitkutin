# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Key findings

- **Baseline (sb_none) is the fastest** at 9329.2 ns median
- 2 variants significantly slower than baseline
- Spread: 18.45x (fastest 9329.2 ns, slowest 172106.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 13437ns | 13235ns | 12318ns | 13100ns | 14501ns | base |
| sb_plan | 17171ns | 17110ns | 15658ns | 16959ns | 18245ns | +27.79% |
| sb_value | 181922ns | 176485ns | 165979ns | 174659ns | 200787ns | +1253.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 9395ns | 8869ns | 9838ns | base | 0.109 |
| sb_plan | 12886ns | 12471ns | 13216ns | +37.16% | 0.079 |
| sb_value | 177812ns | 162211ns | 196317ns | +1792.64% | 0.006 |

## Performance model

- Peak throughput: **0.115 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.110 | 95.1% |
| sb_plan | 0.080 | 69.0% |
| sb_value | 0.006 | 5.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 13437ns | 13437ns | base |
| sb_plan | 17171ns | 17171ns | +27.79% |
| sb_value | 181922ns | 181922ns | +1253.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 9329ns | base | --- | [9171, 9515] | --- | --- | --- | --- |
| sb_plan | 12848ns | +3412.5ns (+36.6%) | [+3302, +3833]ns | [12696, 13112] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 172106ns | +162987.5ns (+1747.1%) | [+159140, +169546]ns | [169073, 180540] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|
| 1 | 9921ns | +28.7% | +1598.3% |
| 2 | 9221ns | +36.7% | +2024.5% |
| 3 | 9279ns | +44.7% | +1724.6% |
| 4 | 9946ns | +28.9% | +1605.8% |
| 5 | 9421ns | +32.0% | +1755.1% |
| 6 | 9171ns | +44.0% | +1782.0% |
| 7 | 9150ns | +37.2% | +1659.0% |
| 8 | 9067ns | +42.0% | +1792.9% |
| 9 | 9504ns | +36.0% | +1815.4% |
| 10 | 9867ns | +34.3% | +1752.2% |
| 11 | 9492ns | +37.4% | +1786.3% |
| 12 | 8838ns | +49.3% | +1854.9% |
| 13 | 8704ns | +47.2% | +2493.9% |
| 14 | 9379ns | +35.4% | +1662.5% |
| 15 | 10262ns | +23.6% | +1969.6% |
| 16 | 9225ns | +41.8% | +1638.5% |
| 17 | 9525ns | +38.0% | +1698.1% |
| 18 | 9138ns | +36.0% | +1712.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | 0.071 | ok |
| sb_plan | -0.227 | moderate- |
| sb_value | -0.333 | moderate- |

**Consistency summary:**

- **sb_plan**: won 0/18, lost 18/18
- **sb_value**: won 0/18, lost 18/18

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 5694772.0ns | 9394.9ns | 60615.5% | HIGH |
| sb_plan | 5702288.4ns | 12886.1ns | 44251.4% | HIGH |
| sb_value | 5709885.2ns | 177812.0ns | 3211.2% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 8869.5-9837.5 ns)
   8869.5 |
   8917.9 |
   8966.3 |
   9014.7 |
   9063.1 |####################
   9111.5 |########################################
   9159.9 |####################
   9208.3 |########################################
   9256.7 |####################
   9305.1 |
   9353.5 |####################
   9401.9 |####################
   9450.3 |####################
   9498.7 |########################################
   9547.1 |
   9595.5 |
   9643.9 |
   9692.3 |
   9740.7 |
   9789.1 |
  (2 below, 4 above range)

sb_plan (n=18, range 12470.8-13216.0 ns)
  12470.8 |
  12508.1 |
  12545.3 |####################
  12582.6 |####################
  12619.9 |
  12657.1 |####################
  12694.4 |####################
  12731.6 |
  12768.9 |####################
  12806.2 |########################################
  12843.4 |####################
  12880.7 |
  12917.9 |####################
  12955.2 |
  12992.4 |
  13029.7 |####################
  13067.0 |####################
  13104.2 |
  13141.5 |####################
  13178.7 |########################################
  (2 below, 2 above range)

sb_value (n=18, range 162211.1-196317.4 ns)
  162211.1 |
  163916.4 |########################################
  165621.7 |
  167327.0 |####################
  169032.4 |########################################
  170737.7 |########################################
  172443.0 |########################################
  174148.3 |####################
  175853.6 |
  177558.9 |####################
  179264.2 |
  180969.5 |####################
  182674.9 |####################
  184380.2 |
  186085.5 |
  187790.8 |
  189496.1 |
  191201.4 |
  192906.7 |
  194612.1 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=60993.0% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=44383.4% of algo (FFI overhead may distort results)
- **sb_value**: bridge=3313.3% of algo (FFI overhead may distort results)
