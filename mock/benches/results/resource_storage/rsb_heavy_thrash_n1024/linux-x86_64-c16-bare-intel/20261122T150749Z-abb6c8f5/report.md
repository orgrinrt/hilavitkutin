# Resource storage layouts under L1-evicting heavy-memory mix-in

6 variants, 120 samples per variant.
Baseline: **v0_blob**

## Highlights

Baseline for all deltas below: **v0_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (v0_blob) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline v0_blob has the worst median (118 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest v2_decomposed at 96 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### v2_decomposed is fastest but the noisiest (CV 13.3%)

v2_decomposed wins on median (96 ns) yet has the highest variance (CV 13.3%), while v1_snapshot is the steadiest (CV 9.1%, 117 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (v2_decomposed, v4_erased) are a dead heat (<1%)

v2_decomposed (96 ns) and v4_erased (97 ns) differ by 0.83%, inside the noise, even though the wider field spreads 22.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### v3_shapebound shows alternating (throttle bounce) (autocorr -0.53)

v3_shapebound's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### v1_snapshot's edge over baseline is significant but tiny (-0 ns, 0.42%)

v1_snapshot differs from baseline v0_blob by -0 ns (0.42%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: v2_decomposed** at 96.1 ns median (-18.4% vs baseline)
- 3 variants significantly faster than baseline
- Spread: 1.22x (fastest 96.1 ns, slowest 117.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Resource storage layouts under L1-evicting heavy-memory mix-in |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112746324 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| v0_blob | 3992ns | 4285ns | 3508ns | 4041ns | 4327ns | base |
| v1_snapshot | 3991ns | 4292ns | 3511ns | 4042ns | 4318ns | -0.02% |
| v2_decomposed | 3716ns | 3518ns | 3509ns | 3543ns | 4442ns | -6.91% |
| v3_shapebound | 3936ns | 4280ns | 3503ns | 3951ns | 4322ns | -1.40% |
| v4_erased | 3831ns | 3533ns | 3504ns | 3779ns | 4313ns | -4.02% |
| v5_handletable | 3884ns | 3536ns | 3508ns | 3866ns | 4313ns | -2.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| v0_blob | 110ns | 96ns | 119ns | base | 9.344 |
| v1_snapshot | 109ns | 96ns | 119ns | -0.46% | 9.387 |
| v2_decomposed | 101ns | 96ns | 121ns | -7.41% | 10.092 |
| v3_shapebound | 107ns | 96ns | 118ns | -1.95% | 9.530 |
| v4_erased | 105ns | 96ns | 118ns | -3.98% | 9.732 |
| v5_handletable | 107ns | 96ns | 119ns | -2.63% | 9.597 |

## Performance model

- Peak throughput: **10.707 Gops/s** (v3_shapebound; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| v0_blob | 8.700 | 81.3% |
| v1_snapshot | 8.741 | 81.6% |
| v2_decomposed | 10.656 | 99.5% |
| v3_shapebound | 8.771 | 81.9% |
| v4_erased | 10.568 | 98.7% |
| v5_handletable | 10.535 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| v0_blob | 3992ns | 3992ns | base |
| v1_snapshot | 3991ns | 3991ns | -0.02% |
| v2_decomposed | 3716ns | 3716ns | -6.91% |
| v3_shapebound | 3936ns | 3936ns | -1.40% |
| v4_erased | 3831ns | 3831ns | -4.02% |
| v5_handletable | 3884ns | 3884ns | -2.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| v0_blob | 118ns | base | --- | [117, 118] | --- | --- | --- | --- |
| v1_snapshot | 117ns | -0.5ns (-0.4%) | [-1, -0]ns | [117, 117] | YES | 0.0005 | 0.0003 | 2 |
| v2_decomposed | 96ns | -1.5ns (-1.2%) | [-21, -1]ns | [96, 96] | YES | 0.0000 | 0.0000 | 2 |
| v3_shapebound | 117ns | -0.8ns (-0.7%) | [-1, -0]ns | [96, 117] | YES | 0.0005 | 0.0002 | 1 |
| v4_erased | 97ns | no significant difference | [-4, +0]ns | [97, 98] | no | 0.7140 | 0.7140 | 1 |
| v5_handletable | 97ns | no significant difference | [-1, +0]ns | [97, 118] | no | 0.3340 | 0.2672 | 3 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | v0_blob | v1_snapshot | v2_decomposed | v3_shapebound | v4_erased | v5_handletable |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 96ns | -0.6% | -0.6% | -0.6% | +0.3% | +0.2% |
| 1 | 2 | 96ns | -0.5% | -0.3% | +21.9% | +0.5% | -0.2% |
| 1 | 3 | 118ns | -18.8% | -0.5% | -0.8% | -18.1% | -0.1% |
| 1 | 4 | 118ns | -18.5% | -18.6% | -18.7% | +0.3% | -18.0% |
| 1 | 5 | 118ns | -18.5% | -18.5% | -18.8% | -18.1% | -17.9% |
| 2 | 1 | 118ns | -0.3% | -18.6% | -0.6% | -17.8% | +0.4% |
| 2 | 2 | 118ns | -18.5% | -0.5% | -18.7% | -18.0% | -17.8% |
| 2 | 3 | 118ns | -0.3% | -18.3% | -0.3% | -17.9% | -17.7% |
| 2 | 4 | 96ns | +22.3% | +3.2% | +21.9% | +22.9% | +22.7% |
| 2 | 5 | 118ns | -0.5% | -18.5% | -18.4% | +0.3% | +0.0% |
| 3 | 1 | 96ns | +21.7% | -0.4% | +21.7% | +22.8% | +22.6% |
| 3 | 2 | 118ns | -0.2% | -18.4% | -18.4% | +0.6% | -18.1% |
| 3 | 3 | 118ns | -0.3% | -18.7% | -0.3% | -17.8% | +0.0% |
| 3 | 4 | 96ns | +22.0% | -0.3% | -0.2% | +0.6% | -0.3% |
| 3 | 5 | 96ns | +21.9% | +21.9% | +21.9% | +22.7% | +22.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| v0_blob | 0.066 | ok |
| v1_snapshot | 0.512 | HIGH+ (drift/warm-up) |
| v2_decomposed | -0.206 | moderate- |
| v3_shapebound | -0.534 | HIGH- (thermal bounce) |
| v4_erased | 0.232 | moderate+ |
| v5_handletable | -0.284 | moderate- |

**Consistency summary:**

- **v1_snapshot**: won 11/15, lost 4/15
- **v2_decomposed**: won 13/15, lost 2/15
- **v3_shapebound**: won 11/15, lost 4/15
- **v4_erased**: won 6/15, lost 9/15
- **v5_handletable**: won 7/15, lost 5/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| v0_blob | 2287.4ns | 109.6ns | 2087.3% | HIGH |
| v1_snapshot | 2286.5ns | 109.1ns | 2096.2% | HIGH |
| v2_decomposed | 2131.7ns | 101.5ns | 2100.8% | HIGH |
| v3_shapebound | 2258.5ns | 107.5ns | 2101.9% | HIGH |
| v4_erased | 2197.7ns | 105.2ns | 2088.6% | HIGH |
| v5_handletable | 2255.1ns | 106.7ns | 2113.5% | HIGH |

## Distribution (algo ns)

```
v0_blob (n=120, range 96.2-119.4 ns)
     96.2 |########################
     97.4 |
     98.5 |
     99.7 |#
    100.9 |
    102.0 |
    103.2 |
    104.3 |
    105.5 |
    106.6 |
    107.8 |
    109.0 |
    110.1 |
    111.3 |
    112.4 |
    113.6 |
    114.7 |
    115.9 |
    117.1 |########################################
    118.2 |###
  (7 below, 5 above range)

v1_snapshot (n=120, range 95.7-118.6 ns)
     95.7 |###################################
     96.9 |#
     98.0 |##
     99.2 |#
    100.3 |
    101.4 |
    102.6 |
    103.7 |
    104.9 |
    106.0 |
    107.2 |
    108.3 |
    109.5 |
    110.6 |
    111.7 |
    112.9 |
    114.0 |
    115.2 |
    116.3 |########################################
    117.5 |#################################
  (11 below, 4 above range)

v2_decomposed (n=120, range 95.7-121.1 ns)
     95.7 |########################################
     97.0 |#
     98.2 |
     99.5 |
    100.8 |
    102.0 |
    103.3 |
    104.6 |
    105.8 |
    107.1 |
    108.4 |
    109.7 |
    110.9 |
    112.2 |
    113.5 |
    114.7 |
    116.0 |#####
    117.3 |#######
    118.5 |
    119.8 |
  (15 below, 1 above range)

v3_shapebound (n=120, range 95.6-117.8 ns)
     95.6 |################################
     96.7 |
     97.9 |
     99.0 |
    100.1 |
    101.2 |
    102.3 |
    103.4 |
    104.5 |
    105.6 |
    106.7 |
    107.8 |
    108.9 |
    110.0 |
    111.1 |
    112.2 |
    113.4 |
    114.5 |
    115.6 |##
    116.7 |########################################
  (12 below, 12 above range)

v4_erased (n=120, range 96.1-118.4 ns)
     96.1 |########################################
     97.2 |
     98.3 |
     99.4 |
    100.6 |
    101.7 |
    102.8 |
    103.9 |
    105.0 |
    106.1 |
    107.3 |
    108.4 |
    109.5 |
    110.6 |
    111.7 |
    112.9 |
    114.0 |
    115.1 |
    116.2 |##
    117.3 |#########################
  (13 below, 8 above range)

v5_handletable (n=120, range 96.1-118.6 ns)
     96.1 |########################################
     97.2 |
     98.3 |
     99.5 |
    100.6 |
    101.7 |
    102.8 |
    104.0 |
    105.1 |
    106.2 |
    107.3 |
    108.5 |
    109.6 |
    110.7 |
    111.8 |
    112.9 |
    114.1 |
    115.2 |
    116.3 |####
    117.4 |#####################################
  (12 below, 4 above range)

```

## Diagnostics

- **v0_blob**: bridge=2091.8% of algo (FFI overhead may distort results)
- **v1_snapshot**: autocorrelation=0.51 (measurement drift or warm-up artifact)
- **v1_snapshot**: bridge=2101.2% of algo (FFI overhead may distort results)
- **v2_decomposed**: bridge=2101.3% of algo (FFI overhead may distort results)
- **v3_shapebound**: bridge=2109.4% of algo (FFI overhead may distort results)
- **v4_erased**: bridge=2084.3% of algo (FFI overhead may distort results)
- **v5_handletable**: bridge=2107.0% of algo (FFI overhead may distort results)
