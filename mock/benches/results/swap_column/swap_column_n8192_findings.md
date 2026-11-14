# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_exch dominates: 41% faster than the next best (cs_plain)

cs_exch (2.25 ms) leads cs_plain (3.18 ms) by 41%, a clear separation rather than a photo finish. CV 3.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_exch beats baseline by 29% (significant)

cs_exch is -910.41 us (29%) faster than baseline cs_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {cs_exch} vs {cs_plain, cs_snap_twin, cs_snap} (41% apart)

The field splits into a fast tier {cs_exch} and a slow tier {cs_plain, cs_snap_twin, cs_snap} with a 41% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: cs_exch** at 2250381.2 ns median (-29.2% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.46x (fastest 2250381.2 ns, slowest 3286956.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 2281089ns | 2255460ns | 2184958ns | 2244956ns | 2383353ns | -28.47% |
| cs_plain | 3188833ns | 3183194ns | 3107890ns | 3169076ns | 3258940ns | base |
| cs_snap | 3290314ns | 3292081ns | 3158269ns | 3273289ns | 3381874ns | +3.18% |
| cs_snap_twin | 3265635ns | 3263769ns | 3173512ns | 3241929ns | 3347257ns | +2.41% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 2275778ns | 2179267ns | 2378041ns | -28.49% | 0.004 |
| cs_plain | 3182258ns | 3101410ns | 3252753ns | base | 0.003 |
| cs_snap | 3284705ns | 3152561ns | 3376089ns | +3.22% | 0.002 |
| cs_snap_twin | 3259838ns | 3166938ns | 3341551ns | +2.44% | 0.003 |

## Performance model

- Peak throughput: **0.004 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.004 | 96.8% |
| cs_plain | 0.003 | 68.6% |
| cs_snap | 0.002 | 66.3% |
| cs_snap_twin | 0.003 | 66.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 2281089ns | 2281089ns | -28.47% |
| cs_plain | 3188833ns | 3188833ns | base |
| cs_snap | 3290314ns | 3290314ns | +3.18% |
| cs_snap_twin | 3265635ns | 3265635ns | +2.41% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 3176573ns | base | --- | [3140348, 3202935] | --- | --- | --- | --- |
| cs_exch | 2250381ns | -910408.4ns (-28.7%) | [-955648, -825879]ns | [2220925, 2306365] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 3286956ns | +88475.1ns (+2.8%) | [+34425, +118579]ns | [3243598, 3319288] | YES | 0.0002 | 0.0001 | 0 |
| cs_snap_twin | 3258512ns | +86139.5ns (+2.7%) | [+14271, +147967]ns | [3196002, 3307377] | YES | 0.0309 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 3103633ns | -26.3% | +8.2% | +3.2% |
| 1 | 2 | 3141083ns | -27.8% | +3.0% | +6.2% |
| 1 | 3 | 3191608ns | -29.5% | +2.8% | +3.5% |
| 2 | 1 | 3130400ns | -28.1% | +0.6% | +4.1% |
| 2 | 2 | 3238750ns | -31.4% | +2.5% | +1.4% |
| 2 | 3 | 3204408ns | -31.8% | +2.0% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | 0.356 | moderate+ |
| cs_plain | 0.009 | ok |
| cs_snap | -0.415 | moderate- |
| cs_snap_twin | -0.177 | ok |

**Consistency summary:**

- **cs_exch**: won 6/6, lost 0/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 44705134.7ns | 2275777.8ns | 1964.4% | HIGH |
| cs_plain | 44880677.5ns | 3182258.1ns | 1410.3% | HIGH |
| cs_snap | 44741025.9ns | 3284705.3ns | 1362.1% | HIGH |
| cs_snap_twin | 44708803.0ns | 3259838.0ns | 1371.5% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 2179266.7-2378041.0 ns)
  2179266.7 |########################################
  2189205.4 |
  2199144.1 |####################
  2209082.8 |
  2219021.6 |########################################
  2228960.3 |
  2238899.0 |########################################
  2248837.7 |########################################
  2258776.4 |########################################
  2268715.1 |
  2278653.8 |####################
  2288592.6 |
  2298531.3 |
  2308470.0 |
  2318408.7 |####################
  2328347.4 |
  2338286.1 |####################
  2348224.8 |
  2358163.6 |
  2368102.3 |
  (1 below, 3 above range)

cs_plain (n=18, range 3101409.7-3252753.5 ns)
  3101409.7 |#############
  3108976.9 |
  3116544.1 |#############
  3124111.3 |#############
  3131678.5 |
  3139245.7 |########################################
  3146812.9 |
  3154380.0 |
  3161947.2 |
  3169514.4 |##########################
  3177081.6 |#############
  3184648.8 |##########################
  3192216.0 |
  3199783.2 |##########################
  3207350.3 |
  3214917.5 |
  3222484.7 |
  3230051.9 |
  3237619.1 |#############
  3245186.3 |#############
  (1 below, 2 above range)

cs_snap (n=18, range 3152561.1-3376088.9 ns)
  3152561.1 |
  3163737.5 |
  3174913.9 |#############
  3186090.3 |#############
  3197266.7 |
  3208443.0 |
  3219619.4 |
  3230795.8 |#############
  3241972.2 |#############
  3253148.6 |
  3264325.0 |#############
  3275501.4 |#############
  3286677.8 |########################################
  3297854.2 |
  3309030.6 |##########################
  3320207.0 |#############
  3331383.3 |
  3342559.7 |#############
  3353736.1 |##########################
  3364912.5 |
  (2 below, 1 above range)

cs_snap_twin (n=18, range 3166937.5-3341550.7 ns)
  3166937.5 |#############
  3175668.2 |##########################
  3184398.8 |#############
  3193129.5 |#############
  3201860.1 |#############
  3210590.8 |
  3219321.5 |
  3228052.1 |
  3236782.8 |
  3245513.4 |
  3254244.1 |########################################
  3262974.8 |
  3271705.4 |
  3280436.1 |#############
  3289166.7 |#############
  3297897.4 |#############
  3306628.1 |#############
  3315358.7 |
  3324089.4 |
  3332820.0 |#############
  (1 below, 3 above range)

```

## Diagnostics

- **cs_exch**: bridge=1985.0% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1410.7% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1360.2% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1371.3% of algo (FFI overhead may distort results)
