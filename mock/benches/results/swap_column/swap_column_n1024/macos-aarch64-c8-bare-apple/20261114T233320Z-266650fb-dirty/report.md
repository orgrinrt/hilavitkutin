# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (cs_plain, cs_exch) are a dead heat (<1%)

cs_plain (209.26 us) and cs_exch (210.39 us) differ by 0.54%, inside the noise, even though the wider field spreads 47.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (209.26 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {cs_plain, cs_exch} vs {cs_snap, cs_snap_twin} (43% apart)

The field splits into a fast tier {cs_plain, cs_exch} and a slow tier {cs_snap, cs_snap_twin} with a 43% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (cs_plain) is the fastest** at 209260.5 ns median
- 2 variants significantly slower than baseline
- Spread: 1.47x (fastest 209260.5 ns, slowest 307543.8 ns)

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
| cs_exch | 226057ns | 213283ns | 205708ns | 213106ns | 255656ns | +2.19% |
| cs_plain | 221217ns | 214490ns | 210140ns | 214346ns | 237063ns | base |
| cs_snap | 311653ns | 305665ns | 293376ns | 303239ns | 333412ns | +40.88% |
| cs_snap_twin | 321644ns | 312075ns | 291903ns | 303006ns | 364469ns | +45.40% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 222304ns | 201656ns | 251838ns | +2.86% | 0.005 |
| cs_plain | 216114ns | 205715ns | 232280ns | base | 0.005 |
| cs_snap | 307887ns | 289586ns | 329642ns | +42.47% | 0.003 |
| cs_snap_twin | 317523ns | 287917ns | 360427ns | +46.92% | 0.003 |

## Performance model

- Peak throughput: **0.005 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.005 | 95.9% |
| cs_plain | 0.005 | 96.4% |
| cs_snap | 0.003 | 66.9% |
| cs_snap_twin | 0.003 | 65.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 226057ns | 226057ns | +2.19% |
| cs_plain | 221217ns | 221217ns | base |
| cs_snap | 311653ns | 311653ns | +40.88% |
| cs_snap_twin | 321644ns | 321644ns | +45.40% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 209260ns | base | --- | [207583, 221298] | --- | --- | --- | --- |
| cs_exch | 210385ns | no significant difference | [-4838, +13346]ns | [203965, 226290] | no | 1.0000 | 1.0000 | 0 |
| cs_snap | 301317ns | +89504.1ns (+42.8%) | [+80965, +103019]ns | [294248, 310617] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 307544ns | +93339.5ns (+44.6%) | [+74756, +116888]ns | [290827, 324077] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 209217ns | +6.6% | +43.1% | +47.6% |
| 1 | 2 | 208033ns | -1.6% | +48.8% | +53.8% |
| 1 | 3 | 209342ns | -0.6% | +54.5% | +39.0% |
| 2 | 1 | 207171ns | +7.2% | +43.8% | +41.5% |
| 2 | 2 | 206908ns | +1.8% | +42.2% | +79.9% |
| 2 | 3 | 227800ns | -11.3% | +28.7% | +27.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.234 | moderate- |
| cs_plain | -0.096 | ok |
| cs_snap | 0.203 | moderate+ |
| cs_snap_twin | -0.459 | moderate- |

**Consistency summary:**

- **cs_exch**: won 3/6, lost 3/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 5593570.8ns | 222304.4ns | 2516.2% | HIGH |
| cs_plain | 5591934.5ns | 216114.1ns | 2587.5% | HIGH |
| cs_snap | 5595773.4ns | 307887.0ns | 1817.5% | HIGH |
| cs_snap_twin | 5625483.3ns | 317523.2ns | 1771.7% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 201655.6-251838.2 ns)
  201655.6 |########################################
  204164.7 |####################
  206673.8 |##########
  209183.0 |####################
  211692.1 |
  214201.2 |
  216710.4 |##########
  219219.5 |
  221728.6 |####################
  224237.7 |
  226746.9 |
  229256.0 |##########
  231765.1 |##########
  234274.3 |
  236783.4 |
  239292.5 |
  241801.7 |
  244310.8 |
  246819.9 |
  249329.1 |
  (1 below, 3 above range)

cs_plain (n=18, range 205715.3-232279.9 ns)
  205715.3 |##############################
  207043.5 |########################################
  208371.7 |##############################
  209700.0 |
  211028.2 |
  212356.4 |##########
  213684.6 |
  215012.9 |
  216341.1 |
  217669.3 |
  218997.6 |##########
  220325.8 |
  221654.0 |
  222982.3 |####################
  224310.5 |##########
  225638.7 |
  226966.9 |##########
  228295.2 |
  229623.4 |
  230951.6 |
  (1 below, 1 above range)

cs_snap (n=18, range 289586.1-329641.7 ns)
  289586.1 |########################################
  291588.9 |####################
  293591.7 |########################################
  295594.5 |####################
  297597.2 |########################################
  299600.0 |
  301602.8 |####################
  303605.6 |
  305608.3 |####################
  307611.1 |########################################
  309613.9 |
  311616.7 |########################################
  313619.5 |
  315622.2 |
  317625.0 |
  319627.8 |
  321630.6 |####################
  323633.3 |
  325636.1 |
  327638.9 |
  (1 below, 2 above range)

cs_snap_twin (n=18, range 287916.7-360427.1 ns)
  287916.7 |########################################
  291542.2 |####################
  295167.7 |
  298793.3 |
  302418.8 |
  306044.3 |########################################
  309669.8 |
  313295.3 |
  316920.8 |##########
  320546.4 |
  324171.9 |
  327797.4 |##########
  331422.9 |
  335048.4 |
  338674.0 |
  342299.5 |
  345925.0 |
  349550.5 |
  353176.0 |
  356801.5 |##########
  (2 below, 3 above range)

```

## Diagnostics

- **cs_exch**: bridge=2656.3% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=2665.7% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1851.2% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1818.6% of algo (FFI overhead may distort results)
