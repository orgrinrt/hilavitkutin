# wide_parallel, four independent heavy chains

4 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_par shows alternating (throttle bounce) (autocorr -0.50)

std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader std vs stability leader engine (+6% speed for 2.0x steadier)

std is fastest (53.12 us, CV 14.5%); engine gives up 6.4% median for 2.0x lower variance (CV 7.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### std_par is inconsistent: worst-20% is 4.1x its best-20%

std_par's best 20% of batches run at 50.90 us but its worst 20% at 210.15 us (4.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std** at 53118.7 ns median (-6.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.45x (fastest 53118.7 ns, slowest 76879.1 ns)

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
| Counter frequency | 2300408230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 58290ns | 56786ns | 55116ns | 55528ns | 61154ns | base |
| engine_par | 76532ns | 79165ns | 61945ns | 73397ns | 82530ns | +31.30% |
| std | 54010ns | 53348ns | 45922ns | 46568ns | 61613ns | -7.34% |
| std_par | 133993ns | 65798ns | 51348ns | 59192ns | 210755ns | +129.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 58067ns | 54976ns | 60966ns | base | 0.071 |
| engine_par | 75568ns | 61741ns | 81270ns | +30.14% | 0.054 |
| std | 53799ns | 45759ns | 61386ns | -7.35% | 0.076 |
| std_par | 133476ns | 50896ns | 210147ns | +129.86% | 0.031 |

## Performance model

- Peak throughput: **0.090 Gops/s** (std; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.072 | 81.0% |
| engine_par | 0.053 | 59.5% |
| std | 0.077 | 86.1% |
| std_par | 0.063 | 70.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 58290ns | 58290ns | base |
| engine_par | 76532ns | 76532ns | +31.30% |
| std | 54010ns | 54010ns | -7.34% |
| std_par | 133993ns | 133993ns | +129.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 56525ns | base | --- | [54976, 61799] | --- | --- | --- | --- |
| engine_par | 76879ns | +21902.8ns (+38.7%) | [+5052, +22833]ns | [69332, 80233] | YES (adj: no) | 0.2109 | 0.0703 | 0 |
| std | 53119ns | no significant difference | [-11494, +4653]ns | [45906, 61144] | no | 0.7266 | 0.7266 | 0 |
| std_par | 65293ns | no significant difference | [-1406, +204670]ns | [53570, 259647] | no | 0.4336 | 0.2891 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | std | std_par |
|---|---|---|---|---|---|
| 1 | 1 | 56525ns | +38.4% | -18.2% | +6.7% |
| 1 | 2 | 56293ns | +33.0% | +7.8% | +187.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| std | -0.500 | moderate- |
| std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 0/2, lost 2/2
- **std**: won 1/2, lost 1/2
- **std_par**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 68976.7ns | 58067.3ns | 118.8% | HIGH |
| engine_par | 74560.7ns | 75567.9ns | 98.7% | HIGH |
| std | 70975.0ns | 53798.7ns | 131.9% | HIGH |
| std_par | 77384.7ns | 133476.2ns | 58.0% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 54976.2-60965.9 ns)
  54976.2 |########################################
  55275.7 |
  55575.2 |#############
  55874.7 |
  56174.1 |
  56473.6 |
  56773.1 |
  57072.6 |
  57372.1 |##########################
  57671.6 |
  57971.1 |
  58270.5 |
  58570.0 |
  58869.5 |
  59169.0 |
  59468.5 |
  59768.0 |
  60067.5 |
  60367.0 |
  60666.4 |
  (0 below, 2 above range)

engine_par (n=8, range 61741.3-81270.4 ns)
  61741.3 |####################
  62717.8 |
  63694.2 |
  64670.7 |
  65647.1 |
  66623.6 |
  67600.0 |
  68576.5 |####################
  69552.9 |
  70529.4 |
  71505.9 |####################
  72482.3 |
  73458.8 |
  74435.2 |
  75411.7 |####################
  76388.1 |
  77364.6 |####################
  78341.1 |
  79317.5 |########################################
  80294.0 |
  (0 below, 1 above range)

std (n=8, range 45758.6-61386.1 ns)
  45758.6 |########################################
  46540.0 |########################################
  47321.3 |
  48102.7 |
  48884.1 |
  49665.5 |
  50446.8 |
  51228.2 |
  52009.6 |
  52791.0 |
  53572.3 |
  54353.7 |
  55135.1 |
  55916.5 |
  56697.8 |
  57479.2 |
  58260.6 |
  59042.0 |####################
  59823.3 |####################
  60604.7 |####################
  (0 below, 1 above range)

std_par (n=8, range 50896.3-210147.0 ns)
  50896.3 |########################################
  58858.8 |########################################
  66821.4 |########################################
  74783.9 |
  82746.4 |
  90709.0 |
  98671.5 |
  106634.0 |
  114596.6 |
  122559.1 |
  130521.6 |
  138484.2 |
  146446.7 |
  154409.2 |
  162371.8 |
  170334.3 |
  178296.8 |
  186259.4 |
  194221.9 |
  202184.4 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: bridge=120.8% of algo (FFI overhead may distort results)
- **engine_par**: bridge=97.3% of algo (FFI overhead may distort results)
- **std**: bridge=134.0% of algo (FFI overhead may distort results)
- **std_par**: CV=100.4% (high variance, measurements may be unstable)
- **std_par**: worst_20/best_20 = 4.1x (possible bimodal distribution)
- **std_par**: bridge=116.2% of algo (FFI overhead may distort results)
