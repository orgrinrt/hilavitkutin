# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_rev, rcm_half) are a dead heat (<1%)

rcm_rev (325.56 us) and rcm_half (328.74 us) differ by 0.98%, inside the noise, even though the wider field spreads 6.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rcm_half shows alternating (throttle bounce) (autocorr -0.62)

rcm_half's per-pass series has lag-1 autocorrelation -0.62, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Speed leader rcm_rev vs stability leader rcm_half (+1% speed for 1.2x steadier)

rcm_rev is fastest (325.56 us, CV 5.5%); rcm_half gives up 1.0% median for 1.2x lower variance (CV 4.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 325562.5 ns median (-1.6% vs baseline)
- Spread: 1.06x (fastest 325562.5 ns, slowest 346400.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 347776ns | 336975ns | 321944ns | 334896ns | 380011ns | base |
| rcm_half | 336935ns | 337929ns | 320710ns | 331677ns | 352935ns | -3.12% |
| rcm_rev | 338154ns | 332100ns | 321133ns | 330316ns | 358422ns | -2.77% |
| rcm_scr | 358400ns | 352438ns | 325710ns | 347384ns | 391268ns | +3.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 341756ns | 315535ns | 374320ns | base | 0.001 |
| rcm_half | 330590ns | 315318ns | 346888ns | -3.27% | 0.001 |
| rcm_rev | 332077ns | 314778ns | 353004ns | -2.83% | 0.001 |
| rcm_scr | 352978ns | 319793ns | 386468ns | +3.28% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 95.2% |
| rcm_half | 0.001 | 95.8% |
| rcm_rev | 0.001 | 96.7% |
| rcm_scr | 0.001 | 90.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 347776ns | 347776ns | base |
| rcm_half | 336935ns | 336935ns | -3.12% |
| rcm_rev | 338154ns | 338154ns | -2.77% |
| rcm_scr | 358400ns | 358400ns | +3.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 330696ns | base | --- | [324610, 340723] | --- | --- | --- | --- |
| rcm_half | 328744ns | no significant difference | [-19248, +11167]ns | [319129, 335879] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 325562ns | no significant difference | [-12125, +2456]ns | [319635, 339338] | no | 0.3568 | 0.2379 | 0 |
| rcm_scr | 346400ns | no significant difference | [-2094, +27127]ns | [334271, 373819] | no | 0.3568 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 339742ns | -2.5% | -0.1% | +10.9% |
| 1 | 2 | 322192ns | -1.6% | -2.0% | +9.8% |
| 1 | 3 | 328754ns | +3.2% | -0.5% | +6.3% |
| 2 | 1 | 333762ns | -1.7% | -4.1% | +2.6% |
| 2 | 2 | 326188ns | +2.0% | -0.2% | +5.7% |
| 2 | 3 | 331071ns | -0.5% | +3.0% | +3.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.465 | moderate- |
| rcm_half | -0.617 | HIGH- (thermal bounce) |
| rcm_rev | -0.251 | moderate- |
| rcm_scr | 0.219 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 4/6, lost 2/6
- **rcm_rev**: won 5/6, lost 1/6
- **rcm_scr**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 11212279.9ns | 341756.0ns | 3280.8% | HIGH |
| rcm_half | 11180432.2ns | 330590.1ns | 3382.0% | HIGH |
| rcm_rev | 11179718.7ns | 332076.6ns | 3366.6% | HIGH |
| rcm_scr | 11188120.8ns | 352978.2ns | 3169.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 315534.7-374320.1 ns)
  315534.7 |##########################
  318474.0 |
  321413.3 |##########################
  324352.5 |##########################
  327291.8 |#############
  330231.1 |##########################
  333170.4 |#############
  336109.6 |#############
  339048.9 |########################################
  341988.2 |
  344927.4 |
  347866.7 |
  350806.0 |
  353745.3 |
  356684.5 |#############
  359623.8 |
  362563.1 |
  365502.3 |
  368441.6 |
  371380.9 |
  (1 below, 2 above range)

rcm_half (n=18, range 315318.1-346888.2 ns)
  315318.1 |#############
  316896.6 |##########################
  318475.1 |#############
  320053.6 |
  321632.1 |
  323210.6 |##########################
  324789.1 |
  326367.6 |
  327946.1 |##########################
  329524.6 |
  331103.1 |########################################
  332681.6 |
  334260.2 |
  335838.7 |
  337417.2 |
  338995.7 |##########################
  340574.2 |#############
  342152.7 |
  343731.2 |
  345309.7 |
  (2 below, 2 above range)

rcm_rev (n=18, range 314777.8-353004.2 ns)
  314777.8 |##########################
  316689.1 |
  318600.4 |########################################
  320511.7 |#############
  322423.0 |
  324334.4 |########################################
  326245.7 |#############
  328157.0 |
  330068.3 |#############
  331979.7 |
  333891.0 |
  335802.3 |
  337713.6 |#############
  339624.9 |##########################
  341536.3 |
  343447.6 |
  345358.9 |
  347270.2 |#############
  349181.5 |
  351092.9 |
  (1 below, 2 above range)

rcm_scr (n=18, range 319793.1-386468.1 ns)
  319793.1 |
  323126.8 |########################################
  326460.6 |####################
  329794.3 |
  333128.1 |
  336461.8 |####################
  339795.6 |########################################
  343129.3 |####################
  346463.1 |########################################
  349796.8 |
  353130.6 |####################
  356464.3 |
  359798.1 |
  363131.8 |
  366465.6 |
  369799.3 |####################
  373133.1 |
  376466.8 |########################################
  379800.6 |########################################
  383134.3 |
  (2 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3384.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3403.2% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3433.3% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3229.0% of algo (FFI overhead may distort results)
