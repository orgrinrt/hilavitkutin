# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (1.31 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 235 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 22% faster than the next best (hash_sip)

hash_xxh (235 ns) leads hash_sip (287 ns) by 22%, a clear separation rather than a photo finish. CV 6.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 82% (significant)

hash_xxh is -1.07 us (82%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.6x slower than the field

hash_fnv1a (1.31 us) is 5.6x the fastest (235 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 6.5%)

hash_xxh wins on median (235 ns) yet has the highest variance (CV 6.5%), while hash_fnv1a is the steadiest (CV 5.1%, 1.31 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.6x the fastest

Fastest hash_xxh (235 ns) to slowest hash_fnv1a (1.31 us): 5.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 235.4 ns median (-82.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.58x (fastest 235.4 ns, slowest 1314.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1111222233334444 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 3558ns | 3558ns | 3324ns | 3542ns | 3842ns | base |
| hash_sip | 2527ns | 2511ns | 2363ns | 2519ns | 2713ns | -28.99% |
| hash_xxh | 2438ns | 2412ns | 2311ns | 2410ns | 2648ns | -31.49% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1314ns | 1227ns | 1421ns | base | 0.779 |
| hash_sip | 290ns | 271ns | 313ns | -77.95% | 3.533 |
| hash_xxh | 241ns | 226ns | 265ns | -81.65% | 4.246 |

## Performance model

- Peak throughput: **4.527 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.779 | 17.2% |
| hash_sip | 3.564 | 78.7% |
| hash_xxh | 4.350 | 96.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 3558ns | 3558ns | base |
| hash_sip | 2527ns | 2527ns | -28.99% |
| hash_xxh | 2438ns | 2438ns | -31.49% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 1314ns | base | --- | [1313, 1316] | --- | --- | --- | --- |
| hash_sip | 287ns | -1026.2ns (-78.1%) | [-1028, -1024]ns | [286, 289] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 235ns | -1073.5ns (-81.7%) | [-1083, -1069]ns | [233, 243] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 1270ns | -78.4% | -82.0% |
| 1 | 2 | 1317ns | -78.7% | -82.5% |
| 1 | 3 | 1250ns | -77.8% | -80.2% |
| 1 | 4 | 1320ns | -77.9% | -80.3% |
| 1 | 5 | 1231ns | -76.3% | -81.4% |
| 1 | 6 | 1319ns | -78.0% | -81.3% |
| 1 | 7 | 1414ns | -77.9% | -83.7% |
| 1 | 8 | 1310ns | -78.0% | -80.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.093 | ok |
| hash_sip | 0.231 | moderate+ |
| hash_xxh | -0.285 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.9ns | 1314.3ns | 0.2% |  |
| hash_sip | 3.0ns | 289.8ns | 1.0% |  |
| hash_xxh | 2.9ns | 241.2ns | 1.2% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 1226.6-1421.2 ns)
   1226.6 |################
   1236.3 |##
   1246.0 |###
   1255.8 |
   1265.5 |####
   1275.2 |
   1285.0 |##
   1294.7 |
   1304.4 |#########################
   1314.2 |########################################
   1323.9 |#
   1333.6 |
   1343.4 |
   1353.1 |
   1362.8 |
   1372.6 |
   1382.3 |
   1392.0 |
   1401.7 |##
   1411.5 |###########################
  (18 below, 2 above range)

hash_sip (n=160, range 271.3-312.9 ns)
    271.3 |##########
    273.4 |##############
    275.4 |########
    277.5 |##############
    279.6 |################
    281.7 |#########################
    283.8 |################
    285.8 |########################################
    287.9 |###########################
    290.0 |################
    292.1 |################
    294.2 |####
    296.2 |####
    298.3 |
    300.4 |
    302.5 |##
    304.6 |##
    306.7 |####
    308.7 |#######################
    310.8 |###################################
  (17 below, 8 above range)

hash_xxh (n=160, range 226.2-264.9 ns)
    226.2 |##########################
    228.1 |########################################
    230.1 |##########################
    232.0 |######################
    233.9 |##################
    235.9 |####
    237.8 |######
    239.7 |####
    241.7 |##########
    243.6 |######################
    245.5 |##########################
    247.5 |##############
    249.4 |########
    251.3 |######
    253.3 |####
    255.2 |##
    257.1 |####
    259.1 |
    261.0 |############
    262.9 |######
  (15 below, 15 above range)

```
