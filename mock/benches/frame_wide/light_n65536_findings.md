## Before the numbers

Nothing in this file is a verdict on any arm, and no delta below is a reason to
use one arm over another.

The engine arms measure the engine without its headline features. Op, on 13
November: the engine arms measure something that "doesn't yet include any of
the actual headline features like fusing and the adaptive trunk/fiber/morsel
crunching on the datadlow-ish schedule. It's bare bones, barely lowers to
anything interesting yet. So no calls should be made based on those yet, they
are a baseline to compare against then." An engine number here is a baseline
for a later comparison and decides nothing.

Every delta below is taken against `hand`, the hand-written fused path, rather
than against an engine arm. The bench matrix names `hand` as the bar the engine
is measured against, so a row against it says how far an arm is from what a
person writes for this workload. Where the report says the baseline is the
slowest variant and reads as picked by accident, it was not: it is set in this
member's driver, on purpose, and is meant to be the bar whether it wins or
loses. Do not re-baseline on whichever arm happens to be fastest.

The harness writes a `Why it matters` line under each highlight, calling a large
delta a reason to switch and a dominant winner a safe default pick. Those lines
are its stock phrasing for any dataset, they are judgements this file has no
standing to make, and they are stripped before it is written. What is left under
each highlight is the shape of the numbers.

A median here is wall-clock time and carries nothing about how many cores an
arm used to reach it. Two arms with the same median are not the same cost if
one of them ran on one core and the other on two. The occupancy of every
parallel arm is measured separately and stated beside these medians in
`mock/research/202611131400_frame-wide-first-run.md`. Read the two together or
neither.

The generated report follows.

# Wide frame, one multiply per stage

7 variants, 8 samples per variant.
Baseline: hand

## Highlights

Baseline for all deltas below: hand. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### hand dominates: 127% faster than the next best (rust_std)

hand (71.33 us) leads rust_std (161.96 us) by 127%, a clear separation rather than a photo finish. CV 6.2%.


### rust_std_par is an outlier: 7.3x slower than the field

rust_std_par (524.02 us) is 7.3x the fastest (71.33 us), well off the pack.


### hand_par shows alternating (throttle bounce) (autocorr -0.50)

hand_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (71.33 us median); no rival improves on it (all deltas are >= 0).


### Wide spread: slowest is 7.3x the fastest

Fastest hand (71.33 us) to slowest rust_std_par (524.02 us): 7.3x. The strategy choice matters a lot for this workload.


### engine_par is inconsistent: worst-20% is 1.9x its best-20%

engine_par's best 20% of batches run at 290.42 us but its worst 20% at 564.37 us (1.9x) - a bimodal or bursty profile the median hides.


## Key findings

- Baseline (hand) is the fastest at 71334.2 ns median
- 6 variants significantly slower than baseline
- Spread: 7.35x (fastest 71334.2 ns, slowest 524019.6 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 175129ns | 174619ns | 160938ns | 170984ns | 181785ns | +138.76% |
| engine_par | 433643ns | 325641ns | 291828ns | 304965ns | 565606ns | +491.19% |
| hand | 73351ns | 71921ns | 67834ns | 71121ns | 76402ns | base |
| hand_par | 219229ns | 193013ns | 179152ns | 186446ns | 253835ns | +198.88% |
| rust_nostd | 186629ns | 191470ns | 154691ns | 174095ns | 204013ns | +154.43% |
| rust_std | 169684ns | 163075ns | 135818ns | 153867ns | 190013ns | +131.33% |
| rust_std_par | 540787ns | 525608ns | 434024ns | 500280ns | 597857ns | +637.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 173817ns | 159640ns | 180415ns | +140.64% | 4.524 |
| engine_par | 432430ns | 290416ns | 564373ns | +498.67% | 1.819 |
| hand | 72231ns | 66751ns | 75141ns | base | 10.888 |
| hand_par | 217777ns | 177749ns | 252066ns | +201.50% | 3.611 |
| rust_nostd | 185030ns | 153410ns | 202356ns | +156.16% | 4.250 |
| rust_std | 168330ns | 134746ns | 188577ns | +133.04% | 4.672 |
| rust_std_par | 539055ns | 431822ns | 596363ns | +646.29% | 1.459 |

## Performance model

- Peak throughput: 11.782 Gops/s (hand; best 20% batches)
- Ops per call: 786432

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 4.534 | 38.5% |
| engine_par | 2.421 | 20.6% |
| hand | 11.025 | 93.6% |
| hand_par | 4.101 | 34.8% |
| rust_nostd | 4.147 | 35.2% |
| rust_std | 4.856 | 41.2% |
| rust_std_par | 1.501 | 12.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 175129ns | 175129ns | +138.76% |
| engine_par | 433643ns | 433643ns | +491.19% |
| hand | 73351ns | 73351ns | base |
| hand_par | 219229ns | 219229ns | +198.88% |
| rust_nostd | 186629ns | 186629ns | +154.43% |
| rust_std | 169684ns | 169684ns | +131.33% |
| rust_std_par | 540787ns | 540787ns | +637.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 71334ns | base | --- | [69440, 73478] | --- | --- | --- | --- |
| engine | 173468ns | +100183.5ns (+140.4%) | [+91229, +107918]ns | [166199, 175992] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 324793ns | +253763.6ns (+355.7%) | [+218822, +532667]ns | [290604, 603553] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 191754ns | +118662.1ns (+166.3%) | [+111800, +187762]ns | [182686, 260381] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 189644ns | +119481.4ns (+167.5%) | [+93406, +129328]ns | [165189, 208496] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 161964ns | +93868.4ns (+131.6%) | [+70083, +110310]ns | [148359, 183788] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 524020ns | +453905.2ns (+636.3%) | [+398623, +524024]ns | [468063, 606708] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 71334ns | +140.4% | +526.7% | +178.5% | +132.2% | +127.2% | +678.1% |
| 1 | 2 | 71029ns | +145.4% | +357.3% | +170.0% | +184.6% | +128.0% | +592.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | HIGH- (thermal bounce) |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

Consistency summary:

- engine: won 0/2, lost 2/2
- engine_par: won 0/2, lost 2/2
- hand_par: won 0/2, lost 2/2
- rust_nostd: won 0/2, lost 2/2
- rust_std: won 0/2, lost 2/2
- rust_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 974090.1ns | 173816.7ns | 560.4% | HIGH |
| engine_par | 1003642.4ns | 432429.9ns | 232.1% | HIGH |
| hand | 917417.3ns | 72231.4ns | 1270.1% | HIGH |
| hand_par | 1068760.2ns | 217776.6ns | 490.8% | HIGH |
| rust_nostd | 994521.7ns | 185030.3ns | 537.5% | HIGH |
| rust_std | 947401.6ns | 168330.4ns | 562.8% | HIGH |
| rust_std_par | 906890.0ns | 539054.8ns | 168.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 159640.2-180414.8 ns)
  159640.2 |########################################
  160678.9 |
  161717.7 |
  162756.4 |
  163795.1 |
  164833.9 |
  165872.6 |########################################
  166911.3 |
  167950.1 |
  168988.8 |########################################
  170027.5 |
  171066.3 |
  172105.0 |########################################
  173143.7 |########################################
  174182.5 |########################################
  175221.2 |########################################
  176259.9 |
  177298.7 |
  178337.4 |
  179376.1 |
  (0 below, 1 above range)

