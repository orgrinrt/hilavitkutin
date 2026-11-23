# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 27960% faster than the next best (fib_spectral)

fib_greedy (409 ns) leads fib_spectral (114.78 us) by 27960%, a clear separation rather than a photo finish. CV 50.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 50.1%)

fib_greedy wins on median (409 ns) yet has the highest variance (CV 50.1%), while fib_spectral is the steadiest (CV 10.2%, 114.78 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fib_spectral shows warm-up / thermal drift (autocorr +0.62)

fib_spectral's per-pass series has lag-1 autocorrelation +0.62, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (409 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 280.6x the fastest

Fastest fib_greedy (409 ns) to slowest fib_spectral (114.78 us): 280.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### fib_greedy is inconsistent: worst-20% is 2.3x its best-20%

fib_greedy's best 20% of batches run at 328 ns but its worst 20% at 741 ns (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 409.1 ns median
- 1 variant significantly slower than baseline
- Spread: 280.60x (fastest 409.1 ns, slowest 114778.1 ns)

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
| Counter frequency | 2112925964 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fib_greedy | 2578ns | 2299ns | 1872ns | 2267ns | 4217ns | base |
| fib_spectral | 123953ns | 116344ns | 112971ns | 122392ns | 139620ns | +4707.81% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 455ns | 328ns | 741ns | base | 0.141 |
| fib_spectral | 122247ns | 111396ns | 137695ns | +26768.68% | 0.001 |

## Performance model

- Peak throughput: **0.195 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.156 | 80.2% |
| fib_spectral | 0.001 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2578ns | 2578ns | base |
| fib_spectral | 123953ns | 123953ns | +4707.81% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 409ns | base | --- | [401, 413] | --- | --- | --- | --- |
| fib_spectral | 114778ns | +114103.1ns (+27894.7%) | [+113658, +114458]ns | [114088, 114902] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 416ns | +27272.6% |
| 1 | 2 | 380ns | +29217.4% |
| 1 | 3 | 422ns | +27103.5% |
| 1 | 4 | 409ns | +27165.9% |
| 1 | 5 | 437ns | +26030.1% |
| 1 | 6 | 412ns | +33094.6% |
| 1 | 7 | 326ns | +42200.8% |
| 1 | 8 | 335ns | +40879.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | 0.333 | moderate+ |
| fib_spectral | 0.619 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 531.9ns | 455.0ns | 116.9% | HIGH |
| fib_spectral | 419.6ns | 122247.1ns | 0.3% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 328.1-741.4 ns)
    328.1 |##################
    348.8 |####
    369.5 |#############
    390.1 |####################
    410.8 |########################################
    431.5 |####
    452.1 |
    472.8 |#
    493.4 |##
    514.1 |##
    534.8 |
    555.4 |
    576.1 |
    596.7 |
    617.4 |
    638.1 |
    658.7 |
    679.4 |
    700.0 |
    720.7 |
  (12 below, 14 above range)

fib_spectral (n=160, range 111395.8-137695.1 ns)
  111395.8 |######################
  112710.8 |#####################
  114025.7 |#####################################
  115340.7 |##
  116655.7 |
  117970.6 |#
  119285.6 |
  120600.6 |
  121915.5 |
  123230.5 |
  124545.5 |
  125860.4 |
  127175.4 |
  128490.4 |
  129805.3 |
  131120.3 |
  132435.3 |
  133750.2 |
  135065.2 |
  136380.1 |########################################
  (17 below, 20 above range)

```

## Diagnostics

- **fib_greedy**: CV=45.0% (high variance, measurements may be unstable)
- **fib_greedy**: bridge=115.2% of algo (FFI overhead may distort results)
- **fib_spectral**: autocorrelation=0.62 (measurement drift or warm-up artifact)
