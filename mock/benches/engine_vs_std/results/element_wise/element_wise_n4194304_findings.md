# element_wise, four-stage RAW chain

2 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### engine dominates: 25% faster than the next best (std)

engine (3.90 ms) leads std (4.86 ms) by 25%, a clear separation rather than a photo finish. CV 11.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std shows alternating (throttle bounce) (autocorr -0.50)

std's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (engine)

The baseline engine is the fastest (3.90 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (engine) is the fastest** at 3903607.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.25x (fastest 3903607.0 ns, slowest 4864594.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | element_wise, four-stage RAW chain |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000001 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 4083486ns | 3907409ns | 3828877ns | 3873155ns | 4304886ns | base |
| std | 4934671ns | 4870612ns | 4021800ns | 4375769ns | 5582065ns | +20.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 4078557ns | 3824262ns | 4299348ns | base | 1.028 |
| std | 4928361ns | 4014102ns | 5575786ns | +20.84% | 0.851 |

## Performance model

- Peak throughput: **1.097 Gops/s** (engine; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.074 | 98.0% |
| std | 0.862 | 78.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 4083486ns | 4083486ns | base |
| std | 4934671ns | 4934671ns | +20.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 3903607ns | base | --- | [3851266, 4102819] | --- | --- | --- | --- |
| std | 4864594ns | +534849.0ns (+13.7%) | [+101710, +1973567]ns | [4085857, 5843482] | YES (adj: no) | 0.0703 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | std |
|---|---|---|---|
| 1 | 1 | 3934884ns | +6.1% |
| 1 | 2 | 3895754ns | +46.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| std | -0.500 | moderate- |

**Consistency summary:**

- **std**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 21241477.2ns | 4078557.0ns | 520.8% | HIGH |
| std | 23480361.0ns | 4928360.8ns | 476.4% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 3824261.7-4299348.1 ns)
  3824261.7 |####################
  3848016.0 |########################################
  3871770.3 |####################
  3895524.7 |
  3919279.0 |####################
  3943033.3 |
  3966787.6 |####################
  3990541.9 |
  4014296.2 |
  4038050.6 |
  4061804.9 |
  4085559.2 |####################
  4109313.5 |
  4133067.8 |
  4156822.2 |
  4180576.5 |
  4204330.8 |
  4228085.1 |
  4251839.4 |
  4275593.8 |
  (0 below, 1 above range)

std (n=8, range 4014102.1-5575786.0 ns)
  4014102.1 |########################################
  4092186.3 |
  4170270.5 |
  4248354.7 |####################
  4326438.9 |
  4404523.1 |
  4482607.3 |
  4560691.5 |
  4638775.7 |
  4716859.9 |####################
  4794944.1 |
  4873028.3 |
  4951112.5 |####################
  5029196.7 |
  5107280.8 |
  5185365.0 |
  5263449.2 |
  5341533.4 |
  5419617.6 |
  5497701.8 |
  (0 below, 3 above range)

```

## Diagnostics

- **engine**: bridge=541.2% of algo (FFI overhead may distort results)
- **std**: bridge=475.8% of algo (FFI overhead may distort results)
