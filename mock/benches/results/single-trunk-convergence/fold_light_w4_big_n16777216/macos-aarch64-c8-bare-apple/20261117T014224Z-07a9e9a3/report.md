# fold_light, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (1.10 ms) and kernel_nway (1.10 ms) differ by 0.37%, inside the noise, even though the wider field spreads 81.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.10 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, std_parallel, nway, sequential, kernel_seq} vs {head_tail_dyn} (37% apart)

The field splits into a fast tier {head_tail, kernel_nway, std_parallel, nway, sequential, kernel_seq} and a slow tier {head_tail_dyn} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader head_tail vs stability leader kernel_nway (+0% speed for 11.5x steadier)

head_tail is fastest (1.10 ms, CV 3.7%); kernel_nway gives up 0.4% median for 11.5x lower variance (CV 0.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (head_tail) is the fastest** at 1098635.4 ns median
- 4 variants significantly slower than baseline
- Spread: 1.81x (fastest 1098635.4 ns, slowest 1991838.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 1119189ns | 1101583ns | 1094219ns | 1101111ns | 1158792ns | base |
| head_tail_dyn | 1990656ns | 1994833ns | 1948521ns | 1984986ns | 2020229ns | +77.87% |
| kernel_nway | 1107750ns | 1106943ns | 1103583ns | 1106524ns | 1111672ns | -1.02% |
| kernel_seq | 1450915ns | 1454464ns | 1437062ns | 1448681ns | 1461193ns | +29.64% |
| nway | 1266135ns | 1290500ns | 1124354ns | 1237979ns | 1379260ns | +13.13% |
| sequential | 1453311ns | 1453464ns | 1438448ns | 1450694ns | 1464667ns | +29.85% |
| std_parallel | 1251582ns | 1164406ns | 1146479ns | 1163747ns | 1435885ns | +11.83% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1115260ns | 1090885ns | 1153380ns | base | 15.043 |
| head_tail_dyn | 1987307ns | 1945219ns | 2015500ns | +78.19% | 8.442 |
| kernel_nway | 1103396ns | 1098417ns | 1107531ns | -1.06% | 15.205 |
| kernel_seq | 1449095ns | 1435240ns | 1459083ns | +29.93% | 11.578 |
| nway | 1262458ns | 1120625ns | 1375630ns | +13.20% | 13.289 |
| sequential | 1451510ns | 1436458ns | 1463193ns | +30.15% | 11.558 |
| std_parallel | 1249319ns | 1144844ns | 1432797ns | +12.02% | 13.429 |

## Performance model

- Peak throughput: **15.379 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 15.271 | 99.3% |
| head_tail_dyn | 8.423 | 54.8% |
| kernel_nway | 15.215 | 98.9% |
| kernel_seq | 11.548 | 75.1% |
| nway | 13.035 | 84.8% |
| sequential | 11.554 | 75.1% |
| std_parallel | 14.434 | 93.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1119189ns | 1119189ns | base |
| head_tail_dyn | 1990656ns | 1990656ns | +77.87% |
| kernel_nway | 1107750ns | 1107750ns | -1.02% |
| kernel_seq | 1450915ns | 1450915ns | +29.64% |
| nway | 1266135ns | 1266135ns | +13.13% |
| sequential | 1453311ns | 1453311ns | +29.85% |
| std_parallel | 1251582ns | 1251582ns | +11.83% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1098635ns | base | --- | [1093766, 1153380] | --- | --- | --- | --- |
| head_tail_dyn | 1991839ns | +873895.8ns (+79.5%) | [+830286, +911958]ns | [1954583, 2015500] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 1102672ns | no significant difference | [-50708, +9292]ns | [1099984, 1107531] | no | 0.2188 | 0.2188 | 0 |
| kernel_seq | 1452880ns | +356343.8ns (+32.4%) | [+283203, +361958]ns | [1435323, 1459083] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 1287073ns | no significant difference | [-28646, +278531]ns | [1124672, 1375630] | no | 0.2188 | 0.2188 | 0 |
| sequential | 1452052ns | +356390.7ns (+32.4%) | [+287167, +365193]ns | [1439286, 1463193] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 1162349ns | +63135.4ns (+5.7%) | [+4547, +334495]ns | [1152812, 1432797] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1098302ns | +83.0% | +0.5% | +32.7% | +25.3% | +33.2% | +30.5% |
| 1 | 2 | 1153318ns | +72.7% | -4.1% | +25.4% | -2.5% | +25.4% | +0.7% |
| 1 | 3 | 1094161ns | +79.1% | +0.5% | +32.0% | +17.6% | +32.1% | +5.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.664 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.015 | ok |
| kernel_nway | -0.423 | moderate- |
| kernel_seq | -0.105 | ok |
| nway | -0.586 | HIGH- (thermal bounce) |
| sequential | -0.129 | ok |
| std_parallel | -0.153 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1121935.8ns | 1115260.4ns | 100.6% | HIGH |
| head_tail_dyn | 1952942.7ns | 1987307.3ns | 98.3% | HIGH |
| kernel_nway | 1104546.9ns | 1103395.8ns | 100.1% | HIGH |
| kernel_seq | 1446958.3ns | 1449095.5ns | 99.9% | HIGH |
| nway | 1242302.1ns | 1262458.3ns | 98.4% | HIGH |
| sequential | 1455692.7ns | 1451510.4ns | 100.3% | HIGH |
| std_parallel | 1254055.6ns | 1249319.4ns | 100.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1090885.4-1153380.2 ns)
  1090885.4 |#############
  1094010.1 |#############
  1097134.9 |########################################
  1100259.6 |
  1103384.4 |
  1106509.1 |
  1109633.8 |
  1112758.6 |
  1115883.3 |
  1119008.1 |
  1122132.8 |
  1125257.5 |
  1128382.3 |
  1131507.0 |
  1134631.8 |
  1137756.5 |
  1140881.2 |
  1144006.0 |
  1147130.7 |
  1150255.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1945218.8-2015500.0 ns)
  1945218.8 |####################
  1948732.9 |
  1952246.9 |
  1955761.0 |
  1959275.0 |
  1962789.1 |####################
  1966303.2 |
  1969817.2 |
  1973331.3 |####################
  1976845.3 |
  1980359.4 |
  1983873.5 |
  1987387.5 |
  1990901.6 |
  1994415.6 |
  1997929.7 |
  2001443.8 |
  2004957.8 |
  2008471.9 |########################################
  2011985.9 |
  (0 below, 1 above range)

