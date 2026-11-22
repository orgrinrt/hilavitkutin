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

### rust_std_par is an outlier: 5.8x slower than the field

rust_std_par (10.54 ms) is 5.8x the fastest (1.82 ms), well off the pack.


### Top two (hand, hand_par) are a dead heat (<1%)

hand (1.82 ms) and hand_par (1.83 ms) differ by 0.21%, inside the noise, even though the wider field spreads 478.9%.


### rust_std_par shows alternating (throttle bounce) (autocorr -0.50)

rust_std_par's per-pass series has lag-1 autocorrelation -0.50, indicating alternating (throttle bounce). Its timing may not be at steady state.


### No variant beats the baseline (hand)

The baseline hand is the fastest (1.82 ms median); no rival improves on it (all deltas are >= 0).


### Two tiers: {hand, hand_par} vs {rust_nostd, rust_std, engine_par, engine, rust_std_par} (308% apart)

The field splits into a fast tier {hand, hand_par} and a slow tier {rust_nostd, rust_std, engine_par, engine, rust_std_par} with a 308% jump between them - a qualitative difference, not a gradient.


### Wide spread: slowest is 5.8x the fastest

Fastest hand (1.82 ms) to slowest rust_std_par (10.54 ms): 5.8x. The strategy choice matters a lot for this workload.


## Key findings

- Baseline (hand) is the fastest at 1821341.7 ns median
- 5 variants significantly slower than baseline
- Spread: 5.79x (fastest 1821341.7 ns, slowest 10543400.0 ns)

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
| engine | 8382579ns | 8251504ns | 8116050ns | 8190594ns | 8593200ns | +352.04% |
| engine_par | 8044259ns | 8040033ns | 7778908ns | 7906961ns | 8213571ns | +333.79% |
| hand | 1854400ns | 1824504ns | 1819375ns | 1822036ns | 1887429ns | base |
| hand_par | 1841610ns | 1829342ns | 1825325ns | 1826544ns | 1856981ns | -0.69% |
| rust_nostd | 7540785ns | 7453071ns | 7294525ns | 7386108ns | 7718358ns | +306.64% |
| rust_std | 7472677ns | 7476888ns | 7310467ns | 7362525ns | 7595844ns | +302.97% |
| rust_std_par | 10553814ns | 10547212ns | 10465475ns | 10509317ns | 10609271ns | +469.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| engine | 8380083ns | 8113150ns | 8590585ns | +352.50% | 6.006 |
| engine_par | 8040145ns | 7774475ns | 8209262ns | +334.15% | 6.260 |
| hand | 1851949ns | 1817342ns | 1885025ns | base | 27.178 |
| hand_par | 1837736ns | 1821217ns | 1853260ns | -0.77% | 27.388 |
| rust_nostd | 7538200ns | 7292167ns | 7715779ns | +307.04% | 6.677 |
| rust_std | 7470297ns | 7309000ns | 7593181ns | +303.37% | 6.738 |
| rust_std_par | 10550013ns | 10461883ns | 10605290ns | +469.67% | 4.771 |

## Performance model

- Peak throughput: 27.695 Gops/s (hand; best 20% batches)
- Ops per call: 50331648

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| engine | 6.102 | 22.0% |
| engine_par | 6.263 | 22.6% |
| hand | 27.634 | 99.8% |
| hand_par | 27.576 | 99.6% |
| rust_nostd | 6.755 | 24.4% |
| rust_std | 6.734 | 24.3% |
| rust_std_par | 4.774 | 17.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| engine | 8382579ns | 8382579ns | +352.04% |
| engine_par | 8044259ns | 8044259ns | +333.79% |
| hand | 1854400ns | 1854400ns | base |
| hand_par | 1841610ns | 1841610ns | -0.69% |
| rust_nostd | 7540785ns | 7540785ns | +306.64% |
| rust_std | 7472677ns | 7472677ns | +302.97% |
| rust_std_par | 10553814ns | 10553814ns | +469.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hand | 1821342ns | base | --- | [1818375, 1906158] | --- | --- | --- | --- |
| engine | 8248796ns | +6424962.6ns (+352.8%) | [+6290858, +6939242]ns | [8166375, 8764583] | YES | 0.0094 | 0.0078 | 0 |
| engine_par | 8036283ns | +6218425.0ns (+341.4%) | [+6009633, +6299600]ns | [7831925, 8148358] | YES | 0.0094 | 0.0078 | 0 |
| hand_par | 1825179ns | no significant difference | [-10667, +10575]ns | [1822042, 1859167] | no | 0.2891 | 0.2891 | 0 |
| rust_nostd | 7450517ns | +5589554.2ns (+306.9%) | [+5466100, +5992117]ns | [7330542, 7815592] | YES | 0.0094 | 0.0078 | 0 |
| rust_std | 7474458ns | +5530308.3ns (+303.6%) | [+5509692, +5743233]ns | [7327033, 7565525] | YES | 0.0094 | 0.0078 | 0 |
| rust_std_par | 10543400ns | +8723066.7ns (+478.9%) | [+8555725, +8801342]ns | [10476767, 10637292] | YES | 0.0094 | 0.0078 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hand | engine | engine_par | hand_par | rust_nostd | rust_std | rust_std_par |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 1819383ns | +350.7% | +341.7% | +0.2% | +309.5% | +308.5% | +481.0% |
| 1 | 2 | 1823817ns | +356.5% | +338.0% | +1.1% | +316.3% | +309.9% | +476.8% |

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
- hand_par: won 0/2, lost 2/2
- rust_nostd: won 0/2, lost 2/2
- rust_std: won 0/2, lost 2/2
- rust_std_par: won 0/2, lost 2/2

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| engine | 67510442.7ns | 8380083.3ns | 805.6% | HIGH |
| engine_par | 67573322.9ns | 8040144.8ns | 840.4% | HIGH |
| hand | 67426058.3ns | 1851949.0ns | 3640.8% | HIGH |
| hand_par | 67391349.0ns | 1837736.5ns | 3667.1% | HIGH |
| rust_nostd | 67793666.7ns | 7538200.0ns | 899.3% | HIGH |
| rust_std | 67629437.5ns | 7470296.9ns | 905.3% | HIGH |
| rust_std_par | 67551183.3ns | 10550012.5ns | 640.3% | HIGH |

