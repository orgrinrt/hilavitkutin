# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 45422% faster than the next best (fib_spectral)

fib_greedy (191 ns) leads fib_spectral (86.81 us) by 45422%, a clear separation rather than a photo finish. CV 76.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 76.5%)

fib_greedy wins on median (191 ns) yet has the highest variance (CV 76.5%), while fib_spectral is the steadiest (CV 17.7%, 86.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (191 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 455.2x the fastest

Fastest fib_greedy (191 ns) to slowest fib_spectral (86.81 us): 455.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 3.1x its best-20%

fib_greedy's best 20% of batches run at 157 ns but its worst 20% at 483 ns (3.1x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 190.7 ns median
- 1 variant significantly slower than baseline
- Spread: 455.22x (fastest 190.7 ns, slowest 86810.1 ns)

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
| fib_greedy | 2633ns | 2072ns | 1697ns | 2072ns | 5254ns | base |
| fib_spectral | 81630ns | 88557ns | 56762ns | 85375ns | 95262ns | +3000.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 242ns | 157ns | 483ns | base | 0.132 |
| fib_spectral | 79836ns | 55056ns | 93334ns | +32880.09% | 0.000 |

## Performance model

- Peak throughput: **0.204 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 32

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.168 | 82.1% |
| fib_spectral | 0.000 | 0.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2633ns | 2633ns | base |
| fib_spectral | 81630ns | 81630ns | +3000.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 191ns | base | --- | [188, 193] | --- | --- | --- | --- |
| fib_spectral | 86810ns | +86389.2ns (+45301.1%) | [+76492, +92708]ns | [76723, 92908] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 152ns | +50076.1% |
| 1 | 2 | 181ns | +34123.8% |
| 1 | 3 | 192ns | +39824.5% |
| 1 | 4 | 161ns | +57732.1% |
| 1 | 5 | 193ns | +48385.9% |
| 1 | 6 | 193ns | +26340.5% |
| 1 | 7 | 198ns | +46929.0% |
| 1 | 8 | 186ns | +49914.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.039 | ok |
| fib_spectral | -0.181 | ok |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 297.0ns | 242.1ns | 122.7% | HIGH |
| fib_spectral | 214.9ns | 79836.4ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 156.6-483.3 ns)
    156.6 |############
    173.0 |##############################
    189.3 |########################################
    205.6 |##
    222.0 |
    238.3 |
    254.6 |
    271.0 |
    287.3 |
    303.6 |
    320.0 |
    336.3 |
    352.6 |
    369.0 |
    385.3 |
    401.6 |
    418.0 |
    434.3 |
    450.6 |
    467.0 |##
  (15 below, 17 above range)

fib_spectral (n=160, range 55056.4-93334.1 ns)
  55056.4 |
  56970.3 |
  58884.1 |
  60798.0 |############
  62711.9 |
  64625.8 |
  66539.7 |
  68453.6 |
  70367.5 |
  72281.4 |
  74195.2 |
  76109.1 |#######################
  78023.0 |
  79936.9 |
  81850.8 |
  83764.7 |
  85678.6 |
  87592.5 |
  89506.3 |
  91420.2 |########################################
  (20 below, 16 above range)

```

## Diagnostics

- **fib_greedy**: CV=60.3% (high variance, measurements may be unstable)
- **fib_greedy**: worst_20/best_20 = 3.1x (possible bimodal distribution)
- **fib_greedy**: bridge=121.6% of algo (FFI overhead may distort results)
