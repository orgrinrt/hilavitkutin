# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (5.06 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 1.07 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 79% (significant)

hash_xxh is -3.99 us (79%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.7x slower than the field

hash_fnv1a (5.06 us) is 4.7x the fastest (1.07 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.7x the fastest

Fastest hash_xxh (1.07 us) to slowest hash_fnv1a (5.06 us): 4.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 1070.0 ns median (-78.9% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.73x (fastest 1070.0 ns, slowest 5064.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1111222233334444 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 7186ns | 7167ns | 7160ns | 7170ns | 7260ns | base |
| hash_sip | 3216ns | 3210ns | 3203ns | 3210ns | 3246ns | -55.24% |
| hash_xxh | 3181ns | 3172ns | 3166ns | 3173ns | 3222ns | -55.73% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 5076ns | 5059ns | 5128ns | base | 0.807 |
| hash_sip | 1125ns | 1119ns | 1137ns | -77.83% | 3.640 |
| hash_xxh | 1072ns | 1066ns | 1082ns | -78.89% | 3.822 |

## Performance model

- Peak throughput: **3.844 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.809 | 21.0% |
| hash_sip | 3.646 | 94.9% |
| hash_xxh | 3.828 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 7186ns | 7186ns | base |
| hash_sip | 3216ns | 3216ns | -55.24% |
| hash_xxh | 3181ns | 3181ns | -55.73% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 5064ns | base | --- | [5063, 5065] | --- | --- | --- | --- |
| hash_sip | 1123ns | -3940.6ns (-77.8%) | [-3941, -3940]ns | [1123, 1124] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 1070ns | -3994.1ns (-78.9%) | [-3995, -3993]ns | [1070, 1071] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 5065ns | -77.8% | -78.9% |
| 1 | 2 | 5064ns | -77.8% | -78.9% |
| 1 | 3 | 5064ns | -77.8% | -78.9% |
| 1 | 4 | 5063ns | -77.8% | -78.9% |
| 1 | 5 | 5066ns | -77.8% | -78.9% |
| 1 | 6 | 5063ns | -77.8% | -78.9% |
| 1 | 7 | 5063ns | -77.8% | -78.9% |
| 1 | 8 | 5066ns | -77.8% | -78.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.406 | moderate- |
| hash_sip | 0.240 | moderate+ |
| hash_xxh | -0.357 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.7ns | 5076.1ns | 0.1% |  |
| hash_sip | 2.5ns | 1125.1ns | 0.2% |  |
| hash_xxh | 3.1ns | 1071.6ns | 0.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 5058.9-5127.9 ns)
   5058.9 |################################
   5062.4 |########################################
   5065.8 |#########
   5069.3 |##
   5072.7 |###
   5076.2 |###
   5079.6 |#
   5083.1 |
   5086.5 |
   5090.0 |##
   5093.4 |
   5096.9 |
   5100.3 |
   5103.8 |##
   5107.2 |
   5110.7 |#
   5114.1 |
   5117.6 |
   5121.0 |
   5124.5 |
  (12 below, 12 above range)

hash_sip (n=160, range 1118.9-1136.7 ns)
   1118.9 |##########
   1119.8 |###############
   1120.7 |###########
   1121.6 |####################
   1122.5 |########################################
   1123.3 |################
   1124.2 |#############
   1125.1 |#################
   1126.0 |#####
   1126.9 |#####
   1127.8 |###
   1128.7 |#####
   1129.6 |#
   1130.5 |##
   1131.4 |#
   1132.3 |
   1133.2 |#
   1134.1 |
   1134.9 |###
   1135.8 |#
  (12 below, 9 above range)

hash_xxh (n=160, range 1065.5-1081.7 ns)
   1065.5 |##############
   1066.3 |###############################
   1067.2 |#####################
   1068.0 |#####################
   1068.8 |################
   1069.6 |########################################
   1070.4 |###########################
   1071.2 |###############################
   1072.0 |#########################
   1072.8 |################
   1073.6 |##############
   1074.4 |########
   1075.2 |####
   1076.0 |##
   1076.9 |
   1077.7 |
   1078.5 |##
   1079.3 |##
   1080.1 |######
   1080.9 |##
  (13 below, 10 above range)

```
