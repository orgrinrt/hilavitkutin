# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (3.75 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 725 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 84% (significant)

hash_xxh is -3.14 us (84%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.2x slower than the field

hash_fnv1a (3.75 us) is 5.2x the fastest (725 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_xxh is fastest but the noisiest (CV 59.6%)

hash_xxh wins on median (725 ns) yet has the highest variance (CV 59.6%), while hash_fnv1a is the steadiest (CV 10.7%, 3.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 5.2x the fastest

Fastest hash_xxh (725 ns) to slowest hash_fnv1a (3.75 us): 5.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_xxh is inconsistent: worst-20% is 2.3x its best-20%

hash_xxh's best 20% of batches run at 593 ns but its worst 20% at 1.37 us (2.3x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 725.0 ns median (-80.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 5.17x (fastest 725.0 ns, slowest 3746.6 ns)

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
| hash_fnv1a | 5736ns | 5292ns | 5278ns | 5643ns | 6474ns | base |
| hash_sip | 3143ns | 2672ns | 2418ns | 2670ns | 5284ns | -45.22% |
| hash_xxh | 2872ns | 2606ns | 2132ns | 2423ns | 4961ns | -49.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 4057ns | 3737ns | 4577ns | base | 1.010 |
| hash_sip | 918ns | 705ns | 1541ns | -77.37% | 4.462 |
| hash_xxh | 796ns | 593ns | 1368ns | -80.38% | 5.147 |

## Performance model

- Peak throughput: **6.905 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.093 | 15.8% |
| hash_sip | 5.213 | 75.5% |
| hash_xxh | 5.650 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 5736ns | 5736ns | base |
| hash_sip | 3143ns | 3143ns | -45.22% |
| hash_xxh | 2872ns | 2872ns | -49.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 3747ns | base | --- | [3746, 3748] | --- | --- | --- | --- |
| hash_sip | 786ns | -2964.0ns (-79.1%) | [-2973, -2963]ns | [774, 788] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 725ns | -3143.7ns (-83.9%) | [-3153, -3134]ns | [603, 725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 4567ns | -82.7% | -87.0% |
| 1 | 2 | 3741ns | -79.3% | -84.1% |
| 1 | 3 | 3742ns | -78.9% | -84.1% |
| 1 | 4 | 3737ns | -78.9% | -80.6% |
| 1 | 5 | 4568ns | -83.0% | -87.0% |
| 1 | 6 | 4567ns | -86.1% | -84.1% |
| 1 | 7 | 3737ns | -79.3% | -80.6% |
| 1 | 8 | 3737ns | -79.3% | -80.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | 0.057 | ok |
| hash_sip | -0.068 | ok |
| hash_xxh | 0.125 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 21.3ns | 4056.9ns | 0.5% |  |
| hash_sip | 28.2ns | 918.1ns | 3.1% |  |
| hash_xxh | 26.1ns | 795.8ns | 3.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 3736.8-4576.6 ns)
   3736.8 |########################################
   3778.8 |
   3820.8 |
   3862.8 |
   3904.8 |
   3946.8 |
   3988.8 |
   4030.8 |
   4072.8 |
   4114.7 |
   4156.7 |
   4198.7 |
   4240.7 |
   4282.7 |
   4324.7 |
   4366.7 |
   4408.7 |
   4450.7 |
   4492.6 |
   4534.6 |#################
  (16 below, 25 above range)

hash_sip (n=160, range 704.7-1540.5 ns)
    704.7 |
    746.5 |########################################
    788.3 |#####################
    830.1 |
    871.9 |
    913.7 |
    955.4 |
    997.2 |
   1039.0 |
   1080.8 |
   1122.6 |
   1164.4 |
   1206.2 |
   1248.0 |
   1289.8 |
   1331.6 |
   1373.4 |
   1415.1 |
   1456.9 |
   1498.7 |
  (16 below, 15 above range)

hash_xxh (n=160, range 593.2-1368.1 ns)
    593.2 |##################################
    631.9 |
    670.7 |
    709.4 |########################################
    748.2 |
    786.9 |
    825.7 |
    864.4 |
    903.1 |
    941.9 |
    980.6 |
   1019.4 |
   1058.1 |
   1096.9 |
   1135.6 |
   1174.4 |
   1213.1 |
   1251.9 |
   1290.6 |
   1329.4 |
  (8 below, 13 above range)

```

## Diagnostics

- **hash_sip**: CV=50.2% (high variance, measurements may be unstable)
- **hash_xxh**: CV=54.3% (high variance, measurements may be unstable)
