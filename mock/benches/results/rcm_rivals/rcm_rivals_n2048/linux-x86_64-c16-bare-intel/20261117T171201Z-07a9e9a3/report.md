# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (457.49 us) is smaller than the fastest variant's own run-to-run std-dev (1.01 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 2.4% of the fastest

All 5 variants sit between 18.68 ms and 19.13 ms - a 2.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_rowm** at 18676594.0 ns median (-2.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.02x (fastest 18676594.0 ns, slowest 19134083.8 ns)

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
| Counter frequency | 2112768073 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 20240445ns | 19084003ns | 18618743ns | 19274830ns | 22499718ns | base |
| riv_rowm | 19133870ns | 18680447ns | 18560233ns | 18689535ns | 20087191ns | -5.47% |
| riv_scr | 19888144ns | 19138111ns | 18498786ns | 19068515ns | 21812267ns | -1.74% |
| riv_snake | 19428928ns | 18822808ns | 18460601ns | 18730517ns | 20960710ns | -4.01% |
| riv_spec | 19332748ns | 18975635ns | 18326595ns | 18881661ns | 20512454ns | -4.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 20236411ns | 18614673ns | 22495726ns | base | 0.000 |
| riv_rowm | 19129728ns | 18556187ns | 20082686ns | -5.47% | 0.000 |
| riv_scr | 19884152ns | 18494822ns | 21808335ns | -1.74% | 0.000 |
| riv_snake | 19424820ns | 18456681ns | 20956651ns | -4.01% | 0.000 |
| riv_spec | 19328716ns | 18322611ns | 20508465ns | -4.49% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 96.0% |
| riv_rowm | 0.000 | 98.1% |
| riv_scr | 0.000 | 95.8% |
| riv_snake | 0.000 | 97.4% |
| riv_spec | 0.000 | 96.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 20240445ns | 20240445ns | base |
| riv_rowm | 19133870ns | 19133870ns | -5.47% |
| riv_scr | 19888144ns | 19888144ns | -1.74% |
| riv_snake | 19428928ns | 19428928ns | -4.01% |
| riv_spec | 19332748ns | 19332748ns | -4.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 19079707ns | base | --- | [18999509, 21916989] | --- | --- | --- | --- |
| riv_rowm | 18676594ns | -477212.5ns (-2.5%) | [-2463581, -164161]ns | [18601948, 19152794] | YES | 0.0052 | 0.0013 | 0 |
| riv_scr | 19134084ns | no significant difference | [-1593321, +371262]ns | [18868103, 20402659] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 18818820ns | -208473.5ns (-1.1%) | [-2617828, -111267]ns | [18592377, 19656428] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| riv_spec | 18971424ns | no significant difference | [-1591189, +76428]ns | [18703529, 19230034] | no | 0.6409 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 22033376ns | -15.2% | -16.4% | -16.2% | -15.0% |
| 1 | 2 | 18646583ns | -0.3% | +8.0% | -1.0% | -1.6% |
| 1 | 3 | 19281476ns | -3.2% | +19.0% | -2.6% | -2.0% |
| 2 | 1 | 22919906ns | -16.2% | -17.6% | -11.9% | -4.8% |
| 2 | 2 | 19038326ns | -2.4% | +1.5% | -1.0% | +0.9% |
| 2 | 3 | 19072244ns | -0.8% | -0.5% | -1.4% | +0.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.349 | moderate- |
| riv_rowm | -0.407 | moderate- |
| riv_scr | -0.105 | ok |
| riv_snake | 0.003 | ok |
| riv_spec | -0.049 | ok |

**Consistency summary:**

- **riv_rowm**: won 6/6, lost 0/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 6/6, lost 0/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 92920844.1ns | 20236411.0ns | 459.2% | HIGH |
| riv_rowm | 92568286.2ns | 19129728.1ns | 483.9% | HIGH |
| riv_scr | 92420544.8ns | 19884151.7ns | 464.8% | HIGH |
| riv_snake | 92625428.5ns | 19424820.3ns | 476.8% | HIGH |
| riv_spec | 92504490.0ns | 19328715.6ns | 478.6% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 18614673.3-22495726.3 ns)
  18614673.3 |################
  18808726.0 |################
  19002778.6 |########################################
  19196831.3 |########
  19390883.9 |
  19584936.6 |
  19778989.2 |
  19973041.9 |
  20167094.5 |
  20361147.2 |
  20555199.8 |
  20749252.5 |
  20943305.1 |########
  21137357.8 |
  21331410.4 |
  21525463.1 |
  21719515.7 |########
  21913568.4 |########
  22107621.0 |########
  22301673.7 |
  (1 below, 3 above range)

riv_rowm (n=18, range 18556187.0-20082685.6 ns)
  18556187.0 |########################################
  18632512.0 |####################
  18708836.9 |
  18785161.8 |######
  18861486.7 |######
  18937811.7 |
  19014136.6 |
  19090461.5 |######
  19166786.5 |#############
  19243111.4 |
  19319436.3 |######
  19395761.2 |
  19472086.2 |
  19548411.1 |
  19624736.0 |
  19701061.0 |
  19777385.9 |
  19853710.8 |
  19930035.7 |
  20006360.7 |
  (1 below, 2 above range)

riv_scr (n=18, range 18494822.3-21808334.6 ns)
  18494822.3 |##########
  18660497.9 |##########
  18826173.5 |########################################
  18991849.1 |##########
  19157524.8 |##########
  19323200.4 |####################
  19488876.0 |
  19654551.6 |
  19820227.2 |
  19985902.8 |##########
  20151578.4 |
  20317254.1 |
  20482929.7 |
  20648605.3 |
  20814280.9 |##########
  20979956.5 |
  21145632.1 |
  21311307.7 |##########
  21476983.4 |
  21642659.0 |
  (2 below, 3 above range)

riv_snake (n=18, range 18456680.8-20956650.7 ns)
  18456680.8 |########################################
  18581679.3 |####################
  18706677.8 |####################
  18831676.3 |##############################
  18956674.8 |
  19081673.3 |##########
  19206671.8 |
  19331670.3 |
  19456668.8 |
  19581667.3 |
  19706665.8 |
  19831664.2 |
  19956662.7 |
  20081661.2 |##########
  20206659.7 |
  20331658.2 |
  20456656.7 |
  20581655.2 |
  20706653.7 |
  20831652.2 |##########
  (1 below, 3 above range)

riv_spec (n=18, range 18322611.1-20508465.5 ns)
  18322611.1 |########################################
  18431903.8 |
  18541196.6 |####################
  18650489.3 |########################################
  18759782.0 |########################################
  18869074.7 |####################
  18978367.4 |####################
  19087660.1 |########################################
  19196952.9 |########################################
  19306245.6 |
  19415538.3 |
  19524831.0 |
  19634123.7 |
  19743416.4 |####################
  19852709.2 |
  19962001.9 |
  20071294.6 |
  20180587.3 |
  20289880.0 |####################
  20399172.8 |
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=485.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=494.2% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=481.3% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=489.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=485.7% of algo (FFI overhead may distort results)
