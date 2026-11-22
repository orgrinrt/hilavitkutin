# Shift amount: const vs loop-invariant runtime vs per-iter independent

3 variants, 160 samples per variant.
Baseline: **shr_const**

## Highlights

Baseline for all deltas below: **shr_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### shr_per_iter is an outlier: 2.8x slower than the field

shr_per_iter (184 ns) is 2.8x the fastest (66 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### shr_loop_invariant shows alternating (throttle bounce) (autocorr -0.62)

shr_loop_invariant's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (shr_const)

The baseline shr_const is the fastest (66 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (shr_const) is the fastest** at 65.8 ns median
- 2 variants significantly slower than baseline
- Spread: 2.79x (fastest 65.8 ns, slowest 183.8 ns)

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
| shr_const | 2170ns | 2140ns | 2134ns | 2151ns | 2267ns | base |
| shr_loop_invariant | 2201ns | 2154ns | 2137ns | 2183ns | 2321ns | +1.42% |
| shr_per_iter | 2288ns | 2261ns | 2253ns | 2268ns | 2383ns | +5.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| shr_const | 66ns | 59ns | 75ns | base | 61.857 |
| shr_loop_invariant | 68ns | 61ns | 77ns | +3.14% | 59.971 |
| shr_per_iter | 185ns | 176ns | 196ns | +178.96% | 22.174 |

## Performance model

- Peak throughput: **69.354 Gops/s** (shr_const; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| shr_const | 62.249 | 89.8% |
| shr_loop_invariant | 60.324 | 87.0% |
| shr_per_iter | 22.285 | 32.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| shr_const | 2170ns | 2170ns | base |
| shr_loop_invariant | 2201ns | 2201ns | +1.42% |
| shr_per_iter | 2288ns | 2288ns | +5.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| shr_const | 66ns | base | --- | [65, 67] | --- | --- | --- | --- |
| shr_loop_invariant | 68ns | +2.5ns (+3.8%) | [+2, +3]ns | [67, 68] | YES | 0.0000 | 0.0000 | 6 |
| shr_per_iter | 184ns | +117.9ns (+179.2%) | [+117, +118]ns | [182, 184] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | shr_const | shr_loop_invariant | shr_per_iter |
|---|---|---|---|---|
| 1 | 1 | 66ns | +1.4% | +180.5% |
| 1 | 2 | 65ns | +8.3% | +181.9% |
| 1 | 3 | 66ns | +3.5% | +177.4% |
| 1 | 4 | 66ns | +1.4% | +188.5% |
| 1 | 5 | 65ns | +8.3% | +180.8% |
| 1 | 6 | 65ns | +1.2% | +179.2% |
| 1 | 7 | 66ns | +5.8% | +175.2% |
| 1 | 8 | 68ns | -5.1% | +168.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| shr_const | 0.090 | ok |
| shr_loop_invariant | -0.622 | HIGH- (thermal bounce) |
| shr_per_iter | -0.135 | ok |

**Consistency summary:**

- **shr_loop_invariant**: won 1/8, lost 7/8
- **shr_per_iter**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| shr_const | 2.4ns | 66.2ns | 3.7% |  |
| shr_loop_invariant | 2.9ns | 68.3ns | 4.2% |  |
| shr_per_iter | 2.7ns | 184.7ns | 1.5% |  |

## Distribution (algo ns)

```
shr_const (n=160, range 59.1-74.6 ns)
     59.1 |########
     59.8 |#############
     60.6 |################
     61.4 |################
     62.2 |################
     62.9 |##########
     63.7 |################################
     64.5 |################################
     65.3 |#####################################
     66.0 |##########################
     66.8 |#####################################
     67.6 |########################################
     68.4 |################
     69.2 |#####################
     69.9 |##################
     70.7 |#####
     71.5 |########
     72.3 |########
     73.0 |#####
     73.8 |##
  (15 below, 6 above range)

shr_loop_invariant (n=160, range 60.6-77.4 ns)
     60.6 |######################
     61.4 |###########
     62.3 |#########################
     63.1 |#####
     64.0 |######################
     64.8 |######################
     65.7 |###############################
     66.5 |###############################
     67.3 |########################################
     68.2 |#########################
     69.0 |###############################
     69.9 |#################
     70.7 |######################
     71.6 |#########################
     72.4 |##############
     73.2 |######################
     74.1 |#################
     74.9 |#####
     75.8 |#####
     76.6 |#####
  (12 below, 5 above range)

shr_per_iter (n=160, range 176.3-196.0 ns)
    176.3 |####
    177.3 |##################
    178.3 |##################
    179.3 |###########
    180.3 |################################
    181.3 |##################
    182.2 |##############################
    183.2 |########################################
    184.2 |#########
    185.2 |##############
    186.2 |##############################
    187.2 |#########################
    188.2 |################
    189.1 |################
    190.1 |#########
    191.1 |#######
    192.1 |####
    193.1 |####
    194.1 |##
    195.0 |
  (17 below, 8 above range)

```
