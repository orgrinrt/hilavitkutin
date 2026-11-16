# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_snake shows alternating (throttle bounce) (autocorr -0.51)

riv_snake's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 2.9% of the fastest

All 5 variants sit between 812.33 us and 835.81 us - a 2.9% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: riv_snake** at 812331.2 ns median (-2.2% vs baseline)
- 2 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 812331.2 ns, slowest 835812.5 ns)

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
| riv_rcm | 835634ns | 835431ns | 830299ns | 834228ns | 840410ns | base |
| riv_rowm | 836348ns | 835092ns | 826739ns | 833313ns | 845706ns | +0.09% |
| riv_scr | 842184ns | 840212ns | 832929ns | 840019ns | 850058ns | +0.78% |
| riv_snake | 817673ns | 816829ns | 810969ns | 816206ns | 823225ns | -2.15% |
| riv_spec | 823604ns | 821440ns | 811215ns | 819207ns | 836394ns | -1.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 831206ns | 825774ns | 836126ns | base | 0.000 |
| riv_rowm | 831874ns | 822193ns | 841331ns | +0.08% | 0.000 |
| riv_scr | 837635ns | 828597ns | 845462ns | +0.77% | 0.000 |
| riv_snake | 813180ns | 806512ns | 818623ns | -2.17% | 0.000 |
| riv_spec | 819139ns | 806465ns | 831866ns | -1.45% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_spec; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.1% |
| riv_rowm | 0.000 | 97.1% |
| riv_scr | 0.000 | 96.5% |
| riv_snake | 0.000 | 99.3% |
| riv_spec | 0.000 | 98.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 835634ns | 835634ns | base |
| riv_rowm | 836348ns | 836348ns | +0.09% |
| riv_scr | 842184ns | 842184ns | +0.78% |
| riv_snake | 817673ns | 817673ns | -2.15% |
| riv_spec | 823604ns | 823604ns | -1.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 830958ns | base | --- | [828567, 833298] | --- | --- | --- | --- |
| riv_rowm | 830388ns | no significant difference | [-4152, +4394]ns | [826817, 832677] | no | 0.2379 | 0.2379 | 0 |
| riv_scr | 835812ns | +5843.7ns (+0.7%) | [+21, +12410]ns | [833142, 840510] | YES (adj: no) | 0.1283 | 0.0963 | 0 |
| riv_snake | 812331ns | -17341.7ns (-2.1%) | [-20658, -14296]ns | [810379, 815694] | YES | 0.0000 | 0.0000 | 0 |
| riv_spec | 817162ns | -16014.6ns (-1.9%) | [-19044, -12104]ns | [811290, 819598] | YES | 0.0003 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 838017ns | -0.0% | +0.5% | -3.2% | -2.3% |
| 1 | 2 | 828879ns | -0.5% | +0.7% | -0.8% | -1.6% |
| 1 | 3 | 831225ns | -0.2% | +1.2% | -2.5% | -1.4% |
| 2 | 1 | 831842ns | -0.3% | -0.0% | -2.4% | -2.1% |
| 2 | 2 | 827104ns | +0.6% | +0.5% | -1.4% | -1.0% |
| 2 | 3 | 828804ns | +2.4% | +1.3% | -1.8% | -2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.134 | ok |
| riv_rowm | 0.002 | ok |
| riv_scr | -0.270 | moderate- |
| riv_snake | -0.513 | HIGH- (thermal bounce) |
| riv_spec | -0.399 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 3/6, lost 2/6
- **riv_scr**: won 0/6, lost 5/6
- **riv_snake**: won 6/6, lost 0/6
- **riv_spec**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 16784655.3ns | 831205.8ns | 2019.3% | HIGH |
| riv_rowm | 16839606.5ns | 831873.6ns | 2024.3% | HIGH |
| riv_scr | 16796660.2ns | 837635.2ns | 2005.2% | HIGH |
| riv_snake | 16834682.6ns | 813180.3ns | 2070.2% | HIGH |
| riv_spec | 16805952.8ns | 819138.7ns | 2051.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 825773.6-836125.7 ns)
  825773.6 |
  826291.2 |
  826808.8 |##############################
  827326.4 |
  827844.0 |
  828361.7 |########################################
  828879.3 |
  829396.9 |
  829914.5 |
  830432.1 |##########
  830949.7 |##########
  831467.3 |####################
  831984.9 |
  832502.5 |##########
  833020.1 |
  833537.7 |##########
  834055.3 |
  834572.9 |##########
  835090.5 |##########
  835608.1 |
  (1 below, 2 above range)

riv_rowm (n=18, range 822193.0-841330.5 ns)
  822193.0 |####################
  823149.9 |####################
  824106.8 |####################
  825063.7 |####################
  826020.5 |####################
  826977.4 |
  827934.3 |
  828891.2 |########################################
  829848.0 |####################
  830804.9 |########################################
  831761.8 |########################################
  832718.7 |####################
  833675.5 |
  834632.4 |
  835589.3 |
  836546.2 |
  837503.0 |####################
  838459.9 |####################
  839416.8 |
  840373.7 |
  (1 below, 2 above range)

riv_scr (n=18, range 828597.2-845461.8 ns)
  828597.2 |#############
  829440.5 |
  830283.7 |
  831126.9 |##########################
  831970.1 |#############
  832813.4 |
  833656.6 |#############
  834499.8 |########################################
  835343.1 |
  836186.3 |#############
  837029.5 |
  837872.7 |
  838716.0 |
  839559.2 |########################################
  840402.4 |#############
  841245.7 |#############
  842088.9 |#############
  842932.1 |
  843775.3 |
  844618.6 |
  (1 below, 2 above range)

riv_snake (n=18, range 806512.5-818622.9 ns)
  806512.5 |
  807118.0 |########################################
  807723.5 |
  808329.1 |####################
  808934.6 |
  809540.1 |####################
  810145.6 |####################
  810751.2 |####################
  811356.7 |########################################
  811962.2 |
  812567.7 |####################
  813173.2 |
  813778.8 |####################
  814384.3 |
  814989.8 |####################
  815595.3 |########################################
  816200.8 |
  816806.4 |####################
  817411.9 |
  818017.4 |####################
  (1 below, 2 above range)

riv_spec (n=18, range 806465.3-831866.0 ns)
  806465.3 |#############
  807735.3 |#############
  809005.3 |#############
  810275.4 |#############
  811545.4 |#############
  812815.4 |
  814085.5 |#############
  815355.5 |##########################
  816625.5 |
  817895.6 |########################################
  819165.6 |##########################
  820435.7 |
  821705.7 |#############
  822975.7 |
  824245.8 |#############
  825515.8 |
  826785.8 |#############
  828055.9 |
  829325.9 |
  830595.9 |
  (1 below, 1 above range)

```

## Diagnostics

- **riv_rcm**: bridge=2019.3% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=2020.8% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=2008.3% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=2065.3% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=2053.1% of algo (FFI overhead may distort results)
