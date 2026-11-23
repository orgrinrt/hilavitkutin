# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (9.61 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_rowm at 9.31 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### riv_rowm is fastest but the noisiest (CV 11.6%)

riv_rowm wins on median (9.31 ms) yet has the highest variance (CV 11.6%), while riv_scr is the steadiest (CV 1.5%, 9.37 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (riv_rowm, riv_snake) are a dead heat (<1%)

riv_rowm (9.31 ms) and riv_snake (9.31 ms) differ by 0.07%, inside the noise, even though the wider field spreads 3.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (304.80 us) is smaller than the fastest variant's own run-to-run std-dev (1.08 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_rowm vs stability leader riv_scr (+1% speed for 7.7x steadier)

riv_rowm is fastest (9.31 ms, CV 11.6%); riv_scr gives up 0.7% median for 7.7x lower variance (CV 1.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.3% of the fastest

All 5 variants sit between 9.31 ms and 9.61 ms - a 3.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_rowm** at 9305437.7 ns median (-3.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.03x (fastest 9305437.7 ns, slowest 9610240.4 ns)

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
| riv_rcm | 9989388ns | 9614349ns | 9200595ns | 9526082ns | 11078743ns | base |
| riv_rowm | 10007285ns | 9310224ns | 9128063ns | 9321725ns | 11475236ns | +0.18% |
| riv_scr | 9346980ns | 9371962ns | 9145850ns | 9309772ns | 9503359ns | -6.43% |
| riv_snake | 9799366ns | 9315579ns | 9137084ns | 9299668ns | 10880053ns | -1.90% |
| riv_spec | 9831502ns | 9500429ns | 9184027ns | 9400468ns | 10801790ns | -1.58% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 9985301ns | 9196492ns | 11074575ns | base | 0.000 |
| riv_rowm | 10003215ns | 9124007ns | 11471325ns | +0.18% | 0.000 |
| riv_scr | 9343084ns | 9141788ns | 9499497ns | -6.43% | 0.000 |
| riv_snake | 9795447ns | 9132978ns | 10876276ns | -1.90% | 0.000 |
| riv_spec | 9827558ns | 9180082ns | 10797898ns | -1.58% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 94.9% |
| riv_rowm | 0.000 | 98.1% |
| riv_scr | 0.000 | 97.4% |
| riv_snake | 0.000 | 98.0% |
| riv_spec | 0.000 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 9989388ns | 9989388ns | base |
| riv_rowm | 10007285ns | 10007285ns | +0.18% |
| riv_scr | 9346980ns | 9346980ns | -6.43% |
| riv_snake | 9799366ns | 9799366ns | -1.90% |
| riv_spec | 9831502ns | 9831502ns | -1.58% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 9610240ns | base | --- | [9344265, 10506207] | --- | --- | --- | --- |
| riv_rowm | 9305438ns | no significant difference | [-133493, +13091]ns | [9148901, 11144947] | no | 0.1283 | 0.0963 | 0 |
| riv_scr | 9368073ns | -174289.7ns (-1.8%) | [-1081233, -89588]ns | [9201631, 9437750] | YES | 0.0006 | 0.0001 | 0 |
| riv_snake | 9311733ns | no significant difference | [-1260108, +789470]ns | [9237404, 10289727] | no | 0.1283 | 0.0963 | 0 |
| riv_spec | 9496451ns | no significant difference | [-756014, +82024]ns | [9246273, 10115206] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 9447051ns | -0.3% | -1.1% | +18.3% | +0.4% |
| 1 | 2 | 9208839ns | -0.9% | -0.8% | +21.0% | +15.8% |
| 1 | 3 | 9230106ns | -1.0% | -0.3% | -1.0% | +0.2% |
| 2 | 1 | 10640298ns | -13.9% | -11.8% | -13.2% | -13.3% |
| 2 | 2 | 9633582ns | +20.5% | -2.2% | -3.5% | +16.3% |
| 2 | 3 | 11601729ns | +0.0% | -17.9% | -18.6% | -17.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.069 | ok |
| riv_rowm | 0.419 | moderate+ |
| riv_scr | 0.378 | moderate+ |
| riv_snake | 0.381 | moderate+ |
| riv_spec | -0.491 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 1/6
- **riv_scr**: won 6/6, lost 0/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 46264645.3ns | 9985300.9ns | 463.3% | HIGH |
| riv_rowm | 46479627.7ns | 10003215.2ns | 464.6% | HIGH |
| riv_scr | 46123251.6ns | 9343084.3ns | 493.7% | HIGH |
| riv_snake | 46264905.6ns | 9795446.9ns | 472.3% | HIGH |
| riv_spec | 46229866.4ns | 9827558.0ns | 470.4% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 9196491.5-11074575.1 ns)
  9196491.5 |########################################
  9290395.7 |
  9384299.9 |####################
  9478204.0 |##########
  9572108.2 |##############################
  9666012.4 |
  9759916.6 |
  9853820.8 |
  9947724.9 |##########
  10041629.1 |
  10135533.3 |
  10229437.5 |
  10323341.7 |##########
  10417245.8 |
  10511150.0 |
  10605054.2 |##########
  10698958.4 |
  10792862.6 |##########
  10886766.7 |
  10980670.9 |
  (1 below, 3 above range)

riv_rowm (n=18, range 9124007.5-11471324.6 ns)
  9124007.5 |########################################
  9241373.3 |
  9358739.2 |##########
  9476105.0 |
  9593470.9 |
  9710836.8 |
  9828202.6 |
  9945568.5 |#####
  10062934.3 |
  10180300.2 |
  10297666.0 |
  10415031.9 |
  10532397.7 |
  10649763.6 |#####
  10767129.5 |
  10884495.3 |
  11001861.2 |
  11119227.0 |
  11236592.9 |
  11353958.7 |
  (1 below, 5 above range)

riv_scr (n=18, range 9141787.9-9499496.7 ns)
  9141787.9 |#############
  9159673.4 |#############
  9177558.8 |
  9195444.2 |##########################
  9213329.7 |
  9231215.1 |
  9249100.5 |
  9266986.0 |
  9284871.4 |
  9302756.9 |
  9320642.3 |
  9338527.7 |##########################
  9356413.2 |########################################
  9374298.6 |#############
  9392184.0 |
  9410069.5 |#############
  9427954.9 |
  9445840.3 |#############
  9463725.8 |
  9481611.2 |#############
  (2 below, 3 above range)

riv_snake (n=18, range 9132978.0-10876275.6 ns)
  9132978.0 |########################
  9220142.9 |########################################
  9307307.8 |################
  9394472.6 |################
  9481637.5 |
  9568802.4 |
  9655967.3 |
  9743132.2 |
  9830297.0 |
  9917461.9 |
  10004626.8 |
  10091791.7 |
  10178956.6 |
  10266121.5 |
  10353286.3 |
  10440451.2 |
  10527616.1 |
  10614781.0 |
  10701945.9 |
  10789110.7 |
  (1 below, 5 above range)

riv_spec (n=18, range 9180081.8-10797897.6 ns)
  9180081.8 |########################################
  9260972.6 |########
  9341863.4 |
  9422754.2 |################
  9503645.0 |################################
  9584535.8 |
  9665426.6 |
  9746317.3 |
  9827208.1 |
  9908098.9 |
  9988989.7 |
  10069880.5 |
  10150771.3 |
  10231662.1 |
  10312552.8 |
  10393443.6 |
  10474334.4 |
  10555225.2 |
  10636116.0 |########
  10717006.8 |
  (1 below, 4 above range)

```

## Diagnostics

- **riv_rcm**: bridge=479.9% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=497.6% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=491.5% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=494.8% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=485.2% of algo (FFI overhead may distort results)
