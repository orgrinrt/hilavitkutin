# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_std_par is fastest but the noisiest (CV 14.2%)

evs_std_par wins on median (630.37 us) yet has the highest variance (CV 14.2%), while evs_engine is the steadiest (CV 0.2%, 674.34 us).


### Top two (evs_std_par, evs_engine_par) are a dead heat (<1%)

evs_std_par (630.37 us) and evs_engine_par (634.90 us) differ by 0.72%, inside the noise, even though the wider field spreads 7.0%.


### evs_std_par shows alternating (throttle bounce) (autocorr -0.50)

evs_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (43.97 us) is smaller than the fastest variant's own run-to-run std-dev (89.79 us); the ranking is inside the noise.


### Speed leader evs_std_par vs stability leader evs_engine (+7% speed for 74.5x steadier)

evs_std_par is fastest (630.37 us, CV 14.2%); evs_engine gives up 7.0% median for 74.5x lower variance (CV 0.2%).


## Key findings

- Fastest: evs_std_par at 630365.4 ns median (-5.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.07x (fastest 630365.4 ns, slowest 674337.8 ns)

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
| Counter frequency | 2112761676 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 674096ns | 674511ns | 671915ns | 673469ns | 675111ns | +0.84% |
| evs_engine_par | 634212ns | 635419ns | 551427ns | 613235ns | 670642ns | -5.13% |
| evs_std | 668473ns | 668438ns | 666227ns | 667376ns | 669857ns | base |
| evs_std_par | 652089ns | 631002ns | 550586ns | 614848ns | 705396ns | -2.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 673882ns | 671640ns | 674914ns | +0.84% | 0.097 |
| evs_engine_par | 633667ns | 550986ns | 670078ns | -5.18% | 0.103 |
| evs_std | 668269ns | 666054ns | 669643ns | base | 0.098 |
| evs_std_par | 651413ns | 550346ns | 704567ns | -2.52% | 0.101 |

## Performance model

- Peak throughput: 0.119 Gops/s (evs_std_par; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.097 | 81.6% |
| evs_engine_par | 0.103 | 86.7% |
| evs_std | 0.098 | 82.4% |
| evs_std_par | 0.104 | 87.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 674096ns | 674096ns | +0.84% |
| evs_engine_par | 634212ns | 634212ns | -5.13% |
| evs_std | 668473ns | 668473ns | base |
| evs_std_par | 652089ns | 652089ns | -2.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 668269ns | base | --- | [666804, 669920] | --- | --- | --- | --- |
| evs_engine | 674338ns | +5370.5ns (+0.8%) | [+4428, +8246]ns | [672477, 675137] | YES | 0.0234 | 0.0078 | 0 |
| evs_engine_par | 634905ns | no significant difference | [-84157, +4159]ns | [585902, 674080] | no | 0.7266 | 0.7266 | 0 |
| evs_std_par | 630365ns | -38466.8ns (-5.8%) | [-61173, -9603]ns | [608886, 659192] | YES (adj: no) | 0.1055 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 667359ns | +1.0% | -6.2% | -4.0% |
| 1 | 2 | 669859ns | +0.7% | +0.1% | -6.6% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | moderate- |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 0/2, lost 2/2
- evs_engine_par: won 1/2, lost 1/2
- evs_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 986552.9ns | 673881.8ns | 146.4% | HIGH |
| evs_engine_par | 1073369.8ns | 633667.0ns | 169.4% | HIGH |
| evs_std | 985865.0ns | 668268.6ns | 147.5% | HIGH |
| evs_std_par | 1140725.5ns | 651412.5ns | 175.1% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 671640.5-674913.7 ns)
  671640.5 |####################
  671804.2 |
  671967.8 |
  672131.5 |
  672295.1 |
  672458.8 |####################
  672622.5 |
  672786.1 |
  672949.8 |####################
  673113.5 |
  673277.1 |
  673440.8 |
  673604.4 |
  673768.1 |
  673931.8 |
  674095.4 |
  674259.1 |########################################
  674422.7 |####################
  674586.4 |
  674750.1 |
  (0 below, 2 above range)

evs_engine_par (n=8, range 550985.9-670077.7 ns)
  550985.9 |####################
  556940.5 |
  562895.1 |
  568849.7 |
  574804.3 |
  580758.9 |####################
  586713.4 |
  592668.0 |
  598622.6 |
  604577.2 |
  610531.8 |
  616486.4 |####################
  622441.0 |
  628395.6 |
  634350.2 |########################################
  640304.8 |
  646259.3 |
  652213.9 |
  658168.5 |
  664123.1 |####################
  (0 below, 2 above range)

evs_std (n=8, range 666054.2-669643.1 ns)
  666054.2 |########################################
  666233.6 |
  666413.1 |
  666592.5 |
  666772.0 |########################################
  666951.4 |########################################
  667130.9 |
  667310.3 |
  667489.8 |
  667669.2 |########################################
  667848.6 |
  668028.1 |
  668207.5 |
  668387.0 |
  668566.4 |
  668745.9 |########################################
  668925.3 |
  669104.8 |
  669284.2 |
  669463.7 |
  (0 below, 3 above range)

evs_std_par (n=8, range 550345.9-704566.8 ns)
  550345.9 |####################
  558056.9 |
  565768.0 |
  573479.0 |
  581190.1 |
  588901.1 |
  596612.2 |
  604323.2 |########################################
  612034.3 |
  619745.3 |####################
  627456.4 |
  635167.4 |####################
  642878.4 |####################
  650589.5 |
  658300.5 |####################
  666011.6 |
  673722.6 |
  681433.7 |
  689144.7 |
  696855.8 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=146.3% of algo (FFI overhead may distort results)
- evs_engine_par: bridge=158.5% of algo (FFI overhead may distort results)
- evs_std: bridge=147.5% of algo (FFI overhead may distort results)
- evs_std_par: bridge=161.2% of algo (FFI overhead may distort results)
