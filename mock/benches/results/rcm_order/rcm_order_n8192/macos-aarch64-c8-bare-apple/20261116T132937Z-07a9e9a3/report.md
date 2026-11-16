# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.3% (fastest 17.49 ms to slowest 17.88 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (394.04 us) is smaller than the fastest variant's own run-to-run std-dev (2.48 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (17.49 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader rcm_adj vs stability leader rcm_rev (+2% speed for 1.2x steadier)

rcm_adj is fastest (17.49 ms, CV 14.2%); rcm_rev gives up 2.2% median for 1.2x lower variance (CV 11.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 17486427.1 ns median
- Spread: 1.02x (fastest 17486427.1 ns, slowest 17880464.6 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 18170966ns | 17494479ns | 17237947ns | 17422575ns | 19760061ns | base |
| rcm_half | 19224530ns | 17888873ns | 17233360ns | 17722718ns | 22472833ns | +5.80% |
| rcm_rev | 18763089ns | 17885354ns | 17209461ns | 17695730ns | 21140943ns | +3.26% |
| rcm_scr | 19697989ns | 17553075ns | 17249593ns | 17542766ns | 24155021ns | +8.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 18162773ns | 17229525ns | 19751851ns | base | 0.000 |
| rcm_half | 19216573ns | 17226029ns | 22464694ns | +5.80% | 0.000 |
| rcm_rev | 18754543ns | 17201706ns | 21133775ns | +3.26% | 0.000 |
| rcm_scr | 19690146ns | 17241906ns | 24147267ns | +8.41% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.4% |
| rcm_half | 0.000 | 96.2% |
| rcm_rev | 0.000 | 96.2% |
| rcm_scr | 0.000 | 98.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 18170966ns | 18170966ns | base |
| rcm_half | 19224530ns | 19224530ns | +5.80% |
| rcm_rev | 18763089ns | 18763089ns | +3.26% |
| rcm_scr | 19697989ns | 19697989ns | +8.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 17486427ns | base | --- | [17302650, 17745212] | --- | --- | --- | --- |
| rcm_half | 17880465ns | no significant difference | [-157225, +1100048]ns | [17376358, 20412225] | no | 1.0000 | 1.0000 | 0 |
| rcm_rev | 17875300ns | no significant difference | [-204646, +1006035]ns | [17362475, 18541994] | no | 1.0000 | 0.8145 | 0 |
| rcm_scr | 17545350ns | no significant difference | [-184675, +2857735]ns | [17289027, 22352383] | no | 1.0000 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 17482079ns | -1.4% | -1.2% | -1.0% |
| 1 | 2 | 17490775ns | +35.1% | -0.5% | +46.9% |
| 1 | 3 | 17612346ns | +28.9% | +27.6% | +31.2% |
| 2 | 1 | 17249600ns | +1.5% | +7.8% | +0.2% |
| 2 | 2 | 17314883ns | +2.3% | +0.2% | +1.0% |
| 2 | 3 | 17664171ns | -1.5% | +0.4% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.185 | ok |
| rcm_half | 0.120 | ok |
| rcm_rev | -0.088 | ok |
| rcm_scr | 0.114 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 359023675.2ns | 18162773.4ns | 1976.7% | HIGH |
| rcm_half | 360328891.0ns | 19216573.4ns | 1875.1% | HIGH |
| rcm_rev | 360204905.8ns | 18754542.8ns | 1920.6% | HIGH |
| rcm_scr | 360613082.9ns | 19690145.8ns | 1831.4% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 17229525.0-19751850.7 ns)
  17229525.0 |########################################
  17355641.3 |######
  17481757.6 |####################
  17607873.9 |#############
  17733990.1 |######
  17860106.4 |#############
  17986222.7 |
  18112339.0 |
  18238455.3 |
  18364571.6 |
  18490687.9 |
  18616804.1 |
  18742920.4 |
  18869036.7 |######
  18995153.0 |
  19121269.3 |
  19247385.6 |
  19373501.8 |
  19499618.1 |
  19625734.4 |
  (1 below, 1 above range)

rcm_half (n=18, range 17226029.2-22464694.4 ns)
  17226029.2 |########################################
  17487962.4 |########################
  17749895.7 |
  18011829.0 |########################
  18273762.2 |
  18535695.5 |
  18797628.7 |
  19059562.0 |########
  19321495.3 |
  19583428.5 |
  19845361.8 |
  20107295.1 |
  20369228.3 |
  20631161.6 |
  20893094.9 |
  21155028.1 |
  21416961.4 |
  21678894.6 |########
  21940827.9 |
  22202761.2 |
  (1 below, 4 above range)

rcm_rev (n=18, range 17201705.5-21133775.0 ns)
  17201705.5 |########################################
  17398309.0 |################
  17594912.5 |########
  17791516.0 |
  17988119.4 |########
  18184722.9 |################
  18381326.4 |########
  18577929.8 |########
  18774533.3 |
  18971136.8 |
  19167740.3 |
  19364343.7 |
  19560947.2 |
  19757550.7 |
  19954154.2 |
  20150757.6 |
  20347361.1 |
  20543964.6 |
  20740568.1 |
  20937171.5 |
  (1 below, 4 above range)

rcm_scr (n=18, range 17241905.6-24147266.7 ns)
  17241905.6 |########################################
  17587173.6 |#####
  17932441.7 |#####
  18277709.7 |
  18622977.8 |
  18968245.8 |
  19313513.9 |
  19658781.9 |
  20004050.0 |
  20349318.1 |
  20694586.1 |
  21039854.2 |
  21385122.2 |#####
  21730390.3 |
  22075658.3 |
  22420926.4 |
  22766194.4 |#####
  23111462.5 |#####
  23456730.5 |
  23801998.6 |
  (2 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2051.6% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2008.6% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2007.6% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2044.6% of algo (FFI overhead may distort results)
