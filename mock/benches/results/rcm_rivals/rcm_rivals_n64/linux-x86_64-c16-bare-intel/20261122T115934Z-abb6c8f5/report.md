# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (riv_spec, riv_rowm) are a dead heat (<1%)

riv_spec (457.89 us) and riv_rowm (460.73 us) differ by 0.62%, inside the noise, even though the wider field spreads 3.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (15.78 us) is smaller than the fastest variant's own run-to-run std-dev (16.34 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 3.4% of the fastest

All 5 variants sit between 457.89 us and 473.67 us - a 3.4% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_spec** at 457885.7 ns median (-1.7% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 457885.7 ns, slowest 473668.7 ns)

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
| Counter frequency | 2112741408 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 473068ns | 467951ns | 418408ns | 465232ns | 512153ns | base |
| riv_rowm | 450630ns | 462786ns | 417180ns | 447680ns | 471780ns | -4.74% |
| riv_scr | 492643ns | 475941ns | 461216ns | 473389ns | 537237ns | +4.14% |
| riv_snake | 484138ns | 466817ns | 450561ns | 463606ns | 531723ns | +2.34% |
| riv_spec | 462442ns | 460015ns | 445200ns | 457910ns | 477863ns | -2.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 470917ns | 416376ns | 509929ns | base | 0.000 |
| riv_rowm | 448522ns | 415187ns | 469594ns | -4.76% | 0.000 |
| riv_scr | 490313ns | 459059ns | 534672ns | +4.12% | 0.000 |
| riv_snake | 481911ns | 448355ns | 529455ns | +2.33% | 0.000 |
| riv_spec | 460209ns | 443037ns | 475506ns | -2.27% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 89.1% |
| riv_rowm | 0.000 | 90.1% |
| riv_scr | 0.000 | 87.7% |
| riv_snake | 0.000 | 89.4% |
| riv_spec | 0.000 | 90.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 473068ns | 473068ns | base |
| riv_rowm | 450630ns | 450630ns | -4.74% |
| riv_scr | 492643ns | 492643ns | +4.14% |
| riv_snake | 484138ns | 484138ns | +2.34% |
| riv_spec | 462442ns | 462442ns | -2.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 465873ns | base | --- | [460193, 472963] | --- | --- | --- | --- |
| riv_rowm | 460728ns | no significant difference | [-51332, +2751]ns | [418282, 464559] | no | 0.4758 | 0.2379 | 0 |
| riv_scr | 473669ns | +6987.9ns (+1.5%) | [+4735, +27395]ns | [466234, 511051] | YES | 0.0302 | 0.0075 | 0 |
| riv_snake | 464633ns | no significant difference | [-9993, +42785]ns | [455949, 525639] | no | 0.4807 | 0.4807 | 0 |
| riv_spec | 457886ns | no significant difference | [-21024, +8643]ns | [452620, 460771] | no | 0.4807 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 459632ns | -0.0% | +0.8% | +15.4% | -0.4% |
| 1 | 2 | 415989ns | +0.1% | +11.9% | +11.5% | +8.4% |
| 1 | 3 | 466609ns | -10.9% | +1.4% | -0.3% | -3.6% |
| 2 | 1 | 462487ns | +2.5% | +17.1% | -1.6% | -0.6% |
| 2 | 2 | 470118ns | -1.7% | +1.9% | -4.7% | +0.8% |
| 2 | 3 | 538919ns | -13.6% | +1.2% | -2.4% | -14.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.090 | ok |
| riv_rowm | 0.140 | ok |
| riv_scr | -0.132 | ok |
| riv_snake | -0.112 | ok |
| riv_spec | 0.297 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 1/6
- **riv_scr**: won 0/6, lost 6/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 2879868.2ns | 470916.6ns | 611.5% | HIGH |
| riv_rowm | 2870680.4ns | 448522.1ns | 640.0% | HIGH |
| riv_scr | 2879828.8ns | 490313.0ns | 587.3% | HIGH |
| riv_snake | 2875857.9ns | 481911.0ns | 596.8% | HIGH |
| riv_spec | 2875820.0ns | 460209.4ns | 624.9% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 416375.8-509929.3 ns)
  416375.8 |##########
  421053.4 |
  425731.1 |
  430408.8 |
  435086.5 |
  439764.2 |
  444441.8 |
  449119.5 |
  453797.2 |##########
  458474.9 |########################################
  463152.5 |########################################
  467830.2 |##########
  472507.9 |##########
  477185.6 |
  481863.3 |##########
  486540.9 |
  491218.6 |
  495896.3 |
  500574.0 |
  505251.7 |
  (2 below, 3 above range)

riv_rowm (n=18, range 415186.7-469594.3 ns)
  415186.7 |########################################
  417907.0 |##########
  420627.4 |
  423347.8 |
  426068.2 |
  428788.6 |
  431509.0 |
  434229.3 |
  436949.7 |
  439670.1 |
  442390.5 |
  445110.9 |
  447831.2 |
  450551.6 |
  453272.0 |##########
  455992.4 |##########
  458712.8 |##########
  461433.2 |########################################
  464153.5 |####################
  466873.9 |
  (1 below, 3 above range)

riv_scr (n=18, range 459059.3-534672.5 ns)
  459059.3 |##########
  462840.0 |####################
  466620.6 |##########
  470401.3 |########################################
  474182.0 |##########
  477962.6 |####################
  481743.3 |
  485523.9 |
  489304.6 |
  493085.2 |
  496865.9 |
  500646.5 |
  504427.2 |
  508207.9 |
  511988.5 |
  515769.2 |
  519549.8 |
  523330.5 |
  527111.1 |
  530891.8 |
  (2 below, 5 above range)

riv_snake (n=18, range 448355.2-529454.8 ns)
  448355.2 |#############
  452410.2 |##########################
  456465.2 |#############
  460520.2 |########################################
  464575.1 |##########################
  468630.1 |#############
  472685.1 |
  476740.1 |
  480795.0 |
  484850.0 |
  488905.0 |
  492960.0 |
  497014.9 |
  501069.9 |
  505124.9 |
  509179.9 |
  513234.8 |
  517289.8 |
  521344.8 |
  525399.8 |########################################
  (2 below, 3 above range)

riv_spec (n=18, range 443037.4-475505.9 ns)
  443037.4 |
  444660.8 |#############
  446284.2 |
  447907.6 |#############
  449531.1 |##########################
  451154.5 |
  452777.9 |
  454401.4 |##########################
  456024.8 |#############
  457648.2 |##########################
  459271.6 |########################################
  460895.1 |#############
  462518.5 |
  464141.9 |
  465765.4 |
  467388.8 |
  469012.2 |#############
  470635.6 |#############
  472259.1 |#############
  473882.5 |
  (1 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=617.4% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=623.0% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=607.2% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=618.7% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=627.6% of algo (FFI overhead may distort results)
