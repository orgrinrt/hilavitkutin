# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.7% (fastest 37.46 ms to slowest 37.74 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (277.03 us) is smaller than the fastest variant's own run-to-run std-dev (637.34 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_rev** at 37461488.5 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.01x (fastest 37461488.5 ns, slowest 37738520.2 ns)

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
| Counter frequency | 2112746224 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 37585648ns | 37512975ns | 37255834ns | 37480607ns | 37908117ns | base |
| rcm_half | 37773192ns | 37742464ns | 37449750ns | 37672877ns | 38085386ns | +0.50% |
| rcm_rev | 37753190ns | 37465374ns | 37164065ns | 37433439ns | 38527379ns | +0.45% |
| rcm_scr | 37652379ns | 37477001ns | 37176012ns | 37441992ns | 38206143ns | +0.18% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 37581799ns | 37251959ns | 37904314ns | base | 0.000 |
| rcm_half | 37769297ns | 37445799ns | 38081533ns | +0.50% | 0.000 |
| rcm_rev | 37749240ns | 37160143ns | 38523368ns | +0.45% | 0.000 |
| rcm_scr | 37648547ns | 37172194ns | 38202367ns | +0.18% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 99.1% |
| rcm_half | 0.000 | 98.5% |
| rcm_rev | 0.000 | 99.2% |
| rcm_scr | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 37585648ns | 37585648ns | base |
| rcm_half | 37773192ns | 37773192ns | +0.50% |
| rcm_rev | 37753190ns | 37753190ns | +0.45% |
| rcm_scr | 37652379ns | 37652379ns | +0.18% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 37508958ns | base | --- | [37348540, 37800187] | --- | --- | --- | --- |
| rcm_half | 37738520ns | no significant difference | [-2359, +268320]ns | [37593360, 37801083] | no | 0.2888 | 0.0963 | 0 |
| rcm_rev | 37461489ns | no significant difference | [-223613, +282397]ns | [37361605, 37791018] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 37473147ns | no significant difference | [-313526, +196209]ns | [37373479, 37563055] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 37366146ns | +0.6% | +3.8% | +0.2% |
| 1 | 2 | 37681806ns | +0.1% | -0.6% | +1.9% |
| 1 | 3 | 37888580ns | -0.1% | -0.7% | -1.4% |
| 2 | 1 | 37255201ns | +0.7% | +0.3% | +0.8% |
| 2 | 2 | 37349674ns | +1.1% | -0.3% | -0.5% |
| 2 | 3 | 37824742ns | -0.1% | -0.9% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.205 | moderate- |
| rcm_half | -0.398 | moderate- |
| rcm_rev | -0.020 | ok |
| rcm_scr | -0.307 | moderate- |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 4/6, lost 2/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 245290850.1ns | 37581799.1ns | 652.7% | HIGH |
| rcm_half | 245382889.7ns | 37769296.7ns | 649.7% | HIGH |
| rcm_rev | 245589666.1ns | 37749239.8ns | 650.6% | HIGH |
| rcm_scr | 245508099.2ns | 37648546.7ns | 652.1% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 37251958.9-37904314.0 ns)
  37251958.9 |##########################
  37284576.6 |
  37317194.4 |########################################
  37349812.1 |#############
  37382429.9 |
  37415047.6 |#############
  37447665.4 |#############
  37480283.2 |
  37512900.9 |
  37545518.7 |#############
  37578136.4 |
  37610754.2 |
  37643371.9 |
  37675989.7 |#############
  37708607.4 |
  37741225.2 |#############
  37773843.0 |#############
  37806460.7 |##########################
  37839078.5 |#############
  37871696.2 |#############
  (1 below, 1 above range)

rcm_half (n=18, range 37445799.2-38081533.5 ns)
  37445799.2 |
  37477585.9 |#############
  37509372.6 |##########################
  37541159.3 |
  37572946.0 |########################################
  37604732.7 |
  37636519.5 |
  37668306.2 |
  37700092.9 |##########################
  37731879.6 |##########################
  37763666.3 |##########################
  37795453.0 |#############
  37827239.8 |##########################
  37859026.5 |
  37890813.2 |
  37922599.9 |
  37954386.6 |
  37986173.3 |
  38017960.1 |#############
  38049746.8 |
  (1 below, 1 above range)

rcm_rev (n=18, range 37160142.6-38523368.0 ns)
  37160142.6 |#############
  37228303.9 |#############
  37296465.1 |##########################
  37364626.4 |########################################
  37432787.7 |########################################
  37500949.0 |
  37569110.2 |#############
  37637271.5 |
  37705432.8 |#############
  37773594.0 |
  37841755.3 |#############
  37909916.6 |
  37978077.9 |
  38046239.1 |
  38114400.4 |
  38182561.7 |
  38250722.9 |
  38318884.2 |
  38387045.5 |
  38455206.8 |
  (1 below, 4 above range)

rcm_scr (n=18, range 37172193.6-38202367.2 ns)
  37172193.6 |##########################
  37223702.3 |
  37275211.0 |
  37326719.7 |##########################
  37378228.3 |##########################
  37429737.0 |########################################
  37481245.7 |##########################
  37532754.4 |##########################
  37584263.0 |#############
  37635771.7 |
  37687280.4 |
  37738789.1 |
  37790297.8 |
  37841806.4 |#############
  37893315.1 |
  37944823.8 |
  37996332.5 |
  38047841.1 |
  38099349.8 |
  38150858.5 |
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=654.3% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=649.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=655.4% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=654.5% of algo (FFI overhead may distort results)
