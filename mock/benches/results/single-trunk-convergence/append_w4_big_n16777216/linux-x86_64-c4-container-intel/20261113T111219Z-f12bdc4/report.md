# append, far above cache, W = 4

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 18% faster than the next best (kernel_nway)

std_parallel (6.03 ms) leads kernel_nway (7.13 ms) by 18%, a clear separation rather than a photo finish. CV 13.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### std_parallel beats baseline by 61% (significant)

std_parallel is -9.51 ms (61%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 4.2x slower than the field

sequential (25.24 ms) is 4.2x the fastest (6.03 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.61)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.61, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.2x the fastest

Fastest std_parallel (6.03 ms) to slowest sequential (25.24 ms): 4.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 1.6x its best-20%

nway's best 20% of batches run at 7.13 ms but its worst 20% at 11.50 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 6025065.0 ns median (-61.1% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 4.19x (fastest 6025065.0 ns, slowest 25236836.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 4 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 15549285ns | 15507735ns | 14121209ns | 15294262ns | 16645859ns | base |
| head_tail_dyn | 15152435ns | 15015768ns | 14455920ns | 15007518ns | 15718069ns | -2.55% |
| kernel_nway | 7071645ns | 7135459ns | 5995331ns | 7025392ns | 7679181ns | -54.52% |
| kernel_seq | 15591625ns | 15514423ns | 14857305ns | 15370866ns | 16289924ns | +0.27% |
| nway | 9529733ns | 9335970ns | 7132731ns | 9009846ns | 11508065ns | -38.71% |
| sequential | 25902066ns | 25241360ns | 22811316ns | 24793449ns | 29110365ns | +66.58% |
| std_parallel | 6101952ns | 6031201ns | 4938911ns | 5856604ns | 7051496ns | -60.76% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15542900ns | 14113371ns | 16639256ns | base | 1.079 |
| head_tail_dyn | 15146214ns | 14448084ns | 15711788ns | -2.55% | 1.108 |
| kernel_nway | 7065662ns | 5990964ns | 7674744ns | -54.54% | 2.374 |
| kernel_seq | 15584933ns | 14848681ns | 16281298ns | +0.27% | 1.077 |
| nway | 9524862ns | 7127806ns | 11503297ns | -38.72% | 1.761 |
| sequential | 25897580ns | 22807071ns | 29105746ns | +66.62% | 0.648 |
| std_parallel | 6095262ns | 4930528ns | 7045435ns | -60.78% | 2.753 |

## Performance model

- Peak throughput: **3.403 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.082 | 31.8% |
| head_tail_dyn | 1.118 | 32.8% |
| kernel_nway | 2.354 | 69.2% |
| kernel_seq | 1.082 | 31.8% |
| nway | 1.798 | 52.8% |
| sequential | 0.665 | 19.5% |
| std_parallel | 2.785 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15549285ns | 15549285ns | base |
| head_tail_dyn | 15152435ns | 15152435ns | -2.55% |
| kernel_nway | 7071645ns | 7071645ns | -54.52% |
| kernel_seq | 15591625ns | 15591625ns | +0.27% |
| nway | 9529733ns | 9529733ns | -38.71% |
| sequential | 25902066ns | 25902066ns | +66.58% |
| std_parallel | 6101952ns | 6101952ns | -60.76% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 15501424ns | base | --- | [14488021, 16639256] | --- | --- | --- | --- |
| head_tail_dyn | 15010267ns | no significant difference | [-1007920, +229165]ns | [14716586, 15711788] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 7127960ns | -8629485.1ns (-55.7%) | [-9373873, -7428357]ns | [6394282, 7674744] | YES | 0.0469 | 0.0313 | 0 |
| kernel_seq | 15509747ns | no significant difference | [-1675501, +1755263]ns | [14963755, 16281298] | no | 1.0000 | 1.0000 | 0 |
| nway | 9330966ns | -6373303.0ns (-41.1%) | [-7682685, -3998127]ns | [7740323, 11503297] | YES | 0.0469 | 0.0313 | 0 |
| sequential | 25236836ns | +8768428.6ns (+56.6%) | [+7715901, +14579711]ns | [23350159, 29105746] | YES | 0.0469 | 0.0313 | 0 |
| std_parallel | 6025065ns | -9507032.3ns (-61.3%) | [-11051434, -7784449]ns | [5215285, 7045435] | YES | 0.0469 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15423008ns | -2.1% | -51.1% | -0.4% | -49.8% | +63.9% | -56.1% |
| 1 | 2 | 16305008ns | -5.8% | -55.4% | -6.8% | -33.8% | +53.4% | -67.8% |
| 1 | 3 | 14900685ns | +0.6% | -57.1% | +8.6% | -32.6% | +83.9% | -58.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.576 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.609 | HIGH- (thermal bounce) |
| kernel_nway | -0.057 | ok |
| kernel_seq | -0.264 | moderate- |
| nway | -0.319 | moderate- |
| sequential | -0.229 | moderate- |
| std_parallel | -0.591 | HIGH- (thermal bounce) |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 105063437.0ns | 15542900.2ns | 676.0% | HIGH |
| head_tail_dyn | 106043695.5ns | 15146213.7ns | 700.1% | HIGH |
| kernel_nway | 92442159.8ns | 7065661.9ns | 1308.3% | HIGH |
| kernel_seq | 104745793.6ns | 15584933.0ns | 672.1% | HIGH |
| nway | 95457800.9ns | 9524861.9ns | 1002.2% | HIGH |
| sequential | 111387293.9ns | 25897580.4ns | 430.1% | HIGH |
| std_parallel | 92156545.8ns | 6095261.8ns | 1511.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 14113371.4-16639255.7 ns)
  14113371.4 |########################################
  14239665.6 |
  14365959.8 |
  14492254.0 |
  14618548.3 |
  14744842.5 |########################################
  14871136.7 |########################################
  14997430.9 |
  15123725.1 |
  15250019.3 |
  15376313.6 |
  15502607.8 |
  15628902.0 |
  15755196.2 |
  15881490.4 |
  16007784.6 |########################################
  16134078.8 |
  16260373.1 |
  16386667.3 |
  16512961.5 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 14448084.3-15711787.8 ns)
  14448084.3 |####################
  14511269.5 |
  14574454.7 |
  14637639.8 |
  14700825.0 |
  14764010.2 |
  14827195.4 |
  14890380.5 |
  14953565.7 |########################################
  15016750.9 |####################
  15079936.1 |
  15143121.3 |
  15206306.4 |
  15269491.6 |
  15332676.8 |
  15395862.0 |
  15459047.1 |
  15522232.3 |
  15585417.5 |
  15648602.7 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 5990963.7-7674743.7 ns)
  5990963.7 |########################################
  6075152.7 |
  6159341.7 |
  6243530.7 |
  6327719.7 |
  6411908.7 |
  6496097.7 |
  6580286.7 |
  6664475.7 |
  6748664.7 |########################################
  6832853.7 |########################################
  6917042.7 |
  7001231.7 |
  7085420.7 |
  7169609.7 |
  7253798.7 |
  7337987.7 |########################################
  7422176.7 |
  7506365.7 |
  7590554.7 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 14848680.6-16281297.8 ns)
  14848680.6 |########################################
  14920311.5 |
  14991942.3 |
  15063573.2 |########################################
  15135204.0 |
  15206834.9 |
  15278465.8 |
  15350096.6 |
  15421727.5 |########################################
  15493358.3 |########################################
  15564989.2 |
  15636620.1 |########################################
  15708250.9 |
  15779881.8 |
  15851512.6 |
  15923143.5 |
  15994774.4 |
  16066405.2 |
  16138036.1 |
  16209666.9 |
  (0 below, 1 above range)

