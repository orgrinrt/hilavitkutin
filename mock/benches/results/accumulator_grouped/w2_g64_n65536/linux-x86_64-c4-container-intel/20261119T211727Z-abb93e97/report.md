# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 18% faster than the next best (grouped)

per_core (170.62 us) leads grouped (202.15 us) by 18%, a clear separation rather than a photo finish. CV 10.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.3x slower than the field

sequential (390.13 us) is 2.3x the fastest (170.62 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential shows alternating (throttle bounce) (autocorr -0.66)

sequential's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 170616.0 ns median (-15.6% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.29x (fastest 170616.0 ns, slowest 390131.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 2, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 201512ns | 202631ns | 159381ns | 191935ns | 236943ns | base |
| per_core | 173042ns | 171393ns | 151006ns | 168281ns | 191202ns | -14.13% |
| sequential | 393775ns | 391188ns | 310744ns | 361962ns | 483008ns | +95.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 200547ns | 158203ns | 235475ns | base | 0.327 |
| per_core | 172281ns | 150505ns | 190239ns | -14.09% | 0.380 |
| sequential | 384765ns | 309434ns | 458699ns | +91.86% | 0.170 |

## Performance model

- Peak throughput: **0.435 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.324 | 74.5% |
| per_core | 0.384 | 88.2% |
| sequential | 0.168 | 38.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 201512ns | 201512ns | base |
| per_core | 173042ns | 173042ns | -14.13% |
| sequential | 393775ns | 393775ns | +95.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 202152ns | base | --- | [175556, 211500] | --- | --- | --- | --- |
| per_core | 170616ns | -28374.2ns (-14.0%) | [-54894, -6372]ns | [162722, 177968] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 390132ns | +194734.9ns (+96.3%) | [+103829, +210307]ns | [315995, 402812] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 202152ns | -14.3% | +99.2% |
| 1 | 2 | 233128ns | -28.5% | +34.6% |
| 1 | 3 | 160996ns | +3.2% | +144.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.451 | moderate- |
| per_core | -0.125 | ok |
| sequential | -0.661 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 205255.4ns | 200546.6ns | 102.3% | HIGH |
| per_core | 174522.5ns | 172280.9ns | 101.3% | HIGH |
| sequential | 388790.4ns | 384765.3ns | 101.0% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 158202.9-235475.0 ns)
  158202.9 |##########################
  162066.5 |
  165930.1 |
  169793.7 |#############
  173657.3 |
  177521.0 |#############
  181384.6 |
  185248.2 |
  189111.8 |
  192975.4 |
  196839.0 |########################################
  200702.6 |#############
  204566.2 |##########################
  208429.8 |#############
  212293.4 |#############
  216157.0 |
  220020.6 |
  223884.2 |
  227747.8 |
  231611.4 |##########################
  (2 below, 2 above range)

per_core (n=18, range 150505.0-190238.6 ns)
  150505.0 |
  152491.6 |
  154478.3 |#############
  156465.0 |##########################
  158451.7 |#############
  160438.4 |
  162425.1 |#############
  164411.7 |
  166398.4 |########################################
  168385.1 |
  170371.8 |
  172358.5 |##########################
  174345.2 |##########################
  176331.8 |
  178318.5 |#############
  180305.2 |#############
  182291.9 |
  184278.6 |#############
  186265.3 |
  188251.9 |
  (1 below, 2 above range)

sequential (n=18, range 309433.5-458698.6 ns)
  309433.5 |########################################
  316896.8 |
  324360.0 |
  331823.3 |
  339286.5 |
  346749.8 |##########
  354213.0 |
  361676.3 |
  369139.5 |##########
  376602.8 |
  384066.1 |##########
  391529.3 |####################
  398992.6 |########################################
  406455.8 |
  413919.1 |####################
  421382.3 |
  428845.6 |
  436308.8 |
  443772.1 |
  451235.3 |
  (2 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=102.1% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.8% of algo (FFI overhead may distort results)
- **sequential**: CV=22.7% (high variance, measurements may be unstable)
- **sequential**: bridge=99.0% of algo (FFI overhead may distort results)