## Distribution (algo ns)

```
engine (n=8, range 8113150.0-8590585.4 ns)
  8113150.0 |########################################
  8137021.8 |
  8160893.5 |########################################
  8184765.3 |########################################
  8208637.1 |########################################
  8232508.9 |
  8256380.6 |
  8280252.4 |########################################
  8304124.2 |
  8327995.9 |
  8351867.7 |########################################
  8375739.5 |
  8399611.3 |
  8423483.0 |
  8447354.8 |
  8471226.6 |
  8495098.3 |
  8518970.1 |
  8542841.9 |
  8566713.7 |
  (0 below, 2 above range)

engine_par (n=8, range 7774475.0-8209262.5 ns)
  7774475.0 |########################################
  7796214.4 |
  7817953.8 |########################################
  7839693.1 |########################################
  7861432.5 |
  7883171.9 |
  7904911.2 |
  7926650.6 |
  7948390.0 |
  7970129.4 |
  7991868.8 |
  8013608.1 |########################################
  8035347.5 |########################################
  8057086.9 |
  8078826.2 |
  8100565.6 |
  8122305.0 |########################################
  8144044.4 |########################################
  8165783.8 |
  8187523.1 |
  (0 below, 1 above range)

hand (n=8, range 1817341.7-1885025.0 ns)
  1817341.7 |########################################
  1820725.9 |##########
  1824110.0 |##########
  1827494.2 |
  1830878.4 |
  1834262.5 |
  1837646.7 |
  1841030.9 |
  1844415.0 |
  1847799.2 |
  1851183.4 |
  1854567.5 |
  1857951.7 |
  1861335.8 |
  1864720.0 |
  1868104.2 |
  1871488.3 |
  1874872.5 |
  1878256.7 |
  1881640.8 |
  (0 below, 2 above range)

hand_par (n=8, range 1821216.7-1853260.4 ns)
  1821216.7 |########################################
  1822818.9 |#############
  1824421.1 |
  1826023.3 |#############
  1827625.4 |
  1829227.6 |
  1830829.8 |#############
  1832432.0 |
  1834034.2 |
  1835636.4 |
  1837238.6 |
  1838840.8 |
  1840442.9 |
  1842045.1 |
  1843647.3 |
  1845249.5 |
  1846851.7 |
  1848453.9 |
  1850056.1 |
  1851658.3 |
  (0 below, 2 above range)

rust_nostd (n=8, range 7292166.7-7715779.2 ns)
  7292166.7 |####################
  7313347.3 |####################
  7334527.9 |
  7355708.6 |####################
  7376889.2 |
  7398069.8 |
  7419250.4 |
  7440431.1 |########################################
  7461611.7 |
  7482792.3 |
  7503972.9 |
  7525153.5 |
  7546334.2 |
  7567514.8 |
  7588695.4 |
  7609876.0 |
  7631056.7 |
  7652237.3 |
  7673417.9 |
  7694598.5 |
  (0 below, 3 above range)

rust_std (n=8, range 7309000.0-7593181.2 ns)
  7309000.0 |####################
  7323209.1 |########################################
  7337418.1 |
  7351627.2 |
  7365836.2 |
  7380045.3 |
  7394254.4 |
  7408463.4 |####################
  7422672.5 |
  7436881.6 |
  7451090.6 |
  7465299.7 |
  7479508.8 |
  7493717.8 |
  7507926.9 |
  7522135.9 |########################################
  7536345.0 |
  7550554.1 |
  7564763.1 |####################
  7578972.2 |
  (0 below, 1 above range)

rust_std_par (n=8, range 10461883.3-10605289.6 ns)
  10461883.3 |####################
  10469053.6 |
  10476223.9 |####################
  10483394.2 |
  10490564.6 |####################
  10497734.9 |
  10504905.2 |
  10512075.5 |
  10519245.8 |
  10526416.1 |
  10533586.4 |
  10540756.8 |########################################
  10547927.1 |
  10555097.4 |
  10562267.7 |
  10569438.0 |
  10576608.3 |
  10583778.7 |
  10590949.0 |
  10598119.3 |####################
  (0 below, 2 above range)

```

## Diagnostics

- engine: bridge=817.3% of algo (FFI overhead may distort results)
- engine_par: bridge=839.2% of algo (FFI overhead may distort results)
- hand: bridge=3698.4% of algo (FFI overhead may distort results)
- hand_par: bridge=3688.6% of algo (FFI overhead may distort results)
- rust_nostd: bridge=908.6% of algo (FFI overhead may distort results)
- rust_std: bridge=902.4% of algo (FFI overhead may distort results)
- rust_std_par: bridge=640.3% of algo (FFI overhead may distort results)
