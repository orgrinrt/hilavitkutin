# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 3.6x slower than the field

shr_per_iter (710 ns) is 3.6x the fastest (195 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (195 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 3.6x the fastest

Fastest shr_const (195 ns) to slowest shr_per_iter (710 ns): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (shr_const) is the fastest** at 195.2 ns median
- 2 variants significantly slower than baseline
- Spread: 3.64x (fastest 195.2 ns, slowest 710.0 ns)

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
| shr_const | 2380ns | 2367ns | 2253ns | 2354ns | 2584ns | base |
| shr_loop_invariant | 2454ns | 2421ns | 2316ns | 2448ns | 2608ns | +3.11% |
| shr_per_iter | 2905ns | 2917ns | 2732ns | 2897ns | 3101ns | +22.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 197ns | 184ns | 215ns | base | 82.982 |
| shr_loop_invariant | 207ns | 192ns | 225ns | +4.71% | 79.250 |
| shr_per_iter | 709ns | 666ns | 762ns | +258.98% | 23.116 |

## Performance model

- Peak throughput: **89.236 Gops/s** (shr_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 83.934 | 94.1% |
| shr_loop_invariant | 79.922 | 89.6% |
| shr_per_iter | 23.076 | 25.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2380ns | 2380ns | base |
| shr_loop_invariant | 2454ns | 2454ns | +3.11% |
| shr_per_iter | 2905ns | 2905ns | +22.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 195ns | base | --- | [193, 198] | --- | --- | --- | --- |
| shr_loop_invariant | 205ns | +10.0ns (+5.1%) | [+8, +14]ns | [203, 207] | YES | 0.0000 | 0.0000 | 2 |
| shr_per_iter | 710ns | +509.4ns (+260.9%) | [+505, +512]ns | [707, 712] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 192ns | +8.1% | +247.5% |
| 1 | 2 | 189ns | +8.4% | +266.9% |
| 1 | 3 | 209ns | -0.6% | +241.7% |
| 1 | 4 | 201ns | +9.5% | +240.1% |
| 1 | 5 | 212ns | -7.7% | +238.9% |
| 1 | 6 | 189ns | +4.2% | +264.2% |
| 1 | 7 | 201ns | +1.8% | +255.6% |
| 1 | 8 | 188ns | +11.1% | +279.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | -0.223 | moderate- |
| shr_loop_invariant | -0.045 | ok |
| shr_per_iter | -0.242 | moderate- |

**Consistency summary:**

- **shr_loop_invariant**: won 2/8, lost 6/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 3.3ns | 197.4ns | 1.7% |  |
| shr_loop_invariant | 3.7ns | 206.7ns | 1.8% |  |
| shr_per_iter | 3.6ns | 708.8ns | 0.5% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 183.6-215.3 ns)
    183.6 |##################
    185.2 |#############################
    186.8 |########################################
    188.4 |####################################
    189.9 |################################
    191.5 |########################################
    193.1 |########################################
    194.7 |#####################
    196.3 |##############
    197.9 |#############################
    199.5 |################################
    201.0 |################################
    202.6 |#########################
    204.2 |##############
    205.8 |#####################
    207.4 |#####################
    209.0 |###
    210.6 |###
    212.1 |###
    213.7 |#######
  (13 below, 18 above range)

shr_loop_invariant (n=160, range 192.2-224.9 ns)
    192.2 |########################
    193.8 |##################
    195.4 |#################################
    197.1 |########################
    198.7 |##############################
    200.3 |#########
    202.0 |####################################
    203.6 |########################################
    205.3 |##############################
    206.9 |#####################
    208.5 |##############################
    210.2 |######
    211.8 |###############
    213.4 |##################
    215.1 |#########
    216.7 |############
    218.3 |##############################
    220.0 |#########
    221.6 |######
    223.2 |###############
  (11 below, 11 above range)

shr_per_iter (n=160, range 665.6-762.4 ns)
    665.6 |##########
    670.5 |#######
    675.3 |#
    680.1 |######
    685.0 |############
    689.8 |##########
    694.7 |######
    699.5 |#####
    704.3 |###############
    709.2 |########################################
    714.0 |############################
    718.8 |######
    723.7 |
    728.5 |
    733.4 |
    738.2 |
    743.0 |
    747.9 |
    752.7 |#
    757.5 |#
  (17 below, 24 above range)

```
