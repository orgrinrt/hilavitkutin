# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole field within 2.6% of the fastest

All 5 variants sit between 791.80 us and 812.37 us - a 2.6% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_spec** at 791804.2 ns median (-1.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.03x (fastest 791804.2 ns, slowest 812366.7 ns)

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
| riv_rcm | 821771ns | 807277ns | 797812ns | 806507ns | 856645ns | base |
| riv_rowm | 811856ns | 808090ns | 802900ns | 807450ns | 822942ns | -1.21% |
| riv_scr | 820949ns | 817325ns | 811893ns | 816857ns | 831614ns | -0.10% |
| riv_snake | 807535ns | 796750ns | 790533ns | 796311ns | 832872ns | -1.73% |
| riv_spec | 800005ns | 796450ns | 791371ns | 795963ns | 810384ns | -2.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 817135ns | 793076ns | 852198ns | base | 0.000 |
| riv_rowm | 807146ns | 798029ns | 818173ns | -1.22% | 0.000 |
| riv_scr | 816249ns | 807581ns | 826911ns | -0.11% | 0.000 |
| riv_snake | 802772ns | 785650ns | 828080ns | -1.76% | 0.000 |
| riv_spec | 795255ns | 786626ns | 805617ns | -2.68% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_snake; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.9% |
| riv_rowm | 0.000 | 97.7% |
| riv_scr | 0.000 | 96.7% |
| riv_snake | 0.000 | 99.2% |
| riv_spec | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 821771ns | 821771ns | base |
| riv_rowm | 811856ns | 811856ns | -1.21% |
| riv_scr | 820949ns | 820949ns | -0.10% |
| riv_snake | 807535ns | 807535ns | -1.73% |
| riv_spec | 800005ns | 800005ns | -2.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 802342ns | base | --- | [796629, 827648] | --- | --- | --- | --- |
| riv_rowm | 803756ns | no significant difference | [-18969, +13356]ns | [801062, 808308] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 812367ns | no significant difference | [-6890, +18106]ns | [811410, 815377] | no | 0.6409 | 0.4807 | 0 |
| riv_snake | 792246ns | -9031.2ns (-1.1%) | [-34250, -458]ns | [789506, 811250] | YES (adj: no) | 0.1925 | 0.0963 | 0 |
| riv_spec | 791804ns | -12572.9ns (-1.6%) | [-32727, -2467]ns | [789356, 796288] | YES (adj: no) | 0.1235 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 798812ns | +0.0% | +1.9% | +2.8% | -1.6% |
| 1 | 2 | 794738ns | +1.5% | +2.2% | +0.4% | -0.1% |
| 1 | 3 | 809871ns | -0.7% | +0.3% | -2.3% | -1.9% |
| 2 | 1 | 833138ns | -3.4% | -2.3% | -5.1% | -5.1% |
| 2 | 2 | 799896ns | +0.1% | +1.4% | -0.6% | -0.9% |
| 2 | 3 | 794479ns | +1.1% | +1.6% | -1.1% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.053 | ok |
| riv_rowm | -0.295 | moderate- |
| riv_scr | 0.103 | ok |
| riv_snake | 0.112 | ok |
| riv_spec | -0.151 | ok |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 3/6
- **riv_scr**: won 1/6, lost 5/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 4/6, lost 1/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 16757883.8ns | 817135.2ns | 2050.8% | HIGH |
| riv_rowm | 16739963.7ns | 807145.8ns | 2074.0% | HIGH |
| riv_scr | 16728313.4ns | 816249.3ns | 2049.4% | HIGH |
| riv_snake | 16741401.6ns | 802772.2ns | 2085.4% | HIGH |
| riv_spec | 16737339.6ns | 795254.9ns | 2104.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 793076.4-852197.9 ns)
  793076.4 |########################################
  796032.5 |##########################
  798988.6 |##########################
  801944.6 |#############
  804900.7 |
  807856.8 |#############
  810812.9 |
  813768.9 |#############
  816725.0 |
  819681.1 |#############
  822637.2 |
  825593.2 |
  828549.3 |
  831505.4 |#############
  834461.5 |#############
  837417.6 |
  840373.6 |
  843329.7 |
  846285.8 |
  849241.9 |
  (2 below, 3 above range)

riv_rowm (n=18, range 798029.2-818172.9 ns)
  798029.2 |
  799036.4 |####################
  800043.5 |########################################
  801050.7 |########################################
  802057.9 |####################
  803065.1 |####################
  804072.3 |########################################
  805079.5 |
  806086.7 |########################################
  807093.8 |
  808101.0 |
  809108.2 |####################
  810115.4 |
  811122.6 |
  812129.8 |
  813137.0 |
  814144.2 |####################
  815151.3 |
  816158.5 |
  817165.7 |
  (2 below, 3 above range)

riv_scr (n=18, range 807580.6-826911.1 ns)
  807580.6 |
  808547.1 |
  809513.6 |##########
  810480.1 |####################
  811446.7 |########################################
  812413.2 |####################
  813379.7 |####################
  814346.3 |
  815312.8 |
  816279.3 |##########
  817245.8 |
  818212.4 |
  819178.9 |
  820145.4 |
  821112.0 |
  822078.5 |##########
  823045.0 |
  824011.5 |##########
  824978.1 |
  825944.6 |
  (2 below, 2 above range)

riv_snake (n=18, range 785650.0-828079.9 ns)
  785650.0 |##############################
  787771.5 |##########
  789893.0 |########################################
  792014.5 |##########
  794136.0 |##########
  796257.5 |##########
  798379.0 |
  800500.5 |
  802622.0 |##########
  804743.5 |
  806864.9 |
  808986.4 |
  811107.9 |
  813229.4 |
  815350.9 |
  817472.4 |##########
  819593.9 |##########
  821715.4 |
  823836.9 |##########
  825958.4 |
  (1 below, 2 above range)

riv_spec (n=18, range 786626.4-805616.7 ns)
  786626.4 |
  787575.9 |#############
  788525.4 |##########################
  789474.9 |#############
  790424.5 |########################################
  791374.0 |
  792323.5 |#############
  793273.0 |#############
  794222.5 |#############
  795172.0 |#############
  796121.5 |
  797071.1 |#############
  798020.6 |
  798970.1 |#############
  799919.6 |
  800869.1 |
  801818.6 |#############
  802768.1 |
  803717.7 |
  804667.2 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=2084.5% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=2080.5% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=2058.7% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2110.9% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2112.0% of algo (FFI overhead may distort results)
