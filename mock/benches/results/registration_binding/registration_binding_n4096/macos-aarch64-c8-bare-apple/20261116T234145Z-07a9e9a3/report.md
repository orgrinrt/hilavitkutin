# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 25.3x slower than the field

reg_bound_record (5.47 us) is 25.3x the fastest (216 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### reg_bound_units is fastest but the noisiest (CV 8.0%)

reg_bound_units wins on median (216 ns) yet has the highest variance (CV 8.0%), while reg_bound_record is the steadiest (CV 1.1%, 5.47 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (reg_bound_units, reg_bound_columns) are a dead heat (<1%)

reg_bound_units (216 ns) and reg_bound_columns (217 ns) differ by 0.60%, inside the noise, even though the wider field spreads 2433.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### reg_bound_units shows alternating (throttle bounce) (autocorr -0.61)

reg_bound_units's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 25.3x the fastest

Fastest reg_bound_units (216 ns) to slowest reg_bound_record (5.47 us): 25.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: reg_bound_units** at 215.8 ns median (-6.3% vs baseline)
- 2 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 25.33x (fastest 215.8 ns, slowest 5466.7 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 2317ns | 2300ns | 2291ns | 2302ns | 2392ns | -0.69% |
| reg_bound_record | 7583ns | 7555ns | 7543ns | 7559ns | 7696ns | +224.97% |
| reg_bound_strided | 3376ns | 3338ns | 3326ns | 3348ns | 3507ns | +44.66% |
| reg_bound_units | 2341ns | 2306ns | 2292ns | 2323ns | 2444ns | +0.32% |
| reg_typed | 2334ns | 2315ns | 2304ns | 2317ns | 2414ns | base |
| reg_typed_twin | 2337ns | 2318ns | 2307ns | 2321ns | 2413ns | +0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 219ns | 215ns | 226ns | -6.07% | 18.744 |
| reg_bound_record | 5488ns | 5464ns | 5572ns | +2259.13% | 0.746 |
| reg_bound_strided | 1269ns | 1249ns | 1326ns | +445.58% | 3.227 |
| reg_bound_units | 221ns | 213ns | 240ns | -5.11% | 18.556 |
| reg_typed | 233ns | 227ns | 244ns | base | 17.607 |
| reg_typed_twin | 232ns | 227ns | 241ns | -0.11% | 17.626 |

## Performance model

- Peak throughput: **19.216 Gops/s** (reg_bound_units; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 18.867 | 98.2% |
| reg_bound_record | 0.749 | 3.9% |
| reg_bound_strided | 3.267 | 17.0% |
| reg_bound_units | 18.981 | 98.8% |
| reg_typed | 17.778 | 92.5% |
| reg_typed_twin | 17.747 | 92.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 2317ns | 2317ns | -0.69% |
| reg_bound_record | 7583ns | 7583ns | +224.97% |
| reg_bound_strided | 3376ns | 3376ns | +44.66% |
| reg_bound_units | 2341ns | 2341ns | +0.32% |
| reg_typed | 2334ns | 2334ns | base |
| reg_typed_twin | 2337ns | 2337ns | +0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 230ns | base | --- | [230, 231] | --- | --- | --- | --- |
| reg_bound_columns | 217ns | -12.9ns (-5.6%) | [-13, -12]ns | [217, 217] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_record | 5467ns | +5236.2ns (+2272.7%) | [+5236, +5237]ns | [5466, 5467] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 1254ns | +1024.3ns (+444.6%) | [+1023, +1025]ns | [1253, 1255] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 216ns | -14.5ns (-6.3%) | [-15, -13]ns | [215, 217] | YES | 0.0000 | 0.0000 | 0 |
| reg_typed_twin | 231ns | no significant difference | [+0, +2]ns | [230, 232] | no (adj: YES) | 0.0224 | 0.0224 | 9 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 231ns | -6.2% | +2263.6% | +460.1% | -7.4% | +2.7% |
| 1 | 2 | 230ns | -5.7% | +2274.9% | +443.6% | -2.8% | +0.1% |
| 1 | 3 | 229ns | -4.7% | +2282.4% | +447.7% | -6.4% | +0.3% |
| 1 | 4 | 234ns | -6.9% | +2240.7% | +435.8% | -5.6% | +1.9% |
| 1 | 5 | 229ns | -5.6% | +2314.7% | +466.6% | -6.4% | +1.9% |
| 1 | 6 | 231ns | -5.9% | +2270.4% | +443.1% | -6.8% | -0.3% |
| 1 | 7 | 230ns | -5.6% | +2281.1% | +446.0% | -4.5% | -0.3% |
| 1 | 8 | 230ns | -5.6% | +2281.0% | +445.3% | -5.4% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | 0.151 | ok |
| reg_bound_record | -0.102 | ok |
| reg_bound_strided | -0.263 | moderate- |
| reg_bound_units | -0.614 | HIGH- (thermal bounce) |
| reg_typed | -0.489 | moderate- |
| reg_typed_twin | 0.039 | ok |

**Consistency summary:**

- **reg_bound_columns**: won 8/8, lost 0/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 8/8, lost 0/8
- **reg_typed_twin**: won 2/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 8198.8ns | 218.5ns | 3752.0% | HIGH |
| reg_bound_record | 8176.7ns | 5488.1ns | 149.0% | HIGH |
| reg_bound_strided | 8252.7ns | 1269.2ns | 650.2% | HIGH |
| reg_bound_units | 8287.3ns | 220.7ns | 3754.4% | HIGH |
| reg_typed | 8165.1ns | 232.6ns | 3509.9% | HIGH |
| reg_typed_twin | 8185.7ns | 232.4ns | 3522.5% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 215.1-226.3 ns)
    215.1 |#######
    215.7 |########################################
    216.2 |#######################
    216.8 |#######################
    217.4 |###############################
    217.9 |############
    218.5 |##########
    219.0 |############
    219.6 |###
    220.2 |
    220.7 |
    221.3 |###
    221.8 |
    222.4 |###
    223.0 |
    223.5 |
    224.1 |
    224.7 |
    225.2 |#
    225.8 |
  (16 below, 2 above range)

reg_bound_record (n=160, range 5463.6-5571.7 ns)
   5463.6 |########################################
   5469.0 |####
   5474.4 |#
   5479.8 |#
   5485.2 |##
   5490.6 |#
   5496.0 |
   5501.4 |
   5506.8 |#
   5512.2 |
   5517.6 |
   5523.0 |
   5528.5 |
   5533.9 |
   5539.3 |
   5544.7 |
   5550.1 |
   5555.5 |
   5560.9 |
   5566.3 |
  (12 below, 10 above range)

reg_bound_strided (n=160, range 1249.4-1326.4 ns)
   1249.4 |########################################
   1253.3 |#####################################
   1257.1 |#######
   1261.0 |####
   1264.8 |#
   1268.7 |#
   1272.5 |
   1276.3 |
   1280.2 |
   1284.0 |
   1287.9 |
   1291.7 |#
   1295.6 |#####
   1299.4 |#####
   1303.3 |
   1307.1 |##
   1311.0 |
   1314.8 |
   1318.7 |#
   1322.5 |##
  (13 below, 10 above range)

reg_bound_units (n=160, range 213.2-240.4 ns)
    213.2 |############################
    214.5 |########################################
    215.9 |#################
    217.2 |#########
    218.6 |####
    220.0 |####
    221.3 |###
    222.7 |####
    224.0 |######
    225.4 |######
    226.8 |##
    228.1 |##
    229.5 |#
    230.8 |
    232.2 |
    233.6 |
    234.9 |
    236.3 |
    237.7 |
    239.0 |
  (11 below, 6 above range)

reg_typed (n=160, range 227.3-244.5 ns)
    227.3 |##################
    228.1 |########################################
    229.0 |########################################
    229.9 |##########################
    230.7 |##############################
    231.6 |####################
    232.4 |##########################
    233.3 |##########
    234.2 |###########
    235.0 |###
    235.9 |###
    236.7 |
    237.6 |
    238.5 |
    239.3 |
    240.2 |
    241.0 |#
    241.9 |
    242.8 |
    243.6 |
  (11 below, 10 above range)

reg_typed_twin (n=160, range 227.4-240.7 ns)
    227.4 |############
    228.1 |#######################
    228.7 |#############################
    229.4 |#################################
    230.0 |#########################
    230.7 |#############################
    231.4 |##
    232.0 |########################################
    232.7 |#####################
    233.4 |##############
    234.0 |############
    234.7 |##
    235.4 |########
    236.0 |##
    236.7 |##
    237.4 |####
    238.0 |##
    238.7 |######
    239.4 |
    240.0 |####
  (14 below, 15 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=3745.7% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=148.9% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=649.6% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=3790.9% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=3517.0% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=3511.4% of algo (FFI overhead may distort results)
