# link-static: the fixture's dev profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (142.95 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 86.02 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 66% faster than the next best (default_relink)

default_replay (86.02 ms) leads default_relink (142.95 ms) by 66%, a clear separation rather than a photo finish. CV 19.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 42% (significant)

default_replay is -59.86 ms (42%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 19.2%)

default_replay wins on median (86.02 ms) yet has the highest variance (CV 19.2%), while default_relink is the steadiest (CV 6.7%, 142.95 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### default_replay shows alternating (throttle bounce) (autocorr -0.69)

default_replay's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### default_replay is inconsistent: worst-20% is 1.9x its best-20%

default_replay's best 20% of batches run at 54.54 ms but its worst 20% at 101.27 ms (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: default_replay** at 86022158.0 ns median (-39.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.66x (fastest 86022158.0 ns, slowest 142952766.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's dev profile |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c6b737461740001 |
| Counter frequency | 2112767800 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 141455675ns | 142955049ns | 127083167ns | 139581115ns | 151453770ns | base |
| default_replay | 84251149ns | 86024522ns | 54546777ns | 82805769ns | 101271404ns | -40.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 141452987ns | 127079736ns | 151451130ns | base |
| default_replay | 84248636ns | 54544099ns | 101268427ns | -40.44% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 141455675ns | 141455675ns | base |
| default_replay | 84251149ns | 84251149ns | -40.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 142952766ns | base | --- | [129955064, 151451130] | --- | --- | --- | --- |
| default_replay | 86022158ns | -59859812.1ns (-41.9%) | [-74055454, -37697788]ns | [65455322, 101268427] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 148539767ns | -44.0% |
| 1 | 2 | 149747746ns | -40.7% |
| 1 | 3 | 132830392ns | -42.5% |
| 1 | 4 | 153154514ns | -38.4% |
| 1 | 5 | 137365766ns | -60.3% |
| 1 | 6 | 127079736ns | -14.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.186 | ok |
| default_replay | -0.690 | HIGH- (thermal bounce) |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 48729880.1ns | 141452987.0ns | 34.4% | HIGH |
| default_replay | 67071935.2ns | 84248635.6ns | 79.6% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 127079736.3-151451130.2 ns)
  127079736.3 |########################################
  128298306.0 |
  129516875.7 |
  130735445.4 |
  131954015.1 |########################################
  133172584.8 |
  134391154.5 |
  135609724.1 |
  136828293.8 |########################################
  138046863.5 |
  139265433.2 |
  140484002.9 |
  141702572.6 |
  142921142.3 |
  144139712.0 |
  145358281.7 |
  146576851.4 |
  147795421.1 |########################################
  149013990.8 |########################################
  150232560.5 |
  (0 below, 1 above range)

default_replay (n=6, range 54544098.9-101268426.8 ns)
  54544098.9 |########################################
  56880315.3 |
  59216531.7 |
  61552748.1 |
  63888964.5 |
  66225180.9 |
  68561397.3 |
  70897613.7 |
  73233830.1 |
  75570046.5 |########################################
  77906262.9 |
  80242479.3 |
  82578695.7 |########################################
  84914912.1 |
  87251128.5 |########################################
  89587344.9 |
  91923561.3 |
  94259777.7 |########################################
  96595994.1 |
  98932210.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=33.3% of algo (FFI overhead may distort results)
- **default_replay**: bridge=77.7% of algo (FFI overhead may distort results)
