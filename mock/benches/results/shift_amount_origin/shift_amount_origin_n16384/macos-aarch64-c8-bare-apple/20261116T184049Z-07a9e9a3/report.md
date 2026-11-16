# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 3.5x slower than the field

shr_per_iter (679 ns) is 3.5x the fastest (192 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (192 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.5x the fastest

Fastest shr_const (192 ns) to slowest shr_per_iter (679 ns): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### shr_loop_invariant's edge over baseline is significant but tiny (1 ns, 0.62%)

shr_loop_invariant differs from baseline shr_const by 1 ns (0.62%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (shr_const) is the fastest** at 192.1 ns median
- 2 variants significantly slower than baseline
- Spread: 3.53x (fastest 192.1 ns, slowest 678.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Shift amount: const vs loop-invariant runtime vs per-iter independent |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6a92bf08e314750d |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| shr_const | 2320ns | 2308ns | 2300ns | 2310ns | 2371ns | base |
| shr_loop_invariant | 2327ns | 2310ns | 2304ns | 2314ns | 2388ns | +0.29% |
| shr_per_iter | 2813ns | 2797ns | 2789ns | 2803ns | 2868ns | +21.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 193ns | 185ns | 201ns | base | 85.065 |
| shr_loop_invariant | 194ns | 186ns | 202ns | +0.72% | 84.459 |
| shr_per_iter | 681ns | 672ns | 698ns | +253.68% | 24.051 |

## Performance model

- Peak throughput: **88.586 Gops/s** (shr_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 85.289 | 96.3% |
| shr_loop_invariant | 84.367 | 95.2% |
| shr_per_iter | 24.140 | 27.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2320ns | 2320ns | base |
| shr_loop_invariant | 2327ns | 2327ns | +0.29% |
| shr_per_iter | 2813ns | 2813ns | +21.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 192ns | base | --- | [191, 194] | --- | --- | --- | --- |
| shr_loop_invariant | 194ns | +1.2ns (+0.6%) | [+0, +2]ns | [193, 195] | YES | 0.0004 | 0.0004 | 5 |
| shr_per_iter | 679ns | +485.8ns (+252.9%) | [+485, +487]ns | [678, 680] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 194ns | +0.3% | +247.5% |
| 1 | 2 | 192ns | +0.9% | +252.6% |
| 1 | 3 | 192ns | +0.8% | +253.4% |
| 1 | 4 | 191ns | +0.2% | +254.6% |
| 1 | 5 | 191ns | +2.0% | +255.5% |
| 1 | 6 | 190ns | +0.9% | +256.3% |
| 1 | 7 | 191ns | +5.6% | +256.2% |
| 1 | 8 | 196ns | -1.9% | +247.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.043 | ok |
| shr_loop_invariant | -0.436 | moderate- |
| shr_per_iter | 0.040 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 3.1ns | 192.6ns | 1.6% |  |
| shr_loop_invariant | 3.5ns | 194.0ns | 1.8% |  |
| shr_per_iter | 3.8ns | 681.2ns | 0.6% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 185.0-200.9 ns)
    185.0 |###############
    185.7 |######
    186.5 |###############
    187.3 |###########################
    188.1 |##############################
    188.9 |##################
    189.7 |###########################
    190.5 |#################################
    191.3 |###########################
    192.1 |###############
    192.9 |#########
    193.7 |########################################
    194.5 |########################
    195.3 |###########################
    196.1 |###############
    196.9 |##############################
    197.7 |########################
    198.5 |#####################
    199.3 |############
    200.1 |###
  (15 below, 6 above range)

shr_loop_invariant (n=160, range 186.0-201.7 ns)
    186.0 |##
    186.8 |####################
    187.6 |######################
    188.4 |########
    189.1 |##############
    189.9 |########
    190.7 |##############
    191.5 |##################################
    192.3 |####################
    193.1 |###############################
    193.8 |############################
    194.6 |########################################
    195.4 |##############
    196.2 |#################
    197.0 |#########################
    197.8 |##################################
    198.5 |####################
    199.3 |#####
    200.1 |###########
    200.9 |##
  (14 below, 14 above range)

shr_per_iter (n=160, range 671.8-697.7 ns)
    671.8 |##################
    673.1 |###########################
    674.3 |###########################
    675.6 |#####################
    676.9 |#########################
    678.2 |########################################
    679.5 |###########################
    680.8 |#################################
    682.1 |#####################
    683.4 |############
    684.7 |##########
    686.0 |########
    687.3 |##
    688.6 |##
    689.9 |##
    691.2 |####
    692.5 |
    693.8 |##
    695.1 |
    696.4 |##
  (12 below, 11 above range)

```
