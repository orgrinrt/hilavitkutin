# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 24% faster than the next best (cs_exch)

cs_plain (584.76 us) leads cs_exch (725.26 us) by 24%, a clear separation rather than a photo finish. CV 14.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_exch shows alternating (throttle bounce) (autocorr -0.61)

cs_exch's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (584.76 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cs_plain) is the fastest** at 584761.2 ns median
- 3 variants significantly slower than baseline
- Spread: 1.87x (fastest 584761.2 ns, slowest 1093172.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 2100370818 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 747922ns | 730031ns | 622132ns | 719330ns | 853704ns | +25.71% |
| cs_plain | 594955ns | 589586ns | 505066ns | 571756ns | 674697ns | base |
| cs_snap | 1106974ns | 1098355ns | 990482ns | 1089270ns | 1191775ns | +86.06% |
| cs_snap_twin | 1068287ns | 1012934ns | 907715ns | 985912ns | 1272135ns | +79.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 743005ns | 618159ns | 848442ns | +26.00% | 0.001 |
| cs_plain | 589665ns | 500755ns | 668159ns | base | 0.002 |
| cs_snap | 1101555ns | 985878ns | 1186005ns | +86.81% | 0.001 |
| cs_snap_twin | 1063404ns | 903382ns | 1266501ns | +80.34% | 0.001 |

## Performance model

- Peak throughput: **0.002 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 69.0% |
| cs_plain | 0.002 | 85.6% |
| cs_snap | 0.001 | 45.8% |
| cs_snap_twin | 0.001 | 49.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 747922ns | 747922ns | +25.71% |
| cs_plain | 594955ns | 594955ns | base |
| cs_snap | 1106974ns | 1106974ns | +86.06% |
| cs_snap_twin | 1068287ns | 1068287ns | +79.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 584761ns | base | --- | [538099, 603270] | --- | --- | --- | --- |
| cs_exch | 725259ns | +133687.7ns (+22.9%) | [+108373, +164704]ns | [697826, 758483] | YES | 0.0001 | 0.0001 | 0 |
| cs_snap | 1093172ns | +543513.8ns (+92.9%) | [+490545, +569452]ns | [1050050, 1154832] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 1008511ns | +434553.5ns (+74.3%) | [+385507, +482940]ns | [937288, 1087478] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 601356ns | +21.6% | +99.5% | +51.6% |
| 1 | 2 | 574883ns | +25.9% | +90.7% | +98.1% |
| 1 | 3 | 605183ns | +29.2% | +92.8% | +81.7% |
| 2 | 1 | 630217ns | +12.3% | +64.9% | +60.1% |
| 2 | 2 | 510528ns | +80.4% | +112.7% | +83.8% |
| 2 | 3 | 502051ns | +21.0% | +110.6% | +86.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.615 | HIGH- (thermal bounce) |
| cs_plain | 0.205 | moderate+ |
| cs_snap | -0.159 | ok |
| cs_snap_twin | 0.107 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 5098298.2ns | 743005.0ns | 686.2% | HIGH |
| cs_plain | 4898953.1ns | 589664.5ns | 830.8% | HIGH |
| cs_snap | 5271130.2ns | 1101554.8ns | 478.5% | HIGH |
| cs_snap_twin | 4813612.2ns | 1063404.0ns | 452.7% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 618159.3-848442.2 ns)
  618159.3 |
  629673.4 |########
  641187.6 |
  652701.7 |
  664215.9 |
  675730.0 |########
  687244.2 |########
  698758.3 |################
  710272.5 |
  721786.6 |########################################
  733300.8 |
  744814.9 |########
  756329.1 |########
  767843.2 |
  779357.4 |########
  790871.5 |
  802385.7 |
  813899.8 |
  825414.0 |########
  836928.1 |
  (2 below, 2 above range)

cs_plain (n=18, range 500754.7-668158.6 ns)
  500754.7 |########################################
  509124.9 |#############
  517495.1 |
  525865.3 |
  534235.5 |
  542605.7 |
  550975.9 |
  559346.1 |#############
  567716.3 |#############
  576086.5 |##########################
  584456.7 |#############
  592826.9 |##########################
  601197.1 |##########################
  609567.3 |
  617937.5 |#############
  626307.7 |#############
  634677.8 |
  643048.0 |
  651418.2 |
  659788.4 |#############
  (1 below, 1 above range)

cs_snap (n=18, range 985878.1-1186004.9 ns)
  985878.1 |
  995884.4 |#############
  1005890.7 |
  1015897.1 |
  1025903.4 |#############
  1035909.8 |##########################
  1045916.1 |
  1055922.4 |##########################
  1065928.8 |
  1075935.1 |#############
  1085941.5 |#############
  1095947.8 |#############
  1105954.1 |
  1115960.5 |
  1125966.8 |
  1135973.2 |########################################
  1145979.5 |
  1155985.8 |
  1165992.2 |#############
  1175998.5 |
  (1 below, 4 above range)

cs_snap_twin (n=18, range 903381.8-1266500.8 ns)
  903381.8 |##########################
  921537.8 |##########################
  939693.7 |#############
  957849.7 |
  976005.6 |
  994161.6 |########################################
  1012317.5 |#############
  1030473.5 |#############
  1048629.4 |
  1066785.4 |#############
  1084941.3 |#############
  1103097.3 |
  1121253.2 |#############
  1139409.2 |
  1157565.1 |
  1175721.1 |
  1193877.0 |#############
  1212033.0 |
  1230188.9 |
  1248344.9 |
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=730.2% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=868.1% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=486.0% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=474.5% of algo (FFI overhead may distort results)
