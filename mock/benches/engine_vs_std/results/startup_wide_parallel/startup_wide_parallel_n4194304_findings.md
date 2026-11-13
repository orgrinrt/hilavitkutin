# wide_parallel, get-ready

2 variants, 8 samples per variant.
Baseline: **engine_startup**

## Highlights

Baseline for all deltas below: **engine_startup**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine_startup has the worst median (5.59 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_startup at 4.91 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std_startup is fastest but the noisiest (CV 25.0%)

std_startup wins on median (4.91 ms) yet has the highest variance (CV 25.0%), while engine_startup is the steadiest (CV 7.1%, 5.59 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_startup shows alternating (throttle bounce) (autocorr -0.50)

std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (673.48 us) is smaller than the fastest variant's own run-to-run std-dev (1.23 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### std_startup is inconsistent: worst-20% is 1.7x its best-20%

std_startup's best 20% of batches run at 3.80 ms but its worst 20% at 6.32 ms (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_startup** at 4913463.9 ns median (-12.1% vs baseline)
- Spread: 1.14x (fastest 4913463.9 ns, slowest 5586946.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010004 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine_startup | 5583607ns | 5592488ns | 4869139ns | 5359354ns | 5930415ns | base |
| std_startup | 5144149ns | 4919516ns | 3806692ns | 4015308ns | 6325144ns | -7.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine_startup | 5578124ns | 4864064ns | 5924869ns | base | 0.752 |
| std_startup | 5138484ns | 3801744ns | 6318817ns | -7.88% | 0.816 |

## Performance model

- Peak throughput: **1.103 Gops/s** (std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine_startup | 0.751 | 68.0% |
| std_startup | 0.854 | 77.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine_startup | 5583607ns | 5583607ns | base |
| std_startup | 5144149ns | 5144149ns | -7.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine_startup | 5586946ns | base | --- | [5270278, 5909833] | --- | --- | --- | --- |
| std_startup | 4913464ns | no significant difference | [-2108089, +782590]ns | [3936368, 6668398] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine_startup | std_startup |
|---|---|---|---|
| 1 | 1 | 5827218ns | -31.7% |
| 1 | 2 | 5395587ns | +17.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine_startup | -0.500 | moderate- |
| std_startup | -0.500 | moderate- |

**Consistency summary:**

- **std_startup**: won 1/2, lost 1/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine_startup | 193044271.3ns | 5578124.3ns | 3460.7% | HIGH |
| std_startup | 189743188.3ns | 5138483.7ns | 3692.6% | HIGH |

## Distribution (algo ns)

```
engine_startup (n=8, range 4864064.0-5924869.5 ns)
  4864064.0 |####################
  4917104.3 |
  4970144.5 |
  5023184.8 |
  5076225.1 |
  5129265.4 |
  5182305.6 |
  5235345.9 |####################
  5288386.2 |
  5341426.5 |####################
  5394466.7 |####################
  5447507.0 |
  5500547.3 |
  5553587.6 |
  5606627.8 |
  5659668.1 |
  5712708.4 |####################
  5765748.7 |
  5818788.9 |
  5871829.2 |########################################
  (0 below, 1 above range)

std_startup (n=8, range 3801744.3-6318817.5 ns)
  3801744.3 |####################
  3927598.0 |########################################
  4053451.6 |####################
  4179305.3 |
  4305158.9 |
  4431012.6 |
  4556866.3 |
  4682719.9 |
  4808573.6 |
  4934427.2 |
  5060280.9 |
  5186134.6 |
  5311988.2 |
  5437841.9 |
  5563695.5 |
  5689549.2 |####################
  5815402.9 |
  5941256.5 |####################
  6067110.2 |
  6192963.8 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine_startup**: bridge=3429.4% of algo (FFI overhead may distort results)
- **std_startup**: CV=23.9% (high variance, measurements may be unstable)
- **std_startup**: bridge=3872.7% of algo (FFI overhead may distort results)
