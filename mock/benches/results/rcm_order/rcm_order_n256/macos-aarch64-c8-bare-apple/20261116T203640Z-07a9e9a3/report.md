# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (11.30 us) is smaller than the fastest variant's own run-to-run std-dev (18.26 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (311.47 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader rcm_adj vs stability leader rcm_rev (+1% speed for 2.9x steadier)

rcm_adj is fastest (311.47 us, CV 5.9%); rcm_rev gives up 1.4% median for 2.9x lower variance (CV 2.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 3.6% of the fastest

All 4 variants sit between 311.47 us and 322.77 us - a 3.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 311470.8 ns median
- 2 variants significantly slower than baseline
- Spread: 1.04x (fastest 311470.8 ns, slowest 322768.8 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 327288ns | 318829ns | 316478ns | 318431ns | 345978ns | base |
| rcm_half | 344352ns | 331181ns | 323079ns | 329163ns | 377770ns | +5.21% |
| rcm_rev | 325803ns | 325269ns | 319706ns | 323794ns | 331866ns | -0.45% |
| rcm_scr | 336895ns | 329052ns | 324156ns | 328524ns | 355820ns | +2.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 319978ns | 309112ns | 338892ns | base | 0.001 |
| rcm_half | 336448ns | 314522ns | 370797ns | +5.15% | 0.001 |
| rcm_rev | 317732ns | 312682ns | 323652ns | -0.70% | 0.001 |
| rcm_scr | 329556ns | 316200ns | 349310ns | +2.99% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 99.2% |
| rcm_half | 0.001 | 96.1% |
| rcm_rev | 0.001 | 97.8% |
| rcm_scr | 0.001 | 95.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 327288ns | 327288ns | base |
| rcm_half | 344352ns | 344352ns | +5.21% |
| rcm_rev | 325803ns | 325803ns | -0.45% |
| rcm_scr | 336895ns | 336895ns | +2.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 311471ns | base | --- | [310385, 317210] | --- | --- | --- | --- |
| rcm_half | 321669ns | +7212.5ns (+2.3%) | [+5231, +27808]ns | [316948, 345406] | YES | 0.0226 | 0.0075 | 0 |
| rcm_rev | 315925ns | no significant difference | [-2383, +6269]ns | [314488, 318817] | no | 0.2379 | 0.2379 | 0 |
| rcm_scr | 322769ns | +9912.6ns (+3.2%) | [+2719, +19388]ns | [318312, 331408] | YES | 0.0463 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 309800ns | +2.6% | +2.9% | +2.8% |
| 1 | 2 | 310383ns | +14.9% | +1.3% | +4.0% |
| 1 | 3 | 310662ns | +15.3% | +1.8% | +1.8% |
| 2 | 1 | 311629ns | +2.8% | +1.1% | +8.1% |
| 2 | 2 | 345388ns | -8.7% | -8.7% | -6.5% |
| 2 | 3 | 311362ns | +1.5% | +2.5% | +9.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.184 | ok |
| rcm_half | 0.219 | moderate+ |
| rcm_rev | -0.277 | moderate- |
| rcm_scr | -0.258 | moderate- |

**Consistency summary:**

- **rcm_half**: won 1/6, lost 5/6
- **rcm_rev**: won 1/6, lost 5/6
- **rcm_scr**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 11185875.0ns | 319977.8ns | 3495.8% | HIGH |
| rcm_half | 11184846.8ns | 336448.2ns | 3324.4% | HIGH |
| rcm_rev | 11180692.1ns | 317731.9ns | 3518.9% | HIGH |
| rcm_scr | 11181556.3ns | 329556.0ns | 3392.9% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 309112.5-338892.4 ns)
  309112.5 |#################################
  310601.5 |########################################
  312090.5 |
  313579.5 |
  315068.5 |######
  316557.5 |
  318046.5 |######
  319535.4 |
  321024.4 |
  322513.4 |
  324002.4 |
  325491.4 |######
  326980.4 |
  328469.4 |
  329958.4 |
  331447.4 |
  332936.4 |
  334425.4 |
  335914.4 |
  337403.4 |
  (1 below, 3 above range)

rcm_half (n=18, range 314522.2-370796.5 ns)
  314522.2 |########################################
  317335.9 |########################################
  320149.7 |#############
  322963.4 |##########################
  325777.1 |
  328590.8 |
  331404.5 |#############
  334218.2 |#############
  337032.0 |
  339845.7 |
  342659.4 |
  345473.1 |
  348286.8 |
  351100.5 |
  353914.2 |#############
  356728.0 |#############
  359541.7 |
  362355.4 |
  365169.1 |
  367982.8 |#############
  (2 below, 2 above range)

rcm_rev (n=18, range 312682.0-323652.1 ns)
  312682.0 |
  313230.5 |########################################
  313779.0 |
  314327.5 |########################################
  314876.0 |########################################
  315424.5 |####################
  315973.0 |####################
  316521.5 |####################
  317070.0 |
  317618.5 |####################
  318167.0 |
  318715.5 |########################################
  319264.0 |########################################
  319812.5 |
  320361.0 |
  320909.6 |
  321458.1 |
  322006.6 |
  322555.1 |
  323103.6 |####################
  (2 below, 1 above range)

rcm_scr (n=18, range 316200.0-349309.7 ns)
  316200.0 |##########################
  317855.5 |########################################
  319511.0 |
  321166.5 |##########################
  322822.0 |########################################
  324477.5 |#############
  326132.9 |
  327788.4 |
  329443.9 |
  331099.4 |
  332754.9 |
  334410.4 |
  336065.8 |#############
  337721.3 |
  339376.8 |#############
  341032.3 |#############
  342687.8 |
  344343.3 |
  345998.7 |
  347654.2 |
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3589.6% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3474.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3540.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3464.1% of algo (FFI overhead may distort results)
