# link-static: the fixture's release profile

2 variants, 3 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (137.90 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 62.37 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 121% faster than the next best (default_relink)

default_replay (62.37 ms) leads default_relink (137.90 ms) by 121%, a clear separation rather than a photo finish. CV 12.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 55% (significant)

default_replay is -75.42 ms (55%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 12.3%)

default_replay wins on median (62.37 ms) yet has the highest variance (CV 12.3%), while default_relink is the steadiest (CV 3.2%, 137.90 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### default_replay shows alternating (throttle bounce) (autocorr -0.65)

default_replay's per-pass series has lag-1 autocorrelation -0.65, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: default_replay** at 62369007.2 ns median (-54.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.21x (fastest 62369007.2 ns, slowest 137902494.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's release profile |
| Passes | 3 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c6b737461740001 |
| Counter frequency | 2300475748 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 141020649ns | 137910792ns | 137793324ns | -0ns | 141020649ns | base |
| default_replay | 66412193ns | 62377807ns | 59684673ns | -0ns | 66412193ns | -52.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 141012385ns | 137785329ns | 141012385ns | base |
| default_replay | 66403212ns | 59676242ns | 66403212ns | -52.91% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 141020649ns | 141020649ns | base |
| default_replay | 66412193ns | 66412193ns | -52.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 137902494ns | base | --- | [137785329, 147349332] | --- | --- | --- | --- |
| default_replay | 62369007ns | -75416321.6ns (-54.7%) | [-87673090, -60738109]ns | [59676242, 77164385] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 137785329ns | -54.7% |
| 1 | 2 | 137902494ns | -44.0% |
| 1 | 3 | 147349332ns | -59.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.161 | ok |
| default_replay | -0.653 | HIGH- (thermal bounce) |

**Consistency summary:**

- **default_replay**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 60220218.3ns | 141012385.0ns | 42.7% | HIGH |
| default_replay | 63110292.8ns | 66403211.5ns | 95.0% | HIGH |

## Distribution (algo ns)

```
default_relink (n=3, range 137785328.8-141012385.0 ns)
  137785328.8 |########################################
  137946681.6 |
  138108034.4 |
  138269387.2 |
  138430740.0 |
  138592092.9 |
  138753445.7 |
  138914798.5 |
  139076151.3 |
  139237504.1 |
  139398856.9 |
  139560209.7 |
  139721562.5 |
  139882915.3 |
  140044268.1 |
  140205620.9 |
  140366973.8 |
  140528326.6 |
  140689679.4 |
  140851032.2 |
  (0 below, 1 above range)

default_replay (n=3, range 59676242.5-66403211.5 ns)
  59676242.5 |########################################
  60012591.0 |
  60348939.4 |
  60685287.9 |
  61021636.3 |
  61357984.8 |
  61694333.2 |
  62030681.7 |
  62367030.1 |########################################
  62703378.6 |
  63039727.0 |
  63376075.5 |
  63712423.9 |
  64048772.4 |
  64385120.8 |
  64721469.3 |
  65057817.7 |
  65394166.2 |
  65730514.6 |
  66066863.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=42.1% of algo (FFI overhead may distort results)
- **default_replay**: bridge=101.5% of algo (FFI overhead may distort results)
