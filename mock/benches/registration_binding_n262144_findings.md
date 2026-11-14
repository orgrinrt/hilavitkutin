# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.7x slower than the field

reg_bound_record (451.52 us) is 7.7x the fastest (58.75 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### reg_bound_units is fastest but the noisiest (CV 24.1%)

reg_bound_units wins on median (58.75 us) yet has the highest variance (CV 24.1%), while reg_bound_strided is the steadiest (CV 16.3%, 104.83 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (reg_bound_units, reg_bound_columns) are a dead heat (<1%)

reg_bound_units (58.75 us) and reg_bound_columns (59.09 us) differ by 0.58%, inside the noise, even though the wider field spreads 668.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {reg_bound_units, reg_bound_columns, reg_typed, reg_typed_twin, reg_bound_strided} vs {reg_bound_record} (331% apart)

The field splits into a fast tier {reg_bound_units, reg_bound_columns, reg_typed, reg_typed_twin, reg_bound_strided} and a slow tier {reg_bound_record} with a 331% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.7x the fastest

Fastest reg_bound_units (58.75 us) to slowest reg_bound_record (451.52 us): 7.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### reg_bound_units is inconsistent: worst-20% is 1.8x its best-20%

reg_bound_units's best 20% of batches run at 50.18 us but its worst 20% at 88.50 us (1.8x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: reg_bound_units** at 58753.1 ns median (-1.4% vs baseline)
- 3 variants significantly slower than baseline
- Spread: 7.69x (fastest 58753.1 ns, slowest 451523.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Registration: typed binding vs a run-time binding table over a column workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71c40e2b95a3d86f |
| Counter frequency | 2100480538 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 66597ns | 62342ns | 56098ns | 63575ns | 86161ns | +0.33% |
| reg_bound_record | 474467ns | 455764ns | 361963ns | 460801ns | 627969ns | +614.82% |
| reg_bound_strided | 114653ns | 108343ns | 95852ns | 111082ns | 144164ns | +72.73% |
| reg_bound_units | 67854ns | 61939ns | 53094ns | 64536ns | 92567ns | +2.23% |
| reg_typed | 66376ns | 62739ns | 54785ns | 64705ns | 82980ns | base |
| reg_typed_twin | 68196ns | 64388ns | 58287ns | 65593ns | 85916ns | +2.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 63125ns | 53256ns | 81456ns | +0.50% | 4.153 |
| reg_bound_record | 470325ns | 358634ns | 623104ns | +648.76% | 0.557 |
| reg_bound_strided | 111144ns | 92935ns | 139765ns | +76.94% | 2.359 |
| reg_bound_units | 64435ns | 50182ns | 88503ns | +2.58% | 4.068 |
| reg_typed | 62814ns | 51917ns | 78792ns | base | 4.173 |
| reg_typed_twin | 64568ns | 55234ns | 81562ns | +2.79% | 4.060 |

## Performance model

- Peak throughput: **5.224 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 262144

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 4.436 | 84.9% |
| reg_bound_record | 0.581 | 11.1% |
| reg_bound_strided | 2.501 | 47.9% |
| reg_bound_units | 4.462 | 85.4% |
| reg_typed | 4.398 | 84.2% |
| reg_typed_twin | 4.309 | 82.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 66597ns | 66597ns | +0.33% |
| reg_bound_record | 474467ns | 474467ns | +614.82% |
| reg_bound_strided | 114653ns | 114653ns | +72.73% |
| reg_bound_units | 67854ns | 67854ns | +2.23% |
| reg_typed | 66376ns | 66376ns | base |
| reg_typed_twin | 68196ns | 68196ns | +2.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 59608ns | base | --- | [58009, 61555] | --- | --- | --- | --- |
| reg_bound_columns | 59091ns | no significant difference | [-618, +2159]ns | [58670, 59834] | no | 0.2945 | 0.2356 | 0 |
| reg_bound_record | 451524ns | +389768.6ns (+653.9%) | [+376765, +401791]ns | [443328, 462730] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 104826ns | +44831.9ns (+75.2%) | [+41508, +48547]ns | [103419, 107343] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 58753ns | no significant difference | [-2477, +2814]ns | [57693, 61113] | no | 0.8126 | 0.8126 | 0 |
| reg_typed_twin | 60838ns | +1882.7ns (+3.2%) | [+132, +4316]ns | [60271, 62790] | YES (adj: no) | 0.0796 | 0.0478 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 61737ns | -11.1% | +562.9% | +67.9% | -6.3% | -4.8% |
| 1 | 2 | 62628ns | -4.9% | +609.3% | +65.4% | -9.5% | -2.4% |
| 1 | 3 | 56545ns | +12.8% | +674.3% | +85.2% | +10.1% | +24.3% |
| 1 | 4 | 54693ns | +7.2% | +750.6% | +83.3% | +16.8% | +9.9% |
| 1 | 5 | 57693ns | +1.8% | +626.4% | +78.8% | -8.1% | +6.0% |
| 1 | 6 | 57759ns | +3.9% | +708.4% | +117.3% | -17.1% | +4.7% |
| 1 | 7 | 60288ns | -2.6% | +802.7% | +77.0% | +20.3% | -4.5% |
| 1 | 8 | 64675ns | +9.0% | +682.0% | +100.6% | +27.3% | +3.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.105 | ok |
| reg_bound_record | 0.347 | moderate+ |
| reg_bound_strided | -0.039 | ok |
| reg_bound_units | 0.231 | moderate+ |
| reg_typed | 0.330 | moderate+ |
| reg_typed_twin | -0.249 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 3/8, lost 5/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 4/8, lost 4/8
- **reg_typed_twin**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 728657.0ns | 63125.2ns | 1154.3% | HIGH |
| reg_bound_record | 725937.0ns | 470325.2ns | 154.3% | HIGH |
| reg_bound_strided | 720712.7ns | 111143.9ns | 648.5% | HIGH |
| reg_bound_units | 721354.8ns | 64434.7ns | 1119.5% | HIGH |
| reg_typed | 710278.8ns | 62814.2ns | 1130.8% | HIGH |
| reg_typed_twin | 725699.8ns | 64568.0ns | 1123.9% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 53255.5-81455.9 ns)
  53255.5 |########
  54665.5 |########
  56075.6 |##########
  57485.6 |########################################
  58895.6 |#######################
  60305.6 |#########
  61715.6 |#######
  63125.7 |##
  64535.7 |###
  65945.7 |##
  67355.7 |####
  68765.7 |###
  70175.8 |#######
  71585.8 |##
  72995.8 |###
  74405.8 |##
  75815.9 |###
  77225.9 |#
  78635.9 |#
  80045.9 |##
  (12 below, 11 above range)

reg_bound_record (n=160, range 358634.0-623103.6 ns)
  358634.0 |######
  371857.5 |######
  385081.0 |####
  398304.4 |##################
  411527.9 |##########################
  424751.4 |######################
  437974.9 |########################################
  451198.4 |##############################
  464421.8 |########################
  477645.3 |################
  490868.8 |######
  504092.3 |######
  517315.7 |######
  530539.2 |##########
  543762.7 |####
  556986.2 |########
  570209.7 |##########
  583433.1 |##
  596656.6 |##
  609880.1 |######
  (19 below, 15 above range)

reg_bound_strided (n=160, range 92935.5-139764.5 ns)
  92935.5 |################
  95277.0 |########################
  97618.4 |############################
  99959.9 |##########################
  102301.3 |########################################
  104642.8 |##############################
  106984.2 |##########
  109325.7 |############
  111667.1 |##########
  114008.6 |######
  116350.0 |##########
  118691.5 |##########
  121032.9 |
  123374.4 |####
  125715.8 |######
  128057.3 |############
  130398.7 |########
  132740.2 |##
  135081.6 |############
  137423.1 |##
  (10 below, 16 above range)

reg_bound_units (n=160, range 50182.0-88503.3 ns)
  50182.0 |###
  52098.1 |###########
  54014.1 |################
  55930.2 |########################################
  57846.2 |################
  59762.3 |##########
  61678.4 |#########
  63594.4 |#######
  65510.5 |#######
  67426.6 |######
  69342.6 |##
  71258.7 |###########
  73174.8 |##
  75090.8 |#####
  77006.9 |
  78923.0 |##
  80839.0 |#####
  82755.1 |#####
  84671.2 |
  86587.2 |
  (16 below, 16 above range)

reg_typed (n=160, range 51917.3-78791.9 ns)
  51917.3 |##############
  53261.0 |##################
  54604.7 |####################
  55948.5 |######################
  57292.2 |########################################
  58635.9 |##########################
  59979.6 |########
  61323.4 |################
  62667.1 |####
  64010.8 |############
  65354.6 |##
  66698.3 |############
  68042.0 |####################
  69385.8 |######
  70729.5 |########
  72073.2 |##########
  73416.9 |########
  74760.7 |############
  76104.4 |######
  77448.1 |##
  (16 below, 11 above range)

reg_typed_twin (n=160, range 55233.7-81562.0 ns)
  55233.7 |#################
  56550.1 |####################
  57866.6 |##############################
  59183.0 |########################################
  60499.4 |##############################
  61815.8 |###################
  63132.2 |####################
  64448.6 |###########
  65765.1 |###########
  67081.5 |###
  68397.9 |#######
  69714.3 |#####
  71030.7 |#
  72347.1 |#####
  73663.5 |###########
  74980.0 |###
  76296.4 |#########
  77612.8 |#
  78929.2 |#######
  80245.6 |#
  (14 below, 8 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1215.6% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=165.8% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=684.4% of algo (FFI overhead may distort results)
- **reg_bound_units**: CV=22.0% (high variance, measurements may be unstable)
- **reg_bound_units**: bridge=1210.1% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1200.6% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1189.9% of algo (FFI overhead may distort results)
