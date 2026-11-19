# W = 3, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 12% faster than the next best (grouped)

per_core (111.92 us) leads grouped (125.62 us) by 12%, a clear separation rather than a photo finish. CV 14.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.8x slower than the field

sequential (315.19 us) is 2.8x the fastest (111.92 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 111922.2 ns median (-10.9% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.82x (fastest 111922.2 ns, slowest 315185.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 3, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 131231ns | 126335ns | 110552ns | 123926ns | 152529ns | base |
| per_core | 111688ns | 112709ns | 86329ns | 108742ns | 128786ns | -14.89% |
| sequential | 317657ns | 316323ns | 286869ns | 309852ns | 344760ns | +142.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 130693ns | 110149ns | 151963ns | base | 0.501 |
| per_core | 110996ns | 85933ns | 127739ns | -15.07% | 0.590 |
| sequential | 316622ns | 286187ns | 343633ns | +142.26% | 0.207 |

## Performance model

- Peak throughput: **0.763 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.522 | 68.4% |
| per_core | 0.586 | 76.8% |
| sequential | 0.208 | 27.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 131231ns | 131231ns | base |
| per_core | 111688ns | 111688ns | -14.89% |
| sequential | 317657ns | 317657ns | +142.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 125624ns | base | --- | [117755, 138853] | --- | --- | --- | --- |
| per_core | 111922ns | -16059.6ns (-12.8%) | [-26921, -9916]ns | [103064, 116127] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 315186ns | +183544.7ns (+146.1%) | [+176581, +194241]ns | [299070, 323015] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 118010ns | -5.5% | +169.6% |
| 1 | 2 | 138853ns | -17.4% | +133.3% |
| 1 | 3 | 119816ns | -23.8% | +145.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.663 | HIGH- (thermal bounce) |
| per_core | -0.243 | moderate- |
| sequential | -0.279 | moderate- |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 130998.1ns | 130693.1ns | 100.2% | HIGH |
| per_core | 119612.4ns | 110995.6ns | 107.8% | HIGH |
| sequential | 315801.2ns | 316622.0ns | 99.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 110148.7-151963.1 ns)
  110148.7 |
  112239.4 |
  114330.1 |##########################
  116420.9 |########################################
  118511.6 |#############
  120602.3 |#############
  122693.0 |#############
  124783.7 |
  126874.5 |#############
  128965.2 |
  131055.9 |#############
  133146.6 |#############
  135237.4 |
  137328.1 |##########################
  139418.8 |#############
  141509.5 |
  143600.2 |
  145691.0 |#############
  147781.7 |
  149872.4 |
  (1 below, 2 above range)

per_core (n=18, range 85932.9-127739.1 ns)
  85932.9 |##########
  88023.2 |
  90113.5 |
  92203.9 |
  94294.2 |##########
  96384.5 |
  98474.8 |
  100565.1 |##########
  102655.4 |##########
  104745.7 |
  106836.0 |
  108926.3 |##########
  111016.6 |########################################
  113106.9 |####################
  115197.2 |##########
  117287.5 |
  119377.9 |
  121468.2 |##########
  123558.5 |
  125648.8 |
  (2 below, 3 above range)

sequential (n=18, range 286186.7-343632.9 ns)
  286186.7 |
  289059.0 |
  291931.3 |########################################
  294803.6 |
  297675.9 |
  300548.2 |
  303420.6 |##########
  306292.9 |##########
  309165.2 |##########
  312037.5 |##########
  314909.8 |####################
  317782.1 |##########
  320654.4 |##########
  323526.7 |####################
  326399.0 |
  329271.3 |
  332143.7 |
  335016.0 |
  337888.3 |
  340760.6 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=103.7% of algo (FFI overhead may distort results)
- **per_core**: bridge=102.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.2% of algo (FFI overhead may distort results)
