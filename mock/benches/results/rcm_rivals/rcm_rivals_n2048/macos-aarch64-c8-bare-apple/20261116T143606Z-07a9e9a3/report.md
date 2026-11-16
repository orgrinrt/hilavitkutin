# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_rcm is fastest but the noisiest (CV 23.1%)

riv_rcm wins on median (8.12 ms) yet has the highest variance (CV 23.1%), while riv_scr is the steadiest (CV 11.2%, 8.50 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (456.89 us) is smaller than the fastest variant's own run-to-run std-dev (1.88 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (8.12 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader riv_rcm vs stability leader riv_scr (+5% speed for 2.1x steadier)

riv_rcm is fastest (8.12 ms, CV 23.1%); riv_scr gives up 4.8% median for 2.1x lower variance (CV 11.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### riv_rcm is inconsistent: worst-20% is 1.5x its best-20%

riv_rcm's best 20% of batches run at 7.95 ms but its worst 20% at 11.92 ms (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 8117604.2 ns median
- Spread: 1.06x (fastest 8117604.2 ns, slowest 8574495.8 ns)

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
| riv_rcm | 9483966ns | 8123679ns | 7954108ns | 8363301ns | 11929893ns | base |
| riv_rowm | 9065805ns | 8580373ns | 8070539ns | 8515375ns | 10389082ns | -4.41% |
| riv_scr | 8961811ns | 8510473ns | 8093082ns | 8422991ns | 10204406ns | -5.51% |
| riv_snake | 9236559ns | 8447950ns | 7942886ns | 8507182ns | 10977460ns | -2.61% |
| riv_spec | 8895875ns | 8344098ns | 7883592ns | 8252847ns | 10366557ns | -6.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 9477666ns | 7948090ns | 11922812ns | base | 0.000 |
| riv_rowm | 9059581ns | 8065474ns | 10381800ns | -4.41% | 0.000 |
| riv_scr | 8955489ns | 8087646ns | 10197231ns | -5.51% | 0.000 |
| riv_snake | 9229796ns | 7937271ns | 10969040ns | -2.62% | 0.000 |
| riv_spec | 8889634ns | 7877444ns | 10359013ns | -6.20% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.0% |
| riv_rowm | 0.000 | 91.9% |
| riv_scr | 0.000 | 92.6% |
| riv_snake | 0.000 | 93.3% |
| riv_spec | 0.000 | 94.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 9483966ns | 9483966ns | base |
| riv_rowm | 9065805ns | 9065805ns | -4.41% |
| riv_scr | 8961811ns | 8961811ns | -5.51% |
| riv_snake | 9236559ns | 9236559ns | -2.61% |
| riv_spec | 8895875ns | 8895875ns | -6.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 8117604ns | base | --- | [8052298, 10780538] | --- | --- | --- | --- |
| riv_rowm | 8574496ns | no significant difference | [-1161188, +304156]ns | [8311890, 9759129] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 8504465ns | no significant difference | [-989423, +314352]ns | [8242965, 9557023] | no | 0.8145 | 0.8145 | 0 |
| riv_snake | 8442367ns | no significant difference | [-355342, +117208]ns | [8086096, 10421365] | no | 0.8145 | 0.4807 | 0 |
| riv_spec | 8339140ns | no significant difference | [-866323, +76858]ns | [8051633, 9793942] | no | 0.8145 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 8039908ns | +5.8% | +2.1% | +4.5% | +1.7% |
| 1 | 2 | 8085962ns | +2.5% | +5.0% | -0.9% | -0.0% |
| 1 | 3 | 8103188ns | +1.8% | +2.1% | -1.4% | -0.6% |
| 2 | 1 | 8011021ns | +7.9% | +3.5% | +1.8% | -1.6% |
| 2 | 2 | 10822508ns | -1.9% | -3.8% | -2.0% | -2.5% |
| 2 | 3 | 12351583ns | -16.1% | -20.0% | -15.2% | -20.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.398 | moderate+ |
| riv_rowm | 0.477 | moderate+ |
| riv_scr | 0.322 | moderate+ |
| riv_snake | 0.432 | moderate+ |
| riv_spec | 0.289 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 4/6
- **riv_scr**: won 2/6, lost 4/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 138680855.6ns | 9477666.2ns | 1463.2% | HIGH |
| riv_rowm | 137298517.1ns | 9059581.0ns | 1515.5% | HIGH |
| riv_scr | 137412043.1ns | 8955489.4ns | 1534.4% | HIGH |
| riv_snake | 139182411.6ns | 9229795.8ns | 1508.0% | HIGH |
| riv_spec | 137760125.7ns | 8889633.6ns | 1549.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 7948090.3-11922812.5 ns)
  7948090.3 |########################################
  8146826.4 |
  8345562.5 |####
  8544298.6 |
  8743034.7 |
  8941770.8 |
  9140506.9 |
  9339243.0 |
  9537979.2 |
  9736715.3 |
  9935451.4 |
  10134187.5 |####
  10332923.6 |
  10531659.7 |
  10730395.8 |########
  10929131.9 |
  11127868.0 |
  11326604.2 |
  11525340.3 |
  11724076.4 |####
  (1 below, 3 above range)

riv_rowm (n=18, range 8065473.6-10381800.0 ns)
  8065473.6 |########################################
  8181290.0 |########################################
  8297106.3 |########################################
  8412922.6 |########################################
  8528738.9 |####################
  8644555.2 |####################
  8760371.5 |
  8876187.9 |
  8992004.2 |####################
  9107820.5 |
  9223636.8 |
  9339453.1 |
  9455269.4 |####################
  9571085.8 |
  9686902.1 |
  9802718.4 |
  9918534.7 |####################
  10034351.0 |
  10150167.3 |
  10265983.7 |####################
  (1 below, 3 above range)

riv_scr (n=18, range 8087645.8-10197230.6 ns)
  8087645.8 |##########################
  8193125.1 |########################################
  8298604.3 |#############
  8404083.5 |#############
  8509562.8 |##########################
  8615042.0 |
  8720521.3 |
  8826000.5 |#############
  8931479.7 |
  9036959.0 |
  9142438.2 |
  9247917.4 |#############
  9353396.7 |
  9458875.9 |
  9564355.1 |
  9669834.4 |
  9775313.6 |
  9880792.9 |#############
  9986272.1 |
  10091751.3 |#############
  (2 below, 3 above range)

riv_snake (n=18, range 7937270.8-10969040.3 ns)
  7937270.8 |########################################
  8088859.3 |####################
  8240447.8 |##########
  8392036.3 |####################
  8543624.7 |##########
  8695213.2 |
  8846801.7 |
  8998390.1 |
  9149978.6 |
  9301567.1 |
  9453155.6 |
  9604744.0 |
  9756332.5 |
  9907921.0 |
  10059509.4 |
  10211097.9 |##########
  10362686.4 |####################
  10514274.9 |####################
  10665863.3 |##########
  10817451.8 |
  (1 below, 1 above range)

riv_spec (n=18, range 7877444.5-10359012.5 ns)
  7877444.5 |####################
  8001522.9 |########################################
  8125601.3 |####################
  8249679.7 |
  8373758.1 |##########
  8497836.5 |##########
  8621914.9 |##########
  8745993.3 |
  8870071.7 |
  8994150.1 |
  9118228.5 |
  9242306.9 |
  9366385.3 |
  9490463.7 |
  9614542.1 |
  9738620.5 |####################
  9862698.9 |
  9986777.3 |
  10110855.7 |
  10234934.1 |##########
  (1 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1686.1% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1601.5% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1614.6% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1628.5% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1643.4% of algo (FFI overhead may distort results)
