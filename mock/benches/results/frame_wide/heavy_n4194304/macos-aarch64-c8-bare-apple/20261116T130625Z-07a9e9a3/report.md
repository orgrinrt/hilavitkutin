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

# Wide frame, heavy per-record compute

7 variants, 8 samples per variant.
Baseline: hand

## Highlights

Baseline for all deltas below: hand. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hand) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hand has the worst median (53.45 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 11.02 ms).


### rust_std_par dominates: 13% faster than the next best (hand_par)

rust_std_par (11.02 ms) leads hand_par (12.40 ms) by 13%, a clear separation rather than a photo finish. CV 0.3%.


### rust_std_par beats baseline by 79% (significant)

rust_std_par is -42.42 ms (79%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.8x slower than the field

hand (53.45 ms) is 4.8x the fastest (11.02 ms), well off the pack.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (154% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 154% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.8x the fastest

Fastest rust_std_par (11.02 ms) to slowest hand (53.45 ms): 4.8x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 11022708.3 ns median (-79.4% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.85x (fastest 11022708.3 ns, slowest 53451187.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Wide frame, heavy per-record compute |
| Passes | 2 |
| Runs per pass | 20 |
| Batch size | 5 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0xf4a3e00000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 37457920ns | 37420029ns | 37327650ns | 37355736ns | 37567125ns | -29.93% |
| engine_par | 31568344ns | 31525529ns | 31508092ns | 31522628ns | 31617694ns | -40.94% |
| hand | 53455773ns | 53459292ns | 53417008ns | 53442525ns | 53475400ns | base |
| hand_par | 12414631ns | 12408254ns | 12264158ns | 12306947ns | 12533012ns | -76.78% |
| rust_nostd | 37138984ns | 37115479ns | 37082917ns | 37103872ns | 37179335ns | -30.52% |
| rust_std | 37354020ns | 37334125ns | 37294167ns | 37309122ns | 37402656ns | -30.12% |
| rust_std_par | 11031152ns | 11027188ns | 10986208ns | 11019703ns | 11050975ns | -79.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 37455386ns | 37325442ns | 37564440ns | -29.92% | 1.344 |
| engine_par | 31564405ns | 31504400ns | 31613638ns | -40.95% | 1.595 |
| hand | 53449353ns | 53412692ns | 53470123ns | base | 0.942 |
| hand_par | 12409248ns | 12258092ns | 12527698ns | -76.78% | 4.056 |
| rust_nostd | 37136574ns | 37080292ns | 37176975ns | -30.52% | 1.355 |
| rust_std | 37351867ns | 37292342ns | 37400619ns | -30.12% | 1.348 |
| rust_std_par | 11027387ns | 10983008ns | 11047323ns | -79.37% | 4.564 |

## Performance model

- Peak throughput: 4.583 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.345 | 29.4% |
| engine_par | 1.597 | 34.8% |
| hand | 0.942 | 20.5% |
| hand_par | 4.058 | 88.6% |
| rust_nostd | 1.356 | 29.6% |
| rust_std | 1.348 | 29.4% |
| rust_std_par | 4.566 | 99.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 37457920ns | 37457920ns | -29.93% |
| engine_par | 31568344ns | 31568344ns | -40.94% |
| hand | 53455773ns | 53455773ns | base |
| hand_par | 12414631ns | 12414631ns | -76.78% |
| rust_nostd | 37138984ns | 37138984ns | -30.52% |
| rust_std | 37354020ns | 37354020ns | -30.12% |
| rust_std_par | 11031152ns | 11031152ns | -79.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 53451188ns | base | --- | [53427567, 53468158] | --- | --- | --- | --- |
| engine | 37417421ns | -16022233.4ns (-30.0%) | [-16102125, -15962367]ns | [37345633, 37524992] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 31521483ns | -21914308.4ns (-41.0%) | [-21950200, -21893033]ns | [31517650, 31563375] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 12402371ns | -41047654.1ns (-76.8%) | [-41164725, -40909508]ns | [12265333, 12518058] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 37113300ns | -16327520.9ns (-30.5%) | [-16363083, -16242667]ns | [37095950, 37187392] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 37331896ns | -16117204.2ns (-30.2%) | [-16144083, -16055308]ns | [37299933, 37412850] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 11022708ns | -42417158.4ns (-79.4%) | [-42445533, -42396433]ns | [11010292, 11031133] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 53435792ns | -30.1% | -41.0% | -76.8% | -30.5% | -30.2% | -79.4% |
| 1 | 2 | 53467388ns | -29.8% | -41.0% | -76.9% | -30.5% | -30.1% | -79.4% |

Autocorrelation (lag-1) per-pass series:

| Variant | r₁ | note |
|---|---|---|
| engine | -0.500 | moderate- |
| engine_par | -0.500 | moderate- |
| hand | -0.500 | moderate- |
| hand_par | -0.500 | moderate- |
| rust_nostd | -0.500 | moderate- |
| rust_std | -0.500 | moderate- |
| rust_std_par | -0.500 | moderate- |

Consistency summary:

- engine: won 2/2, lost 0/2
- engine_par: won 2/2, lost 0/2
- hand_par: won 2/2, lost 0/2
- rust_nostd: won 2/2, lost 0/2
- rust_std: won 2/2, lost 0/2
- rust_std_par: won 2/2, lost 0/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 67465385.4ns | 37455386.5ns | 180.1% | HIGH |
| engine_par | 67484219.8ns | 31564405.2ns | 213.8% | HIGH |
| hand | 67820336.5ns | 53449353.1ns | 126.9% | HIGH |
| hand_par | 67880880.2ns | 12409247.9ns | 547.0% | HIGH |
| rust_nostd | 67363306.2ns | 37136574.0ns | 181.4% | HIGH |
| rust_std | 67358370.8ns | 37351866.7ns | 180.3% | HIGH |
| rust_std_par | 67463434.4ns | 11027387.5ns | 611.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 37325441.7-37564439.6 ns)
  37325441.7 |####################
  37337391.6 |####################
  37349341.5 |########################################
  37361291.4 |
  37373241.3 |
  37385191.2 |
  37397141.1 |
  37409091.0 |
  37421040.9 |
  37432990.8 |
  37444940.7 |
  37456890.5 |
  37468840.4 |####################
  37480790.3 |####################
  37492740.2 |
  37504690.1 |
  37516640.0 |####################
  37528589.9 |
  37540539.8 |
  37552489.7 |
  (0 below, 1 above range)

engine_par (n=8, range 31504400.0-31613637.5 ns)
  31504400.0 |#############
  31509861.9 |
  31515323.8 |########################################
  31520785.6 |#############
  31526247.5 |
  31531709.4 |#############
  31537171.2 |
  31542633.1 |
  31548095.0 |
  31553556.9 |
  31559018.8 |#############
  31564480.6 |
  31569942.5 |
  31575404.4 |
  31580866.2 |
  31586328.1 |
  31591790.0 |
  31597251.9 |
  31602713.8 |
  31608175.6 |
  (0 below, 1 above range)

hand (n=8, range 53412691.7-53470122.9 ns)
  53412691.7 |########################################
  53415563.3 |
  53418434.8 |
  53421306.4 |
  53424177.9 |
  53427049.5 |########################################
  53429921.1 |########################################
  53432792.6 |
  53435664.2 |
  53438535.7 |
  53441407.3 |########################################
  53444278.9 |
  53447150.4 |
  53450022.0 |
  53452893.5 |
  53455765.1 |########################################
  53458636.7 |
  53461508.2 |
  53464379.8 |########################################
  53467251.3 |########################################
  (0 below, 1 above range)

hand_par (n=8, range 12258091.7-12527697.9 ns)
  12258091.7 |########################################
  12271572.0 |####################
  12285052.3 |
  12298532.6 |
  12312012.9 |
  12325493.3 |
  12338973.6 |
  12352453.9 |####################
  12365934.2 |
  12379414.5 |
  12392894.8 |
  12406375.1 |
  12419855.4 |
  12433335.7 |
  12446816.1 |####################
  12460296.4 |
  12473776.7 |####################
  12487257.0 |
  12500737.3 |
  12514217.6 |####################
  (0 below, 1 above range)

rust_nostd (n=8, range 37080291.7-37176975.0 ns)
  37080291.7 |########################################
  37085125.9 |
  37089960.0 |
  37094794.2 |########################################
  37099628.4 |########################################
  37104462.5 |########################################
  37109296.7 |
  37114130.9 |
  37118965.0 |########################################
  37123799.2 |########################################
  37128633.4 |
  37133467.5 |
  37138301.7 |
  37143135.8 |
  37147970.0 |
  37152804.2 |
  37157638.3 |
  37162472.5 |
  37167306.7 |
  37172140.8 |
  (0 below, 2 above range)

rust_std (n=8, range 37292341.7-37400618.8 ns)
  37292341.7 |########################################
  37297755.6 |########################################
  37303169.4 |########################################
  37308583.3 |########################################
  37313997.1 |
  37319411.0 |
  37324824.8 |
  37330238.7 |
  37335652.5 |
  37341066.4 |
  37346480.2 |########################################
  37351894.1 |
  37357307.9 |
  37362721.8 |########################################
  37368135.6 |
  37373549.5 |
  37378963.3 |
  37384377.2 |
  37389791.0 |
  37395204.9 |
  (0 below, 2 above range)

rust_std_par (n=8, range 10983008.3-11047322.9 ns)
  10983008.3 |####################
  10986224.0 |
  10989439.8 |
  10992655.5 |
  10995871.2 |
  10999087.0 |
  11002302.7 |
  11005518.4 |
  11008734.1 |####################
  11011949.9 |
  11015165.6 |####################
  11018381.3 |####################
  11021597.1 |####################
  11024812.8 |
  11028028.5 |########################################
  11031244.2 |
  11034460.0 |
  11037675.7 |
  11040891.4 |
  11044107.2 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=180.0% of algo (FFI overhead may distort results)
- engine_par: bridge=213.9% of algo (FFI overhead may distort results)
- hand: bridge=126.9% of algo (FFI overhead may distort results)
- hand_par: bridge=547.3% of algo (FFI overhead may distort results)
- rust_nostd: bridge=181.4% of algo (FFI overhead may distort results)
- rust_std: bridge=180.4% of algo (FFI overhead may distort results)
- rust_std_par: bridge=610.9% of algo (FFI overhead may distort results)
