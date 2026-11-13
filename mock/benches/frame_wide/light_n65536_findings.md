# Wide frame, one multiply per stage

7 variants, 8 samples per variant.
Baseline: **engine**

## Highlights

Baseline for all deltas below: **engine**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hand dominates: 64% faster than the next best (rust_std)

hand (106.74 us) leads rust_std (174.62 us) by 64%, a clear separation rather than a photo finish. CV 18.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hand beats baseline by 51% (significant)

hand is -111.25 us (51%) faster than baseline engine, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### rust_std_par is an outlier: 5.6x slower than the field

rust_std_par (602.62 us) is 5.6x the fastest (106.74 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.6x the fastest

Fastest hand (106.74 us) to slowest rust_std_par (602.62 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### engine_par is inconsistent: worst-20% is 2.0x its best-20%

engine_par's best 20% of batches run at 194.11 us but its worst 20% at 381.55 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hand** at 106740.2 ns median (-51.3% vs baseline)
- 2 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 5.65x (fastest 106740.2 ns, slowest 602616.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 222986ns | 220965ns | 168871ns | 177855ns | 270364ns | base |
| engine_par | 334976ns | 346169ns | 195945ns | 317163ns | 383094ns | +50.22% |
| hand | 102720ns | 107909ns | 69652ns | 94911ns | 116843ns | -53.93% |
| hand_par | 271905ns | 272698ns | 205147ns | 231994ns | 318529ns | +21.94% |
| rust_nostd | 194681ns | 195852ns | 172798ns | 186184ns | 206524ns | -12.69% |
| rust_std | 176394ns | 175800ns | 153562ns | 170808ns | 186290ns | -20.89% |
| rust_std_par | 604469ns | 604619ns | 549072ns | 574594ns | 640724ns | +171.08% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 221397ns | 167604ns | 268550ns | base | 3.552 |
| engine_par | 333454ns | 194106ns | 381546ns | +50.61% | 2.358 |
| hand | 101588ns | 69092ns | 115435ns | -54.11% | 7.741 |
| hand_par | 270081ns | 204049ns | 316114ns | +21.99% | 2.912 |
| rust_nostd | 192835ns | 171595ns | 204480ns | -12.90% | 4.078 |
| rust_std | 175113ns | 152624ns | 184930ns | -20.91% | 4.491 |
| rust_std_par | 602509ns | 547332ns | 638620ns | +172.14% | 1.305 |

## Performance model

- Peak throughput: **11.382 Gops/s** (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 3.585 | 31.5% |
| engine_par | 2.282 | 20.0% |
| hand | 7.368 | 64.7% |
| hand_par | 2.900 | 25.5% |
| rust_nostd | 4.057 | 35.6% |
| rust_std | 4.504 | 39.6% |
| rust_std_par | 1.305 | 11.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 222986ns | 222986ns | base |
| engine_par | 334976ns | 334976ns | +50.22% |
| hand | 102720ns | 102720ns | -53.93% |
| hand_par | 271905ns | 271905ns | +21.94% |
| rust_nostd | 194681ns | 194681ns | -12.69% |
| rust_std | 176394ns | 176394ns | -20.89% |
| rust_std_par | 604469ns | 604469ns | +171.08% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| engine | 219340ns | base | --- | [174328, 267911] | --- | --- | --- | --- |
| engine_par | 344635ns | +110197.0ns (+50.2%) | [+83567, +160140]ns | [278568, 382966] | YES | 0.0094 | 0.0078 | 0 |
| hand | 106740ns | -111253.1ns (-50.7%) | [-198820, -59795]ns | [70731, 117595] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 271221ns | +40611.3ns (+18.5%) | [+22477, +81547]ns | [214234, 343746] | YES | 0.0094 | 0.0078 | 0 |
| rust_nostd | 193853ns | no significant difference | [-83986, +27583]ns | [177845, 204974] | no | 1.0000 | 1.0000 | 0 |
| rust_std | 174625ns | -41624.5ns (-19.0%) | [-85747, -2698]ns | [161884, 189843] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 602616ns | +379030.3ns (+172.8%) | [+362355, +416588]ns | [548186, 659241] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | engine | engine_par | hand | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 175859ns | +73.3% | -36.2% | +23.2% | +10.2% | -1.4% | +226.0% |
| 1 | 2 | 266132ns | +43.8% | -67.1% | +18.4% | -28.6% | -31.4% | +141.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

**Consistency summary:**

- **engine_par**: won 0/2, lost 2/2
- **hand**: won 2/2, lost 0/2
- **hand_par**: won 0/2, lost 2/2
- **rust_nostd**: won 1/2, lost 1/2
- **rust_std**: won 2/2, lost 0/2
- **rust_std_par**: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 1002657.1ns | 221396.8ns | 452.9% | HIGH |
| engine_par | 1034517.2ns | 333454.4ns | 310.2% | HIGH |
| hand | 1035176.8ns | 101588.0ns | 1019.0% | HIGH |
| hand_par | 1032422.5ns | 270080.7ns | 382.3% | HIGH |
| rust_nostd | 1008116.4ns | 192835.2ns | 522.8% | HIGH |
| rust_std | 998569.1ns | 175113.4ns | 570.2% | HIGH |
| rust_std_par | 1001863.8ns | 602509.4ns | 166.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 167603.7-268550.2 ns)
  167603.7 |#############
  172651.0 |########################################
  177698.3 |
  182745.7 |
  187793.0 |
  192840.3 |
  197887.6 |
  202935.0 |
  207982.3 |
  213029.6 |
  218076.9 |
  223124.3 |
  228171.6 |
  233218.9 |
  238266.2 |
  243313.6 |
  248360.9 |
  253408.2 |
  258455.5 |#############
  263502.9 |##########################
  (0 below, 1 above range)

engine_par (n=8, range 194106.1-381546.2 ns)
  194106.1 |########################################
  203478.1 |
  212850.1 |
  222222.1 |
  231594.1 |
  240966.1 |
  250338.1 |
  259710.1 |
  269082.2 |
  278454.2 |########################################
  287826.2 |
  297198.2 |
  306570.2 |
  315942.2 |
  325314.2 |########################################
  334686.2 |########################################
  344058.2 |########################################
  353430.2 |
  362802.2 |
  372174.2 |
  (0 below, 3 above range)

hand (n=8, range 69091.7-115435.3 ns)
  69091.7 |########################################
  71408.9 |
  73726.1 |
  76043.2 |
  78360.4 |
  80677.6 |
  82994.8 |
  85312.0 |
  87629.1 |
  89946.3 |
  92263.5 |
  94580.7 |
  96897.9 |
  99215.1 |
  101532.2 |
  103849.4 |####################
  106166.6 |########################################
  108483.8 |
  110801.0 |####################
  113118.1 |
  (0 below, 2 above range)

hand_par (n=8, range 204048.6-316114.2 ns)
  204048.6 |####################
  209651.9 |####################
  215255.2 |####################
  220858.4 |
  226461.7 |
  232065.0 |
  237668.3 |
  243271.6 |
  248874.9 |
  254478.1 |####################
  260081.4 |
  265684.7 |
  271288.0 |
  276891.3 |
  282494.6 |########################################
  288097.8 |
  293701.1 |
  299304.4 |
  304907.7 |
  310511.0 |
  (0 below, 2 above range)

rust_nostd (n=8, range 171595.2-204480.3 ns)
  171595.2 |########################################
  173239.5 |
  174883.7 |
  176528.0 |########################################
  178172.2 |
  179816.5 |
  181460.7 |
  183105.0 |
  184749.2 |########################################
  186393.5 |
  188037.8 |########################################
  189682.0 |
  191326.3 |
  192970.5 |
  194614.8 |
  196259.0 |
  197903.3 |########################################
  199547.5 |
  201191.8 |########################################
  202836.0 |
  (0 below, 2 above range)

rust_std (n=8, range 152623.5-184930.0 ns)
  152623.5 |####################
  154238.8 |
  155854.2 |
  157469.5 |
  159084.8 |
  160700.1 |####################
  162315.5 |
  163930.8 |
  165546.1 |
  167161.4 |
  168776.8 |
  170392.1 |
  172007.4 |####################
  173622.7 |########################################
  175238.1 |####################
  176853.4 |
  178468.7 |
  180084.0 |
  181699.4 |
  183314.7 |
  (0 below, 2 above range)

rust_std_par (n=8, range 547332.5-638620.2 ns)
  547332.5 |########################################
  551896.9 |
  556461.3 |
  561025.7 |
  565590.1 |
  570154.4 |####################
  574718.8 |
  579283.2 |
  583847.6 |
  588412.0 |
  592976.4 |
  597540.8 |####################
  602105.2 |
  606669.5 |####################
  611233.9 |
  615798.3 |
  620362.7 |####################
  624927.1 |
  629491.5 |
  634055.9 |
  (0 below, 2 above range)

```

## Diagnostics

- **engine**: CV=21.5% (high variance, measurements may be unstable)
- **engine**: bridge=458.3% of algo (FFI overhead may distort results)
- **engine_par**: bridge=298.0% of algo (FFI overhead may distort results)
- **hand**: bridge=961.2% of algo (FFI overhead may distort results)
- **hand_par**: bridge=362.5% of algo (FFI overhead may distort results)
- **rust_nostd**: bridge=527.0% of algo (FFI overhead may distort results)
- **rust_std**: bridge=569.9% of algo (FFI overhead may distort results)
- **rust_std_par**: bridge=165.8% of algo (FFI overhead may distort results)
