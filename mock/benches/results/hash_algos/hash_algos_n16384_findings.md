# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (20.44 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_sip at 4.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_sip beats baseline by 78% (significant)

hash_sip is -15.88 us (78%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.5x slower than the field

hash_fnv1a (20.44 us) is 4.5x the fastest (4.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.5x the fastest

Fastest hash_sip (4.54 us) to slowest hash_fnv1a (20.44 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_sip** at 4538.8 ns median (-77.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.50x (fastest 4538.8 ns, slowest 20439.2 ns)

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
| hash_fnv1a | 22623ns | 22554ns | 22496ns | 22554ns | 22954ns | base |
| hash_sip | 6723ns | 6624ns | 6608ns | 6646ns | 7070ns | -70.28% |
| hash_xxh | 6954ns | 6925ns | 6696ns | 6933ns | 7275ns | -69.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 20509ns | 20401ns | 20814ns | base | 0.799 |
| hash_sip | 4605ns | 4532ns | 4838ns | -77.55% | 3.558 |
| hash_xxh | 4794ns | 4595ns | 5035ns | -76.63% | 3.418 |

## Performance model

- Peak throughput: **3.615 Gops/s** (hash_sip; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.802 | 22.2% |
| hash_sip | 3.610 | 99.9% |
| hash_xxh | 3.435 | 95.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 22623ns | 22623ns | base |
| hash_sip | 6723ns | 6723ns | -70.28% |
| hash_xxh | 6954ns | 6954ns | -69.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 20439ns | base | --- | [20422, 20459] | --- | --- | --- | --- |
| hash_sip | 4539ns | -15884.8ns (-77.7%) | [-15893, -15870]ns | [4538, 4542] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4770ns | -15693.8ns (-76.8%) | [-15731, -15635]ns | [4735, 4810] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 20453ns | -77.8% | -77.4% |
| 1 | 2 | 20405ns | -77.7% | -75.9% |
| 1 | 3 | 20459ns | -77.8% | -75.4% |
| 1 | 4 | 20531ns | -77.1% | -77.3% |
| 1 | 5 | 20433ns | -77.1% | -77.3% |
| 1 | 6 | 20456ns | -77.8% | -76.5% |
| 1 | 7 | 20419ns | -77.8% | -76.8% |
| 1 | 8 | 20422ns | -77.8% | -75.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.057 | ok |
| hash_sip | 0.263 | moderate+ |
| hash_xxh | -0.094 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 3.4ns | 20508.7ns | 0.0% |  |
| hash_sip | 2.7ns | 4604.7ns | 0.1% |  |
| hash_xxh | 3.2ns | 4793.9ns | 0.1% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 20401.0-20814.0 ns)
  20401.0 |########################################
  20421.7 |##############
  20442.3 |#############
  20463.0 |#############
  20483.6 |#######
  20504.3 |####
  20524.9 |##
  20545.6 |####
  20566.2 |#
  20586.9 |
  20607.5 |
  20628.2 |##
  20648.8 |#
  20669.5 |
  20690.1 |#
  20710.8 |
  20731.4 |
  20752.1 |
  20772.7 |
  20793.4 |
  (15 below, 9 above range)

hash_sip (n=160, range 4532.3-4838.3 ns)
   4532.3 |########################################
   4547.6 |#####
   4562.9 |##
   4578.2 |#
   4593.5 |
   4608.8 |###
   4624.1 |#
   4639.4 |
   4654.7 |
   4670.0 |
   4685.3 |#
   4700.6 |#
   4715.9 |
   4731.2 |
   4746.5 |
   4761.8 |
   4777.1 |#
   4792.4 |
   4807.7 |
   4823.0 |
  (13 below, 11 above range)

hash_xxh (n=160, range 4594.9-5034.7 ns)
   4594.9 |########################################
   4616.9 |####################################
   4638.9 |####################################
   4660.9 |################################
   4682.9 |####################################
   4704.9 |################################
   4726.9 |####################
   4748.9 |########################################
   4770.9 |############################
   4792.8 |########################
   4814.8 |####################
   4836.8 |########################
   4858.8 |####################
   4880.8 |####################
   4902.8 |########################
   4924.8 |################
   4946.8 |############################
   4968.8 |############
   4990.7 |############
   5012.7 |################################
  (13 below, 14 above range)

```
