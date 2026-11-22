# fold_light, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail is fastest but the noisiest (CV 10.3%)

head_tail wins on median (1.09 ms) yet has the highest variance (CV 10.3%), while kernel_nway is the steadiest (CV 0.3%, 1.10 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (head_tail, kernel_nway) are a dead heat (<1%)

head_tail (1.09 ms) and kernel_nway (1.10 ms) differ by 0.62%, inside the noise, even though the wider field spreads 81.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.09 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, nway, std_parallel, sequential, kernel_seq} vs {head_tail_dyn} (37% apart)

The field splits into a fast tier {head_tail, kernel_nway, nway, std_parallel, sequential, kernel_seq} and a slow tier {head_tail_dyn} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader head_tail vs stability leader kernel_nway (+1% speed for 36.0x steadier)

head_tail is fastest (1.09 ms, CV 10.3%); kernel_nway gives up 0.6% median for 36.0x lower variance (CV 0.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (head_tail) is the fastest** at 1091739.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.82x (fastest 1091739.6 ns, slowest 1984307.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 2 |
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
| head_tail | 1165698ns | 1095583ns | 1090469ns | 1094823ns | 1309625ns | base |
| head_tail_dyn | 1997906ns | 1987380ns | 1930229ns | 1984993ns | 2051115ns | +71.39% |
| kernel_nway | 1103010ns | 1102016ns | 1099198ns | 1101663ns | 1106938ns | -5.38% |
| kernel_seq | 1460587ns | 1449161ns | 1433521ns | 1447399ns | 1493901ns | +25.30% |
| nway | 1142417ns | 1119641ns | 1105385ns | 1116576ns | 1199693ns | -2.00% |
| sequential | 1486665ns | 1438406ns | 1428854ns | 1436517ns | 1590792ns | +27.53% |
| std_parallel | 1308193ns | 1316932ns | 1151281ns | 1299236ns | 1400083ns | +12.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1162106ns | 1087292ns | 1306135ns | base | 14.437 |
| head_tail_dyn | 1994686ns | 1927208ns | 2046964ns | +71.64% | 8.411 |
| kernel_nway | 1099781ns | 1096292ns | 1103859ns | -5.36% | 15.255 |
| kernel_seq | 1459014ns | 1432052ns | 1492234ns | +25.55% | 11.499 |
| nway | 1138363ns | 1102354ns | 1194990ns | -2.04% | 14.738 |
| sequential | 1484745ns | 1426823ns | 1588328ns | +27.76% | 11.300 |
| std_parallel | 1306240ns | 1149333ns | 1397906ns | +12.40% | 12.844 |

## Performance model

- Peak throughput: **15.430 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 15.367 | 99.6% |
| head_tail_dyn | 8.455 | 54.8% |
| kernel_nway | 15.273 | 99.0% |
| kernel_seq | 11.590 | 75.1% |
| nway | 15.048 | 97.5% |
| sequential | 11.676 | 75.7% |
| std_parallel | 12.755 | 82.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1165698ns | 1165698ns | base |
| head_tail_dyn | 1997906ns | 1997906ns | +71.39% |
| kernel_nway | 1103010ns | 1103010ns | -5.38% |
| kernel_seq | 1460587ns | 1460587ns | +25.30% |
| nway | 1142417ns | 1142417ns | -2.00% |
| sequential | 1486665ns | 1486665ns | +27.53% |
| std_parallel | 1308193ns | 1308193ns | +12.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1091740ns | base | --- | [1088443, 1306135] | --- | --- | --- | --- |
| head_tail_dyn | 1984307ns | +892026.0ns (+81.7%) | [+680839, +924875]ns | [1952786, 2046964] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_nway | 1098484ns | no significant difference | [-203823, +11583]ns | [1097000, 1103859] | no | 0.6875 | 0.6875 | 0 |
| kernel_seq | 1447573ns | +351760.4ns (+32.2%) | [+136328, +402635]ns | [1437234, 1492234] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| nway | 1114885ns | no significant difference | [-193797, +104167]ns | [1105214, 1194990] | no | 0.6875 | 0.6875 | 0 |
| sequential | 1436932ns | +339374.9ns (+31.1%) | [+130797, +497745]ns | [1428974, 1588328] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 1315391ns | no significant difference | [-33120, +306167]ns | [1205422, 1397906] | no | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1089599ns | +81.9% | +0.7% | +37.0% | +9.6% | +31.1% | +17.9% |
| 1 | 2 | 1306135ns | +52.1% | -15.6% | +10.4% | -14.8% | +10.0% | -1.1% |
| 1 | 3 | 1090583ns | +84.8% | +0.9% | +32.3% | +1.7% | +45.6% | +23.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.667 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.090 | ok |
| kernel_nway | -0.459 | moderate- |
| kernel_seq | -0.165 | ok |
| nway | -0.147 | ok |
| sequential | -0.142 | ok |
| std_parallel | -0.111 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 1155376.7ns | 1162105.9ns | 99.4% | HIGH |
| head_tail_dyn | 2018810.8ns | 1994685.8ns | 101.2% | HIGH |
| kernel_nway | 1099331.6ns | 1099781.2ns | 100.0% | HIGH |
| kernel_seq | 1457079.9ns | 1459013.9ns | 99.9% | HIGH |
| nway | 1141362.9ns | 1138362.9ns | 100.3% | HIGH |
| sequential | 1487763.9ns | 1484744.8ns | 100.2% | HIGH |
| std_parallel | 1302656.3ns | 1306239.6ns | 99.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1087291.7-1306135.4 ns)
  1087291.7 |########################################
  1098233.9 |
  1109176.1 |
  1120118.3 |
  1131060.4 |
  1142002.6 |
  1152944.8 |
  1163887.0 |
  1174829.2 |
  1185771.4 |
  1196713.5 |
  1207655.7 |
  1218597.9 |##########
  1229540.1 |
  1240482.3 |
  1251424.5 |
  1262366.7 |
  1273308.8 |
  1284251.0 |
  1295193.2 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1927208.3-2046963.6 ns)
  1927208.3 |####################
  1933196.1 |
  1939183.8 |
  1945171.6 |
  1951159.4 |
  1957147.1 |
  1963134.9 |
  1969122.6 |
  1975110.4 |####################
  1981098.2 |########################################
  1987085.9 |
  1993073.7 |
  1999061.4 |
  2005049.2 |
  2011037.0 |
  2017024.7 |
  2023012.5 |
  2029000.3 |
  2034988.0 |
  2040975.8 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 1096291.7-1103859.4 ns)
  1096291.7 |####################
  1096670.1 |
  1097048.5 |
  1097426.8 |########################################
  1097805.2 |
  1098183.6 |
  1098562.0 |
  1098940.4 |####################
  1099318.8 |
  1099697.1 |
  1100075.5 |
  1100453.9 |
  1100832.3 |
  1101210.7 |
  1101589.1 |
  1101967.4 |####################
  1102345.8 |
  1102724.2 |
  1103102.6 |
  1103481.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1432052.1-1492234.4 ns)
  1432052.1 |####################
  1435061.2 |
  1438070.3 |
  1441079.4 |########################################
  1444088.6 |
  1447097.7 |
  1450106.8 |####################
  1453115.9 |
  1456125.0 |
  1459134.1 |
  1462143.2 |
  1465152.4 |####################
  1468161.5 |
  1471170.6 |
  1474179.7 |
  1477188.8 |
  1480197.9 |
  1483207.1 |
  1486216.2 |
  1489225.3 |
  (0 below, 1 above range)

