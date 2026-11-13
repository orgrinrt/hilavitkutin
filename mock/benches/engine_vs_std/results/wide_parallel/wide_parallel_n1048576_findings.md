# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (15.50 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_par at 6.77 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### engine_par beats baseline by 56% (significant)

engine_par is -8.72 ms (56%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine is an outlier: 2.3x slower than the field

engine (15.50 ms) is 2.3x the fastest (6.77 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std_par, engine_par} vs {std, engine} (108% apart)

The field splits into a fast tier {std_par, engine_par} and a slow tier {std, engine} with a 108% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: std_par** at 6772959.3 ns median (-56.3% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.29x (fastest 6772959.3 ns, slowest 15496363.9 ns)

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
| engine | 15743135ns | 15501441ns | 15071028ns | 15269379ns | 16266479ns | base |
| engine_par | 6846696ns | 6936500ns | 5908265ns | 6607046ns | 7261043ns | -56.51% |
| std | 14479381ns | 14454884ns | 14101854ns | 14367161ns | 14657928ns | -8.03% |
| std_par | 6812472ns | 6778072ns | 6553746ns | 6732465ns | 6937158ns | -56.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 15737492ns | 15066762ns | 16259812ns | base | 0.067 |
| engine_par | 6841622ns | 5902378ns | 7256214ns | -56.53% | 0.153 |
| std | 14475394ns | 14097862ns | 14653788ns | -8.02% | 0.072 |
| std_par | 6807031ns | 6548437ns | 6931161ns | -56.75% | 0.154 |

## Performance model

- Peak throughput: **0.178 Gops/s** (engine_par; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.068 | 38.1% |
| engine_par | 0.151 | 85.2% |
| std | 0.073 | 40.8% |
| std_par | 0.155 | 87.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 15743135ns | 15743135ns | base |
| engine_par | 6846696ns | 6846696ns | -56.51% |
| std | 14479381ns | 14479381ns | -8.03% |
| std_par | 6812472ns | 6812472ns | -56.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 15496364ns | base | --- | [15186803, 16111449] | --- | --- | --- | --- |
| engine_par | 6931599ns | -8721966.4ns (-56.3%) | [-9903486, -8199708]ns | [6207963, 7302269] | YES | 0.0078 | 0.0078 | 0 |
| std | 14450828ns | -1114420.8ns (-7.2%) | [-1595203, -681353]ns | [14344648, 14672014] | YES | 0.0078 | 0.0078 | 0 |
| std_par | 6772959ns | -8677348.3ns (-56.0%) | [-9040544, -8486967]ns | [6699836, 6902003] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 15692161ns | -56.3% | -8.0% | -56.8% |
| 1 | 2 | 15303561ns | -54.5% | -5.5% | -55.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 2/2, lost 0/2
- **std**: won 2/2, lost 0/2
- **std_par**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 21856378.0ns | 15737492.2ns | 138.9% | HIGH |
| engine_par | 21526861.1ns | 6841621.7ns | 314.6% | HIGH |
| std | 20793944.6ns | 14475394.3ns | 143.7% | HIGH |
| std_par | 21049573.3ns | 6807031.2ns | 309.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 15066762.0-16259812.5 ns)
  15066762.0 |####################
  15126414.5 |
  15186067.0 |########################################
  15245719.6 |
  15305372.1 |
  15365024.6 |####################
  15424677.1 |
  15484329.7 |
  15543982.2 |####################
  15603634.7 |
  15663287.2 |
  15722939.8 |
  15782592.3 |####################
  15842244.8 |
  15901897.3 |
  15961549.9 |
  16021202.4 |
  16080854.9 |####################
  16140507.4 |
  16200160.0 |
  (0 below, 1 above range)

engine_par (n=8, range 5902377.5-7256213.9 ns)
  5902377.5 |########################################
  5970069.3 |
  6037761.1 |
  6105453.0 |
  6173144.8 |########################################
  6240836.6 |
  6308528.4 |
  6376220.2 |
  6443912.1 |
  6511603.9 |
  6579295.7 |
  6646987.5 |
  6714679.3 |########################################
  6782371.2 |
  6850063.0 |########################################
  6917754.8 |
  6985446.6 |########################################
  7053138.4 |########################################
  7120830.3 |
  7188522.1 |
  (0 below, 2 above range)

std (n=8, range 14097861.8-14653788.2 ns)
  14097861.8 |####################
  14125658.1 |
  14153454.4 |
  14181250.8 |
  14209047.1 |
  14236843.4 |
  14264639.7 |
  14292436.0 |
  14320232.4 |####################
  14348028.7 |####################
  14375825.0 |####################
  14403621.3 |
  14431417.7 |
  14459214.0 |
  14487010.3 |
  14514806.6 |########################################
  14542602.9 |
  14570399.3 |
  14598195.6 |
  14625991.9 |
  (0 below, 2 above range)

std_par (n=8, range 6548436.7-6931160.8 ns)
  6548436.7 |########################################
  6567572.9 |
  6586709.1 |
  6605845.3 |
  6624981.5 |
  6644117.7 |
  6663253.9 |
  6682390.1 |########################################
  6701526.3 |########################################
  6720662.5 |
  6739798.7 |
  6758934.9 |########################################
  6778071.1 |########################################
  6797207.3 |
  6816343.6 |
  6835479.8 |
  6854616.0 |########################################
  6873752.2 |
  6892888.4 |########################################
  6912024.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: bridge=139.7% of algo (FFI overhead may distort results)
- **engine_par**: bridge=308.4% of algo (FFI overhead may distort results)
- **std**: bridge=144.1% of algo (FFI overhead may distort results)
- **std_par**: bridge=311.0% of algo (FFI overhead may distort results)
