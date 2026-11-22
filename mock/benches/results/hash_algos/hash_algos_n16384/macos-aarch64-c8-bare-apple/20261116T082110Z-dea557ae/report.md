# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (20.46 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_sip at 4.62 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_sip beats baseline by 78% (significant)

hash_sip is -15.87 us (78%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 4.4x slower than the field

hash_fnv1a (20.46 us) is 4.4x the fastest (4.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 4.4x the fastest

Fastest hash_sip (4.62 us) to slowest hash_fnv1a (20.46 us): 4.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_sip** at 4622.1 ns median (-77.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.43x (fastest 4622.1 ns, slowest 20456.7 ns)

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
| hash_fnv1a | 22629ns | 22556ns | 22493ns | 22580ns | 22911ns | base |
| hash_sip | 6724ns | 6736ns | 6612ns | 6731ns | 6813ns | -70.29% |
| hash_xxh | 6860ns | 6868ns | 6710ns | 6863ns | 7002ns | -69.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 20521ns | 20401ns | 20773ns | base | 0.798 |
| hash_sip | 4612ns | 4536ns | 4673ns | -77.52% | 3.552 |
| hash_xxh | 4738ns | 4612ns | 4859ns | -76.91% | 3.458 |

## Performance model

- Peak throughput: **3.612 Gops/s** (hash_sip; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.801 | 22.2% |
| hash_sip | 3.545 | 98.1% |
| hash_xxh | 3.453 | 95.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 22629ns | 22629ns | base |
| hash_sip | 6724ns | 6724ns | -70.29% |
| hash_xxh | 6860ns | 6860ns | -69.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 20457ns | base | --- | [20450, 20468] | --- | --- | --- | --- |
| hash_sip | 4622ns | -15866.2ns (-77.6%) | [-15898, -15841]ns | [4620, 4623] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4745ns | -15756.0ns (-77.0%) | [-15788, -15732]ns | [4722, 4761] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 20491ns | -77.4% | -76.6% |
| 1 | 2 | 20458ns | -77.4% | -76.7% |
| 1 | 3 | 20464ns | -77.4% | -76.8% |
| 1 | 4 | 20441ns | -77.4% | -76.7% |
| 1 | 5 | 20477ns | -77.6% | -76.6% |
| 1 | 6 | 20449ns | -77.3% | -77.4% |
| 1 | 7 | 20453ns | -77.4% | -76.9% |
| 1 | 8 | 20451ns | -77.4% | -76.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.255 | moderate- |
| hash_sip | -0.439 | moderate- |
| hash_xxh | -0.054 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 3.1ns | 20520.8ns | 0.0% |  |
| hash_sip | 2.8ns | 4612.2ns | 0.1% |  |
| hash_xxh | 3.3ns | 4737.7ns | 0.1% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 20401.2-20773.2 ns)
  20401.2 |########################################
  20419.8 |###################
  20438.4 |#########################
  20457.0 |####################
  20475.6 |#########
  20494.2 |###
  20512.8 |######
  20531.4 |###
  20550.0 |###
  20568.6 |###
  20587.2 |
  20605.8 |##
  20624.4 |###
  20643.0 |#####
  20661.6 |#########
  20680.2 |#
  20698.8 |##
  20717.4 |#
  20736.0 |###
  20754.6 |##
  (14 below, 15 above range)

hash_sip (n=160, range 4536.4-4672.6 ns)
   4536.4 |##############
   4543.2 |
   4550.0 |
   4556.8 |
   4563.6 |
   4570.4 |
   4577.3 |
   4584.1 |
   4590.9 |#
   4597.7 |#
   4604.5 |
   4611.3 |#######
   4618.1 |########################################
   4624.9 |########################
   4631.8 |#
   4638.6 |###
   4645.4 |#
   4652.2 |#####
   4659.0 |###
   4665.8 |
  (18 below, 13 above range)

hash_xxh (n=160, range 4612.2-4858.8 ns)
   4612.2 |##############
   4624.5 |#######
   4636.9 |##################
   4649.2 |########################################
   4661.5 |#########################
   4673.9 |#########################
   4686.2 |#####################
   4698.5 |#####################
   4710.8 |##################
   4723.2 |#############################
   4735.5 |##################
   4747.8 |########################################
   4760.2 |#########################
   4772.5 |#############################
   4784.8 |#########################
   4797.2 |####################################
   4809.5 |#########################
   4821.8 |####################################
   4834.2 |#######
   4846.5 |#######
  (15 below, 15 above range)

```
