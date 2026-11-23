# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_adj, rcm_rev) are a dead heat (<1%)

rcm_adj (125.12 us) and rcm_rev (125.91 us) differ by 0.63%, inside the noise, even though the wider field spreads 9.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rcm_adj shows alternating (throttle bounce) (autocorr -0.63)

rcm_adj's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (125.12 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 125123.1 ns median
- 1 variant significantly slower than baseline
- Spread: 1.09x (fastest 125123.1 ns, slowest 136523.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 2112776360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 126399ns | 126954ns | 121538ns | 126649ns | 128454ns | base |
| rcm_half | 127416ns | 131003ns | 116465ns | 127768ns | 132363ns | +0.80% |
| rcm_rev | 128230ns | 127716ns | 125634ns | 127343ns | 130858ns | +1.45% |
| rcm_scr | 142817ns | 138339ns | 133056ns | 136985ns | 156444ns | +12.99% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 124586ns | 119746ns | 126630ns | base | 0.001 |
| rcm_half | 125620ns | 114694ns | 130579ns | +0.83% | 0.001 |
| rcm_rev | 126442ns | 123852ns | 129062ns | +1.49% | 0.001 |
| rcm_scr | 140944ns | 131159ns | 154512ns | +13.13% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 91.7% |
| rcm_half | 0.000 | 88.8% |
| rcm_rev | 0.001 | 91.1% |
| rcm_scr | 0.000 | 84.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 126399ns | 126399ns | base |
| rcm_half | 127416ns | 127416ns | +0.80% |
| rcm_rev | 128230ns | 128230ns | +1.45% |
| rcm_scr | 142817ns | 142817ns | +12.99% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 125123ns | base | --- | [124252, 126250] | --- | --- | --- | --- |
| rcm_half | 129201ns | no significant difference | [-5098, +5803]ns | [119641, 130329] | no | 0.2379 | 0.2379 | 0 |
| rcm_rev | 125910ns | no significant difference | [-59, +3187]ns | [124765, 127930] | no | 0.1444 | 0.0963 | 0 |
| rcm_scr | 136524ns | +11615.1ns (+9.3%) | [+7715, +15802]ns | [132345, 140994] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 126840ns | +2.8% | -0.2% | +21.0% |
| 1 | 2 | 123802ns | +5.2% | +4.5% | +6.4% |
| 1 | 3 | 126578ns | -9.4% | +1.0% | +10.9% |
| 2 | 1 | 122379ns | +5.3% | +1.1% | +15.2% |
| 2 | 2 | 125200ns | +3.8% | -0.0% | +5.8% |
| 2 | 3 | 124623ns | -4.5% | +0.1% | +7.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.632 | HIGH- (thermal bounce) |
| rcm_half | -0.332 | moderate- |
| rcm_rev | 0.283 | moderate+ |
| rcm_scr | -0.281 | moderate- |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 1/6, lost 4/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 1913814.3ns | 124586.2ns | 1536.1% | HIGH |
| rcm_half | 1914219.5ns | 125620.2ns | 1523.8% | HIGH |
| rcm_rev | 1914714.9ns | 126441.9ns | 1514.3% | HIGH |
| rcm_scr | 1936794.2ns | 140944.1ns | 1374.2% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 119745.8-126630.2 ns)
  119745.8 |
  120090.0 |
  120434.2 |
  120778.5 |
  121122.7 |
  121466.9 |
  121811.1 |
  122155.3 |#############
  122499.5 |
  122843.8 |
  123188.0 |
  123532.2 |##########################
  123876.4 |
  124220.6 |##########################
  124564.8 |#############
  124909.1 |########################################
  125253.3 |
  125597.5 |##########################
  125941.7 |#############
  126285.9 |##########################
  (1 below, 3 above range)

rcm_half (n=18, range 114694.1-130578.5 ns)
  114694.1 |######
  115488.4 |
  116282.6 |
  117076.8 |
  117871.0 |
  118665.2 |#############
  119459.5 |######
  120253.7 |
  121047.9 |
  121842.1 |
  122636.3 |
  123430.6 |
  124224.8 |
  125019.0 |
  125813.2 |
  126607.4 |
  127401.7 |
  128195.9 |#############
  128990.1 |#############
  129784.3 |########################################
  (2 below, 2 above range)

rcm_rev (n=18, range 123852.5-129062.4 ns)
  123852.5 |####################
  124113.0 |####################
  124373.5 |
  124634.0 |########################################
  124894.5 |####################
  125155.0 |########################################
  125415.5 |
  125676.0 |
  125936.5 |
  126197.0 |
  126457.5 |########################################
  126718.0 |
  126978.5 |
  127239.0 |
  127499.5 |####################
  127759.9 |########################################
  128020.4 |####################
  128280.9 |
  128541.4 |
  128801.9 |
  (2 below, 3 above range)

rcm_scr (n=18, range 131159.5-154512.1 ns)
  131159.5 |########################################
  132327.1 |####################
  133494.7 |####################
  134662.4 |
  135830.0 |
  136997.6 |
  138165.3 |##########
  139332.9 |####################
  140500.5 |##############################
  141668.2 |
  142835.8 |##########
  144003.4 |
  145171.1 |
  146338.7 |
  147506.3 |
  148674.0 |
  149841.6 |
  151009.2 |
  152176.9 |
  153344.5 |##########
  (1 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: bridge=1529.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=1481.0% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=1519.6% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=1401.4% of algo (FFI overhead may distort results)
