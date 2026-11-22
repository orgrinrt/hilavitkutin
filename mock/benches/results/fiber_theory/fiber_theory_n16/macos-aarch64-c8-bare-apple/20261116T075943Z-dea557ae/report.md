# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 68754% faster than the next best (fib_spectral)

fib_greedy (95 ns) leads fib_spectral (65.69 us) by 68754%, a clear separation rather than a photo finish. CV 8.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 8.3%)

fib_greedy wins on median (95 ns) yet has the highest variance (CV 8.3%), while fib_spectral is the steadiest (CV 2.3%, 65.69 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (95 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 688.5x the fastest

Fastest fib_greedy (95 ns) to slowest fib_spectral (65.69 us): 688.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 95.4 ns median
- 1 variant significantly slower than baseline
- Spread: 688.54x (fastest 95.4 ns, slowest 65686.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fiber grouping theories: greedy topo walk vs spectral bisection |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3f1be00000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2228ns | 2209ns | 2186ns | 2214ns | 2311ns | base |
| fib_spectral | 68004ns | 67852ns | 66349ns | 67848ns | 70127ns | +2952.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 97ns | 88ns | 110ns | base | 0.164 |
| fib_spectral | 65830ns | 64205ns | 67919ns | +67546.88% | 0.000 |

## Performance model

- Peak throughput: **0.182 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.168 | 92.4% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2228ns | 2228ns | base |
| fib_spectral | 68004ns | 68004ns | +2952.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 95ns | base | --- | [94, 97] | --- | --- | --- | --- |
| fib_spectral | 65686ns | +65580.4ns (+68742.6%) | [+65318, +65743]ns | [65431, 65845] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 105ns | +62510.5% |
| 1 | 2 | 90ns | +71425.7% |
| 1 | 3 | 90ns | +71614.5% |
| 1 | 4 | 89ns | +73719.2% |
| 1 | 5 | 98ns | +68194.8% |
| 1 | 6 | 96ns | +67559.8% |
| 1 | 7 | 95ns | +68434.4% |
| 1 | 8 | 112ns | +59731.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | 0.012 | ok |
| fib_spectral | -0.079 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 128.4ns | 97.3ns | 132.0% | HIGH |
| fib_spectral | 141.8ns | 65830.1ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 88.1-109.7 ns)
     88.1 |#########################
     89.2 |##################################
     90.3 |########################################
     91.3 |#################
     92.4 |############################
     93.5 |##############
     94.6 |##################################
     95.7 |#########################
     96.7 |########
     97.8 |#########################
     98.9 |########
    100.0 |###########
    101.0 |########
    102.1 |##
    103.2 |######################
    104.3 |########
    105.4 |#########################
    106.4 |##############
    107.5 |##
    108.6 |##############
  (14 below, 15 above range)

fib_spectral (n=160, range 64205.1-67918.7 ns)
  64205.1 |###############
  64390.7 |#############
  64576.4 |########################################
  64762.1 |#################
  64947.8 |########################
  65133.5 |###########
  65319.2 |########
  65504.8 |##########################
  65690.5 |############################
  65876.2 |#################
  66061.9 |###############
  66247.6 |####
  66433.2 |####
  66618.9 |########################
  66804.6 |##########################
  66990.3 |#################
  67176.0 |######
  67361.6 |
  67547.3 |########
  67733.0 |
  (10 below, 9 above range)

```

## Diagnostics

- **fib_greedy**: bridge=134.1% of algo (FFI overhead may distort results)
