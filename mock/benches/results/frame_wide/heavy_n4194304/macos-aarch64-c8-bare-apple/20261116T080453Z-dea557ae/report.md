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

The declared/defaulted baseline hand has the worst median (55.06 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rust_std_par at 13.73 ms).


### rust_std_par beats baseline by 75% (significant)

rust_std_par is -41.44 ms (75%) faster than baseline hand, with a CI that excludes zero.


### hand is an outlier: 4.0x slower than the field

hand (55.06 ms) is 4.0x the fastest (13.73 ms), well off the pack.


### rust_std_par is fastest but the noisiest (CV 15.8%)

rust_std_par wins on median (13.73 ms) yet has the highest variance (CV 15.8%), while rust_std is the steadiest (CV 0.4%, 38.68 ms).


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {rust_std_par, hand_par} vs {engine_par, rust_nostd, rust_std, engine, hand} (131% apart)

The field splits into a fast tier {rust_std_par, hand_par} and a slow tier {engine_par, rust_nostd, rust_std, engine, hand} with a 131% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 4.0x the fastest

Fastest rust_std_par (13.73 ms) to slowest hand (55.06 ms): 4.0x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: rust_std_par at 13729487.5 ns median (-75.1% vs baseline)
- 6 variants significantly faster than baseline
- Spread: 4.01x (fastest 13729487.5 ns, slowest 55063100.0 ns)

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
| engine | 39089710ns | 38972454ns | 38862442ns | 38934922ns | 39262619ns | -29.01% |
| engine_par | 33252008ns | 33216425ns | 32908800ns | 33138414ns | 33423006ns | -39.61% |
| hand | 55060145ns | 55068121ns | 54725017ns | 54872172ns | 55284906ns | base |
| hand_par | 14624656ns | 14370758ns | 13935983ns | 14130819ns | 15167202ns | -73.44% |
| rust_nostd | 38495553ns | 38324158ns | 38208342ns | 38268631ns | 38737548ns | -30.08% |
| rust_std | 38764053ns | 38682317ns | 38643533ns | 38663361ns | 38869702ns | -29.60% |
| rust_std_par | 14622829ns | 13732783ns | 13293467ns | 13460936ns | 15826590ns | -73.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 39086231ns | 38858733ns | 39259046ns | -29.01% | 1.288 |
| engine_par | 33248395ns | 32905042ns | 33419304ns | -39.61% | 1.514 |
| hand | 55055683ns | 54722050ns | 55280175ns | base | 0.914 |
| hand_par | 14621234ns | 13932608ns | 15163781ns | -73.44% | 3.442 |
| rust_nostd | 38491617ns | 38205275ns | 38733308ns | -30.09% | 1.308 |
| rust_std | 38760456ns | 38640275ns | 38865923ns | -29.60% | 1.299 |
| rust_std_par | 14619310ns | 13290342ns | 15822863ns | -73.45% | 3.443 |

## Performance model

- Peak throughput: 3.787 Gops/s (rust_std_par; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 1.292 | 34.1% |
| engine_par | 1.515 | 40.0% |
| hand | 0.914 | 24.1% |
| hand_par | 3.503 | 92.5% |
| rust_nostd | 1.313 | 34.7% |
| rust_std | 1.301 | 34.4% |
| rust_std_par | 3.666 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 39089710ns | 39089710ns | -29.01% |
| engine_par | 33252008ns | 33252008ns | -39.61% |
| hand | 55060145ns | 55060145ns | base |
| hand_par | 14624656ns | 14624656ns | -73.44% |
| rust_nostd | 38495553ns | 38495553ns | -30.08% |
| rust_std | 38764053ns | 38764053ns | -29.60% |
| rust_std_par | 14622829ns | 14622829ns | -73.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 55063100ns | base | --- | [54766392, 55269442] | --- | --- | --- | --- |
| engine | 38968446ns | -15864283.4ns (-28.8%) | [-16183133, -15837400]ns | [38900908, 39406358] | YES | 0.0078 | 0.0078 | 0 |
| engine_par | 33212471ns | -21713116.6ns (-39.4%) | [-21970025, -21640192]ns | [33081858, 33600383] | YES | 0.0078 | 0.0078 | 0 |
| hand_par | 14367383ns | -40565054.1ns (-73.7%) | [-40833783, -39713267]ns | [13973133, 15801892] | YES | 0.0078 | 0.0078 | 0 |
| rust_nostd | 38320029ns | -16633312.5ns (-30.2%) | [-16984267, -16420675]ns | [38243058, 38590325] | YES | 0.0078 | 0.0078 | 0 |
| rust_std | 38678679ns | -16183750.0ns (-29.4%) | [-16473367, -16102675]ns | [38650892, 38820225] | YES | 0.0078 | 0.0078 | 0 |
| rust_std_par | 13729488ns | -41438645.9ns (-75.3%) | [-41720658, -39682042]ns | [13416967, 15084350] | YES | 0.0078 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 55217546ns | -28.8% | -39.5% | -72.6% | -30.6% | -29.7% | -75.6% |
| 1 | 2 | 54795858ns | -28.9% | -39.5% | -74.1% | -30.0% | -29.5% | -73.5% |

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
| engine | 69391294.8ns | 39086231.2ns | 177.5% | HIGH |
| engine_par | 69334800.0ns | 33248394.8ns | 208.5% | HIGH |
| hand | 69764943.8ns | 55055683.3ns | 126.7% | HIGH |
| hand_par | 69709609.4ns | 14621234.4ns | 476.8% | HIGH |
| rust_nostd | 69663550.0ns | 38491616.7ns | 181.0% | HIGH |
| rust_std | 69520934.4ns | 38760456.2ns | 179.4% | HIGH |
| rust_std_par | 69710720.8ns | 14619310.4ns | 476.8% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 38858733.3-39259045.8 ns)
  38858733.3 |####################
  38878748.9 |
  38898764.6 |####################
  38918780.2 |####################
  38938795.8 |
  38958811.4 |########################################
  38978827.1 |
  38998842.7 |
  39018858.3 |
  39038873.9 |
  39058889.6 |
  39078905.2 |
  39098920.8 |
  39118936.4 |
  39138952.1 |
  39158967.7 |####################
  39178983.3 |
  39198998.9 |
  39219014.6 |
  39239030.2 |
  (0 below, 2 above range)

