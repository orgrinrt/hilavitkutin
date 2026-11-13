# branching, get-ready

2 variants, 8 samples per variant.
Baseline: **engine_startup**

## Highlights

Baseline for all deltas below: **engine_startup**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine_startup) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine_startup has the worst median (3.44 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_startup at 1.32 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std_startup dominates: 161% faster than the next best (engine_startup)

std_startup (1.32 ms) leads engine_startup (3.44 ms) by 161%, a clear separation rather than a photo finish. CV 21.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_startup beats baseline by 62% (significant)

std_startup is -2.13 ms (62%) faster than baseline engine_startup, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_startup is fastest but the noisiest (CV 21.4%)

std_startup wins on median (1.32 ms) yet has the highest variance (CV 21.4%), while engine_startup is the steadiest (CV 8.7%, 3.44 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### std_startup shows alternating (throttle bounce) (autocorr -0.50)

std_startup's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: std_startup** at 1317261.0 ns median (-61.7% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.61x (fastest 1317261.0 ns, slowest 3443471.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | branching, get-ready |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000010002 |
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine_startup | 3546707ns | 3449894ns | 3088035ns | 3353220ns | 3806490ns | base |
| std_startup | 1472254ns | 1321449ns | 1282836ns | 1309773ns | 1641468ns | -58.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine_startup | 3540375ns | 3079023ns | 3800636ns | base | 1.185 |
| std_startup | 1467845ns | 1278414ns | 1636818ns | -58.54% | 2.857 |

## Performance model

- Peak throughput: **3.281 Gops/s** (std_startup; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine_startup | 1.218 | 37.1% |
| std_startup | 3.184 | 97.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine_startup | 3546707ns | 3546707ns | base |
| std_startup | 1472254ns | 1472254ns | -58.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine_startup | 3443472ns | base | --- | [3324992, 3908250] | --- | --- | --- | --- |
| std_startup | 1317261ns | -2129330.0ns (-61.8%) | [-2589337, -1422402]ns | [1291068, 1908189] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine_startup | std_startup |
|---|---|---|---|
| 1 | 1 | 3581760ns | -55.3% |
| 1 | 2 | 3443472ns | -61.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine_startup | -0.500 | moderate- |
| std_startup | -0.500 | moderate- |

**Consistency summary:**

- **std_startup**: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine_startup | 55872372.4ns | 3540375.4ns | 1578.1% | HIGH |
| std_startup | 42446909.2ns | 1467845.4ns | 2891.8% | HIGH |

## Distribution (algo ns)

```
engine_startup (n=8, range 3079023.3-3800635.9 ns)
  3079023.3 |####################
  3115103.9 |
  3151184.6 |
  3187265.2 |
  3223345.8 |
  3259426.5 |
  3295507.1 |########################################
  3331587.7 |
  3367668.3 |####################
  3403749.0 |
  3439829.6 |
  3475910.2 |####################
  3511990.9 |
  3548071.5 |
  3584152.1 |
  3620232.8 |
  3656313.4 |
  3692394.0 |
  3728474.7 |
  3764555.3 |
  (0 below, 3 above range)

std_startup (n=8, range 1278413.8-1636818.0 ns)
  1278413.8 |##########################
  1296334.0 |#############
  1314254.2 |########################################
  1332174.4 |
  1350094.6 |
  1368014.9 |
  1385935.1 |
  1403855.3 |
  1421775.5 |
  1439695.7 |
  1457615.9 |
  1475536.1 |
  1493456.3 |
  1511376.5 |
  1529296.7 |
  1547216.9 |
  1565137.2 |
  1583057.4 |
  1600977.6 |
  1618897.8 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine_startup**: bridge=1616.5% of algo (FFI overhead may distort results)
- **std_startup**: bridge=3202.6% of algo (FFI overhead may distort results)
