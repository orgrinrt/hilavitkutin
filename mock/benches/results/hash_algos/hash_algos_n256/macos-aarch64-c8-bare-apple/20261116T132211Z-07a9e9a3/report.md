# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (267 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 34 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 66% faster than the next best (hash_sip)

hash_xxh (34 ns) leads hash_sip (56 ns) by 66%, a clear separation rather than a photo finish. CV 11.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 87% (significant)

hash_xxh is -233 ns (87%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 7.9x slower than the field

hash_fnv1a (267 ns) is 7.9x the fastest (34 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 11.0%)

hash_xxh wins on median (34 ns) yet has the highest variance (CV 11.0%), while hash_sip is the steadiest (CV 5.5%, 56 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 7.9x the fastest

Fastest hash_xxh (34 ns) to slowest hash_fnv1a (267 ns): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 33.8 ns median (-87.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.90x (fastest 33.8 ns, slowest 267.1 ns)

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
| hash_fnv1a | 2390ns | 2366ns | 2361ns | 2367ns | 2487ns | base |
| hash_sip | 2143ns | 2141ns | 2135ns | 2141ns | 2159ns | -10.31% |
| hash_xxh | 2125ns | 2122ns | 2116ns | 2122ns | 2144ns | -11.06% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 270ns | 264ns | 282ns | base | 0.950 |
| hash_sip | 56ns | 52ns | 60ns | -79.23% | 4.574 |
| hash_xxh | 34ns | 28ns | 39ns | -87.51% | 7.607 |

## Performance model

- Peak throughput: **9.053 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.958 | 10.6% |
| hash_sip | 4.571 | 50.5% |
| hash_xxh | 7.574 | 83.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2390ns | 2390ns | base |
| hash_sip | 2143ns | 2143ns | -10.31% |
| hash_xxh | 2125ns | 2125ns | -11.06% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 267ns | base | --- | [267, 268] | --- | --- | --- | --- |
| hash_sip | 56ns | -211.3ns (-79.1%) | [-212, -211]ns | [55, 57] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 34ns | -233.3ns (-87.3%) | [-234, -233]ns | [33, 35] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 267ns | -78.8% | -87.4% |
| 1 | 2 | 268ns | -79.1% | -87.6% |
| 1 | 3 | 268ns | -79.2% | -87.6% |
| 1 | 4 | 267ns | -79.3% | -87.3% |
| 1 | 5 | 266ns | -78.8% | -87.4% |
| 1 | 6 | 267ns | -79.0% | -87.4% |
| 1 | 7 | 266ns | -79.1% | -86.8% |
| 1 | 8 | 267ns | -78.8% | -87.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.220 | moderate- |
| hash_sip | -0.170 | ok |
| hash_xxh | 0.469 | moderate+ |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 3.1ns | 269.5ns | 1.2% |  |
| hash_sip | 3.5ns | 56.0ns | 6.3% | HIGH |
| hash_xxh | 3.0ns | 33.7ns | 8.8% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 264.1-281.8 ns)
    264.1 |#############
    265.0 |########################################
    265.9 |#######################
    266.7 |##############################
    267.6 |#####################
    268.5 |######################
    269.4 |############
    270.3 |#####
    271.2 |#####
    272.1 |########
    272.9 |##
    273.8 |#
    274.7 |
    275.6 |
    276.5 |
    277.4 |
    278.3 |
    279.2 |
    280.0 |
    280.9 |
  (9 below, 2 above range)

hash_sip (n=160, range 51.6-60.2 ns)
     51.6 |##
     52.0 |####
     52.4 |########
     52.9 |########
     53.3 |##############
     53.7 |########
     54.2 |##################
     54.6 |########################################
     55.0 |#####################
     55.4 |##############
     55.9 |##############
     56.3 |#######################
     56.7 |################
     57.2 |##################
     57.6 |##################
     58.0 |############
     58.4 |##########
     58.9 |########
     59.3 |########
     59.7 |######
  (13 below, 14 above range)

hash_xxh (n=160, range 28.3-38.7 ns)
     28.3 |##
     28.8 |####
     29.3 |###########
     29.8 |#######
     30.4 |##################
     30.9 |##############
     31.4 |#########################
     31.9 |#########
     32.4 |#######################
     33.0 |#####################
     33.5 |##################
     34.0 |#####################
     34.5 |########################################
     35.0 |#####################
     35.6 |##################
     36.1 |##############
     36.6 |###########
     37.1 |###########
     37.6 |####
     38.2 |##
  (14 below, 17 above range)

```
