# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (46.35 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sequential at 29.60 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sequential dominates: 39% faster than the next best (per_core)

sequential (29.60 us) leads per_core (41.21 us) by 39%, a clear separation rather than a photo finish. CV 10.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 37% (significant)

sequential is -17.15 us (37%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 1.7x its best-20%

grouped's best 20% of batches run at 41.97 us but its worst 20% at 73.35 us (1.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 29602.6 ns median (-36.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.57x (fastest 29602.6 ns, slowest 46348.8 ns)

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
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 55190ns | 47104ns | 42409ns | 46499ns | 74618ns | base |
| per_core | 43176ns | 41636ns | 38772ns | 41442ns | 47979ns | -21.77% |
| sequential | 30262ns | 30099ns | 26283ns | 29515ns | 33373ns | -45.17% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 54381ns | 41973ns | 73346ns | base | 0.075 |
| per_core | 42639ns | 38299ns | 47236ns | -21.59% | 0.096 |
| sequential | 29608ns | 25758ns | 32516ns | -45.55% | 0.138 |

## Performance model

- Peak throughput: **0.159 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.088 | 55.6% |
| per_core | 0.099 | 62.5% |
| sequential | 0.138 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 55190ns | 55190ns | base |
| per_core | 43176ns | 43176ns | -21.77% |
| sequential | 30262ns | 30262ns | -45.17% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 46349ns | base | --- | [44155, 66111] | --- | --- | --- | --- |
| per_core | 41212ns | -5274.2ns (-11.4%) | [-25463, -1478]ns | [40372, 44298] | YES | 0.0013 | 0.0013 | 0 |
| sequential | 29603ns | -17145.1ns (-37.0%) | [-35800, -14034]ns | [28272, 30663] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 46349ns | -9.6% | -35.5% |
| 1 | 2 | 74849ns | -43.9% | -62.1% |
| 1 | 3 | 43060ns | -5.2% | -29.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.661 | HIGH- (thermal bounce) |
| per_core | -0.203 | moderate- |
| sequential | -0.645 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 55912.6ns | 54381.5ns | 102.8% | HIGH |
| per_core | 42831.6ns | 42638.7ns | 100.5% | HIGH |
| sequential | 30079.2ns | 29608.2ns | 101.6% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 41973.2-73345.7 ns)
  41973.2 |####################
  43541.8 |########################################
  45110.4 |##########
  46679.1 |##########
  48247.7 |####################
  49816.3 |
  51384.9 |
  52953.6 |
  54522.2 |
  56090.8 |
  57659.4 |
  59228.0 |
  60796.7 |
  62365.3 |
  63933.9 |##########
  65502.5 |
  67071.2 |##########
  68639.8 |
  70208.4 |
  71777.0 |
  (2 below, 4 above range)

per_core (n=18, range 38299.2-47235.8 ns)
  38299.2 |
  38746.0 |#############
  39192.8 |
  39639.7 |
  40086.5 |########################################
  40533.3 |########################################
  40980.1 |
  41427.0 |##########################
  41873.8 |
  42320.6 |#############
  42767.5 |
  43214.3 |#############
  43661.1 |
  44107.9 |
  44554.8 |
  45001.6 |##########################
  45448.4 |
  45895.3 |
  46342.1 |
  46788.9 |
  (2 below, 3 above range)

sequential (n=18, range 25757.8-32515.9 ns)
  25757.8 |
  26095.7 |########################################
  26433.6 |####################
  26771.5 |
  27109.5 |
  27447.4 |
  27785.3 |####################
  28123.2 |
  28461.1 |########################################
  28799.0 |####################
  29136.9 |####################
  29474.8 |####################
  29812.7 |####################
  30150.6 |####################
  30488.5 |####################
  30826.4 |########################################
  31164.3 |####################
  31502.2 |
  31840.1 |####################
  32178.0 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: CV=25.6% (high variance, measurements may be unstable)
- **grouped**: bridge=103.2% of algo (FFI overhead may distort results)
- **per_core**: bridge=104.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.5% of algo (FFI overhead may distort results)
