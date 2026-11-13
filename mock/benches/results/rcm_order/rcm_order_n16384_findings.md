# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Key findings

- **Fastest: rcm_half** at 36860900.0 ns median (-1.0% vs baseline)
- Spread: 1.07x (fastest 36860900.0 ns, slowest 39580077.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 42181657ns | 37232356ns | 35670368ns | 37157800ns | 52973087ns | base |
| rcm_half | 41888242ns | 36867575ns | 35915778ns | 36791300ns | 52519887ns | -0.70% |
| rcm_rev | 73458217ns | 39587123ns | 36230722ns | 39700472ns | 142708583ns | +74.15% |
| rcm_scr | 45453350ns | 37717958ns | 36220486ns | 37562272ns | 61906399ns | +7.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 42173459ns | 35663537ns | 52963692ns | base | 0.000 |
| rcm_half | 41880440ns | 35909078ns | 52510688ns | -0.69% | 0.000 |
| rcm_rev | 73443577ns | 36222528ns | 142678897ns | +74.15% | 0.000 |
| rcm_scr | 45443608ns | 36212765ns | 61892742ns | +7.75% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 95.8% |
| rcm_half | 0.000 | 96.8% |
| rcm_rev | 0.000 | 90.1% |
| rcm_scr | 0.000 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 42181657ns | 42181657ns | base |
| rcm_half | 41888242ns | 41888242ns | -0.70% |
| rcm_rev | 73458217ns | 73458217ns | +74.15% |
| rcm_scr | 45453350ns | 45453350ns | +7.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 37223942ns | base | --- | [36326579, 44220267] | --- | --- | --- | --- |
| rcm_half | 36860900ns | no significant difference | [-7396256, +2052927]ns | [36346938, 38527252] | no | 1.0000 | 0.8145 | 0 |
| rcm_rev | 39580077ns | no significant difference | [-291548, +13446933]ns | [36987888, 53562381] | no | 0.7137 | 0.2379 | 0 |
| rcm_scr | 37710944ns | no significant difference | [-3217562, +1453388]ns | [37188498, 39299415] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|
| 1 | 43163025ns | -16.4% | -11.1% | -14.7% |
| 2 | 35496383ns | +4.5% | +0.4% | +0.4% |
| 3 | 36313042ns | +13.4% | +58.2% | -0.3% |
| 4 | 36904367ns | -1.8% | -1.5% | +4.9% |
| 5 | 45277508ns | -21.1% | -18.9% | -16.9% |
| 6 | 36710104ns | -2.1% | -0.0% | +2.2% |
| 7 | 60629812ns | -39.6% | -14.3% | -38.6% |
| 8 | 39860904ns | -8.5% | -7.0% | -3.4% |
| 9 | 35957596ns | +6.8% | +2.9% | +10.9% |
| 10 | 36156312ns | +4.6% | +5.3% | +2.0% |
| 11 | 37543517ns | -2.6% | +35.8% | -1.4% |
| 12 | 35536633ns | +86.1% | +4.0% | +5.8% |
| 13 | 66978967ns | -43.6% | +212.8% | +131.4% |
| 14 | 55369879ns | -34.8% | +390.4% | -27.6% |
| 15 | 46362958ns | -16.6% | +354.0% | -10.9% |
| 16 | 37815629ns | +20.6% | +45.9% | -0.1% |
| 17 | 36719050ns | +131.6% | +11.0% | +3.0% |
| 18 | 36326579ns | +0.1% | +13.6% | +55.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.166 | ok |
| rcm_half | -0.019 | ok |
| rcm_rev | 0.644 | HIGH+ (drift/warm-up) |
| rcm_scr | -0.055 | ok |

**Consistency summary:**

- **rcm_half**: won 10/18, lost 7/18
- **rcm_rev**: won 5/18, lost 12/18
- **rcm_scr**: won 8/18, lost 9/18

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 733868650.2ns | 42173459.3ns | 1740.1% | HIGH |
| rcm_half | 752390560.0ns | 41880440.1ns | 1796.5% | HIGH |
| rcm_rev | 831999623.8ns | 73443576.8ns | 1132.8% | HIGH |
| rcm_scr | 741101214.3ns | 45443608.3ns | 1630.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 35663537.5-52963691.7 ns)
  35663537.5 |########################################
  36528545.2 |##############################
  37393552.9 |####################
  38258560.6 |
  39123568.3 |##########
  39988576.0 |
  40853583.7 |
  41718591.4 |
  42583599.1 |##########
  43448606.9 |
  44313614.6 |
  45178622.3 |##########
  46043630.0 |##########
  46908637.7 |
  47773645.4 |
  48638653.1 |
  49503660.8 |
  50368668.5 |
  51233676.2 |
  52098684.0 |
  (2 below, 3 above range)

rcm_half (n=18, range 35909077.8-52510687.5 ns)
  35909077.8 |########################################
  36739158.3 |#####
  37569238.7 |##########
  38399319.2 |##########
  39229399.7 |
  40059480.2 |
  40889560.7 |#####
  41719641.2 |
  42549721.7 |
  43379802.1 |
  44209882.6 |
  45039963.1 |#####
  45870043.6 |
  46700124.1 |
  47530204.6 |
  48360285.1 |
  49190365.6 |
  50020446.0 |
  50850526.5 |
  51680607.0 |
  (1 below, 2 above range)

rcm_rev (n=18, range 36222527.8-142678897.2 ns)
  36222527.8 |########################################
  41545346.2 |
  46868164.7 |########
  52190983.2 |########
  57513801.7 |
  62836620.1 |
  68159438.6 |
  73482257.1 |
  78805075.5 |
  84127894.0 |
  89450712.5 |
  94773531.0 |
  100096349.4 |
  105419167.9 |
  110741986.4 |
  116064804.9 |
  121387623.3 |
  126710441.8 |
  132033260.3 |
  137356078.7 |
  (1 below, 3 above range)

rcm_scr (n=18, range 36212765.3-61892741.7 ns)
  36212765.3 |######################
  37496764.1 |########################################
  38780762.9 |#####
  40064761.7 |###########
  41348760.5 |
  42632759.4 |
  43916758.2 |
  45200757.0 |
  46484755.8 |
  47768754.6 |
  49052753.5 |
  50336752.3 |
  51620751.1 |
  52904749.9 |
  54188748.7 |
  55472747.6 |#####
  56756746.4 |
  58040745.2 |
  59324744.0 |
  60608742.8 |
  (2 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: CV=21.8% (high variance, measurements may be unstable)
- **rcm_adj**: bridge=1954.7% of algo (FFI overhead may distort results)
- **rcm_half**: CV=30.0% (high variance, measurements may be unstable)
- **rcm_half**: bridge=1978.2% of algo (FFI overhead may distort results)
- **rcm_rev**: CV=97.4% (high variance, measurements may be unstable)
- **rcm_rev**: worst_20/best_20 = 3.9x (possible bimodal distribution)
- **rcm_rev**: autocorrelation=0.64 (measurement drift or warm-up artifact)
- **rcm_rev**: bridge=1850.0% of algo (FFI overhead may distort results)
- **rcm_scr**: CV=59.3% (high variance, measurements may be unstable)
- **rcm_scr**: bridge=1941.8% of algo (FFI overhead may distort results)
