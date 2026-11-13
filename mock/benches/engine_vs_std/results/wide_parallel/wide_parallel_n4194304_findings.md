# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_par beats baseline by 68% (significant)

std_par is -39.13 ms (68%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std is an outlier: 3.3x slower than the field

std (62.68 ms) is 3.3x the fastest (19.10 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_par is fastest but the noisiest (CV 13.3%)

std_par wins on median (19.10 ms) yet has the highest variance (CV 13.3%), while engine is the steadiest (CV 1.6%, 57.67 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_par, engine_par} vs {engine, std} (180% apart)

The field splits into a fast tier {std_par, engine_par} and a slow tier {engine, std} with a 180% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.3x the fastest

Fastest std_par (19.10 ms) to slowest std (62.68 ms): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: std_par** at 19104703.6 ns median (-66.9% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.28x (fastest 19104703.6 ns, slowest 62675051.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | wide_parallel, four independent heavy chains |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000004 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 58082130ns | 57670774ns | 56874435ns | 57409707ns | 58888372ns | base |
| engine_par | 20728374ns | 20571302ns | 18302399ns | 20011835ns | 21872271ns | -64.31% |
| std | 61675187ns | 62681510ns | 58316863ns | 59647342ns | 64035653ns | +6.19% |
| std_par | 19698470ns | 19109998ns | 16516311ns | 17872793ns | 21863268ns | -66.09% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 58076830ns | 56869032ns | 58883028ns | base | 0.072 |
| engine_par | 20722829ns | 18297482ns | 21866566ns | -64.32% | 0.202 |
| std | 61669004ns | 58312272ns | 64028460ns | +6.19% | 0.068 |
| std_par | 19692917ns | 16511842ns | 21858093ns | -66.09% | 0.213 |

## Performance model

- Peak throughput: **0.254 Gops/s** (std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.073 | 28.6% |
| engine_par | 0.204 | 80.3% |
| std | 0.067 | 26.3% |
| std_par | 0.220 | 86.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 58082130ns | 58082130ns | base |
| engine_par | 20728374ns | 20728374ns | -64.31% |
| std | 61675187ns | 61675187ns | +6.19% |
| std_par | 19698470ns | 19698470ns | -66.09% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 57665851ns | base | --- | [57374572, 59250116] | --- | --- | --- | --- |
| engine_par | 20565127ns | -37240392.6ns (-64.6%) | [-39077091, -35842825]ns | [19506573, 21716123] | YES | 0.0078 | 0.0078 | 0 |
| std | 62675051ns | +4109192.6ns (+7.1%) | [+1005116, +4725555]ns | [58379689, 63852566] | YES | 0.0078 | 0.0078 | 0 |
| std_par | 19104704ns | -39130538.0ns (-67.9%) | [-40306820, -35163506]ns | [17082006, 22683137] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 59173014ns | -64.6% | +7.9% | -65.7% |
| 1 | 2 | 57412336ns | -64.9% | +1.7% | -68.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 2/2, lost 0/2
- **std**: won 0/2, lost 2/2
- **std_par**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 83970912.3ns | 58076830.0ns | 144.6% | HIGH |
| engine_par | 85244804.6ns | 20722829.3ns | 411.4% | HIGH |
| std | 87397493.9ns | 61669004.5ns | 141.7% | HIGH |
| std_par | 85656700.9ns | 19692917.0ns | 435.0% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 56869031.7-58883027.6 ns)
  56869031.7 |#############
  56969731.5 |
  57070431.3 |
  57171131.1 |
  57271830.9 |
  57372530.7 |########################################
  57473230.5 |
  57573930.3 |
  57674630.1 |
  57775329.9 |
  57876029.7 |#############
  57976729.5 |
  58077429.3 |
  58178129.1 |
  58278828.8 |
  58379528.6 |
  58480228.4 |
  58580928.2 |
  58681628.0 |
  58782327.8 |
  (0 below, 3 above range)

engine_par (n=8, range 18297481.5-21866566.0 ns)
  18297481.5 |########################################
  18475935.7 |
  18654389.9 |
  18832844.2 |
  19011298.4 |
  19189752.6 |
  19368206.9 |########################################
  19546661.1 |
  19725115.3 |
  19903569.5 |
  20082023.8 |########################################
  20260478.0 |########################################
  20438932.2 |
  20617386.4 |########################################
  20795840.6 |
  20974294.9 |
  21152749.1 |
  21331203.3 |
  21509657.6 |########################################
  21688111.8 |########################################
  (0 below, 1 above range)

std (n=8, range 58312272.2-64028460.4 ns)
  58312272.2 |########################################
  58598081.6 |
  58883891.0 |
  59169700.4 |
  59455509.8 |
  59741319.3 |
  60027128.7 |
  60312938.1 |
  60598747.5 |
  60884556.9 |
  61170366.3 |
  61456175.7 |
  61741985.1 |
  62027794.5 |#############
  62313604.0 |
  62599413.4 |
  62885222.8 |
  63171032.2 |#############
  63456841.6 |
  63742651.0 |##########################
  (0 below, 1 above range)

std_par (n=8, range 16511842.4-21858092.8 ns)
  16511842.4 |########################################
  16779154.9 |
  17046467.4 |########################################
  17313780.0 |
  17581092.5 |
  17848405.0 |########################################
  18115717.5 |
  18383030.0 |########################################
  18650342.6 |
  18917655.1 |
  19184967.6 |
  19452280.1 |########################################
  19719592.7 |
  19986905.2 |
  20254217.7 |
  20521530.2 |
  20788842.7 |########################################
  21056155.3 |
  21323467.8 |
  21590780.3 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=145.5% of algo (FFI overhead may distort results)
- **engine_par**: bridge=411.7% of algo (FFI overhead may distort results)
- **std**: bridge=142.0% of algo (FFI overhead may distort results)
- **std_par**: bridge=449.1% of algo (FFI overhead may distort results)