kernel_nway (n=6, range 1098416.7-1107531.2 ns)
  1098416.7 |####################
  1098872.4 |
  1099328.1 |
  1099783.9 |
  1100239.6 |
  1100695.3 |
  1101151.1 |####################
  1101606.8 |
  1102062.5 |
  1102518.2 |########################################
  1102973.9 |
  1103429.7 |
  1103885.4 |
  1104341.1 |
  1104796.8 |####################
  1105252.6 |
  1105708.3 |
  1106164.0 |
  1106619.8 |
  1107075.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1435239.6-1459083.4 ns)
  1435239.6 |########################################
  1436431.8 |
  1437624.0 |
  1438816.2 |
  1440008.4 |
  1441200.5 |
  1442392.7 |
  1443584.9 |
  1444777.1 |
  1445969.3 |
  1447161.5 |
  1448353.7 |
  1449545.9 |
  1450738.0 |
  1451930.2 |########################################
  1453122.4 |
  1454314.6 |
  1455506.8 |####################
  1456699.0 |
  1457891.2 |
  (0 below, 1 above range)

nway (n=6, range 1120625.0-1375630.2 ns)
  1120625.0 |########################################
  1133375.3 |
  1146125.5 |
  1158875.8 |
  1171626.0 |
  1184376.3 |
  1197126.6 |
  1209876.8 |
  1222627.1 |
  1235377.3 |
  1248127.6 |
  1260877.9 |
  1273628.1 |####################
  1286378.4 |########################################
  1299128.6 |
  1311878.9 |
  1324629.2 |
  1337379.4 |
  1350129.7 |
  1362879.9 |
  (0 below, 1 above range)

sequential (n=6, range 1436458.3-1463192.8 ns)
  1436458.3 |########################################
  1437795.0 |
  1439131.7 |
  1440468.5 |
  1441805.2 |########################################
  1443141.9 |
  1444478.6 |
  1445815.4 |
  1447152.1 |
  1448488.8 |
  1449825.5 |########################################
  1451162.2 |
  1452499.0 |########################################
  1453835.7 |
  1455172.4 |
  1456509.1 |
  1457845.9 |
  1459182.6 |
  1460519.3 |
  1461856.0 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 1144843.8-1432796.9 ns)
  1144843.8 |#############
  1159241.5 |########################################
  1173639.1 |
  1188036.8 |
  1202434.4 |
  1216832.1 |
  1231229.7 |
  1245627.4 |
  1260025.0 |
  1274422.7 |
  1288820.4 |
  1303218.0 |
  1317615.7 |
  1332013.3 |
  1346411.0 |
  1360808.6 |
  1375206.3 |
  1389603.9 |
  1404001.6 |#############
  1418399.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.7% of algo (FFI overhead may distort results)
- **nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.9% of algo (FFI overhead may distort results)
