# fold_heavy, W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (head_tail_dyn, kernel_nway) are a dead heat (<1%)

head_tail_dyn (52.08 us) and kernel_nway (52.11 us) differ by 0.06%, inside the noise, even though the wider field spreads 33.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: head_tail_dyn** at 52081.2 ns median (-1.2% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.33x (fastest 52081.2 ns, slowest 69272.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 1 |
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
| head_tail | 53052ns | 52927ns | 52128ns | 52754ns | 53960ns | base |
| head_tail_dyn | 52278ns | 52273ns | 52039ns | 52238ns | 52458ns | -1.46% |
| kernel_nway | 52586ns | 52306ns | 52026ns | 52259ns | 53357ns | -0.88% |
| kernel_seq | 52867ns | 52319ns | 52064ns | 52313ns | 54099ns | -0.35% |
| nway | 61874ns | 61865ns | 61693ns | 61840ns | 62015ns | +16.63% |
| sequential | 56633ns | 56235ns | 55839ns | 56194ns | 57689ns | +6.75% |
| std_parallel | 70681ns | 69508ns | 68988ns | 69538ns | 73242ns | +33.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 52823ns | 51935ns | 53692ns | base | 1.241 |
| head_tail_dyn | 52096ns | 51853ns | 52274ns | -1.38% | 1.258 |
| kernel_nway | 52379ns | 51843ns | 53119ns | -0.84% | 1.251 |
| kernel_seq | 52656ns | 51857ns | 53842ns | -0.32% | 1.245 |
| nway | 61678ns | 61499ns | 61819ns | +16.76% | 1.063 |
| sequential | 56453ns | 55669ns | 57499ns | +6.87% | 1.161 |
| std_parallel | 70429ns | 68762ns | 72948ns | +33.33% | 0.931 |

## Performance model

- Peak throughput: **1.264 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.244 | 98.4% |
| head_tail_dyn | 1.258 | 99.5% |
| kernel_nway | 1.258 | 99.5% |
| kernel_seq | 1.257 | 99.4% |
| nway | 1.063 | 84.1% |
| sequential | 1.169 | 92.5% |
| std_parallel | 0.946 | 74.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 53052ns | 53052ns | base |
| head_tail_dyn | 52278ns | 52278ns | -1.46% |
| kernel_nway | 52586ns | 52586ns | -0.88% |
| kernel_seq | 52867ns | 52867ns | -0.35% |
| nway | 61874ns | 61874ns | +16.63% |
| sequential | 56633ns | 56633ns | +6.75% |
| std_parallel | 70681ns | 70681ns | +33.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 52690ns | base | --- | [52308, 53215] | --- | --- | --- | --- |
| head_tail_dyn | 52081ns | -531.2ns (-1.0%) | [-1112, -171]ns | [52008, 52183] | YES | 0.0463 | 0.0309 | 0 |
| kernel_nway | 52110ns | no significant difference | [-979, +183]ns | [51967, 52638] | no | 0.2379 | 0.2379 | 0 |
| kernel_seq | 52140ns | no significant difference | [-958, +75]ns | [52033, 52667] | no | 0.1155 | 0.0963 | 0 |
| nway | 61675ns | +8933.3ns (+17.0%) | [+8617, +9235]ns | [61602, 61744] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 56073ns | +3529.2ns (+6.7%) | [+3192, +4356]ns | [55685, 56835] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 69273ns | +16500.0ns (+31.3%) | [+16346, +17771]ns | [69123, 70062] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 52448ns | -0.8% | -0.7% | +0.9% | +17.5% | +6.1% | +35.1% |
| 1 | 2 | 53006ns | -1.7% | -1.5% | -1.7% | +16.3% | +5.8% | +30.6% |
| 1 | 3 | 52375ns | -0.5% | -0.4% | -0.7% | +17.9% | +8.7% | +32.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.659 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.051 | ok |
| kernel_nway | -0.201 | moderate- |
| kernel_seq | -0.134 | ok |
| nway | -0.094 | ok |
| sequential | -0.027 | ok |
| std_parallel | -0.155 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 3/3, lost 0/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 105713.4ns | 52822.9ns | 200.1% | HIGH |
| head_tail_dyn | 104199.5ns | 52096.1ns | 200.0% | HIGH |
| kernel_nway | 104786.3ns | 52379.4ns | 200.1% | HIGH |
| kernel_seq | 105177.5ns | 52656.3ns | 199.7% | HIGH |
| nway | 123330.3ns | 61677.5ns | 200.0% | HIGH |
| sequential | 113145.8ns | 56452.5ns | 200.4% | HIGH |
| std_parallel | 140115.3ns | 70428.9ns | 198.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 51934.7-53692.3 ns)
  51934.7 |
  52022.6 |
  52110.5 |##########################
  52198.4 |#############
  52286.3 |#############
  52374.1 |
  52462.0 |#############
  52549.9 |#############
  52637.8 |########################################
  52725.7 |
  52813.5 |
  52901.4 |#############
  52989.3 |
  53077.2 |#############
  53165.1 |
  53252.9 |#############
  53340.8 |
  53428.7 |#############
  53516.6 |
  53604.5 |#############
  (2 below, 2 above range)

head_tail_dyn (n=18, range 51852.8-52274.3 ns)
  51852.8 |
  51873.8 |
  51894.9 |
  51916.0 |####################
  51937.1 |
  51958.2 |####################
  51979.2 |####################
  52000.3 |####################
  52021.4 |####################
  52042.5 |
  52063.5 |########################################
  52084.6 |####################
  52105.7 |
  52126.8 |########################################
  52147.9 |####################
  52168.9 |
  52190.0 |####################
  52211.1 |########################################
  52232.2 |
  52253.2 |
  (2 below, 2 above range)

kernel_nway (n=18, range 51843.1-53119.5 ns)
  51843.1 |##########################
  51906.9 |#############
  51970.7 |########################################
  52034.5 |#############
  52098.3 |#############
  52162.2 |#############
  52226.0 |#############
  52289.8 |
  52353.6 |
  52417.4 |#############
  52481.3 |
  52545.1 |
  52608.9 |
  52672.7 |
  52736.5 |
  52800.4 |#############
  52864.2 |
  52928.0 |
  52991.8 |#############
  53055.6 |
  (2 below, 3 above range)

kernel_seq (n=18, range 51857.0-53842.4 ns)
  51857.0 |##########
  51956.2 |########################################
  52055.5 |##############################
  52154.8 |##########
  52254.0 |
  52353.3 |
  52452.6 |##########
  52551.9 |##########
  52651.1 |####################
  52750.4 |
  52849.7 |
  52948.9 |
  53048.2 |##########
  53147.5 |
  53246.7 |
  53346.0 |
  53445.3 |
  53544.6 |
  53643.8 |
  53743.1 |
  (2 below, 2 above range)

nway (n=18, range 61498.6-61819.5 ns)
  61498.6 |
  61514.6 |
  61530.7 |
  61546.7 |########################################
  61562.8 |
  61578.8 |####################
  61594.9 |
  61610.9 |####################
  61626.9 |####################
  61643.0 |
  61659.0 |########################################
  61675.1 |########################################
  61691.1 |####################
  61707.2 |####################
  61723.2 |
  61739.2 |
  61755.3 |####################
  61771.3 |
  61787.4 |####################
  61803.4 |
  (2 below, 3 above range)

sequential (n=18, range 55669.4-57498.6 ns)
  55669.4 |########################################
  55760.9 |
  55852.4 |
  55943.8 |################
  56035.3 |################
  56126.7 |
  56218.2 |
  56309.6 |########
  56401.1 |
  56492.6 |
  56584.0 |########
  56675.5 |########
  56766.9 |
  56858.4 |
  56949.9 |########
  57041.3 |########
  57132.8 |########
  57224.2 |
  57315.7 |
  57407.2 |
  (1 below, 2 above range)

std_parallel (n=18, range 68762.5-72947.9 ns)
  68762.5 |##########
  68971.8 |########################################
  69181.0 |########################################
  69390.3 |
  69599.6 |##########
  69808.9 |####################
  70018.1 |##########
  70227.4 |
  70436.7 |
  70645.9 |
  70855.2 |
  71064.5 |
  71273.8 |##########
  71483.0 |
  71692.3 |
  71901.6 |##########
  72110.8 |
  72320.1 |
  72529.4 |
  72738.7 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=200.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=199.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=200.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=200.4% of algo (FFI overhead may distort results)
- **nway**: bridge=200.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=200.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=199.7% of algo (FFI overhead may distort results)
