# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (cs_plain) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline cs_plain has the worst median (3.29 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest cs_exch at 2.33 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### cs_exch dominates: 37% faster than the next best (cs_snap)

cs_exch (2.33 ms) leads cs_snap (3.19 ms) by 37%, a clear separation rather than a photo finish. CV 10.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_exch beats baseline by 30% (significant)

cs_exch is -999.58 us (30%) faster than baseline cs_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### Two tiers: {cs_exch} vs {cs_snap, cs_snap_twin, cs_plain} (37% apart)

The field splits into a fast tier {cs_exch} and a slow tier {cs_snap, cs_snap_twin, cs_plain} with a 37% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: cs_exch** at 2329279.1 ns median (-29.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.41x (fastest 2329279.1 ns, slowest 3289008.4 ns)

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
| cs_exch | 2408278ns | 2334715ns | 2167276ns | 2291840ns | 2703437ns | -28.95% |
| cs_plain | 3389707ns | 3295488ns | 3194457ns | 3280696ns | 3650848ns | base |
| cs_snap | 3209506ns | 3191381ns | 3121203ns | 3176919ns | 3302537ns | -5.32% |
| cs_snap_twin | 3399029ns | 3248965ns | 3120125ns | 3208904ns | 3823670ns | +0.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 2402955ns | 2162006ns | 2697812ns | -28.98% | 0.003 |
| cs_plain | 3383601ns | 3189039ns | 3643985ns | base | 0.002 |
| cs_snap | 3204247ns | 3116301ns | 3297337ns | -5.30% | 0.003 |
| cs_snap_twin | 3393563ns | 3114919ns | 3817549ns | +0.29% | 0.002 |

## Performance model

- Peak throughput: **0.004 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.004 | 92.8% |
| cs_plain | 0.002 | 65.7% |
| cs_snap | 0.003 | 67.8% |
| cs_snap_twin | 0.003 | 66.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 2408278ns | 2408278ns | -28.95% |
| cs_plain | 3389707ns | 3389707ns | base |
| cs_snap | 3209506ns | 3209506ns | -5.32% |
| cs_snap_twin | 3399029ns | 3399029ns | +0.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 3289008ns | base | --- | [3248167, 3372673] | --- | --- | --- | --- |
| cs_exch | 2329279ns | -999575.0ns (-30.4%) | [-1087225, -786090]ns | [2179156, 2526685] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 3186633ns | -125445.8ns (-3.8%) | [-162862, -15219]ns | [3139256, 3267558] | YES (adj: no) | 0.1444 | 0.0963 | 0 |
| cs_snap_twin | 3244312ns | no significant difference | [-131535, +89562]ns | [3133808, 3447119] | no | 0.4807 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 3254162ns | -33.4% | +1.6% | -3.8% |
| 1 | 2 | 3264879ns | -30.8% | -3.2% | -2.8% |
| 1 | 3 | 3337046ns | -33.9% | -5.0% | -3.1% |
| 2 | 1 | 3459262ns | -26.8% | -7.9% | +8.2% |
| 2 | 2 | 3231238ns | -18.9% | +1.3% | +0.7% |
| 2 | 3 | 3257438ns | -33.4% | -2.1% | -4.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | 0.042 | ok |
| cs_plain | -0.051 | ok |
| cs_snap | -0.246 | moderate- |
| cs_snap_twin | -0.028 | ok |

**Consistency summary:**

- **cs_exch**: won 6/6, lost 0/6
- **cs_snap**: won 4/6, lost 2/6
- **cs_snap_twin**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 45070834.3ns | 2402955.3ns | 1875.6% | HIGH |
| cs_plain | 45419127.3ns | 3383600.9ns | 1342.3% | HIGH |
| cs_snap | 44721614.6ns | 3204247.2ns | 1395.7% | HIGH |
| cs_snap_twin | 45001896.8ns | 3393563.0ns | 1326.1% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 2162005.6-2697812.5 ns)
  2162005.6 |########################################
  2188795.9 |##########
  2215586.3 |
  2242376.6 |##########
  2269167.0 |
  2295957.3 |
  2322747.6 |####################
  2349538.0 |
  2376328.3 |
  2403118.7 |##########
  2429909.0 |
  2456699.4 |
  2483489.7 |
  2510280.1 |##############################
  2537070.4 |
  2563860.8 |
  2590651.1 |
  2617441.5 |##########
  2644231.8 |
  2671022.2 |##########
  (2 below, 2 above range)

cs_plain (n=18, range 3189038.9-3643984.7 ns)
  3189038.9 |##########################
  3211786.2 |#############
  3234533.5 |##########################
  3257280.7 |########################################
  3280028.0 |
  3302775.3 |##########################
  3325522.6 |#############
  3348269.9 |#############
  3371017.2 |
  3393764.5 |#############
  3416511.8 |#############
  3439259.1 |#############
  3462006.4 |
  3484753.7 |#############
  3507500.9 |
  3530248.2 |
  3552995.5 |
  3575742.8 |
  3598490.1 |
  3621237.4 |
  (1 below, 1 above range)

cs_snap (n=18, range 3116301.4-3297337.5 ns)
  3116301.4 |####################
  3125353.2 |########################################
  3134405.0 |########################################
  3143456.8 |
  3152508.6 |
  3161560.4 |########################################
  3170612.2 |
  3179664.0 |########################################
  3188715.8 |
  3197767.6 |####################
  3206819.4 |
  3215871.2 |
  3224923.1 |####################
  3233974.9 |
  3243026.7 |
  3252078.5 |
  3261130.3 |####################
  3270182.1 |####################
  3279233.9 |####################
  3288285.7 |
  (1 below, 3 above range)

cs_snap_twin (n=18, range 3114919.4-3817549.3 ns)
  3114919.4 |########################################
  3150050.9 |########
  3185182.4 |
  3220313.9 |################
  3255445.4 |
  3290576.9 |########
  3325708.4 |########
  3360839.9 |
  3395971.4 |########
  3431102.9 |
  3466234.4 |########
  3501365.9 |########
  3536497.4 |
  3571628.9 |
  3606760.4 |
  3641891.8 |
  3677023.3 |
  3712154.8 |########
  3747286.3 |
  3782417.8 |
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=1927.5% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1366.0% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1403.7% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1381.3% of algo (FFI overhead may distort results)
