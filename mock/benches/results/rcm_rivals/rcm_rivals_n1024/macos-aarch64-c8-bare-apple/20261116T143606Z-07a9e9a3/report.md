# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (5.33 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_snake at 4.88 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### riv_snake is fastest but the noisiest (CV 8.0%)

riv_snake wins on median (4.88 ms) yet has the highest variance (CV 8.0%), while riv_scr is the steadiest (CV 6.5%, 5.21 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### riv_rowm shows alternating (throttle bounce) (autocorr -0.63)

riv_rowm's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader riv_snake vs stability leader riv_scr (+7% speed for 1.2x steadier)

riv_snake is fastest (4.88 ms, CV 8.0%); riv_scr gives up 6.9% median for 1.2x lower variance (CV 6.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_snake** at 4875400.0 ns median (-8.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.09x (fastest 4875400.0 ns, slowest 5326473.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 5366906ns | 5333917ns | 4887493ns | 5226411ns | 5817356ns | base |
| riv_rowm | 5092591ns | 5051681ns | 4546231ns | 4978778ns | 5536491ns | -5.11% |
| riv_scr | 5312839ns | 5220700ns | 4893533ns | 5182333ns | 5718250ns | -1.01% |
| riv_snake | 4967051ns | 4882035ns | 4553308ns | 4830462ns | 5378805ns | -7.45% |
| riv_spec | 5190689ns | 5115940ns | 4720996ns | 5055202ns | 5628767ns | -3.28% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 5359881ns | 4880690ns | 5809862ns | base | 0.000 |
| riv_rowm | 5086134ns | 4539704ns | 5530167ns | -5.11% | 0.000 |
| riv_scr | 5306331ns | 4887226ns | 5711534ns | -1.00% | 0.000 |
| riv_snake | 4960230ns | 4547037ns | 5371499ns | -7.46% | 0.000 |
| riv_spec | 5184202ns | 4714651ns | 5622347ns | -3.28% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 85.2% |
| riv_rowm | 0.000 | 90.0% |
| riv_scr | 0.000 | 87.1% |
| riv_snake | 0.000 | 93.1% |
| riv_spec | 0.000 | 88.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 5366906ns | 5366906ns | base |
| riv_rowm | 5092591ns | 5092591ns | -5.11% |
| riv_scr | 5312839ns | 5312839ns | -1.01% |
| riv_snake | 4967051ns | 4967051ns | -7.45% |
| riv_spec | 5190689ns | 5190689ns | -3.28% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 5326473ns | base | --- | [4990600, 5610152] | --- | --- | --- | --- |
| riv_rowm | 5045640ns | no significant difference | [-539838, +34912]ns | [4782696, 5458844] | no | 0.4807 | 0.4807 | 0 |
| riv_scr | 5214175ns | no significant difference | [-256469, +275571]ns | [5059925, 5499144] | no | 0.4807 | 0.4807 | 0 |
| riv_snake | 4875400ns | -414097.9ns (-7.8%) | [-706281, -173446]ns | [4736556, 4931156] | YES | 0.0302 | 0.0075 | 0 |
| riv_spec | 5109638ns | no significant difference | [-400162, +24758]ns | [4884967, 5408342] | no | 0.4758 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 5435171ns | -1.0% | +0.2% | -11.0% | +2.2% |
| 1 | 2 | 4898688ns | +4.5% | +4.4% | -0.0% | +7.6% |
| 1 | 3 | 5312167ns | +0.3% | -0.7% | -8.2% | +0.3% |
| 2 | 1 | 5266733ns | -7.1% | -2.3% | -7.4% | -5.2% |
| 2 | 2 | 4938808ns | +12.4% | +9.1% | -4.3% | -1.7% |
| 2 | 3 | 5705433ns | -18.9% | -12.8% | -13.3% | -9.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.488 | moderate- |
| riv_rowm | -0.633 | HIGH- (thermal bounce) |
| riv_scr | -0.549 | HIGH- (thermal bounce) |
| riv_snake | -0.505 | HIGH- (thermal bounce) |
| riv_spec | 0.269 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 3/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 5/6, lost 0/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 69473136.1ns | 5359880.5ns | 1296.2% | HIGH |
| riv_rowm | 69211983.3ns | 5086133.8ns | 1360.8% | HIGH |
| riv_scr | 68973514.6ns | 5306331.5ns | 1299.8% | HIGH |
| riv_snake | 69202525.5ns | 4960229.6ns | 1395.1% | HIGH |
| riv_spec | 69178693.3ns | 5184201.6ns | 1334.4% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 4880690.3-5809862.5 ns)
  4880690.3 |##########################
  4927148.9 |#############
  4973607.5 |
  5020066.1 |#############
  5066524.7 |
  5112983.3 |
  5159441.9 |
  5205900.5 |
  5252359.2 |#############
  5298817.8 |########################################
  5345276.4 |
  5391735.0 |##########################
  5438193.6 |
  5484652.2 |
  5531110.8 |
  5577569.4 |#############
  5624028.0 |##########################
  5670486.7 |#############
  5716945.3 |
  5763403.9 |#############
  (2 below, 1 above range)

riv_rowm (n=18, range 4539704.2-5530166.7 ns)
  4539704.2 |
  4589227.3 |####################
  4638750.4 |####################
  4688273.5 |
  4737796.7 |####################
  4787319.8 |####################
  4836842.9 |
  4886366.0 |########################################
  4935889.2 |####################
  4985412.3 |
  5034935.4 |
  5084458.5 |####################
  5133981.7 |
  5183504.8 |
  5233027.9 |
  5282551.0 |########################################
  5332074.2 |####################
  5381597.3 |
  5431120.4 |
  5480643.5 |
  (2 below, 5 above range)

riv_scr (n=18, range 4887226.4-5711534.0 ns)
  4887226.4 |
  4928441.7 |####################
  4969657.1 |########################################
  5010872.5 |
  5052087.9 |
  5093303.3 |########################################
  5134518.7 |########################################
  5175734.0 |
  5216949.4 |
  5258164.8 |####################
  5299380.2 |
  5340595.6 |
  5381811.0 |########################################
  5423026.3 |####################
  5464241.7 |
  5505457.1 |
  5546672.5 |####################
  5587887.9 |
  5629103.3 |####################
  5670318.6 |####################
  (2 below, 2 above range)

riv_snake (n=18, range 4547037.5-5371498.6 ns)
  4547037.5 |##########
  4588260.5 |##########
  4629483.6 |
  4670706.6 |##########
  4711929.7 |####################
  4753152.8 |
  4794375.8 |
  4835598.9 |########################################
  4876821.9 |##############################
  4918045.0 |##########
  4959268.0 |
  5000491.1 |
  5041714.1 |##########
  5082937.2 |
  5124160.3 |
  5165383.3 |
  5206606.4 |
  5247829.4 |
  5289052.5 |
  5330275.5 |
  (1 below, 3 above range)

riv_spec (n=18, range 4714651.4-5622347.2 ns)
  4714651.4 |#############
  4760036.2 |
  4805421.0 |
  4850805.8 |##########################
  4896190.6 |#############
  4941575.3 |
  4986960.1 |########################################
  5032344.9 |
  5077729.7 |
  5123114.5 |
  5168499.3 |#############
  5213884.1 |
  5259268.9 |#############
  5304653.7 |#############
  5350038.5 |#############
  5395423.2 |
  5440808.0 |#############
  5486192.8 |#############
  5531577.6 |#############
  5576962.4 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=1296.8% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=1366.9% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1321.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=1411.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=1347.7% of algo (FFI overhead may distort results)
