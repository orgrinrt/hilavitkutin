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

The declared/defaulted baseline hand has the worst median (155.21 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hand_par at 31.54 ms).


### hand_par beats baseline by 80% (significant)

hand_par is -123.83 ms (80%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.9x slower than the field

hand (155.21 ms) is 4.9x the fastest (31.54 ms), well off the pack.


### hand_par is fastest but the noisiest (CV 33.3%)

hand_par wins on median (31.54 ms) yet has the highest variance (CV 33.3%), while hand is the steadiest (CV 0.6%, 155.21 ms).


### rust_nostd shows alternating (throttle bounce) (autocorr -0.50)

rust_nostd's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, rust_std_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (258% apart)

The field splits into a fast tier {hand_par, rust_std_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 258% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.9x the fastest

Fastest hand_par (31.54 ms) to slowest hand (155.21 ms): 4.9x. The strategy choice matters a lot for this workload.


### hand_par is inconsistent: worst-20% is 1.7x its best-20%

hand_par's best 20% of batches run at 25.17 ms but its worst 20% at 43.40 ms (1.7x) - a bimodal or bursty profile the median hides.


## Key findings

- Fastest: hand_par at 31537753.6 ns median (-79.7% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.92x (fastest 31537753.6 ns, slowest 155209930.2 ns)

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
| Counter frequency | 2112801267 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 129405384ns | 129952609ns | 127433396ns | 128884274ns | 130289214ns | -16.82% |
| engine_par | 115355966ns | 115147422ns | 109735913ns | 114102955ns | 117700738ns | -25.85% |
| hand | 155580442ns | 155211288ns | 155048694ns | 155123482ns | 156056098ns | base |
| hand_par | 35353134ns | 31541068ns | 25172378ns | 28016694ns | 43400654ns | -77.28% |
| rust_nostd | 128469938ns | 127326881ns | 127085130ns | 127264413ns | 129720283ns | -17.43% |
| rust_std | 147831865ns | 147399262ns | 144647462ns | 146256393ns | 149809570ns | -4.98% |
| rust_std_par | 32182774ns | 32182029ns | 31043191ns | 31794554ns | 32758836ns | -79.31% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 129404231ns | 127432535ns | 130288131ns | -16.82% | 0.389 |
| engine_par | 115353352ns | 109733387ns | 117698125ns | -25.86% | 0.436 |
| hand | 155578851ns | 155047613ns | 156054490ns | base | 0.324 |
| hand_par | 35349786ns | 25169090ns | 43397185ns | -77.28% | 1.424 |
| rust_nostd | 128468362ns | 127082925ns | 129718960ns | -17.43% | 0.392 |
| rust_std | 147830217ns | 144646231ns | 149808019ns | -4.98% | 0.340 |
| rust_std_par | 32180028ns | 31040601ns | 32756029ns | -79.32% | 1.564 |

## Performance model

- Peak throughput: 2.000 Gops/s (hand_par; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 0.387 | 19.4% |
| engine_par | 0.437 | 21.9% |
| hand | 0.324 | 16.2% |
| hand_par | 1.596 | 79.8% |
| rust_nostd | 0.395 | 19.8% |
| rust_std | 0.341 | 17.1% |
| rust_std_par | 1.564 | 78.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 129405384ns | 129405384ns | -16.82% |
| engine_par | 115355966ns | 115355966ns | -25.85% |
| hand | 155580442ns | 155580442ns | base |
| hand_par | 35353134ns | 35353134ns | -77.28% |
| rust_nostd | 128469938ns | 128469938ns | -17.43% |
| rust_std | 147831865ns | 147831865ns | -4.98% |
| rust_std_par | 32182774ns | 32182774ns | -79.31% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 155209930ns | base | --- | [155084319, 155598087] | --- | --- | --- | --- |
| engine | 129951212ns | -25690975.7ns (-16.6%) | [-27615078, -25249253]ns | [127510183, 130364578] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 115144816ns | -40222921.2ns (-25.9%) | [-42693538, -37132375]ns | [112390781, 117915239] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 31537754ns | -123833703.3ns (-79.8%) | [-129211764, -110175186]ns | [25872556, 44960900] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 127325403ns | -27930364.2ns (-18.0%) | [-28285552, -25017828]ns | [127198843, 130126999] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 147397458ns | -7753532.2ns (-5.0%) | [-10498596, -5842822]ns | [144655193, 149204792] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 32179004ns | -123119430.1ns (-79.3%) | [-124557486, -122615798]ns | [31581816, 32659235] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 155333514ns | -17.0% | -24.4% | -77.2% | -17.0% | -4.6% | -79.3% |
| 1 | 2 | 155205560ns | -16.3% | -26.8% | -81.9% | -18.0% | -5.1% | -79.3% |

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
| engine | 47481676.7ns | 129404231.2ns | 36.7% | HIGH |
| engine_par | 48764517.8ns | 115353352.5ns | 42.3% | HIGH |
| hand | 47689160.9ns | 155578850.7ns | 30.7% | HIGH |
| hand_par | 48934149.6ns | 35349785.7ns | 138.4% | HIGH |
| rust_nostd | 47417614.1ns | 128468361.8ns | 36.9% | HIGH |
| rust_std | 47428905.1ns | 147830217.0ns | 32.1% | HIGH |
| rust_std_par | 49270751.3ns | 32180027.9ns | 153.1% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 127432535.2-130288130.7 ns)
  127432535.2 |########################################
  127575315.0 |
  127718094.8 |
  127860874.5 |
  128003654.3 |
  128146434.1 |
  128289213.9 |
  128431993.6 |
  128574773.4 |
  128717553.2 |
  128860333.0 |
  129003112.7 |
  129145892.5 |####################
  129288672.3 |
  129431452.0 |
  129574231.8 |
  129717011.6 |
  129859791.4 |####################
  130002571.2 |########################################
  130145350.9 |
  (0 below, 2 above range)

engine_par (n=8, range 109733386.8-117698124.8 ns)
  109733386.8 |####################
  110131623.7 |
  110529860.6 |
  110928097.5 |
  111326334.4 |
  111724571.3 |
  112122808.2 |####################
  112521045.1 |
  112919282.0 |
  113317518.9 |
  113715755.8 |
  114113992.7 |
  114512229.6 |####################
  114910466.5 |########################################
  115308703.4 |
  115706940.3 |
  116105177.2 |
  116503414.1 |
  116901651.0 |####################
  117299887.9 |
  (0 below, 2 above range)

hand (n=8, range 155047613.4-156054490.0 ns)
  155047613.4 |########################################
  155097957.2 |########################################
  155148301.1 |
  155198644.9 |
  155248988.7 |####################
  155299332.6 |
  155349676.4 |
  155400020.2 |
  155450364.0 |
  155500707.9 |####################
  155551051.7 |####################
  155601395.5 |
  155651739.4 |
  155702083.2 |
  155752427.0 |
  155802770.9 |
  155853114.7 |
  155903458.5 |
  155953802.4 |
  156004146.2 |
  (0 below, 1 above range)

hand_par (n=8, range 25169090.1-43397185.0 ns)
  25169090.1 |########################################
  26080494.8 |
  26991899.6 |####################
  27903304.3 |
  28814709.1 |
  29726113.8 |####################
  30637518.6 |
  31548923.3 |
  32460328.1 |####################
  33371732.8 |
  34283137.6 |
  35194542.3 |
  36105947.1 |
  37017351.8 |
  37928756.5 |####################
  38840161.3 |
  39751566.0 |
  40662970.8 |
  41574375.5 |
  42485780.3 |
  (0 below, 2 above range)

rust_nostd (n=8, range 127082925.4-129718959.6 ns)
  127082925.4 |##########################
  127214727.1 |########################################
  127346528.8 |
  127478330.5 |#############
  127610132.2 |
  127741934.0 |
  127873735.7 |
  128005537.4 |
  128137339.1 |
  128269140.8 |
  128400942.5 |
  128532744.2 |
  128664545.9 |
  128796347.6 |
  128928149.4 |
  129059951.1 |
  129191752.8 |
  129323554.5 |
  129455356.2 |
  129587157.9 |
  (0 below, 2 above range)

rust_std (n=8, range 144646231.3-149808019.0 ns)
  144646231.3 |########################################
  144904320.7 |
  145162410.1 |
  145420499.5 |
  145678588.8 |
  145936678.2 |
  146194767.6 |
  146452857.0 |
  146710946.4 |
  146969035.8 |########################################
  147227125.2 |
  147485214.5 |####################
  147743303.9 |
  148001393.3 |
  148259482.7 |
  148517572.1 |
  148775661.5 |
  149033750.8 |########################################
  149291840.2 |
  149549929.6 |
  (0 below, 1 above range)

rust_std_par (n=8, range 31040601.2-32756028.5 ns)
  31040601.2 |########################################
  31126372.6 |
  31212143.9 |
  31297915.3 |
  31383686.7 |
  31469458.0 |
  31555229.4 |########################################
  31641000.8 |
  31726772.1 |
  31812543.5 |########################################
  31898314.9 |########################################
  31984086.2 |
  32069857.6 |
  32155629.0 |
  32241400.3 |
  32327171.7 |########################################
  32412943.1 |########################################
  32498714.4 |
  32584485.8 |########################################
  32670257.2 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=36.5% of algo (FFI overhead may distort results)
- engine_par: bridge=41.8% of algo (FFI overhead may distort results)
- hand: bridge=30.5% of algo (FFI overhead may distort results)
- hand_par: CV=29.7% (high variance, measurements may be unstable)
- hand_par: bridge=152.7% of algo (FFI overhead may distort results)
- rust_nostd: bridge=37.2% of algo (FFI overhead may distort results)
- rust_std: bridge=32.1% of algo (FFI overhead may distort results)
- rust_std_par: bridge=150.0% of algo (FFI overhead may distort results)
