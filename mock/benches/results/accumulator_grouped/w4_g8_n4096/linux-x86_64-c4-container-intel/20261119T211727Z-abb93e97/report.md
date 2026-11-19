# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 44% faster than the next best (grouped)

sequential (31.46 us) leads grouped (45.46 us) by 44%, a clear separation rather than a photo finish. CV 51.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 29% (significant)

sequential is -13.20 us (29%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is fastest but the noisiest (CV 51.4%)

sequential wins on median (31.46 us) yet has the highest variance (CV 51.4%), while per_core is the steadiest (CV 20.5%, 48.63 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### sequential is inconsistent: worst-20% is 2.0x its best-20%

sequential's best 20% of batches run at 25.91 us but its worst 20% at 51.77 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 31464.9 ns median (-30.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.55x (fastest 31464.9 ns, slowest 48629.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 49183ns | 45924ns | 42137ns | 45033ns | 58930ns | base |
| per_core | 50250ns | 49080ns | 39668ns | 47334ns | 59916ns | +2.17% |
| sequential | 37445ns | 31964ns | 26469ns | 31044ns | 52535ns | -23.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 48586ns | 41693ns | 58291ns | base | 0.084 |
| per_core | 49619ns | 38988ns | 59194ns | +2.13% | 0.083 |
| sequential | 36773ns | 25915ns | 51766ns | -24.31% | 0.111 |

## Performance model

- Peak throughput: **0.158 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.090 | 57.0% |
| per_core | 0.084 | 53.3% |
| sequential | 0.130 | 82.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 49183ns | 49183ns | base |
| per_core | 50250ns | 50250ns | +2.17% |
| sequential | 37445ns | 37445ns | -23.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 45461ns | base | --- | [43188, 47569] | --- | --- | --- | --- |
| per_core | 48630ns | no significant difference | [-3855, +6689]ns | [44811, 50932] | no | 0.4807 | 0.4807 | 0 |
| sequential | 31465ns | -13204.8ns (-29.0%) | [-17603, -7086]ns | [27940, 37938] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 46052ns | -5.3% | -40.7% |
| 1 | 2 | 42393ns | +7.6% | -10.5% |
| 1 | 3 | 45765ns | +11.3% | -23.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.663 | HIGH- (thermal bounce) |
| per_core | -0.042 | ok |
| sequential | -0.350 | moderate- |

**Consistency summary:**

- **per_core**: won 1/3, lost 2/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 48771.2ns | 48585.8ns | 100.4% | HIGH |
| per_core | 50647.0ns | 49618.9ns | 102.1% | HIGH |
| sequential | 39000.7ns | 36772.7ns | 106.1% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 41693.3-58291.2 ns)
  41693.3 |########################################
  42523.2 |#############
  43353.1 |##########################
  44183.0 |##########################
  45012.9 |
  45842.8 |########################################
  46672.7 |#############
  47502.6 |##########################
  48332.5 |
  49162.4 |
  49992.3 |
  50822.2 |
  51652.1 |
  52482.0 |
  53311.9 |
  54141.8 |
  54971.7 |#############
  55801.5 |
  56631.4 |
  57461.3 |
  (1 below, 2 above range)

per_core (n=18, range 38988.5-59193.8 ns)
  38988.5 |
  39998.7 |
  41009.0 |##########################
  42019.3 |
  43029.5 |#############
  44039.8 |#############
  45050.1 |########################################
  46060.3 |
  47070.6 |
  48080.9 |##########################
  49091.1 |##########################
  50101.4 |#############
  51111.7 |#############
  52121.9 |#############
  53132.2 |#############
  54142.5 |
  55152.7 |
  56163.0 |
  57173.3 |
  58183.5 |
  (1 below, 2 above range)

sequential (n=18, range 25914.6-51765.8 ns)
  25914.6 |########################################
  27207.2 |##########################
  28499.7 |#############
  29792.3 |#############
  31084.8 |##########################
  32377.4 |#############
  33670.0 |
  34962.5 |#############
  36255.1 |
  37547.6 |#############
  38840.2 |#############
  40132.8 |#############
  41425.3 |
  42717.9 |
  44010.4 |
  45303.0 |
  46595.6 |##########################
  47888.1 |
  49180.7 |
  50473.2 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=20.8% (high variance, measurements may be unstable)
- **grouped**: bridge=105.5% of algo (FFI overhead may distort results)
- **per_core**: CV=20.1% (high variance, measurements may be unstable)
- **per_core**: bridge=94.8% of algo (FFI overhead may distort results)
- **sequential**: CV=44.0% (high variance, measurements may be unstable)
- **sequential**: bridge=100.0% of algo (FFI overhead may distort results)