engine_par (n=8, range 32905041.7-33419304.2 ns)
  32905041.7 |########################################
  32930754.8 |
  32956467.9 |
  32982181.1 |
  33007894.2 |
  33033607.3 |
  33059320.4 |########################################
  33085033.6 |
  33110746.7 |########################################
  33136459.8 |
  33162172.9 |
  33187886.1 |########################################
  33213599.2 |########################################
  33239312.3 |########################################
  33265025.4 |
  33290738.6 |
  33316451.7 |
  33342164.8 |
  33367877.9 |
  33393591.1 |
  (0 below, 2 above range)

hand (n=8, range 54722050.0-55280175.0 ns)
  54722050.0 |########################################
  54749956.2 |########################################
  54777862.5 |
  54805768.8 |########################################
  54833675.0 |
  54861581.2 |
  54889487.5 |
  54917393.8 |
  54945300.0 |
  54973206.2 |
  55001112.5 |########################################
  55029018.8 |
  55056925.0 |
  55084831.2 |
  55112737.5 |########################################
  55140643.8 |########################################
  55168550.0 |
  55196456.2 |
  55224362.5 |
  55252268.8 |########################################
  (0 below, 1 above range)

hand_par (n=8, range 13932608.3-15163781.3 ns)
  13932608.3 |########################################
  13994166.9 |
  14055725.6 |####################
  14117284.2 |
  14178842.9 |
  14240401.5 |
  14301960.2 |####################
  14363518.8 |####################
  14425077.5 |
  14486636.1 |
  14548194.8 |####################
  14609753.4 |
  14671312.1 |
  14732870.7 |
  14794429.4 |
  14855988.0 |
  14917546.7 |
  14979105.3 |
  15040664.0 |
  15102222.6 |
  (0 below, 2 above range)

rust_nostd (n=8, range 38205275.0-38733308.3 ns)
  38205275.0 |########################################
  38231676.7 |########################################
  38258078.3 |########################################
  38284480.0 |########################################
  38310881.7 |
  38337283.3 |########################################
  38363685.0 |
  38390086.7 |########################################
  38416488.3 |
  38442890.0 |
  38469291.7 |
  38495693.3 |
  38522095.0 |
  38548496.7 |
  38574898.3 |########################################
  38601300.0 |
  38627701.7 |
  38654103.3 |
  38680505.0 |
  38706906.7 |
  (0 below, 1 above range)

rust_std (n=8, range 38640275.0-38865922.9 ns)
  38640275.0 |########################################
  38651557.4 |
  38662839.8 |########################################
  38674122.2 |
  38685404.6 |####################
  38696687.0 |
  38707969.4 |
  38719251.8 |
  38730534.2 |
  38741816.6 |
  38753099.0 |
  38764381.3 |
  38775663.7 |
  38786946.1 |
  38798228.5 |
  38809510.9 |########################################
  38820793.3 |
  38832075.7 |
  38843358.1 |
  38854640.5 |
  (0 below, 1 above range)

rust_std_par (n=8, range 13290341.7-15822862.5 ns)
  13290341.7 |########################################
  13416967.7 |########################################
  13543593.8 |
  13670219.8 |
  13796845.9 |
  13923471.9 |####################
  14050097.9 |####################
  14176724.0 |
  14303350.0 |
  14429976.1 |
  14556602.1 |
  14683228.2 |
  14809854.2 |
  14936480.2 |
  15063106.3 |####################
  15189732.3 |
  15316358.4 |
  15442984.4 |
  15569610.4 |
  15696236.5 |
  (0 below, 1 above range)

```

## Diagnostics

- engine: bridge=178.1% of algo (FFI overhead may distort results)
- engine_par: bridge=208.7% of algo (FFI overhead may distort results)
- hand: bridge=126.2% of algo (FFI overhead may distort results)
- hand_par: bridge=484.8% of algo (FFI overhead may distort results)
- rust_nostd: bridge=181.6% of algo (FFI overhead may distort results)
- rust_std: bridge=179.8% of algo (FFI overhead may distort results)
- rust_std_par: bridge=508.1% of algo (FFI overhead may distort results)
