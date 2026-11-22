# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_exch is fastest but the noisiest (CV 10.4%)

cs_exch wins on median (21.35 us) yet has the highest variance (CV 10.4%), while cs_plain is the steadiest (CV 0.4%, 21.58 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Speed leader cs_exch vs stability leader cs_plain (+1% speed for 28.3x steadier)

cs_exch is fastest (21.35 us, CV 10.4%); cs_plain gives up 1.1% median for 28.3x lower variance (CV 0.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: cs_exch** at 21354.2 ns median (-1.0% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.26x (fastest 21354.2 ns, slowest 26925.0 ns)

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
| cs_exch | 24815ns | 23971ns | 23492ns | 23959ns | 26760ns | +1.78% |
| cs_plain | 24380ns | 24304ns | 23943ns | 24224ns | 24833ns | base |
| cs_snap | 29723ns | 29481ns | 29067ns | 29375ns | 30574ns | +21.92% |
| cs_snap_twin | 29950ns | 29840ns | 29371ns | 29765ns | 30518ns | +22.85% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 22193ns | 21250ns | 23919ns | +2.74% | 0.003 |
| cs_plain | 21600ns | 21499ns | 21694ns | base | 0.003 |
| cs_snap | 27006ns | 26811ns | 27285ns | +25.03% | 0.002 |
| cs_snap_twin | 27112ns | 26831ns | 27550ns | +25.52% | 0.002 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.003 | 99.5% |
| cs_plain | 0.003 | 98.5% |
| cs_snap | 0.002 | 79.0% |
| cs_snap_twin | 0.002 | 78.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 24815ns | 24815ns | +1.78% |
| cs_plain | 24380ns | 24380ns | base |
| cs_snap | 29723ns | 29723ns | +21.92% |
| cs_snap_twin | 29950ns | 29950ns | +22.85% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 21579ns | base | --- | [21556, 21631] | --- | --- | --- | --- |
| cs_exch | 21354ns | no significant difference | [-300, +475]ns | [21292, 22110] | no | 0.4807 | 0.4807 | 0 |
| cs_snap | 26896ns | +5325.1ns (+24.7%) | [+5229, +5560]ns | [26838, 27117] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 26925ns | +5354.1ns (+24.8%) | [+5254, +5596]ns | [26888, 27171] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 21558ns | -1.0% | +24.5% | +24.5% |
| 1 | 2 | 21554ns | -1.4% | +25.8% | +24.8% |
| 1 | 3 | 21583ns | -1.3% | +24.6% | +26.7% |
| 2 | 1 | 21633ns | -1.4% | +24.4% | +27.2% |
| 2 | 2 | 21592ns | +3.0% | +24.4% | +24.4% |
| 2 | 3 | 21512ns | +6.2% | +25.1% | +25.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | 0.409 | moderate+ |
| cs_plain | 0.091 | ok |
| cs_snap | -0.390 | moderate- |
| cs_snap_twin | 0.098 | ok |

**Consistency summary:**

- **cs_exch**: won 4/6, lost 2/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 363671.5ns | 22192.6ns | 1638.7% | HIGH |
| cs_plain | 356201.1ns | 21599.8ns | 1649.1% | HIGH |
| cs_snap | 355982.6ns | 27005.8ns | 1318.2% | HIGH |
| cs_snap_twin | 357391.2ns | 27112.5ns | 1318.2% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 21250.0-23918.8 ns)
  21250.0 |########################################
  21383.5 |####
  21516.9 |####
  21650.3 |
  21783.8 |
  21917.2 |####
  22050.7 |
  22184.1 |####
  22317.5 |
  22451.0 |####
  22584.4 |
  22717.8 |########
  22851.3 |
  22984.7 |
  23118.1 |
  23251.6 |
  23385.0 |
  23518.5 |
  23651.9 |
  23785.3 |
  (1 below, 1 above range)

cs_plain (n=18, range 21498.6-21694.5 ns)
  21498.6 |#############
  21508.4 |#############
  21518.2 |
  21528.0 |#############
  21537.8 |
  21547.6 |#############
  21557.4 |#############
  21567.1 |########################################
  21576.9 |#############
  21586.7 |##########################
  21596.5 |
  21606.3 |
  21616.1 |
  21625.9 |##########################
  21635.7 |
  21645.5 |
  21655.3 |
  21665.1 |
  21674.9 |#############
  21684.7 |
  (1 below, 3 above range)

cs_snap (n=18, range 26811.1-27284.7 ns)
  26811.1 |########################################
  26834.8 |####################
  26858.5 |##########
  26882.2 |####################
  26905.8 |##########
  26929.5 |
  26953.2 |
  26976.9 |
  27000.6 |
  27024.2 |##########
  27047.9 |
  27071.6 |
  27095.3 |##########
  27119.0 |##########
  27142.6 |##########
  27166.3 |
  27190.0 |
  27213.7 |
  27237.4 |
  27261.0 |
  (1 below, 3 above range)

cs_snap_twin (n=18, range 26830.6-27550.0 ns)
  26830.6 |##############################
  26866.5 |####################
  26902.5 |########################################
  26938.5 |##########
  26974.5 |####################
  27010.4 |
  27046.4 |
  27082.4 |
  27118.3 |
  27154.3 |
  27190.3 |
  27226.3 |
  27262.2 |
  27298.2 |
  27334.2 |##########
  27370.1 |##########
  27406.1 |
  27442.1 |
  27478.1 |
  27514.0 |##########
  (1 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=1667.0% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1649.9% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1322.0% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1321.3% of algo (FFI overhead may distort results)
