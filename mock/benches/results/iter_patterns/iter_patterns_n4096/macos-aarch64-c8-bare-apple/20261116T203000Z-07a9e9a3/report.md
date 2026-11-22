# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1044% faster than the next best (iter_seq)

iter_strided (52 ns) leads iter_seq (591 ns) by 1044%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 95% (significant)

iter_strided is -1.04 us (95%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 21.2x slower than the field

iter_gather (1.09 us) is 21.2x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 8.3%)

iter_strided wins on median (52 ns) yet has the highest variance (CV 8.3%), while iter_seq is the steadiest (CV 1.9%, 591 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 21.2x the fastest

Fastest iter_strided (52 ns) to slowest iter_gather (1.09 us): 21.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 51.7 ns median (-95.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.17x (fastest 51.7 ns, slowest 1094.6 ns)

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
| iter_gather | 3223ns | 3192ns | 3184ns | 3201ns | 3327ns | base |
| iter_seq | 2706ns | 2687ns | 2680ns | 2687ns | 2789ns | -16.04% |
| iter_strided | 2152ns | 2138ns | 2131ns | 2140ns | 2209ns | -33.23% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1105ns | 1090ns | 1147ns | base | 3.705 |
| iter_seq | 595ns | 587ns | 614ns | -46.17% | 6.883 |
| iter_strided | 51ns | 45ns | 57ns | -95.35% | 79.732 |

## Performance model

- Peak throughput: **90.827 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.742 | 4.1% |
| iter_seq | 6.928 | 7.6% |
| iter_strided | 79.226 | 87.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3223ns | 3223ns | base |
| iter_seq | 2706ns | 2706ns | -16.04% |
| iter_strided | 2152ns | 2152ns | -33.23% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1095ns | base | --- | [1094, 1095] | --- | --- | --- | --- |
| iter_seq | 591ns | -502.5ns (-45.9%) | [-504, -502]ns | [591, 592] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 52ns | -1043.8ns (-95.4%) | [-1045, -1043]ns | [51, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 1094ns | -46.0% | -95.3% |
| 1 | 2 | 1092ns | -45.9% | -95.3% |
| 1 | 3 | 1095ns | -46.1% | -95.3% |
| 1 | 4 | 1126ns | -47.5% | -95.3% |
| 1 | 5 | 1093ns | -46.0% | -95.4% |
| 1 | 6 | 1094ns | -43.6% | -95.5% |
| 1 | 7 | 1093ns | -45.9% | -95.2% |
| 1 | 8 | 1131ns | -47.7% | -95.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.191 | ok |
| iter_seq | -0.157 | ok |
| iter_strided | -0.005 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.8ns | 1105.4ns | 0.3% |  |
| iter_seq | 2.6ns | 595.1ns | 0.4% |  |
| iter_strided | 2.4ns | 51.4ns | 4.7% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 1089.5-1147.4 ns)
   1089.5 |##################
   1092.4 |########################################
   1095.3 |################
   1098.2 |###
   1101.1 |
   1104.0 |#
   1106.9 |###
   1109.8 |
   1112.7 |#
   1115.6 |
   1118.5 |
   1121.4 |
   1124.3 |###
   1127.2 |#####
   1130.0 |#####
   1132.9 |###
   1135.8 |
   1138.7 |##
   1141.6 |
   1144.5 |
  (14 below, 5 above range)

iter_seq (n=160, range 586.6-614.0 ns)
    586.6 |##############
    588.0 |#################
    589.4 |#######################
    590.8 |########################################
    592.1 |##########
    593.5 |###########
    594.9 |#######
    596.2 |
    597.6 |#
    599.0 |##
    600.3 |###
    601.7 |
    603.1 |#
    604.5 |##
    605.8 |#
    607.2 |##
    608.6 |
    609.9 |####
    611.3 |#
    612.7 |
  (13 below, 14 above range)

iter_strided (n=160, range 45.1-57.2 ns)
     45.1 |#############
     45.7 |#############
     46.3 |########
     46.9 |###########
     47.5 |###########
     48.1 |####################
     48.7 |######
     49.3 |####
     49.9 |####################
     50.5 |########
     51.1 |########################################
     51.7 |#############
     52.3 |############################
     52.9 |#############
     53.5 |###############
     54.1 |##########################
     54.7 |######
     55.4 |####################
     56.0 |######
     56.6 |###########
  (13 below, 12 above range)

```
