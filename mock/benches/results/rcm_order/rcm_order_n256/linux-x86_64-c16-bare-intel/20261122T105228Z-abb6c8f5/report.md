# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (rcm_adj) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline rcm_adj has the worst median (1.08 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rcm_rev at 974.87 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### rcm_half shows alternating (throttle bounce) (autocorr -0.56)

rcm_half's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader rcm_rev vs stability leader rcm_half (+6% speed for 1.1x steadier)

rcm_rev is fastest (974.87 us, CV 9.9%); rcm_half gives up 5.8% median for 1.1x lower variance (CV 8.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 974870.2 ns median (-9.8% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.11x (fastest 974870.2 ns, slowest 1080963.9 ns)

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
| Counter frequency | 2112776360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 1049545ns | 1084433ns | 870724ns | 1034983ns | 1160798ns | base |
| rcm_half | 1011215ns | 1034587ns | 863167ns | 1000037ns | 1102008ns | -3.65% |
| rcm_rev | 984195ns | 978495ns | 836270ns | 962263ns | 1091055ns | -6.23% |
| rcm_scr | 1011917ns | 1011629ns | 828752ns | 987189ns | 1140592ns | -3.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 1045918ns | 867495ns | 1156543ns | base | 0.000 |
| rcm_half | 1007825ns | 860122ns | 1098337ns | -3.64% | 0.000 |
| rcm_rev | 980697ns | 833161ns | 1087281ns | -6.24% | 0.000 |
| rcm_scr | 1008625ns | 825796ns | 1137105ns | -3.57% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 76.4% |
| rcm_half | 0.000 | 80.1% |
| rcm_rev | 0.000 | 84.7% |
| rcm_scr | 0.000 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 1049545ns | 1049545ns | base |
| rcm_half | 1011215ns | 1011215ns | -3.65% |
| rcm_rev | 984195ns | 984195ns | -6.23% |
| rcm_scr | 1011917ns | 1011917ns | -3.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 1080964ns | base | --- | [937788, 1135654] | --- | --- | --- | --- |
| rcm_half | 1031061ns | -43621.2ns (-4.0%) | [-63965, -16076]ns | [927560, 1059105] | YES | 0.0309 | 0.0309 | 0 |
| rcm_rev | 974870ns | -55645.5ns (-5.1%) | [-109871, -16970]ns | [914150, 1068821] | YES | 0.0309 | 0.0309 | 0 |
| rcm_scr | 1007963ns | -27916.8ns (-2.6%) | [-95040, -12315]ns | [919196, 1119332] | YES | 0.0309 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 1084999ns | -5.7% | -0.4% | +3.5% |
| 1 | 2 | 1117533ns | -6.4% | -3.7% | -10.4% |
| 1 | 3 | 1069338ns | -1.7% | -13.2% | +6.8% |
| 2 | 1 | 1048705ns | -2.4% | +1.0% | -12.5% |
| 2 | 2 | 1134599ns | -3.6% | -18.9% | -11.7% |
| 2 | 3 | 1076929ns | -4.7% | -8.8% | -2.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.447 | moderate- |
| rcm_half | -0.561 | HIGH- (thermal bounce) |
| rcm_rev | -0.247 | moderate- |
| rcm_scr | -0.439 | moderate- |

**Consistency summary:**

- **rcm_half**: won 6/6, lost 0/6
- **rcm_rev**: won 5/6, lost 1/6
- **rcm_scr**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 7777687.8ns | 1045917.7ns | 743.6% | HIGH |
| rcm_half | 7754625.8ns | 1007825.5ns | 769.4% | HIGH |
| rcm_rev | 7718541.5ns | 980697.1ns | 787.0% | HIGH |
| rcm_scr | 7754051.8ns | 1008624.6ns | 768.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 867494.7-1156542.8 ns)
  867494.7 |
  881947.1 |
  896399.5 |#############
  910851.9 |#############
  925304.4 |#############
  939756.8 |#############
  954209.2 |
  968661.6 |
  983114.0 |
  997566.4 |
  1012018.8 |
  1026471.2 |
  1040923.6 |#############
  1055376.0 |#############
  1069828.4 |#############
  1084280.8 |##########################
  1098733.2 |
  1113185.6 |#############
  1127638.0 |##########################
  1142090.4 |########################################
  (2 below, 1 above range)

rcm_half (n=18, range 860122.4-1098337.4 ns)
  860122.4 |##########################
  872033.2 |
  883943.9 |
  895854.7 |
  907765.4 |##########################
  919676.2 |#############
  931586.9 |
  943497.7 |
  955408.4 |
  967319.2 |
  979229.9 |
  991140.6 |
  1003051.4 |
  1014962.1 |########################################
  1026872.9 |#############
  1038783.6 |#############
  1050694.4 |########################################
  1062605.1 |
  1074515.9 |
  1086426.6 |#############
  (1 below, 3 above range)

rcm_rev (n=18, range 833161.0-1087280.8 ns)
  833161.0 |
  845867.0 |
  858573.0 |##########
  871279.0 |
  883985.0 |
  896691.0 |####################
  909397.0 |##########
  922102.9 |####################
  934808.9 |
  947514.9 |
  960220.9 |##########
  972926.9 |##########
  985632.9 |
  998338.9 |
  1011044.8 |##########
  1023750.8 |
  1036456.8 |
  1049162.8 |####################
  1061868.8 |
  1074574.8 |########################################
  (2 below, 1 above range)

rcm_scr (n=18, range 825796.5-1137105.4 ns)
  825796.5 |##########################
  841361.9 |
  856927.4 |
  872492.8 |
  888058.2 |
  903623.7 |##########################
  919189.1 |##########################
  934754.6 |
  950320.0 |
  965885.5 |
  981450.9 |
  997016.4 |##########################
  1012581.8 |#############
  1028147.2 |
  1043712.7 |#############
  1059278.1 |
  1074843.6 |
  1090409.0 |
  1105974.5 |##########################
  1121539.9 |########################################
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=721.1% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=755.1% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=788.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=771.9% of algo (FFI overhead may distort results)
