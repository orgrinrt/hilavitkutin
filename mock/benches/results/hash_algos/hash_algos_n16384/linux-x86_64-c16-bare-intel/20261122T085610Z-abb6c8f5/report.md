# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (14.95 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_sip at 2.52 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_sip dominates: 14% faster than the next best (hash_xxh)

hash_sip (2.52 us) leads hash_xxh (2.86 us) by 14%, a clear separation rather than a photo finish. CV 10.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### hash_xxh beats baseline by 84% (significant)

hash_xxh is -12.59 us (84%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.9x slower than the field

hash_fnv1a (14.95 us) is 5.9x the fastest (2.52 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_sip is fastest but the noisiest (CV 10.8%)

hash_sip wins on median (2.52 us) yet has the highest variance (CV 10.8%), while hash_xxh is the steadiest (CV 7.9%, 2.86 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### hash_sip shows warm-up / thermal drift (autocorr +0.59)

hash_sip's per-pass series has lag-1 autocorrelation +0.59, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.9x the fastest

Fastest hash_sip (2.52 us) to slowest hash_fnv1a (14.95 us): 5.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: hash_sip** at 2516.8 ns median (-83.2% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.94x (fastest 2516.8 ns, slowest 14950.5 ns)

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
| Counter frequency | 2112774913 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 17898ns | 16500ns | 16478ns | 17611ns | 20180ns | base |
| hash_sip | 4395ns | 4060ns | 4045ns | 4320ns | 4971ns | -75.44% |
| hash_xxh | 4538ns | 4742ns | 3882ns | 4676ns | 4779ns | -74.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 16220ns | 14936ns | 18286ns | base | 1.010 |
| hash_sip | 2721ns | 2506ns | 3076ns | -83.23% | 6.022 |
| hash_xxh | 2734ns | 2340ns | 2875ns | -83.14% | 5.993 |

## Performance model

- Peak throughput: **7.002 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.096 | 15.7% |
| hash_sip | 6.510 | 93.0% |
| hash_xxh | 5.730 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 17898ns | 17898ns | base |
| hash_sip | 4395ns | 4395ns | -75.44% |
| hash_xxh | 4538ns | 4538ns | -74.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 14950ns | base | --- | [14949, 15117] | --- | --- | --- | --- |
| hash_sip | 2517ns | -12441.2ns (-83.2%) | [-12454, -12439]ns | [2514, 2520] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 2859ns | -12589.0ns (-84.2%) | [-12608, -12102]ns | [2859, 2859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 18262ns | -83.2% | -87.2% |
| 1 | 2 | 18267ns | -83.2% | -84.3% |
| 1 | 3 | 14951ns | -79.5% | -80.9% |
| 1 | 4 | 14946ns | -83.2% | -84.3% |
| 1 | 5 | 14947ns | -83.2% | -80.9% |
| 1 | 6 | 14946ns | -83.2% | -80.9% |
| 1 | 7 | 18268ns | -86.3% | -84.3% |
| 1 | 8 | 14939ns | -83.2% | -80.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.057 | ok |
| hash_sip | 0.589 | HIGH+ (drift/warm-up) |
| hash_xxh | -0.209 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 20.9ns | 16219.8ns | 0.1% |  |
| hash_sip | 20.7ns | 2720.8ns | 0.8% |  |
| hash_xxh | 22.6ns | 2733.9ns | 0.8% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 14936.4-18285.7 ns)
  14936.4 |########################################
  15103.9 |#
  15271.3 |
  15438.8 |
  15606.3 |
  15773.7 |
  15941.2 |
  16108.7 |
  16276.1 |
  16443.6 |
  16611.1 |
  16778.5 |
  16946.0 |
  17113.5 |
  17280.9 |
  17448.4 |
  17615.9 |
  17783.3 |
  17950.8 |
  18118.3 |##########################
  (5 below, 3 above range)

hash_sip (n=160, range 2506.4-3075.8 ns)
   2506.4 |########################################
   2534.8 |
   2563.3 |
   2591.8 |
   2620.2 |
   2648.7 |
   2677.2 |
   2705.7 |
   2734.1 |
   2762.6 |
   2791.1 |
   2819.5 |
   2848.0 |
   2876.5 |
   2904.9 |
   2933.4 |
   2961.9 |
   2990.4 |
   3018.8 |
   3047.3 |#####################
  (15 below, 15 above range)

hash_xxh (n=160, range 2339.9-2874.8 ns)
   2339.9 |###
   2366.6 |
   2393.4 |
   2420.1 |
   2446.9 |
   2473.6 |
   2500.3 |
   2527.1 |
   2553.8 |
   2580.6 |
   2607.3 |
   2634.1 |
   2660.8 |
   2687.6 |
   2714.3 |
   2741.0 |
   2767.8 |
   2794.5 |
   2821.3 |
   2848.0 |########################################
  (30 below, 4 above range)

```

## Diagnostics

- **hash_sip**: autocorrelation=0.59 (measurement drift or warm-up artifact)
