# fiber grouping theories: greedy topo walk vs spectral bisection

2 variants, 160 samples per variant.
Baseline: **fib_greedy**

## Highlights

Baseline for all deltas below: **fib_greedy**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fib_greedy dominates: 32563% faster than the next best (fib_spectral)

fib_greedy (392 ns) leads fib_spectral (128.20 us) by 32563%, a clear separation rather than a photo finish. CV 6.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fib_greedy is fastest but the noisiest (CV 6.4%)

fib_greedy wins on median (392 ns) yet has the highest variance (CV 6.4%), while fib_spectral is the steadiest (CV 1.2%, 128.20 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fib_spectral shows alternating (throttle bounce) (autocorr -0.54)

fib_spectral's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (fib_greedy)

The baseline fib_greedy is the fastest (392 ns median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 326.6x the fastest

Fastest fib_greedy (392 ns) to slowest fib_spectral (128.20 us): 326.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (fib_greedy) is the fastest** at 392.5 ns median
- 1 variant significantly slower than baseline
- Spread: 326.63x (fastest 392.5 ns, slowest 128203.6 ns)

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
| fib_greedy | 2557ns | 2540ns | 2443ns | 2543ns | 2713ns | base |
| fib_spectral | 130566ns | 130434ns | 128643ns | 130557ns | 132515ns | +5006.97% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fib_greedy | 397ns | 372ns | 438ns | base | 0.161 |
| fib_spectral | 128336ns | 126428ns | 130272ns | +32243.04% | 0.000 |

## Performance model

- Peak throughput: **0.172 Gops/s** (fib_greedy; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fib_greedy | 0.163 | 94.7% |
| fib_spectral | 0.000 | 0.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fib_greedy | 2557ns | 2557ns | base |
| fib_spectral | 130566ns | 130566ns | +5006.97% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fib_greedy | 392ns | base | --- | [388, 397] | --- | --- | --- | --- |
| fib_spectral | 128204ns | +127801.6ns (+32560.9%) | [+127703, +128002]ns | [128091, 128374] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fib_greedy | fib_spectral |
|---|---|---|---|
| 1 | 1 | 409ns | +31224.0% |
| 1 | 2 | 375ns | +33628.3% |
| 1 | 3 | 401ns | +32091.7% |
| 1 | 4 | 380ns | +33186.2% |
| 1 | 5 | 436ns | +29558.9% |
| 1 | 6 | 392ns | +32564.8% |
| 1 | 7 | 380ns | +33926.5% |
| 1 | 8 | 372ns | +34245.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fib_greedy | -0.262 | moderate- |
| fib_spectral | -0.536 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fib_spectral**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fib_greedy | 510.9ns | 396.8ns | 128.8% | HIGH |
| fib_spectral | 508.7ns | 128335.6ns | 0.4% |  |

## Distribution (algo ns)

```
fib_greedy (n=160, range 371.8-438.3 ns)
    371.8 |######################################
    375.2 |########################################
    378.5 |################
    381.8 |#
    385.1 |#######
    388.4 |############
    391.8 |###########################
    395.1 |#######
    398.4 |#####################
    401.7 |##################
    405.1 |##########
    408.4 |##########
    411.7 |#########
    415.0 |#
    418.3 |
    421.7 |
    425.0 |
    428.3 |###
    431.6 |############
    435.0 |##########
  (12 below, 10 above range)

fib_spectral (n=160, range 126428.1-130271.8 ns)
  126428.1 |######################
  126620.3 |########
  126812.4 |###
  127004.6 |
  127196.8 |
  127389.0 |
  127581.2 |#
  127773.4 |###################
  127965.6 |########################################
  128157.8 |#########################
  128349.9 |###
  128542.1 |#########
  128734.3 |#########
  128926.5 |#################
  129118.7 |#################
  129310.9 |#################
  129503.1 |######
  129695.3 |######
  129887.5 |####
  130079.6 |###
  (17 below, 8 above range)

```

## Diagnostics

- **fib_greedy**: bridge=129.4% of algo (FFI overhead may distort results)
