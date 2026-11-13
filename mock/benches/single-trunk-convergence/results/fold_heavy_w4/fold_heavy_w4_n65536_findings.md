# fold_heavy, W = 4

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel dominates: 18% faster than the next best (nway)

std_parallel (153.73 us) leads nway (181.25 us) by 18%, a clear separation rather than a photo finish. CV 10.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.53)

kernel_seq's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### kernel_nway is inconsistent: worst-20% is 2.5x its best-20%

kernel_nway's best 20% of batches run at 207.75 us but its worst 20% at 510.77 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: std_parallel** at 153726.3 ns median (-21.4% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 1.90x (fastest 153726.3 ns, slowest 292353.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, W = 4 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100312120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 193105ns | 196127ns | 161697ns | 184096ns | 222323ns | base |
| head_tail_dyn | 226659ns | 219527ns | 156350ns | 200791ns | 300614ns | +17.38% |
| kernel_nway | 334900ns | 267208ns | 208640ns | 259148ns | 511658ns | +73.43% |
| kernel_seq | 298800ns | 293253ns | 279157ns | 290348ns | 321301ns | +54.73% |
| nway | 204031ns | 181623ns | 149534ns | 179751ns | 267701ns | +5.66% |
| sequential | 308879ns | 288245ns | 262777ns | 282544ns | 371431ns | +59.95% |
| std_parallel | 159667ns | 154311ns | 142879ns | 152441ns | 178901ns | -17.32% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 192408ns | 161178ns | 221501ns | base | 0.341 |
| head_tail_dyn | 225844ns | 155902ns | 299679ns | +17.38% | 0.290 |
| kernel_nway | 334076ns | 207755ns | 510771ns | +73.63% | 0.196 |
| kernel_seq | 297879ns | 278533ns | 320261ns | +54.82% | 0.220 |
| nway | 203529ns | 149069ns | 267074ns | +5.78% | 0.322 |
| sequential | 308316ns | 262437ns | 370527ns | +60.24% | 0.213 |
| std_parallel | 158956ns | 142329ns | 177959ns | -17.39% | 0.412 |

## Performance model

- Peak throughput: **0.460 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.335 | 72.8% |
| head_tail_dyn | 0.300 | 65.1% |
| kernel_nway | 0.246 | 53.5% |
| kernel_seq | 0.224 | 48.7% |
| nway | 0.362 | 78.5% |
| sequential | 0.228 | 49.5% |
| std_parallel | 0.426 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 193105ns | 193105ns | base |
| head_tail_dyn | 226659ns | 226659ns | +17.38% |
| kernel_nway | 334900ns | 334900ns | +73.43% |
| kernel_seq | 298800ns | 298800ns | +54.73% |
| nway | 204031ns | 204031ns | +5.66% |
| sequential | 308879ns | 308879ns | +59.95% |
| std_parallel | 159667ns | 159667ns | -17.32% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 195623ns | base | --- | [171177, 202152] | --- | --- | --- | --- |
| head_tail_dyn | 218763ns | no significant difference | [-33199, +74598]ns | [170219, 251062] | no | 0.4807 | 0.4807 | 0 |
| kernel_nway | 266010ns | +77679.3ns (+39.7%) | [+25834, +193328]ns | [230448, 366443] | YES | 0.0003 | 0.0001 | 0 |
| kernel_seq | 292354ns | +108859.7ns (+55.6%) | [+85635, +132738]ns | [283574, 305793] | YES | 0.0000 | 0.0000 | 0 |
| nway | 181251ns | no significant difference | [-4249, +51966]ns | [171048, 246347] | no | 0.2855 | 0.2379 | 0 |
| sequential | 287788ns | +89501.7ns (+45.8%) | [+73627, +130930]ns | [274625, 324957] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 153726ns | -32784.5ns (-16.8%) | [-52801, -7613]ns | [148455, 167884] | YES | 0.0463 | 0.0309 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 200998ns | -21.1% | +13.4% | +46.3% | +33.0% | +35.7% | -26.0% |
| 1 | 2 | 173115ns | +26.4% | +102.1% | +74.8% | +0.6% | +67.6% | -10.9% |
| 1 | 3 | 169650ns | +50.9% | +121.8% | +68.9% | +3.2% | +106.9% | -7.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.112 | ok |
| head_tail_dyn | -0.012 | ok |
| kernel_nway | -0.081 | ok |
| kernel_seq | -0.526 | HIGH- (thermal bounce) |
| nway | -0.172 | ok |
| sequential | -0.062 | ok |
| std_parallel | -0.018 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 192146.3ns | 192408.2ns | 99.9% | HIGH |
| head_tail_dyn | 226614.8ns | 225843.8ns | 100.3% | HIGH |
| kernel_nway | 300956.8ns | 334075.5ns | 90.1% | HIGH |
| kernel_seq | 323068.3ns | 297878.9ns | 108.5% | HIGH |
| nway | 209887.9ns | 203528.8ns | 103.1% | HIGH |
| sequential | 305275.3ns | 308316.2ns | 99.0% | HIGH |
| std_parallel | 169244.2ns | 158955.8ns | 106.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 161178.1-221501.4 ns)
  161178.1 |#############
  164194.3 |
  167210.5 |##########################
  170226.6 |#############
  173242.8 |#############
  176259.0 |#############
  179275.1 |
  182291.3 |
  185307.5 |
  188323.6 |
  191339.8 |
  194356.0 |##########################
  197372.1 |########################################
  200388.3 |
  203404.5 |########################################
  206420.6 |
  209436.8 |
  212453.0 |
  215469.1 |
  218485.3 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 155901.6-299679.3 ns)
  155901.6 |########################################
  163090.5 |#############
  170279.4 |
  177468.3 |
  184657.2 |#############
  191846.1 |#############
  199034.9 |
  206223.8 |#############
  213412.7 |
  220601.6 |
  227790.5 |#############
  234979.4 |##########################
  242168.2 |#############
  249357.1 |##########################
  256546.0 |#############
  263734.9 |
  270923.8 |
  278112.7 |
  285301.5 |
  292490.4 |
  (2 below, 2 above range)

