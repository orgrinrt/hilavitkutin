# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (18.26 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 2.86 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 84% (significant)

hash_xxh is -15.40 us (84%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 6.4x slower than the field

hash_fnv1a (18.26 us) is 6.4x the fastest (2.86 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 9.3%)

hash_xxh wins on median (2.86 us) yet has the highest variance (CV 9.3%), while hash_sip is the steadiest (CV 7.9%, 3.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.4x the fastest

Fastest hash_xxh (2.86 us) to slowest hash_fnv1a (18.26 us): 6.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader hash_xxh vs stability leader hash_sip (+8% speed for 1.2x steadier)

hash_xxh is fastest (2.86 us, CV 9.3%); hash_sip gives up 8.0% median for 1.2x lower variance (CV 7.9%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: hash_xxh** at 2858.6 ns median (-84.3% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.39x (fastest 2858.6 ns, slowest 18257.6 ns)

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
| Counter frequency | 2112768575 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| hash_fnv1a | 18792ns | 20151ns | 16482ns | 19087ns | 20216ns | base |
| hash_sip | 4754ns | 4974ns | 4063ns | 4899ns | 5010ns | -74.70% |
| hash_xxh | 4334ns | 4738ns | 3880ns | 4329ns | 4800ns | -76.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 17028ns | 14938ns | 18309ns | base | 0.962 |
| hash_sip | 2949ns | 2521ns | 3106ns | -82.68% | 5.555 |
| hash_xxh | 2609ns | 2339ns | 2885ns | -84.68% | 6.280 |

## Performance model

- Peak throughput: **7.004 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.897 | 12.8% |
| hash_sip | 5.305 | 75.7% |
| hash_xxh | 5.731 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 18792ns | 18792ns | base |
| hash_sip | 4754ns | 4754ns | -74.70% |
| hash_xxh | 4334ns | 4334ns | -76.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 18258ns | base | --- | [18257, 18268] | --- | --- | --- | --- |
| hash_sip | 3088ns | -15165.1ns (-83.1%) | [-15170, -15160]ns | [3078, 3092] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 2859ns | -15398.9ns (-84.3%) | [-15410, -15397]ns | [2349, 2859] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 14946ns | -83.2% | -80.9% |
| 1 | 2 | 18268ns | -83.0% | -84.3% |
| 1 | 3 | 18269ns | -83.1% | -84.3% |
| 1 | 4 | 18268ns | -86.1% | -87.2% |
| 1 | 5 | 18268ns | -83.2% | -87.2% |
| 1 | 6 | 14938ns | -79.3% | -84.3% |
| 1 | 7 | 18268ns | -83.1% | -87.2% |
| 1 | 8 | 14947ns | -79.3% | -80.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.275 | moderate- |
| hash_sip | -0.197 | ok |
| hash_xxh | 0.375 | moderate+ |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 22.0ns | 17027.6ns | 0.1% |  |
| hash_sip | 22.4ns | 2949.2ns | 0.8% |  |
| hash_xxh | 21.6ns | 2608.7ns | 0.8% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 14938.3-18309.5 ns)
  14938.3 |################
  15106.8 |
  15275.4 |
  15443.9 |
  15612.5 |
  15781.1 |
  15949.6 |
  16118.2 |
  16286.7 |
  16455.3 |
  16623.9 |
  16792.4 |
  16961.0 |
  17129.6 |
  17298.1 |
  17466.7 |
  17635.2 |
  17803.8 |
  17972.4 |
  18140.9 |########################################
  (21 below, 6 above range)

hash_sip (n=160, range 2521.2-3105.7 ns)
   2521.2 |#############
   2550.4 |
   2579.6 |
   2608.8 |
   2638.1 |
   2667.3 |
   2696.5 |
   2725.8 |
   2755.0 |
   2784.2 |
   2813.4 |
   2842.7 |
   2871.9 |
   2901.1 |
   2930.4 |
   2959.6 |
   2988.8 |
   3018.0 |
   3047.3 |############
   3076.5 |########################################
  (14 below, 16 above range)

hash_xxh (n=160, range 2339.1-2885.4 ns)
   2339.1 |#################################
   2366.4 |
   2393.7 |
   2421.0 |
   2448.4 |
   2475.7 |
   2503.0 |
   2530.3 |
   2557.6 |
   2584.9 |
   2612.3 |
   2639.6 |
   2666.9 |
   2694.2 |
   2721.5 |
   2748.8 |
   2776.2 |
   2803.5 |
   2830.8 |
   2858.1 |########################################
  (13 below, 3 above range)

```
