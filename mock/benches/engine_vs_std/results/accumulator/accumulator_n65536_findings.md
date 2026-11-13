# accumulator, a transform feeding the append surface

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std dominates: 173% faster than the next best (engine)

std (21.87 us) leads engine (59.62 us) by 173%, a clear separation rather than a photo finish. CV 5.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std beats baseline by 61% (significant)

std is -36.09 us (61%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### engine_par is an outlier: 5.7x slower than the field

engine_par (125.52 us) is 5.7x the fastest (21.87 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {std} vs {engine, std_par, engine_par} (173% apart)

The field splits into a fast tier {std} and a slow tier {engine, std_par, engine_par} with a 173% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 5.7x the fastest

Fastest std (21.87 us) to slowest engine_par (125.52 us): 5.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: std** at 21865.5 ns median (-63.3% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 5.74x (fastest 21865.5 ns, slowest 125519.1 ns)

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
| engine | 59537ns | 59817ns | 56980ns | 58038ns | 61302ns | base |
| engine_par | 134393ns | 125783ns | 108752ns | 118837ns | 152470ns | +125.73% |
| std | 22821ns | 22211ns | 22036ns | 22115ns | 23548ns | -61.67% |
| std_par | 92893ns | 93162ns | 77402ns | 84601ns | 102984ns | +56.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 59296ns | 56793ns | 61036ns | base | 1.105 |
| engine_par | 134059ns | 108446ns | 152077ns | +126.08% | 0.489 |
| std | 22547ns | 21848ns | 23239ns | -61.98% | 2.907 |
| std_par | 92540ns | 77158ns | 102564ns | +56.06% | 0.708 |

## Performance model

- Peak throughput: **3.000 Gops/s** (std; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.099 | 36.6% |
| engine_par | 0.522 | 17.4% |
| std | 2.997 | 99.9% |
| std_par | 0.706 | 23.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 59537ns | 59537ns | base |
| engine_par | 134393ns | 134393ns | +125.73% |
| std | 22821ns | 22821ns | -61.67% |
| std_par | 92893ns | 92893ns | +56.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 59621ns | base | --- | [56881, 61472] | --- | --- | --- | --- |
| engine_par | 125519ns | +65011.6ns (+109.0%) | [+56650, +84775]ns | [115145, 144474] | YES | 0.0078 | 0.0078 | 0 |
| std | 21866ns | -36089.1ns (-60.5%) | [-37919, -34934]ns | [21853, 24430] | YES | 0.0078 | 0.0078 | 0 |
| std_par | 92819ns | +32311.2ns (+54.2%) | [+17741, +44557]ns | [77529, 101438] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 56944ns | +107.1% | -61.6% | +77.2% |
| 1 | 2 | 59743ns | +130.0% | -63.4% | +37.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 0/2, lost 2/2
- **std**: won 2/2, lost 0/2
- **std_par**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 272534.5ns | 59296.2ns | 459.6% | HIGH |
| engine_par | 305138.5ns | 134058.5ns | 227.6% | HIGH |
| std | 281384.3ns | 22547.2ns | 1248.0% | HIGH |
| std_par | 301320.5ns | 92540.4ns | 325.6% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 56793.0-61036.2 ns)
  56793.0 |########################################
  57005.2 |####################
  57217.3 |
  57429.5 |
  57641.6 |
  57853.8 |
  58066.0 |
  58278.1 |
  58490.3 |
  58702.5 |
  58914.6 |
  59126.8 |
  59338.9 |####################
  59551.1 |####################
  59763.3 |####################
  59975.4 |
  60187.6 |
  60399.7 |
  60611.9 |
  60824.1 |
  (0 below, 2 above range)

engine_par (n=8, range 108445.6-152077.4 ns)
  108445.6 |####################
  110627.2 |
  112808.8 |
  114990.4 |####################
  117172.0 |
  119353.6 |########################################
  121535.1 |
  123716.7 |
  125898.3 |
  128079.9 |
  130261.5 |####################
  132443.1 |
  134624.7 |
  136806.3 |####################
  138987.9 |
  141169.5 |
  143351.0 |####################
  145532.6 |
  147714.2 |
  149895.8 |
  (0 below, 1 above range)

std (n=8, range 21847.7-23238.8 ns)
  21847.7 |########################################
  21917.3 |
  21986.8 |
  22056.4 |
  22125.9 |
  22195.5 |
  22265.0 |
  22334.6 |
  22404.1 |
  22473.7 |
  22543.2 |
  22612.8 |
  22682.4 |
  22751.9 |
  22821.5 |
  22891.0 |
  22960.6 |
  23030.1 |
  23099.7 |
  23169.2 |
  (0 below, 2 above range)

std_par (n=8, range 77158.0-102563.8 ns)
  77158.0 |########################################
  78428.3 |
  79698.6 |
  80968.9 |
  82239.2 |
  83509.5 |
  84779.7 |
  86050.0 |####################
  87320.3 |
  88590.6 |####################
  89860.9 |
  91131.2 |
  92401.5 |
  93671.8 |
  94942.1 |
  96212.4 |####################
  97482.7 |
  98753.0 |
  100023.2 |####################
  101293.5 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **engine**: bridge=459.9% of algo (FFI overhead may distort results)
- **engine_par**: bridge=238.4% of algo (FFI overhead may distort results)
- **std**: bridge=1283.1% of algo (FFI overhead may distort results)
- **std_par**: bridge=324.4% of algo (FFI overhead may distort results)
