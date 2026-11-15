# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.17 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 54 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1057% faster than the next best (iter_seq)

iter_strided (54 ns) leads iter_seq (629 ns) by 1057%, a clear separation rather than a photo finish. CV 9.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 95% (significant)

iter_strided is -1.11 us (95%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 21.5x slower than the field

iter_gather (1.17 us) is 21.5x the fastest (54 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 9.7%)

iter_strided wins on median (54 ns) yet has the highest variance (CV 9.7%), while iter_seq is the steadiest (CV 4.1%, 629 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 21.5x the fastest

Fastest iter_strided (54 ns) to slowest iter_gather (1.17 us): 21.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 54.4 ns median (-95.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.51x (fastest 54.4 ns, slowest 1170.0 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 3381ns | 3404ns | 3182ns | 3370ns | 3613ns | base |
| iter_seq | 2817ns | 2868ns | 2678ns | 2816ns | 2960ns | -16.69% |
| iter_strided | 2293ns | 2278ns | 2127ns | 2281ns | 2494ns | -32.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1162ns | 1094ns | 1244ns | base | 3.525 |
| iter_seq | 620ns | 589ns | 654ns | -46.64% | 6.605 |
| iter_strided | 54ns | 47ns | 62ns | -95.32% | 75.353 |

## Performance model

- Peak throughput: **87.457 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.501 | 4.0% |
| iter_seq | 6.510 | 7.4% |
| iter_strided | 75.294 | 86.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3381ns | 3381ns | base |
| iter_seq | 2817ns | 2817ns | -16.69% |
| iter_strided | 2293ns | 2293ns | -32.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1170ns | base | --- | [1167, 1171] | --- | --- | --- | --- |
| iter_seq | 629ns | -538.9ns (-46.1%) | [-542, -537]ns | [621, 631] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 54ns | -1115.0ns (-95.3%) | [-1118, -1112]ns | [54, 55] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 1173ns | -49.7% | -95.2% |
| 1 | 2 | 1109ns | -46.6% | -95.5% |
| 1 | 3 | 1095ns | -42.0% | -94.8% |
| 1 | 4 | 1259ns | -49.7% | -95.5% |
| 1 | 5 | 1169ns | -42.2% | -95.6% |
| 1 | 6 | 1165ns | -46.1% | -95.3% |
| 1 | 7 | 1177ns | -49.8% | -95.1% |
| 1 | 8 | 1170ns | -46.1% | -95.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.160 | ok |
| iter_seq | 0.190 | ok |
| iter_strided | -0.457 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 3.1ns | 1162.1ns | 0.3% |  |
| iter_seq | 2.9ns | 620.1ns | 0.5% |  |
| iter_strided | 2.7ns | 54.4ns | 4.9% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 1094.2-1243.5 ns)
   1094.2 |############
   1101.7 |###
   1109.1 |###
   1116.6 |#
   1124.1 |####
   1131.5 |#########
   1139.0 |##
   1146.5 |
   1153.9 |#
   1161.4 |##########
   1168.9 |########################################
   1176.3 |#######
   1183.8 |
   1191.3 |#
   1198.7 |
   1206.2 |##
   1213.7 |
   1221.1 |#
   1228.6 |
   1236.1 |
  (18 below, 19 above range)

iter_seq (n=160, range 588.5-654.1 ns)
    588.5 |######################################
    591.8 |##########################
    595.1 |########
    598.4 |#
    601.6 |#
    604.9 |###
    608.2 |#
    611.5 |######
    614.7 |#
    618.0 |###
    621.3 |###
    624.6 |########
    627.9 |####################################
    631.1 |######################################
    634.4 |########################################
    637.7 |######
    641.0 |###
    644.3 |
    647.5 |
    650.8 |
  (14 below, 13 above range)

iter_strided (n=160, range 46.8-61.6 ns)
     46.8 |###############
     47.6 |############
     48.3 |#####
     49.1 |###############
     49.8 |####################
     50.5 |####################
     51.3 |##########
     52.0 |####################
     52.8 |############
     53.5 |########################################
     54.2 |############
     55.0 |#########################
     55.7 |####################
     56.5 |#########################
     57.2 |#################
     57.9 |####################
     58.7 |####################
     59.4 |###############
     60.2 |#######
     60.9 |##
  (12 below, 14 above range)

```
