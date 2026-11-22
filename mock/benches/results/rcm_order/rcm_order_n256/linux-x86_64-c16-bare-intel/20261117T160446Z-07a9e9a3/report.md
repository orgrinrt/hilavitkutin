# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rcm_rev shows alternating (throttle bounce) (autocorr -0.53)

rcm_rev's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (64.64 us) is smaller than the fastest variant's own run-to-run std-dev (101.05 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (998.83 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader rcm_adj vs stability leader rcm_rev (+4% speed for 1.0x steadier)

rcm_adj is fastest (998.83 us, CV 10.1%); rcm_rev gives up 3.9% median for 1.0x lower variance (CV 10.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 998833.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 998833.6 ns, slowest 1063477.0 ns)

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
| Counter frequency | 2112757625 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 996678ns | 1002496ns | 826084ns | 977868ns | 1110190ns | base |
| rcm_half | 1029125ns | 1016486ns | 877567ns | 1006195ns | 1139297ns | +3.26% |
| rcm_rev | 1030536ns | 1040860ns | 877561ns | 1005935ns | 1143925ns | +3.40% |
| rcm_scr | 1045442ns | 1067317ns | 888442ns | 1015785ns | 1168429ns | +4.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 993101ns | 822815ns | 1106391ns | base | 0.000 |
| rcm_half | 1025495ns | 874123ns | 1135494ns | +3.26% | 0.000 |
| rcm_rev | 1026997ns | 874250ns | 1140226ns | +3.41% | 0.000 |
| rcm_scr | 1041937ns | 885189ns | 1164756ns | +4.92% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 82.4% |
| rcm_half | 0.000 | 81.2% |
| rcm_rev | 0.000 | 79.3% |
| rcm_scr | 0.000 | 77.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 996678ns | 996678ns | base |
| rcm_half | 1029125ns | 1029125ns | +3.26% |
| rcm_rev | 1030536ns | 1030536ns | +3.40% |
| rcm_scr | 1045442ns | 1045442ns | +4.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 998834ns | base | --- | [941578, 1072189] | --- | --- | --- | --- |
| rcm_half | 1012867ns | no significant difference | [-6507, +50444]ns | [980799, 1055032] | no | 0.3568 | 0.2379 | 0 |
| rcm_rev | 1037409ns | no significant difference | [-22616, +87178]ns | [930052, 1101625] | no | 0.4807 | 0.4807 | 0 |
| rcm_scr | 1063477ns | +47900.0ns (+4.8%) | [+19720, +78778]ns | [939637, 1144045] | YES (adj: no) | 0.0927 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 982355ns | +2.0% | +3.8% | +12.2% |
| 1 | 2 | 1000245ns | +5.1% | +14.8% | +5.4% |
| 1 | 3 | 997422ns | +5.7% | -3.0% | +9.4% |
| 2 | 1 | 1114671ns | -12.2% | -2.0% | +4.1% |
| 2 | 2 | 1108861ns | -9.0% | +4.3% | -7.0% |
| 2 | 3 | 995937ns | +1.9% | +4.7% | -3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.175 | ok |
| rcm_half | -0.089 | ok |
| rcm_rev | -0.531 | HIGH- (thermal bounce) |
| rcm_scr | 0.083 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 7729527.2ns | 993100.6ns | 778.3% | HIGH |
| rcm_half | 7793239.9ns | 1025495.5ns | 759.9% | HIGH |
| rcm_rev | 7856149.3ns | 1026996.6ns | 765.0% | HIGH |
| rcm_scr | 7849922.4ns | 1041936.9ns | 753.4% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 822814.9-1106391.3 ns)
  822814.9 |########
  836993.7 |
  851172.6 |
  865351.4 |
  879530.2 |
  893709.0 |########
  907887.8 |
  922066.7 |
  936245.5 |################
  950424.3 |
  964603.1 |
  978781.9 |########
  992960.7 |########################################
  1007139.6 |
  1021318.4 |
  1035497.2 |########
  1049676.0 |
  1063854.8 |
  1078033.6 |
  1092212.5 |
  (2 below, 5 above range)

rcm_half (n=18, range 874122.9-1135493.5 ns)
  874122.9 |
  887191.4 |
  900260.0 |
  913328.5 |####################
  926397.0 |
  939465.6 |
  952534.1 |
  965602.6 |##########
  978671.2 |##########
  991739.7 |##########
  1004808.2 |##############################
  1017876.8 |
  1030945.3 |
  1044013.8 |########################################
  1057082.4 |
  1070150.9 |
  1083219.4 |
  1096288.0 |##########
  1109356.5 |##########
  1122425.0 |##########
  (2 below, 1 above range)

rcm_rev (n=18, range 874250.0-1140226.3 ns)
  874250.0 |####################
  887548.8 |
  900847.7 |####################
  914146.5 |########################################
  927445.3 |####################
  940744.1 |
  954042.9 |####################
  967341.7 |
  980640.6 |
  993939.4 |
  1007238.2 |####################
  1020537.0 |####################
  1033835.8 |####################
  1047134.6 |
  1060433.4 |
  1073732.3 |####################
  1087031.1 |########################################
  1100329.9 |####################
  1113628.7 |
  1126927.5 |
  (1 below, 4 above range)

rcm_scr (n=18, range 885189.4-1164755.5 ns)
  885189.4 |####################
  899167.7 |####################
  913146.0 |########################################
  927124.3 |
  941102.7 |####################
  955081.0 |####################
  969059.3 |
  983037.6 |
  997015.9 |
  1010994.2 |
  1024972.5 |####################
  1038950.8 |
  1052929.1 |####################
  1066907.4 |####################
  1080885.7 |####################
  1094864.0 |####################
  1108842.3 |
  1122820.6 |####################
  1136798.9 |
  1150777.2 |########################################
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=771.6% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=768.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=751.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=732.2% of algo (FFI overhead may distort results)
