# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rcm_half shows alternating (throttle bounce) (autocorr -0.55)

rcm_half's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (53.94 us) is smaller than the fastest variant's own run-to-run std-dev (107.06 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader rcm_rev vs stability leader rcm_half (+6% speed for 1.0x steadier)

rcm_rev is fastest (978.89 us, CV 10.9%); rcm_half gives up 5.5% median for 1.0x lower variance (CV 10.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 978887.9 ns median (-3.9% vs baseline)
- Spread: 1.06x (fastest 978887.9 ns, slowest 1032823.5 ns)

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
| rcm_adj | 998662ns | 1022580ns | 815077ns | 974283ns | 1127023ns | base |
| rcm_half | 1031636ns | 1036099ns | 850498ns | 1011716ns | 1152087ns | +3.30% |
| rcm_rev | 980647ns | 982654ns | 815509ns | 956902ns | 1098832ns | -1.80% |
| rcm_scr | 1013913ns | 1011488ns | 823983ns | 985049ns | 1152173ns | +1.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 995193ns | 811889ns | 1123421ns | base | 0.000 |
| rcm_half | 1028197ns | 847428ns | 1148398ns | +3.32% | 0.000 |
| rcm_rev | 977126ns | 812345ns | 1095147ns | -1.82% | 0.000 |
| rcm_scr | 1010446ns | 820872ns | 1148555ns | +1.53% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 79.7% |
| rcm_half | 0.000 | 78.6% |
| rcm_rev | 0.000 | 82.9% |
| rcm_scr | 0.000 | 80.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 998662ns | 998662ns | base |
| rcm_half | 1031636ns | 1031636ns | +3.30% |
| rcm_rev | 980647ns | 980647ns | -1.80% |
| rcm_scr | 1013913ns | 1013913ns | +1.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 1018946ns | base | --- | [896857, 1082894] | --- | --- | --- | --- |
| rcm_half | 1032824ns | no significant difference | [-3772, +84810]ns | [967944, 1128613] | no | 0.7210 | 0.4807 | 0 |
| rcm_rev | 978888ns | no significant difference | [-62255, +53443]ns | [906653, 1042727] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 1007955ns | no significant difference | [-64871, +85021]ns | [920162, 1126087] | no | 0.7137 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 1141360ns | -9.1% | -11.6% | -12.7% |
| 1 | 2 | 1123174ns | +5.1% | -12.9% | -10.5% |
| 1 | 3 | 1031986ns | -0.3% | +5.8% | +10.5% |
| 2 | 1 | 1010265ns | +11.3% | -0.5% | +13.8% |
| 2 | 2 | 987606ns | +12.8% | -2.5% | +2.4% |
| 2 | 3 | 994778ns | +0.7% | +7.6% | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.529 | HIGH+ (drift/warm-up) |
| rcm_half | -0.547 | HIGH- (thermal bounce) |
| rcm_rev | -0.435 | moderate- |
| rcm_scr | -0.074 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 4/6, lost 2/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 7715761.6ns | 995193.5ns | 775.3% | HIGH |
| rcm_half | 7832148.2ns | 1028197.3ns | 761.7% | HIGH |
| rcm_rev | 7708005.3ns | 977126.5ns | 788.8% | HIGH |
| rcm_scr | 7803977.2ns | 1010446.2ns | 772.3% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 811889.2-1123421.4 ns)
  811889.2 |########################################
  827465.8 |
  843042.5 |####################
  858619.1 |####################
  874195.7 |
  889772.3 |
  905348.9 |
  920925.5 |####################
  936502.1 |
  952078.7 |
  967655.3 |
  983232.0 |########################################
  998808.6 |####################
  1014385.2 |####################
  1029961.8 |########################################
  1045538.4 |
  1061115.0 |####################
  1076691.6 |
  1092268.2 |####################
  1107844.8 |####################
  (1 below, 3 above range)

rcm_half (n=18, range 847428.1-1148397.5 ns)
  847428.1 |
  862476.6 |
  877525.1 |
  892573.5 |#############
  907622.0 |#############
  922670.5 |
  937718.9 |#############
  952767.4 |
  967815.9 |
  982864.4 |#############
  997912.8 |##########################
  1012961.3 |
  1028009.8 |########################################
  1043058.2 |
  1058106.7 |
  1073155.2 |
  1088203.6 |
  1103252.1 |#############
  1118300.6 |##########################
  1133349.0 |##########################
  (2 below, 2 above range)

rcm_rev (n=18, range 812345.3-1095147.4 ns)
  812345.3 |##########################
  826485.4 |
  840625.5 |
  854765.6 |#############
  868905.7 |
  883045.8 |
  897185.9 |##########################
  911326.1 |
  925466.2 |
  939606.3 |
  953746.4 |#############
  967886.5 |########################################
  982026.6 |
  996166.7 |##########################
  1010306.8 |
  1024446.9 |
  1038587.0 |#############
  1052727.1 |
  1066867.2 |##########################
  1081007.3 |##########################
  (1 below, 1 above range)

rcm_scr (n=18, range 820871.9-1148555.1 ns)
  820871.9 |##########################
  837256.1 |
  853640.3 |
  870024.4 |#############
  886408.6 |#############
  902792.7 |
  919176.9 |
  935561.0 |#############
  951945.2 |
  968329.4 |
  984713.5 |#############
  1001097.7 |########################################
  1017481.8 |
  1033866.0 |##########################
  1050250.2 |
  1066634.3 |
  1083018.5 |
  1099402.6 |
  1115786.8 |##########################
  1132171.0 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: autocorrelation=0.53 (measurement drift or warm-up artifact)
- **rcm_adj**: bridge=755.8% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=753.5% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=782.9% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=769.6% of algo (FFI overhead may distort results)
