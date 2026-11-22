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

The declared/defaulted baseline hand has the worst median (218.90 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 50.20 ms).


### rust_std_par beats baseline by 77% (significant)

rust_std_par is -168.59 ms (77%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.4x slower than the field

hand (218.90 ms) is 4.4x the fastest (50.20 ms), well off the pack.


### rust_std_par is fastest but the noisiest (CV 10.0%)

rust_std_par wins on median (50.20 ms) yet has the highest variance (CV 10.0%), while hand is the steadiest (CV 0.4%, 218.90 ms).


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, engine, rust_std, hand} (148% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, engine, rust_std, hand} with a 148% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.4x the fastest

Fastest rust_std_par (50.20 ms) to slowest hand (218.90 ms): 4.4x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 50197037.5 ns median (-77.1% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.36x (fastest 50197037.5 ns, slowest 218902862.5 ns)

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
| engine | 154558147ns | 153996296ns | 152399550ns | 152784872ns | 156427752ns | -29.44% |
| engine_par | 130370435ns | 130137592ns | 128252767ns | 128634653ns | 132201690ns | -40.48% |
| hand | 219040894ns | 218906438ns | 218081250ns | 218265017ns | 219862712ns | base |
| hand_par | 52104807ns | 52505012ns | 49263708ns | 50672486ns | 53889323ns | -76.21% |
| rust_nostd | 153013241ns | 152379121ns | 151367425ns | 151504222ns | 154556458ns | -30.14% |
| rust_std | 153975530ns | 154402367ns | 152252008ns | 153112108ns | 155053977ns | -29.70% |
| rust_std_par | 49571474ns | 50201417ns | 43444933ns | 44987050ns | 54541427ns | -77.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 154553765ns | 152394650ns | 156423435ns | -29.44% | 1.303 |
| engine_par | 130366001ns | 128249592ns | 132196602ns | -40.48% | 1.544 |
| hand | 219036656ns | 218077717ns | 219858690ns | base | 0.919 |
| hand_par | 52100314ns | 49258775ns | 53884846ns | -76.21% | 3.864 |
| rust_nostd | 153008717ns | 151363358ns | 154552115ns | -30.14% | 1.316 |
| rust_std | 153970203ns | 152246775ns | 155048798ns | -29.71% | 1.308 |
| rust_std_par | 49566754ns | 43440017ns | 54536310ns | -77.37% | 4.062 |

## Performance model

- Peak throughput: 4.635 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 201326592

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.307 | 28.2% |
| engine_par | 1.547 | 33.4% |
| hand | 0.920 | 19.8% |
| hand_par | 3.835 | 82.7% |
| rust_nostd | 1.321 | 28.5% |
| rust_std | 1.304 | 28.1% |
| rust_std_par | 4.011 | 86.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 154558147ns | 154558147ns | -29.44% |
| engine_par | 130370435ns | 130370435ns | -40.48% |
| hand | 219040894ns | 219040894ns | base |
| hand_par | 52104807ns | 52104807ns | -76.21% |
| rust_nostd | 153013241ns | 153013241ns | -30.14% |
| rust_std | 153975530ns | 153975530ns | -29.70% |
| rust_std_par | 49571474ns | 49571474ns | -77.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 218902862ns | base | --- | [218108850, 219851642] | --- | --- | --- | --- |
| engine | 153992308ns | -65020429.2ns (-29.7%) | [-65714200, -63081575]ns | [152518575, 156884508] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 130132858ns | -88776608.4ns (-40.6%) | [-89994883, -87318433]ns | [128360867, 132282583] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 52500646ns | -166614079.2ns (-76.1%) | [-168507825, -165636725]ns | [49808350, 53888150] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 152374292ns | -66683166.7ns (-30.5%) | [-66885312, -66355742]ns | [151412783, 153645075] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 154398271ns | -65415395.8ns (-29.9%) | [-65862075, -64070117]ns | [152247483, 155263208] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 50197038ns | -168588841.7ns (-77.0%) | [-174287525, -165379742]ns | [44028650, 54471900] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 219657288ns | -28.7% | -39.8% | -75.5% | -30.1% | -29.4% | -75.2% |
| 1 | 2 | 218212512ns | -30.0% | -41.1% | -77.0% | -30.6% | -30.1% | -79.7% |

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
| engine | 277129970.8ns | 154553764.6ns | 179.3% | HIGH |
| engine_par | 276755307.3ns | 130366001.0ns | 212.3% | HIGH |
| hand | 277260597.9ns | 219036656.2ns | 126.6% | HIGH |
| hand_par | 276376768.7ns | 52100313.5ns | 530.5% | HIGH |
| rust_nostd | 277160680.2ns | 153008716.7ns | 181.1% | HIGH |
| rust_std | 276398021.9ns | 153970203.1ns | 179.5% | HIGH |
| rust_std_par | 277480539.6ns | 49566754.2ns | 559.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 152394650.0-156423435.4 ns)
  152394650.0 |########################################
  152596089.3 |
  152797528.5 |########################################
  152998967.8 |
  153200407.1 |
  153401846.3 |
  153603285.6 |
  153804724.9 |
  154006164.2 |
  154207603.4 |
  154409042.7 |
  154610482.0 |
  154811921.2 |
  155013360.5 |####################
  155214799.8 |
  155416239.1 |
  155617678.3 |
  155819117.6 |
  156020556.9 |
  156221996.1 |####################
  (0 below, 2 above range)

