# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (13 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 12 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### layout_soa is fastest but the noisiest (CV 27.9%)

layout_soa wins on median (12 ns) yet has the highest variance (CV 27.9%), while layout_aos is the steadiest (CV 20.0%, 13 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (3 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader layout_soa vs stability leader layout_aos (+3% speed for 1.4x steadier)

layout_soa is fastest (12 ns, CV 27.9%); layout_aos gives up 3.2% median for 1.4x lower variance (CV 20.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_soa is inconsistent: worst-20% is 2.4x its best-20%

layout_soa's best 20% of batches run at 7 ns but its worst 20% at 16 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: layout_soa** at 12.5 ns median (-3.1% vs baseline)
- Spread: 1.03x (fastest 12.5 ns, slowest 12.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | AoS vs SoA (column-store) layout under partial-field iteration |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eadbeef01234567 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 2186ns | 2161ns | 2085ns | 2150ns | 2396ns | base |
| layout_soa | 2358ns | 2406ns | 2138ns | 2404ns | 2441ns | +7.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 12ns | 8ns | 15ns | base | 82.369 |
| layout_soa | 12ns | 7ns | 16ns | -3.49% | 85.351 |

## Performance model

- Peak throughput: **150.105 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 79.380 | 52.9% |
| layout_soa | 81.920 | 54.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2186ns | 2186ns | base |
| layout_soa | 2358ns | 2358ns | +7.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 13ns | base | --- | [12, 13] | --- | --- | --- | --- |
| layout_soa | 12ns | no significant difference | [-1, +0]ns | [11, 13] | no | 0.9350 | 0.9350 | 10 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 7ns | +13.7% |
| 1 | 2 | 14ns | -5.9% |
| 1 | 3 | 13ns | +10.1% |
| 1 | 4 | 13ns | +14.3% |
| 1 | 5 | 13ns | -35.7% |
| 1 | 6 | 13ns | +5.0% |
| 1 | 7 | 12ns | -31.6% |
| 1 | 8 | 13ns | +21.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | -0.246 | moderate- |
| layout_soa | -0.427 | moderate- |

**Consistency summary:**

- **layout_soa**: won 3/8, lost 5/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.3ns | 12.4ns | 26.7% | HIGH |
| layout_soa | 3.5ns | 12.0ns | 28.9% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 8.4-15.4 ns)
      8.4 |
      8.7 |######
      9.1 |##########
      9.4 |##########
      9.8 |######
     10.1 |########
     10.5 |##########
     10.8 |################
     11.2 |
     11.6 |########
     11.9 |##################
     12.3 |#############################
     12.6 |#########################
     13.0 |########################################
     13.3 |
     13.7 |###############################
     14.0 |#########################
     14.4 |#####################
     14.7 |##########
     15.1 |########
  (13 below, 10 above range)

layout_soa (n=160, range 6.8-16.4 ns)
      6.8 |##########
      7.3 |##
      7.8 |##################
      8.3 |################
      8.7 |#############################
      9.2 |##########
      9.7 |##########
     10.2 |################
     10.6 |################
     11.1 |################
     11.6 |#####
     12.1 |#####################################
     12.6 |########
     13.0 |##################
     13.5 |#############################
     14.0 |#####################
     14.5 |########
     15.0 |########################################
     15.4 |########################
     15.9 |##################
  (14 below, 12 above range)

```

## Diagnostics

- **layout_aos**: CV=20.7% (high variance, measurements may be unstable)
- **layout_aos**: bridge=22.5% of algo (FFI overhead may distort results)
- **layout_soa**: CV=29.0% (high variance, measurements may be unstable)
- **layout_soa**: bridge=26.4% of algo (FFI overhead may distort results)
