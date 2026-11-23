# Resource storage layouts under clean morsel loop (algo only)

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (v0_blob) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline v0_blob has the worst median (118 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest v5_handletable at 100 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### v5_handletable dominates: 16% faster than the next best (v1_snapshot)

v5_handletable (100 ns) leads v1_snapshot (116 ns) by 16%, a clear separation rather than a photo finish. CV 65.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### v5_handletable is fastest but the noisiest (CV 65.0%)

v5_handletable wins on median (100 ns) yet has the highest variance (CV 65.0%), while v3_shapebound is the steadiest (CV 52.6%, 117 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### v2_decomposed shows alternating (throttle bounce) (autocorr -0.53)

v2_decomposed's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (18 ns) is smaller than the fastest variant's own run-to-run std-dev (65 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### v2_decomposed is inconsistent: worst-20% is 2.6x its best-20%

v2_decomposed's best 20% of batches run at 96 ns but its worst 20% at 252 ns (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### v1_snapshot's edge over baseline is significant but tiny (-0 ns, 0.43%)

v1_snapshot differs from baseline v0_blob by -0 ns (0.43%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v5_handletable** at 100.0 ns median (-15.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.18x (fastest 100.0 ns, slowest 117.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under clean morsel loop (algo only) |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112759259 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 191ns | 166ns | 136ns | 154ns | 355ns | base |
| v1_snapshot | 195ns | 165ns | 135ns | 155ns | 375ns | +2.27% |
| v2_decomposed | 198ns | 166ns | 135ns | 158ns | 382ns | +3.90% |
| v3_shapebound | 194ns | 165ns | 135ns | 163ns | 345ns | +1.64% |
| v4_erased | 193ns | 166ns | 136ns | 154ns | 368ns | +1.42% |
| v5_handletable | 191ns | 140ns | 136ns | 150ns | 369ns | +0.29% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 131ns | 96ns | 233ns | base | 7.794 |
| v1_snapshot | 134ns | 95ns | 247ns | +2.19% | 7.627 |
| v2_decomposed | 136ns | 96ns | 252ns | +3.82% | 7.507 |
| v3_shapebound | 133ns | 95ns | 226ns | +1.41% | 7.685 |
| v4_erased | 134ns | 96ns | 244ns | +1.76% | 7.659 |
| v5_handletable | 132ns | 96ns | 244ns | +0.20% | 7.778 |

## Performance model

- Peak throughput: **10.737 Gops/s** (v1_snapshot; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 8.707 | 81.1% |
| v1_snapshot | 8.805 | 82.0% |
| v2_decomposed | 8.752 | 81.5% |
| v3_shapebound | 8.767 | 81.7% |
| v4_erased | 8.745 | 81.4% |
| v5_handletable | 10.245 | 95.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 191ns | 191ns | base |
| v1_snapshot | 195ns | 195ns | +2.27% |
| v2_decomposed | 198ns | 198ns | +3.90% |
| v3_shapebound | 194ns | 194ns | +1.64% |
| v4_erased | 193ns | 193ns | +1.42% |
| v5_handletable | 191ns | 191ns | +0.29% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 118ns | base | --- | [97, 118] | --- | --- | --- | --- |
| v1_snapshot | 116ns | -0.5ns (-0.4%) | [-1, -0]ns | [97, 117] | YES (adj: no) | 0.0524 | 0.0210 | 2 |
| v2_decomposed | 117ns | -0.3ns (-0.3%) | [-0, -0]ns | [116, 117] | YES | 0.0053 | 0.0011 | 4 |
| v3_shapebound | 117ns | no significant difference | [-1, +0]ns | [116, 117] | no | 0.5187 | 0.3112 | 2 |
| v4_erased | 117ns | no significant difference | [-0, +0]ns | [97, 117] | no | 0.8546 | 0.8546 | 1 |
| v5_handletable | 100ns | no significant difference | [-3, +1]ns | [97, 117] | no | 0.6537 | 0.5230 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 96ns | -0.7% | -0.4% | -0.1% | +23.1% | +22.6% |
| 1 | 2 | 96ns | +0.1% | +21.9% | +21.9% | -0.5% | +0.9% |
| 1 | 3 | 96ns | +21.4% | -0.2% | +22.0% | +0.7% | +22.2% |
| 1 | 4 | 118ns | -0.3% | -0.8% | -18.8% | -17.6% | -18.0% |
| 1 | 5 | 118ns | -18.2% | -18.2% | -0.9% | -0.6% | -18.6% |
| 2 | 1 | 96ns | -0.4% | -0.3% | +21.6% | +22.8% | +0.5% |
| 2 | 2 | 96ns | +21.7% | +21.8% | +21.5% | +0.5% | +0.6% |
| 2 | 3 | 96ns | -0.2% | -0.2% | -0.7% | +0.1% | +0.3% |
| 2 | 4 | 96ns | +20.8% | +21.7% | +21.3% | +0.6% | +22.0% |
| 2 | 5 | 118ns | -0.3% | -18.3% | -0.8% | -0.9% | -17.5% |
| 3 | 1 | 118ns | +0.2% | -0.3% | -16.2% | -0.1% | -17.8% |
| 3 | 2 | 118ns | -18.0% | -0.2% | -0.4% | -17.8% | -18.5% |
| 3 | 3 | 96ns | -0.4% | -0.2% | -0.8% | +21.5% | +22.6% |
| 3 | 4 | 118ns | -18.1% | -0.2% | +0.5% | +0.3% | +0.1% |
| 3 | 5 | 118ns | -0.5% | -0.3% | -0.8% | -17.9% | -17.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.265 | moderate+ |
| v1_snapshot | 0.002 | ok |
| v2_decomposed | -0.534 | HIGH- (thermal bounce) |
| v3_shapebound | -0.435 | moderate- |
| v4_erased | -0.011 | ok |
| v5_handletable | -0.119 | ok |

**Consistency summary:**

- **v1_snapshot**: won 10/15, lost 5/15
- **v2_decomposed**: won 12/15, lost 3/15
- **v3_shapebound**: won 9/15, lost 6/15
- **v4_erased**: won 6/15, lost 7/15
- **v5_handletable**: won 6/15, lost 9/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 2746.0ns | 131.4ns | 2090.0% | HIGH |
| v1_snapshot | 2821.4ns | 134.3ns | 2101.4% | HIGH |
| v2_decomposed | 2860.9ns | 136.4ns | 2097.3% | HIGH |
| v3_shapebound | 2812.2ns | 133.2ns | 2110.6% | HIGH |
| v4_erased | 2797.9ns | 133.7ns | 2092.6% | HIGH |
| v5_handletable | 2778.7ns | 131.7ns | 2110.5% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 96.3-233.2 ns)
     96.3 |#######################################
    103.1 |
    110.0 |
    116.8 |########################################
    123.7 |
    130.5 |
    137.3 |#
    144.2 |
    151.0 |
    157.9 |
    164.7 |
    171.6 |
    178.4 |
    185.3 |
    192.1 |
    199.0 |#
    205.8 |
    212.7 |
    219.5 |
    226.4 |
  (7 below, 13 above range)

v1_snapshot (n=120, range 95.4-247.2 ns)
     95.4 |#################################
    103.0 |
    110.6 |########################################
    118.1 |#
    125.7 |
    133.3 |
    140.9 |
    148.5 |
    156.1 |
    163.7 |
    171.3 |
    178.9 |
    186.5 |
    194.1 |
    201.7 |##
    209.2 |
    216.8 |
    224.4 |
    232.0 |
    239.6 |
  (11 below, 13 above range)

v2_decomposed (n=120, range 95.8-251.8 ns)
     95.8 |##########################
    103.6 |
    111.4 |########################################
    119.2 |
    127.0 |
    134.8 |
    142.6 |
    150.4 |
    158.2 |
    166.0 |
    173.8 |
    181.6 |
    189.4 |
    197.2 |
    205.0 |
    212.8 |
    220.6 |
    228.4 |
    236.2 |
    244.0 |
  (8 below, 14 above range)

v3_shapebound (n=120, range 95.4-226.0 ns)
     95.4 |#########
    101.9 |
    108.4 |
    115.0 |########################################
    121.5 |
    128.0 |
    134.6 |
    141.1 |
    147.6 |
    154.2 |
    160.7 |
    167.2 |
    173.8 |
    180.3 |
    186.8 |
    193.4 |
    199.9 |
    206.4 |
    213.0 |
    219.5 |
  (15 below, 11 above range)

v4_erased (n=120, range 96.0-244.3 ns)
     96.0 |########################################
    103.4 |
    110.8 |##################################
    118.3 |##############
    125.7 |
    133.1 |#
    140.5 |#
    147.9 |#
    155.3 |##
    162.8 |#
    170.2 |
    177.6 |#
    185.0 |
    192.4 |
    199.8 |
    207.3 |
    214.7 |
    222.1 |
    229.5 |##
    236.9 |
  (16 below, 13 above range)

v5_handletable (n=120, range 95.9-243.6 ns)
     95.9 |########################################
    103.3 |
    110.7 |##################
    118.1 |########
    125.5 |
    132.8 |
    140.2 |
    147.6 |
    155.0 |
    162.4 |#
    169.7 |
    177.1 |
    184.5 |
    191.9 |
    199.3 |#
    206.7 |
    214.0 |
    221.4 |
    228.8 |#
    236.2 |
  (12 below, 12 above range)

```

## Diagnostics

- **v0_blob**: CV=49.1% (high variance, measurements may be unstable)
- **v0_blob**: bridge=2092.2% of algo (FFI overhead may distort results)
- **v1_snapshot**: CV=49.0% (high variance, measurements may be unstable)
- **v1_snapshot**: bridge=2115.9% of algo (FFI overhead may distort results)
- **v2_decomposed**: CV=51.7% (high variance, measurements may be unstable)
- **v2_decomposed**: bridge=2105.1% of algo (FFI overhead may distort results)
- **v3_shapebound**: CV=46.1% (high variance, measurements may be unstable)
- **v3_shapebound**: bridge=2111.8% of algo (FFI overhead may distort results)
- **v4_erased**: CV=49.6% (high variance, measurements may be unstable)
- **v4_erased**: bridge=2102.4% of algo (FFI overhead may distort results)
- **v5_handletable**: CV=49.4% (high variance, measurements may be unstable)
- **v5_handletable**: bridge=2045.4% of algo (FFI overhead may distort results)
