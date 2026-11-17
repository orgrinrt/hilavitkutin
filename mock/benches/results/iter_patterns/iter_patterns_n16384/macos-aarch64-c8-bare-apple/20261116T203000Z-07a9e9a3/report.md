# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (4.59 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 269 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 832% faster than the next best (iter_seq)

iter_strided (269 ns) leads iter_seq (2.51 us) by 832%, a clear separation rather than a photo finish. CV 1.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 94% (significant)

iter_strided is -4.32 us (94%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 17.0x slower than the field

iter_gather (4.59 us) is 17.0x the fastest (269 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_gather shows alternating (throttle bounce) (autocorr -0.72)

iter_gather's per-pass series has lag-1 autocorrelation -0.72, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 17.0x the fastest

Fastest iter_strided (269 ns) to slowest iter_gather (4.59 us): 17.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: iter_strided** at 269.4 ns median (-94.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 17.05x (fastest 269.4 ns, slowest 4592.9 ns)

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
| iter_gather | 6702ns | 6686ns | 6672ns | 6687ns | 6776ns | base |
| iter_seq | 4628ns | 4609ns | 4603ns | 4612ns | 4701ns | -30.94% |
| iter_strided | 2370ns | 2368ns | 2362ns | 2368ns | 2387ns | -64.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 4605ns | 4581ns | 4662ns | base | 3.558 |
| iter_seq | 2518ns | 2504ns | 2555ns | -45.32% | 6.507 |
| iter_strided | 269ns | 265ns | 273ns | -94.15% | 60.814 |

## Performance model

- Peak throughput: **61.759 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.567 | 5.8% |
| iter_seq | 6.529 | 10.6% |
| iter_strided | 60.817 | 98.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 6702ns | 6702ns | base |
| iter_seq | 4628ns | 4628ns | -30.94% |
| iter_strided | 2370ns | 2370ns | -64.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 4593ns | base | --- | [4591, 4595] | --- | --- | --- | --- |
| iter_seq | 2510ns | -2082.5ns (-45.3%) | [-2085, -2081]ns | [2509, 2511] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 269ns | -4323.4ns (-94.1%) | [-4325, -4322]ns | [269, 270] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 4593ns | -45.4% | -94.1% |
| 1 | 2 | 4590ns | -45.3% | -94.1% |
| 1 | 3 | 4595ns | -45.4% | -94.1% |
| 1 | 4 | 4590ns | -44.8% | -94.1% |
| 1 | 5 | 4597ns | -45.5% | -94.2% |
| 1 | 6 | 4592ns | -45.3% | -94.1% |
| 1 | 7 | 4593ns | -45.4% | -94.1% |
| 1 | 8 | 4592ns | -45.3% | -94.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.716 | HIGH- (thermal bounce) |
| iter_seq | -0.236 | moderate- |
| iter_strided | -0.320 | moderate- |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 3.2ns | 4604.8ns | 0.1% |  |
| iter_seq | 2.8ns | 2517.9ns | 0.1% |  |
| iter_strided | 3.1ns | 269.4ns | 1.1% |  |

## Distribution (algo ns)

```
iter_gather (n=160, range 4580.9-4661.7 ns)
   4580.9 |###################
   4584.9 |#############################
   4589.0 |########################################
   4593.0 |#####################
   4597.1 |##################
   4601.1 |#############
   4605.1 |#######
   4609.2 |########
   4613.2 |######
   4617.3 |#
   4621.3 |##
   4625.4 |
   4629.4 |##
   4633.4 |
   4637.5 |
   4641.5 |
   4645.6 |
   4649.6 |
   4653.7 |
   4657.7 |
  (11 below, 9 above range)

iter_seq (n=160, range 2504.4-2554.9 ns)
   2504.4 |########################################
   2506.9 |########################################
   2509.5 |####################################
   2512.0 |################
   2514.5 |##
   2517.0 |
   2519.6 |###
   2522.1 |###
   2524.6 |##
   2527.1 |###
   2529.7 |##
   2532.2 |##
   2534.7 |#
   2537.2 |###
   2539.8 |
   2542.3 |
   2544.8 |
   2547.3 |#
   2549.9 |##
   2552.4 |###
  (15 below, 12 above range)

iter_strided (n=160, range 265.3-273.4 ns)
    265.3 |######
    265.7 |
    266.1 |###############
    266.5 |###########################
    266.9 |##################
    267.3 |##############################
    267.7 |###############
    268.1 |##############################
    268.5 |##################
    268.9 |########################################
    269.3 |###############
    269.7 |#####################
    270.1 |#################################
    270.5 |##############################
    270.9 |###########################
    271.3 |###########################
    271.7 |########################
    272.1 |###############
    272.5 |#########
    272.9 |#########
  (14 below, 10 above range)

```
