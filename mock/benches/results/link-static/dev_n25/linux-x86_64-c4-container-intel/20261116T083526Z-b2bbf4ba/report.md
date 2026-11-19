# link-static: the fixture's dev profile

2 variants, 3 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (148.57 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 57.84 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 157% faster than the next best (default_relink)

default_replay (57.84 ms) leads default_relink (148.57 ms) by 157%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 61% (significant)

default_replay is -91.26 ms (61%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 7.1%)

default_replay wins on median (57.84 ms) yet has the highest variance (CV 7.1%), while default_relink is the steadiest (CV 4.8%, 148.57 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### default_relink shows alternating (throttle bounce) (autocorr -0.53)

default_relink's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: default_replay** at 57838585.8 ns median (-61.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.57x (fastest 57838585.8 ns, slowest 148571715.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's dev profile |
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
| default_relink | 148057111ns | 148579405ns | 139062713ns | -0ns | 148057111ns | base |
| default_replay | 59620368ns | 57844964ns | 55746800ns | -0ns | 59620368ns | -59.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 148048820ns | 139054172ns | 148048820ns | base |
| default_replay | 59613189ns | 55739205ns | 59613189ns | -59.73% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 148057111ns | 148057111ns | base |
| default_replay | 59620368ns | 59620368ns | -59.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 148571715ns | base | --- | [139054172, 156520574] | --- | --- | --- | --- |
| default_replay | 57838586ns | -91258797.7ns (-61.4%) | [-92832511, -81215586]ns | [55739205, 65261776] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 156520574ns | -58.3% |
| 1 | 2 | 139054172ns | -58.4% |
| 1 | 3 | 148571715ns | -62.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.529 | HIGH- (thermal bounce) |
| default_replay | -0.063 | ok |

**Consistency summary:**

- **default_replay**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 75549109.5ns | 148048820.1ns | 51.0% | HIGH |
| default_replay | 62135875.8ns | 59613188.8ns | 104.2% | HIGH |

## Distribution (algo ns)

```
default_relink (n=3, range 139054171.5-148048820.1 ns)
  139054171.5 |########################################
  139503903.9 |
  139953636.4 |
  140403368.8 |
  140853101.2 |
  141302833.7 |
  141752566.1 |
  142202298.5 |
  142652031.0 |
  143101763.4 |
  143551495.8 |
  144001228.2 |
  144450960.7 |
  144900693.1 |
  145350425.5 |
  145800158.0 |
  146249890.4 |
  146699622.8 |
  147149355.3 |
  147599087.7 |
  (0 below, 2 above range)

default_replay (n=3, range 55739204.6-59613188.8 ns)
  55739204.6 |########################################
  55932903.8 |
  56126603.0 |
  56320302.2 |
  56514001.4 |
  56707700.7 |
  56901399.9 |
  57095099.1 |
  57288798.3 |
  57482497.5 |
  57676196.7 |########################################
  57869895.9 |
  58063595.1 |
  58257294.3 |
  58450993.5 |
  58644692.8 |
  58838392.0 |
  59032091.2 |
  59225790.4 |
  59419489.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=48.0% of algo (FFI overhead may distort results)
- **default_replay**: bridge=99.9% of algo (FFI overhead may distort results)
