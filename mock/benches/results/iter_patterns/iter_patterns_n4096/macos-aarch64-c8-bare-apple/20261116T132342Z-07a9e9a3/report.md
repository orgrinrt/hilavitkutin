# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (1.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 52 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 1043% faster than the next best (iter_seq)

iter_strided (52 ns) leads iter_seq (591 ns) by 1043%, a clear separation rather than a photo finish. CV 8.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 95% (significant)

iter_strided is -1.04 us (95%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 21.1x slower than the field

iter_gather (1.09 us) is 21.1x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 8.2%)

iter_strided wins on median (52 ns) yet has the highest variance (CV 8.2%), while iter_gather is the steadiest (CV 0.8%, 1.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_gather shows alternating (throttle bounce) (autocorr -0.56)

iter_gather's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 21.1x the fastest

Fastest iter_strided (52 ns) to slowest iter_gather (1.09 us): 21.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 51.7 ns median (-95.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 21.15x (fastest 51.7 ns, slowest 1093.3 ns)

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
| iter_gather | 3195ns | 3186ns | 3180ns | 3187ns | 3233ns | base |
| iter_seq | 2696ns | 2685ns | 2680ns | 2686ns | 2746ns | -15.59% |
| iter_strided | 2147ns | 2136ns | 2130ns | 2136ns | 2197ns | -32.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 1095ns | 1089ns | 1106ns | base | 3.741 |
| iter_seq | 593ns | 586ns | 605ns | -45.86% | 6.910 |
| iter_strided | 51ns | 45ns | 57ns | -95.32% | 79.967 |

## Performance model

- Peak throughput: **91.029 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.746 | 4.1% |
| iter_seq | 6.933 | 7.6% |
| iter_strided | 79.226 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 3195ns | 3195ns | base |
| iter_seq | 2696ns | 2696ns | -15.59% |
| iter_strided | 2147ns | 2147ns | -32.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 1093ns | base | --- | [1093, 1094] | --- | --- | --- | --- |
| iter_seq | 591ns | -502.5ns (-46.0%) | [-503, -502]ns | [590, 592] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 52ns | -1041.7ns (-95.3%) | [-1043, -1041]ns | [51, 52] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 1093ns | -45.9% | -95.3% |
| 1 | 2 | 1093ns | -45.9% | -95.4% |
| 1 | 3 | 1094ns | -46.0% | -95.3% |
| 1 | 4 | 1092ns | -45.9% | -95.2% |
| 1 | 5 | 1096ns | -44.9% | -95.1% |
| 1 | 6 | 1093ns | -46.0% | -95.4% |
| 1 | 7 | 1094ns | -46.1% | -95.4% |
| 1 | 8 | 1092ns | -45.9% | -95.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.561 | HIGH- (thermal bounce) |
| iter_seq | -0.159 | ok |
| iter_strided | 0.151 | ok |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.7ns | 1094.9ns | 0.2% |  |
| iter_seq | 2.6ns | 592.8ns | 0.4% |  |
| iter_strided | 2.5ns | 51.2ns | 4.8% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 1088.9-1106.2 ns)
   1088.9 |###########
   1089.7 |#################
   1090.6 |####################
   1091.5 |########################
   1092.3 |######################
   1093.2 |########################################
   1094.1 |########################
   1094.9 |#########################
   1095.8 |###
   1096.7 |#########
   1097.6 |####
   1098.4 |####
   1099.3 |
   1100.2 |#
   1101.0 |
   1101.9 |
   1102.8 |#
   1103.6 |###
   1104.5 |
   1105.4 |
  (15 below, 11 above range)

iter_seq (n=160, range 586.1-604.8 ns)
    586.1 |##########
    587.1 |####################
    588.0 |###############
    588.9 |########################
    589.9 |########################
    590.8 |########################################
    591.7 |#####################
    592.7 |#######################
    593.6 |############
    594.5 |######
    595.5 |#
    596.4 |
    597.3 |#
    598.3 |#
    599.2 |
    600.1 |#
    601.1 |
    602.0 |######
    602.9 |###
    603.9 |#
  (10 below, 10 above range)

iter_strided (n=160, range 45.0-57.0 ns)
     45.0 |###############
     45.6 |##########
     46.2 |###############
     46.8 |##########
     47.4 |####################
     48.0 |##########
     48.6 |##########
     49.2 |##########
     49.8 |########################################
     50.4 |###############
     51.0 |############
     51.6 |###################################
     52.2 |######################
     52.8 |#####################################
     53.4 |###############
     54.0 |####################
     54.6 |##
     55.2 |####################
     55.8 |#######
     56.4 |#######
  (11 below, 15 above range)

```
