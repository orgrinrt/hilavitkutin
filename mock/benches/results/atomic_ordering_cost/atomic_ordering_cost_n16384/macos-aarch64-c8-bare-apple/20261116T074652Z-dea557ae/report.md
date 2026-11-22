# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (14.28 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 4.34 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed dominates: 226% faster than the next best (atomic_seqcst)

atomic_relaxed (4.34 us) leads atomic_seqcst (14.17 us) by 226%, a clear separation rather than a photo finish. CV 0.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 70% (significant)

atomic_relaxed is -9.94 us (70%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_acquire is an outlier: 3.3x slower than the field

atomic_acquire (14.28 us) is 3.3x the fastest (4.34 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_acquire shows alternating (throttle bounce) (autocorr -0.61)

atomic_acquire's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.3x the fastest

Fastest atomic_relaxed (4.34 us) to slowest atomic_acquire (14.28 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: atomic_relaxed** at 4344.6 ns median (-69.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 3.29x (fastest 4344.6 ns, slowest 14284.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 16373ns | 16429ns | 16118ns | 16387ns | 16589ns | base |
| atomic_relaxed | 6468ns | 6472ns | 6402ns | 6473ns | 6519ns | -60.50% |
| atomic_seqcst | 16396ns | 16308ns | 16119ns | 16315ns | 16916ns | +0.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 14237ns | 14014ns | 14428ns | base | 1.151 |
| atomic_relaxed | 4342ns | 4301ns | 4375ns | -69.50% | 3.773 |
| atomic_seqcst | 14257ns | 14014ns | 14716ns | +0.14% | 1.149 |

## Performance model

- Peak throughput: **3.809 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.147 | 30.1% |
| atomic_relaxed | 3.771 | 99.0% |
| atomic_seqcst | 1.156 | 30.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 16373ns | 16373ns | base |
| atomic_relaxed | 6468ns | 6468ns | -60.50% |
| atomic_seqcst | 16396ns | 16396ns | +0.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 14284ns | base | --- | [14279, 14291] | --- | --- | --- | --- |
| atomic_relaxed | 4345ns | -9940.0ns (-69.6%) | [-9950, -9932]ns | [4343, 4347] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 14173ns | no significant difference | [-10, +12]ns | [14094, 14281] | no | 0.8126 | 0.8126 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 14300ns | -69.6% | -0.1% |
| 1 | 2 | 14335ns | -69.7% | -0.4% |
| 1 | 3 | 14092ns | -69.7% | -0.4% |
| 1 | 4 | 14315ns | -69.7% | +3.6% |
| 1 | 5 | 14033ns | -69.0% | +0.3% |
| 1 | 6 | 14284ns | -69.6% | -1.2% |
| 1 | 7 | 14015ns | -69.0% | +1.9% |
| 1 | 8 | 14298ns | -69.6% | -2.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.606 | HIGH- (thermal bounce) |
| atomic_relaxed | -0.159 | ok |
| atomic_seqcst | -0.445 | moderate- |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 5/8, lost 3/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.2ns | 14236.7ns | 0.0% |  |
| atomic_relaxed | 3.1ns | 4342.2ns | 0.1% |  |
| atomic_seqcst | 3.2ns | 14257.2ns | 0.0% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 14014.3-14428.4 ns)
  14014.3 |##########################
  14035.0 |#####
  14055.7 |#########
  14076.4 |#
  14097.1 |##
  14117.8 |
  14138.5 |
  14159.2 |
  14180.0 |#####
  14200.7 |
  14221.4 |
  14242.1 |
  14262.8 |##############################
  14283.5 |########################################
  14304.2 |#######################
  14324.9 |################
  14345.6 |#####
  14366.3 |########
  14387.0 |#
  14407.7 |####
  (17 below, 12 above range)

atomic_relaxed (n=160, range 4301.2-4375.5 ns)
   4301.2 |#
   4304.9 |
   4308.6 |#
   4312.3 |
   4316.0 |
   4319.7 |
   4323.5 |#
   4327.2 |#
   4330.9 |###
   4334.6 |##################
   4338.3 |##############
   4342.0 |########################################
   4345.7 |###################################
   4349.5 |################
   4353.2 |##########
   4356.9 |#####
   4360.6 |#
   4364.3 |##
   4368.0 |#
   4371.7 |#
  (15 below, 10 above range)

atomic_seqcst (n=160, range 14013.7-14716.5 ns)
  14013.7 |########################################
  14048.8 |###################
  14083.9 |############
  14119.1 |##
  14154.2 |####
  14189.4 |
  14224.5 |#
  14259.6 |#############################
  14294.8 |##############
  14329.9 |#############
  14365.1 |#
  14400.2 |##
  14435.4 |##
  14470.5 |#
  14505.6 |#
  14540.8 |###
  14575.9 |##
  14611.1 |####
  14646.2 |
  14681.3 |##
  (16 below, 13 above range)

```