engine_par (n=8, range 290415.5-564372.9 ns)
  290415.5 |########################################
  304113.4 |
  317811.2 |##########################
  331509.1 |
  345207.0 |
  358904.8 |
  372602.7 |
  386300.6 |#############
  399998.4 |
  413696.3 |
  427394.2 |
  441092.0 |
  454789.9 |
  468487.8 |
  482185.6 |
  495883.5 |
  509581.4 |
  523279.2 |
  536977.1 |
  550675.0 |
  (0 below, 2 above range)

hand (n=8, range 66751.0-75140.6 ns)
  66751.0 |########################################
  67170.5 |
  67590.0 |
  68009.4 |
  68428.9 |
  68848.4 |
  69267.9 |########################################
  69687.4 |
  70106.9 |########################################
  70526.3 |########################################
  70945.8 |
  71365.3 |########################################
  71784.8 |
  72204.3 |########################################
  72623.7 |
  73043.2 |
  73462.7 |########################################
  73882.2 |
  74301.7 |
  74721.1 |
  (0 below, 1 above range)

hand_par (n=8, range 177749.3-252065.8 ns)
  177749.3 |####################
  181465.1 |####################
  185181.0 |########################################
  188896.8 |
  192612.6 |####################
  196328.4 |
  200044.3 |
  203760.1 |
  207475.9 |
  211191.7 |####################
  214907.6 |
  218623.4 |
  222339.2 |
  226055.0 |
  229770.9 |
  233486.7 |
  237202.5 |
  240918.3 |
  244634.2 |
  248350.0 |
  (0 below, 2 above range)

rust_nostd (n=8, range 153410.3-202355.7 ns)
  153410.3 |########################################
  155857.6 |
  158304.8 |
  160752.1 |
  163199.4 |########################################
  165646.6 |########################################
  168093.9 |
  170541.2 |
  172988.5 |
  175435.7 |
  177883.0 |
  180330.3 |
  182777.5 |
  185224.8 |########################################
  187672.1 |
  190119.4 |
  192566.6 |########################################
  195013.9 |########################################
  197461.2 |
  199908.4 |
  (0 below, 2 above range)

rust_std (n=8, range 134746.4-188577.0 ns)
  134746.4 |########################################
  137437.9 |
  140129.5 |
  142821.0 |
  145512.5 |
  148204.1 |########################################
  150895.6 |########################################
  153587.1 |
  156278.7 |########################################
  158970.2 |
  161661.7 |
  164353.3 |
  167044.8 |########################################
  169736.3 |
  172427.9 |
  175119.4 |########################################
  177810.9 |
  180502.5 |
  183194.0 |########################################
  185885.5 |
  (0 below, 1 above range)

rust_std_par (n=8, range 431821.5-596362.5 ns)
  431821.5 |########################################
  440048.6 |
  448275.6 |
  456502.7 |
  464729.7 |########################################
  472956.8 |
  481183.8 |
  489410.9 |
  497637.9 |
  505865.0 |########################################
  514092.0 |########################################
  522319.1 |
  530546.1 |########################################
  538773.2 |
  547000.2 |
  555227.3 |
  563454.3 |
  571681.4 |########################################
  579908.4 |
  588135.5 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=558.0% of algo (FFI overhead may distort results)
- engine_par: CV=48.9% (high variance, measurements may be unstable)
- engine_par: bridge=308.4% of algo (FFI overhead may distort results)
- hand: bridge=1286.9% of algo (FFI overhead may distort results)
- hand_par: CV=23.8% (high variance, measurements may be unstable)
- hand_par: bridge=545.4% of algo (FFI overhead may distort results)
- rust_nostd: bridge=528.2% of algo (FFI overhead may distort results)
- rust_std: bridge=587.7% of algo (FFI overhead may distort results)
- rust_std_par: bridge=171.8% of algo (FFI overhead may distort results)
