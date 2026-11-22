# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (268 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 34 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 66% faster than the next best (hash_sip)

hash_xxh (34 ns) leads hash_sip (56 ns) by 66%, a clear separation rather than a photo finish. CV 12.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 87% (significant)

hash_xxh is -234 ns (87%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 7.9x slower than the field

hash_fnv1a (268 ns) is 7.9x the fastest (34 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 12.3%)

hash_xxh wins on median (34 ns) yet has the highest variance (CV 12.3%), while hash_fnv1a is the steadiest (CV 1.2%, 268 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 7.9x the fastest

Fastest hash_xxh (34 ns) to slowest hash_fnv1a (268 ns): 7.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_xxh** at 33.8 ns median (-87.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 7.91x (fastest 33.8 ns, slowest 267.5 ns)

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
| hash_fnv1a | 2372ns | 2367ns | 2361ns | 2367ns | 2400ns | base |
| hash_sip | 2165ns | 2140ns | 2132ns | 2142ns | 2269ns | -8.72% |
| hash_xxh | 2126ns | 2122ns | 2115ns | 2122ns | 2151ns | -10.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 268ns | 264ns | 273ns | base | 0.956 |
| hash_sip | 56ns | 51ns | 62ns | -78.97% | 4.546 |
| hash_xxh | 34ns | 27ns | 39ns | -87.46% | 7.622 |

## Performance model

- Peak throughput: **9.325 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.957 | 10.3% |
| hash_sip | 4.555 | 48.8% |
| hash_xxh | 7.574 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2372ns | 2372ns | base |
| hash_sip | 2165ns | 2165ns | -8.72% |
| hash_xxh | 2126ns | 2126ns | -10.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 268ns | base | --- | [267, 268] | --- | --- | --- | --- |
| hash_sip | 56ns | -211.3ns (-79.0%) | [-212, -211]ns | [56, 57] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 34ns | -233.8ns (-87.4%) | [-235, -233]ns | [33, 35] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 267ns | -79.2% | -87.3% |
| 1 | 2 | 268ns | -78.9% | -87.7% |
| 1 | 3 | 267ns | -79.5% | -87.6% |
| 1 | 4 | 267ns | -77.4% | -87.4% |
| 1 | 5 | 269ns | -78.9% | -87.6% |
| 1 | 6 | 270ns | -79.3% | -87.7% |
| 1 | 7 | 268ns | -78.8% | -87.1% |
| 1 | 8 | 267ns | -79.3% | -87.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.233 | moderate+ |
| hash_sip | -0.287 | moderate- |
| hash_xxh | 0.198 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 3.0ns | 267.8ns | 1.1% |  |
| hash_sip | 3.6ns | 56.3ns | 6.3% | HIGH |
| hash_xxh | 2.9ns | 33.6ns | 8.7% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 263.8-272.5 ns)
    263.8 |###########
    264.2 |##############
    264.7 |####################
    265.1 |#################
    265.5 |##############
    266.0 |######################
    266.4 |#####################################
    266.9 |#####################################
    267.3 |########################################
    267.7 |####################
    268.2 |####################
    268.6 |#################
    269.0 |####################
    269.5 |######################
    269.9 |##############
    270.4 |#####
    270.8 |############################
    271.2 |
    271.7 |#################
    272.1 |########
  (14 below, 10 above range)

hash_sip (n=160, range 51.2-61.5 ns)
     51.2 |####
     51.7 |#######
     52.3 |#########
     52.8 |
     53.3 |##################
     53.8 |################
     54.3 |#########################
     54.8 |#######################
     55.4 |###################################
     55.9 |#####################
     56.4 |#####################
     56.9 |################
     57.4 |########################################
     57.9 |#########
     58.4 |#######
     59.0 |####
     59.5 |#########
     60.0 |#########################
     60.5 |####
     61.0 |####
  (16 below, 14 above range)

hash_xxh (n=160, range 27.5-39.3 ns)
     27.5 |#
     28.0 |
     28.6 |#############
     29.2 |#####
     29.8 |###########
     30.4 |###
     31.0 |#########
     31.6 |####################
     32.2 |###############
     32.8 |####################################
     33.4 |#################
     33.9 |###########
     34.5 |########################################
     35.1 |#################
     35.7 |###########
     36.3 |#######
     36.9 |
     37.5 |#########
     38.1 |#########
     38.7 |###########
  (14 below, 13 above range)

```
