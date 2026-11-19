# W = 2, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core dominates: 22% faster than the next best (grouped)

per_core (134.79 us) leads grouped (164.57 us) by 22%, a clear separation rather than a photo finish. CV 11.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential is an outlier: 2.4x slower than the field

sequential (321.31 us) is 2.4x the fastest (134.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### per_core shows alternating (throttle bounce) (autocorr -0.66)

per_core's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 134786.0 ns median (-18.1% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.38x (fastest 134786.0 ns, slowest 321311.8 ns)

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
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 175244ns | 165024ns | 155086ns | 163379ns | 203121ns | base |
| per_core | 143438ns | 135366ns | 126423ns | 136050ns | 163026ns | -18.15% |
| sequential | 313794ns | 323735ns | 268270ns | 311086ns | 340617ns | +79.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 174632ns | 154335ns | 202492ns | base | 0.375 |
| per_core | 142772ns | 125884ns | 162420ns | -18.24% | 0.459 |
| sequential | 312325ns | 267515ns | 339018ns | +78.85% | 0.210 |

## Performance model

- Peak throughput: **0.521 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.398 | 76.5% |
| per_core | 0.486 | 93.4% |
| sequential | 0.204 | 39.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 175244ns | 175244ns | base |
| per_core | 143438ns | 143438ns | -18.15% |
| sequential | 313794ns | 313794ns | +79.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 164572ns | base | --- | [157540, 180027] | --- | --- | --- | --- |
| per_core | 134786ns | -27472.5ns (-16.7%) | [-43006, -11278]ns | [129112, 159131] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 321312ns | +143462.2ns (+87.2%) | [+111510, +162069]ns | [300649, 333338] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 181011ns | -26.3% | +77.5% |
| 1 | 2 | 167292ns | -3.9% | +61.9% |
| 1 | 3 | 157225ns | -18.0% | +115.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.005 | ok |
| per_core | -0.656 | HIGH- (thermal bounce) |
| sequential | -0.627 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 172805.5ns | 174632.5ns | 99.0% | HIGH |
| per_core | 143429.9ns | 142771.5ns | 100.5% | HIGH |
| sequential | 314791.3ns | 312324.9ns | 100.8% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 154334.5-202492.4 ns)
  154334.5 |########################################
  156742.4 |########################################
  159150.3 |####################
  161558.2 |####################
  163966.1 |########################################
  166374.0 |
  168781.9 |####################
  171189.8 |
  173597.7 |########################################
  176005.6 |
  178413.5 |
  180821.4 |
  183229.3 |####################
  185637.2 |####################
  188045.1 |
  190453.0 |
  192860.9 |
  195268.8 |
  197676.7 |####################
  200084.6 |
  (2 below, 2 above range)

per_core (n=18, range 125883.9-162420.1 ns)
  125883.9 |
  127710.7 |########################################
  129537.5 |################
  131364.3 |
  133191.1 |########
  135017.9 |########
  136844.8 |
  138671.6 |
  140498.4 |
  142325.2 |########
  144152.0 |
  145978.8 |
  147805.6 |
  149632.4 |
  151459.2 |
  153286.0 |
  155112.8 |
  156939.6 |################
  158766.5 |########
  160593.3 |################
  (1 below, 2 above range)

sequential (n=18, range 267514.6-339018.4 ns)
  267514.6 |##########################
  271089.8 |#############
  274665.0 |
  278240.2 |
  281815.4 |
  285390.6 |
  288965.8 |
  292541.0 |
  296116.2 |#############
  299691.4 |
  303266.5 |##########################
  306841.7 |#############
  310416.9 |
  313992.1 |#############
  317567.3 |
  321142.5 |
  324717.7 |########################################
  328292.9 |
  331868.1 |#############
  335443.3 |########################################
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=101.9% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.9% of algo (FFI overhead may distort results)
