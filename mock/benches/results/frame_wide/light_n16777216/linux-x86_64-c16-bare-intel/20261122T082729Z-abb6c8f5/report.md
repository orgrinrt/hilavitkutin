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
slowest variant and reads as picked by accident, it was not: it is set in the
bench tree's hooks library, on purpose, and is meant to be the bar whether it
wins or loses. Do not re-baseline on whichever arm happens to be fastest.

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

### hand dominates: 21% faster than the next best (hand_par)

hand (16.43 ms) leads hand_par (19.88 ms) by 21%, a clear separation rather than a photo finish. CV 5.5%.


### rust_std is an outlier: 8.8x slower than the field

rust_std (144.16 ms) is 8.8x the fastest (16.43 ms), well off the pack.


### hand is fastest but the noisiest (CV 5.5%)

hand wins on median (16.43 ms) yet has the highest variance (CV 5.5%), while rust_std is the steadiest (CV 0.2%, 144.16 ms).


### rust_nostd shows alternating (throttle bounce) (autocorr -0.50)

rust_nostd's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (16.43 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {engine_par, engine, rust_nostd, rust_std_par, rust_std} (269% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {engine_par, engine, rust_nostd, rust_std_par, rust_std} with a 269% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 8.8x the fastest

Fastest hand (16.43 ms) to slowest rust_std (144.16 ms): 8.8x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 16431029.1 ns median
- 6 variants significantly slower than baseline
- Spread: 8.77x (fastest 16431029.1 ns, slowest 144162623.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide frame, one multiply per stage |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xf4a3e00000000002 |
| Counter frequency | 2112754058 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 74562980ns | 74595547ns | 74228727ns | 74389556ns | 74776612ns | +339.54% |
| engine_par | 74455312ns | 73267509ns | 70654517ns | 71726617ns | 77452032ns | +338.91% |
| hand | 16963768ns | 16432529ns | 16213971ns | 16327868ns | 17628141ns | base |
| hand_par | 20000385ns | 19878381ns | 19656710ns | 19724191ns | 20293450ns | +17.90% |
| rust_nostd | 89589598ns | 89844805ns | 88768917ns | 89392866ns | 89942318ns | +428.12% |
| rust_std | 144201519ns | 144164134ns | 143665852ns | 143949164ns | 144524701ns | +750.06% |
| rust_std_par | 124756576ns | 125548672ns | 119457899ns | 122804293ns | 127545458ns | +635.43% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 74561737ns | 74227928ns | 74775169ns | +339.58% | 2.700 |
| engine_par | 74452438ns | 70651756ns | 77449062ns | +338.94% | 2.704 |
| hand | 16961959ns | 16211732ns | 17626428ns | base | 11.869 |
| hand_par | 19997004ns | 19653431ns | 20290184ns | +17.89% | 10.068 |
| rust_nostd | 89587836ns | 88766817ns | 89940425ns | +428.17% | 2.247 |
| rust_std | 144199801ns | 143664421ns | 144522942ns | +750.14% | 1.396 |
| rust_std_par | 124753290ns | 119454253ns | 127542320ns | +635.49% | 1.614 |

## Performance model

- Peak throughput: 12.419 Gops/s (hand; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 2.699 | 21.7% |
| engine_par | 2.748 | 22.1% |
| hand | 12.253 | 98.7% |
| hand_par | 10.130 | 81.6% |
| rust_nostd | 2.241 | 18.0% |
| rust_std | 1.397 | 11.2% |
| rust_std_par | 1.604 | 12.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 74562980ns | 74562980ns | +339.54% |
| engine_par | 74455312ns | 74455312ns | +338.91% |
| hand | 16963768ns | 16963768ns | base |
| hand_par | 20000385ns | 20000385ns | +17.90% |
| rust_nostd | 89589598ns | 89589598ns | +428.12% |
| rust_std | 144201519ns | 144201519ns | +750.06% |
| rust_std_par | 124756576ns | 124756576ns | +635.43% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 16431029ns | base | --- | [16215659, 18464053] | --- | --- | --- | --- |
| engine | 74594389ns | +57955812.9ns (+352.7%) | [+56362896, +58344421]ns | [74351184, 74773957] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 73264722ns | +56951777.2ns (+346.6%) | [+54199928, +59809119]ns | [70711212, 76916811] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 19875048ns | +3440661.0ns (+20.9%) | [+1501907, +3951703]ns | [19683274, 20244308] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 89842828ns | +72785043.1ns (+443.0%) | [+71519326, +73621016]ns | [89106317, 89963323] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 144162624ns | +127605109.0ns (+776.6%) | [+125499555, +128135633]ns | [143904660, 144505008] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 125545494ns | +108703414.2ns (+661.6%) | [+103101911, +110904162]ns | [119912405, 127869354] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 16333743ns | +355.3% | +337.7% | +22.1% | +450.3% | +784.6% | +668.6% |
| 1 | 2 | 17437142ns | +328.8% | +332.1% | +14.0% | +411.4% | +725.4% | +621.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | HIGH- (thermal bounce) |
| rust_nostd | -0.500 | HIGH- (thermal bounce) |
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
| engine | 188641156.8ns | 74561736.9ns | 253.0% | HIGH |
| engine_par | 189884188.8ns | 74452438.1ns | 255.0% | HIGH |
| hand | 189456555.9ns | 16961959.4ns | 1116.9% | HIGH |
| hand_par | 190129758.4ns | 19997004.1ns | 950.8% | HIGH |
| rust_nostd | 190121049.1ns | 89587835.8ns | 212.2% | HIGH |
| rust_std | 188890336.9ns | 144199801.4ns | 131.0% | HIGH |
| rust_std_par | 191161428.1ns | 124753290.4ns | 153.2% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 74227928.4-74775169.4 ns)
  74227928.4 |####################
  74255290.5 |
  74282652.5 |
  74310014.6 |
  74337376.6 |####################
  74364738.7 |####################
  74392100.7 |
  74419462.8 |####################
  74446824.8 |
  74474186.9 |
  74501548.9 |
  74528911.0 |
  74556273.0 |
  74583635.1 |
  74610997.1 |
  74638359.2 |
  74665721.2 |
  74693083.3 |
  74720445.3 |####################
  74747807.4 |########################################
  (0 below, 1 above range)

engine_par (n=8, range 70651755.7-77449062.4 ns)
  70651755.7 |########################################
  70991621.0 |
  71331486.4 |
  71671351.7 |
  72011217.0 |########################################
  72351082.4 |
  72690947.7 |
  73030813.0 |
  73370678.4 |
  73710543.7 |
  74050409.0 |####################
  74390274.4 |
  74730139.7 |
  75070005.0 |
  75409870.4 |
  75749735.7 |
  76089601.0 |
  76429466.4 |####################
  76769331.7 |####################
  77109197.0 |
  (0 below, 1 above range)

hand (n=8, range 16211731.9-17626427.9 ns)
  16211731.9 |########################################
  16282466.7 |####################
  16353201.5 |####################
  16423936.3 |####################
  16494671.1 |
  16565405.9 |
  16636140.7 |
  16706875.5 |
  16777610.3 |
  16848345.1 |
  16919079.9 |
  16989814.7 |
  17060549.5 |####################
  17131284.3 |
  17202019.1 |
  17272753.9 |
  17343488.7 |
  17414223.5 |
  17484958.3 |
  17555693.1 |
  (0 below, 2 above range)

hand_par (n=8, range 19653430.9-20290184.0 ns)
  19653430.9 |########################################
  19685268.6 |####################
  19717106.2 |
  19748943.9 |####################
  19780781.5 |
  19812619.2 |
  19844456.8 |
  19876294.5 |
  19908132.2 |
  19939969.8 |
  19971807.5 |####################
  20003645.1 |
  20035482.8 |
  20067320.4 |
  20099158.1 |
  20130995.8 |
  20162833.4 |####################
  20194671.1 |
  20226508.7 |####################
  20258346.4 |
  (0 below, 1 above range)

rust_nostd (n=8, range 88766817.1-89940425.5 ns)
  88766817.1 |####################
  88825497.5 |
  88884177.9 |
  88942858.4 |
  89001538.8 |
  89060219.2 |####################
  89118899.6 |
  89177580.0 |####################
  89236260.4 |
  89294940.9 |
  89353621.3 |
  89412301.7 |
  89470982.1 |
  89529662.5 |
  89588343.0 |
  89647023.4 |
  89705703.8 |
  89764384.2 |
  89823064.6 |########################################
  89881745.1 |####################
  (0 below, 2 above range)

rust_std (n=8, range 143664420.6-144522942.1 ns)
  143664420.6 |####################
  143707346.7 |
  143750272.7 |
  143793198.8 |
  143836124.9 |
  143879051.0 |####################
  143921977.0 |####################
  143964903.1 |####################
  144007829.2 |
  144050755.3 |
  144093681.3 |
  144136607.4 |
  144179533.5 |
  144222459.6 |
  144265385.6 |
  144308311.7 |
  144351237.8 |####################
  144394163.9 |
  144437089.9 |
  144480016.0 |########################################
  (0 below, 1 above range)

rust_std_par (n=8, range 119454253.4-127542320.3 ns)
  119454253.4 |####################
  119858656.7 |####################
  120263060.1 |
  120667463.4 |
  121071866.8 |
  121476270.1 |
  121880673.5 |
  122285076.8 |
  122689480.2 |
  123093883.5 |
  123498286.8 |
  123902690.2 |########################################
  124307093.5 |
  124711496.9 |
  125115900.2 |
  125520303.6 |
  125924706.9 |
  126329110.2 |
  126733513.6 |####################
  127137916.9 |####################
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=252.4% of algo (FFI overhead may distort results)
- engine_par: bridge=259.2% of algo (FFI overhead may distort results)
- hand: bridge=1148.1% of algo (FFI overhead may distort results)
- hand_par: bridge=957.6% of algo (FFI overhead may distort results)
- rust_nostd: bridge=210.1% of algo (FFI overhead may distort results)
- rust_std: bridge=130.9% of algo (FFI overhead may distort results)
- rust_std_par: bridge=152.1% of algo (FFI overhead may distort results)
