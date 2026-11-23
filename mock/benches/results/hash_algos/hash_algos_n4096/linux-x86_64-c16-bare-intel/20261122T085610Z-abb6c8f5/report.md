# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (4.57 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 725 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 84% (significant)

hash_xxh is -3.84 us (84%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 6.3x slower than the field

hash_fnv1a (4.57 us) is 6.3x the fastest (725 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 59.5%)

hash_xxh wins on median (725 ns) yet has the highest variance (CV 59.5%), while hash_fnv1a is the steadiest (CV 7.9%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.3x the fastest

Fastest hash_xxh (725 ns) to slowest hash_fnv1a (4.57 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 2.5x its best-20%

hash_xxh's best 20% of batches run at 593 ns but its worst 20% at 1.49 us (2.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 725.1 ns median (-84.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.30x (fastest 725.1 ns, slowest 4567.3 ns)

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
| hash_fnv1a | 6189ns | 6454ns | 5280ns | 6385ns | 6512ns | base |
| hash_sip | 3051ns | 2662ns | 2183ns | 2656ns | 5102ns | -50.71% |
| hash_xxh | 3053ns | 2607ns | 2133ns | 2507ns | 5612ns | -50.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 4381ns | 3740ns | 4615ns | base | 0.935 |
| hash_sip | 896ns | 644ns | 1495ns | -79.55% | 4.572 |
| hash_xxh | 835ns | 593ns | 1491ns | -80.95% | 4.908 |

## Performance model

- Peak throughput: **6.905 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.897 | 13.0% |
| hash_sip | 5.222 | 75.6% |
| hash_xxh | 5.649 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 6189ns | 6189ns | base |
| hash_sip | 3051ns | 3051ns | -50.71% |
| hash_xxh | 3053ns | 3053ns | -50.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 4567ns | base | --- | [4567, 4567] | --- | --- | --- | --- |
| hash_sip | 784ns | -3780.2ns (-82.8%) | [-3790, -3778]ns | [774, 787] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 725ns | -3842.1ns (-84.1%) | [-3842, -3842]ns | [725, 725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 4573ns | -83.1% | -87.0% |
| 1 | 2 | 4573ns | -82.8% | -87.0% |
| 1 | 3 | 4567ns | -82.7% | -84.1% |
| 1 | 4 | 4567ns | -82.8% | -84.1% |
| 1 | 5 | 4568ns | -82.8% | -84.1% |
| 1 | 6 | 3741ns | -82.7% | -80.6% |
| 1 | 7 | 4573ns | -85.9% | -84.1% |
| 1 | 8 | 3737ns | -79.0% | -84.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.211 | moderate- |
| hash_sip | 0.307 | moderate+ |
| hash_xxh | 0.258 | moderate+ |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 22.8ns | 4380.7ns | 0.5% |  |
| hash_sip | 27.3ns | 896.0ns | 3.0% |  |
| hash_xxh | 27.3ns | 834.6ns | 3.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 3739.8-4615.3 ns)
   3739.8 |#########
   3783.6 |
   3827.3 |
   3871.1 |
   3914.9 |
   3958.7 |
   4002.4 |
   4046.2 |
   4090.0 |
   4133.7 |
   4177.5 |
   4221.3 |
   4265.1 |
   4308.8 |
   4352.6 |
   4396.4 |
   4440.2 |
   4483.9 |
   4527.7 |########################################
   4571.5 |#############################
  (22 below, 6 above range)

hash_sip (n=160, range 644.5-1494.6 ns)
    644.5 |########
    687.0 |
    729.5 |
    772.0 |########################################
    814.5 |
    857.0 |
    899.5 |
    942.0 |
    984.5 |
   1027.0 |
   1069.5 |
   1112.1 |
   1154.6 |
   1197.1 |
   1239.6 |
   1282.1 |
   1324.6 |
   1367.1 |
   1409.6 |
   1452.1 |
  (11 below, 15 above range)

hash_xxh (n=160, range 593.2-1490.7 ns)
    593.2 |#################
    638.1 |
    683.0 |########################################
    727.9 |#####
    772.7 |
    817.6 |
    862.5 |
    907.4 |
    952.2 |
    997.1 |
   1042.0 |
   1086.9 |
   1131.7 |
   1176.6 |
   1221.5 |
   1266.4 |
   1311.2 |
   1356.1 |
   1401.0 |
   1445.9 |
  (21 below, 17 above range)

```

## Diagnostics

- **hash_sip**: CV=49.4% (high variance, measurements may be unstable)
- **hash_xxh**: CV=51.7% (high variance, measurements may be unstable)
