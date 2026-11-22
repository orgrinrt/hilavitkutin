# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (602 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_unaligned at 602 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 602 ns to slowest 602 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (11 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: load_unaligned** at 601.9 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 601.9 ns, slowest 602.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x2c8af49106b7ed53 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 2761ns | 2737ns | 2722ns | 2743ns | 2853ns | base |
| load_byte_pack | 2751ns | 2738ns | 2723ns | 2741ns | 2807ns | -0.35% |
| load_unaligned | 2750ns | 2735ns | 2723ns | 2740ns | 2807ns | -0.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 606ns | 597ns | 623ns | base | 6.763 |
| load_byte_pack | 605ns | 597ns | 620ns | -0.17% | 6.774 |
| load_unaligned | 605ns | 597ns | 619ns | -0.14% | 6.773 |

## Performance model

- Peak throughput: **6.860 Gops/s** (load_byte_pack; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.798 | 99.1% |
| load_byte_pack | 6.803 | 99.2% |
| load_unaligned | 6.805 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2761ns | 2761ns | base |
| load_byte_pack | 2751ns | 2751ns | -0.35% |
| load_unaligned | 2750ns | 2750ns | -0.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 602ns | base | --- | [602, 603] | --- | --- | --- | --- |
| load_byte_pack | 602ns | no significant difference | [-1, +1]ns | [601, 603] | no | 0.3382 | 0.3382 | 3 |
| load_unaligned | 602ns | no significant difference | [-1, +0]ns | [601, 603] | no | 0.3382 | 0.2267 | 6 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 602ns | +0.4% | +0.1% |
| 1 | 2 | 601ns | -0.1% | -0.0% |
| 1 | 3 | 601ns | +0.1% | -0.1% |
| 1 | 4 | 601ns | +0.1% | +1.6% |
| 1 | 5 | 602ns | -0.1% | +0.0% |
| 1 | 6 | 607ns | -1.3% | -1.3% |
| 1 | 7 | 612ns | -1.5% | -1.9% |
| 1 | 8 | 602ns | +0.9% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.292 | moderate+ |
| load_byte_pack | 0.050 | ok |
| load_unaligned | -0.106 | ok |

**Consistency summary:**

- **load_byte_pack**: won 4/8, lost 4/8
- **load_unaligned**: won 2/8, lost 1/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.8ns | 605.7ns | 0.5% |  |
| load_byte_pack | 2.8ns | 604.6ns | 0.5% |  |
| load_unaligned | 2.7ns | 604.8ns | 0.5% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 597.4-622.6 ns)
    597.4 |###################
    598.6 |##########################
    599.9 |####################################
    601.2 |########################################
    602.4 |########################################
    603.7 |############################
    604.9 |#######
    606.2 |#######
    607.5 |#######
    608.7 |#
    610.0 |###
    611.2 |#####
    612.5 |#####
    613.7 |###########
    615.0 |###
    616.3 |
    617.5 |#
    618.8 |#####
    620.0 |#
    621.3 |
  (14 below, 12 above range)

load_byte_pack (n=160, range 597.1-620.0 ns)
    597.1 |####################
    598.2 |########################################
    599.3 |####################
    600.5 |####################
    601.6 |################
    602.8 |#######################
    603.9 |####################
    605.1 |###############
    606.2 |######
    607.4 |#######
    608.5 |######
    609.7 |
    610.8 |####
    612.0 |######
    613.1 |####
    614.2 |#
    615.4 |#
    616.5 |#
    617.7 |####
    618.8 |
  (12 below, 5 above range)

load_unaligned (n=160, range 597.3-619.3 ns)
    597.3 |########################################
    598.4 |######################
    599.5 |##################################
    600.6 |######################
    601.7 |########################
    602.8 |###############
    603.9 |###############
    605.0 |######################
    606.1 |#########
    607.2 |#####
    608.3 |#########
    609.4 |#####
    610.5 |#######
    611.6 |#####
    612.7 |#######
    613.8 |#####
    614.9 |#####
    616.0 |#
    617.1 |###
    618.2 |###
  (12 below, 6 above range)

```