kernel_nway (n=18, range 207754.9-510771.2 ns)
  207754.9 |########
  222905.7 |########################################
  238056.5 |########
  253207.3 |
  268358.2 |
  283509.0 |################
  298659.8 |
  313810.6 |
  328961.4 |
  344112.2 |################
  359263.0 |
  374413.8 |########
  389564.7 |
  404715.5 |
  419866.3 |
  435017.1 |
  450167.9 |
  465318.7 |
  480469.5 |
  495620.4 |
  (2 below, 4 above range)

kernel_seq (n=18, range 278533.3-320260.6 ns)
  278533.3 |########################################
  280619.6 |########################################
  282706.0 |####################
  284792.4 |####################
  286878.7 |####################
  288965.1 |####################
  291051.5 |
  293137.9 |####################
  295224.2 |
  297310.6 |####################
  299397.0 |####################
  301483.3 |
  303569.7 |####################
  305656.1 |####################
  307742.4 |
  309828.8 |####################
  311915.2 |
  314001.5 |
  316087.9 |
  318174.3 |####################
  (1 below, 2 above range)

nway (n=18, range 149068.6-267074.0 ns)
  149068.6 |
  154968.9 |
  160869.2 |#############
  166769.4 |########################################
  172669.7 |##########################
  178570.0 |##########################
  184470.3 |
  190370.5 |##########################
  196270.8 |
  202171.1 |
  208071.3 |
  213971.6 |
  219871.9 |
  225772.2 |
  231672.4 |
  237572.7 |
  243473.0 |#############
  249373.2 |
  255273.5 |#############
  261173.8 |#############
  (2 below, 3 above range)

sequential (n=18, range 262437.3-370527.3 ns)
  262437.3 |##########################
  267841.8 |#############
  273246.3 |########################################
  278650.8 |
  284055.3 |########################################
  289459.8 |##########################
  294864.3 |
  300268.8 |
  305673.3 |
  311077.8 |
  316482.3 |
  321886.8 |#############
  327291.3 |#############
  332695.8 |
  338100.3 |
  343504.8 |#############
  348909.3 |#############
  354313.8 |
  359718.3 |
  365122.8 |
  (1 below, 2 above range)

std_parallel (n=18, range 142328.6-177958.8 ns)
  142328.6 |##########################
  144110.1 |
  145891.6 |#############
  147673.1 |##########################
  149454.6 |#############
  151236.1 |#############
  153017.6 |##########################
  154799.2 |#############
  156580.7 |#############
  158362.2 |
  160143.7 |
  161925.2 |#############
  163706.7 |
  165488.2 |
  167269.7 |
  169051.2 |
  170832.7 |
  172614.2 |########################################
  174395.7 |
  176177.2 |
  (1 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=95.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=30.7% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=109.6% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=43.0% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=105.3% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.4% of algo (FFI overhead may distort results)
- **nway**: CV=23.3% (high variance, measurements may be unstable)
- **nway**: bridge=102.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.0% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.8% of algo (FFI overhead may distort results)
