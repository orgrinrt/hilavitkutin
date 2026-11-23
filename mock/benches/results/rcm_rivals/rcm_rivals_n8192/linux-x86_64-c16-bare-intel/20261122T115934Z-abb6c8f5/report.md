# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (riv_rcm) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline riv_rcm has the worst median (75.29 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest riv_scr at 74.83 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### Whole field within 0.6% of the fastest

All 5 variants sit between 74.83 ms and 75.29 ms - a 0.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_scr** at 74826321.8 ns median (-0.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.01x (fastest 74826321.8 ns, slowest 75285862.7 ns)

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
| Counter frequency | 2112741408 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 75944785ns | 75289612ns | 74621970ns | 75104289ns | 77866937ns | base |
| riv_rowm | 75422302ns | 74892915ns | 74294450ns | 74724882ns | 77032358ns | -0.69% |
| riv_scr | 74949510ns | 74830782ns | 74677815ns | 74808342ns | 75297109ns | -1.31% |
| riv_snake | 75013794ns | 74972081ns | 74624299ns | 74886362ns | 75399690ns | -1.23% |
| riv_spec | 75233889ns | 74963781ns | 74712783ns | 74900710ns | 75994209ns | -0.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 75940718ns | 74617940ns | 77862787ns | base | 0.000 |
| riv_rowm | 75418199ns | 74290325ns | 77028251ns | -0.69% | 0.000 |
| riv_scr | 74945295ns | 74673711ns | 75293060ns | -1.31% | 0.000 |
| riv_snake | 75009710ns | 74620371ns | 75395527ns | -1.23% | 0.000 |
| riv_spec | 75229816ns | 74708685ns | 75990002ns | -0.94% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 98.7% |
| riv_rowm | 0.000 | 99.2% |
| riv_scr | 0.000 | 99.3% |
| riv_snake | 0.000 | 99.1% |
| riv_spec | 0.000 | 99.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 75944785ns | 75944785ns | base |
| riv_rowm | 75422302ns | 75422302ns | -0.69% |
| riv_scr | 74949510ns | 74949510ns | -1.31% |
| riv_snake | 75013794ns | 75013794ns | -1.23% |
| riv_spec | 75233889ns | 75233889ns | -0.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 75285863ns | base | --- | [74792219, 75476164] | --- | --- | --- | --- |
| riv_rowm | 74888790ns | -388028.7ns (-0.5%) | [-603371, -109505]ns | [74445228, 75142666] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| riv_scr | 74826322ns | no significant difference | [-646753, +46270]ns | [74745003, 75044473] | no | 0.3172 | 0.2379 | 0 |
| riv_snake | 74967934ns | -214740.6ns (-0.3%) | [-503096, -54400]ns | [74786670, 75117826] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| riv_spec | 74959738ns | no significant difference | [-681511, +119605]ns | [74816703, 75247531] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 75474599ns | -0.4% | +0.0% | -0.3% | -0.3% |
| 1 | 2 | 74800465ns | -0.7% | -0.1% | -0.1% | +0.3% |
| 1 | 3 | 74703133ns | -0.1% | +0.0% | -0.1% | -0.0% |
| 2 | 1 | 75468250ns | -0.6% | -0.9% | -0.7% | -0.9% |
| 2 | 2 | 75277458ns | +3.5% | -0.2% | -0.5% | -0.6% |
| 2 | 3 | 74670294ns | -0.3% | +0.2% | +0.5% | +0.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.214 | moderate- |
| riv_rowm | -0.224 | moderate- |
| riv_scr | -0.213 | moderate- |
| riv_snake | -0.128 | ok |
| riv_spec | 0.229 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 1/6
- **riv_scr**: won 3/6, lost 1/6
- **riv_snake**: won 3/6, lost 1/6
- **riv_spec**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 369771026.5ns | 75940717.9ns | 486.9% | HIGH |
| riv_rowm | 370081720.9ns | 75418198.5ns | 490.7% | HIGH |
| riv_scr | 369359060.2ns | 74945294.8ns | 492.8% | HIGH |
| riv_snake | 369172939.2ns | 75009710.0ns | 492.2% | HIGH |
| riv_spec | 370211218.2ns | 75229815.6ns | 492.1% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 74617940.1-77862786.8 ns)
  74617940.1 |##############################
  74780182.4 |##############################
  74942424.8 |
  75104667.1 |##########
  75266909.4 |####################
  75429151.8 |########################################
  75591394.1 |
  75753636.4 |
  75915878.8 |
  76078121.1 |##########
  76240363.4 |
  76402605.8 |
  76564848.1 |
  76727090.4 |
  76889332.8 |
  77051575.1 |
  77213817.4 |
  77376059.8 |
  77538302.1 |
  77700544.4 |
  (1 below, 3 above range)

riv_rowm (n=18, range 74290324.7-77028250.7 ns)
  74290324.7 |##############################
  74427221.0 |##########
  74564117.3 |####################
  74701013.6 |##########
  74837909.9 |
  74974806.2 |########################################
  75111702.5 |##########
  75248598.8 |##########
  75385495.1 |
  75522391.4 |
  75659287.7 |
  75796184.0 |
  75933080.3 |
  76069976.6 |
  76206872.9 |
  76343769.2 |
  76480665.5 |
  76617561.8 |
  76754458.1 |
  76891354.4 |
  (2 below, 3 above range)

riv_scr (n=18, range 74673710.6-75293060.0 ns)
  74673710.6 |##########################
  74704678.1 |##########################
  74735645.6 |##########################
  74766613.0 |
  74797580.5 |########################################
  74828548.0 |#############
  74859515.5 |
  74890482.9 |
  74921450.4 |#############
  74952417.9 |
  74983385.3 |#############
  75014352.8 |
  75045320.3 |
  75076287.7 |
  75107255.2 |
  75138222.7 |##########################
  75169190.2 |
  75200157.6 |
  75231125.1 |
  75262092.6 |
  (1 below, 3 above range)

riv_snake (n=18, range 74620370.9-75395526.9 ns)
  74620370.9 |########################################
  74659128.7 |####################
  74697886.5 |
  74736644.3 |####################
  74775402.1 |
  74814159.9 |####################
  74852917.7 |########################################
  74891675.5 |
  74930433.3 |####################
  74969191.1 |####################
  75007948.9 |########################################
  75046706.7 |####################
  75085464.5 |
  75124222.3 |
  75162980.1 |########################################
  75201737.9 |####################
  75240495.7 |####################
  75279253.5 |
  75318011.3 |
  75356769.1 |
  (1 below, 1 above range)

riv_spec (n=18, range 74708684.6-75990002.4 ns)
  74708684.6 |
  74772750.5 |########################################
  74836816.4 |########
  74900882.2 |########
  74964948.1 |################
  75029014.0 |########
  75093079.9 |
  75157145.8 |
  75221211.7 |################
  75285277.6 |########
  75349343.5 |########
  75413409.4 |
  75477475.3 |
  75541541.2 |
  75605607.1 |
  75669673.0 |
  75733738.8 |
  75797804.7 |
  75861870.6 |
  75925936.5 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=489.9% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=494.3% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=492.7% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=492.2% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=493.8% of algo (FFI overhead may distort results)
