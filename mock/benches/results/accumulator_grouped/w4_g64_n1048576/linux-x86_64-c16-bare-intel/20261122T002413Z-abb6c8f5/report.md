# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (4.64 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 3.23 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 18% faster than the next best (per_core)

sequential (3.23 ms) leads per_core (3.82 ms) by 18%, a clear separation rather than a photo finish. CV 2.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 31% (significant)

sequential is -1.42 ms (31%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### per_core is inconsistent: worst-20% is 3.0x its best-20%

per_core's best 20% of batches run at 1.88 ms but its worst 20% at 5.62 ms (3.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 3229524.4 ns median (-30.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.44x (fastest 3229524.4 ns, slowest 4639805.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112755559 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 4861874ns | 4642197ns | 4021575ns | 4608542ns | 5662021ns | base |
| per_core | 4042394ns | 3819557ns | 1882734ns | 3704308ns | 5629355ns | -16.86% |
| sequential | 3253479ns | 3231430ns | 3214435ns | 3228090ns | 3311083ns | -33.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 4859314ns | 4019455ns | 5659291ns | base | 0.216 |
| per_core | 4038406ns | 1880669ns | 5624127ns | -16.89% | 0.260 |
| sequential | 3251825ns | 3213581ns | 3308622ns | -33.08% | 0.322 |

## Performance model

- Peak throughput: **0.558 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.226 | 40.5% |
| per_core | 0.275 | 49.3% |
| sequential | 0.325 | 58.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 4861874ns | 4861874ns | base |
| per_core | 4042394ns | 4042394ns | -16.86% |
| sequential | 3253479ns | 3253479ns | -33.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 4639806ns | base | --- | [4432033, 5358781] | --- | --- | --- | --- |
| per_core | 3815164ns | no significant difference | [-1828246, +355841]ns | [3517642, 4880859] | no | 0.2379 | 0.2379 | 0 |
| sequential | 3229524ns | -1417081.0ns (-30.5%) | [-1988976, -1200595]ns | [3219517, 3244770] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 4504847ns | -7.7% | -28.3% |
| 1 | 2 | 4928248ns | -28.6% | -34.4% |
| 1 | 3 | 4540144ns | -4.1% | -28.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.663 | HIGH- (thermal bounce) |
| per_core | -0.634 | HIGH- (thermal bounce) |
| sequential | -0.600 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 5289399.6ns | 4859313.8ns | 108.9% | HIGH |
| per_core | 3887371.7ns | 4038406.0ns | 96.3% | HIGH |
| sequential | 3246896.9ns | 3251824.8ns | 99.8% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 4019454.6-5659291.4 ns)
  4019454.6 |##########################
  4101446.4 |
  4183438.3 |
  4265430.1 |#############
  4347422.0 |#############
  4429413.8 |#############
  4511405.6 |#############
  4593397.5 |########################################
  4675389.3 |#############
  4757381.2 |
  4839373.0 |
  4921364.8 |
  5003356.7 |
  5085348.5 |#############
  5167340.4 |#############
  5249332.2 |
  5331324.0 |
  5413315.9 |
  5495307.7 |##########################
  5577299.6 |#############
  (1 below, 2 above range)

per_core (n=18, range 1880669.5-5624127.0 ns)
  1880669.5 |
  2067842.3 |########################################
  2255015.2 |
  2442188.1 |####################
  2629361.0 |
  2816533.8 |
  3003706.7 |
  3190879.6 |
  3378052.5 |########################################
  3565225.3 |########################################
  3752398.2 |########################################
  3939571.1 |
  4126744.0 |####################
  4313916.8 |####################
  4501089.7 |
  4688262.6 |####################
  4875435.5 |####################
  5062608.3 |####################
  5249781.2 |
  5436954.1 |
  (1 below, 3 above range)

sequential (n=18, range 3213581.2-3308621.8 ns)
  3213581.2 |########################################
  3218333.3 |##########################
  3223085.3 |########################################
  3227837.3 |#############
  3232589.4 |#############
  3237341.4 |#############
  3242093.4 |#############
  3246845.4 |#############
  3251597.5 |
  3256349.5 |########################################
  3261101.5 |
  3265853.6 |
  3270605.6 |
  3275357.6 |
  3280109.7 |
  3284861.7 |
  3289613.7 |
  3294365.7 |
  3299117.8 |
  3303869.8 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=112.7% of algo (FFI overhead may distort results)
- **per_core**: CV=34.9% (high variance, measurements may be unstable)
- **per_core**: bridge=109.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
