# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (54 ns) is smaller than the fastest variant's own run-to-run std-dev (361 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (inline_always)

The baseline inline_always is the fastest (20.46 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 0.3% of the fastest

All 3 variants sit between 20.46 us and 20.51 us - a 0.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (inline_always) is the fastest** at 20461.4 ns median
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 20461.4 ns, slowest 20515.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 22775ns | 22567ns | 22501ns | 22634ns | 23471ns | base |
| inline_default | 22776ns | 22592ns | 22506ns | 22646ns | 23434ns | +0.00% |
| inline_never | 22829ns | 22609ns | 22537ns | 22689ns | 23544ns | +0.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 20646ns | 20401ns | 21271ns | base | 0.794 |
| inline_default | 20649ns | 20402ns | 21248ns | +0.02% | 0.793 |
| inline_never | 20716ns | 20454ns | 21368ns | +0.34% | 0.791 |

## Performance model

- Peak throughput: **0.803 Gops/s** (inline_always; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.801 | 99.7% |
| inline_default | 0.800 | 99.6% |
| inline_never | 0.799 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 22775ns | 22775ns | base |
| inline_default | 22776ns | 22776ns | +0.00% |
| inline_never | 22829ns | 22829ns | +0.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 20461ns | base | --- | [20448, 20482] | --- | --- | --- | --- |
| inline_default | 20486ns | no significant difference | [-7, +30]ns | [20462, 20522] | no | 0.8126 | 0.8126 | 0 |
| inline_never | 20515ns | +63.7ns (+0.3%) | [+45, +80]ns | [20498, 20573] | YES | 0.0000 | 0.0000 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 20498ns | -0.1% | +0.0% |
| 1 | 2 | 20466ns | +0.5% | +0.1% |
| 1 | 3 | 20447ns | +0.5% | +0.4% |
| 1 | 4 | 20780ns | -1.7% | +0.3% |
| 1 | 5 | 20436ns | +0.0% | +1.6% |
| 1 | 6 | 20467ns | -0.1% | +1.6% |
| 1 | 7 | 20436ns | +0.3% | +0.3% |
| 1 | 8 | 20429ns | +3.2% | +0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.227 | moderate- |
| inline_default | 0.001 | ok |
| inline_never | 0.354 | moderate+ |

**Consistency summary:**

- **inline_default**: won 1/8, lost 4/8
- **inline_never**: won 0/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.3ns | 20645.6ns | 0.0% |  |
| inline_default | 3.2ns | 20648.9ns | 0.0% |  |
| inline_never | 3.7ns | 20715.6ns | 0.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 20401.3-21271.3 ns)
  20401.3 |########################################
  20444.8 |#########################
  20488.3 |#######
  20531.8 |##
  20575.3 |
  20618.8 |###
  20662.3 |##
  20705.8 |
  20749.3 |#
  20792.8 |#####
  20836.3 |##
  20879.8 |#
  20923.3 |
  20966.8 |#
  21010.3 |
  21053.8 |
  21097.3 |#######
  21140.8 |####
  21184.3 |
  21227.8 |#
  (16 below, 9 above range)

inline_default (n=160, range 20402.2-21248.2 ns)
  20402.2 |########################################
  20444.5 |##########################
  20486.8 |#############
  20529.1 |#########
  20571.4 |########
  20613.7 |##
  20656.0 |##
  20698.3 |##
  20740.6 |
  20782.9 |#####
  20825.2 |###
  20867.5 |#
  20909.8 |##
  20952.1 |##
  20994.4 |#
  21036.7 |##
  21079.0 |#######
  21121.3 |#####
  21163.6 |#####
  21205.9 |##
  (16 below, 7 above range)

inline_never (n=160, range 20453.9-21367.7 ns)
  20453.9 |########################################
  20499.6 |#############
  20545.3 |######
  20590.9 |#####
  20636.6 |##
  20682.3 |#
  20728.0 |#
  20773.7 |#
  20819.4 |########
  20865.1 |#
  20910.8 |#
  20956.5 |
  21002.2 |##
  21047.8 |
  21093.5 |
  21139.2 |##
  21184.9 |#####
  21230.6 |
  21276.3 |##
  21322.0 |#
  (15 below, 11 above range)

```
