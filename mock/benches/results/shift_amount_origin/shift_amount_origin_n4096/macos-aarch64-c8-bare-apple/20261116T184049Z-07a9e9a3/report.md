# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 2.8x slower than the field

shr_per_iter (186 ns) is 2.8x the fastest (67 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (67 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### shr_loop_invariant's edge over baseline is significant but tiny (1 ns, 1.79%)

shr_loop_invariant differs from baseline shr_const by 1 ns (1.79%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Baseline (shr_const) is the fastest** at 67.1 ns median
- 2 variants significantly slower than baseline
- Spread: 2.78x (fastest 67.1 ns, slowest 186.4 ns)

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
| shr_const | 2212ns | 2190ns | 2177ns | 2197ns | 2289ns | base |
| shr_loop_invariant | 2196ns | 2185ns | 2178ns | 2186ns | 2243ns | -0.71% |
| shr_per_iter | 2314ns | 2304ns | 2295ns | 2306ns | 2354ns | +4.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 67ns | 60ns | 73ns | base | 61.281 |
| shr_loop_invariant | 69ns | 62ns | 79ns | +3.22% | 59.371 |
| shr_per_iter | 187ns | 179ns | 194ns | +179.10% | 21.956 |

## Performance model

- Peak throughput: **68.185 Gops/s** (shr_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 61.043 | 89.5% |
| shr_loop_invariant | 60.324 | 88.5% |
| shr_per_iter | 21.968 | 32.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2212ns | 2212ns | base |
| shr_loop_invariant | 2196ns | 2196ns | -0.71% |
| shr_per_iter | 2314ns | 2314ns | +4.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 67ns | base | --- | [66, 68] | --- | --- | --- | --- |
| shr_loop_invariant | 68ns | +1.2ns (+1.8%) | [+0, +2]ns | [67, 69] | YES | 0.0000 | 0.0000 | 7 |
| shr_per_iter | 186ns | +118.8ns (+177.0%) | [+118, +119]ns | [186, 187] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 67ns | +5.5% | +177.0% |
| 1 | 2 | 67ns | +0.6% | +177.3% |
| 1 | 3 | 68ns | -0.3% | +177.0% |
| 1 | 4 | 67ns | +4.4% | +176.1% |
| 1 | 5 | 68ns | -4.0% | +175.6% |
| 1 | 6 | 67ns | +0.0% | +177.7% |
| 1 | 7 | 66ns | +2.3% | +183.2% |
| 1 | 8 | 67ns | +5.9% | +177.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.204 | moderate+ |
| shr_loop_invariant | -0.204 | moderate- |
| shr_per_iter | -0.235 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 2/8, lost 5/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 2.5ns | 66.8ns | 3.7% |  |
| shr_loop_invariant | 2.8ns | 69.0ns | 4.1% |  |
| shr_per_iter | 2.8ns | 186.6ns | 1.5% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 60.1-73.0 ns)
     60.1 |
     60.7 |##################
     61.4 |###
     62.0 |###############
     62.7 |####################################
     63.3 |#########
     63.9 |###
     64.6 |##############################
     65.2 |#################################
     65.9 |###############
     66.5 |########################################
     67.2 |#####################
     67.8 |####################################
     68.5 |##################
     69.1 |########################################
     69.8 |###############
     70.4 |############
     71.0 |############
     71.7 |#####################
     72.3 |##################
  (15 below, 14 above range)

shr_loop_invariant (n=160, range 61.6-78.7 ns)
     61.6 |######
     62.5 |####################################
     63.3 |##################
     64.2 |########################################
     65.0 |###############
     65.9 |####################################
     66.7 |########################################
     67.6 |####################################
     68.4 |###############
     69.3 |#################################
     70.1 |###########################
     71.0 |#################################
     71.8 |#################################
     72.7 |###############
     73.5 |##################
     74.4 |#####################
     75.2 |###
     76.1 |######
     77.0 |###
     77.8 |###
  (12 below, 3 above range)

shr_per_iter (n=160, range 179.4-194.1 ns)
    179.4 |##############
    180.2 |###########
    180.9 |###########
    181.6 |########
    182.4 |####################
    183.1 |#################
    183.8 |#########################
    184.6 |#########################
    185.3 |##################################
    186.0 |########################################
    186.8 |###############################
    187.5 |####################
    188.2 |#####################################
    189.0 |#########################
    189.7 |############################
    190.4 |##
    191.2 |#########################
    191.9 |#################
    192.6 |##
    193.4 |#####
  (14 below, 4 above range)

```
