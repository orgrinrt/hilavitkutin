# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 16% faster than the next best (cs_exch)

cs_plain (6.27 ms) leads cs_exch (7.26 ms) by 16%, a clear separation rather than a photo finish. CV 1.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (6.27 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cs_plain) is the fastest** at 6268488.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.46x (fastest 6268488.8 ns, slowest 9147404.5 ns)

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
| Counter frequency | 2112802069 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 7252243ns | 7259808ns | 7149756ns | 7231777ns | 7334185ns | +15.82% |
| cs_plain | 6261818ns | 6272289ns | 6135984ns | 6249873ns | 6342654ns | base |
| cs_snap | 9150837ns | 9083263ns | 9018241ns | 9086638ns | 9313434ns | +46.14% |
| cs_snap_twin | 9167922ns | 9150884ns | 8990417ns | 9143076ns | 9293943ns | +46.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 7248398ns | 7146018ns | 7330270ns | +15.82% | 0.001 |
| cs_plain | 6258118ns | 6132316ns | 6338892ns | base | 0.001 |
| cs_snap | 9147062ns | 9014424ns | 9309756ns | +46.16% | 0.001 |
| cs_snap_twin | 9164294ns | 8986746ns | 9290274ns | +46.44% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 84.5% |
| cs_plain | 0.001 | 97.8% |
| cs_snap | 0.001 | 67.5% |
| cs_snap_twin | 0.001 | 67.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 7252243ns | 7252243ns | +15.82% |
| cs_plain | 6261818ns | 6261818ns | base |
| cs_snap | 9150837ns | 9150837ns | +46.14% |
| cs_snap_twin | 9167922ns | 9167922ns | +46.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 6268489ns | base | --- | [6216319, 6295686] | --- | --- | --- | --- |
| cs_exch | 7255938ns | +973977.1ns (+15.5%) | [+906866, +1071414]ns | [7190027, 7280993] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 9079388ns | +2886514.6ns (+46.0%) | [+2777916, +2983308]ns | [9042418, 9202314] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 9147404ns | +2934051.3ns (+46.8%) | [+2796853, +2980403]ns | [9112982, 9230166] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 6289211ns | +14.4% | +44.4% | +42.4% |
| 1 | 2 | 6156807ns | +19.3% | +47.5% | +47.9% |
| 1 | 3 | 6179717ns | +17.5% | +48.8% | +47.6% |
| 2 | 1 | 6354543ns | +14.7% | +45.8% | +46.1% |
| 2 | 2 | 6340218ns | +13.1% | +42.3% | +44.1% |
| 2 | 3 | 6265292ns | +14.6% | +44.1% | +47.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.003 | ok |
| cs_plain | 0.172 | ok |
| cs_snap | 0.101 | ok |
| cs_snap_twin | 0.050 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 30908344.4ns | 7248397.8ns | 426.4% | HIGH |
| cs_plain | 30860357.7ns | 6258118.2ns | 493.1% | HIGH |
| cs_snap | 30681330.3ns | 9147062.3ns | 335.4% | HIGH |
| cs_snap_twin | 30779751.7ns | 9164294.2ns | 335.9% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 7146018.2-7330270.4 ns)
  7146018.2 |
  7155230.8 |
  7164443.4 |#############
  7173656.0 |########################################
  7182868.6 |
  7192081.2 |##########################
  7201293.8 |
  7210506.5 |
  7219719.1 |
  7228931.7 |
  7238144.3 |
  7247356.9 |##########################
  7256569.5 |##########################
  7265782.1 |##########################
  7274994.7 |
  7284207.4 |#############
  7293420.0 |
  7302632.6 |
  7311845.2 |#############
  7321057.8 |
  (1 below, 3 above range)

cs_plain (n=18, range 6132316.1-6338892.1 ns)
  6132316.1 |
  6142644.9 |
  6152973.7 |########################################
  6163302.5 |
  6173631.3 |####################
  6183960.1 |
  6194288.9 |
  6204617.7 |
  6214946.5 |
  6225275.3 |
  6235604.1 |
  6245932.9 |########################################
  6256261.7 |########################################
  6266590.5 |####################
  6276919.3 |####################
  6287248.1 |########################################
  6297576.9 |####################
  6307905.7 |
  6318234.5 |
  6328563.3 |
  (2 below, 4 above range)

cs_snap (n=18, range 9014424.4-9309756.5 ns)
  9014424.4 |########################################
  9029191.0 |
  9043957.6 |##########
  9058724.2 |
  9073490.8 |########################################
  9088257.4 |
  9103024.0 |
  9117790.6 |
  9132557.2 |##########
  9147323.8 |
  9162090.4 |
  9176857.1 |
  9191623.7 |####################
  9206390.3 |##########
  9221156.9 |
  9235923.5 |##########
  9250690.1 |##########
  9265456.7 |
  9280223.3 |
  9294989.9 |
  (1 below, 2 above range)

cs_snap_twin (n=18, range 8986746.0-9290274.0 ns)
  8986746.0 |
  9001922.4 |
  9017098.8 |
  9032275.2 |
  9047451.6 |
  9062628.0 |
  9077804.4 |####################
  9092980.8 |########################################
  9108157.2 |########################################
  9123333.6 |########################################
  9138510.0 |
  9153686.4 |####################
  9168862.8 |####################
  9184039.2 |
  9199215.6 |####################
  9214392.0 |####################
  9229568.4 |####################
  9244744.8 |####################
  9259921.2 |
  9275097.6 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=423.4% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=490.1% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=337.7% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=335.9% of algo (FFI overhead may distort results)
