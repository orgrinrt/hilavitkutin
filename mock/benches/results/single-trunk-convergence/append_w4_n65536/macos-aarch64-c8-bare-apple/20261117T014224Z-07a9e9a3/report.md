# append, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway is an outlier: 6.3x slower than the field

nway (60.61 us) is 6.3x the fastest (9.55 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (head_tail_dyn, head_tail) are a dead heat (<1%)

head_tail_dyn (9.55 us) and head_tail (9.57 us) differ by 0.28%, inside the noise, even though the wider field spreads 534.8%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### nway shows alternating (throttle bounce) (autocorr -0.65)

nway's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 6.3x the fastest

Fastest head_tail_dyn (9.55 us) to slowest nway (60.61 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### kernel_nway is inconsistent: worst-20% is 1.5x its best-20%

kernel_nway's best 20% of batches run at 16.54 us but its worst 20% at 25.27 us (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: head_tail_dyn** at 9547.9 ns median (-0.3% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 6.35x (fastest 9547.9 ns, slowest 60606.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 4 |
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
| head_tail | 10446ns | 10410ns | 9921ns | 10347ns | 10858ns | base |
| head_tail_dyn | 10271ns | 10233ns | 10017ns | 10198ns | 10508ns | -1.68% |
| kernel_nway | 22016ns | 21773ns | 17471ns | 20916ns | 25939ns | +110.75% |
| kernel_seq | 10296ns | 10360ns | 9847ns | 10290ns | 10530ns | -1.44% |
| nway | 61417ns | 61225ns | 57150ns | 60805ns | 64470ns | +487.93% |
| sequential | 56038ns | 55948ns | 55354ns | 55821ns | 56705ns | +436.44% |
| std_parallel | 53737ns | 54246ns | 49524ns | 52881ns | 57129ns | +414.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 9693ns | 9461ns | 10024ns | base | 6.761 |
| head_tail_dyn | 9559ns | 9443ns | 9657ns | -1.39% | 6.856 |
| kernel_nway | 21162ns | 16539ns | 25269ns | +118.32% | 3.097 |
| kernel_seq | 9585ns | 9456ns | 9692ns | -1.12% | 6.838 |
| nway | 60645ns | 56525ns | 63592ns | +525.64% | 1.081 |
| sequential | 55515ns | 54821ns | 56065ns | +472.72% | 1.181 |
| std_parallel | 52694ns | 48569ns | 56238ns | +443.62% | 1.244 |

## Performance model

- Peak throughput: **6.940 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 6.844 | 98.6% |
| head_tail_dyn | 6.864 | 98.9% |
| kernel_nway | 3.153 | 45.4% |
| kernel_seq | 6.836 | 98.5% |
| nway | 1.081 | 15.6% |
| sequential | 1.180 | 17.0% |
| std_parallel | 1.233 | 17.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 10446ns | 10446ns | base |
| head_tail_dyn | 10271ns | 10271ns | -1.68% |
| kernel_nway | 22016ns | 22016ns | +110.75% |
| kernel_seq | 10296ns | 10296ns | -1.44% |
| nway | 61417ns | 61417ns | +487.93% |
| sequential | 56038ns | 56038ns | +436.44% |
| std_parallel | 53737ns | 53737ns | +414.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 9575ns | base | --- | [9498, 9638] | --- | --- | --- | --- |
| head_tail_dyn | 9548ns | no significant difference | [-148, +67]ns | [9504, 9596] | no | 0.9774 | 0.8145 | 0 |
| kernel_nway | 20785ns | +11131.2ns (+116.3%) | [+9329, +13715]ns | [18952, 23596] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 9588ns | no significant difference | [-119, +79]ns | [9521, 9633] | no | 1.0000 | 1.0000 | 0 |
| nway | 60606ns | +51041.7ns (+533.1%) | [+49425, +52181]ns | [59092, 61812] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 55550ns | +45789.6ns (+478.2%) | [+45463, +46098]ns | [55062, 55783] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 53142ns | +43481.2ns (+454.1%) | [+40619, +45062]ns | [50179, 55106] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 9523ns | +0.8% | +82.5% | +0.9% | +544.3% | +480.0% | +462.8% |
| 1 | 2 | 9725ns | -2.0% | +120.7% | -2.0% | +505.2% | +471.8% | +463.8% |
| 1 | 3 | 9598ns | -1.0% | +159.8% | -0.1% | +544.1% | +479.3% | +431.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.577 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.030 | ok |
| kernel_nway | -0.001 | ok |
| kernel_seq | -0.610 | HIGH- (thermal bounce) |
| nway | -0.653 | HIGH- (thermal bounce) |
| sequential | -0.175 | ok |
| std_parallel | -0.374 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 442525.7ns | 9693.3ns | 4565.3% | HIGH |
| head_tail_dyn | 441961.6ns | 9559.0ns | 4623.5% | HIGH |
| kernel_nway | 449824.1ns | 21162.5ns | 2125.6% | HIGH |
| kernel_seq | 440891.7ns | 9584.7ns | 4599.9% | HIGH |
| nway | 474357.9ns | 60645.4ns | 782.2% | HIGH |
| sequential | 461420.8ns | 55515.0ns | 831.2% | HIGH |
| std_parallel | 470643.5ns | 52694.2ns | 893.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 9461.1-10024.3 ns)
   9461.1 |##########################
   9489.3 |########################################
   9517.5 |
   9545.6 |########################################
   9573.8 |##########################
   9601.9 |##########################
   9630.1 |#############
   9658.2 |
   9686.4 |
   9714.6 |
   9742.7 |#############
   9770.9 |
   9799.0 |
   9827.2 |
   9855.4 |
   9883.5 |#############
   9911.7 |
   9939.8 |
   9968.0 |
   9996.1 |
  (1 below, 2 above range)

head_tail_dyn (n=18, range 9443.1-9656.9 ns)
   9443.1 |
   9453.8 |#############
   9464.5 |
   9475.1 |
   9485.8 |########################################
   9496.5 |
   9507.2 |##########################
   9517.9 |
   9528.6 |##########################
   9539.3 |
   9550.0 |
   9560.7 |##########################
   9571.4 |
   9582.1 |##########################
   9592.8 |##########################
   9603.5 |
   9614.2 |
   9624.9 |
   9635.6 |
   9646.3 |
  (1 below, 3 above range)

kernel_nway (n=18, range 16538.9-25268.7 ns)
  16538.9 |####################
  16975.4 |####################
  17411.9 |####################
  17848.4 |
  18284.9 |
  18721.4 |########################################
  19157.8 |####################
  19594.3 |
  20030.8 |########################################
  20467.3 |
  20903.8 |########################################
  21340.3 |####################
  21776.8 |
  22213.3 |
  22649.8 |
  23086.3 |
  23522.8 |####################
  23959.3 |####################
  24395.8 |
  24832.2 |
  (1 below, 4 above range)

kernel_seq (n=18, range 9455.6-9691.7 ns)
   9455.6 |
   9467.4 |
   9479.2 |########################################
   9491.0 |
   9502.8 |
   9514.6 |########################################
   9526.4 |####################
   9538.2 |####################
   9550.0 |
   9561.8 |####################
   9573.6 |
   9585.4 |
   9597.2 |########################################
   9609.0 |####################
   9620.8 |####################
   9632.6 |####################
   9644.4 |
   9656.3 |
   9668.1 |
   9679.9 |####################
  (2 below, 3 above range)

nway (n=18, range 56525.0-63591.7 ns)
  56525.0 |
  56878.3 |
  57231.7 |
  57585.0 |
  57938.3 |
  58291.7 |########################################
  58645.0 |####################
  58998.3 |####################
  59351.7 |####################
  59705.0 |####################
  60058.3 |####################
  60411.7 |
  60765.0 |####################
  61118.3 |####################
  61471.7 |########################################
  61825.0 |####################
  62178.3 |####################
  62531.7 |
  62885.0 |
  63238.3 |
  (2 below, 3 above range)

sequential (n=18, range 54820.8-56065.3 ns)
  54820.8 |
  54883.0 |
  54945.2 |#############
  55007.5 |#############
  55069.7 |#############
  55131.9 |
  55194.1 |
  55256.4 |
  55318.6 |
  55380.8 |#############
  55443.0 |
  55505.3 |##########################
  55567.5 |##########################
  55629.7 |
  55691.9 |##########################
  55754.2 |
  55816.4 |########################################
  55878.6 |#############
  55940.8 |
  56003.1 |
  (3 below, 1 above range)

std_parallel (n=18, range 48569.4-56237.5 ns)
  48569.4 |
  48952.8 |#############
  49336.2 |#############
  49719.6 |#############
  50103.0 |
  50486.5 |##########################
  50869.9 |
  51253.3 |#############
  51636.7 |
  52020.1 |
  52403.5 |#############
  52786.9 |
  53170.3 |
  53553.7 |########################################
  53937.1 |#############
  54320.5 |
  54703.9 |
  55087.3 |
  55470.7 |
  55854.1 |########################################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=4619.3% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=4629.1% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=2166.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=4598.2% of algo (FFI overhead may distort results)
- **nway**: bridge=774.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=828.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=885.4% of algo (FFI overhead may distort results)
