# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Highlights

Baseline for all deltas below: **layout_aos**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (layout_aos) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline layout_aos has the worst median (13 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest layout_soa at 12 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### layout_soa is fastest but the noisiest (CV 17.0%)

layout_soa wins on median (12 ns) yet has the highest variance (CV 17.0%), while layout_aos is the steadiest (CV 14.4%, 13 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (2 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader layout_soa vs stability leader layout_aos (+3% speed for 1.2x steadier)

layout_soa is fastest (12 ns, CV 17.0%); layout_aos gives up 3.2% median for 1.2x lower variance (CV 14.4%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### layout_soa is inconsistent: worst-20% is 1.6x its best-20%

layout_soa's best 20% of batches run at 10 ns but its worst 20% at 16 ns (1.6x) - a bimodal or bursty profile the median hides.

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
| layout_aos | 2102ns | 2100ns | 2094ns | 2100ns | 2119ns | base |
| layout_soa | 2104ns | 2099ns | 2094ns | 2100ns | 2125ns | +0.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 13ns | 10ns | 15ns | base | 80.757 |
| layout_soa | 13ns | 10ns | 16ns | +0.30% | 80.519 |

## Performance model

- Peak throughput: **102.915 Gops/s** (layout_soa; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 79.380 | 77.1% |
| layout_soa | 81.920 | 79.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 2102ns | 2102ns | base |
| layout_soa | 2104ns | 2104ns | +0.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 13ns | base | --- | [12, 13] | --- | --- | --- | --- |
| layout_soa | 12ns | no significant difference | [-0, +0]ns | [12, 13] | no | 0.6792 | 0.6792 | 14 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | layout_aos | layout_soa |
|---|---|---|---|
| 1 | 1 | 14ns | -7.7% |
| 1 | 2 | 13ns | -3.1% |
| 1 | 3 | 12ns | -1.6% |
| 1 | 4 | 13ns | +0.0% |
| 1 | 5 | 13ns | +6.7% |
| 1 | 6 | 12ns | -5.3% |
| 1 | 7 | 13ns | -1.6% |
| 1 | 8 | 12ns | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.018 | ok |
| layout_soa | -0.476 | moderate- |

**Consistency summary:**

- **layout_soa**: won 6/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.0ns | 12.7ns | 23.7% | HIGH |
| layout_soa | 3.1ns | 12.7ns | 24.4% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 10.0-15.2 ns)
     10.0 |
     10.3 |########
     10.5 |
     10.8 |######################
     11.1 |####################
     11.3 |
     11.6 |############################
     11.8 |##########################
     12.1 |
     12.4 |#################################
     12.6 |
     12.9 |########################################
     13.1 |###############################
     13.4 |
     13.7 |############################
     13.9 |
     14.2 |########################
     14.5 |########
     14.7 |
     15.0 |#################
  (15 below, 14 above range)

layout_soa (n=160, range 10.0-15.8 ns)
     10.0 |################
     10.2 |################
     10.5 |########################################
     10.8 |
     11.1 |#############################
     11.4 |########################
     11.7 |
     12.0 |#####################################
     12.3 |#####################################
     12.6 |
     12.9 |#####################################
     13.2 |##################
     13.5 |
     13.7 |########################
     14.0 |########################
     14.3 |##################
     14.6 |
     14.9 |##########################
     15.2 |#############
     15.5 |
  (10 below, 14 above range)

```

## Diagnostics

- **layout_aos**: bridge=24.0% of algo (FFI overhead may distort results)
- **layout_soa**: bridge=23.2% of algo (FFI overhead may distort results)
