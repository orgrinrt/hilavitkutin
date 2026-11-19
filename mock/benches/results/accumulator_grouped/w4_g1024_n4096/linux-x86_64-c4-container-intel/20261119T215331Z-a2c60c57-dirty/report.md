# W = 4, G = 1024

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 50% faster than the next best (grouped)

sequential (27.31 us) leads grouped (40.96 us) by 50%, a clear separation rather than a photo finish. CV 21.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 32% (significant)

sequential is -13.21 us (32%) faster than baseline grouped, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### grouped shows alternating (throttle bounce) (autocorr -0.59)

grouped's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### sequential is inconsistent: worst-20% is 1.6x its best-20%

sequential's best 20% of batches run at 23.95 us but its worst 20% at 37.16 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 27308.5 ns median (-33.3% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.77x (fastest 27308.5 ns, slowest 48334.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 1024 |
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
| grouped | 45506ns | 42059ns | 37848ns | 41015ns | 56071ns | base |
| per_core | 48109ns | 48799ns | 42125ns | 46818ns | 53039ns | +5.72% |
| sequential | 30621ns | 27830ns | 24539ns | 27379ns | 38524ns | -32.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 44890ns | 37438ns | 55519ns | base | 0.091 |
| per_core | 47526ns | 41443ns | 52446ns | +5.87% | 0.086 |
| sequential | 29784ns | 23950ns | 37156ns | -33.65% | 0.138 |

## Performance model

- Peak throughput: **0.171 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.100 | 58.5% |
| per_core | 0.085 | 49.5% |
| sequential | 0.150 | 87.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 45506ns | 45506ns | base |
| per_core | 48109ns | 48109ns | +5.72% |
| sequential | 30621ns | 30621ns | -32.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 40964ns | base | --- | [38811, 44346] | --- | --- | --- | --- |
| per_core | 48334ns | +4561.6ns (+11.1%) | [+1172, +10056]ns | [43653, 50525] | YES | 0.0309 | 0.0309 | 0 |
| sequential | 27308ns | -13209.4ns (-32.2%) | [-14340, -7364]ns | [25158, 32209] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 40964ns | +14.7% | -33.3% |
| 1 | 2 | 38811ns | +32.4% | -36.5% |
| 1 | 3 | 42095ns | +3.1% | -11.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.590 | HIGH- (thermal bounce) |
| per_core | -0.533 | HIGH- (thermal bounce) |
| sequential | -0.292 | moderate- |

**Consistency summary:**

- **per_core**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 42568.3ns | 44890.4ns | 94.8% | HIGH |
| per_core | 47014.7ns | 47526.1ns | 98.9% | HIGH |
| sequential | 30888.2ns | 29784.1ns | 103.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 37437.9-55518.7 ns)
  37437.9 |########################################
  38341.9 |
  39246.0 |##########
  40150.0 |##############################
  41054.1 |##########
  41958.1 |####################
  42862.1 |
  43766.2 |####################
  44670.2 |##########
  45574.3 |
  46478.3 |##########
  47382.3 |
  48286.4 |
  49190.4 |
  50094.4 |
  50998.5 |
  51902.5 |
  52806.6 |
  53710.6 |
  54614.6 |
  (1 below, 2 above range)

per_core (n=18, range 41443.0-52446.1 ns)
  41443.0 |#############
  41993.1 |#############
  42543.3 |#############
  43093.4 |#############
  43643.6 |#############
  44193.7 |
  44743.9 |#############
  45294.0 |#############
  45844.2 |
  46394.4 |
  46944.5 |
  47494.7 |
  48044.8 |##########################
  48595.0 |#############
  49145.1 |
  49695.3 |
  50245.4 |########################################
  50795.6 |
  51345.7 |
  51895.9 |##########################
  (1 below, 2 above range)

sequential (n=18, range 23949.6-37155.6 ns)
  23949.6 |##########################
  24609.9 |##########################
  25270.2 |#############
  25930.5 |
  26590.8 |########################################
  27251.1 |#############
  27911.4 |#############
  28571.7 |
  29232.0 |#############
  29892.3 |#############
  30552.6 |
  31212.9 |
  31873.2 |
  32533.5 |
  33193.8 |
  33854.1 |#############
  34514.4 |
  35174.7 |
  35835.0 |#############
  36495.3 |
  (1 below, 3 above range)

```

## Diagnostics

- **grouped**: CV=29.2% (high variance, measurements may be unstable)
- **grouped**: bridge=102.5% of algo (FFI overhead may distort results)
- **per_core**: bridge=94.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=105.1% of algo (FFI overhead may distort results)
