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

### hash_xxh shows alternating (throttle bounce) (autocorr -0.57)

hash_xxh's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.5x the fastest

Fastest hash_sip (4.54 us) to slowest hash_fnv1a (20.41 us): 4.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_sip** at 4538.1 ns median (-77.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.50x (fastest 4538.1 ns, slowest 20409.6 ns)

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
| hash_fnv1a | 22547ns | 22522ns | 22502ns | 22531ns | 22638ns | base |
| hash_sip | 6668ns | 6626ns | 6619ns | 6632ns | 6823ns | -70.43% |
| hash_xxh | 6729ns | 6725ns | 6645ns | 6724ns | 6829ns | -70.15% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 20439ns | 20400ns | 20530ns | base | 0.802 |
| hash_sip | 4565ns | 4532ns | 4673ns | -77.67% | 3.589 |
| hash_xxh | 4622ns | 4544ns | 4710ns | -77.39% | 3.545 |

## Performance model

- Peak throughput: **3.615 Gops/s** (hash_sip; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.803 | 22.2% |
| hash_sip | 3.610 | 99.9% |
| hash_xxh | 3.547 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 22547ns | 22547ns | base |
| hash_sip | 6668ns | 6668ns | -70.43% |
| hash_xxh | 6729ns | 6729ns | -70.15% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 20410ns | base | --- | [20406, 20422] | --- | --- | --- | --- |
| hash_sip | 4538ns | -15869.2ns (-77.8%) | [-15872, -15868]ns | [4537, 4539] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4619ns | -15815.6ns (-77.5%) | [-15830, -15806]ns | [4598, 4632] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 20406ns | -77.8% | -77.4% |
| 1 | 2 | 20430ns | -77.0% | -77.4% |
| 1 | 3 | 20432ns | -77.8% | -77.6% |
| 1 | 4 | 20405ns | -77.8% | -77.3% |
| 1 | 5 | 20413ns | -77.8% | -77.3% |
| 1 | 6 | 20408ns | -77.8% | -77.5% |
| 1 | 7 | 20409ns | -77.8% | -77.2% |
| 1 | 8 | 20405ns | -77.8% | -77.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.103 | ok |
| hash_sip | -0.155 | ok |
| hash_xxh | -0.570 | HIGH- (thermal bounce) |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.9ns | 20439.3ns | 0.0% |  |
| hash_sip | 2.6ns | 4564.6ns | 0.1% |  |
| hash_xxh | 3.1ns | 4621.6ns | 0.1% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 20400.3-20529.5 ns)
  20400.3 |########################################
  20406.8 |######
  20413.3 |###
  20419.7 |###
  20426.2 |#
  20432.6 |#
  20439.1 |##
  20445.6 |#####
  20452.0 |####
  20458.5 |#
  20464.9 |###
  20471.4 |#
  20477.8 |
  20484.3 |###
  20490.8 |#
  20497.2 |##
  20503.7 |#
  20510.1 |#
  20516.6 |#
  20523.0 |
  (13 below, 11 above range)

hash_sip (n=160, range 4531.8-4672.8 ns)
   4531.8 |########################################
   4538.8 |##################
   4545.9 |
   4552.9 |#
   4560.0 |#
   4567.0 |#
   4574.1 |
   4581.2 |#
   4588.2 |
   4595.3 |#
   4602.3 |
   4609.4 |
   4616.4 |##
   4623.5 |
   4630.5 |
   4637.6 |
   4644.6 |
   4651.7 |
   4658.7 |
   4665.8 |
  (13 below, 16 above range)

hash_xxh (n=160, range 4543.7-4709.9 ns)
   4543.7 |#################
   4552.0 |###########
   4560.4 |##############
   4568.7 |####################
   4577.0 |#########################
   4585.3 |########################################
   4593.6 |#########################
   4601.9 |########
   4610.2 |#################
   4618.5 |############################
   4626.8 |#########################
   4635.1 |##############
   4643.4 |######################
   4651.7 |#################
   4660.0 |#################
   4668.3 |#################
   4676.6 |#################
   4684.9 |####################
   4693.3 |#####
   4701.6 |########
  (16 below, 13 above range)

```
