# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (95 ns) is smaller than the fastest variant's own run-to-run std-dev (343 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Whole field within 0.5% of the fastest

All 3 variants sit between 20.84 us and 20.93 us - a 0.5% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: inline_default** at 20836.4 ns median (-0.1% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.00x (fastest 20836.4 ns, slowest 20931.5 ns)

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
| inline_always | 23029ns | 23004ns | 22603ns | 23010ns | 23512ns | base |
| inline_default | 23011ns | 22980ns | 22612ns | 23005ns | 23428ns | -0.08% |
| inline_never | 23066ns | 23062ns | 22700ns | 23072ns | 23415ns | +0.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 20874ns | 20493ns | 21287ns | base | 0.785 |
| inline_default | 20862ns | 20509ns | 21222ns | -0.06% | 0.785 |
| inline_never | 20931ns | 20598ns | 21244ns | +0.28% | 0.783 |

## Performance model

- Peak throughput: **0.800 Gops/s** (inline_always; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.786 | 98.3% |
| inline_default | 0.786 | 98.3% |
| inline_never | 0.783 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 23029ns | 23029ns | base |
| inline_default | 23011ns | 23011ns | -0.08% |
| inline_never | 23066ns | 23066ns | +0.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 20851ns | base | --- | [20819, 20882] | --- | --- | --- | --- |
| inline_default | 20836ns | no significant difference | [-43, +28]ns | [20813, 20901] | no | 0.5801 | 0.5801 | 0 |
| inline_never | 20931ns | +78.1ns (+0.4%) | [+50, +137]ns | [20903, 20957] | YES | 0.0013 | 0.0006 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 20893ns | -0.7% | +0.2% |
| 1 | 2 | 20922ns | -0.1% | +0.2% |
| 1 | 3 | 20872ns | +0.2% | -0.0% |
| 1 | 4 | 20885ns | -0.2% | -0.9% |
| 1 | 5 | 20710ns | +1.0% | +0.6% |
| 1 | 6 | 20887ns | -0.3% | +0.7% |
| 1 | 7 | 20654ns | -0.7% | +1.6% |
| 1 | 8 | 20556ns | +1.8% | +1.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | 0.258 | moderate+ |
| inline_default | -0.199 | ok |
| inline_never | 0.347 | moderate+ |

**Consistency summary:**

- **inline_default**: won 4/8, lost 3/8
- **inline_never**: won 1/8, lost 6/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 3.1ns | 20873.7ns | 0.0% |  |
| inline_default | 3.0ns | 20862.0ns | 0.0% |  |
| inline_never | 3.5ns | 20931.4ns | 0.0% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 20492.6-21286.7 ns)
  20492.6 |########
  20532.3 |######
  20572.0 |#
  20611.7 |#
  20651.4 |
  20691.1 |
  20730.8 |##
  20770.5 |########################################
  20810.2 |#####################
  20849.9 |#########################
  20889.6 |##################
  20929.3 |########
  20969.0 |##########
  21008.7 |########
  21048.4 |######
  21088.1 |##########
  21127.9 |##
  21167.6 |
  21207.3 |#####
  21247.0 |
  (18 below, 9 above range)

inline_default (n=160, range 20508.9-21221.5 ns)
  20508.9 |#########
  20544.5 |#####
  20580.1 |#
  20615.8 |##
  20651.4 |####
  20687.0 |
  20722.7 |##
  20758.3 |#################
  20793.9 |########################################
  20829.6 |##########
  20865.2 |#########
  20900.8 |#############################
  20936.5 |################
  20972.1 |##########
  21007.7 |#########
  21043.4 |########
  21079.0 |##
  21114.6 |####
  21150.3 |
  21185.9 |
  (15 below, 8 above range)

inline_never (n=160, range 20598.3-21244.2 ns)
  20598.3 |###
  20630.6 |###
  20662.9 |#####
  20695.2 |
  20727.5 |
  20759.8 |
  20792.1 |#
  20824.4 |##########################
  20856.7 |########################################
  20889.0 |###################
  20921.3 |########################
  20953.6 |###################
  20985.9 |###############
  21018.2 |########################
  21050.4 |############
  21082.7 |##########
  21115.0 |###
  21147.3 |######
  21179.6 |########
  21211.9 |#
  (19 below, 11 above range)

```
