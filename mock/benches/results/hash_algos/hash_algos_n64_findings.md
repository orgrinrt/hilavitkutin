# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (39 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh dominates: 110% faster than the next best (hash_sip)

hash_xxh (4 ns) leads hash_sip (9 ns) by 110%, a clear separation rather than a photo finish. CV 30.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 89% (significant)

hash_xxh is -35 ns (89%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 9.2x slower than the field

hash_fnv1a (39 ns) is 9.2x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 30.2%)

hash_xxh wins on median (4 ns) yet has the highest variance (CV 30.2%), while hash_fnv1a is the steadiest (CV 15.3%, 39 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hash_fnv1a shows alternating (throttle bounce) (autocorr -0.54)

hash_fnv1a's per-pass series has lag-1 autocorrelation -0.54, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 9.2x the fastest

Fastest hash_xxh (4 ns) to slowest hash_fnv1a (39 ns): 9.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 2.5x its best-20%

hash_xxh's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 4.2 ns median (-89.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 9.24x (fastest 4.2 ns, slowest 38.8 ns)

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
| hash_fnv1a | 2345ns | 2437ns | 2118ns | 2382ns | 2462ns | base |
| hash_sip | 2253ns | 2221ns | 2084ns | 2250ns | 2431ns | -3.92% |
| hash_xxh | 2252ns | 2243ns | 2081ns | 2253ns | 2422ns | -3.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 39ns | 32ns | 47ns | base | 1.638 |
| hash_sip | 9ns | 6ns | 13ns | -75.87% | 6.788 |
| hash_xxh | 4ns | 2ns | 6ns | -89.32% | 15.339 |

## Performance model

- Peak throughput: **26.701 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.649 | 6.2% |
| hash_sip | 7.273 | 27.2% |
| hash_xxh | 15.238 | 57.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 2345ns | 2345ns | base |
| hash_sip | 2253ns | 2253ns | -3.92% |
| hash_xxh | 2252ns | 2252ns | -3.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 39ns | base | --- | [38, 40] | --- | --- | --- | --- |
| hash_sip | 9ns | -29.4ns (-75.6%) | [-30, -28]ns | [9, 10] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 4ns | -34.6ns (-89.2%) | [-36, -34]ns | [4, 5] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 36ns | -71.5% | -89.8% |
| 1 | 2 | 41ns | -75.5% | -88.7% |
| 1 | 3 | 35ns | -76.3% | -87.4% |
| 1 | 4 | 40ns | -81.2% | -89.4% |
| 1 | 5 | 38ns | -77.7% | -89.2% |
| 1 | 6 | 40ns | -72.7% | -88.9% |
| 1 | 7 | 40ns | -70.0% | -91.3% |
| 1 | 8 | 39ns | -80.8% | -90.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.539 | HIGH- (thermal bounce) |
| hash_sip | 0.056 | ok |
| hash_xxh | 0.064 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 3.1ns | 39.1ns | 7.8% | HIGH |
| hash_sip | 2.8ns | 9.4ns | 29.6% | HIGH |
| hash_xxh | 12.9ns | 4.2ns | 309.7% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 32.0-46.9 ns)
     32.0 |##################
     32.7 |#####
     33.5 |##########
     34.2 |#####
     35.0 |################
     35.7 |##################################
     36.5 |########################################
     37.2 |##########################
     38.0 |#############
     38.7 |################################
     39.5 |##########################
     40.2 |#####################
     41.0 |#############
     41.7 |################
     42.4 |#####################
     43.2 |########################
     43.9 |##########
     44.7 |##################
     45.4 |#####
     46.2 |########
  (14 below, 8 above range)

hash_sip (n=160, range 6.4-13.2 ns)
      6.4 |##################################
      6.7 |
      7.1 |#####################
      7.4 |#####################################
      7.7 |##################################
      8.1 |##################
      8.4 |
      8.7 |########################################
      9.1 |##########
      9.4 |########################
      9.8 |################
     10.1 |#############################
     10.4 |
     10.8 |#####################
     11.1 |##################
     11.5 |########
     11.8 |##########################
     12.1 |
     12.5 |##########
     12.8 |##########
  (11 below, 13 above range)

hash_xxh (n=160, range 2.4-5.9 ns)
      2.4 |###############
      2.6 |
      2.7 |######################
      2.9 |
      3.1 |
      3.3 |##########################
      3.5 |
      3.6 |########################################
      3.8 |
      4.0 |
      4.2 |###########################
      4.3 |
      4.5 |######################################
      4.7 |
      4.9 |#############################
      5.0 |
      5.2 |
      5.4 |##########################
      5.6 |
      5.7 |#############
  (11 below, 11 above range)

```

## Diagnostics

- **hash_sip**: CV=26.2% (high variance, measurements may be unstable)
- **hash_sip**: bridge=30.7% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=30.4% (high variance, measurements may be unstable)
- **hash_xxh**: bridge=307.1% of algo (FFI overhead may distort results)
