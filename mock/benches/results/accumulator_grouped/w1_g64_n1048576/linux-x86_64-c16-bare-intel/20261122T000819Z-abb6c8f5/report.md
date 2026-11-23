# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (3.43 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 3.23 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Top two (sequential, per_core) are a dead heat (<1%)

sequential (3.23 ms) and per_core (3.24 ms) differ by 0.10%, inside the noise, even though the wider field spreads 6.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### grouped shows alternating (throttle bounce) (autocorr -0.67)

grouped's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sequential** at 3233918.2 ns median (-5.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.06x (fastest 3233918.2 ns, slowest 3432064.5 ns)

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
| Counter frequency | 2112760367 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 3429546ns | 3432779ns | 3405998ns | 3426829ns | 3445395ns | base |
| per_core | 3299967ns | 3238557ns | 3214343ns | 3232599ns | 3443830ns | -3.78% |
| sequential | 3237608ns | 3235660ns | 3212202ns | 3231826ns | 3258985ns | -5.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 3428761ns | 3405295ns | 3444548ns | base | 0.306 |
| per_core | 3298038ns | 3213336ns | 3440868ns | -3.81% | 0.318 |
| sequential | 3235708ns | 3211421ns | 3256091ns | -5.63% | 0.324 |

## Performance model

- Peak throughput: **0.327 Gops/s** (sequential; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.306 | 93.6% |
| per_core | 0.324 | 99.2% |
| sequential | 0.324 | 99.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 3429546ns | 3429546ns | base |
| per_core | 3299967ns | 3299967ns | -3.78% |
| sequential | 3237608ns | 3237608ns | -5.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 3432065ns | base | --- | [3418090, 3437951] | --- | --- | --- | --- |
| per_core | 3237083ns | -194385.7ns (-5.7%) | [-207352, -171678]ns | [3220978, 3251043] | YES | 0.0001 | 0.0001 | 0 |
| sequential | 3233918ns | -194721.9ns (-5.7%) | [-202345, -186203]ns | [3223170, 3245209] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 3427260ns | -5.9% | -5.7% |
| 1 | 2 | 3433512ns | -5.3% | -6.0% |
| 1 | 3 | 3427058ns | -5.7% | -5.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.666 | HIGH- (thermal bounce) |
| per_core | -0.636 | HIGH- (thermal bounce) |
| sequential | -0.361 | moderate- |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 3593498.5ns | 3428761.0ns | 104.8% | HIGH |
| per_core | 3322731.6ns | 3298038.2ns | 100.7% | HIGH |
| sequential | 3235060.7ns | 3235708.2ns | 100.0% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 3405295.1-3444548.1 ns)
  3405295.1 |
  3407257.8 |
  3409220.4 |
  3411183.1 |
  3413145.7 |
  3415108.4 |########################################
  3417071.0 |
  3419033.7 |####################
  3420996.3 |
  3422959.0 |
  3424921.6 |##########
  3426884.3 |
  3428846.9 |##########
  3430809.6 |
  3432772.2 |##########
  3434734.9 |####################
  3436697.5 |####################
  3438660.2 |
  3440622.8 |##########
  3442585.5 |####################
  (1 below, 1 above range)

per_core (n=18, range 3213336.2-3440868.0 ns)
  3213336.2 |########################################
  3224712.8 |########################
  3236089.3 |########################
  3247465.9 |########################
  3258842.5 |
  3270219.1 |################
  3281595.7 |
  3292972.3 |
  3304348.9 |
  3315725.5 |
  3327102.1 |
  3338478.7 |
  3349855.3 |
  3361231.9 |
  3372608.5 |
  3383985.0 |
  3395361.6 |
  3406738.2 |
  3418114.8 |
  3429491.4 |
  (1 below, 1 above range)

sequential (n=18, range 3211421.0-3256090.9 ns)
  3211421.0 |####################
  3213654.5 |####################
  3215888.0 |
  3218121.5 |####################
  3220355.0 |####################
  3222588.5 |####################
  3224822.0 |########################################
  3227055.5 |####################
  3229288.9 |
  3231522.4 |
  3233755.9 |
  3235989.4 |
  3238222.9 |####################
  3240456.4 |####################
  3242689.9 |########################################
  3244923.4 |####################
  3247156.9 |
  3249390.4 |####################
  3251623.9 |
  3253857.4 |####################
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=104.9% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
