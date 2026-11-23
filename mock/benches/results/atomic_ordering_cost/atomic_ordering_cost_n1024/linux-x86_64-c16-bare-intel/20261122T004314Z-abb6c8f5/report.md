# Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst

3 variants, 160 samples per variant.
Baseline: **atomic_acquire**

## Highlights

Baseline for all deltas below: **atomic_acquire**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (atomic_acquire) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline atomic_acquire has the worst median (1.09 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest atomic_relaxed at 670 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### atomic_relaxed beats baseline by 39% (significant)

atomic_relaxed is -424 ns (39%) faster than baseline atomic_acquire, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### atomic_relaxed is fastest but the noisiest (CV 63.3%)

atomic_relaxed wins on median (670 ns) yet has the highest variance (CV 63.3%), while atomic_acquire is the steadiest (CV 46.3%, 1.09 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Top two (atomic_relaxed, atomic_seqcst) are a dead heat (<1%)

atomic_relaxed (670 ns) and atomic_seqcst (670 ns) differ by 0.00%, inside the noise, even though the wider field spreads 63.4%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### atomic_relaxed is inconsistent: worst-20% is 2.6x its best-20%

atomic_relaxed's best 20% of batches run at 548 ns but its worst 20% at 1.43 us (2.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: atomic_relaxed** at 670.1 ns median (-38.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.63x (fastest 670.1 ns, slowest 1094.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Atomic ordering cost: Relaxed vs Acquire/Release vs SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x0ddc0ffee055b011 |
| Counter frequency | 2112895079 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| atomic_acquire | 3222ns | 2971ns | 2431ns | 2868ns | 5076ns | base |
| atomic_relaxed | 3000ns | 2551ns | 2086ns | 2468ns | 5508ns | -6.90% |
| atomic_seqcst | 3037ns | 2551ns | 2275ns | 2553ns | 5250ns | -5.75% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| atomic_acquire | 1184ns | 896ns | 1857ns | base | 0.865 |
| atomic_relaxed | 785ns | 548ns | 1431ns | -33.71% | 1.304 |
| atomic_seqcst | 795ns | 597ns | 1367ns | -32.84% | 1.288 |

## Performance model

- Peak throughput: **1.868 Gops/s** (atomic_relaxed; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| atomic_acquire | 0.935 | 50.1% |
| atomic_relaxed | 1.528 | 81.8% |
| atomic_seqcst | 1.528 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| atomic_acquire | 3222ns | 3222ns | base |
| atomic_relaxed | 3000ns | 3000ns | -6.90% |
| atomic_seqcst | 3037ns | 3037ns | -5.75% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| atomic_acquire | 1095ns | base | --- | [1095, 1095] | --- | --- | --- | --- |
| atomic_relaxed | 670ns | -424.1ns (-38.7%) | [-430, -423]ns | [670, 670] | YES | 0.0000 | 0.0000 | 0 |
| atomic_seqcst | 670ns | -424.0ns (-38.7%) | [-425, -423]ns | [670, 670] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | atomic_acquire | atomic_relaxed | atomic_seqcst |
|---|---|---|---|---|
| 1 | 1 | 896ns | -25.2% | -25.2% |
| 1 | 2 | 896ns | -25.1% | -25.2% |
| 1 | 3 | 1095ns | -49.8% | -38.6% |
| 1 | 4 | 1095ns | -38.8% | -38.8% |
| 1 | 5 | 1095ns | -49.9% | -38.8% |
| 1 | 6 | 1095ns | -49.9% | -49.9% |
| 1 | 7 | 1095ns | -38.6% | -38.8% |
| 1 | 8 | 896ns | -25.2% | -25.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| atomic_acquire | 0.258 | moderate+ |
| atomic_relaxed | -0.140 | ok |
| atomic_seqcst | -0.152 | ok |

**Consistency summary:**

- **atomic_relaxed**: won 8/8, lost 0/8
- **atomic_seqcst**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| atomic_acquire | 25.9ns | 1184.1ns | 2.2% |  |
| atomic_relaxed | 27.9ns | 785.0ns | 3.6% |  |
| atomic_seqcst | 28.4ns | 795.2ns | 3.6% |  |

## Distribution (algo ns)

```
atomic_acquire (n=160, range 895.7-1857.0 ns)
    895.7 |###############
    943.8 |
    991.8 |
   1039.9 |
   1088.0 |########################################
   1136.0 |
   1184.1 |
   1232.2 |
   1280.2 |
   1328.3 |
   1376.3 |
   1424.4 |
   1472.5 |
   1520.5 |
   1568.6 |
   1616.7 |
   1664.7 |
   1712.8 |
   1760.8 |
   1808.9 |
  (16 below, 15 above range)

atomic_relaxed (n=160, range 548.2-1431.2 ns)
    548.2 |#################
    592.3 |
    636.5 |########################################
    680.6 |###
    724.8 |
    768.9 |
    813.1 |
    857.2 |
    901.4 |
    945.5 |
    989.7 |
   1033.8 |
   1078.0 |
   1122.2 |
   1166.3 |
   1210.5 |
   1254.6 |
   1298.8 |
   1342.9 |
   1387.1 |#
  (14 below, 17 above range)

atomic_seqcst (n=160, range 597.5-1367.5 ns)
    597.5 |
    636.0 |########################################
    674.5 |##
    713.0 |
    751.5 |
    790.0 |
    828.5 |
    867.0 |
    905.5 |
    944.0 |
    982.5 |
   1021.0 |
   1059.5 |
   1098.0 |
   1136.5 |
   1175.0 |
   1213.5 |
   1252.0 |
   1290.5 |
   1329.0 |
  (19 below, 17 above range)

```

## Diagnostics

- **atomic_acquire**: CV=42.8% (high variance, measurements may be unstable)
- **atomic_relaxed**: CV=54.1% (high variance, measurements may be unstable)
- **atomic_seqcst**: CV=52.5% (high variance, measurements may be unstable)
