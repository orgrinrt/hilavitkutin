# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_adj, rcm_rev) are a dead heat (<1%)

rcm_adj (124.81 us) and rcm_rev (125.09 us) differ by 0.22%, inside the noise, even though the wider field spreads 7.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (124.81 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 124812.3 ns median
- Spread: 1.07x (fastest 124812.3 ns, slowest 134007.6 ns)

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
| Counter frequency | 2112746224 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 126726ns | 126699ns | 125630ns | 126482ns | 127640ns | base |
| rcm_half | 128848ns | 132765ns | 117520ns | 128647ns | 134813ns | +1.67% |
| rcm_rev | 126796ns | 126899ns | 125056ns | 126389ns | 128277ns | +0.06% |
| rcm_scr | 132289ns | 135788ns | 121383ns | 129882ns | 141354ns | +4.39% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 124898ns | 123856ns | 125838ns | base | 0.001 |
| rcm_half | 127004ns | 115620ns | 132986ns | +1.69% | 0.001 |
| rcm_rev | 124986ns | 123264ns | 126454ns | +0.07% | 0.001 |
| rcm_scr | 130439ns | 119604ns | 139414ns | +4.44% | 0.000 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 92.6% |
| rcm_half | 0.000 | 88.3% |
| rcm_rev | 0.001 | 92.4% |
| rcm_scr | 0.000 | 86.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 126726ns | 126726ns | base |
| rcm_half | 128848ns | 128848ns | +1.67% |
| rcm_rev | 126796ns | 126796ns | +0.06% |
| rcm_scr | 132289ns | 132289ns | +4.39% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 124812ns | base | --- | [124290, 125436] | --- | --- | --- | --- |
| rcm_half | 130900ns | no significant difference | [-7237, +8152]ns | [119113, 132527] | no | 0.7137 | 0.2379 | 0 |
| rcm_rev | 125091ns | no significant difference | [-1271, +1692]ns | [124071, 126134] | no | 0.7210 | 0.4807 | 0 |
| rcm_scr | 134008ns | no significant difference | [-2130, +11580]ns | [122507, 135772] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 124173ns | +5.5% | +1.8% | +8.4% |
| 1 | 2 | 123821ns | +7.4% | +2.0% | -2.0% |
| 1 | 3 | 125279ns | +4.3% | -1.6% | +13.1% |
| 2 | 1 | 125445ns | -8.0% | -1.6% | +6.8% |
| 2 | 2 | 126431ns | -6.3% | -0.7% | -4.9% |
| 2 | 3 | 124330ns | +7.0% | -0.0% | -1.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.110 | ok |
| rcm_half | 0.161 | ok |
| rcm_rev | 0.102 | ok |
| rcm_scr | -0.159 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 3/6, lost 2/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 1913135.9ns | 124898.4ns | 1531.8% | HIGH |
| rcm_half | 1913042.5ns | 127004.2ns | 1506.3% | HIGH |
| rcm_rev | 1913255.4ns | 124986.4ns | 1530.8% | HIGH |
| rcm_scr | 1913241.4ns | 130438.5ns | 1466.8% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 123856.3-125837.6 ns)
  123856.3 |
  123955.4 |####################
  124054.5 |
  124153.5 |########################################
  124252.6 |########################################
  124351.6 |####################
  124450.7 |
  124549.8 |####################
  124648.8 |
  124747.9 |
  124847.0 |
  124946.0 |####################
  125045.1 |
  125144.2 |####################
  125243.2 |####################
  125342.3 |####################
  125441.3 |####################
  125540.4 |########################################
  125639.5 |
  125738.5 |
  (2 below, 2 above range)

rcm_half (n=18, range 115620.1-132986.4 ns)
  115620.1 |######
  116488.4 |
  117356.7 |
  118225.0 |#############
  119093.3 |######
  119961.6 |
  120830.0 |
  121698.3 |
  122566.6 |
  123434.9 |
  124303.2 |
  125171.5 |
  126039.8 |
  126908.2 |
  127776.5 |
  128644.8 |
  129513.1 |
  130381.4 |########################################
  131249.7 |
  132118.1 |####################
  (2 below, 3 above range)

rcm_rev (n=18, range 123263.8-126453.6 ns)
  123263.8 |##########################
  123423.3 |#############
  123582.8 |
  123742.3 |
  123901.8 |#############
  124061.3 |#############
  124220.8 |##########################
  124380.2 |
  124539.7 |
  124699.2 |
  124858.7 |#############
  125018.2 |
  125177.7 |#############
  125337.2 |##########################
  125496.6 |
  125656.1 |
  125815.6 |
  125975.1 |#############
  126134.6 |#############
  126294.1 |########################################
  (1 below, 1 above range)

rcm_scr (n=18, range 119603.8-139413.9 ns)
  119603.8 |##########################
  120594.3 |#############
  121584.8 |##########################
  122575.3 |#############
  123565.8 |
  124556.3 |
  125546.8 |#############
  126537.3 |
  127527.8 |
  128518.3 |
  129508.8 |
  130499.3 |
  131489.9 |
  132480.4 |
  133470.9 |########################################
  134461.4 |##########################
  135451.9 |
  136442.4 |#############
  137432.9 |
  138423.4 |
  (1 below, 4 above range)

```

## Diagnostics

- **rcm_adj**: bridge=1532.7% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=1461.5% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=1529.3% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=1427.5% of algo (FFI overhead may distort results)
