# link-static: the fixture's dev profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (186.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 78.47 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 138% faster than the next best (default_relink)

default_replay (78.47 ms) leads default_relink (186.70 ms) by 138%, a clear separation rather than a photo finish. CV 6.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 57% (significant)

default_replay is -107.30 ms (57%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay shows alternating (throttle bounce) (autocorr -0.66)

default_replay's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: default_replay** at 78471821.6 ns median (-58.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.38x (fastest 78471821.6 ns, slowest 186695608.2 ns)

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
| Counter frequency | 2100641458 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 187481455ns | 186700313ns | 169250462ns | 182606378ns | 203909566ns | base |
| default_replay | 79533974ns | 78476128ns | 73868222ns | 77055118ns | 86085134ns | -57.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 187476749ns | 169246045ns | 203904189ns | base |
| default_replay | 79529230ns | 73863482ns | 86080944ns | -57.58% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 187481455ns | 187481455ns | base |
| default_replay | 79533974ns | 79533974ns | -57.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 186695608ns | base | --- | [171830450, 203904189] | --- | --- | --- | --- |
| default_replay | 78471822ns | -107295678.3ns (-57.5%) | [-125849830, -90697050]ns | [74034923, 86080944] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 169246045ns | -48.5% |
| 1 | 2 | 174414856ns | -57.0% |
| 1 | 3 | 189762135ns | -55.2% |
| 1 | 4 | 183629082ns | -59.8% |
| 1 | 5 | 203393969ns | -59.7% |
| 1 | 6 | 204414410ns | -63.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | 0.384 | moderate+ |
| default_replay | -0.661 | HIGH- (thermal bounce) |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 73153925.4ns | 187476749.2ns | 39.0% | HIGH |
| default_replay | 75170371.5ns | 79529229.8ns | 94.5% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 169246044.8-203904189.1 ns)
  169246044.8 |########################################
  170978952.0 |
  172711859.2 |########################################
  174444766.4 |
  176177673.7 |
  177910580.9 |
  179643488.1 |
  181376395.3 |
  183109302.5 |########################################
  184842209.7 |
  186575116.9 |
  188308024.1 |########################################
  190040931.4 |
  191773838.6 |
  193506745.8 |
  195239653.0 |
  196972560.2 |
  198705467.4 |
  200438374.6 |
  202171281.8 |########################################
  (0 below, 1 above range)

default_replay (n=6, range 73863481.9-86080944.5 ns)
  73863481.9 |########################################
  74474355.0 |####################
  75085228.2 |
  75696101.3 |
  76306974.4 |
  76917847.6 |
  77528720.7 |
  78139593.8 |
  78750466.9 |
  79361340.1 |
  79972213.2 |
  80583086.3 |
  81193959.5 |
  81804832.6 |####################
  82415705.7 |
  83026578.8 |
  83637452.0 |
  84248325.1 |
  84859198.2 |####################
  85470071.4 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=38.9% of algo (FFI overhead may distort results)
- **default_replay**: bridge=97.0% of algo (FFI overhead may distort results)
