# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: evs_std

## Highlights

Baseline for all deltas below: evs_std. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### evs_engine_par shows alternating (throttle bounce) (autocorr -0.50)

evs_engine_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (evs_std)

The baseline evs_std is the fastest (61.76 us median); no rival improves on it (all deltas are >= 0).


### Two tiers: {evs_std, evs_engine} vs {evs_engine_par, evs_std_par} (35% apart)

The field splits into a fast tier {evs_std, evs_engine} and a slow tier {evs_engine_par, evs_std_par} with a 35% jump between them - a qualitative difference, not a gradient.


### Speed leader evs_std vs stability leader evs_engine (+1% speed for 2.3x steadier)

evs_std is fastest (61.76 us, CV 16.4%); evs_engine gives up 1.2% median for 2.3x lower variance (CV 7.2%).


### evs_engine_par is inconsistent: worst-20% is 3.3x its best-20%

evs_engine_par's best 20% of batches run at 59.07 us but its worst 20% at 193.86 us (3.3x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (evs_std) is the fastest at 61760.9 ns median
- 2 variants significantly slower than baseline
- Spread: 1.38x (fastest 61760.9 ns, slowest 84934.4 ns)

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
| Counter frequency | 2100267710 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| evs_engine | 62768ns | 62707ns | 58108ns | 59315ns | 66524ns | -2.72% |
| evs_engine_par | 132722ns | 84462ns | 59323ns | 74825ns | 194496ns | +105.69% |
| evs_std | 64525ns | 62029ns | 54354ns | 54878ns | 74304ns | base |
| evs_std_par | 119730ns | 85227ns | 68861ns | 82250ns | 160557ns | +85.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| evs_engine | 62551ns | 57929ns | 66287ns | -2.60% | 0.065 |
| evs_engine_par | 132239ns | 59072ns | 193864ns | +105.92% | 0.031 |
| evs_std | 64218ns | 54107ns | 73916ns | base | 0.064 |
| evs_std_par | 119268ns | 68566ns | 160109ns | +85.73% | 0.034 |

## Performance model

- Peak throughput: 0.076 Gops/s (evs_std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| evs_engine | 0.066 | 86.6% |
| evs_engine_par | 0.049 | 64.3% |
| evs_std | 0.066 | 87.6% |
| evs_std_par | 0.048 | 63.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| evs_engine | 62768ns | 62768ns | -2.72% |
| evs_engine_par | 132722ns | 132722ns | +105.69% |
| evs_std | 64525ns | 64525ns | base |
| evs_std_par | 119730ns | 119730ns | +85.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| evs_std | 61761ns | base | --- | [54135, 76551] | --- | --- | --- | --- |
| evs_engine | 62513ns | no significant difference | [-18582, +11424]ns | [57958, 65907] | no | 1.0000 | 1.0000 | 0 |
| evs_engine_par | 84152ns | +24884.2ns (+40.3%) | [+5611, +96653]ns | [63778, 150760] | YES (adj: no) | 0.1055 | 0.0703 | 0 |
| evs_std_par | 84934ns | +28638.1ns (+46.4%) | [+1858, +46087]ns | [78409, 100571] | YES (adj: no) | 0.1055 | 0.0703 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | evs_std | evs_engine | evs_engine_par | evs_std_par |
|---|---|---|---|---|---|
| 1 | 1 | 54309ns | +19.6% | +46.9% | +61.8% |
| 1 | 2 | 74164ns | -21.8% | +40.0% | +10.7% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| evs_engine | -0.500 | moderate- |
| evs_engine_par | -0.500 | HIGH- (thermal bounce) |
| evs_std | -0.500 | moderate- |
| evs_std_par | -0.500 | moderate- |

Consistency summary:

- evs_engine: won 1/2, lost 1/2
- evs_engine_par: won 0/2, lost 2/2
- evs_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| evs_engine | 76540.1ns | 62550.7ns | 122.4% | HIGH |
| evs_engine_par | 88709.0ns | 132238.7ns | 67.1% | HIGH |
| evs_std | 85370.3ns | 64217.8ns | 132.9% | HIGH |
| evs_std_par | 86174.4ns | 119268.5ns | 72.3% | HIGH |

## Distribution (algo ns)

```
evs_engine (n=8, range 57928.7-66287.2 ns)
  57928.7 |########################################
  58346.6 |
  58764.5 |
  59182.5 |
  59600.4 |
  60018.3 |
  60436.2 |
  60854.2 |
  61272.1 |#############
  61690.0 |
  62107.9 |
  62525.9 |
  62943.8 |
  63361.7 |#############
  63779.6 |#############
  64197.6 |
  64615.5 |
  65033.4 |
  65451.3 |
  65869.3 |#############
  (0 below, 1 above range)

evs_engine_par (n=8, range 59072.2-193863.6 ns)
  59072.2 |########################################
  65811.8 |
  72551.3 |####################
  79290.9 |########################################
  86030.5 |
  92770.1 |
  99509.6 |
  106249.2 |
  112988.8 |
  119728.3 |####################
  126467.9 |
  133207.5 |
  139947.1 |
  146686.6 |####################
  153426.2 |
  160165.8 |
  166905.3 |
  173644.9 |
  180384.5 |
  187124.1 |
  (0 below, 1 above range)

evs_std (n=8, range 54107.4-73915.6 ns)
  54107.4 |########################################
  55097.8 |#############
  56088.2 |
  57078.6 |
  58069.1 |
  59059.5 |
  60049.9 |
  61040.3 |
  62030.7 |
  63021.1 |
  64011.5 |
  65001.9 |
  65992.4 |
  66982.8 |
  67973.2 |#############
  68963.6 |
  69954.0 |
  70944.4 |#############
  71934.8 |
  72925.2 |
  (0 below, 2 above range)

evs_std_par (n=8, range 68566.5-160108.8 ns)
  68566.5 |#############
  73143.6 |
  77720.7 |#############
  82297.8 |########################################
  86875.0 |
  91452.1 |#############
  96029.2 |#############
  100606.3 |
  105183.4 |
  109760.5 |
  114337.7 |
  118914.8 |
  123491.9 |
  128069.0 |
  132646.1 |
  137223.2 |
  141800.4 |
  146377.5 |
  150954.6 |
  155531.7 |
  (0 below, 1 above range)

```

## Diagnostics

- evs_engine: bridge=122.9% of algo (FFI overhead may distort results)
- evs_engine_par: CV=84.3% (high variance, measurements may be unstable)
- evs_engine_par: worst_20/best_20 = 3.3x (possible bimodal distribution)
- evs_engine_par: bridge=104.5% of algo (FFI overhead may distort results)
- evs_std: bridge=136.1% of algo (FFI overhead may distort results)
- evs_std_par: CV=77.4% (high variance, measurements may be unstable)
- evs_std_par: bridge=101.3% of algo (FFI overhead may distort results)
