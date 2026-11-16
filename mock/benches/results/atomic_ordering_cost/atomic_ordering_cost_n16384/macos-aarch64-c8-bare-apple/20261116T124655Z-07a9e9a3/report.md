# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (14.30 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 4.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed dominates: 229% faster than the next best (atomic_seqcst)

atomic_relaxed (4.35 us) leads atomic_seqcst (14.28 us) by 229%, a clear separation rather than a photo finish. CV 0.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### atomic_relaxed beats baseline by 70% (significant)

atomic_relaxed is -9.95 us (70%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_acquire is an outlier: 3.3x slower than the field

atomic_acquire (14.30 us) is 3.3x the fastest (4.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### atomic_acquire shows alternating (throttle bounce) (autocorr -0.56)

atomic_acquire's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.3x the fastest

Fastest atomic_relaxed (4.35 us) to slowest atomic_acquire (14.30 us): 3.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: atomic_relaxed** at 4347.3 ns median (-69.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 3.29x (fastest 4347.3 ns, slowest 14296.9 ns)

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
| atomic_acquire | 16570ns | 16457ns | 16350ns | 16514ns | 16956ns | base |
| atomic_relaxed | 6491ns | 6482ns | 6471ns | 6483ns | 6533ns | -60.83% |
| atomic_seqcst | 16371ns | 16435ns | 16129ns | 16393ns | 16548ns | -1.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 14398ns | 14207ns | 14740ns | base | 1.138 |
| atomic_relaxed | 4352ns | 4339ns | 4381ns | -69.77% | 3.764 |
| atomic_seqcst | 14225ns | 14016ns | 14377ns | -1.20% | 1.152 |

## Performance model

- Peak throughput: **3.776 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 1.146 | 30.3% |
| atomic_relaxed | 3.769 | 99.8% |
| atomic_seqcst | 1.147 | 30.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 16570ns | 16570ns | base |
| atomic_relaxed | 6491ns | 6491ns | -60.83% |
| atomic_seqcst | 16371ns | 16371ns | -1.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 14297ns | base | --- | [14285, 14327] | --- | --- | --- | --- |
| atomic_relaxed | 4347ns | -9945.6ns (-69.6%) | [-9974, -9940]ns | [4346, 4349] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 14281ns | -60.4ns (-0.4%) | [-136, -13]ns | [14279, 14283] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 14287ns | -69.5% | -0.1% |
| 1 | 2 | 14502ns | -70.0% | -1.4% |
| 1 | 3 | 14283ns | -69.6% | -0.0% |
| 1 | 4 | 14536ns | -70.1% | -3.3% |
| 1 | 5 | 14282ns | -69.6% | -0.0% |
| 1 | 6 | 14754ns | -70.5% | -3.2% |
| 1 | 7 | 14282ns | -69.6% | -0.0% |
| 1 | 8 | 14281ns | -69.5% | -1.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | -0.563 | HIGH- (thermal bounce) |
| atomic_relaxed | 0.030 | ok |
| atomic_seqcst | -0.175 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 4/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 3.4ns | 14398.2ns | 0.0% |  |
| atomic_relaxed | 3.0ns | 4352.4ns | 0.1% |  |
| atomic_seqcst | 3.0ns | 14224.8ns | 0.0% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 14206.7-14739.7 ns)
  14206.7 |
  14233.4 |
  14260.0 |########################################
  14286.7 |############
  14313.3 |####
  14340.0 |#####
  14366.6 |#
  14393.3 |#
  14419.9 |
  14446.6 |
  14473.2 |####
  14499.8 |######
  14526.5 |####
  14553.1 |####
  14579.8 |##
  14606.4 |##
  14633.1 |
  14659.7 |
  14686.4 |
  14713.0 |##
  (11 below, 10 above range)

atomic_relaxed (n=160, range 4338.8-4381.1 ns)
   4338.8 |#####################
   4340.9 |############################
   4343.0 |#######################
   4345.1 |########################################
   4347.3 |########################################
   4349.4 |##################
   4351.5 |###############
   4353.6 |#############
   4355.7 |######
   4357.8 |######
   4360.0 |
   4362.1 |###
   4364.2 |#
   4366.3 |###
   4368.4 |
   4370.6 |#
   4372.7 |#####
   4374.8 |
   4376.9 |#
   4379.0 |
  (12 below, 10 above range)

atomic_seqcst (n=160, range 14015.5-14376.8 ns)
  14015.5 |######
  14033.6 |#
  14051.7 |#######
  14069.7 |
  14087.8 |#
  14105.9 |
  14123.9 |
  14142.0 |
  14160.0 |
  14178.1 |
  14196.2 |
  14214.2 |
  14232.3 |
  14250.4 |
  14268.4 |########################################
  14286.5 |############
  14304.6 |#####
  14322.6 |#######
  14340.7 |##
  14358.8 |
  (23 below, 11 above range)

```
