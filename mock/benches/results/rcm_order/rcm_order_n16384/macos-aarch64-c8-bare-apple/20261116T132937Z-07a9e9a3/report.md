# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (7.96 ms) is smaller than the fastest variant's own run-to-run std-dev (9.72 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (42.92 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader rcm_adj vs stability leader rcm_half (+8% speed for 1.1x steadier)

rcm_adj is fastest (42.92 ms, CV 22.6%); rcm_half gives up 8.5% median for 1.1x lower variance (CV 19.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### rcm_rev is inconsistent: worst-20% is 2.3x its best-20%

rcm_rev's best 20% of batches run at 34.40 ms but its worst 20% at 79.57 ms (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 42922279.1 ns median
- Spread: 1.19x (fastest 42922279.1 ns, slowest 50885118.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 44845625ns | 42932767ns | 34816346ns | 40326655ns | 56638720ns | base |
| rcm_half | 47018267ns | 46576604ns | 35044912ns | 43636809ns | 58077131ns | +4.84% |
| rcm_rev | 53454978ns | 44919673ns | 34403504ns | 42387972ns | 79581223ns | +19.20% |
| rcm_scr | 50405011ns | 50894875ns | 34634092ns | 44980101ns | 66427836ns | +12.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 44836228ns | 34807757ns | 56629289ns | base | 0.000 |
| rcm_half | 47007258ns | 35034335ns | 58064571ns | +4.84% | 0.000 |
| rcm_rev | 53444243ns | 34395194ns | 79565766ns | +19.20% | 0.000 |
| rcm_scr | 50394444ns | 34623492ns | 66414644ns | +12.40% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 80.1% |
| rcm_half | 0.000 | 73.9% |
| rcm_rev | 0.000 | 76.6% |
| rcm_scr | 0.000 | 67.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 44845625ns | 44845625ns | base |
| rcm_half | 47018267ns | 47018267ns | +4.84% |
| rcm_rev | 53454978ns | 53454978ns | +19.20% |
| rcm_scr | 50405011ns | 50405011ns | +12.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 42922279ns | base | --- | [36334004, 50424427] | --- | --- | --- | --- |
| rcm_half | 46568500ns | no significant difference | [-118550, +3516025]ns | [39271167, 53612912] | no | 0.2888 | 0.0963 | 0 |
| rcm_rev | 44912688ns | no significant difference | [-1856573, +5954854]ns | [35276194, 56801698] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 50885119ns | no significant difference | [-2747294, +4137435]ns | [38154883, 56061877] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 50429300ns | +0.2% | +5.7% | +1.0% |
| 1 | 2 | 36684171ns | +18.9% | +126.7% | +108.2% |
| 1 | 3 | 37426392ns | -5.8% | -6.3% | -4.3% |
| 2 | 1 | 57582333ns | -0.2% | +38.3% | +2.7% |
| 2 | 2 | 43498971ns | +5.8% | +0.9% | +1.3% |
| 2 | 3 | 35313454ns | +4.7% | -2.1% | -1.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.230 | moderate- |
| rcm_half | -0.319 | moderate- |
| rcm_rev | -0.490 | moderate- |
| rcm_scr | -0.359 | moderate- |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 728375471.1ns | 44836228.2ns | 1624.5% | HIGH |
| rcm_half | 729689912.7ns | 47007258.1ns | 1552.3% | HIGH |
| rcm_rev | 729614073.8ns | 53444243.3ns | 1365.2% | HIGH |
| rcm_scr | 728871013.4ns | 50394443.5ns | 1446.3% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 34807756.9-56629288.9 ns)
  34807756.9 |########################################
  35898833.5 |##########################
  36989910.1 |#############
  38080986.7 |#############
  39172063.3 |
  40263139.9 |
  41354216.5 |
  42445293.1 |########################################
  43536369.7 |
  44627446.3 |
  45718522.9 |
  46809599.5 |
  47900676.1 |
  48991752.7 |#############
  50082829.3 |##########################
  51173905.9 |
  52264982.5 |
  53356059.1 |
  54447135.7 |
  55538212.3 |
  (1 below, 4 above range)

rcm_half (n=18, range 35034334.7-58064570.8 ns)
  35034334.7 |####################
  36185846.5 |########################################
  37337358.3 |
  38488870.1 |####################
  39640382.0 |
  40791893.8 |
  41943405.6 |
  43094917.4 |####################
  44246429.2 |####################
  45397941.0 |####################
  46549452.8 |####################
  47700964.6 |
  48852476.4 |########################################
  50003988.2 |####################
  51155500.0 |
  52307011.8 |
  53458523.6 |
  54610035.4 |
  55761547.2 |####################
  56913059.0 |####################
  (2 below, 3 above range)

rcm_rev (n=18, range 34395194.5-79565766.0 ns)
  34395194.5 |########################################
  36653723.0 |
  38912251.6 |
  41170780.2 |################
  43429308.8 |########
  45687837.3 |################
  47946365.9 |
  50204894.5 |
  52463423.1 |########
  54721951.6 |########
  56980480.2 |########
  59239008.8 |
  61497537.4 |
  63756065.9 |
  66014594.5 |
  68273123.1 |
  70531651.7 |
  72790180.2 |
  75048708.8 |
  77307237.4 |
  (1 below, 4 above range)

rcm_scr (n=18, range 34623491.7-66414643.7 ns)
  34623491.7 |########################################
  36213049.3 |
  37802606.9 |
  39392164.5 |##########
  40981722.1 |
  42571279.7 |####################
  44160837.3 |
  45750394.9 |
  47339952.5 |
  48929510.1 |
  50519067.7 |##############################
  52108625.3 |####################
  53698182.9 |
  55287740.5 |
  56877298.1 |
  58466855.7 |##########
  60056413.3 |##########
  61645970.9 |
  63235528.5 |
  64825086.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: CV=21.7% (high variance, measurements may be unstable)
- **rcm_adj**: bridge=1691.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=1566.2% of algo (FFI overhead may distort results)
- **rcm_rev**: CV=41.8% (high variance, measurements may be unstable)
- **rcm_rev**: bridge=1621.8% of algo (FFI overhead may distort results)
- **rcm_scr**: CV=27.5% (high variance, measurements may be unstable)
- **rcm_scr**: bridge=1433.9% of algo (FFI overhead may distort results)
