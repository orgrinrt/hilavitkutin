# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (328.49 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 328486.0 ns median
- 1 variant significantly slower than baseline
- Spread: 1.02x (fastest 328486.0 ns, slowest 334208.2 ns)

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
| seq_live | 330622ns | 329672ns | 328624ns | 330074ns | 334267ns | base |
| seq_snapshot | 336952ns | 334875ns | 334216ns | 335718ns | 343389ns | +1.91% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 329877ns | 327983ns | 333709ns | base | 0.199 |
| seq_snapshot | 336524ns | 333696ns | 343254ns | +2.02% | 0.195 |

## Performance model

- Peak throughput: **0.200 Gops/s** (seq_live; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 0.200 | 99.8% |
| seq_snapshot | 0.196 | 98.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 330622ns | 330622ns | base |
| seq_snapshot | 336952ns | 336952ns | +1.91% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 328486ns | base | --- | [328327, 329212] | --- | --- | --- | --- |
| seq_snapshot | 334208ns | +5742.2ns (+1.7%) | [+5534, +5945]ns | [334031, 334956] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 332237ns | +1.8% |
| 1 | 2 | 330887ns | +0.9% |
| 1 | 3 | 328232ns | +1.7% |
| 1 | 4 | 328342ns | +1.7% |
| 1 | 5 | 328545ns | +1.6% |
| 2 | 1 | 328935ns | +1.6% |
| 2 | 2 | 328276ns | +1.7% |
| 2 | 3 | 328467ns | +1.7% |
| 2 | 4 | 328344ns | +1.8% |
| 2 | 5 | 328131ns | +4.7% |
| 3 | 1 | 328967ns | +1.5% |
| 3 | 2 | 328337ns | +1.7% |
| 3 | 3 | 328043ns | +3.0% |
| 3 | 4 | 330288ns | +3.1% |
| 3 | 5 | 334516ns | +0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | 0.297 | moderate+ |
| seq_snapshot | 0.035 | ok |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 106919.4ns | 329876.5ns | 32.4% | HIGH |
| seq_snapshot | 107620.5ns | 336524.4ns | 32.0% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 327983.0-333709.0 ns)
  327983.0 |########################################
  328269.3 |####################
  328555.6 |##
  328841.9 |####
  329128.2 |######
  329414.5 |##
  329700.8 |
  329987.1 |##
  330273.4 |####
  330559.7 |##
  330846.0 |#
  331132.3 |#####
  331418.6 |##
  331704.9 |#
  331991.2 |######
  332277.5 |######
  332563.8 |#
  332850.1 |
  333136.4 |#
  333422.7 |
  (9 below, 13 above range)

seq_snapshot (n=120, range 333695.5-343254.3 ns)
  333695.5 |########################################
  334173.5 |######
  334651.4 |####
  335129.4 |#
  335607.3 |
  336085.2 |
  336563.2 |
  337041.1 |###
  337519.1 |####
  337997.0 |######
  338474.9 |###
  338952.9 |#
  339430.8 |
  339908.8 |##
  340386.7 |##
  340864.6 |
  341342.6 |
  341820.5 |
  342298.5 |
  342776.4 |
  (10 below, 9 above range)

```

## Diagnostics

- **seq_live**: bridge=32.4% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=32.0% of algo (FFI overhead may distort results)
