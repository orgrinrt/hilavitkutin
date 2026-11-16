# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.12 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1052% faster than the next best (iter_seq)

iter_strided (52 ns) leads iter_seq (603 ns) by 1052%, a clear separation rather than a photo finish. CV 7.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 95% (significant)

iter_strided is -1.06 us (95%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 21.3x slower than the field

iter_gather (1.12 us) is 21.3x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 7.9%)

iter_strided wins on median (52 ns) yet has the highest variance (CV 7.9%), while iter_seq is the steadiest (CV 2.1%, 603 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 21.3x the fastest

Fastest iter_strided (52 ns) to slowest iter_gather (1.12 us): 21.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 52.3 ns median (-95.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.33x (fastest 52.3 ns, slowest 1115.4 ns)

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
| iter_gather | 3285ns | 3245ns | 3230ns | 3249ns | 3446ns | base |
| iter_seq | 2743ns | 2730ns | 2719ns | 2735ns | 2794ns | -16.48% |
| iter_strided | 2190ns | 2173ns | 2160ns | 2178ns | 2256ns | -33.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1126ns | 1109ns | 1170ns | base | 3.638 |
| iter_seq | 606ns | 597ns | 622ns | -46.19% | 6.762 |
| iter_strided | 53ns | 47ns | 58ns | -95.33% | 77.954 |

## Performance model

- Peak throughput: **87.826 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.672 | 4.2% |
| iter_seq | 6.796 | 7.7% |
| iter_strided | 78.317 | 89.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3285ns | 3285ns | base |
| iter_seq | 2743ns | 2743ns | -16.48% |
| iter_strided | 2190ns | 2190ns | -33.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1115ns | base | --- | [1114, 1117] | --- | --- | --- | --- |
| iter_seq | 603ns | -513.3ns (-46.0%) | [-514, -512]ns | [602, 603] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 52ns | -1063.6ns (-95.4%) | [-1065, -1062]ns | [52, 53] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 1118ns | -45.8% | -95.3% |
| 1 | 2 | 1122ns | -46.3% | -95.4% |
| 1 | 3 | 1112ns | -45.8% | -95.3% |
| 1 | 4 | 1113ns | -45.9% | -95.3% |
| 1 | 5 | 1114ns | -45.4% | -95.1% |
| 1 | 6 | 1117ns | -46.0% | -95.4% |
| 1 | 7 | 1117ns | -46.1% | -95.4% |
| 1 | 8 | 1112ns | -45.9% | -95.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.027 | ok |
| iter_seq | -0.145 | ok |
| iter_strided | -0.204 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.7ns | 1125.8ns | 0.2% |  |
| iter_seq | 2.7ns | 605.8ns | 0.4% |  |
| iter_strided | 2.5ns | 52.5ns | 4.8% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 1109.2-1170.2 ns)
   1109.2 |########################################
   1112.2 |#####################################
   1115.3 |############################
   1118.3 |#############
   1121.4 |######
   1124.4 |####
   1127.5 |#########
   1130.5 |#########
   1133.6 |###
   1136.6 |#
   1139.7 |###
   1142.8 |##
   1145.8 |#
   1148.9 |#
   1151.9 |#
   1155.0 |
   1158.0 |#
   1161.1 |
   1164.1 |
   1167.2 |#
  (9 below, 6 above range)

iter_seq (n=160, range 597.0-622.5 ns)
    597.0 |####################
    598.3 |#############
    599.6 |########################
    600.8 |###################################
    602.1 |########################################
    603.4 |#######
    604.7 |#############
    605.9 |#######
    607.2 |######
    608.5 |#######
    609.7 |###
    611.0 |######
    612.3 |#######
    613.6 |####
    614.8 |####
    616.1 |#
    617.4 |####
    618.7 |######
    619.9 |#
    621.2 |
  (13 below, 6 above range)

iter_strided (n=160, range 46.6-58.2 ns)
     46.6 |#############
     47.2 |################
     47.8 |##########################
     48.4 |##########
     48.9 |##########################
     49.5 |################
     50.1 |#######################
     50.7 |########################################
     51.3 |#################################
     51.8 |##############################
     52.4 |########################################
     53.0 |#######################
     53.6 |######
     54.1 |#######################
     54.7 |################
     55.3 |########################################
     55.9 |##########
     56.5 |##########################
     57.0 |##########################
     57.6 |######
  (9 below, 14 above range)

```