engine_par (n=8, range 128249591.7-132196602.1 ns)
  128249591.7 |########################################
  128446942.2 |####################
  128644292.7 |
  128841643.3 |####################
  129038993.8 |
  129236344.3 |
  129433694.8 |
  129631045.3 |
  129828395.9 |
  130025746.4 |
  130223096.9 |
  130420447.4 |
  130617797.9 |
  130815148.4 |
  131012499.0 |
  131209849.5 |####################
  131407200.0 |
  131604550.5 |
  131801901.0 |
  131999251.6 |####################
  (0 below, 2 above range)

hand (n=8, range 218077716.7-219858689.6 ns)
  218077716.7 |########################################
  218166765.3 |
  218255814.0 |####################
  218344862.6 |####################
  218433911.3 |
  218522959.9 |
  218612008.6 |
  218701057.2 |
  218790105.8 |
  218879154.5 |
  218968203.1 |
  219057251.8 |
  219146300.4 |
  219235349.1 |
  219324397.7 |
  219413446.4 |########################################
  219502495.0 |
  219591543.6 |
  219680592.3 |
  219769640.9 |####################
  (0 below, 1 above range)

hand_par (n=8, range 49258775.0-53884845.8 ns)
  49258775.0 |########################################
  49490078.5 |
  49721382.1 |########################################
  49952685.6 |
  50183989.2 |
  50415292.7 |########################################
  50646596.2 |
  50877899.8 |
  51109203.3 |
  51340506.9 |
  51571810.4 |########################################
  51803114.0 |
  52034417.5 |
  52265721.0 |
  52497024.6 |
  52728328.1 |
  52959631.7 |
  53190935.2 |########################################
  53422238.7 |
  53653542.3 |########################################
  (0 below, 2 above range)

rust_nostd (n=8, range 151363358.3-154552114.6 ns)
  151363358.3 |########################################
  151522796.1 |#############
  151682233.9 |
  151841671.7 |
  152001109.6 |
  152160547.4 |
  152319985.2 |
  152479423.0 |
  152638860.8 |
  152798298.6 |
  152957736.4 |#############
  153117174.3 |
  153276612.1 |
  153436049.9 |#############
  153595487.7 |#############
  153754925.5 |
  153914363.3 |
  154073801.1 |
  154233238.9 |
  154392676.8 |
  (0 below, 1 above range)

rust_std (n=8, range 152246775.0-155048797.9 ns)
  152246775.0 |########################################
  152386876.1 |
  152526977.3 |
  152667078.4 |
  152807179.6 |####################
  152947280.7 |
  153087381.9 |
  153227483.0 |
  153367584.2 |
  153507685.3 |
  153647786.4 |
  153787887.6 |
  153927988.7 |
  154068089.9 |
  154208191.0 |####################
  154348292.2 |
  154488393.3 |####################
  154628494.5 |####################
  154768595.6 |
  154908696.8 |
  (0 below, 2 above range)

rust_std_par (n=8, range 43440016.7-54536310.4 ns)
  43440016.7 |#############
  43994831.4 |##########################
  44549646.1 |
  45104460.8 |
  45659275.4 |
  46214090.1 |#############
  46768904.8 |
  47323719.5 |
  47878534.2 |
  48433348.9 |
  48988163.6 |
  49542978.2 |
  50097792.9 |
  50652607.6 |
  51207422.3 |
  51762237.0 |
  52317051.7 |
  52871866.4 |
  53426681.1 |
  53981495.7 |########################################
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=179.9% of algo (FFI overhead may distort results)
- engine_par: bridge=212.8% of algo (FFI overhead may distort results)
- hand: bridge=126.6% of algo (FFI overhead may distort results)
- hand_par: bridge=526.4% of algo (FFI overhead may distort results)
- rust_nostd: bridge=181.8% of algo (FFI overhead may distort results)
- rust_std: bridge=179.2% of algo (FFI overhead may distort results)
- rust_std_par: bridge=552.9% of algo (FFI overhead may distort results)
