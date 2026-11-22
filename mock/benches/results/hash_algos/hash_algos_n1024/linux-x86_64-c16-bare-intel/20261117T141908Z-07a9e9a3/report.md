# Hash mixer: FNV-1a vs xxh-style block vs SipHash-style ARX

3 variants, 160 samples per variant.
Baseline: **hash_fnv1a**

## Highlights

Baseline for all deltas below: **hash_fnv1a**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (hash_fnv1a) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline hash_fnv1a has the worst median (944 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest hash_xxh at 189 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### hash_xxh beats baseline by 83% (significant)

hash_xxh is -784 ns (83%) faster than baseline hash_fnv1a, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### hash_fnv1a is an outlier: 5.0x slower than the field

hash_fnv1a (944 ns) is 5.0x the fastest (189 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### hash_fnv1a shows alternating (throttle bounce) (autocorr -0.67)

hash_fnv1a's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 5.0x the fastest

Fastest hash_xxh (189 ns) to slowest hash_fnv1a (944 ns): 5.0x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### hash_sip is inconsistent: worst-20% is 3.6x its best-20%

hash_sip's best 20% of batches run at 165 ns but its worst 20% at 595 ns (3.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: hash_xxh** at 189.1 ns median (-80.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 4.99x (fastest 189.1 ns, slowest 943.6 ns)

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
| hash_fnv1a | 3032ns | 2493ns | 2480ns | 2692ns | 4606ns | base |
| hash_sip | 2809ns | 2087ns | 1707ns | 2067ns | 6134ns | -7.39% |
| hash_xxh | 2705ns | 2072ns | 1696ns | 1942ns | 6005ns | -10.79% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| hash_fnv1a | 1144ns | 939ns | 1724ns | base | 0.895 |
| hash_sip | 272ns | 165ns | 595ns | -76.21% | 3.764 |
| hash_xxh | 247ns | 155ns | 547ns | -78.38% | 4.140 |

## Performance model

- Peak throughput: **6.598 Gops/s** (hash_xxh; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| hash_fnv1a | 1.085 | 16.4% |
| hash_sip | 5.100 | 77.3% |
| hash_xxh | 5.415 | 82.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| hash_fnv1a | 3032ns | 3032ns | base |
| hash_sip | 2809ns | 2809ns | -7.39% |
| hash_xxh | 2705ns | 2705ns | -10.79% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| hash_fnv1a | 944ns | base | --- | [939, 1147] | --- | --- | --- | --- |
| hash_sip | 201ns | -772.6ns (-81.9%) | [-775, -749]ns | [200, 202] | YES | 0.0000 | 0.0000 | 0 |
| hash_xxh | 189ns | -784.0ns (-83.1%) | [-784, -783]ns | [166, 190] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | hash_fnv1a | hash_sip | hash_xxh |
|---|---|---|---|---|
| 1 | 1 | 939ns | -78.6% | -79.8% |
| 1 | 2 | 1148ns | -85.7% | -86.2% |
| 1 | 3 | 939ns | -82.2% | -79.7% |
| 1 | 4 | 1148ns | -82.5% | -86.4% |
| 1 | 5 | 939ns | -82.2% | -83.4% |
| 1 | 6 | 939ns | -78.4% | -83.4% |
| 1 | 7 | 1148ns | -82.3% | -83.4% |
| 1 | 8 | 939ns | -78.3% | -83.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| hash_fnv1a | -0.675 | HIGH- (thermal bounce) |
| hash_sip | -0.074 | ok |
| hash_xxh | -0.439 | moderate- |

**Consistency summary:**

- **hash_sip**: won 8/8, lost 0/8
- **hash_xxh**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| hash_fnv1a | 23.8ns | 1143.8ns | 2.1% |  |
| hash_sip | 31.9ns | 272.1ns | 11.7% | HIGH |
| hash_xxh | 30.8ns | 247.3ns | 12.5% | HIGH |

## Distribution (algo ns)

```
hash_fnv1a (n=160, range 938.9-1724.4 ns)
    938.9 |########################################
    978.2 |
   1017.5 |
   1056.7 |
   1096.0 |
   1135.3 |#############################
   1174.6 |
   1213.8 |
   1253.1 |
   1292.4 |
   1331.6 |
   1370.9 |
   1410.2 |
   1449.5 |
   1488.7 |
   1528.0 |
   1567.3 |
   1606.5 |
   1645.8 |
   1685.1 |
  (18 below, 10 above range)

hash_sip (n=160, range 164.7-594.6 ns)
    164.7 |########
    186.2 |########################################
    207.7 |
    229.2 |
    250.7 |
    272.2 |
    293.7 |
    315.2 |
    336.7 |
    358.1 |
    379.6 |
    401.1 |
    422.6 |
    444.1 |
    465.6 |
    487.1 |
    508.6 |#
    530.1 |
    551.6 |
    573.1 |
  (25 below, 19 above range)

hash_xxh (n=160, range 155.2-547.0 ns)
    155.2 |#######################################
    174.8 |########################################
    194.4 |
    214.0 |
    233.6 |
    253.2 |
    272.7 |
    292.3 |
    311.9 |
    331.5 |
    351.1 |
    370.7 |
    390.3 |
    409.9 |##
    429.5 |
    449.1 |##
    468.6 |#
    488.2 |###
    507.8 |
    527.4 |#
  (20 below, 15 above range)

```

## Diagnostics

- **hash_fnv1a**: CV=42.7% (high variance, measurements may be unstable)
- **hash_sip**: CV=62.2% (high variance, measurements may be unstable)
- **hash_sip**: worst_20/best_20 = 3.6x (possible bimodal distribution)
- **hash_sip**: bridge=11.9% of algo (FFI overhead may distort results)
- **hash_xxh**: CV=63.2% (high variance, measurements may be unstable)
- **hash_xxh**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **hash_xxh**: bridge=12.5% of algo (FFI overhead may distort results)
