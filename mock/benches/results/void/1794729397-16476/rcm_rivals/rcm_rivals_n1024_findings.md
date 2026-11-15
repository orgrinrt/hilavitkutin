# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### riv_snake is fastest but the noisiest (CV 10.3%)

riv_snake wins on median (9.45 ms) yet has the highest variance (CV 10.3%), while riv_rcm is the steadiest (CV 1.2%, 9.52 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (riv_snake, riv_rowm) are a dead heat (<1%)

riv_snake (9.45 ms) and riv_rowm (9.46 ms) differ by 0.13%, inside the noise, even though the wider field spreads 6.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### riv_scr shows alternating (throttle bounce) (autocorr -0.54)

riv_scr's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (612.43 us) is smaller than the fastest variant's own run-to-run std-dev (974.62 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader riv_snake vs stability leader riv_rcm (+1% speed for 8.8x steadier)

riv_snake is fastest (9.45 ms, CV 10.3%); riv_rcm gives up 0.7% median for 8.8x lower variance (CV 1.2%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: riv_snake** at 9448629.2 ns median (-0.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 1.06x (fastest 9448629.2 ns, slowest 10061060.6 ns)

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
| Counter frequency | 2112802731 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 9535922ns | 9523345ns | 9393152ns | 9496283ns | 9666767ns | base |
| riv_rowm | 9613190ns | 9464792ns | 9302973ns | 9458842ns | 9999820ns | +0.81% |
| riv_scr | 10479594ns | 10065053ns | 9527122ns | 10084948ns | 11547799ns | +9.90% |
| riv_snake | 10126523ns | 9452535ns | 9357199ns | 9467739ns | 11499360ns | +6.19% |
| riv_spec | 9921745ns | 9589129ns | 9537686ns | 9582040ns | 10623331ns | +4.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 9531961ns | 9389252ns | 9662759ns | base | 0.000 |
| riv_rowm | 9609100ns | 9298752ns | 9995644ns | +0.81% | 0.000 |
| riv_scr | 10475590ns | 9523245ns | 11543800ns | +9.90% | 0.000 |
| riv_snake | 10122555ns | 9353334ns | 11495370ns | +6.20% | 0.000 |
| riv_spec | 9917768ns | 9533712ns | 10619341ns | +4.05% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 97.7% |
| riv_rowm | 0.000 | 98.3% |
| riv_scr | 0.000 | 92.4% |
| riv_snake | 0.000 | 98.4% |
| riv_spec | 0.000 | 97.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 9535922ns | 9535922ns | base |
| riv_rowm | 9613190ns | 9613190ns | +0.81% |
| riv_scr | 10479594ns | 10479594ns | +9.90% |
| riv_snake | 10126523ns | 10126523ns | +6.19% |
| riv_spec | 9921745ns | 9921745ns | +4.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 9519188ns | base | --- | [9447995, 9585997] | --- | --- | --- | --- |
| riv_rowm | 9460786ns | -63839.3ns (-0.7%) | [-155711, -7197]ns | [9434661, 9533248] | YES (adj: no) | 0.0618 | 0.0309 | 0 |
| riv_scr | 10061061ns | +620106.8ns (+6.5%) | [+108681, +1937530]ns | [9598525, 11409197] | YES | 0.0302 | 0.0075 | 0 |
| riv_snake | 9448629ns | no significant difference | [-165253, +1991657]ns | [9401274, 11430109] | no | 0.8145 | 0.8145 | 0 |
| riv_spec | 9585224ns | no significant difference | [-2915, +416334]ns | [9563576, 9887173] | no | 0.3172 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 9708020ns | -1.6% | -1.7% | -2.7% | -1.6% |
| 1 | 2 | 9484829ns | -0.5% | +23.0% | -1.3% | +1.3% |
| 1 | 3 | 9531711ns | -0.2% | +1.1% | +21.1% | +0.7% |
| 2 | 1 | 9576908ns | -1.1% | +20.5% | -1.8% | -0.2% |
| 2 | 2 | 9440931ns | -1.5% | +19.5% | -0.4% | +7.5% |
| 2 | 3 | 9426849ns | +0.3% | +1.2% | +21.4% | +9.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | -0.058 | ok |
| riv_rowm | -0.109 | ok |
| riv_scr | -0.539 | HIGH- (thermal bounce) |
| riv_snake | -0.348 | moderate- |
| riv_spec | 0.392 | moderate+ |

**Consistency summary:**

- **riv_rowm**: won 5/6, lost 1/6
- **riv_scr**: won 1/6, lost 5/6
- **riv_snake**: won 4/6, lost 2/6
- **riv_spec**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 46091985.5ns | 9531960.7ns | 483.6% | HIGH |
| riv_rowm | 46018363.6ns | 9609099.6ns | 478.9% | HIGH |
| riv_scr | 46412941.5ns | 10475590.2ns | 443.1% | HIGH |
| riv_snake | 46412844.4ns | 10122555.3ns | 458.5% | HIGH |
| riv_spec | 46087062.9ns | 9917767.8ns | 464.7% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 9389251.8-9662758.9 ns)
  9389251.8 |
  9402927.2 |####################
  9416602.5 |####################
  9430277.9 |########################################
  9443953.2 |####################
  9457628.6 |####################
  9471304.0 |####################
  9484979.3 |
  9498654.7 |####################
  9512330.0 |
  9526005.4 |####################
  9539680.7 |####################
  9553356.1 |
  9567031.4 |########################################
  9580706.8 |
  9594382.1 |####################
  9608057.5 |
  9621732.8 |
  9635408.2 |
  9649083.5 |
  (1 below, 4 above range)

riv_rowm (n=18, range 9298751.7-9995644.1 ns)
  9298751.7 |####################
  9333596.3 |
  9368440.9 |
  9403285.5 |########################################
  9438130.2 |##############################
  9472974.8 |####################
  9507819.4 |##########
  9542664.0 |##############################
  9577508.6 |
  9612353.3 |
  9647197.9 |
  9682042.5 |
  9716887.1 |
  9751731.8 |
  9786576.4 |
  9821421.0 |
  9856265.6 |
  9891110.2 |
  9925954.9 |
  9960799.5 |
  (1 below, 2 above range)

riv_scr (n=18, range 9523244.7-11543800.2 ns)
  9523244.7 |########################################
  9624272.4 |########################################
  9725300.2 |
  9826328.0 |
  9927355.8 |
  10028383.5 |
  10129411.3 |
  10230439.1 |
  10331466.9 |##########
  10432494.6 |
  10533522.4 |
  10634550.2 |
  10735578.0 |
  10836605.8 |
  10937633.5 |
  11038661.3 |
  11139689.1 |
  11240716.9 |##############################
  11341744.6 |
  11442772.4 |####################
  (1 below, 3 above range)

riv_snake (n=18, range 9353334.1-11495369.7 ns)
  9353334.1 |########################################
  9460435.9 |
  9567537.7 |####
  9674639.5 |####
  9781741.2 |
  9888843.0 |
  9995944.8 |
  10103046.6 |
  10210148.4 |
  10317250.1 |
  10424351.9 |
  10531453.7 |
  10638555.5 |
  10745657.3 |
  10852759.0 |
  10959860.8 |
  11066962.6 |
  11174064.4 |
  11281166.2 |
  11388267.9 |#############
  (1 below, 3 above range)

riv_spec (n=18, range 9533712.5-10619340.6 ns)
  9533712.5 |########################################
  9587993.9 |######################
  9642275.3 |
  9696556.7 |
  9750838.1 |
  9805119.5 |
  9859400.9 |
  9913682.3 |
  9967963.7 |
  10022245.1 |
  10076526.6 |
  10130808.0 |#####
  10185089.4 |
  10239370.8 |
  10293652.2 |
  10347933.6 |#####
  10402215.0 |
  10456496.4 |#####
  10510777.8 |
  10565059.2 |
  (2 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=483.6% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=485.6% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=458.7% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=487.8% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=479.8% of algo (FFI overhead may distort results)
