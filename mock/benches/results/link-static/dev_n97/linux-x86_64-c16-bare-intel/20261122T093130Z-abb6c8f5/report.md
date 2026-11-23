# link-static: the fixture's dev profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (411.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 252.88 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 63% faster than the next best (default_relink)

default_replay (252.88 ms) leads default_relink (411.70 ms) by 63%, a clear separation rather than a photo finish. CV 10.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 39% (significant)

default_replay is -160.12 ms (39%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 10.3%)

default_replay wins on median (252.88 ms) yet has the highest variance (CV 10.3%), while default_relink is the steadiest (CV 3.7%, 411.70 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: default_replay** at 252881218.6 ns median (-38.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.63x (fastest 252881218.6 ns, slowest 411702403.5 ns)

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
| default_relink | 413591240ns | 411704344ns | 395683073ns | 408391920ns | 430344302ns | base |
| default_replay | 256299247ns | 252883204ns | 216148750ns | 248158154ns | 288586137ns | -38.03% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 413589147ns | 395681162ns | 430342078ns | base |
| default_replay | 256295810ns | 216143013ns | 288581623ns | -38.03% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 413591240ns | 413591240ns | base |
| default_replay | 256299247ns | 256299247ns | -38.03% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 411702404ns | base | --- | [398722959, 430342078] | --- | --- | --- | --- |
| default_replay | 252881219ns | -160115407.2ns (-38.9%) | [-188643821, -123120781]ns | [227424590, 288581623] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 406402387ns | -28.8% |
| 1 | 2 | 395681162ns | -39.7% |
| 1 | 3 | 401764756ns | -46.2% |
| 1 | 4 | 417860561ns | -39.1% |
| 1 | 5 | 442823594ns | -43.3% |
| 1 | 6 | 417002420ns | -31.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | 0.368 | moderate+ |
| default_replay | 0.010 | ok |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 228341081.4ns | 413589146.7ns | 55.2% | HIGH |
| default_replay | 240304219.3ns | 256295810.4ns | 93.8% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 395681161.5-430342077.7 ns)
  395681161.5 |####################
  397414207.3 |
  399147253.1 |
  400880298.9 |####################
  402613344.7 |
  404346390.6 |
  406079436.4 |####################
  407812482.2 |
  409545528.0 |
  411278573.8 |
  413011619.6 |
  414744665.4 |
  416477711.2 |########################################
  418210757.0 |
  419943802.8 |
  421676848.6 |
  423409894.5 |
  425142940.3 |
  426875986.1 |
  428609031.9 |
  (0 below, 1 above range)

default_replay (n=6, range 216143012.9-288581622.6 ns)
  216143012.9 |########################################
  219764943.4 |
  223386873.9 |
  227008804.4 |
  230630734.8 |
  234252665.3 |
  237874595.8 |########################################
  241496526.3 |
  245118456.8 |
  248740387.3 |########################################
  252362317.8 |########################################
  255984248.2 |
  259606178.7 |
  263228109.2 |
  266850039.7 |
  270471970.2 |
  274093900.7 |
  277715831.1 |
  281337761.6 |
  284959692.1 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=55.5% of algo (FFI overhead may distort results)
- **default_replay**: bridge=95.7% of algo (FFI overhead may distort results)
