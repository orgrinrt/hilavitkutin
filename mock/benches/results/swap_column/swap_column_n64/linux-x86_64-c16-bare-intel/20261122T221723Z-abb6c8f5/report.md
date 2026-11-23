# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (16.66 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### cs_exch is inconsistent: worst-20% is 1.9x its best-20%

cs_exch's best 20% of batches run at 17.33 us but its worst 20% at 33.31 us (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (cs_plain) is the fastest** at 16662.8 ns median
- 3 variants significantly slower than baseline
- Spread: 1.34x (fastest 16662.8 ns, slowest 22348.6 ns)

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
| Counter frequency | 2112756002 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 24859ns | 19302ns | 18959ns | 19272ns | 36188ns | +19.51% |
| cs_plain | 20800ns | 18296ns | 17669ns | 18174ns | 26305ns | base |
| cs_snap | 23907ns | 23712ns | 23302ns | 23619ns | 24643ns | +14.94% |
| cs_snap_twin | 26442ns | 24009ns | 23192ns | 23835ns | 31978ns | +27.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 22804ns | 17334ns | 33307ns | +20.51% | 0.003 |
| cs_plain | 18923ns | 16056ns | 23938ns | base | 0.003 |
| cs_snap | 22248ns | 21622ns | 22985ns | +17.57% | 0.003 |
| cs_snap_twin | 24604ns | 21491ns | 29804ns | +30.02% | 0.003 |

## Performance model

- Peak throughput: **0.004 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.004 | 90.8% |
| cs_plain | 0.004 | 96.4% |
| cs_snap | 0.003 | 72.8% |
| cs_snap_twin | 0.003 | 71.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 24859ns | 24859ns | +19.51% |
| cs_plain | 20800ns | 20800ns | base |
| cs_snap | 23907ns | 23907ns | +14.94% |
| cs_snap_twin | 26442ns | 26442ns | +27.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 16663ns | base | --- | [16247, 17225] | --- | --- | --- | --- |
| cs_exch | 17675ns | +822.6ns (+4.9%) | [+613, +1237]ns | [17591, 17828] | YES | 0.0075 | 0.0075 | 0 |
| cs_snap | 22068ns | +5527.0ns (+33.2%) | [+4867, +5916]ns | [21789, 22664] | YES | 0.0075 | 0.0075 | 0 |
| cs_snap_twin | 22349ns | +5666.2ns (+34.0%) | [+5030, +5913]ns | [21894, 22538] | YES | 0.0039 | 0.0013 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 17144ns | +3.1% | +32.4% | +32.0% |
| 1 | 2 | 16299ns | +7.9% | +33.7% | +38.1% |
| 1 | 3 | 16911ns | +4.7% | +29.1% | +27.1% |
| 2 | 1 | 16496ns | +5.5% | +34.5% | +34.4% |
| 2 | 2 | 30610ns | -42.2% | -29.4% | -4.5% |
| 2 | 3 | 16056ns | +142.1% | +44.9% | +36.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.029 | ok |
| cs_plain | -0.285 | moderate- |
| cs_snap | -0.306 | moderate- |
| cs_snap_twin | -0.264 | moderate- |

**Consistency summary:**

- **cs_exch**: won 1/6, lost 5/6
- **cs_snap**: won 1/6, lost 5/6
- **cs_snap_twin**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 314893.0ns | 22803.6ns | 1380.9% | HIGH |
| cs_plain | 279481.0ns | 18923.1ns | 1476.9% | HIGH |
| cs_snap | 239774.0ns | 22247.6ns | 1077.8% | HIGH |
| cs_snap_twin | 263032.7ns | 24604.0ns | 1069.1% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 17333.7-33307.0 ns)
  17333.7 |########################################
  18132.4 |##
  18931.0 |
  19729.7 |
  20528.4 |
  21327.0 |
  22125.7 |
  22924.3 |
  23723.0 |
  24521.7 |
  25320.3 |
  26119.0 |
  26917.7 |
  27716.3 |
  28515.0 |
  29313.7 |
  30112.3 |
  30911.0 |
  31709.6 |
  32508.3 |
  (1 below, 2 above range)

cs_plain (n=18, range 16056.2-23938.4 ns)
  16056.2 |########################################
  16450.3 |########################################
  16844.4 |##############################
  17238.5 |####################
  17632.6 |
  18026.7 |
  18420.9 |
  18815.0 |
  19209.1 |
  19603.2 |
  19997.3 |
  20391.4 |
  20785.5 |
  21179.6 |
  21573.7 |
  21967.8 |
  22361.9 |
  22756.1 |
  23150.2 |
  23544.3 |
  (2 below, 3 above range)

cs_snap (n=18, range 21622.0-22984.8 ns)
  21622.0 |#############
  21690.1 |#############
  21758.3 |########################################
  21826.4 |#############
  21894.6 |
  21962.7 |#############
  22030.8 |
  22099.0 |#############
  22167.1 |#############
  22235.3 |
  22303.4 |
  22371.5 |#############
  22439.7 |
  22507.8 |
  22575.9 |
  22644.1 |##########################
  22712.2 |#############
  22780.4 |
  22848.5 |
  22916.6 |
  (2 below, 3 above range)

cs_snap_twin (n=18, range 21490.7-29803.7 ns)
  21490.7 |##########################
  21906.4 |##########################
  22322.0 |########################################
  22737.7 |######
  23153.3 |
  23569.0 |
  23984.6 |
  24400.3 |
  24815.9 |
  25231.6 |
  25647.2 |
  26062.9 |
  26478.5 |
  26894.2 |
  27309.8 |
  27725.4 |
  28141.1 |
  28556.7 |
  28972.4 |######
  29388.0 |
  (1 below, 1 above range)

```

## Diagnostics

- **cs_exch**: CV=73.8% (high variance, measurements may be unstable)
- **cs_exch**: bridge=1360.2% of algo (FFI overhead may distort results)
- **cs_plain**: CV=27.6% (high variance, measurements may be unstable)
- **cs_plain**: bridge=1437.5% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1084.1% of algo (FFI overhead may distort results)
- **cs_snap_twin**: CV=34.7% (high variance, measurements may be unstable)
- **cs_snap_twin**: bridge=1073.8% of algo (FFI overhead may distort results)
