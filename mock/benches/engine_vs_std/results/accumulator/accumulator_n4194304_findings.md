# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (engine) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline engine has the worst median (7.79 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest std_par at 1.95 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### std_par dominates: 67% faster than the next best (std)

std_par (1.95 ms) leads std (3.25 ms) by 67%, a clear separation rather than a photo finish. CV 13.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_par beats baseline by 78% (significant)

std_par is -6.09 ms (78%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine is an outlier: 4.0x slower than the field

engine (7.79 ms) is 4.0x the fastest (1.95 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.0x the fastest

Fastest std_par (1.95 ms) to slowest engine (7.79 ms): 4.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### std is inconsistent: worst-20% is 1.9x its best-20%

std's best 20% of batches run at 2.17 ms but its worst 20% at 4.18 ms (1.9x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_par** at 1951908.4 ns median (-74.9% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 3.99x (fastest 1951908.4 ns, slowest 7791916.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | accumulator, a transform feeding the append surface |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xe5d0000000000003 |
| Counter frequency | 2300371984 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 7968593ns | 7795996ns | 6291236ns | 6820076ns | 9249320ns | base |
| engine_par | 3637405ns | 3520025ns | 2645535ns | 3073965ns | 4307954ns | -54.35% |
| std | 3269954ns | 3255968ns | 2169538ns | 2416807ns | 4184918ns | -58.96% |
| std_par | 1891545ns | 1957454ns | 1464911ns | 1734144ns | 2116255ns | -76.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 7964448ns | 6287872ns | 9245261ns | base | 0.527 |
| engine_par | 3633061ns | 2642089ns | 4302739ns | -54.38% | 1.154 |
| std | 3266028ns | 2167699ns | 4179245ns | -58.99% | 1.284 |
| std_par | 1887162ns | 1462212ns | 2111074ns | -76.31% | 2.223 |

## Performance model

- Peak throughput: **2.868 Gops/s** (std_par; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.538 | 18.8% |
| engine_par | 1.193 | 41.6% |
| std | 1.290 | 45.0% |
| std_par | 2.149 | 74.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 7968593ns | 7968593ns | base |
| engine_par | 3637405ns | 3637405ns | -54.35% |
| std | 3269954ns | 3269954ns | -58.96% |
| std_par | 1891545ns | 1891545ns | -76.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 7791916ns | base | --- | [6682093, 9674962] | --- | --- | --- | --- |
| engine_par | 3515805ns | -4302004.2ns (-55.2%) | [-5096969, -3820210]ns | [2722336, 4497824] | YES | 0.0078 | 0.0078 | 0 |
| std | 3252240ns | -4437957.8ns (-57.0%) | [-5521740, -4174221]ns | [2338145, 4153222] | YES | 0.0078 | 0.0078 | 0 |
| std_par | 1951908ns | -6090609.8ns (-78.2%) | [-7503852, -4825661]ns | [1474584, 2133295] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 6771469ns | -57.4% | -65.2% | -75.6% |
| 1 | 2 | 9297702ns | -51.8% | -55.7% | -77.9% |

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
| engine | 20745822.7ns | 7964447.8ns | 260.5% | HIGH |
| engine_par | 21344252.7ns | 3633061.4ns | 587.5% | HIGH |
| std | 20468814.6ns | 3266027.9ns | 626.7% | HIGH |
| std_par | 20646854.9ns | 1887161.9ns | 1094.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 6287872.2-9245261.4 ns)
  6287872.2 |########################################
  6435741.7 |
  6583611.1 |########################################
  6731480.6 |########################################
  6879350.0 |########################################
  7027219.5 |
  7175088.9 |
  7322958.4 |
  7470827.9 |
  7618697.3 |
  7766566.8 |
  7914436.2 |
  8062305.7 |
  8210175.1 |
  8358044.6 |
  8505914.1 |
  8653783.5 |########################################
  8801653.0 |########################################
  8949522.4 |
  9097391.9 |
  (0 below, 2 above range)

engine_par (n=8, range 2642088.7-4302739.3 ns)
  2642088.7 |########################################
  2725121.2 |
  2808153.8 |
  2891186.3 |
  2974218.8 |####################
  3057251.3 |
  3140283.9 |
  3223316.4 |
  3306348.9 |
  3389381.5 |####################
  3472414.0 |
  3555446.5 |####################
  3638479.0 |
  3721511.6 |
  3804544.1 |
  3887576.6 |
  3970609.2 |
  4053641.7 |
  4136674.2 |
  4219706.7 |
  (0 below, 3 above range)

std (n=8, range 2167698.8-4179244.7 ns)
  2167698.8 |####################
  2268276.1 |####################
  2368853.4 |####################
  2469430.7 |####################
  2570008.0 |
  2670585.3 |
  2771162.6 |
  2871739.9 |
  2972317.2 |
  3072894.5 |
  3173471.8 |
  3274049.1 |
  3374626.4 |
  3475203.7 |
  3575780.9 |
  3676358.2 |
  3776935.5 |
  3877512.8 |####################
  3978090.1 |
  4078667.4 |########################################
  (0 below, 1 above range)

std_par (n=8, range 1462211.5-2111074.1 ns)
  1462211.5 |########################################
  1494654.6 |
  1527097.8 |
  1559540.9 |
  1591984.0 |
  1624427.1 |
  1656870.3 |
  1689313.4 |
  1721756.5 |
  1754199.7 |
  1786642.8 |
  1819085.9 |####################
  1851529.0 |
  1883972.2 |####################
  1916415.3 |
  1948858.4 |
  1981301.6 |
  2013744.7 |####################
  2046187.8 |
  2078630.9 |####################
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=264.9% of algo (FFI overhead may distort results)
- **engine_par**: CV=21.1% (high variance, measurements may be unstable)
- **engine_par**: bridge=615.8% of algo (FFI overhead may distort results)
- **std**: CV=28.4% (high variance, measurements may be unstable)
- **std**: bridge=627.7% of algo (FFI overhead may distort results)
- **std_par**: bridge=1063.0% of algo (FFI overhead may distort results)
