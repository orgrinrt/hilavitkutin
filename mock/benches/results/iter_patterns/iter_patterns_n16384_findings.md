# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.73 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 279 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 843% faster than the next best (iter_seq)

iter_strided (279 ns) leads iter_seq (2.63 us) by 843%, a clear separation rather than a photo finish. CV 4.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.46 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 17.0x slower than the field

iter_gather (4.73 us) is 17.0x the fastest (279 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided shows alternating (throttle bounce) (autocorr -0.53)

iter_strided's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 17.0x the fastest

Fastest iter_strided (279 ns) to slowest iter_gather (4.73 us): 17.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 278.8 ns median (-94.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 16.96x (fastest 278.8 ns, slowest 4728.4 ns)

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
| iter_gather | 6919ns | 6877ns | 6668ns | 6903ns | 7216ns | base |
| iter_seq | 4856ns | 4820ns | 4611ns | 4824ns | 5198ns | -29.81% |
| iter_strided | 2459ns | 2446ns | 2353ns | 2453ns | 2583ns | -64.46% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4755ns | 4582ns | 4955ns | base | 3.446 |
| iter_seq | 2647ns | 2508ns | 2840ns | -44.33% | 6.190 |
| iter_strided | 281ns | 267ns | 298ns | -94.10% | 58.380 |

## Performance model

- Peak throughput: **61.369 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.465 | 5.6% |
| iter_seq | 6.233 | 10.2% |
| iter_strided | 58.777 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6919ns | 6919ns | base |
| iter_seq | 4856ns | 4856ns | -29.81% |
| iter_strided | 2459ns | 2459ns | -64.46% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4728ns | base | --- | [4638, 4833] | --- | --- | --- | --- |
| iter_seq | 2629ns | -2078.8ns (-44.0%) | [-2095, -2059]ns | [2598, 2684] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 279ns | -4458.8ns (-94.3%) | [-4548, -4358]ns | [274, 285] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4595ns | -45.4% | -94.1% |
| 1 | 2 | 4896ns | -44.4% | -94.4% |
| 1 | 3 | 4897ns | -47.7% | -94.4% |
| 1 | 4 | 4726ns | -45.0% | -93.9% |
| 1 | 5 | 4598ns | -41.5% | -94.1% |
| 1 | 6 | 4589ns | -44.4% | -93.7% |
| 1 | 7 | 4919ns | -45.4% | -94.5% |
| 1 | 8 | 4833ns | -44.4% | -94.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | 0.062 | ok |
| iter_seq | -0.502 | HIGH- (thermal bounce) |
| iter_strided | -0.533 | HIGH- (thermal bounce) |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 3.0ns | 4754.5ns | 0.1% |  |
| iter_seq | 3.2ns | 2646.8ns | 0.1% |  |
| iter_strided | 2.9ns | 280.6ns | 1.0% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 4582.0-4955.4 ns)
   4582.0 |########################################
   4600.7 |###########
   4619.4 |###
   4638.0 |####
   4656.7 |####
   4675.4 |
   4694.0 |##
   4712.7 |###
   4731.4 |##
   4750.1 |##
   4768.7 |##
   4787.4 |#
   4806.1 |#
   4824.7 |#####
   4843.4 |
   4862.1 |
   4880.8 |############
   4899.4 |##########################
   4918.1 |#####################
   4936.8 |#
  (15 below, 6 above range)

iter_seq (n=160, range 2508.2-2839.8 ns)
   2508.2 |#####################
   2524.8 |#
   2541.4 |##################
   2558.0 |#########
   2574.5 |#####
   2591.1 |##########
   2607.7 |#########
   2624.3 |#
   2640.9 |####
   2657.4 |##
   2674.0 |########################################
   2690.6 |################
   2707.2 |
   2723.7 |###
   2740.3 |#
   2756.9 |
   2773.5 |
   2790.0 |#
   2806.6 |
   2823.2 |
  (15 below, 20 above range)

iter_strided (n=160, range 267.0-297.6 ns)
    267.0 |##########################
    268.5 |########################
    270.0 |#####################################
    271.6 |########################################
    273.1 |########################
    274.6 |##########
    276.2 |#####
    277.7 |##########
    279.2 |########
    280.7 |########
    282.3 |##########
    283.8 |########
    285.3 |########################
    286.9 |########################################
    288.4 |##################################
    289.9 |#####################
    291.4 |#############
    293.0 |##########
    294.5 |##########
    296.0 |
  (14 below, 8 above range)

```
