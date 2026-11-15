# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (308 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_atomic_seqcst at 272 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### acc_atomic_seqcst dominates: 12% faster than the next best (acc_plain)

acc_atomic_seqcst (272 ns) leads acc_plain (305 ns) by 12%, a clear separation rather than a photo finish. CV 5.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### acc_atomic_seqcst is fastest but the noisiest (CV 5.7%)

acc_atomic_seqcst wins on median (272 ns) yet has the highest variance (CV 5.7%), while acc_plain is the steadiest (CV 4.8%, 305 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: acc_atomic_seqcst** at 272.5 ns median (-11.5% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 272.5 ns, slowest 307.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 2650ns | 2722ns | 2389ns | 2696ns | 2772ns | base |
| acc_atomic_seqcst | 2461ns | 2388ns | 2354ns | 2411ns | 2718ns | -7.14% |
| acc_plain | 2615ns | 2710ns | 2419ns | 2643ns | 2727ns | -1.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 301ns | 273ns | 317ns | base | 0.850 |
| acc_atomic_seqcst | 279ns | 265ns | 307ns | -7.49% | 0.919 |
| acc_plain | 297ns | 274ns | 312ns | -1.27% | 0.861 |

## Performance model

- Peak throughput: **0.965 Gops/s** (acc_atomic_seqcst; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.831 | 86.2% |
| acc_atomic_seqcst | 0.939 | 97.4% |
| acc_plain | 0.839 | 87.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 2650ns | 2650ns | base |
| acc_atomic_seqcst | 2461ns | 2461ns | -7.14% |
| acc_plain | 2615ns | 2615ns | -1.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 308ns | base | --- | [307, 309] | --- | --- | --- | --- |
| acc_atomic_seqcst | 272ns | -29.2ns (-9.5%) | [-33, -18]ns | [270, 275] | YES | 0.0000 | 0.0000 | 3 |
| acc_plain | 305ns | no significant difference | [-2, +0]ns | [296, 306] | no (adj: YES) | 0.0272 | 0.0272 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 274ns | -1.0% | +7.8% |
| 1 | 2 | 307ns | -12.5% | -0.1% |
| 1 | 3 | 305ns | -10.9% | +1.2% |
| 1 | 4 | 310ns | -12.4% | -0.5% |
| 1 | 5 | 310ns | -10.0% | -0.6% |
| 1 | 6 | 294ns | -8.9% | -8.0% |
| 1 | 7 | 310ns | -5.2% | -1.3% |
| 1 | 8 | 309ns | -13.1% | -7.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.112 | ok |
| acc_atomic_seqcst | -0.460 | moderate- |
| acc_plain | -0.232 | moderate- |

**Consistency summary:**

- **acc_atomic_seqcst**: won 8/8, lost 0/8
- **acc_plain**: won 5/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 2.8ns | 301.2ns | 0.9% |  |
| acc_atomic_seqcst | 2.5ns | 278.7ns | 0.9% |  |
| acc_plain | 2.8ns | 297.4ns | 0.9% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 272.7-316.9 ns)
    272.7 |########
    274.9 |##
    277.1 |#####
    279.3 |##
    281.5 |####
    283.8 |#
    286.0 |####
    288.2 |#####
    290.4 |#
    292.6 |#
    294.8 |
    297.0 |
    299.2 |
    301.5 |#
    303.7 |###########################
    305.9 |########################
    308.1 |########################################
    310.3 |###############################
    312.5 |###############
    314.7 |#########
  (17 below, 7 above range)

acc_atomic_seqcst (n=160, range 265.3-307.5 ns)
    265.3 |########################################
    267.4 |################################
    269.5 |################################
    271.7 |################
    273.8 |#################
    275.9 |########################
    278.0 |##############
    280.1 |###
    282.2 |####
    284.3 |#
    286.4 |
    288.5 |####
    290.6 |
    292.7 |
    294.8 |
    297.0 |
    299.1 |
    301.2 |#
    303.3 |######
    305.4 |####################
  (10 below, 13 above range)

acc_plain (n=160, range 274.4-312.4 ns)
    274.4 |###
    276.3 |#####
    278.2 |#####
    280.1 |###
    282.0 |###
    283.9 |#########
    285.8 |#########################
    287.7 |##########
    289.6 |##########
    291.5 |#####
    293.4 |#######
    295.3 |#######
    297.2 |#
    299.1 |###
    301.0 |
    302.9 |#########
    304.8 |#########################
    306.7 |########################################
    308.6 |########################################
    310.5 |################
  (16 below, 15 above range)

```
