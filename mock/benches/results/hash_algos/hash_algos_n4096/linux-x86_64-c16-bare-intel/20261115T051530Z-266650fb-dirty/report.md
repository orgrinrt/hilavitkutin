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

### hash_xxh is fastest but the noisiest (CV 70.4%)

hash_xxh wins on median (725 ns) yet has the highest variance (CV 70.4%), while hash_fnv1a is the steadiest (CV 6.0%, 4.57 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Wide spread: slowest is 6.3x the fastest

Fastest hash_xxh (725 ns) to slowest hash_fnv1a (4.57 us): 6.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_sip is inconsistent: worst-20% is 2.6x its best-20%

hash_sip's best 20% of batches run at 638 ns but its worst 20% at 1.68 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 725.1 ns median (-84.1% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 6.30x (fastest 725.1 ns, slowest 4567.4 ns)

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
| hash_fnv1a | 6316ns | 6457ns | 5726ns | 6456ns | 6485ns | base |
| hash_sip | 3171ns | 2661ns | 2175ns | 2655ns | 5713ns | -49.79% |
| hash_xxh | 3268ns | 2606ns | 2351ns | 2608ns | 6166ns | -48.25% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 4470ns | 4055ns | 4580ns | base | 0.916 |
| hash_sip | 933ns | 638ns | 1684ns | -79.13% | 4.392 |
| hash_xxh | 908ns | 655ns | 1709ns | -79.68% | 4.511 |

## Performance model

- Peak throughput: **6.418 Gops/s** (hash_sip; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 0.897 | 14.0% |
| hash_sip | 5.241 | 81.7% |
| hash_xxh | 5.649 | 88.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 6316ns | 6316ns | base |
| hash_sip | 3171ns | 3171ns | -49.79% |
| hash_xxh | 3268ns | 3268ns | -48.25% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 4567ns | base | --- | [4567, 4568] | --- | --- | --- | --- |
| hash_sip | 782ns | -3791.1ns (-83.0%) | [-3793, -3786]ns | [776, 785] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 725ns | -3842.3ns (-84.1%) | [-3842, -3842]ns | [725, 725] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 3737ns | -78.8% | -80.6% |
| 1 | 2 | 4567ns | -82.8% | -84.1% |
| 1 | 3 | 4567ns | -82.9% | -84.1% |
| 1 | 4 | 4567ns | -82.8% | -84.1% |
| 1 | 5 | 4573ns | -86.0% | -84.1% |
| 1 | 6 | 4572ns | -83.0% | -87.0% |
| 1 | 7 | 4568ns | -86.1% | -84.1% |
| 1 | 8 | 4578ns | -83.1% | -84.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.015 | ok |
| hash_sip | -0.297 | moderate- |
| hash_xxh | -0.161 | ok |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 23.3ns | 4469.6ns | 0.5% |  |
| hash_sip | 28.3ns | 932.6ns | 3.0% |  |
| hash_xxh | 29.6ns | 908.0ns | 3.3% |  |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 4054.8-4580.0 ns)
   4054.8 |
   4081.1 |
   4107.3 |
   4133.6 |
   4159.9 |
   4186.1 |
   4212.4 |
   4238.6 |
   4264.9 |
   4291.1 |
   4317.4 |
   4343.7 |
   4369.9 |
   4396.2 |
   4422.4 |
   4448.7 |
   4475.0 |
   4501.2 |
   4527.5 |
   4553.7 |########################################
  (20 below, 10 above range)

hash_sip (n=160, range 638.2-1683.9 ns)
    638.2 |#######
    690.5 |
    742.7 |########################################
    795.0 |##
    847.3 |
    899.6 |
    951.9 |
   1004.2 |
   1056.5 |
   1108.8 |
   1161.1 |
   1213.3 |
   1265.6 |
   1317.9 |
   1370.2 |
   1422.5 |
   1474.8 |
   1527.1 |
   1579.4 |
   1631.6 |
  (16 below, 18 above range)

hash_xxh (n=160, range 655.2-1708.9 ns)
    655.2 |
    707.9 |########################################
    760.6 |
    813.3 |
    866.0 |
    918.7 |
    971.3 |
   1024.0 |
   1076.7 |
   1129.4 |
   1182.1 |
   1234.8 |
   1287.4 |
   1340.1 |
   1392.8 |
   1445.5 |
   1498.2 |
   1550.9 |
   1603.5 |
   1656.2 |
  (17 below, 19 above range)

```

## Diagnostics

- **hash_sip**: CV=52.7% (high variance, measurements may be unstable)
- **hash_xxh**: CV=56.2% (high variance, measurements may be unstable)