nway (n=6, range 1102354.2-1194989.6 ns)
  1102354.2 |####################
  1106986.0 |####################
  1111617.7 |########################################
  1116249.5 |####################
  1120881.3 |
  1125513.1 |
  1130144.8 |
  1134776.6 |
  1139408.4 |
  1144040.1 |
  1148671.9 |
  1153303.7 |
  1157935.4 |
  1162567.2 |
  1167199.0 |
  1171830.8 |
  1176462.5 |
  1181094.3 |
  1185726.1 |
  1190357.8 |
  (0 below, 1 above range)

sequential (n=6, range 1426822.9-1588328.1 ns)
  1426822.9 |########################################
  1434898.2 |#############
  1442973.4 |
  1451048.7 |
  1459123.9 |
  1467199.2 |
  1475274.5 |
  1483349.7 |
  1491425.0 |
  1499500.2 |
  1507575.5 |
  1515650.8 |
  1523726.0 |
  1531801.3 |
  1539876.5 |
  1547951.8 |
  1556027.1 |
  1564102.3 |
  1572177.6 |
  1580252.8 |#############
  (0 below, 1 above range)

std_parallel (n=6, range 1149333.3-1397906.2 ns)
  1149333.3 |########################################
  1161761.9 |
  1174190.6 |
  1186619.2 |
  1199047.9 |
  1211476.5 |
  1223905.2 |
  1236333.8 |
  1248762.5 |
  1261191.1 |########################################
  1273619.8 |
  1286048.4 |
  1298477.1 |########################################
  1310905.7 |########################################
  1323334.4 |
  1335763.0 |
  1348191.7 |
  1360620.3 |
  1373049.0 |########################################
  1385477.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.2% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.8% of algo (FFI overhead may distort results)
- **nway**: bridge=99.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.2% of algo (FFI overhead may distort results)
