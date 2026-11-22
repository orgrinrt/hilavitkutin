# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (190.75 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 95.34 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### riv_spec beats baseline by 46% (significant)

riv_spec is -87.60 ms (46%) faster than baseline riv_rcm, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### riv_rcm is an outlier: 2.0x slower than the field

riv_rcm (190.75 ms) is 2.0x the fastest (95.34 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### riv_snake is fastest but the noisiest (CV 136.9%)

riv_snake wins on median (95.34 ms) yet has the highest variance (CV 136.9%), while riv_spec is the steadiest (CV 43.4%, 102.56 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### riv_spec shows alternating (throttle bounce) (autocorr -0.55)

riv_spec's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (95.41 ms) is smaller than the fastest variant's own run-to-run std-dev (130.57 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Two tiers: {riv_snake, riv_scr, riv_spec, riv_rowm} vs {riv_rcm} (79% apart)

The field splits into a fast tier {riv_snake, riv_scr, riv_spec, riv_rowm} and a slow tier {riv_rcm} with a 79% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader riv_snake vs stability leader riv_spec (+8% speed for 3.2x steadier)

riv_snake is fastest (95.34 ms, CV 136.9%); riv_spec gives up 7.6% median for 3.2x lower variance (CV 43.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### riv_rcm is inconsistent: worst-20% is 5.4x its best-20%

riv_rcm's best 20% of batches run at 75.91 ms but its worst 20% at 408.59 ms (5.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: riv_snake** at 95342293.8 ns median (-50.0% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 2.00x (fastest 95342293.8 ns, slowest 190747860.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 231632089ns | 190776073ns | 75918068ns | 165536643ns | 408632269ns | base |
| riv_rowm | 152480253ns | 106871440ns | 65668671ns | 105321002ns | 266624922ns | -34.17% |
| riv_scr | 140657703ns | 101410546ns | 63110425ns | 90106025ns | 255258858ns | -39.28% |
| riv_snake | 151939044ns | 95356975ns | 65353969ns | 89241306ns | 289278188ns | -34.41% |
| riv_spec | 112030894ns | 102577512ns | 63785697ns | 95592150ns | 160811607ns | -51.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 231603347ns | 75910994ns | 408586252ns | base | 0.000 |
| riv_rowm | 152464480ns | 65661407ns | 266599363ns | -34.17% | 0.000 |
| riv_scr | 140641639ns | 63101697ns | 255232857ns | -39.27% | 0.000 |
| riv_snake | 151920785ns | 65346614ns | 289244093ns | -34.40% | 0.000 |
| riv_spec | 112017877ns | 63776931ns | 160794273ns | -51.63% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_scr; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 33.1% |
| riv_rowm | 0.000 | 59.1% |
| riv_scr | 0.000 | 62.2% |
| riv_snake | 0.000 | 66.2% |
| riv_spec | 0.000 | 61.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 231632089ns | 231632089ns | base |
| riv_rowm | 152480253ns | 152480253ns | -34.17% |
| riv_scr | 140657703ns | 140657703ns | -39.28% |
| riv_snake | 151939044ns | 151939044ns | -34.41% |
| riv_spec | 112030894ns | 112030894ns | -51.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 190747860ns | base | --- | [99710617, 317284477] | --- | --- | --- | --- |
| riv_rowm | 106858950ns | no significant difference | [-196090198, +9292052]ns | [82154810, 198846977] | no | 0.4807 | 0.4807 | 0 |
| riv_scr | 101397112ns | -71282964.6ns (-37.4%) | [-192449285, -11622654]ns | [77627881, 132566479] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| riv_snake | 95342294ns | -63796875.0ns (-33.4%) | [-143329185, -2068317]ns | [77617725, 135026450] | YES (adj: no) | 0.1283 | 0.0963 | 0 |
| riv_spec | 102562112ns | -87604981.2ns (-45.9%) | [-208832046, -6746188]ns | [84196658, 129853094] | YES (adj: no) | 0.0618 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 190544692ns | -44.1% | -30.5% | -43.0% | -47.4% |
| 1 | 2 | 84649250ns | -21.7% | -3.0% | -16.1% | -24.6% |
| 1 | 3 | 493666908ns | -41.4% | -39.0% | -6.1% | -72.3% |
| 2 | 1 | 229701471ns | -69.2% | -72.7% | -71.0% | -58.5% |
| 2 | 2 | 294973871ns | -63.6% | -65.8% | -70.0% | -63.5% |
| 2 | 3 | 87498429ns | +131.8% | -6.5% | +12.5% | +19.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.360 | moderate- |
| riv_rowm | -0.482 | moderate- |
| riv_scr | -0.418 | moderate- |
| riv_snake | -0.329 | moderate- |
| riv_spec | -0.551 | HIGH- (thermal bounce) |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 6/6, lost 0/6
- **riv_snake**: won 5/6, lost 1/6
- **riv_spec**: won 5/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 1142436206.7ns | 231603346.5ns | 493.3% | HIGH |
| riv_rowm | 1113136017.3ns | 152464480.3ns | 730.1% | HIGH |
| riv_scr | 1110766199.5ns | 140641639.3ns | 789.8% | HIGH |
| riv_snake | 1111040871.3ns | 151920785.0ns | 731.3% | HIGH |
| riv_spec | 1100848307.9ns | 112017876.8ns | 982.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 75910994.5-408586252.1 ns)
  75910994.5 |########################################
  92544757.3 |##########
  109178520.2 |
  125812283.1 |
  142446046.0 |##########
  159079808.9 |##########
  175713571.8 |####################
  192347334.6 |
  208981097.5 |
  225614860.4 |##########
  242248623.3 |
  258882386.2 |##########
  275516149.0 |
  292149911.9 |##########
  308783674.8 |
  325417437.7 |##########
  342051200.6 |##########
  358684963.4 |
  375318726.3 |##########
  391952489.2 |
  (1 below, 2 above range)

riv_rowm (n=18, range 65661406.9-266599363.2 ns)
  65661406.9 |########################################
  75708304.7 |
  85755202.6 |####################
  95802100.4 |##########
  105848998.2 |####################
  115895896.0 |
  125942793.8 |##########
  135989691.6 |
  146036589.4 |
  156083487.3 |
  166130385.1 |
  176177282.9 |##########
  186224180.7 |##########
  196271078.5 |##########
  206317976.3 |##########
  216364874.1 |
  226411771.9 |
  236458669.8 |
  246505567.6 |
  256552465.4 |
  (1 below, 3 above range)

riv_scr (n=18, range 63101697.2-255232856.9 ns)
  63101697.2 |########
  72708255.2 |########################################
  82314813.2 |
  91921371.2 |########
  101527929.1 |################
  111134487.1 |########
  120741045.1 |
  130347603.1 |################
  139954161.1 |
  149560719.1 |########
  159167277.1 |
  168773835.0 |
  178380393.0 |
  187986951.0 |
  197593509.0 |
  207200067.0 |
  216806625.0 |########
  226413183.0 |
  236019740.9 |
  245626298.9 |
  (2 below, 2 above range)

riv_snake (n=18, range 65346613.9-289244093.1 ns)
  65346613.9 |########################################
  76541487.9 |##########################
  87736361.8 |########################################
  98931235.8 |##########################
  110126109.7 |#############
  121320983.7 |
  132515857.6 |
  143710731.6 |
  154905605.6 |##########################
  166100479.5 |
  177295353.5 |
  188490227.4 |
  199685101.4 |
  210879975.3 |
  222074849.3 |
  233269723.3 |
  244464597.2 |
  255659471.2 |
  266854345.1 |
  278049219.1 |
  (2 below, 3 above range)

riv_spec (n=18, range 63776930.6-160794272.9 ns)
  63776930.6 |########################################
  68627797.7 |
  73478664.8 |#############
  78329531.9 |
  83180399.0 |
  88031266.2 |#############
  92882133.3 |##########################
  97733000.4 |#############
  102583867.5 |#############
  107434734.6 |#############
  112285601.7 |
  117136468.9 |
  121987336.0 |#############
  126838203.1 |#############
  131689070.2 |##########################
  136539937.3 |#############
  141390804.4 |
  146241671.6 |
  151092538.7 |
  155943405.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: CV=63.9% (high variance, measurements may be unstable)
- **riv_rcm**: worst_20/best_20 = 5.4x (possible bimodal distribution)
- **riv_rcm**: bridge=585.4% of algo (FFI overhead may distort results)
- **riv_rowm**: CV=61.2% (high variance, measurements may be unstable)
- **riv_rowm**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **riv_rowm**: bridge=1036.9% of algo (FFI overhead may distort results)
- **riv_scr**: CV=88.7% (high variance, measurements may be unstable)
- **riv_scr**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **riv_scr**: bridge=1084.7% of algo (FFI overhead may distort results)
- **riv_snake**: CV=85.9% (high variance, measurements may be unstable)
- **riv_snake**: worst_20/best_20 = 4.4x (possible bimodal distribution)
- **riv_snake**: bridge=1158.2% of algo (FFI overhead may distort results)
- **riv_spec**: CV=39.8% (high variance, measurements may be unstable)
- **riv_spec**: bridge=1077.1% of algo (FFI overhead may distort results)
