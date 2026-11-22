# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway dominates: 27% faster than the next best (head_tail)

kernel_nway (56.73 us) leads head_tail (71.80 us) by 27%, a clear separation rather than a photo finish. CV 3.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### head_tail_dyn is an outlier: 2.4x slower than the field

head_tail_dyn (137.04 us) is 2.4x the fastest (56.73 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 56725.0 ns median (-21.0% vs baseline)
- 1 variant significantly faster than baseline
- 4 variants significantly slower than baseline
- Spread: 2.42x (fastest 56725.0 ns, slowest 137037.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 71311ns | 72592ns | 63546ns | 72044ns | 74094ns | base |
| head_tail_dyn | 137388ns | 137390ns | 135483ns | 137284ns | 138495ns | +92.66% |
| kernel_nway | 57595ns | 57344ns | 54360ns | 57032ns | 60058ns | -19.23% |
| kernel_seq | 96083ns | 95873ns | 94878ns | 95624ns | 97376ns | +34.74% |
| nway | 72791ns | 72962ns | 69439ns | 72208ns | 75341ns | +2.07% |
| sequential | 100211ns | 99690ns | 98775ns | 99491ns | 102009ns | +40.53% |
| std_parallel | 107885ns | 103331ns | 98804ns | 103280ns | 119332ns | +51.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 70644ns | 62899ns | 73422ns | base | 14.843 |
| head_tail_dyn | 136958ns | 134978ns | 138133ns | +93.87% | 7.656 |
| kernel_nway | 56929ns | 53704ns | 59400ns | -19.41% | 18.419 |
| kernel_seq | 95825ns | 94699ns | 97008ns | +35.64% | 10.943 |
| nway | 72201ns | 68863ns | 74742ns | +2.20% | 14.523 |
| sequential | 99923ns | 98579ns | 101556ns | +41.45% | 10.494 |
| std_parallel | 107472ns | 98382ns | 118891ns | +52.13% | 9.757 |

## Performance model

- Peak throughput: **19.525 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 14.605 | 74.8% |
| head_tail_dyn | 7.652 | 39.2% |
| kernel_nway | 18.485 | 94.7% |
| kernel_seq | 10.962 | 56.1% |
| nway | 14.499 | 74.3% |
| sequential | 10.541 | 54.0% |
| std_parallel | 10.183 | 52.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 71311ns | 71311ns | base |
| head_tail_dyn | 137388ns | 137388ns | +92.66% |
| kernel_nway | 57595ns | 57595ns | -19.23% |
| kernel_seq | 96083ns | 96083ns | +34.74% |
| nway | 72791ns | 72791ns | +2.07% |
| sequential | 100211ns | 100211ns | +40.53% |
| std_parallel | 107885ns | 107885ns | +51.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 71798ns | base | --- | [70825, 72504] | --- | --- | --- | --- |
| head_tail_dyn | 137038ns | +65243.8ns (+90.9%) | [+63844, +66656]ns | [136550, 137442] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 56725ns | -13883.3ns (-19.3%) | [-16538, -12750]ns | [55802, 58150] | YES | 0.0002 | 0.0001 | 0 |
| kernel_seq | 95652ns | +23472.9ns (+32.7%) | [+22906, +25381]ns | [95046, 95948] | YES | 0.0000 | 0.0000 | 0 |
| nway | 72323ns | no significant difference | [-1298, +2304]ns | [70377, 73427] | no | 0.4807 | 0.4807 | 0 |
| sequential | 99477ns | +27789.6ns (+38.7%) | [+26796, +29550]ns | [98754, 100688] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 102971ns | +33002.1ns (+46.0%) | [+29742, +39471]ns | [101683, 107910] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 71656ns | +92.0% | -20.6% | +33.1% | +2.2% | +39.2% | +43.5% |
| 1 | 2 | 72154ns | +89.5% | -23.6% | +31.9% | -2.4% | +36.7% | +49.0% |
| 1 | 3 | 71958ns | +90.7% | -20.7% | +33.2% | +1.1% | +40.2% | +41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.425 | moderate- |
| head_tail_dyn | -0.571 | HIGH- (thermal bounce) |
| kernel_nway | -0.663 | HIGH- (thermal bounce) |
| kernel_seq | -0.391 | moderate- |
| nway | -0.653 | HIGH- (thermal bounce) |
| sequential | -0.503 | HIGH- (thermal bounce) |
| std_parallel | -0.656 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 138999.3ns | 70644.2ns | 196.8% | HIGH |
| head_tail_dyn | 136391.0ns | 136957.9ns | 99.6% | HIGH |
| kernel_nway | 114440.3ns | 56928.7ns | 201.0% | HIGH |
| kernel_seq | 96109.5ns | 95825.0ns | 100.3% | HIGH |
| nway | 139818.3ns | 72200.9ns | 193.7% | HIGH |
| sequential | 100308.8ns | 99923.1ns | 100.4% | HIGH |
| std_parallel | 106867.8ns | 107471.5ns | 99.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 62898.6-73421.5 ns)
  62898.6 |########
  63424.8 |
  63950.9 |
  64477.1 |
  65003.2 |
  65529.4 |
  66055.5 |
  66581.6 |
  67107.8 |
  67633.9 |
  68160.1 |
  68686.2 |################
  69212.4 |
  69738.5 |
  70264.7 |########
  70790.8 |########
  71317.0 |########################
  71843.1 |################
  72369.2 |########################################
  72895.4 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 134977.8-138133.3 ns)
  134977.8 |
  135135.6 |
  135293.4 |
  135451.1 |
  135608.9 |
  135766.7 |#############
  135924.5 |#############
  136082.2 |#############
  136240.0 |
  136397.8 |#############
  136555.6 |#############
  136713.3 |#############
  136871.1 |##########################
  137028.9 |##########################
  137186.7 |
  137344.5 |########################################
  137502.2 |
  137660.0 |#############
  137817.8 |#############
  137975.6 |
  (1 below, 2 above range)

