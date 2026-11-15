# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (fpt_match_16) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline fpt_match_16 has the worst median (71 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest fpt_table_16 at 45 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### fpt_table_16 dominates: 58% faster than the next best (fpt_match_16)

fpt_table_16 (45 ns) leads fpt_match_16 (71 ns) by 58%, a clear separation rather than a photo finish. CV 11.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_table_16 beats baseline by 35% (significant)

fpt_table_16 is -25 ns (35%) faster than baseline fpt_match_16, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fpt_table_16 is fastest but the noisiest (CV 11.3%)

fpt_table_16 wins on median (45 ns) yet has the highest variance (CV 11.3%), while fpt_match_16 is the steadiest (CV 7.0%, 71 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fpt_table_16 shows alternating (throttle bounce) (autocorr -0.52)

fpt_table_16's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: fpt_table_16** at 45.0 ns median (-36.6% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.58x (fastest 45.0 ns, slowest 71.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 2366ns | 2443ns | 2154ns | 2404ns | 2465ns | base |
| fpt_table_16 | 2261ns | 2187ns | 2098ns | 2252ns | 2452ns | -4.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 71ns | 64ns | 78ns | base | 3.618 |
| fpt_table_16 | 46ns | 39ns | 53ns | -35.57% | 5.615 |

## Performance model

- Peak throughput: **6.616 Gops/s** (fpt_table_16; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 3.606 | 54.5% |
| fpt_table_16 | 5.689 | 86.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 2366ns | 2366ns | base |
| fpt_table_16 | 2261ns | 2261ns | -4.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 71ns | base | --- | [70, 72] | --- | --- | --- | --- |
| fpt_table_16 | 45ns | -24.6ns (-34.6%) | [-26, -24]ns | [44, 47] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 69ns | -37.4% |
| 1 | 2 | 72ns | -36.1% |
| 1 | 3 | 73ns | -34.5% |
| 1 | 4 | 73ns | -40.6% |
| 1 | 5 | 65ns | -33.4% |
| 1 | 6 | 72ns | -30.2% |
| 1 | 7 | 72ns | -40.0% |
| 1 | 8 | 69ns | -31.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.259 | moderate- |
| fpt_table_16 | -0.519 | HIGH- (thermal bounce) |

**Consistency summary:**

- **fpt_table_16**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 3.5ns | 70.8ns | 4.9% |  |
| fpt_table_16 | 2.3ns | 45.6ns | 5.1% | HIGH |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 63.6-77.6 ns)
     63.6 |######
     64.3 |#########
     65.0 |###########################
     65.7 |########################
     66.4 |###
     67.1 |############
     67.8 |#####################
     68.5 |#####################
     69.2 |########################################
     69.9 |#################################
     70.6 |##################
     71.3 |##################
     72.0 |########################################
     72.7 |#################################
     73.4 |###############
     74.1 |##############################
     74.8 |##################
     75.5 |######
     76.2 |##################
     76.9 |###############
  (13 below, 12 above range)

fpt_table_16 (n=160, range 38.7-52.7 ns)
     38.7 |#########
     39.4 |###############
     40.1 |#########
     40.8 |#####################
     41.5 |#####################
     42.2 |#####################
     42.9 |########################################
     43.6 |####################################
     44.3 |##################
     45.0 |##############################
     45.7 |###############
     46.4 |###############
     47.1 |########################
     47.8 |########################################
     48.5 |##############################
     49.2 |#########
     49.9 |##############################
     50.6 |############
     51.3 |#########
     52.0 |###############
  (12 below, 9 above range)

```
