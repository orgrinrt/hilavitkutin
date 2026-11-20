# link-static: the fixture's dev profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (162.10 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 79.97 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 103% faster than the next best (default_relink)

default_replay (79.97 ms) leads default_relink (162.10 ms) by 103%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 51% (significant)

default_replay is -83.40 ms (51%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 10.0%)

default_replay wins on median (79.97 ms) yet has the highest variance (CV 10.0%), while default_relink is the steadiest (CV 3.6%, 162.10 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: default_replay** at 79965118.5 ns median (-50.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.03x (fastest 79965118.5 ns, slowest 162101012.3 ns)

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
| Counter frequency | 2100433564 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 160893252ns | 162105266ns | 152599758ns | 159433279ns | 167229957ns | base |
| default_replay | 81549146ns | 79970539ns | 71711593ns | 78931990ns | 90393658ns | -49.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 160888620ns | 152595551ns | 167224893ns | base |
| default_replay | 81544063ns | 71707130ns | 90388475ns | -49.32% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 160893252ns | 160893252ns | base |
| default_replay | 81549146ns | 81549146ns | -49.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 162101012ns | base | --- | [153339956, 167224893] | --- | --- | --- | --- |
| default_replay | 79965118ns | -83395617.4ns (-51.4%) | [-87644064, -66993990]ns | [74278596, 90388475] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 166606979ns | -53.2% |
| 1 | 2 | 167842808ns | -50.3% |
| 1 | 3 | 160702358ns | -39.4% |
| 1 | 4 | 154084360ns | -53.5% |
| 1 | 5 | 152595551ns | -46.3% |
| 1 | 6 | 163499667ns | -53.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | 0.367 | moderate+ |
| default_replay | -0.360 | moderate- |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 64962733.9ns | 160888620.4ns | 40.4% | HIGH |
| default_replay | 69795221.2ns | 81544063.2ns | 85.6% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 152595550.9-167224893.2 ns)
  152595550.9 |########################################
  153327018.0 |
  154058485.1 |########################################
  154789952.2 |
  155521419.3 |
  156252886.5 |
  156984353.6 |
  157715820.7 |
  158447287.8 |
  159178754.9 |
  159910222.0 |
  160641689.1 |########################################
  161373156.2 |
  162104623.4 |
  162836090.5 |########################################
  163567557.6 |
  164299024.7 |
  165030491.8 |
  165761958.9 |
  166493426.0 |########################################
  (0 below, 1 above range)

default_replay (n=6, range 71707130.3-90388475.4 ns)
  71707130.3 |########################################
  72641197.6 |
  73575264.8 |
  74509332.1 |
  75443399.3 |
  76377466.6 |########################################
  77311533.8 |########################################
  78245601.1 |
  79179668.3 |
  80113735.6 |
  81047802.8 |########################################
  81981870.1 |
  82915937.4 |########################################
  83850004.6 |
  84784071.9 |
  85718139.1 |
  86652206.4 |
  87586273.6 |
  88520340.9 |
  89454408.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=38.5% of algo (FFI overhead may distort results)
- **default_replay**: bridge=88.7% of algo (FFI overhead may distort results)
