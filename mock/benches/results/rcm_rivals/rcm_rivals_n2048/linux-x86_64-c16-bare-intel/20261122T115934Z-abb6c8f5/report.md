# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_rcm shows alternating (throttle bounce) (autocorr -0.72)

riv_rcm's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (156.47 us) is smaller than the fastest variant's own run-to-run std-dev (281.25 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (18.75 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.8% of the fastest

All 5 variants sit between 18.75 ms and 18.90 ms - a 0.8% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 18748427.6 ns median
- 3 variants significantly slower than baseline
- Spread: 1.01x (fastest 18748427.6 ns, slowest 18904892.6 ns)

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
| riv_rcm | 18812961ns | 18752382ns | 18627860ns | 18719460ns | 19045762ns | base |
| riv_rowm | 19077332ns | 18890171ns | 18711794ns | 18858051ns | 19589024ns | +1.41% |
| riv_scr | 19161191ns | 18908804ns | 18803917ns | 18889171ns | 19747856ns | +1.85% |
| riv_snake | 18963882ns | 18869718ns | 18640864ns | 18810821ns | 19354983ns | +0.80% |
| riv_spec | 18983394ns | 18823411ns | 18715859ns | 18814678ns | 19370235ns | +0.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 18808949ns | 18623670ns | 19041740ns | base | 0.000 |
| riv_rowm | 19073261ns | 18707688ns | 19584930ns | +1.41% | 0.000 |
| riv_scr | 19157224ns | 18800161ns | 19743852ns | +1.85% | 0.000 |
| riv_snake | 18959896ns | 18637015ns | 19350916ns | +0.80% | 0.000 |
| riv_spec | 18979440ns | 18711825ns | 19366408ns | +0.91% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rcm; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 99.3% |
| riv_rowm | 0.000 | 98.6% |
| riv_scr | 0.000 | 98.5% |
| riv_snake | 0.000 | 98.7% |
| riv_spec | 0.000 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 18812961ns | 18812961ns | base |
| riv_rowm | 19077332ns | 19077332ns | +1.41% |
| riv_scr | 19161191ns | 19161191ns | +1.85% |
| riv_snake | 18963882ns | 18963882ns | +0.80% |
| riv_spec | 18983394ns | 18983394ns | +0.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 18748428ns | base | --- | [18671231, 18798875] | --- | --- | --- | --- |
| riv_rowm | 18886180ns | +148010.6ns (+0.8%) | [+75366, +186545]ns | [18805958, 18927893] | YES | 0.0412 | 0.0309 | 0 |
| riv_scr | 18904893ns | +204251.9ns (+1.1%) | [+98692, +282502]ns | [18854954, 19005899] | YES | 0.0052 | 0.0013 | 0 |
| riv_snake | 18865924ns | no significant difference | [-54555, +225871]ns | [18736725, 18949124] | no | 0.4807 | 0.4807 | 0 |
| riv_spec | 18819286ns | +108701.7ns (+0.6%) | [+36292, +159494]ns | [18787927, 18883291] | YES | 0.0412 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 18776122ns | +0.8% | +1.7% | +1.0% | +0.6% |
| 1 | 2 | 18678927ns | +0.6% | +1.3% | -0.2% | +0.6% |
| 1 | 3 | 18815355ns | +0.2% | +0.1% | +0.6% | +0.7% |
| 2 | 1 | 18658934ns | +1.4% | +1.7% | +1.1% | +0.7% |
| 2 | 2 | 18816448ns | +1.6% | -0.1% | +4.3% | +0.1% |
| 2 | 3 | 18775675ns | -0.4% | +0.7% | -0.5% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.717 | HIGH- (thermal bounce) |
| riv_rowm | -0.376 | moderate- |
| riv_scr | -0.186 | ok |
| riv_snake | -0.374 | moderate- |
| riv_spec | -0.443 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 1/6, lost 5/6
- **riv_scr**: won 0/6, lost 4/6
- **riv_snake**: won 2/6, lost 4/6
- **riv_spec**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 92227132.5ns | 18808948.7ns | 490.3% | HIGH |
| riv_rowm | 92331927.8ns | 19073260.9ns | 484.1% | HIGH |
| riv_scr | 92323473.9ns | 19157223.9ns | 481.9% | HIGH |
| riv_snake | 92600407.8ns | 18959896.2ns | 488.4% | HIGH |
| riv_spec | 92391178.0ns | 18979440.5ns | 486.8% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 18623669.6-19041739.9 ns)
  18623669.6 |##########################
  18644573.1 |##########################
  18665476.6 |##########################
  18686380.1 |#############
  18707283.6 |#############
  18728187.1 |
  18749090.7 |#############
  18769994.2 |########################################
  18790897.7 |
  18811801.2 |########################################
  18832704.7 |
  18853608.2 |
  18874511.7 |
  18895415.3 |
  18916318.8 |
  18937222.3 |
  18958125.8 |
  18979029.3 |
  18999932.8 |
  19020836.3 |
  (1 below, 2 above range)

riv_rowm (n=18, range 18707687.8-19584929.9 ns)
  18707687.8 |##########
  18751549.9 |####################
  18795412.0 |####################
  18839274.1 |####################
  18883136.2 |########################################
  18926998.3 |####################
  18970860.4 |##########
  19014722.5 |
  19058584.6 |
  19102446.7 |##########
  19146308.9 |
  19190171.0 |
  19234033.1 |
  19277895.2 |
  19321757.3 |
  19365619.4 |
  19409481.5 |
  19453343.6 |
  19497205.7 |
  19541067.8 |
  (2 below, 1 above range)

riv_scr (n=18, range 18800160.6-19743852.5 ns)
  18800160.6 |################################
  18847345.2 |################
  18894529.8 |########################################
  18941714.4 |########
  18988899.0 |########
  19036083.6 |
  19083268.2 |################
  19130452.8 |
  19177637.4 |
  19224822.0 |
  19272006.5 |
  19319191.1 |
  19366375.7 |
  19413560.3 |
  19460744.9 |
  19507929.5 |
  19555114.1 |
  19602298.7 |
  19649483.3 |
  19696667.9 |
  (1 below, 2 above range)

riv_snake (n=18, range 18637015.3-19350915.8 ns)
  18637015.3 |########################################
  18672710.4 |
  18708405.4 |##########################
  18744100.4 |#############
  18779795.4 |
  18815490.4 |#############
  18851185.5 |##########################
  18886880.5 |##########################
  18922575.5 |#############
  18958270.5 |#############
  18993965.5 |
  19029660.6 |
  19065355.6 |
  19101050.6 |#############
  19136745.6 |
  19172440.7 |
  19208135.7 |
  19243830.7 |
  19279525.7 |
  19315220.7 |
  (1 below, 3 above range)

riv_spec (n=18, range 18711825.0-19366408.1 ns)
  18711825.0 |################
  18744554.2 |
  18777283.3 |########################################
  18810012.5 |################
  18842741.6 |################
  18875470.8 |################
  18908199.9 |
  18940929.1 |################
  18973658.2 |
  19006387.4 |
  19039116.5 |########
  19071845.7 |
  19104574.8 |
  19137304.0 |
  19170033.1 |
  19202762.3 |
  19235491.5 |
  19268220.6 |
  19300949.8 |
  19333678.9 |
  (1 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=491.6% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=487.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=487.5% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=488.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=489.7% of algo (FFI overhead may distort results)
