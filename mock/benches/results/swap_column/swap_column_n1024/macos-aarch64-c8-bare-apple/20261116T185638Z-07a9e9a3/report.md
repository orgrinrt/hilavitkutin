# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (cs_exch, cs_plain) are a dead heat (<1%)

cs_exch (208.08 us) and cs_plain (209.30 us) differ by 0.58%, inside the noise, even though the wider field spreads 40.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {cs_exch, cs_plain} vs {cs_snap, cs_snap_twin} (39% apart)

The field splits into a fast tier {cs_exch, cs_plain} and a slow tier {cs_snap, cs_snap_twin} with a 39% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: cs_exch** at 208079.1 ns median (-0.6% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.41x (fastest 208079.1 ns, slowest 292575.0 ns)

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
| cs_exch | 212606ns | 212329ns | 208332ns | 211186ns | 216874ns | -2.85% |
| cs_plain | 218846ns | 214650ns | 210299ns | 213486ns | 231160ns | base |
| cs_snap | 296972ns | 295492ns | 292968ns | 295020ns | 301901ns | +35.70% |
| cs_snap_twin | 296598ns | 296631ns | 294261ns | 296363ns | 298119ns | +35.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 208403ns | 204215ns | 212384ns | -2.74% | 0.005 |
| cs_plain | 214276ns | 205794ns | 226665ns | base | 0.005 |
| cs_snap | 293014ns | 288901ns | 297733ns | +36.75% | 0.003 |
| cs_snap_twin | 292526ns | 290279ns | 294063ns | +36.52% | 0.004 |

## Performance model

- Peak throughput: **0.005 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.005 | 98.1% |
| cs_plain | 0.005 | 97.6% |
| cs_snap | 0.004 | 70.0% |
| cs_snap_twin | 0.003 | 69.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 212606ns | 212606ns | -2.85% |
| cs_plain | 218846ns | 218846ns | base |
| cs_snap | 296972ns | 296972ns | +35.70% |
| cs_snap_twin | 296598ns | 296598ns | +35.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 209296ns | base | --- | [207704, 211629] | --- | --- | --- | --- |
| cs_exch | 208079ns | no significant difference | [-6283, +2412]ns | [205756, 209754] | no | 0.8145 | 0.8145 | 0 |
| cs_snap | 291694ns | +83783.3ns (+40.0%) | [+81096, +84944]ns | [290117, 293321] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 292575ns | +81791.7ns (+39.1%) | [+80717, +85400]ns | [291758, 293135] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 210962ns | -3.3% | +38.8% | +38.7% |
| 1 | 2 | 246283ns | -15.7% | +18.1% | +19.0% |
| 1 | 3 | 206875ns | +0.7% | +40.9% | +41.4% |
| 2 | 1 | 209504ns | -2.0% | +41.9% | +39.5% |
| 2 | 2 | 207121ns | +0.7% | +40.7% | +41.6% |
| 2 | 3 | 209088ns | -0.2% | +38.4% | +39.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.193 | ok |
| cs_plain | -0.207 | moderate- |
| cs_snap | -0.149 | ok |
| cs_snap_twin | -0.347 | moderate- |

**Consistency summary:**

- **cs_exch**: won 4/6, lost 2/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 5703210.7ns | 208402.5ns | 2736.6% | HIGH |
| cs_plain | 5717942.8ns | 214276.4ns | 2668.5% | HIGH |
| cs_snap | 5703600.7ns | 293014.3ns | 1946.5% | HIGH |
| cs_snap_twin | 5694734.3ns | 292525.9ns | 1946.7% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 204215.3-212384.0 ns)
  204215.3 |
  204623.7 |
  205032.2 |##############################
  205440.6 |
  205849.0 |##########
  206257.5 |
  206665.9 |##########
  207074.4 |##########
  207482.8 |##########
  207891.2 |
  208299.7 |########################################
  208708.1 |
  209116.5 |
  209525.0 |
  209933.4 |
  210341.8 |
  210750.3 |##########
  211158.7 |
  211567.1 |
  211975.6 |##########
  (2 below, 3 above range)

cs_plain (n=18, range 205794.5-226665.3 ns)
  205794.5 |
  206838.0 |########################################
  207881.5 |####################
  208925.1 |####################
  209968.6 |####################
  211012.2 |##############################
  212055.7 |
  213099.3 |##########
  214142.8 |
  215186.3 |
  216229.9 |
  217273.4 |
  218317.0 |
  219360.5 |
  220404.0 |
  221447.6 |
  222491.1 |
  223534.7 |
  224578.2 |
  225621.7 |
  (2 below, 2 above range)

cs_snap (n=18, range 288901.4-297733.3 ns)
  288901.4 |
  289343.0 |####################
  289784.6 |########################################
  290226.2 |####################
  290667.8 |####################
  291109.4 |########################################
  291551.0 |####################
  291992.6 |
  292434.2 |####################
  292875.8 |########################################
  293317.4 |####################
  293759.0 |
  294200.6 |
  294642.2 |####################
  295083.8 |
  295525.3 |
  295966.9 |
  296408.5 |
  296850.1 |####################
  297291.7 |
  (2 below, 2 above range)

cs_snap_twin (n=18, range 290279.2-294063.2 ns)
  290279.2 |
  290468.4 |
  290657.6 |
  290846.8 |####################
  291036.0 |####################
  291225.2 |####################
  291414.4 |
  291603.6 |####################
  291792.8 |####################
  291982.0 |
  292171.2 |####################
  292360.4 |########################################
  292549.6 |########################################
  292738.8 |####################
  292928.0 |####################
  293117.2 |####################
  293306.4 |####################
  293495.6 |
  293684.8 |####################
  293874.0 |
  (1 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=2742.8% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=2725.1% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1950.9% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1944.7% of algo (FFI overhead may distort results)
