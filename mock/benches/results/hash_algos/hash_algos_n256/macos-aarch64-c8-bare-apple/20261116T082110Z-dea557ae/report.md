# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (272 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 32 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 84% faster than the next best (hash_sip)

hash_xxh (32 ns) leads hash_sip (58 ns) by 84%, a clear separation rather than a photo finish. CV 15.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 88% (significant)

hash_xxh is -241 ns (88%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 8.6x slower than the field

hash_fnv1a (272 ns) is 8.6x the fastest (32 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 15.2%)

hash_xxh wins on median (32 ns) yet has the highest variance (CV 15.2%), while hash_fnv1a is the steadiest (CV 1.3%, 272 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hash_fnv1a shows warm-up / thermal drift (autocorr +0.56)

hash_fnv1a's per-pass series has lag-1 autocorrelation +0.56, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 8.6x the fastest

Fastest hash_xxh (32 ns) to slowest hash_fnv1a (272 ns): 8.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 1.5x its best-20%

hash_xxh's best 20% of batches run at 25 ns but its worst 20% at 39 ns (1.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 31.7 ns median (-88.4% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 8.60x (fastest 31.7 ns, slowest 272.5 ns)

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
| hash_fnv1a | 2405ns | 2401ns | 2393ns | 2401ns | 2429ns | base |
| hash_sip | 2174ns | 2168ns | 2162ns | 2168ns | 2206ns | -9.60% |
| hash_xxh | 2156ns | 2150ns | 2143ns | 2152ns | 2183ns | -10.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 273ns | 269ns | 278ns | base | 0.938 |
| hash_sip | 59ns | 53ns | 64ns | -78.56% | 4.374 |
| hash_xxh | 32ns | 25ns | 39ns | -88.36% | 8.055 |

## Performance model

- Peak throughput: **10.056 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.939 | 9.3% |
| hash_sip | 4.391 | 43.7% |
| hash_xxh | 8.076 | 80.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2405ns | 2405ns | base |
| hash_sip | 2174ns | 2174ns | -9.60% |
| hash_xxh | 2156ns | 2156ns | -10.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 272ns | base | --- | [272, 273] | --- | --- | --- | --- |
| hash_sip | 58ns | -214.2ns (-78.6%) | [-215, -214]ns | [58, 59] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 32ns | -240.9ns (-88.4%) | [-242, -240]ns | [31, 32] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 273ns | -78.9% | -89.2% |
| 1 | 2 | 274ns | -78.3% | -88.8% |
| 1 | 3 | 274ns | -78.6% | -88.8% |
| 1 | 4 | 273ns | -78.8% | -87.6% |
| 1 | 5 | 272ns | -78.5% | -88.2% |
| 1 | 6 | 272ns | -78.7% | -88.8% |
| 1 | 7 | 272ns | -78.6% | -87.4% |
| 1 | 8 | 273ns | -78.3% | -88.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.561 | HIGH+ (drift/warm-up) |
| hash_sip | -0.336 | moderate- |
| hash_xxh | -0.186 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 2.6ns | 272.9ns | 0.9% |  |
| hash_sip | 3.0ns | 58.5ns | 5.0% | HIGH |
| hash_xxh | 2.9ns | 31.8ns | 9.0% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 268.6-277.8 ns)
    268.6 |########
    269.1 |##################
    269.5 |#############
    270.0 |#####################
    270.5 |##################
    270.9 |#############################
    271.4 |########################
    271.8 |#####################
    272.3 |##################################
    272.8 |#####################
    273.2 |##########################
    273.7 |##################
    274.1 |########################################
    274.6 |##########
    275.1 |##########
    275.5 |########
    276.0 |#############
    276.5 |################
    276.9 |##
    277.4 |########
  (11 below, 12 above range)

hash_sip (n=160, range 53.3-64.1 ns)
     53.3 |####
     53.8 |###########
     54.3 |###########
     54.9 |#########
     55.4 |###########
     56.0 |#######
     56.5 |##################
     57.0 |########################################
     57.6 |#######################
     58.1 |#########################
     58.7 |#####################################
     59.2 |################
     59.7 |#####################
     60.3 |###################################
     60.8 |####
     61.4 |####
     61.9 |
     62.4 |################
     63.0 |#######
     63.5 |#########
  (14 below, 11 above range)

hash_xxh (n=160, range 25.5-38.5 ns)
     25.5 |#####
     26.1 |#####################
     26.8 |##
     27.4 |##################
     28.1 |##########################
     28.7 |##########
     29.4 |########################################
     30.0 |
     30.7 |#############################
     31.3 |#############################
     32.0 |##################################
     32.7 |##########################
     33.3 |#####################
     34.0 |##########################
     34.6 |#####
     35.3 |##########
     35.9 |#############
     36.6 |########################
     37.2 |#####
     37.9 |##########
  (15 below, 9 above range)

```

## Diagnostics

- **hash_fnv1a**: autocorrelation=0.56 (measurement drift or warm-up artifact)
