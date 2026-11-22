# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 106% faster than the next best (seq_snapshot)

seq_live (368.31 us) leads seq_snapshot (759.55 us) by 106%, a clear separation rather than a photo finish. CV 6.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (368.31 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 368308.6 ns median
- 1 variant significantly slower than baseline
- Spread: 2.06x (fastest 368308.6 ns, slowest 759554.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 377260ns | 369417ns | 357883ns | 373718ns | 407260ns | base |
| seq_snapshot | 792071ns | 759840ns | 746943ns | 774156ns | 890945ns | +109.95% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 375919ns | 356202ns | 406230ns | base | 11.157 |
| seq_snapshot | 791704ns | 746656ns | 890387ns | +110.61% | 5.298 |

## Performance model

- Peak throughput: **11.775 Gops/s** (seq_live; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 11.388 | 96.7% |
| seq_snapshot | 5.522 | 46.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 377260ns | 377260ns | base |
| seq_snapshot | 792071ns | 792071ns | +109.95% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 368309ns | base | --- | [361569, 383376] | --- | --- | --- | --- |
| seq_snapshot | 759554ns | +392683.7ns (+106.6%) | [+386100, +399775]ns | [756288, 766792] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 361218ns | +108.4% |
| 1 | 2 | 358528ns | +113.9% |
| 1 | 3 | 357899ns | +126.4% |
| 1 | 4 | 377978ns | +100.8% |
| 1 | 5 | 359860ns | +128.6% |
| 2 | 1 | 386915ns | +97.4% |
| 2 | 2 | 388281ns | +95.7% |
| 2 | 3 | 379267ns | +100.0% |
| 2 | 4 | 392930ns | +95.3% |
| 2 | 5 | 384790ns | +94.7% |
| 3 | 1 | 384176ns | +96.6% |
| 3 | 2 | 389736ns | +102.2% |
| 3 | 3 | 357025ns | +127.0% |
| 3 | 4 | 357933ns | +111.6% |
| 3 | 5 | 385421ns | +95.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | 0.285 | moderate+ |
| seq_snapshot | -0.100 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 6894837.5ns | 375918.5ns | 1834.1% | HIGH |
| seq_snapshot | 6868180.6ns | 791704.1ns | 867.5% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 356202.1-406229.8 ns)
  356202.1 |########################################
  358703.5 |################
  361204.8 |#############
  363706.2 |###
  366207.6 |####
  368709.0 |#
  371210.4 |###
  373711.8 |#
  376213.1 |#
  378714.5 |###
  381215.9 |######
  383717.3 |################
  386218.7 |#######
  388720.1 |##########
  391221.5 |##########
  393722.8 |##########
  396224.2 |######
  398725.6 |#
  401227.0 |###
  403728.4 |
  (10 below, 4 above range)

seq_snapshot (n=120, range 746656.2-890386.6 ns)
  746656.2 |########################################
  753842.7 |#######################
  761029.2 |##############
  768215.8 |#######
  775402.3 |#
  782588.8 |
  789775.3 |
  796961.8 |#
  804148.3 |
  811334.9 |###
  818521.4 |
  825707.9 |##
  832894.4 |#
  840080.9 |##
  847267.5 |####
  854454.0 |####
  861640.5 |#######
  868827.0 |###
  876013.5 |###
  883200.0 |
  (9 below, 9 above range)

```

## Diagnostics

- **seq_live**: bridge=1862.3% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=899.1% of algo (FFI overhead may distort results)
