# link-static: the fixture's release profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (171.70 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 67.19 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 156% faster than the next best (default_relink)

default_replay (67.19 ms) leads default_relink (171.70 ms) by 156%, a clear separation rather than a photo finish. CV 2.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 61% (significant)

default_replay is -104.77 ms (61%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: default_replay** at 67185160.2 ns median (-60.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.56x (fastest 67185160.2 ns, slowest 171701152.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's release profile |
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
| default_relink | 169781371ns | 171706141ns | 155540374ns | 168721061ns | 178492335ns | base |
| default_replay | 67567667ns | 67189433ns | 65083537ns | 66924763ns | 69774089ns | -60.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 169776451ns | 155533368ns | 178488277ns | base |
| default_replay | 67563276ns | 65079964ns | 69769164ns | -60.20% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 169781371ns | 169781371ns | base |
| default_replay | 67567667ns | 67567667ns | -60.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 171701152ns | base | --- | [159139923, 178488277] | --- | --- | --- | --- |
| default_replay | 67185160ns | -104766781.7ns (-61.0%) | [-112030191, -89842550]ns | [65735505, 69769164] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 177248426ns | -62.5% |
| 1 | 2 | 173890896ns | -62.6% |
| 1 | 3 | 162746478ns | -58.3% |
| 1 | 4 | 155533368ns | -54.5% |
| 1 | 5 | 169511409ns | -59.4% |
| 1 | 6 | 179728129ns | -63.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | 0.243 | moderate+ |
| default_replay | 0.282 | moderate+ |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 65318870.9ns | 169776450.8ns | 38.5% | HIGH |
| default_replay | 61130220.0ns | 67563276.5ns | 90.5% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 155533367.5-178488277.2 ns)
  155533367.5 |########################################
  156681113.0 |
  157828858.5 |
  158976604.0 |
  160124349.4 |
  161272094.9 |
  162419840.4 |########################################
  163567585.9 |
  164715331.4 |
  165863076.9 |
  167010822.4 |
  168158567.9 |
  169306313.3 |########################################
  170454058.8 |
  171601804.3 |
  172749549.8 |########################################
  173897295.3 |
  175045040.8 |
  176192786.3 |########################################
  177340531.8 |
  (0 below, 1 above range)

default_replay (n=6, range 65079964.5-69769164.1 ns)
  65079964.5 |########################################
  65314424.5 |
  65548884.5 |
  65783344.4 |
  66017804.4 |
  66252264.4 |########################################
  66486724.4 |########################################
  66721184.4 |
  66955644.3 |
  67190104.3 |
  67424564.3 |
  67659024.3 |########################################
  67893484.3 |
  68127944.2 |
  68362404.2 |
  68596864.2 |########################################
  68831324.2 |
  69065784.2 |
  69300244.1 |
  69534704.1 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=36.6% of algo (FFI overhead may distort results)
- **default_replay**: bridge=90.8% of algo (FFI overhead may distort results)
