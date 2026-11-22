# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (590 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_unaligned at 589 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.1% (fastest 589 ns to slowest 590 ns).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_unaligned is fastest but the noisiest (CV 5.9%)

load_unaligned wins on median (589 ns) yet has the highest variance (CV 5.9%), while load_aligned is the steadiest (CV 1.3%, 590 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (35 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader load_unaligned vs stability leader load_aligned (+0% speed for 4.5x steadier)

load_unaligned is fastest (589 ns, CV 5.9%); load_aligned gives up 0.1% median for 4.5x lower variance (CV 1.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: load_unaligned** at 589.2 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 589.2 ns, slowest 589.6 ns)

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
| load_aligned | 2693ns | 2687ns | 2681ns | 2687ns | 2725ns | base |
| load_byte_pack | 2689ns | 2685ns | 2680ns | 2685ns | 2711ns | -0.16% |
| load_unaligned | 2696ns | 2687ns | 2681ns | 2687ns | 2739ns | +0.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 591ns | 586ns | 598ns | base | 6.936 |
| load_byte_pack | 590ns | 586ns | 598ns | -0.01% | 6.937 |
| load_unaligned | 592ns | 585ns | 608ns | +0.29% | 6.916 |

## Performance model

- Peak throughput: **6.996 Gops/s** (load_unaligned; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 6.947 | 99.3% |
| load_byte_pack | 6.949 | 99.3% |
| load_unaligned | 6.952 | 99.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 2693ns | 2693ns | base |
| load_byte_pack | 2689ns | 2689ns | -0.16% |
| load_unaligned | 2696ns | 2696ns | +0.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 590ns | base | --- | [589, 590] | --- | --- | --- | --- |
| load_byte_pack | 589ns | no significant difference | [-1, +0]ns | [589, 590] | no | 0.1454 | 0.1454 | 7 |
| load_unaligned | 589ns | no significant difference | [-1, +0]ns | [589, 590] | no | 0.1423 | 0.0711 | 11 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 590ns | -0.1% | -0.3% |
| 1 | 2 | 589ns | +0.1% | +0.1% |
| 1 | 3 | 589ns | +0.0% | +0.1% |
| 1 | 4 | 588ns | +0.4% | +0.2% |
| 1 | 5 | 590ns | -0.1% | -0.1% |
| 1 | 6 | 590ns | -0.2% | -0.2% |
| 1 | 7 | 590ns | -0.2% | -0.2% |
| 1 | 8 | 589ns | -0.1% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | -0.053 | ok |
| load_byte_pack | 0.235 | moderate+ |
| load_unaligned | -0.169 | ok |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 2/8
- **load_unaligned**: won 4/8, lost 2/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 2.7ns | 590.6ns | 0.5% |  |
| load_byte_pack | 2.8ns | 590.5ns | 0.5% |  |
| load_unaligned | 2.8ns | 592.2ns | 0.5% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 586.3-597.8 ns)
    586.3 |########
    586.8 |####################
    587.4 |########################
    588.0 |######################
    588.6 |###################
    589.1 |########################################
    589.7 |########
    590.3 |##########################
    590.9 |######
    591.5 |###############
    592.0 |######################
    592.6 |#####
    593.2 |########
    593.8 |######
    594.4 |###
    594.9 |#
    595.5 |#
    596.1 |
    596.7 |#
    597.3 |
  (14 below, 5 above range)

load_byte_pack (n=160, range 585.6-598.5 ns)
    585.6 |######################
    586.2 |########################
    586.9 |####################################
    587.5 |########
    588.2 |########################################
    588.8 |##############
    589.5 |####################################
    590.1 |##############
    590.7 |######################
    591.4 |############
    592.0 |########################
    592.7 |##############
    593.3 |########
    594.0 |############
    594.6 |
    595.3 |
    595.9 |####
    596.6 |####
    597.2 |##
    597.9 |
  (8 below, 4 above range)

load_unaligned (n=160, range 585.5-608.1 ns)
    585.5 |##########
    586.6 |###############################
    587.7 |#####################################
    588.9 |###############################
    590.0 |########################################
    591.1 |#################
    592.3 |###########
    593.4 |#####
    594.5 |#####
    595.7 |##
    596.8 |##
    597.9 |#
    599.1 |#
    600.2 |#
    601.3 |#
    602.5 |
    603.6 |
    604.7 |
    605.9 |
    607.0 |
  (18 below, 1 above range)

```
