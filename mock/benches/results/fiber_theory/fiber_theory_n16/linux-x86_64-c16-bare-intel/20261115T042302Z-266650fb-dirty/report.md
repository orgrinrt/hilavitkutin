# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 75982% faster than the next best (fib_spectral)

fib_greedy (95 ns) leads fib_spectral (72.54 us) by 75982%, a clear separation rather than a photo finish. CV 83.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 83.3%)

fib_greedy wins on median (95 ns) yet has the highest variance (CV 83.3%), while fib_spectral is the steadiest (CV 1.4%, 72.54 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fib_spectral shows alternating (throttle bounce) (autocorr -0.66)

fib_spectral's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (95 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 760.8x the fastest

Fastest fib_greedy (95 ns) to slowest fib_spectral (72.54 us): 760.8x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.5x its best-20%

fib_greedy's best 20% of batches run at 79 ns but its worst 20% at 279 ns (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 95.3 ns median
- 1 variant significantly slower than baseline
- Spread: 760.82x (fastest 95.3 ns, slowest 72544.6 ns)

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
| Counter frequency | 2112788883 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2633ns | 1973ns | 1618ns | 1939ns | 5733ns | base |
| fib_spectral | 74428ns | 74462ns | 73182ns | 74359ns | 75884ns | +2726.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 128ns | 79ns | 279ns | base | 0.125 |
| fib_spectral | 72507ns | 71263ns | 73961ns | +56478.62% | 0.000 |

## Performance model

- Peak throughput: **0.203 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 16

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.168 | 82.5% |
| fib_spectral | 0.000 | 0.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2633ns | 2633ns | base |
| fib_spectral | 74428ns | 74428ns | +2726.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 95ns | base | --- | [94, 96] | --- | --- | --- | --- |
| fib_spectral | 72545ns | +72427.4ns (+75959.5%) | [+72329, +72471]ns | [72489, 72591] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 98ns | +73831.6% |
| 1 | 2 | 78ns | +90606.8% |
| 1 | 3 | 96ns | +76285.0% |
| 1 | 4 | 94ns | +76571.5% |
| 1 | 5 | 80ns | +92811.2% |
| 1 | 6 | 96ns | +74686.3% |
| 1 | 7 | 96ns | +75613.2% |
| 1 | 8 | 85ns | +85163.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.519 | HIGH- (thermal bounce) |
| fib_spectral | -0.658 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 154.6ns | 128.2ns | 120.6% | HIGH |
| fib_spectral | 119.8ns | 72506.6ns | 0.2% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 78.7-279.0 ns)
     78.7 |##############
     88.7 |########################################
     98.7 |###
    108.7 |
    118.7 |
    128.7 |
    138.8 |
    148.8 |
    158.8 |
    168.8 |
    178.8 |
    188.8 |
    198.8 |
    208.9 |
    218.9 |
    228.9 |
    238.9 |
    248.9 |###
    258.9 |#
    269.0 |
  (17 below, 16 above range)

fib_spectral (n=160, range 71263.4-73961.0 ns)
  71263.4 |##
  71398.3 |#####
  71533.2 |###############
  71668.1 |###############
  71802.9 |##########
  71937.8 |###
  72072.7 |
  72207.6 |##
  72342.5 |######
  72477.3 |########################################
  72612.2 |###############
  72747.1 |###
  72882.0 |#####
  73016.9 |##########
  73151.7 |##############
  73286.6 |##
  73421.5 |#
  73556.4 |#
  73691.3 |
  73826.1 |
  (18 below, 21 above range)

```

## Diagnostics

- **fib_greedy**: CV=62.0% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **fib_greedy**: bridge=117.9% of algo (FFI overhead may distort results)
