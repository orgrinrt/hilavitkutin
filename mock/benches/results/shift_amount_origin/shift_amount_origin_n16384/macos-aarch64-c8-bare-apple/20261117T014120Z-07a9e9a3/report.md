# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 3.5x slower than the field

shr_per_iter (667 ns) is 3.5x the fastest (191 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_const is fastest but the noisiest (CV 6.0%)

shr_const wins on median (191 ns) yet has the highest variance (CV 6.0%), while shr_per_iter is the steadiest (CV 3.1%, 667 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (shr_const, shr_loop_invariant) are a dead heat (<1%)

shr_const (191 ns) and shr_loop_invariant (192 ns) differ by 0.68%, inside the noise, even though the wider field spreads 250.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### shr_loop_invariant shows alternating (throttle bounce) (autocorr -0.54)

shr_loop_invariant's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (191 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.5x the fastest

Fastest shr_const (191 ns) to slowest shr_per_iter (667 ns): 3.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (shr_const) is the fastest** at 190.6 ns median
- 1 variant significantly slower than baseline
- Spread: 3.50x (fastest 190.6 ns, slowest 667.1 ns)

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
| shr_const | 2323ns | 2326ns | 2257ns | 2316ns | 2409ns | base |
| shr_loop_invariant | 2311ns | 2269ns | 2261ns | 2293ns | 2415ns | -0.52% |
| shr_per_iter | 2780ns | 2742ns | 2732ns | 2757ns | 2895ns | +19.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 193ns | 183ns | 206ns | base | 84.909 |
| shr_loop_invariant | 193ns | 184ns | 202ns | -0.19% | 85.074 |
| shr_per_iter | 673ns | 658ns | 702ns | +248.73% | 24.348 |

## Performance model

- Peak throughput: **89.411 Gops/s** (shr_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 85.960 | 96.1% |
| shr_loop_invariant | 85.378 | 95.5% |
| shr_per_iter | 24.560 | 27.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2323ns | 2323ns | base |
| shr_loop_invariant | 2311ns | 2311ns | -0.52% |
| shr_per_iter | 2780ns | 2780ns | +19.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 191ns | base | --- | [190, 193] | --- | --- | --- | --- |
| shr_loop_invariant | 192ns | no significant difference | [-0, +2]ns | [191, 193] | no | 0.1506 | 0.1506 | 3 |
| shr_per_iter | 667ns | +475.9ns (+249.7%) | [+475, +478]ns | [665, 668] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 189ns | +1.7% | +251.7% |
| 1 | 2 | 188ns | +0.3% | +261.1% |
| 1 | 3 | 190ns | +4.9% | +250.3% |
| 1 | 4 | 188ns | +0.9% | +256.6% |
| 1 | 5 | 198ns | +0.1% | +235.0% |
| 1 | 6 | 195ns | -1.3% | +253.5% |
| 1 | 7 | 199ns | -5.7% | +234.2% |
| 1 | 8 | 190ns | +1.6% | +250.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.235 | moderate+ |
| shr_loop_invariant | -0.544 | HIGH- (thermal bounce) |
| shr_per_iter | -0.457 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 2/8, lost 6/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 3.1ns | 193.0ns | 1.6% |  |
| shr_loop_invariant | 3.6ns | 192.6ns | 1.8% |  |
| shr_per_iter | 3.9ns | 672.9ns | 0.6% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 183.2-206.5 ns)
    183.2 |########
    184.4 |#####################
    185.6 |#####################
    186.7 |##############
    187.9 |#######################
    189.1 |########################################
    190.2 |#####################
    191.4 |##############
    192.5 |#########################
    193.7 |##########
    194.9 |##########
    196.0 |#####################
    197.2 |################
    198.4 |######
    199.5 |########
    200.7 |#####################
    201.8 |########
    203.0 |##
    204.2 |
    205.3 |########
  (12 below, 4 above range)

shr_loop_invariant (n=160, range 184.4-201.7 ns)
    184.4 |#########
    185.2 |######
    186.1 |#########
    187.0 |########################
    187.8 |##############################
    188.7 |#################################
    189.6 |########################
    190.4 |########################################
    191.3 |###########################
    192.2 |###########################
    193.0 |##################
    193.9 |#####################
    194.8 |########################
    195.6 |##################
    196.5 |###############
    197.4 |##################
    198.2 |#####################
    199.1 |##################
    200.0 |###############
    200.8 |#########
  (16 below, 9 above range)

shr_per_iter (n=160, range 658.3-702.0 ns)
    658.3 |##################
    660.5 |################
    662.7 |##########################
    664.9 |################################
    667.1 |########################################
    669.2 |#########
    671.4 |######
    673.6 |######
    675.8 |#########
    678.0 |####
    680.2 |######
    682.3 |######
    684.5 |####
    686.7 |######
    688.9 |####
    691.1 |#######
    693.3 |#
    695.4 |
    697.6 |
    699.8 |#
  (16 below, 9 above range)

```
