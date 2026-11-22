# link-static: the fixture's dev profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (107.94 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 50.59 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 113% faster than the next best (default_relink)

default_replay (50.59 ms) leads default_relink (107.94 ms) by 113%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 52% (significant)

default_replay is -56.64 ms (52%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: default_replay** at 50591818.2 ns median (-53.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.13x (fastest 50591818.2 ns, slowest 107936067.5 ns)

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
| Counter frequency | 2112646720 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 107256999ns | 107938159ns | 102030527ns | 106475808ns | 111042021ns | base |
| default_replay | 49846397ns | 50594148ns | 46421021ns | 49633868ns | 51877879ns | -53.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 107254593ns | 102027966ns | 111039514ns | base |
| default_replay | 49843919ns | 46418515ns | 51875455ns | -53.53% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 107256999ns | 107256999ns | base |
| default_replay | 49846397ns | 49846397ns | -53.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 107936068ns | base | --- | [102788196, 111039514] | --- | --- | --- | --- |
| default_replay | 50591818ns | -56638748.7ns (-52.5%) | [-60491380, -55101892]ns | [47064484, 51875455] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 108230995ns | -52.6% |
| 1 | 2 | 107641140ns | -51.9% |
| 1 | 3 | 103548426ns | -55.2% |
| 1 | 4 | 108300204ns | -52.0% |
| 1 | 5 | 113778824ns | -56.1% |
| 1 | 6 | 102027966ns | -53.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.375 | moderate- |
| default_replay | -0.422 | moderate- |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 46255482.1ns | 107254592.6ns | 43.1% | HIGH |
| default_replay | 48432087.9ns | 49843919.1ns | 97.2% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 102027966.0-111039514.2 ns)
  102027966.0 |####################
  102478543.4 |
  102929120.8 |
  103379698.2 |####################
  103830275.6 |
  104280853.0 |
  104731430.4 |
  105182007.9 |
  105632585.3 |
  106083162.7 |
  106533740.1 |
  106984317.5 |
  107434894.9 |####################
  107885472.3 |########################################
  108336049.7 |
  108786627.1 |
  109237204.5 |
  109687781.9 |
  110138359.3 |
  110588936.7 |
  (0 below, 1 above range)

default_replay (n=6, range 46418514.8-51875454.9 ns)
  46418514.8 |########################################
  46691361.8 |
  46964208.8 |
  47237055.8 |
  47509902.8 |########################################
  47782749.8 |
  48055596.8 |
  48328443.8 |
  48601290.8 |
  48874137.8 |
  49146984.9 |
  49419831.9 |
  49692678.9 |########################################
  49965525.9 |
  50238372.9 |
  50511219.9 |
  50784066.9 |
  51056913.9 |########################################
  51329760.9 |
  51602607.9 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=42.3% of algo (FFI overhead may distort results)
- **default_replay**: bridge=92.4% of algo (FFI overhead may distort results)
