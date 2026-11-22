# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### wp_nh8k is an outlier: 2.0x slower than the field

wp_nh8k (17.57 us) is 2.0x the fastest (8.61 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (wp_park, wp_lp0) are a dead heat (<1%)

wp_park (8.61 us) and wp_lp0 (8.64 us) differ by 0.39%, inside the noise, even though the wider field spreads 104.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### wp_nh8k is inconsistent: worst-20% is 2.2x its best-20%

wp_nh8k's best 20% of batches run at 9.54 us but its worst 20% at 20.58 us (2.2x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_park** at 8606.2 ns median (-4.5% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 2.04x (fastest 8606.2 ns, slowest 17568.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | frame wake policy: park-immediately vs bounded spin pre-roll |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000003 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 11857ns | 11548ns | 10950ns | 11390ns | 13011ns | base |
| wp_lp0 | 11256ns | 11162ns | 10611ns | 11081ns | 11839ns | -5.07% |
| wp_nh128 | 11754ns | 11610ns | 11010ns | 11485ns | 12531ns | -0.87% |
| wp_nh2k | 15016ns | 14792ns | 14299ns | 14711ns | 15831ns | +26.64% |
| wp_nh8k | 19072ns | 19910ns | 11926ns | 18627ns | 23313ns | +60.85% |
| wp_park | 11389ns | 10988ns | 10571ns | 10927ns | 12492ns | -3.95% |
| wp_spin | 15944ns | 15888ns | 14862ns | 15676ns | 16887ns | +34.47% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 9380ns | 8600ns | 10491ns | base | 0.109 |
| wp_lp0 | 8779ns | 8274ns | 9285ns | -6.40% | 0.117 |
| wp_nh128 | 9279ns | 8633ns | 10019ns | -1.07% | 0.110 |
| wp_nh2k | 12465ns | 11778ns | 13195ns | +32.89% | 0.082 |
| wp_nh8k | 16563ns | 9543ns | 20581ns | +76.58% | 0.062 |
| wp_park | 8995ns | 8253ns | 10055ns | -4.10% | 0.114 |
| wp_spin | 13431ns | 12429ns | 14389ns | +43.20% | 0.076 |

## Performance model

- Peak throughput: **0.124 Gops/s** (wp_park; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.114 | 91.6% |
| wp_lp0 | 0.119 | 95.5% |
| wp_nh128 | 0.113 | 90.7% |
| wp_nh2k | 0.083 | 67.1% |
| wp_nh8k | 0.058 | 47.0% |
| wp_park | 0.119 | 95.9% |
| wp_spin | 0.077 | 62.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 11857ns | 11857ns | base |
| wp_lp0 | 11256ns | 11256ns | -5.07% |
| wp_nh128 | 11754ns | 11754ns | -0.87% |
| wp_nh2k | 15016ns | 15016ns | +26.64% |
| wp_nh8k | 19072ns | 19072ns | +60.85% |
| wp_park | 11389ns | 11389ns | -3.95% |
| wp_spin | 15944ns | 15944ns | +34.47% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 9008ns | base | --- | [8729, 10115] | --- | --- | --- | --- |
| wp_lp0 | 8640ns | -360.5ns (-4.0%) | [-1438, -69]ns | [8546, 8902] | YES | 0.0113 | 0.0075 | 0 |
| wp_nh128 | 9102ns | no significant difference | [-946, +610]ns | [8808, 9600] | no | 1.0000 | 1.0000 | 0 |
| wp_nh2k | 12302ns | +3052.1ns (+33.9%) | [+2631, +3575]ns | [12008, 12740] | YES | 0.0000 | 0.0000 | 0 |
| wp_nh8k | 17569ns | +8702.0ns (+96.6%) | [+5023, +9917]ns | [14223, 19419] | YES | 0.0003 | 0.0001 | 0 |
| wp_park | 8606ns | -500.1ns (-5.6%) | [-1081, -154]ns | [8435, 8819] | YES | 0.0371 | 0.0309 | 0 |
| wp_spin | 13244ns | +3835.5ns (+42.6%) | [+3246, +4960]ns | [12812, 13804] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 8812ns | -1.0% | +1.1% | +53.0% | +118.6% | -3.7% | +49.8% |
| 1 | 2 | 10633ns | -18.4% | -14.7% | +18.2% | +103.4% | -20.9% | +21.3% |
| 1 | 3 | 8700ns | -1.3% | +4.0% | +42.5% | +9.7% | -1.5% | +64.1% |
| 2 | 1 | 9171ns | -6.8% | +9.5% | +39.3% | +91.4% | -6.0% | +35.4% |
| 2 | 2 | 9054ns | -5.0% | -4.1% | +31.0% | +94.2% | -5.1% | +41.4% |
| 2 | 3 | 8600ns | +3.6% | +7.5% | +39.9% | +74.3% | +31.5% | +58.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.402 | moderate- |
| wp_lp0 | 0.045 | ok |
| wp_nh128 | -0.487 | moderate- |
| wp_nh2k | 0.098 | ok |
| wp_nh8k | -0.341 | moderate- |
| wp_park | -0.010 | ok |
| wp_spin | -0.467 | moderate- |

**Consistency summary:**

- **wp_lp0**: won 5/6, lost 1/6
- **wp_nh128**: won 2/6, lost 4/6
- **wp_nh2k**: won 0/6, lost 6/6
- **wp_nh8k**: won 0/6, lost 6/6
- **wp_park**: won 5/6, lost 1/6
- **wp_spin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 1344.2ns | 9379.6ns | 14.3% | HIGH |
| wp_lp0 | 1345.8ns | 8778.9ns | 15.3% | HIGH |
| wp_nh128 | 1347.2ns | 9278.9ns | 14.5% | HIGH |
| wp_nh2k | 1354.4ns | 12464.8ns | 10.9% | HIGH |
| wp_nh8k | 1338.0ns | 16562.7ns | 8.1% | HIGH |
| wp_park | 1342.6ns | 8994.7ns | 14.9% | HIGH |
| wp_spin | 1350.5ns | 13431.5ns | 10.1% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 8600.0-10491.0 ns)
   8600.0 |########################################
   8694.5 |##########################
   8789.1 |#############
   8883.6 |#############
   8978.2 |########################################
   9072.7 |
   9167.3 |#############
   9261.8 |
   9356.4 |
   9450.9 |
   9545.5 |
   9640.0 |
   9734.6 |
   9829.1 |#############
   9923.7 |
  10018.2 |
  10112.8 |
  10207.3 |
  10301.9 |##########################
  10396.4 |
  (1 below, 3 above range)

wp_lp0 (n=18, range 8273.6-9285.4 ns)
   8273.6 |#############
   8324.2 |#############
   8374.8 |
   8425.4 |
   8476.0 |#############
   8526.6 |##########################
   8577.2 |########################################
   8627.7 |#############
   8678.3 |##########################
   8728.9 |
   8779.5 |
   8830.1 |
   8880.7 |##########################
   8931.3 |
   8981.9 |#############
   9032.5 |#############
   9083.1 |
   9133.7 |
   9184.2 |
   9234.8 |#############
  (1 below, 1 above range)

wp_nh128 (n=18, range 8633.3-10018.8 ns)
   8633.3 |########################################
   8702.6 |####################
   8771.9 |
   8841.1 |
   8910.4 |########################################
   8979.7 |####################
   9049.0 |####################
   9118.2 |####################
   9187.5 |####################
   9256.8 |####################
   9326.0 |
   9395.3 |
   9464.6 |
   9533.9 |####################
   9603.1 |
   9672.4 |####################
   9741.7 |
   9810.9 |
   9880.2 |####################
   9949.5 |
  (2 below, 3 above range)

wp_nh2k (n=18, range 11777.8-13195.1 ns)
  11777.8 |####################
  11848.6 |####################
  11919.5 |########################################
  11990.4 |########################################
  12061.2 |####################
  12132.1 |
  12203.0 |####################
  12273.9 |
  12344.7 |####################
  12415.6 |
  12486.5 |
  12557.3 |########################################
  12628.2 |
  12699.1 |####################
  12769.9 |####################
  12840.8 |####################
  12911.7 |
  12982.5 |
  13053.4 |
  13124.3 |
  (1 below, 3 above range)

wp_nh8k (n=18, range 9543.1-20581.3 ns)
   9543.1 |####################
  10095.0 |####################
  10646.9 |
  11198.8 |
  11750.7 |
  12302.6 |
  12854.5 |####################
  13406.4 |####################
  13958.3 |
  14510.3 |########################################
  15062.2 |
  15614.1 |
  16166.0 |####################
  16717.9 |
  17269.8 |########################################
  17821.7 |
  18373.6 |####################
  18925.5 |########################################
  19477.4 |########################################
  20029.4 |
  (1 below, 3 above range)

wp_park (n=18, range 8252.8-10054.9 ns)
   8252.8 |########################################
   8342.9 |#############
   8433.0 |##########################
   8523.1 |##########################
   8613.2 |##########################
   8703.3 |#############
   8793.4 |##########################
   8883.5 |
   8973.6 |#############
   9063.7 |
   9153.8 |
   9243.9 |
   9334.0 |
   9424.1 |
   9514.2 |
   9604.3 |
   9694.4 |
   9784.5 |
   9874.6 |
   9964.7 |
  (1 below, 3 above range)

wp_spin (n=18, range 12429.2-14388.9 ns)
  12429.2 |
  12527.2 |####################
  12625.2 |
  12723.2 |########################################
  12821.1 |########################################
  12919.1 |
  13017.1 |
  13115.1 |####################
  13213.1 |########################################
  13311.1 |
  13409.1 |
  13507.0 |########################################
  13605.0 |####################
  13703.0 |
  13801.0 |
  13899.0 |####################
  13997.0 |
  14094.9 |####################
  14192.9 |####################
  14290.9 |
  (2 below, 2 above range)

```

## Diagnostics

- **wp_isb8**: bridge=14.9% of algo (FFI overhead may distort results)
- **wp_lp0**: bridge=15.6% of algo (FFI overhead may distort results)
- **wp_nh128**: bridge=14.8% of algo (FFI overhead may distort results)
- **wp_nh2k**: bridge=11.0% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=24.3% (high variance, measurements may be unstable)
- **wp_park**: bridge=15.6% of algo (FFI overhead may distort results)
- **wp_spin**: bridge=10.2% of algo (FFI overhead may distort results)