kernel_nway (n=18, range 53704.2-59400.0 ns)
  53704.2 |
  53989.0 |####################
  54273.7 |########################################
  54558.5 |
  54843.3 |
  55128.1 |
  55412.9 |
  55697.7 |########################################
  55982.5 |####################
  56267.3 |####################
  56552.1 |########################################
  56836.9 |####################
  57121.7 |
  57406.4 |
  57691.2 |
  57976.0 |########################################
  58260.8 |####################
  58545.6 |####################
  58830.4 |
  59115.2 |
  (1 below, 3 above range)

kernel_seq (n=18, range 94698.6-97008.3 ns)
  94698.6 |####################
  94814.1 |
  94929.6 |########################################
  95045.1 |####################
  95160.5 |####################
  95276.0 |
  95391.5 |
  95507.0 |####################
  95622.5 |########################################
  95738.0 |########################################
  95853.5 |####################
  95969.0 |####################
  96084.4 |
  96199.9 |
  96315.4 |
  96430.9 |####################
  96546.4 |
  96661.9 |
  96777.4 |
  96892.9 |
  (2 below, 3 above range)

nway (n=18, range 68862.5-74742.4 ns)
  68862.5 |#############
  69156.5 |
  69450.5 |
  69744.5 |#############
  70038.5 |#############
  70332.5 |#############
  70626.5 |
  70920.5 |#############
  71214.5 |
  71508.5 |
  71802.4 |##########################
  72096.4 |
  72390.4 |#############
  72684.4 |
  72978.4 |########################################
  73272.4 |
  73566.4 |##########################
  73860.4 |#############
  74154.4 |#############
  74448.4 |
  (2 below, 1 above range)

sequential (n=18, range 98579.2-101556.2 ns)
  98579.2 |########################################
  98728.0 |########################################
  98876.9 |########################################
  99025.7 |####################
  99174.6 |
  99323.4 |
  99472.3 |
  99621.1 |
  99770.0 |####################
  99918.8 |
  100067.7 |####################
  100216.6 |
  100365.4 |####################
  100514.3 |
  100663.1 |####################
  100812.0 |####################
  100960.8 |########################################
  101109.7 |
  101258.5 |
  101407.4 |
  (2 below, 2 above range)

std_parallel (n=18, range 98381.9-118891.0 ns)
  98381.9 |
  99407.4 |######
  100432.8 |
  101458.3 |########################################
  102483.7 |
  103509.2 |######
  104534.6 |######
  105560.1 |######
  106585.5 |######
  107611.0 |######
  108636.5 |######
  109661.9 |
  110687.4 |
  111712.8 |######
  112738.3 |
  113763.7 |
  114789.2 |
  115814.6 |
  116840.1 |
  117865.5 |######
  (2 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=199.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=201.6% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.3% of algo (FFI overhead may distort results)
- **nway**: bridge=193.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.8% of algo (FFI overhead may distort results)
