# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.2% (fastest 17.21 ms to slowest 17.41 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (199.98 us) is smaller than the fastest variant's own run-to-run std-dev (297.95 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_scr** at 17212750.0 ns median (-0.1% vs baseline)
- Spread: 1.01x (fastest 17212750.0 ns, slowest 17412733.4 ns)

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
| rcm_adj | 17332026ns | 17245415ns | 16790890ns | 17218717ns | 17772558ns | base |
| rcm_half | 17537309ns | 17419265ns | 16998639ns | 17343673ns | 18097100ns | +1.18% |
| rcm_rev | 17500008ns | 17420750ns | 17133124ns | 17368611ns | 17880545ns | +0.97% |
| rcm_scr | 17311074ns | 17220756ns | 17017846ns | 17203919ns | 17618421ns | -0.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 17323359ns | 16781700ns | 17763667ns | base | 0.000 |
| rcm_half | 17528614ns | 16989590ns | 18088630ns | +1.18% | 0.000 |
| rcm_rev | 17491375ns | 17124988ns | 17872258ns | +0.97% | 0.000 |
| rcm_scr | 17302409ns | 17009589ns | 17610197ns | -0.12% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 97.4% |
| rcm_half | 0.000 | 96.4% |
| rcm_rev | 0.000 | 96.4% |
| rcm_scr | 0.000 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 17332026ns | 17332026ns | base |
| rcm_half | 17537309ns | 17537309ns | +1.18% |
| rcm_rev | 17500008ns | 17500008ns | +0.97% |
| rcm_scr | 17311074ns | 17311074ns | -0.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 17236900ns | base | --- | [17126773, 17499717] | --- | --- | --- | --- |
| rcm_half | 17409350ns | no significant difference | [-291256, +481821]ns | [17226748, 17648342] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 17412733ns | no significant difference | [-95129, +351615]ns | [17282810, 17570971] | no | 0.7137 | 0.2379 | 0 |
| rcm_scr | 17212750ns | no significant difference | [-384502, +216762]ns | [17153162, 17338067] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 16832012ns | +4.1% | +2.1% | +2.2% |
| 1 | 2 | 17164842ns | -0.7% | +1.7% | -0.2% |
| 1 | 3 | 17229462ns | +1.7% | +0.6% | -0.1% |
| 2 | 1 | 17332838ns | -0.8% | +0.3% | +0.5% |
| 2 | 2 | 17244338ns | +2.2% | -0.6% | +0.7% |
| 2 | 3 | 17349958ns | +1.9% | +1.9% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.165 | ok |
| rcm_half | -0.294 | moderate- |
| rcm_rev | -0.493 | moderate- |
| rcm_scr | 0.394 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 1/6, lost 5/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 358460570.4ns | 17323358.6ns | 2069.2% | HIGH |
| rcm_half | 359344616.2ns | 17528614.1ns | 2050.0% | HIGH |
| rcm_rev | 360210392.6ns | 17491374.5ns | 2059.4% | HIGH |
| rcm_scr | 358779995.8ns | 17302409.3ns | 2073.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 16781700.0-17763666.7 ns)
  16781700.0 |
  16830798.3 |####################
  16879896.7 |####################
  16928995.0 |
  16978093.3 |
  17027191.7 |####################
  17076290.0 |####################
  17125388.3 |####################
  17174486.7 |########################################
  17223585.0 |########################################
  17272683.3 |
  17321781.7 |########################################
  17370880.0 |
  17419978.3 |
  17469076.7 |########################################
  17518175.0 |
  17567273.3 |####################
  17616371.7 |
  17665470.0 |
  17714568.3 |
  (1 below, 3 above range)

rcm_half (n=18, range 16989590.3-18088629.8 ns)
  16989590.3 |
  17044542.3 |########################################
  17099494.3 |
  17154446.2 |####################
  17209398.2 |####################
  17264350.2 |####################
  17319302.2 |####################
  17374254.1 |########################################
  17429206.1 |
  17484158.1 |########################################
  17539110.1 |
  17594062.1 |####################
  17649014.0 |####################
  17703966.0 |####################
  17758918.0 |####################
  17813870.0 |
  17868821.9 |
  17923773.9 |
  17978725.9 |
  18033677.9 |
  (2 below, 2 above range)

rcm_rev (n=18, range 17124987.5-17872257.6 ns)
  17124987.5 |##########################
  17162351.0 |#############
  17199714.5 |#############
  17237078.0 |
  17274441.5 |
  17311805.0 |#############
  17349168.5 |##########################
  17386532.0 |#############
  17423895.5 |########################################
  17461259.1 |#############
  17498622.6 |
  17535986.1 |
  17573349.6 |
  17610713.1 |
  17648076.6 |##########################
  17685440.1 |#############
  17722803.6 |
  17760167.1 |
  17797530.6 |
  17834894.1 |
  (1 below, 2 above range)

rcm_scr (n=18, range 17009588.9-17610196.5 ns)
  17009588.9 |##########
  17039619.2 |
  17069649.6 |##########
  17099680.0 |####################
  17129710.4 |
  17159740.8 |##########
  17189771.2 |########################################
  17219801.5 |##########
  17249831.9 |##########
  17279862.3 |##########
  17309892.7 |
  17339923.1 |
  17369953.5 |##########
  17399983.9 |##########
  17430014.2 |
  17460044.6 |
  17490075.0 |
  17520105.4 |
  17550135.8 |
  17580166.2 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2078.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2056.7% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2057.0% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2081.1% of algo (FFI overhead may distort results)
