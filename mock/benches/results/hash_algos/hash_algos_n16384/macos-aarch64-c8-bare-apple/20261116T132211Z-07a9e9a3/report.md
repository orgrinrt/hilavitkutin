# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (20.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_sip at 4.54 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_sip beats baseline by 78% (significant)

hash_sip is -15.87 us (78%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.5x slower than the field

hash_fnv1a (20.41 us) is 4.5x the fastest (4.54 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh shows alternating (throttle bounce) (autocorr -0.54)

hash_xxh's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.5x the fastest

Fastest hash_sip (4.54 us) to slowest hash_fnv1a (20.41 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_sip** at 4535.8 ns median (-77.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.50x (fastest 4535.8 ns, slowest 20405.4 ns)

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
| hash_fnv1a | 22539ns | 22514ns | 22502ns | 22525ns | 22615ns | base |
| hash_sip | 6630ns | 6623ns | 6618ns | 6624ns | 6663ns | -70.58% |
| hash_xxh | 6738ns | 6715ns | 6652ns | 6719ns | 6884ns | -70.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 20430ns | 20399ns | 20503ns | base | 0.802 |
| hash_sip | 4541ns | 4532ns | 4563ns | -77.77% | 3.608 |
| hash_xxh | 4630ns | 4550ns | 4758ns | -77.34% | 3.539 |

## Performance model

- Peak throughput: **3.616 Gops/s** (hash_sip; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.803 | 22.2% |
| hash_sip | 3.612 | 99.9% |
| hash_xxh | 3.554 | 98.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 22539ns | 22539ns | base |
| hash_sip | 6630ns | 6630ns | -70.58% |
| hash_xxh | 6738ns | 6738ns | -70.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 20405ns | base | --- | [20404, 20409] | --- | --- | --- | --- |
| hash_sip | 4536ns | -15869.1ns (-77.8%) | [-15870, -15868]ns | [4535, 4537] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4609ns | -15814.5ns (-77.5%) | [-15823, -15794]ns | [4597, 4623] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 20407ns | -77.8% | -77.3% |
| 1 | 2 | 20406ns | -77.8% | -77.4% |
| 1 | 3 | 20407ns | -77.8% | -77.4% |
| 1 | 4 | 20404ns | -77.8% | -77.4% |
| 1 | 5 | 20408ns | -77.8% | -77.3% |
| 1 | 6 | 20404ns | -77.8% | -77.5% |
| 1 | 7 | 20405ns | -77.8% | -77.4% |
| 1 | 8 | 20405ns | -77.8% | -77.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.458 | moderate- |
| hash_sip | -0.100 | ok |
| hash_xxh | -0.536 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.8ns | 20429.9ns | 0.0% |  |
| hash_sip | 2.7ns | 4540.6ns | 0.1% |  |
| hash_xxh | 2.9ns | 4629.9ns | 0.1% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 20399.3-20503.4 ns)
  20399.3 |########################################
  20404.5 |#####################
  20409.7 |#
  20414.9 |#
  20420.1 |
  20425.3 |
  20430.5 |#
  20435.7 |###
  20440.9 |##
  20446.1 |#########
  20451.3 |##
  20456.5 |##
  20461.7 |#
  20467.0 |#
  20472.2 |
  20477.4 |
  20482.6 |##
  20487.8 |##
  20493.0 |##
  20498.2 |
  (13 below, 11 above range)

hash_sip (n=160, range 4531.5-4562.8 ns)
   4531.5 |################
   4533.1 |########################
   4534.7 |########################################
   4536.2 |###################
   4537.8 |##################
   4539.4 |#######
   4540.9 |##
   4542.5 |#
   4544.0 |#
   4545.6 |##
   4547.2 |
   4548.7 |#
   4550.3 |
   4551.9 |#
   4553.4 |
   4555.0 |#
   4556.5 |##
   4558.1 |
   4559.7 |
   4561.2 |
  (12 below, 14 above range)

hash_xxh (n=160, range 4549.5-4758.5 ns)
   4549.5 |#############
   4560.0 |######################
   4570.4 |#####################################
   4580.9 |########################################
   4591.3 |###########
   4601.8 |############################
   4612.2 |###############
   4622.6 |###############
   4633.1 |######################
   4643.5 |#################
   4654.0 |########
   4664.4 |###########
   4674.9 |####################
   4685.3 |####
   4695.8 |###########
   4706.2 |########
   4716.7 |####
   4727.1 |##
   4737.6 |##
   4748.0 |
  (15 below, 11 above range)

```
