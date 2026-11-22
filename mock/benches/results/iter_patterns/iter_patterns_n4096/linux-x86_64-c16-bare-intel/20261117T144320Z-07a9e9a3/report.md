# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 78 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 636% faster than the next best (iter_seq)

iter_strided (78 ns) leads iter_seq (577 ns) by 636%, a clear separation rather than a photo finish. CV 87.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 93% (significant)

iter_strided is -1.09 us (93%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 14.9x slower than the field

iter_gather (1.17 us) is 14.9x the fastest (78 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 87.3%)

iter_strided wins on median (78 ns) yet has the highest variance (CV 87.3%), while iter_gather is the steadiest (CV 49.2%, 1.17 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_gather shows alternating (throttle bounce) (autocorr -0.62)

iter_gather's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 14.9x the fastest

Fastest iter_strided (78 ns) to slowest iter_gather (1.17 us): 14.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 3.7x its best-20%

iter_strided's best 20% of batches run at 64 ns but its worst 20% at 238 ns (3.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 78.4 ns median (-93.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 14.86x (fastest 78.4 ns, slowest 1165.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 2112828630 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 3302ns | 3047ns | 2494ns | 2851ns | 5464ns | base |
| iter_seq | 3065ns | 2465ns | 2460ns | 2469ns | 5460ns | -7.17% |
| iter_strided | 2714ns | 1965ns | 1606ns | 1984ns | 6012ns | -17.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1258ns | 953ns | 2070ns | base | 3.256 |
| iter_seq | 716ns | 577ns | 1270ns | -43.10% | 5.722 |
| iter_strided | 108ns | 64ns | 238ns | -91.42% | 37.922 |

## Performance model

- Peak throughput: **63.916 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.516 | 5.5% |
| iter_seq | 7.098 | 11.1% |
| iter_strided | 52.245 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3302ns | 3302ns | base |
| iter_seq | 3065ns | 3065ns | -7.17% |
| iter_strided | 2714ns | 2714ns | -17.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1165ns | base | --- | [1134, 1166] | --- | --- | --- | --- |
| iter_seq | 577ns | -587.7ns (-50.4%) | [-588, -386]ns | [577, 577] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 78ns | -1086.7ns (-93.3%) | [-1088, -923]ns | [78, 78] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 954ns | -39.5% | -91.8% |
| 1 | 2 | 1166ns | -50.5% | -93.3% |
| 1 | 3 | 1166ns | -50.5% | -93.3% |
| 1 | 4 | 954ns | -39.5% | -93.3% |
| 1 | 5 | 1166ns | -50.5% | -94.5% |
| 1 | 6 | 953ns | -39.4% | -91.8% |
| 1 | 7 | 1166ns | -50.5% | -94.5% |
| 1 | 8 | 953ns | -37.2% | -91.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.624 | HIGH- (thermal bounce) |
| iter_seq | -0.021 | ok |
| iter_strided | -0.160 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 25.2ns | 1258.1ns | 2.0% |  |
| iter_seq | 34.5ns | 715.8ns | 4.8% |  |
| iter_strided | 32.4ns | 108.0ns | 30.0% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 952.7-2070.1 ns)
    952.7 |#################################
   1008.5 |
   1064.4 |
   1120.3 |########################################
   1176.2 |######
   1232.0 |
   1287.9 |
   1343.8 |
   1399.6 |
   1455.5 |
   1511.4 |
   1567.2 |
   1623.1 |
   1679.0 |
   1734.9 |
   1790.7 |
   1846.6 |
   1902.5 |
   1958.3 |
   2014.2 |
  (14 below, 16 above range)

iter_seq (n=160, range 576.9-1269.5 ns)
    576.9 |########################################
    611.5 |#
    646.1 |
    680.8 |
    715.4 |
    750.0 |
    784.7 |
    819.3 |
    853.9 |
    888.6 |
    923.2 |
    957.8 |
    992.5 |
   1027.1 |
   1061.7 |
   1096.4 |
   1131.0 |
   1165.6 |
   1200.3 |
   1234.9 |
  (11 below, 16 above range)

iter_strided (n=160, range 64.1-238.0 ns)
     64.1 |#################
     72.8 |########################################
     81.5 |##
     90.2 |
     98.9 |
    107.6 |
    116.3 |
    124.9 |
    133.6 |
    142.3 |#
    151.0 |
    159.7 |
    168.4 |
    177.1 |
    185.8 |
    194.5 |#
    203.2 |#
    211.9 |##
    220.6 |
    229.3 |##
  (7 below, 12 above range)

```

## Diagnostics

- **iter_gather**: CV=45.6% (high variance, measurements may be unstable)
- **iter_seq**: CV=52.3% (high variance, measurements may be unstable)
- **iter_strided**: CV=63.4% (high variance, measurements may be unstable)
- **iter_strided**: worst_20/best_20 = 3.7x (possible bimodal distribution)
- **iter_strided**: bridge=30.1% of algo (FFI overhead may distort results)
