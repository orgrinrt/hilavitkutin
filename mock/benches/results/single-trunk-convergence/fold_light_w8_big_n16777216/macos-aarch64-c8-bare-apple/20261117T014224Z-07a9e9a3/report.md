# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail is fastest but the noisiest (CV 12.1%)

head_tail wins on median (1.10 ms) yet has the highest variance (CV 12.1%), while head_tail_dyn is the steadiest (CV 0.1%, 1.98 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (1.10 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, nway, std_parallel, sequential, kernel_seq} vs {head_tail_dyn} (37% apart)

The field splits into a fast tier {head_tail, kernel_nway, nway, std_parallel, sequential, kernel_seq} and a slow tier {head_tail_dyn} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (head_tail) is the fastest** at 1099656.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.80x (fastest 1099656.2 ns, slowest 1984604.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
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
| head_tail | 1161276ns | 1103156ns | 1092917ns | 1100986ns | 1285891ns | base |
| head_tail_dyn | 1987630ns | 1988036ns | 1982458ns | 1987698ns | 1990115ns | +71.16% |
| kernel_nway | 1211479ns | 1169615ns | 1140438ns | 1161233ns | 1322370ns | +4.32% |
| kernel_seq | 1455168ns | 1455047ns | 1453375ns | 1454514ns | 1457047ns | +25.31% |
| nway | 1177413ns | 1177141ns | 1158396ns | 1176451ns | 1188365ns | +1.39% |
| sequential | 1453012ns | 1453865ns | 1441854ns | 1453337ns | 1458104ns | +25.12% |
| std_parallel | 1272559ns | 1234964ns | 1226792ns | 1232316ns | 1355807ns | +9.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 1157606ns | 1089854ns | 1281615ns | base | 14.493 |
| head_tail_dyn | 1984328ns | 1980010ns | 1986542ns | +71.42% | 8.455 |
| kernel_nway | 1207616ns | 1136438ns | 1318062ns | +4.32% | 13.893 |
| kernel_seq | 1453220ns | 1451323ns | 1455479ns | +25.54% | 11.545 |
| nway | 1174575ns | 1155646ns | 1185453ns | +1.47% | 14.284 |
| sequential | 1451410ns | 1440427ns | 1456438ns | +25.38% | 11.559 |
| std_parallel | 1270309ns | 1224427ns | 1353042ns | +9.74% | 13.207 |

## Performance model

- Peak throughput: **15.394 Gops/s** (head_tail; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 15.257 | 99.1% |
| head_tail_dyn | 8.454 | 54.9% |
| kernel_nway | 14.393 | 93.5% |
| kernel_seq | 11.549 | 75.0% |
| nway | 14.286 | 92.8% |
| sequential | 11.551 | 75.0% |
| std_parallel | 13.608 | 88.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 1161276ns | 1161276ns | base |
| head_tail_dyn | 1987630ns | 1987630ns | +71.16% |
| kernel_nway | 1211479ns | 1211479ns | +4.32% |
| kernel_seq | 1455168ns | 1455168ns | +25.31% |
| nway | 1177413ns | 1177413ns | +1.39% |
| sequential | 1453012ns | 1453012ns | +25.12% |
| std_parallel | 1272559ns | 1272559ns | +9.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 1099656ns | base | --- | [1091547, 1281615] | --- | --- | --- | --- |
| head_tail_dyn | 1984604ns | +885546.9ns (+80.5%) | [+700589, +894031]ns | [1981839, 1986542] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| kernel_nway | 1165615ns | no significant difference | [-135750, +218406]ns | [1139172, 1318062] | no | 0.2188 | 0.2188 | 0 |
| kernel_seq | 1452719ns | +352260.4ns (+32.0%) | [+171740, +362844]ns | [1451464, 1455479] | YES (adj: no) | 0.2188 | 0.2188 | 0 |
| nway | 1174396ns | no significant difference | [-113479, +93203]ns | [1163875, 1185453] | no | 0.2188 | 0.2188 | 0 |
| sequential | 1452396ns | +347968.8ns (+31.6%) | [+170922, +362521]ns | [1445396, 1456438] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 1232927ns | no significant difference | [-50391, +260974]ns | [1224958, 1353042] | no | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1091547ns | +81.9% | +6.2% | +33.2% | +8.5% | +33.2% | +23.9% |
| 1 | 2 | 1281615ns | +54.7% | -10.6% | +13.4% | -8.5% | +13.5% | -3.9% |
| 1 | 3 | 1099656ns | +80.5% | +19.9% | +32.0% | +6.1% | +31.4% | +11.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.666 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.660 | HIGH- (thermal bounce) |
| kernel_nway | -0.207 | moderate- |
| kernel_seq | -0.006 | ok |
| nway | -0.028 | ok |
| sequential | -0.207 | moderate- |
| std_parallel | -0.151 | ok |

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
| head_tail | 1171364.6ns | 1157605.9ns | 101.2% | HIGH |
| head_tail_dyn | 1985678.8ns | 1984328.1ns | 100.1% | HIGH |
| kernel_nway | 1213486.1ns | 1207616.3ns | 100.5% | HIGH |
| kernel_seq | 1454237.8ns | 1453220.5ns | 100.1% | HIGH |
| nway | 1177435.8ns | 1174574.7ns | 100.2% | HIGH |
| sequential | 1464343.8ns | 1451409.7ns | 100.9% | HIGH |
| std_parallel | 1275342.0ns | 1270309.0ns | 100.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 1089854.2-1281614.6 ns)
  1089854.2 |########################################
  1099442.2 |##########################
  1109030.2 |
  1118618.3 |
  1128206.3 |
  1137794.3 |
  1147382.3 |
  1156970.3 |
  1166558.3 |
  1176146.4 |
  1185734.4 |
  1195322.4 |
  1204910.4 |
  1214498.4 |
  1224086.4 |
  1233674.5 |
  1243262.5 |
  1252850.5 |
  1262438.5 |
  1272026.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 1980010.4-1986541.7 ns)
  1980010.4 |########################################
  1980337.0 |
  1980663.5 |
  1980990.1 |
  1981316.7 |
  1981643.2 |
  1981969.8 |
  1982296.4 |
  1982622.9 |
  1982949.5 |
  1983276.1 |
  1983602.6 |########################################
  1983929.2 |
  1984255.7 |########################################
  1984582.3 |########################################
  1984908.9 |
  1985235.4 |
  1985562.0 |
  1985888.6 |
  1986215.1 |########################################
  (0 below, 1 above range)

kernel_nway (n=6, range 1136437.5-1318062.5 ns)
  1136437.5 |########################################
  1145518.8 |
  1154600.0 |####################
  1163681.2 |
  1172762.5 |####################
  1181843.8 |
  1190925.0 |
  1200006.2 |####################
  1209087.5 |
  1218168.8 |
  1227250.0 |
  1236331.2 |
  1245412.5 |
  1254493.8 |
  1263575.0 |
  1272656.2 |
  1281737.5 |
  1290818.8 |
  1299900.0 |
  1308981.2 |
  (0 below, 1 above range)

kernel_seq (n=6, range 1451322.9-1455479.1 ns)
  1451322.9 |########################################
  1451530.7 |########################################
  1451738.5 |
  1451946.3 |
  1452154.1 |
  1452362.0 |########################################
  1452569.8 |
  1452777.6 |########################################
  1452985.4 |
  1453193.2 |
  1453401.0 |
  1453608.8 |########################################
  1453816.6 |
  1454024.5 |
  1454232.3 |
  1454440.1 |
  1454647.9 |
  1454855.7 |
  1455063.5 |
  1455271.3 |
  (0 below, 1 above range)

nway (n=6, range 1155645.8-1185453.1 ns)
  1155645.8 |####################
  1157136.2 |
  1158626.5 |
  1160116.9 |
  1161607.3 |
  1163097.6 |
  1164588.0 |
  1166078.4 |
  1167568.7 |
  1169059.1 |
  1170549.5 |
  1172039.8 |########################################
  1173530.2 |
  1175020.6 |####################
  1176510.9 |####################
  1178001.3 |
  1179491.7 |
  1180982.0 |
  1182472.4 |
  1183962.8 |
  (0 below, 1 above range)

sequential (n=6, range 1440427.1-1456437.5 ns)
  1440427.1 |########################################
  1441227.6 |
  1442028.1 |
  1442828.7 |
  1443629.2 |
  1444429.7 |
  1445230.2 |
  1446030.7 |
  1446831.3 |
  1447631.8 |
  1448432.3 |
  1449232.8 |
  1450033.3 |########################################
  1450833.9 |
  1451634.4 |########################################
  1452434.9 |########################################
  1453235.4 |
  1454035.9 |
  1454836.5 |
  1455637.0 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 1224427.1-1353041.6 ns)
  1224427.1 |########################################
  1230857.8 |##########################
  1237288.6 |
  1243719.3 |
  1250150.0 |
  1256580.7 |
  1263011.5 |
  1269442.2 |
  1275872.9 |
  1282303.6 |
  1288734.4 |
  1295165.1 |
  1301595.8 |
  1308026.6 |
  1314457.3 |
  1320888.0 |
  1327318.7 |
  1333749.5 |
  1340180.2 |
  1346610.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=99.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=99.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=99.5% of algo (FFI overhead may distort results)
