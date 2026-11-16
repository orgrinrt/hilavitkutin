# link-static: the fixture's dev profile

2 variants, 3 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (524.41 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 319.64 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 64% faster than the next best (default_relink)

default_replay (319.64 ms) leads default_relink (524.41 ms) by 64%, a clear separation rather than a photo finish. CV 10.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 42% (significant)

default_replay is -221.94 ms (42%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 10.5%)

default_replay wins on median (319.64 ms) yet has the highest variance (CV 10.5%), while default_relink is the steadiest (CV 10.0%, 524.41 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: default_replay** at 319637844.2 ns median (-39.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.64x (fastest 319637844.2 ns, slowest 524406067.8 ns)

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
| default_relink | 536057233ns | 524418461ns | 478241758ns | -0ns | 536057233ns | base |
| default_replay | 303269617ns | 319646408ns | 256301221ns | -0ns | 303269617ns | -43.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 536045330ns | 478230141ns | 536045330ns | base |
| default_replay | 303260973ns | 256292739ns | 303260973ns | -43.43% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 536057233ns | 536057233ns | base |
| default_replay | 303269617ns | 303269617ns | -43.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 524406068ns | base | --- | [478230141, 605499782] | --- | --- | --- | --- |
| default_replay | 319637844ns | -221937401.3ns (-42.3%) | [-285861938, -190553733]ns | [256292739, 333852335] | YES (adj: no) | 0.2500 | 0.2500 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 478230141ns | -46.4% |
| 1 | 2 | 524406068ns | -36.3% |
| 1 | 3 | 605499782ns | -47.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.016 | ok |
| default_replay | -0.274 | moderate- |

**Consistency summary:**

- **default_replay**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 323847210.7ns | 536045330.3ns | 60.4% | HIGH |
| default_replay | 312678802.8ns | 303260972.7ns | 103.1% | HIGH |

## Distribution (algo ns)

```
default_relink (n=3, range 478230140.6-536045330.3 ns)
  478230140.6 |########################################
  481120900.1 |
  484011659.6 |
  486902419.1 |
  489793178.5 |
  492683938.0 |
  495574697.5 |
  498465457.0 |
  501356216.5 |
  504246976.0 |
  507137735.4 |
  510028494.9 |
  512919254.4 |
  515810013.9 |
  518700773.4 |
  521591532.9 |########################################
  524482292.3 |
  527373051.8 |
  530263811.3 |
  533154570.8 |
  (0 below, 1 above range)

default_replay (n=3, range 256292739.3-303260972.7 ns)
  256292739.3 |########################################
  258641151.0 |
  260989562.6 |
  263337974.3 |
  265686386.0 |
  268034797.7 |
  270383209.3 |
  272731621.0 |
  275080032.7 |
  277428444.3 |
  279776856.0 |
  282125267.7 |
  284473679.4 |
  286822091.0 |
  289170502.7 |
  291518914.4 |
  293867326.0 |
  296215737.7 |
  298564149.4 |
  300912561.1 |
  (0 below, 2 above range)

```

## Diagnostics

- **default_relink**: bridge=61.2% of algo (FFI overhead may distort results)
- **default_replay**: bridge=99.8% of algo (FFI overhead may distort results)
