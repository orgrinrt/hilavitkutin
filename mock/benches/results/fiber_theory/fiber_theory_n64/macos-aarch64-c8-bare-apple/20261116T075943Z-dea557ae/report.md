# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 32815% faster than the next best (fib_spectral)

fib_greedy (387 ns) leads fib_spectral (127.41 us) by 32815%, a clear separation rather than a photo finish. CV 5.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 5.1%)

fib_greedy wins on median (387 ns) yet has the highest variance (CV 5.1%), while fib_spectral is the steadiest (CV 1.2%, 127.41 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fib_greedy shows alternating (throttle bounce) (autocorr -0.74)

fib_greedy's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (387 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 329.2x the fastest

Fastest fib_greedy (387 ns) to slowest fib_spectral (127.41 us): 329.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 387.1 ns median
- 1 variant significantly slower than baseline
- Spread: 329.15x (fastest 387.1 ns, slowest 127414.4 ns)

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
| fib_greedy | 2518ns | 2513ns | 2479ns | 2514ns | 2568ns | base |
| fib_spectral | 129526ns | 129630ns | 127269ns | 129585ns | 131607ns | +5044.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 390ns | 361ns | 415ns | base | 0.164 |
| fib_spectral | 127284ns | 125044ns | 129347ns | +32565.21% | 0.001 |

## Performance model

- Peak throughput: **0.177 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.165 | 93.4% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2518ns | 2518ns | base |
| fib_spectral | 129526ns | 129526ns | +5044.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 387ns | base | --- | [385, 391] | --- | --- | --- | --- |
| fib_spectral | 127414ns | +127017.8ns (+32812.6%) | [+126762, +127237]ns | [127154, 127628] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 404ns | +31431.8% |
| 1 | 2 | 356ns | +35112.4% |
| 1 | 3 | 408ns | +31282.9% |
| 1 | 4 | 372ns | +33552.4% |
| 1 | 5 | 411ns | +31200.1% |
| 1 | 6 | 386ns | +32826.4% |
| 1 | 7 | 388ns | +32909.9% |
| 1 | 8 | 380ns | +33219.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.739 | HIGH- (thermal bounce) |
| fib_spectral | -0.624 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 501.0ns | 389.7ns | 128.6% | HIGH |
| fib_spectral | 527.3ns | 127283.9ns | 0.4% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 361.5-414.9 ns)
    361.5 |##
    364.2 |
    366.8 |#####
    369.5 |########################
    372.2 |################
    374.8 |#############
    377.5 |#############
    380.2 |#############################
    382.9 |##################################
    385.5 |##################################
    388.2 |########################
    390.9 |#####
    393.5 |#####
    396.2 |
    398.9 |########
    401.6 |########################
    404.2 |########################
    406.9 |########################################
    409.6 |#####################################
    412.2 |########
  (19 below, 10 above range)

fib_spectral (n=160, range 125044.2-129346.9 ns)
  125044.2 |#################
  125259.4 |##############
  125474.5 |########
  125689.6 |###########
  125904.8 |##############
  126119.9 |##
  126335.0 |#################
  126550.2 |######################
  126765.3 |##############
  126980.4 |########################################
  127195.6 |######################
  127410.7 |##################################
  127625.8 |####################
  127841.0 |########################################
  128056.1 |############################
  128271.2 |#########################
  128486.4 |######################
  128701.5 |####################
  128916.6 |
  129131.8 |########
  (15 below, 10 above range)

```

## Diagnostics

- **fib_greedy**: bridge=128.5% of algo (FFI overhead may distort results)
