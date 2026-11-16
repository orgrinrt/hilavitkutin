# build-static: every crate built into an empty target directory

2 variants, 3 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (llvm_non_incremental) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline llvm_non_incremental has the worst median (16002.08 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest llvm_incremental at 15952.99 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.3% (fastest 15952.99 ms to slowest 16002.08 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### llvm_incremental is fastest but the noisiest (CV 21.0%)

llvm_incremental wins on median (15952.99 ms) yet has the highest variance (CV 21.0%), while llvm_non_incremental is the steadiest (CV 6.1%, 16002.08 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (49.09 ms) is smaller than the fastest variant's own run-to-run std-dev (3343.84 ms); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader llvm_incremental vs stability leader llvm_non_incremental (+0% speed for 3.4x steadier)

llvm_incremental is fastest (15952.99 ms, CV 21.0%); llvm_non_incremental gives up 0.3% median for 3.4x lower variance (CV 6.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: llvm_incremental** at 15952990889.2 ns median (-0.3% vs baseline)
- Spread: 1.00x (fastest 15952990889.2 ns, slowest 16002078372.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 3 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2300364018 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 18047639288ns | 15952994593ns | 15423280774ns | -0ns | 18047639288ns | +12.23% |
| llvm_non_incremental | 16080748916ns | 16002085623ns | 14924850562ns | -0ns | 16080748916ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 18047635446ns | 15423277662ns | 18047635446ns | +12.23% |
| llvm_non_incremental | 16080743438ns | 14924846840ns | 16080743438ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 18047639288ns | 18047639288ns | +12.23% |
| llvm_non_incremental | 16080748916ns | 16080748916ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 16002078372ns | base | --- | [14924846840, 17315305101] | --- | --- | --- | --- |
| llvm_incremental | 15952990889ns | no significant difference | [-49087483, +5451332686]ns | [15423277662, 22766637788] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 17315305101ns | +31.5% |
| 1 | 2 | 16002078372ns | -0.3% |
| 1 | 3 | 14924846840ns | +3.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.131 | ok |
| llvm_non_incremental | -0.002 | ok |

**Consistency summary:**

- **llvm_incremental**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 66422610.1ns | 18047635446.1ns | 0.4% |  |
| llvm_non_incremental | 37360113.9ns | 16080743437.9ns | 0.2% |  |

## Distribution (algo ns)

```
llvm_incremental (n=3, range 15423277661.5-18047635446.1 ns)
  15423277661.5 |########################################
  15554495550.7 |
  15685713440.0 |
  15816931329.2 |
  15948149218.4 |########################################
  16079367107.7 |
  16210584996.9 |
  16341802886.1 |
  16473020775.3 |
  16604238664.6 |
  16735456553.8 |
  16866674443.0 |
  16997892332.3 |
  17129110221.5 |
  17260328110.7 |
  17391546000.0 |
  17522763889.2 |
  17653981778.4 |
  17785199667.6 |
  17916417556.9 |
  (0 below, 1 above range)

llvm_non_incremental (n=3, range 14924846839.8-16080743437.9 ns)
  14924846839.8 |########################################
  14982641669.7 |
  15040436499.6 |
  15098231329.5 |
  15156026159.4 |
  15213820989.3 |
  15271615819.2 |
  15329410649.1 |
  15387205479.0 |
  15445000308.9 |
  15502795138.8 |
  15560589968.7 |
  15618384798.6 |
  15676179628.5 |
  15733974458.4 |
  15791769288.4 |
  15849564118.3 |
  15907358948.2 |
  15965153778.1 |########################################
  16022948608.0 |
  (0 below, 1 above range)

```
