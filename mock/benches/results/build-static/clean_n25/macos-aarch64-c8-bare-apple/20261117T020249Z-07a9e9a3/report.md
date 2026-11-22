# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (14119.47 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 14119467604.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 14119467604.1 ns, slowest 14791317541.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 15092473326ns | 14791325729ns | 14156343458ns | 14616471500ns | 16274541000ns | +6.83% |
| llvm_non_incremental | 14127204604ns | 14119472562ns | 13362261417ns | 13983845014ns | 14724715583ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 15092464715ns | 14156333375ns | 16274530625ns | +6.83% |
| llvm_non_incremental | 14127001889ns | 13361678875ns | 14724711271ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 15092473326ns | 15092473326ns | +6.83% |
| llvm_non_incremental | 14127204604ns | 14127204604ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 14119467604ns | base | --- | [13536826792, 14724711271] | --- | --- | --- | --- |
| llvm_incremental | 14791317542ns | +770782645.8ns (+5.5%) | [+282931417, +1842674417]ns | [14211545979, 16274530625] | YES (adj: no) | 0.2188 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 15151737708ns | +7.2% |
| 1 | 2 | 13361678875ns | +6.8% |
| 1 | 3 | 14173122833ns | -0.1% |
| 1 | 4 | 14065812375ns | +4.5% |
| 1 | 5 | 14297684833ns | +4.1% |
| 1 | 6 | 13711974708ns | +18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | 0.002 | ok |
| llvm_non_incremental | -0.490 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 127841486.1ns | 15092464715.3ns | 0.8% |  |
| llvm_non_incremental | 51604652.8ns | 14127001888.9ns | 0.4% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 14156333375.0-16274530625.0 ns)
  14156333375.0 |########################################
  14262243237.5 |########################################
  14368153100.0 |
  14474062962.5 |
  14579972825.0 |
  14685882687.5 |########################################
  14791792550.0 |########################################
  14897702412.5 |
  15003612275.0 |
  15109522137.5 |
  15215432000.0 |
  15321341862.5 |
  15427251725.0 |
  15533161587.5 |
  15639071450.0 |
  15744981312.5 |
  15850891175.0 |
  15956801037.5 |
  16062710900.0 |
  16168620762.5 |########################################
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 13361678875.0-14724711270.8 ns)
  13361678875.0 |########################################
  13429830494.8 |
  13497982114.6 |
  13566133734.4 |
  13634285354.2 |
  13702436974.0 |########################################
  13770588593.7 |
  13838740213.5 |
  13906891833.3 |
  13975043453.1 |
  14043195072.9 |########################################
  14111346692.7 |########################################
  14179498312.5 |
  14247649932.3 |########################################
  14315801552.1 |
  14383953171.8 |
  14452104791.6 |
  14520256411.4 |
  14588408031.2 |
  14656559651.0 |
  (0 below, 1 above range)

```
