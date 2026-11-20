# W = 4, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### per_core is fastest but the noisiest (CV 15.8%)

per_core wins on median (152.07 us) yet has the highest variance (CV 15.8%), while sequential is the steadiest (CV 11.1%, 299.36 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### per_core shows alternating (throttle bounce) (autocorr -0.51)

per_core's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### grouped is inconsistent: worst-20% is 1.6x its best-20%

grouped's best 20% of batches run at 122.24 us but its worst 20% at 189.83 us (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: per_core** at 152071.9 ns median (-5.9% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.97x (fastest 152071.9 ns, slowest 299361.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 64 |
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
| grouped | 162874ns | 162335ns | 123019ns | 157762ns | 190470ns | base |
| per_core | 154399ns | 152731ns | 122909ns | 146719ns | 181663ns | -5.20% |
| sequential | 304561ns | 300205ns | 271333ns | 293734ns | 337417ns | +86.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 162218ns | 122242ns | 189830ns | base | 0.404 |
| per_core | 153751ns | 122155ns | 180907ns | -5.22% | 0.426 |
| sequential | 303273ns | 270385ns | 335581ns | +86.95% | 0.216 |

## Performance model

- Peak throughput: **0.536 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.405 | 75.5% |
| per_core | 0.431 | 80.3% |
| sequential | 0.219 | 40.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 162874ns | 162874ns | base |
| per_core | 154399ns | 154399ns | -5.20% |
| sequential | 304561ns | 304561ns | +86.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 161691ns | base | --- | [147564, 181359] | --- | --- | --- | --- |
| per_core | 152072ns | no significant difference | [-30355, +3664]ns | [136562, 164827] | no | 0.2379 | 0.2379 | 0 |
| sequential | 299362ns | +133421.1ns (+82.5%) | [+118661, +151026]ns | [282635, 308966] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 159579ns | -19.6% | +72.9% |
| 1 | 2 | 175202ns | +3.0% | +70.5% |
| 1 | 3 | 175814ns | -12.5% | +75.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.148 | ok |
| per_core | -0.507 | HIGH- (thermal bounce) |
| sequential | -0.031 | ok |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 178453.2ns | 162218.1ns | 110.0% | HIGH |
| per_core | 163195.4ns | 153750.6ns | 106.1% | HIGH |
| sequential | 307377.9ns | 303273.4ns | 101.4% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 122242.0-189829.6 ns)
  122242.0 |#############
  125621.4 |
  129000.8 |#############
  132380.2 |
  135759.6 |#############
  139138.9 |#############
  142518.3 |
  145897.7 |
  149277.1 |
  152656.5 |#############
  156035.8 |#############
  159415.2 |########################################
  162794.6 |#############
  166174.0 |
  169553.4 |
  172932.7 |#############
  176312.1 |#############
  179691.5 |
  183070.9 |##########################
  186450.3 |
  (1 below, 3 above range)

per_core (n=18, range 122155.1-180907.2 ns)
  122155.1 |
  125092.7 |##########################
  128030.3 |#############
  130967.9 |#############
  133905.5 |
  136843.1 |#############
  139780.7 |
  142718.3 |########################################
  145655.9 |
  148593.5 |
  151531.1 |
  154468.8 |
  157406.4 |#############
  160344.0 |#############
  163281.6 |########################################
  166219.2 |
  169156.8 |
  172094.4 |
  175032.0 |#############
  177969.6 |
  (1 below, 3 above range)

sequential (n=18, range 270385.2-335580.7 ns)
  270385.2 |########################################
  273645.0 |####################
  276904.8 |####################
  280164.5 |
  283424.3 |####################
  286684.1 |
  289943.9 |####################
  293203.6 |####################
  296463.4 |####################
  299723.2 |####################
  302983.0 |########################################
  306242.7 |####################
  309502.5 |####################
  312762.3 |
  316022.1 |
  319281.8 |
  322541.6 |####################
  325801.4 |########################################
  329061.2 |
  332321.0 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=107.2% of algo (FFI overhead may distort results)
- **per_core**: bridge=98.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.5% of algo (FFI overhead may distort results)
