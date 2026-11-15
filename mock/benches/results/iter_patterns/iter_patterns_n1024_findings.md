# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (265 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 4 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 3076% faster than the next best (iter_seq)

iter_strided (4 ns) leads iter_seq (118 ns) by 3076%, a clear separation rather than a photo finish. CV 36.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 98% (significant)

iter_strided is -261 ns (98%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 71.7x slower than the field

iter_gather (265 ns) is 71.7x the fastest (4 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 36.2%)

iter_strided wins on median (4 ns) yet has the highest variance (CV 36.2%), while iter_gather is the steadiest (CV 6.2%, 265 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_strided shows alternating (throttle bounce) (autocorr -0.63)

iter_strided's per-pass series has lag-1 autocorrelation -0.63, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 71.7x the fastest

Fastest iter_strided (4 ns) to slowest iter_gather (265 ns): 71.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 2.7x its best-20%

iter_strided's best 20% of batches run at 2 ns but its worst 20% at 6 ns (2.7x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 3.7 ns median (-98.6% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 71.73x (fastest 3.7 ns, slowest 265.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2507ns | 2499ns | 2324ns | 2501ns | 2708ns | base |
| iter_seq | 2395ns | 2362ns | 2198ns | 2400ns | 2580ns | -4.46% |
| iter_strided | 2309ns | 2403ns | 2105ns | 2339ns | 2422ns | -7.92% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 267ns | 247ns | 289ns | base | 3.839 |
| iter_seq | 117ns | 107ns | 128ns | -56.01% | 8.728 |
| iter_strided | 4ns | 2ns | 6ns | -98.52% | 259.528 |

## Performance model

- Peak throughput: **475.588 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 3.858 | 0.8% |
| iter_seq | 8.715 | 1.8% |
| iter_strided | 276.757 | 58.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2507ns | 2507ns | base |
| iter_seq | 2395ns | 2395ns | -4.46% |
| iter_strided | 2309ns | 2309ns | -7.92% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 265ns | base | --- | [262, 268] | --- | --- | --- | --- |
| iter_seq | 118ns | -146.2ns (-55.1%) | [-153, -144]ns | [115, 120] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 4ns | -261.3ns (-98.5%) | [-264, -258]ns | [4, 4] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 248ns | -55.9% | -98.3% |
| 1 | 2 | 286ns | -56.9% | -98.8% |
| 1 | 3 | 248ns | -55.2% | -98.2% |
| 1 | 4 | 249ns | -51.2% | -98.3% |
| 1 | 5 | 261ns | -54.6% | -98.8% |
| 1 | 6 | 272ns | -60.2% | -98.4% |
| 1 | 7 | 266ns | -53.4% | -98.6% |
| 1 | 8 | 286ns | -56.9% | -98.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.211 | moderate- |
| iter_seq | -0.427 | moderate- |
| iter_strided | -0.626 | HIGH- (thermal bounce) |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.7ns | 266.7ns | 1.0% |  |
| iter_seq | 3.0ns | 117.3ns | 2.5% |  |
| iter_strided | 14.2ns | 3.9ns | 358.7% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 246.6-289.3 ns)
    246.6 |########################################
    248.7 |##########################
    250.8 |##########
    253.0 |######
    255.1 |######
    257.2 |
    259.4 |##########
    261.5 |######
    263.7 |#################
    265.8 |########################
    267.9 |############
    270.1 |###
    272.2 |
    274.4 |#
    276.5 |
    278.6 |#####
    280.8 |#####
    282.9 |###############
    285.0 |####################
    287.2 |###########################
  (9 below, 12 above range)

iter_seq (n=160, range 106.7-127.8 ns)
    106.7 |##
    107.8 |########################################
    108.8 |###########
    109.9 |########
    110.9 |######
    112.0 |###############
    113.0 |####
    114.1 |###############################
    115.1 |########
    116.2 |###########
    117.3 |###############
    118.3 |###########
    119.4 |########################
    120.4 |####
    121.5 |########
    122.5 |###########
    123.6 |####################
    124.6 |#############
    125.7 |######################
    126.7 |#################
  (16 below, 14 above range)

iter_strided (n=160, range 2.2-5.8 ns)
      2.2 |
      2.3 |##################
      2.5 |
      2.7 |
      2.9 |##############################
      3.1 |
      3.3 |########################################
      3.4 |
      3.6 |#########################
      3.8 |
      4.0 |
      4.2 |#############################
      4.4 |
      4.5 |########################################
      4.7 |
      4.9 |##################
      5.1 |
      5.3 |##############################
      5.5 |
      5.7 |#######
  (18 below, 10 above range)

```

## Diagnostics

- **iter_strided**: CV=34.0% (high variance, measurements may be unstable)
- **iter_strided**: bridge=383.8% of algo (FFI overhead may distort results)