nway (n=6, range 7127806.1-11503296.8 ns)
  7127806.1 |########################################
  7346580.6 |
  7565355.2 |
  7784129.7 |
  8002904.2 |
  8221678.8 |########################################
  8440453.3 |########################################
  8659227.8 |
  8878002.4 |
  9096776.9 |
  9315551.4 |
  9534326.0 |
  9753100.5 |
  9971875.1 |########################################
  10190649.6 |
  10409424.1 |
  10628198.7 |
  10846973.2 |
  11065747.7 |
  11284522.3 |########################################
  (0 below, 1 above range)

sequential (n=6, range 22807070.9-29105746.2 ns)
  22807070.9 |####################
  23122004.7 |
  23436938.4 |
  23751872.2 |####################
  24066806.0 |####################
  24381739.7 |
  24696673.5 |
  25011607.3 |
  25326541.0 |
  25641474.8 |
  25956408.6 |########################################
  26271342.3 |
  26586276.1 |
  26901209.9 |
  27216143.6 |
  27531077.4 |
  27846011.2 |
  28160944.9 |
  28475878.7 |
  28790812.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 4930528.0-7045435.0 ns)
  4930528.0 |########################################
  5036273.4 |
  5142018.7 |
  5247764.1 |
  5353509.4 |
  5459254.8 |########################################
  5565000.1 |########################################
  5670745.5 |
  5776490.8 |
  5882236.2 |
  5987981.5 |
  6093726.9 |
  6199472.2 |
  6305217.6 |
  6410962.9 |########################################
  6516708.3 |
  6622453.6 |
  6728199.0 |
  6833944.3 |
  6939689.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=674.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=694.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1259.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=676.9% of algo (FFI overhead may distort results)
- **nway**: bridge=1024.9% of algo (FFI overhead may distort results)
- **sequential**: bridge=438.7% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1532.1% of algo (FFI overhead may distort results)
