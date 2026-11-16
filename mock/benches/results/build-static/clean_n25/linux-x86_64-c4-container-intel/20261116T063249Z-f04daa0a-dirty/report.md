# build-static: every crate built into an empty target directory

2 variants, 3 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### llvm_non_incremental is fastest but the noisiest (CV 7.3%)

llvm_non_incremental wins on median (15672.26 ms) yet has the highest variance (CV 7.3%), while llvm_incremental is the steadiest (CV 0.9%, 16864.64 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### llvm_non_incremental shows alternating (throttle bounce) (autocorr -0.65)

llvm_non_incremental's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (15672.26 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader llvm_non_incremental vs stability leader llvm_incremental (+8% speed for 8.2x steadier)

llvm_non_incremental is fastest (15672.26 ms, CV 7.3%); llvm_incremental gives up 7.6% median for 8.2x lower variance (CV 0.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 15672263159.2 ns median
- Spread: 1.08x (fastest 15672263159.2 ns, slowest 16864642120.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 3 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2300379024 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 16767806349ns | 16864652621ns | 16554493017ns | -0ns | 16767806349ns | +2.93% |
| llvm_non_incremental | 16289704513ns | 15672270738ns | 15298281985ns | -0ns | 16289704513ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 16767794776ns | 16554482158ns | 16767794776ns | +2.93% |
| llvm_non_incremental | 16289694337ns | 15298271742ns | 16289694337ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 16767806349ns | 16767806349ns | +2.93% |
| llvm_non_incremental | 16289704513ns | 16289704513ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 15672263159ns | base | --- | [15298271742, 17898548109] | --- | --- | --- | --- |
| llvm_incremental | 16864642120ns | no significant difference | [-1033905988, +1256210416]ns | [16554482158, 16884260051] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 15298271742ns | +8.2% |
| 1 | 2 | 17898548109ns | -5.8% |
| 1 | 3 | 15672263159ns | +7.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.137 | ok |
| llvm_non_incremental | -0.655 | HIGH- (thermal bounce) |

**Consistency summary:**

- **llvm_incremental**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 63973652.4ns | 16767794776.2ns | 0.4% |  |
| llvm_non_incremental | 41895522.4ns | 16289694336.6ns | 0.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=3, range 16554482157.6-16767794776.2 ns)
  16554482157.6 |########################################
  16565147788.5 |
  16575813419.5 |
  16586479050.4 |
  16597144681.3 |
  16607810312.2 |
  16618475943.2 |
  16629141574.1 |
  16639807205.0 |
  16650472836.0 |
  16661138466.9 |
  16671804097.8 |
  16682469728.7 |
  16693135359.7 |
  16703800990.6 |
  16714466621.5 |
  16725132252.5 |
  16735797883.4 |
  16746463514.3 |
  16757129145.2 |
  (0 below, 2 above range)

llvm_non_incremental (n=3, range 15298271742.0-16289694336.6 ns)
  15298271742.0 |########################################
  15347842871.7 |
  15397414001.5 |
  15446985131.2 |
  15496556260.9 |
  15546127390.7 |
  15595698520.4 |
  15645269650.1 |########################################
  15694840779.9 |
  15744411909.6 |
  15793983039.3 |
  15843554169.0 |
  15893125298.8 |
  15942696428.5 |
  15992267558.2 |
  16041838688.0 |
  16091409817.7 |
  16140980947.4 |
  16190552077.2 |
  16240123206.9 |
  (0 below, 1 above range)

```
