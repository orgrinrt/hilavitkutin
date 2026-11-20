# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (1.15 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_low at 949 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bp_skew_low dominates: 21% faster than the next best (bp_skew_high)

bp_skew_low (949 ns) leads bp_skew_high (1.15 us) by 21%, a clear separation rather than a photo finish. CV 56.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### bp_skew_low is fastest but the noisiest (CV 56.4%)

bp_skew_low wins on median (949 ns) yet has the highest variance (CV 56.4%), while bp_balanced is the steadiest (CV 41.4%, 1.15 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (199 ns) is smaller than the fastest variant's own run-to-run std-dev (535 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### bp_skew_high's comparison is tie-heavy (10% tied pairs)

10% of paired samples for bp_skew_high are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### bp_skew_high is inconsistent: worst-20% is 2.5x its best-20%

bp_skew_high's best 20% of batches run at 939 ns but its worst 20% at 2.31 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

### bp_skew_high's edge over baseline is significant but tiny (-0 ns, 0.02%)

bp_skew_high differs from baseline bp_balanced by -0 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_low** at 948.5 ns median (-17.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.21x (fastest 948.5 ns, slowest 1147.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 2112755873 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 3314ns | 3035ns | 2724ns | 3036ns | 4739ns | base |
| bp_skew_high | 3408ns | 3029ns | 2480ns | 2814ns | 6116ns | +2.81% |
| bp_skew_low | 3148ns | 2493ns | 2477ns | 2725ns | 5090ns | -5.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 1251ns | 1032ns | 1781ns | base | 0.818 |
| bp_skew_high | 1287ns | 939ns | 2306ns | +2.89% | 0.795 |
| bp_skew_low | 1190ns | 939ns | 1916ns | -4.92% | 0.861 |

## Performance model

- Peak throughput: **1.091 Gops/s** (bp_skew_high; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.892 | 81.8% |
| bp_skew_high | 0.892 | 81.8% |
| bp_skew_low | 1.080 | 99.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 3314ns | 3314ns | base |
| bp_skew_high | 3408ns | 3408ns | +2.81% |
| bp_skew_low | 3148ns | 3148ns | -5.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 1148ns | base | --- | [1148, 1148] | --- | --- | --- | --- |
| bp_skew_high | 1148ns | -0.2ns (-0.0%) | [-10, -0]ns | [950, 1148] | YES | 0.0002 | 0.0002 | 16 |
| bp_skew_low | 949ns | -13.0ns (-1.1%) | [-208, -0]ns | [939, 1148] | YES | 0.0000 | 0.0000 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 1148ns | -18.2% | -0.0% |
| 1 | 2 | 939ns | +0.0% | +0.0% |
| 1 | 3 | 1148ns | -18.2% | -0.0% |
| 1 | 4 | 1148ns | +0.0% | -18.2% |
| 1 | 5 | 1148ns | -0.0% | -18.2% |
| 1 | 6 | 1148ns | -18.2% | -18.2% |
| 1 | 7 | 1148ns | +0.0% | -18.2% |
| 1 | 8 | 1148ns | +51.9% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | -0.161 | ok |
| bp_skew_high | 0.129 | ok |
| bp_skew_low | -0.275 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 3/8, lost 1/8
- **bp_skew_low**: won 4/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 26.3ns | 1251.3ns | 2.1% |  |
| bp_skew_high | 26.8ns | 1287.5ns | 2.1% |  |
| bp_skew_low | 24.7ns | 1189.7ns | 2.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 1031.7-1781.3 ns)
   1031.7 |
   1069.2 |
   1106.7 |
   1144.1 |########################################
   1181.6 |
   1219.1 |
   1256.6 |
   1294.1 |
   1331.5 |
   1369.0 |
   1406.5 |
   1444.0 |
   1481.4 |
   1518.9 |
   1556.4 |
   1593.9 |
   1631.4 |
   1668.8 |
   1706.3 |
   1743.8 |
  (18 below, 11 above range)

bp_skew_high (n=160, range 938.9-2305.8 ns)
    938.9 |########################################
   1007.2 |
   1075.6 |
   1143.9 |####################################
   1212.3 |#
   1280.6 |
   1349.0 |
   1417.3 |
   1485.6 |
   1554.0 |
   1622.3 |
   1690.7 |
   1759.0 |
   1827.4 |
   1895.7 |
   1964.1 |
   2032.4 |
   2100.8 |
   2169.1 |
   2237.4 |
  (5 below, 17 above range)

bp_skew_low (n=160, range 938.9-1916.4 ns)
    938.9 |########################################
    987.8 |
   1036.7 |
   1085.5 |
   1134.4 |#################################
   1183.3 |
   1232.2 |
   1281.0 |
   1329.9 |
   1378.8 |
   1427.7 |
   1476.5 |
   1525.4 |
   1574.3 |
   1623.2 |
   1672.0 |
   1720.9 |
   1769.8 |
   1818.7 |
   1867.5 |
  (20 below, 12 above range)

```

## Diagnostics

- **bp_balanced**: CV=38.0% (high variance, measurements may be unstable)
- **bp_skew_high**: CV=48.2% (high variance, measurements may be unstable)
- **bp_skew_low**: CV=45.0% (high variance, measurements may be unstable)
