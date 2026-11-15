# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (18.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 2.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 84% (significant)

hash_xxh is -15.41 us (84%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 6.4x slower than the field

hash_fnv1a (18.27 us) is 6.4x the fastest (2.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 6.4x the fastest

Fastest hash_xxh (2.86 us) to slowest hash_fnv1a (18.27 us): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader hash_xxh vs stability leader hash_sip (+7% speed for 1.0x steadier)

hash_xxh is fastest (2.86 us, CV 6.1%); hash_sip gives up 7.4% median for 1.0x lower variance (CV 6.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: hash_xxh** at 2859.1 ns median (-84.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.39x (fastest 2859.1 ns, slowest 18267.8 ns)

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
| Counter frequency | 2112808777 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 19714ns | 20146ns | 17855ns | 20147ns | 20277ns | base |
| hash_sip | 4845ns | 4950ns | 4391ns | 4950ns | 4986ns | -75.42% |
| hash_xxh | 4645ns | 4742ns | 4203ns | 4743ns | 4794ns | -76.44% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 17872ns | 16186ns | 18371ns | base | 0.917 |
| hash_sip | 3004ns | 2724ns | 3088ns | -83.19% | 5.453 |
| hash_xxh | 2800ns | 2535ns | 2882ns | -84.33% | 5.852 |

## Performance model

- Peak throughput: **6.463 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.897 | 13.9% |
| hash_sip | 5.338 | 82.6% |
| hash_xxh | 5.730 | 88.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 19714ns | 19714ns | base |
| hash_sip | 4845ns | 4845ns | -75.42% |
| hash_xxh | 4645ns | 4645ns | -76.44% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 18268ns | base | --- | [18267, 18268] | --- | --- | --- | --- |
| hash_sip | 3070ns | -15195.5ns (-83.2%) | [-15200, -15193]ns | [3067, 3071] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 2859ns | -15408.0ns (-84.3%) | [-15409, -15407]ns | [2859, 2859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 18267ns | -83.2% | -84.3% |
| 1 | 2 | 18268ns | -83.2% | -87.2% |
| 1 | 3 | 18267ns | -86.2% | -84.3% |
| 1 | 4 | 18269ns | -83.2% | -84.3% |
| 1 | 5 | 18272ns | -83.2% | -84.4% |
| 1 | 6 | 18268ns | -83.2% | -84.3% |
| 1 | 7 | 18267ns | -83.2% | -84.3% |
| 1 | 8 | 14946ns | -79.4% | -80.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.017 | ok |
| hash_sip | -0.147 | ok |
| hash_xxh | -0.161 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 23.1ns | 17871.6ns | 0.1% |  |
| hash_sip | 22.8ns | 3004.4ns | 0.8% |  |
| hash_xxh | 23.1ns | 2799.7ns | 0.8% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 16185.7-18371.5 ns)
  16185.7 |
  16295.0 |
  16404.3 |
  16513.6 |
  16622.8 |
  16732.1 |
  16841.4 |
  16950.7 |
  17060.0 |
  17169.3 |
  17278.6 |
  17387.9 |
  17497.2 |
  17606.5 |
  17715.7 |
  17825.0 |
  17934.3 |
  18043.6 |
  18152.9 |########
  18262.2 |########################################
  (20 below, 2 above range)

hash_sip (n=160, range 2724.4-3087.5 ns)
   2724.4 |
   2742.5 |
   2760.7 |
   2778.9 |
   2797.0 |
   2815.2 |
   2833.3 |
   2851.5 |
   2869.6 |
   2887.8 |
   2906.0 |
   2924.1 |
   2942.3 |
   2960.4 |
   2978.6 |
   2996.7 |
   3014.9 |
   3033.1 |
   3051.2 |##################################
   3069.4 |########################################
  (20 below, 12 above range)

hash_xxh (n=160, range 2535.1-2882.3 ns)
   2535.1 |
   2552.5 |
   2569.8 |
   2587.2 |
   2604.6 |
   2621.9 |
   2639.3 |
   2656.6 |
   2674.0 |
   2691.4 |
   2708.7 |
   2726.1 |
   2743.4 |
   2760.8 |
   2778.2 |
   2795.5 |
   2812.9 |
   2830.2 |
   2847.6 |########################################
   2865.0 |################
  (20 below, 2 above range)

```
