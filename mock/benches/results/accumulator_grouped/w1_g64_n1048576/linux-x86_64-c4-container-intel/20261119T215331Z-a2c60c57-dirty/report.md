# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (4.89 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 4.57 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### per_core shows alternating (throttle bounce) (autocorr -0.55)

per_core's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 4569375.7 ns median (-6.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.07x (fastest 4569375.7 ns, slowest 4892411.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 4938472ns | 4894529ns | 4523804ns | 4837164ns | 5297767ns | base |
| per_core | 4572712ns | 4571411ns | 4281227ns | 4549872ns | 4752716ns | -7.41% |
| sequential | 4882001ns | 4765131ns | 4617763ns | 4763689ns | 5191588ns | -1.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 4936170ns | 4522030ns | 5294790ns | base | 0.212 |
| per_core | 4570159ns | 4278545ns | 4749487ns | -7.41% | 0.229 |
| sequential | 4879586ns | 4615376ns | 5188883ns | -1.15% | 0.215 |

## Performance model

- Peak throughput: **0.245 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.214 | 87.5% |
| per_core | 0.229 | 93.6% |
| sequential | 0.220 | 89.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 4938472ns | 4938472ns | base |
| per_core | 4572712ns | 4572712ns | -7.41% |
| sequential | 4882001ns | 4882001ns | -1.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 4892411ns | base | --- | [4755168, 5075930] | --- | --- | --- | --- |
| per_core | 4569376ns | -309682.5ns (-6.3%) | [-647063, -83294]ns | [4509927, 4661423] | YES | 0.0151 | 0.0075 | 0 |
| sequential | 4762829ns | no significant difference | [-191962, +53183]ns | [4717302, 4958363] | no | 0.4807 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 5186670ns | -12.1% | -5.2% |
| 1 | 2 | 4851172ns | -9.1% | -0.8% |
| 1 | 3 | 4725912ns | -1.4% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.043 | ok |
| per_core | -0.553 | HIGH- (thermal bounce) |
| sequential | -0.000 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 5071671.5ns | 4936169.9ns | 102.7% | HIGH |
| per_core | 4575295.4ns | 4570159.0ns | 100.1% | HIGH |
| sequential | 4889047.7ns | 4879585.9ns | 100.2% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 4522030.3-5294789.6 ns)
  4522030.3 |
  4560668.3 |##########################
  4599306.2 |
  4637944.2 |#############
  4676582.2 |#############
  4715220.1 |
  4753858.1 |
  4792496.0 |########################################
  4831134.0 |
  4869772.0 |##########################
  4908409.9 |#############
  4947047.9 |
  4985685.9 |#############
  5024323.8 |
  5062961.8 |##########################
  5101599.8 |
  5140237.7 |
  5178875.7 |#############
  5217513.6 |
  5256151.6 |
  (1 below, 3 above range)

per_core (n=18, range 4278544.8-4749486.8 ns)
  4278544.8 |
  4302091.9 |
  4325639.0 |#############
  4349186.1 |
  4372733.2 |
  4396280.3 |
  4419827.4 |
  4443374.5 |
  4466921.6 |#############
  4490468.7 |##########################
  4514015.8 |
  4537562.9 |##########################
  4561110.0 |########################################
  4584657.1 |
  4608204.2 |##########################
  4631751.3 |
  4655298.4 |
  4678845.5 |#############
  4702392.6 |
  4725939.7 |
  (2 below, 4 above range)

sequential (n=18, range 4615375.6-5188882.6 ns)
  4615375.6 |####################
  4644050.9 |
  4672726.3 |##########
  4701401.6 |##########
  4730077.0 |########################################
  4758752.3 |##########
  4787427.7 |
  4816103.0 |
  4844778.4 |####################
  4873453.7 |
  4902129.1 |
  4930804.4 |##########
  4959479.8 |##########
  4988155.1 |
  5016830.5 |##########
  5045505.8 |
  5074181.2 |
  5102856.5 |
  5131531.9 |
  5160207.2 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: bridge=102.2% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.5% of algo (FFI overhead may distort results)
