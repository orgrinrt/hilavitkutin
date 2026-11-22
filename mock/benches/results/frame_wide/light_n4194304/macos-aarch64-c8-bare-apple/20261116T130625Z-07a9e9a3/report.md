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

### rust_std_par is an outlier: 5.7x slower than the field

rust_std_par (10.50 ms) is 5.7x the fastest (1.83 ms), well off the pack.


### Top two (hand_par, hand) are a dead heat (<1%)

hand_par (1.83 ms) and hand (1.84 ms) differ by 0.48%, inside the noise, even though the wider field spreads 474.4%.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### Two tiers: {hand_par, hand} vs {rust_nostd, rust_std, engine_par, engine, rust_std_par} (301% apart)

The field splits into a fast tier {hand_par, hand} and a slow tier {rust_nostd, rust_std, engine_par, engine, rust_std_par} with a 301% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.7x the fastest

Fastest hand_par (1.83 ms) to slowest rust_std_par (10.50 ms): 5.7x. The strategy choice matters a lot for this workload.


## Key findings

- Fastest: hand_par at 1827554.1 ns median (-0.5% vs baseline)
- 5 variants significantly slower than baseline
- Spread: 5.74x (fastest 1827554.1 ns, slowest 10496912.5 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| engine | 8325677ns | 8263725ns | 8135450ns | 8186083ns | 8477929ns | +348.32% |
| engine_par | 8011725ns | 7913229ns | 7867742ns | 7892939ns | 8136810ns | +331.42% |
| hand | 1857076ns | 1838146ns | 1824758ns | 1832256ns | 1883771ns | base |
| hand_par | 1839160ns | 1831208ns | 1812450ns | 1827311ns | 1854725ns | -0.96% |
| rust_nostd | 7354681ns | 7364504ns | 7317942ns | 7339497ns | 7375254ns | +296.04% |
| rust_std | 7378008ns | 7380092ns | 7309825ns | 7350419ns | 7415746ns | +297.29% |
| rust_std_par | 10488412ns | 10500612ns | 10389583ns | 10433125ns | 10554585ns | +464.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 8323490ns | 8133183ns | 8475681ns | +348.77% | 6.047 |
| engine_par | 8007500ns | 7862333ns | 8132690ns | +331.73% | 6.286 |
| hand | 1854738ns | 1821950ns | 1881425ns | base | 27.137 |
| hand_par | 1835092ns | 1807108ns | 1850454ns | -1.06% | 27.427 |
| rust_nostd | 7352418ns | 7315208ns | 7372762ns | +296.41% | 6.846 |
| rust_std | 7375655ns | 7307642ns | 7413654ns | +297.67% | 6.824 |
| rust_std_par | 10484770ns | 10385083ns | 10551017ns | +465.30% | 4.800 |

## Performance model

- Peak throughput: 27.852 Gops/s (hand_par; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.092 | 21.9% |
| engine_par | 6.363 | 22.8% |
| hand | 27.408 | 98.4% |
| hand_par | 27.540 | 98.9% |
| rust_nostd | 6.836 | 24.5% |
| rust_std | 6.822 | 24.5% |
| rust_std_par | 4.795 | 17.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 8325677ns | 8325677ns | +348.32% |
| engine_par | 8011725ns | 8011725ns | +331.42% |
| hand | 1857076ns | 1857076ns | base |
| hand_par | 1839160ns | 1839160ns | -0.96% |
| rust_nostd | 7354681ns | 7354681ns | +296.04% |
| rust_std | 7378008ns | 7378008ns | +297.29% |
| rust_std_par | 10488412ns | 10488412ns | +464.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 1836354ns | base | --- | [1825592, 1884183] | --- | --- | --- | --- |
| engine | 8261250ns | +6390366.7ns (+348.0%) | [+6312050, +6504050]ns | [8168225, 8349350] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 7910525ns | +6079675.0ns (+331.1%) | [+6053275, +6255517]ns | [7884975, 8091908] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 1827554ns | no significant difference | [-54317, +4875]ns | [1821950, 1829867] | no | 0.2891 | 0.2891 | 0 |
| rust_nostd | 7362700ns | +5510316.7ns (+300.1%) | [+5449717, +5544425]ns | [7320767, 7376125] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 7377742ns | +5513191.7ns (+300.2%) | [+5470025, +5577108]ns | [7328725, 7415617] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 10496912ns | +8611033.3ns (+468.9%) | [+8549008, +8721783]ns | [10405183, 10568692] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1836354ns | +348.8% | +335.9% | -0.6% | +299.7% | +301.4% | +474.4% |
| 1 | 2 | 1857942ns | +345.6% | +324.6% | -1.6% | +296.5% | +297.3% | +462.0% |

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

- engine: won 0/2, lost 2/2
- engine_par: won 0/2, lost 2/2
- hand_par: won 2/2, lost 0/2
- rust_nostd: won 0/2, lost 2/2
- rust_std: won 0/2, lost 2/2
- rust_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 67421775.0ns | 8323489.6ns | 810.0% | HIGH |
| engine_par | 67402509.4ns | 8007500.0ns | 841.7% | HIGH |
| hand | 67463719.8ns | 1854737.5ns | 3637.4% | HIGH |
| hand_par | 67449934.4ns | 1835091.7ns | 3675.6% | HIGH |
| rust_nostd | 67514414.6ns | 7352417.7ns | 918.3% | HIGH |
| rust_std | 67403242.7ns | 7375655.2ns | 913.9% | HIGH |
| rust_std_par | 67449129.2ns | 10484769.8ns | 643.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 8133183.3-8475681.2 ns)
  8133183.3 |####################
  8150308.2 |
  8167433.1 |########################################
  8184558.0 |
  8201682.9 |####################
  8218807.8 |
  8235932.7 |
  8253057.6 |
  8270182.5 |
  8287307.4 |
  8304432.3 |####################
  8321557.2 |
  8338682.1 |########################################
  8355807.0 |
  8372931.9 |
  8390056.8 |
  8407181.7 |
  8424306.6 |
  8441431.5 |
  8458556.4 |
  (0 below, 1 above range)

engine_par (n=8, range 7862333.3-8132689.6 ns)
  7862333.3 |####################
  7875851.1 |########################################
  7889368.9 |####################
  7902886.7 |
  7916404.6 |####################
  7929922.4 |
  7943440.2 |
  7956958.0 |
  7970475.8 |
  7983993.6 |
  7997511.4 |
  8011029.3 |
  8024547.1 |
  8038064.9 |
  8051582.7 |
  8065100.5 |
  8078618.3 |########################################
  8092136.1 |
  8105653.9 |
  8119171.8 |
  (0 below, 1 above range)

hand (n=8, range 1821950.0-1881425.0 ns)
  1821950.0 |####################
  1824923.8 |####################
  1827897.5 |
  1830871.2 |########################################
  1833845.0 |
  1836818.8 |####################
  1839792.5 |
  1842766.2 |
  1845740.0 |
  1848713.8 |
  1851687.5 |
  1854661.2 |####################
  1857635.0 |
  1860608.8 |
  1863582.5 |
  1866556.2 |
  1869530.0 |
  1872503.8 |
  1875477.5 |
  1878451.2 |
  (0 below, 2 above range)

hand_par (n=8, range 1807108.3-1850454.2 ns)
  1807108.3 |####################
  1809275.6 |
  1811442.9 |
  1813610.2 |
  1815777.5 |
  1817944.8 |
  1820112.1 |####################
  1822279.4 |####################
  1824446.7 |
  1826613.9 |########################################
  1828781.2 |########################################
  1830948.5 |
  1833115.8 |
  1835283.1 |
  1837450.4 |
  1839617.7 |
  1841785.0 |
  1843952.3 |
  1846119.6 |
  1848286.9 |
  (0 below, 1 above range)

rust_nostd (n=8, range 7315208.3-7372762.5 ns)
  7315208.3 |########################################
  7318086.0 |########################################
  7320963.7 |
  7323841.4 |
  7326719.1 |
  7329596.8 |
  7332474.6 |########################################
  7335352.3 |
  7338230.0 |
  7341107.7 |
  7343985.4 |
  7346863.1 |
  7349740.8 |
  7352618.5 |
  7355496.2 |
  7358374.0 |########################################
  7361251.7 |
  7364129.4 |########################################
  7367007.1 |########################################
  7369884.8 |
  (0 below, 2 above range)

rust_std (n=8, range 7307641.7-7413654.2 ns)
  7307641.7 |########################################
  7312942.3 |
  7318242.9 |
  7323543.6 |########################################
  7328844.2 |
  7334144.8 |
  7339445.4 |
  7344746.1 |
  7350046.7 |########################################
  7355347.3 |
  7360647.9 |########################################
  7365948.5 |
  7371249.2 |
  7376549.8 |
  7381850.4 |
  7387151.0 |########################################
  7392451.7 |
  7397752.3 |########################################
  7403052.9 |
  7408353.5 |
  (0 below, 2 above range)

rust_std_par (n=8, range 10385083.3-10551016.7 ns)
  10385083.3 |####################
  10393380.0 |
  10401676.6 |########################################
  10409973.3 |
  10418270.0 |
  10426566.6 |
  10434863.3 |
  10443160.0 |
  10451456.7 |
  10459753.3 |
  10468050.0 |####################
  10476346.7 |
  10484643.3 |
  10492940.0 |
  10501236.7 |
  10509533.3 |
  10517830.0 |####################
  10526126.7 |####################
  10534423.3 |
  10542720.0 |
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=815.8% of algo (FFI overhead may distort results)
- engine_par: bridge=851.8% of algo (FFI overhead may distort results)
- hand: bridge=3667.5% of algo (FFI overhead may distort results)
- hand_par: bridge=3684.1% of algo (FFI overhead may distort results)
- rust_nostd: bridge=916.0% of algo (FFI overhead may distort results)
- rust_std: bridge=913.0% of algo (FFI overhead may distort results)
- rust_std_par: bridge=641.6% of algo (FFI overhead may distort results)
