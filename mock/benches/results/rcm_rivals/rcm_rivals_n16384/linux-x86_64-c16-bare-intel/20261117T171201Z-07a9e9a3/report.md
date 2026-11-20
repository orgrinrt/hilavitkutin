# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (149.46 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_rowm at 148.91 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 148.91 ms to slowest 149.46 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (544.30 us) is smaller than the fastest variant's own run-to-run std-dev (1.51 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_rowm** at 148913810.0 ns median (-0.4% vs baseline)
- Spread: 1.00x (fastest 148913810.0 ns, slowest 149458112.0 ns)

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
| Counter frequency | 2112768073 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 149650617ns | 149462436ns | 148388105ns | 149257538ns | 150871490ns | base |
| riv_rowm | 149473167ns | 148917757ns | 148103008ns | 148897942ns | 151021084ns | -0.12% |
| riv_scr | 150111300ns | 149052033ns | 148431894ns | 149036931ns | 152562557ns | +0.31% |
| riv_snake | 149208248ns | 149168110ns | 147815010ns | 148939776ns | 150307576ns | -0.30% |
| riv_spec | 150551888ns | 149199913ns | 148626069ns | 149236652ns | 153487651ns | +0.60% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 149646361ns | 148383761ns | 150867106ns | base | 0.000 |
| riv_rowm | 149468990ns | 148098802ns | 151016731ns | -0.12% | 0.000 |
| riv_scr | 150107089ns | 148427747ns | 152558412ns | +0.31% | 0.000 |
| riv_snake | 149203973ns | 147810783ns | 150303157ns | -0.30% | 0.000 |
| riv_spec | 150547779ns | 148621971ns | 153483502ns | +0.60% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.9% |
| riv_rowm | 0.000 | 99.3% |
| riv_scr | 0.000 | 99.2% |
| riv_snake | 0.000 | 99.1% |
| riv_spec | 0.000 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 149650617ns | 149650617ns | base |
| riv_rowm | 149473167ns | 149473167ns | -0.12% |
| riv_scr | 150111300ns | 150111300ns | +0.31% |
| riv_snake | 149208248ns | 149208248ns | -0.30% |
| riv_spec | 150551888ns | 150551888ns | +0.60% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 149458112ns | base | --- | [148840879, 150006493] | --- | --- | --- | --- |
| riv_rowm | 148913810ns | no significant difference | [-792290, +76938]ns | [148659192, 149547750] | no | 0.3850 | 0.0963 | 0 |
| riv_scr | 149048158ns | no significant difference | [-749194, +1230198]ns | [148850355, 150138807] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 149163855ns | no significant difference | [-770268, +93426]ns | [148572117, 149775535] | no | 0.4758 | 0.2379 | 0 |
| riv_spec | 149196017ns | no significant difference | [-311127, +724370]ns | [149149127, 150179994] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 148402687ns | +0.2% | +0.9% | -0.3% | +2.0% |
| 1 | 2 | 149449680ns | -0.5% | -0.3% | +0.0% | -0.2% |
| 1 | 3 | 149232046ns | +0.1% | -0.5% | -0.3% | +0.9% |
| 2 | 1 | 149951666ns | -0.7% | -0.7% | -0.0% | -0.0% |
| 2 | 2 | 149554331ns | -0.1% | +0.8% | +0.1% | -0.2% |
| 2 | 3 | 148669762ns | -0.3% | +0.2% | -0.4% | +0.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.061 | ok |
| riv_rowm | -0.395 | moderate- |
| riv_scr | -0.206 | moderate- |
| riv_snake | -0.215 | moderate- |
| riv_spec | -0.208 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 2/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 3/6, lost 1/6
- **riv_spec**: won 2/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 739579956.7ns | 149646361.2ns | 494.2% | HIGH |
| riv_rowm | 738525995.1ns | 149468990.5ns | 494.1% | HIGH |
| riv_scr | 739731705.7ns | 150107088.9ns | 492.8% | HIGH |
| riv_snake | 740244701.4ns | 149203972.9ns | 496.1% | HIGH |
| riv_spec | 739285727.7ns | 150547778.9ns | 491.1% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 148383761.2-150867106.1 ns)
  148383761.2 |#############
  148507928.5 |#############
  148632095.7 |##########################
  148756263.0 |
  148880430.2 |
  149004597.4 |#############
  149128764.7 |#############
  149252931.9 |
  149377099.2 |########################################
  149501266.4 |#############
  149625433.7 |
  149749600.9 |#############
  149873768.1 |#############
  149997935.4 |#############
  150122102.6 |#############
  150246269.9 |
  150370437.1 |#############
  150494604.3 |
  150618771.6 |
  150742938.8 |
  (1 below, 2 above range)

riv_rowm (n=18, range 148098802.4-151016730.9 ns)
  148098802.4 |#############
  148244698.8 |
  148390595.2 |
  148536491.7 |########################################
  148682388.1 |##########################
  148828284.5 |##########################
  148974180.9 |
  149120077.4 |
  149265973.8 |#############
  149411870.2 |##########################
  149557766.6 |##########################
  149703663.1 |
  149849559.5 |
  149995455.9 |
  150141352.3 |
  150287248.8 |
  150433145.2 |
  150579041.6 |
  150724938.0 |
  150870834.4 |
  (2 below, 3 above range)

riv_scr (n=18, range 148427747.0-152558411.6 ns)
  148427747.0 |################
  148634280.2 |########
  148840813.5 |########################################
  149047346.7 |################
  149253879.9 |
  149460413.2 |
  149666946.4 |################
  149873479.6 |
  150080012.8 |
  150286546.1 |########
  150493079.3 |
  150699612.5 |########
  150906145.8 |
  151112679.0 |########
  151319212.2 |
  151525745.5 |
  151732278.7 |########
  151938811.9 |
  152145345.2 |
  152351878.4 |
  (1 below, 1 above range)

riv_snake (n=18, range 147810783.1-150303156.9 ns)
  147810783.1 |####################
  147935401.8 |########################################
  148060020.5 |
  148184639.1 |
  148309257.8 |
  148433876.5 |####################
  148558495.2 |
  148683113.9 |########################################
  148807732.6 |
  148932351.3 |
  149056970.0 |########################################
  149181588.7 |####################
  149306207.4 |
  149430826.1 |####################
  149555444.8 |####################
  149680063.5 |########################################
  149804682.1 |########################################
  149929300.8 |
  150053919.5 |
  150178538.2 |
  (1 below, 2 above range)

riv_spec (n=18, range 148621971.2-153483501.9 ns)
  148621971.2 |
  148865047.7 |###########
  149108124.3 |########################################
  149351200.8 |
  149594277.3 |#####
  149837353.9 |#####
  150080430.4 |
  150323507.0 |#####
  150566583.5 |#####
  150809660.0 |
  151052736.6 |
  151295813.1 |#####
  151538889.6 |
  151781966.2 |
  152025042.7 |
  152268119.2 |
  152511195.8 |
  152754272.3 |
  152997348.8 |
  153240425.4 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=494.7% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=495.2% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=497.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=495.8% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=495.0% of algo (FFI overhead may distort results)
