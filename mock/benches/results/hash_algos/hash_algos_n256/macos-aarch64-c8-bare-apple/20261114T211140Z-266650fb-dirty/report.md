# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (288 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 36 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 70% faster than the next best (hash_sip)

hash_xxh (36 ns) leads hash_sip (61 ns) by 70%, a clear separation rather than a photo finish. CV 14.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 88% (significant)

hash_xxh is -254 ns (88%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 8.1x slower than the field

hash_fnv1a (288 ns) is 8.1x the fastest (36 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 14.5%)

hash_xxh wins on median (36 ns) yet has the highest variance (CV 14.5%), while hash_fnv1a is the steadiest (CV 5.9%, 288 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 8.1x the fastest

Fastest hash_xxh (36 ns) to slowest hash_fnv1a (288 ns): 8.1x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 1.5x its best-20%

hash_xxh's best 20% of batches run at 29 ns but its worst 20% at 43 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 35.8 ns median (-87.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.05x (fastest 35.8 ns, slowest 288.3 ns)

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
| hash_fnv1a | 2566ns | 2533ns | 2359ns | 2581ns | 2728ns | base |
| hash_sip | 2322ns | 2288ns | 2135ns | 2331ns | 2480ns | -9.52% |
| hash_xxh | 2319ns | 2293ns | 2116ns | 2340ns | 2460ns | -9.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 291ns | 266ns | 312ns | base | 0.879 |
| hash_sip | 61ns | 53ns | 68ns | -79.17% | 4.220 |
| hash_xxh | 36ns | 29ns | 43ns | -87.68% | 7.133 |

## Performance model

- Peak throughput: **8.978 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.888 | 9.9% |
| hash_sip | 4.211 | 46.9% |
| hash_xxh | 7.151 | 79.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2566ns | 2566ns | base |
| hash_sip | 2322ns | 2322ns | -9.52% |
| hash_xxh | 2319ns | 2319ns | -9.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 288ns | base | --- | [286, 296] | --- | --- | --- | --- |
| hash_sip | 61ns | -229.6ns (-79.6%) | [-235, -226]ns | [60, 62] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 36ns | -254.3ns (-88.2%) | [-261, -252]ns | [35, 37] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 291ns | -81.0% | -88.2% |
| 1 | 2 | 306ns | -80.4% | -87.9% |
| 1 | 3 | 305ns | -81.2% | -90.4% |
| 1 | 4 | 268ns | -76.5% | -86.3% |
| 1 | 5 | 307ns | -79.4% | -87.2% |
| 1 | 6 | 308ns | -81.1% | -89.0% |
| 1 | 7 | 287ns | -77.8% | -86.7% |
| 1 | 8 | 270ns | -77.4% | -85.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.138 | ok |
| hash_sip | -0.107 | ok |
| hash_xxh | -0.229 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.9ns | 291.2ns | 1.0% |  |
| hash_sip | 3.9ns | 60.7ns | 6.5% | HIGH |
| hash_xxh | 3.3ns | 35.9ns | 9.2% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 266.4-312.1 ns)
    266.4 |########################
    268.7 |######################
    271.0 |####
    273.2 |####
    275.5 |##
    277.8 |##
    280.1 |##
    282.4 |########################
    284.7 |########################################
    287.0 |############################
    289.2 |###########
    291.5 |######
    293.8 |##
    296.1 |
    298.4 |####
    300.7 |##
    303.0 |###############
    305.2 |###################################
    307.5 |#####################################
    309.8 |###############################
  (14 below, 10 above range)

hash_sip (n=160, range 53.5-68.3 ns)
     53.5 |###
     54.2 |#########
     54.9 |##############################
     55.7 |#################################
     56.4 |##################
     57.2 |###############
     57.9 |##################
     58.7 |##################
     59.4 |####################################
     60.1 |########################################
     60.9 |######
     61.6 |###########################
     62.4 |########################################
     63.1 |#####################
     63.9 |########################
     64.6 |######
     65.4 |###############
     66.1 |#####################
     66.8 |############
     67.6 |######
  (15 below, 13 above range)

hash_xxh (n=160, range 28.5-43.1 ns)
     28.5 |##############
     29.2 |
     30.0 |##############
     30.7 |##############
     31.4 |################
     32.2 |##############
     32.9 |###########
     33.6 |##################
     34.4 |########################################
     35.1 |#########################
     35.8 |#######
     36.6 |#######################
     37.3 |##############
     38.0 |##############
     38.7 |##############
     39.5 |##################
     40.2 |################
     40.9 |#########
     41.7 |##############
     42.4 |##############
  (14 below, 12 above range)

```
