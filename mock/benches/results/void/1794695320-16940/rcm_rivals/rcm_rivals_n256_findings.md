# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

No notable statistical pattern fired: the variants do not separate meaningfully on this run.

## Key findings

- **Fastest: riv_snake** at 805368.8 ns median (-2.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 805368.8 ns, slowest 845937.5 ns)

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
| riv_rcm | 832857ns | 829215ns | 808319ns | 826254ns | 855032ns | base |
| riv_rowm | 833026ns | 825052ns | 813450ns | 822223ns | 859019ns | +0.02% |
| riv_scr | 867037ns | 850333ns | 819742ns | 846498ns | 921492ns | +4.10% |
| riv_snake | 820056ns | 810038ns | 799801ns | 809887ns | 845437ns | -1.54% |
| riv_spec | 825833ns | 820581ns | 790710ns | 812975ns | 862681ns | -0.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 828188ns | 803485ns | 850365ns | base | 0.000 |
| riv_rowm | 828363ns | 808281ns | 854315ns | +0.02% | 0.000 |
| riv_scr | 862301ns | 815017ns | 916411ns | +4.12% | 0.000 |
| riv_snake | 815369ns | 795239ns | 840904ns | -1.55% | 0.000 |
| riv_spec | 821224ns | 785792ns | 857876ns | -0.84% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 95.3% |
| riv_rowm | 0.000 | 95.8% |
| riv_scr | 0.000 | 92.9% |
| riv_snake | 0.000 | 97.6% |
| riv_spec | 0.000 | 96.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 832857ns | 832857ns | base |
| riv_rowm | 833026ns | 833026ns | +0.02% |
| riv_scr | 867037ns | 867037ns | +4.10% |
| riv_snake | 820056ns | 820056ns | -1.54% |
| riv_spec | 825833ns | 825833ns | -0.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 824915ns | base | --- | [817144, 831035] | --- | --- | --- | --- |
| riv_rowm | 820604ns | no significant difference | [-12512, +8325]ns | [813515, 835712] | no | 0.8145 | 0.8145 | 0 |
| riv_scr | 845938ns | +31175.0ns (+3.8%) | [+3375, +53469]ns | [833188, 877438] | YES (adj: no) | 0.3172 | 0.0963 | 0 |
| riv_snake | 805369ns | no significant difference | [-26729, +1200]ns | [802131, 819365] | no | 0.3172 | 0.2379 | 0 |
| riv_spec | 816523ns | no significant difference | [-31208, +16479]ns | [795717, 837892] | no | 0.3172 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 821833ns | -1.4% | +6.4% | -2.4% | -3.7% |
| 1 | 2 | 802250ns | +2.9% | +4.4% | +0.2% | +2.4% |
| 1 | 3 | 830329ns | +1.6% | -0.1% | -2.2% | -2.3% |
| 2 | 1 | 833288ns | -0.9% | +0.8% | -3.7% | -0.7% |
| 2 | 2 | 823725ns | -1.3% | +3.4% | -2.4% | -4.0% |
| 2 | 3 | 845725ns | -3.5% | +6.6% | -0.2% | +2.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.030 | ok |
| riv_rowm | 0.170 | ok |
| riv_scr | 0.119 | ok |
| riv_snake | -0.093 | ok |
| riv_spec | -0.457 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 4/6, lost 2/6
- **riv_scr**: won 0/6, lost 5/6
- **riv_snake**: won 5/6, lost 1/6
- **riv_spec**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 16752657.6ns | 828188.2ns | 2022.8% | HIGH |
| riv_rowm | 16762053.9ns | 828363.2ns | 2023.5% | HIGH |
| riv_scr | 16818776.9ns | 862300.7ns | 1950.5% | HIGH |
| riv_snake | 16820969.9ns | 815369.4ns | 2063.0% | HIGH |
| riv_spec | 16849465.7ns | 821224.3ns | 2051.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 803484.7-850364.6 ns)
  803484.7 |
  805828.7 |
  808172.7 |
  810516.7 |########################################
  812860.7 |
  815204.7 |####################
  817548.7 |####################
  819892.7 |####################
  822236.7 |########################################
  824580.7 |########################################
  826924.7 |####################
  829268.7 |####################
  831612.7 |########################################
  833956.6 |
  836300.6 |
  838644.6 |
  840988.6 |
  843332.6 |####################
  845676.6 |####################
  848020.6 |
  (2 below, 1 above range)

riv_rowm (n=18, range 808280.6-854314.6 ns)
  808280.6 |########################################
  810582.3 |
  812884.0 |########################################
  815185.7 |##########################
  817487.4 |
  819789.1 |
  822090.8 |
  824392.5 |########################################
  826694.2 |#############
  828995.9 |
  831297.6 |
  833599.3 |
  835901.0 |
  838202.7 |
  840504.4 |#############
  842806.1 |#############
  845107.8 |
  847409.5 |
  849711.2 |
  852012.9 |
  (1 below, 3 above range)

riv_scr (n=18, range 815016.7-916411.1 ns)
  815016.7 |##############################
  820086.4 |
  825156.1 |##########
  830225.8 |
  835295.6 |########################################
  840365.3 |
  845435.0 |
  850504.7 |##########
  855574.4 |##########
  860644.2 |
  865713.9 |##########
  870783.6 |##########
  875853.3 |##########
  880923.1 |
  885992.8 |
  891062.5 |
  896132.2 |##########
  901201.9 |
  906271.7 |
  911341.4 |
  (1 below, 3 above range)

riv_snake (n=18, range 795238.9-840904.2 ns)
  795238.9 |
  797522.1 |
  799805.4 |##############################
  802088.7 |########################################
  804371.9 |##########
  806655.2 |
  808938.5 |
  811221.7 |####################
  813505.0 |
  815788.3 |
  818071.5 |####################
  820354.8 |
  822638.1 |
  824921.3 |
  827204.6 |
  829487.9 |
  831771.1 |
  834054.4 |##########
  836337.7 |
  838620.9 |
  (2 below, 3 above range)

riv_spec (n=18, range 785791.7-857875.7 ns)
  785791.7 |
  789395.9 |########################################
  793000.1 |
  796604.3 |#############
  800208.5 |#############
  803812.7 |
  807416.9 |#############
  811021.1 |#############
  814625.3 |
  818229.5 |#############
  821833.7 |
  825437.9 |########################################
  829042.1 |
  832646.3 |
  836250.5 |
  839854.7 |
  843458.9 |
  847063.1 |##########################
  850667.3 |
  854271.5 |
  (2 below, 3 above range)

```

## Diagnostics

- **riv_rcm**: bridge=2029.3% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=2040.7% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=1980.4% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2079.6% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2050.8% of algo (FFI overhead may distort results)
