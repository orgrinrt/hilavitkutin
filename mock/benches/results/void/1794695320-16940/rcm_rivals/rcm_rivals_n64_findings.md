# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.9% (fastest 181.29 us to slowest 182.98 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1.69 us) is smaller than the fastest variant's own run-to-run std-dev (3.29 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: riv_scr** at 181287.5 ns median (-0.8% vs baseline)
- Spread: 1.01x (fastest 181287.5 ns, slowest 182981.2 ns)

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
| riv_rcm | 190448ns | 185131ns | 182118ns | 184726ns | 203197ns | base |
| riv_rowm | 188709ns | 186648ns | 182465ns | 185503ns | 196640ns | -0.91% |
| riv_scr | 185285ns | 184140ns | 181622ns | 183783ns | 189369ns | -2.71% |
| riv_snake | 190353ns | 185383ns | 182481ns | 185619ns | 201392ns | -0.05% |
| riv_spec | 186888ns | 184556ns | 182321ns | 184300ns | 193054ns | -1.87% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 187469ns | 179638ns | 199984ns | base | 0.000 |
| riv_rowm | 185600ns | 179611ns | 193556ns | -1.00% | 0.000 |
| riv_scr | 182435ns | 179240ns | 186305ns | -2.69% | 0.000 |
| riv_snake | 187039ns | 179256ns | 197724ns | -0.23% | 0.000 |
| riv_spec | 183844ns | 179478ns | 189698ns | -1.93% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_scr; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.1% |
| riv_rowm | 0.000 | 98.1% |
| riv_scr | 0.000 | 98.9% |
| riv_snake | 0.000 | 98.0% |
| riv_spec | 0.000 | 98.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 190448ns | 190448ns | base |
| riv_rowm | 188709ns | 188709ns | -0.91% |
| riv_scr | 185285ns | 185285ns | -2.71% |
| riv_snake | 190353ns | 190353ns | -0.05% |
| riv_spec | 186888ns | 186888ns | -1.87% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 182762ns | base | --- | [180015, 187654] | --- | --- | --- | --- |
| riv_rowm | 182704ns | no significant difference | [-2788, +6073]ns | [180956, 189465] | no | 1.0000 | 0.8145 | 0 |
| riv_scr | 181288ns | no significant difference | [-5346, +1875]ns | [180202, 183090] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 182981ns | no significant difference | [-2706, +3404]ns | [180938, 188894] | no | 1.0000 | 0.8145 | 0 |
| riv_spec | 181719ns | no significant difference | [-6275, +1444]ns | [181048, 182492] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 180204ns | -0.0% | -0.0% | -0.3% | -0.5% |
| 1 | 2 | 179933ns | +2.2% | +0.9% | -0.1% | +0.7% |
| 1 | 3 | 186288ns | -2.5% | -2.8% | -2.0% | -1.7% |
| 2 | 1 | 184550ns | +3.2% | +2.4% | +2.2% | -2.4% |
| 2 | 2 | 180196ns | +0.2% | -0.6% | +2.0% | +3.0% |
| 2 | 3 | 200496ns | -9.2% | -9.0% | -4.3% | -9.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.168 | ok |
| riv_rowm | -0.417 | moderate- |
| riv_scr | -0.451 | moderate- |
| riv_snake | 0.140 | ok |
| riv_spec | -0.339 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 3/6
- **riv_scr**: won 3/6, lost 2/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 4528747.4ns | 187469.0ns | 2415.7% | HIGH |
| riv_rowm | 4215054.9ns | 185599.5ns | 2271.0% | HIGH |
| riv_scr | 4212353.0ns | 182435.0ns | 2309.0% | HIGH |
| riv_snake | 4228277.8ns | 187039.4ns | 2260.6% | HIGH |
| riv_spec | 4190897.0ns | 183843.8ns | 2279.6% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 179637.5-199984.0 ns)
  179637.5 |########################################
  180654.8 |
  181672.1 |#####
  182689.5 |
  183706.8 |###########
  184724.1 |#####
  185741.5 |#####
  186758.8 |
  187776.1 |
  188793.4 |#####
  189810.8 |
  190828.1 |
  191845.4 |
  192862.7 |#####
  193880.0 |
  194897.4 |
  195914.7 |
  196932.0 |
  197949.4 |
  198966.7 |
  (1 below, 3 above range)

riv_rowm (n=18, range 179611.1-193556.2 ns)
  179611.1 |##########################
  180308.4 |#############
  181005.6 |########################################
  181702.9 |#############
  182400.1 |
  183097.4 |#############
  183794.6 |#############
  184491.9 |
  185189.2 |
  185886.4 |#############
  186583.7 |
  187280.9 |
  187978.2 |
  188675.4 |#############
  189372.7 |
  190070.0 |##########################
  190767.2 |
  191464.5 |
  192161.7 |
  192859.0 |
  (2 below, 3 above range)

riv_scr (n=18, range 179240.3-186304.9 ns)
  179240.3 |#############
  179593.5 |
  179946.7 |########################################
  180300.0 |#############
  180653.2 |##########################
  181006.4 |
  181359.6 |#############
  181712.9 |#############
  182066.1 |#############
  182419.3 |#############
  182772.6 |
  183125.8 |
  183479.0 |#############
  183832.3 |
  184185.5 |#############
  184538.7 |
  184891.9 |
  185245.2 |
  185598.4 |
  185951.6 |
  (2 below, 3 above range)

riv_snake (n=18, range 179255.6-197724.3 ns)
  179255.6 |##########################
  180179.0 |##########################
  181102.4 |
  182025.9 |########################################
  182949.3 |#############
  183872.8 |#############
  184796.2 |
  185719.6 |#############
  186643.1 |
  187566.5 |
  188489.9 |##########################
  189413.4 |#############
  190336.8 |
  191260.3 |#############
  192183.7 |
  193107.1 |
  194030.6 |
  194954.0 |
  195877.4 |
  196800.9 |
  (2 below, 2 above range)

riv_spec (n=18, range 179477.8-189697.9 ns)
  179477.8 |
  179988.8 |#############
  180499.8 |######
  181010.8 |######
  181521.8 |########################################
  182032.8 |######
  182543.8 |#############
  183054.8 |
  183565.9 |
  184076.9 |
  184587.9 |
  185098.9 |######
  185609.9 |
  186120.9 |
  186631.9 |
  187142.9 |
  187653.9 |
  188164.9 |
  188675.9 |
  189186.9 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=2293.2% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=2291.5% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=2309.9% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2290.1% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2300.0% of algo (FFI overhead may distort results)
