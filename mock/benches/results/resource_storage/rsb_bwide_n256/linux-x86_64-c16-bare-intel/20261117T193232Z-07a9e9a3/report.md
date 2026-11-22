# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Highlights

Baseline for all deltas below: **wide_blob**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wide_blob) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wide_blob has the worst median (53.81 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wide_decomposed at 47.64 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wide_decomposed dominates: 13% faster than the next best (wide_blob)

wide_decomposed (47.64 us) leads wide_blob (53.81 us) by 13%, a clear separation rather than a photo finish. CV 9.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### wide_decomposed is fastest but the noisiest (CV 9.8%)

wide_decomposed wins on median (47.64 us) yet has the highest variance (CV 9.8%), while wide_blob is the steadiest (CV 8.9%, 53.81 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: wide_decomposed** at 47644.2 ns median (-11.5% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.13x (fastest 47644.2 ns, slowest 53810.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Intra-resource member gather at M=64: contiguous blob vs scattered columns |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112727760 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 49974ns | 53867ns | 44069ns | 50621ns | 53936ns | base |
| wide_decomposed | 50581ns | 47694ns | 47685ns | 48958ns | 58344ns | +1.21% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 49919ns | 44022ns | 53873ns | base | 0.005 |
| wide_decomposed | 50529ns | 47639ns | 58285ns | +1.22% | 0.005 |

## Performance model

- Peak throughput: **0.006 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 0.005 | 81.8% |
| wide_decomposed | 0.005 | 92.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 49974ns | 49974ns | base |
| wide_decomposed | 50581ns | 50581ns | +1.21% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 53811ns | base | --- | [53806, 53813] | --- | --- | --- | --- |
| wide_decomposed | 47644ns | +3615.1ns (+6.7%) | [+3355, +3618]ns | [47642, 47650] | YES | 0.0353 | 0.0353 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wide_blob | wide_decomposed |
|---|---|---|---|
| 1 | 1 | 53815ns | -11.5% |
| 1 | 2 | 53816ns | -11.5% |
| 1 | 3 | 44028ns | +32.3% |
| 1 | 4 | 53811ns | -11.5% |
| 1 | 5 | 44022ns | +8.2% |
| 2 | 1 | 53814ns | -11.5% |
| 2 | 2 | 44024ns | +8.2% |
| 2 | 3 | 53817ns | -11.5% |
| 2 | 4 | 53815ns | -11.5% |
| 2 | 5 | 53815ns | +8.2% |
| 3 | 1 | 53813ns | +8.2% |
| 3 | 2 | 44035ns | +8.2% |
| 3 | 3 | 44027ns | +8.2% |
| 3 | 4 | 44024ns | +8.2% |
| 3 | 5 | 53822ns | +8.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | -0.156 | ok |
| wide_decomposed | 0.044 | ok |

**Consistency summary:**

- **wide_decomposed**: won 6/15, lost 9/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 108.8ns | 49919.2ns | 0.2% |  |
| wide_decomposed | 323.0ns | 50529.4ns | 0.6% |  |

## Distribution (algo ns)

```
wide_blob (n=120, range 44022.4-53872.9 ns)
  44022.4 |######################
  44514.9 |
  45007.5 |
  45500.0 |
  45992.5 |
  46485.0 |
  46977.6 |
  47470.1 |
  47962.6 |
  48455.1 |
  48947.6 |
  49440.2 |
  49932.7 |
  50425.2 |
  50917.7 |
  51410.3 |
  51902.8 |
  52395.3 |
  52887.8 |
  53380.3 |########################################
  (11 below, 5 above range)

wide_decomposed (n=120, range 47639.2-58285.2 ns)
  47639.2 |########################################
  48171.5 |
  48703.8 |
  49236.1 |
  49768.4 |#
  50300.7 |
  50833.0 |
  51365.3 |
  51897.6 |
  52429.9 |
  52962.2 |
  53494.5 |
  54026.8 |
  54559.1 |
  55091.4 |
  55623.7 |
  56156.0 |
  56688.3 |
  57220.6 |
  57752.9 |#############
  (7 below, 6 above range)

```
