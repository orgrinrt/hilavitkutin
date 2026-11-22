# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (10393.76 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 10393762129.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 10393762129.9 ns, slowest 10836289929.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | build-static: every crate built into an empty target directory |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6276626c73740001 |
| Counter frequency | 2112687708 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 10848918708ns | 10836292461ns | 10748799206ns | 10819058570ns | 10943768666ns | +4.63% |
| llvm_non_incremental | 10369296352ns | 10393764648ns | 10278382614ns | 10372022866ns | 10410663451ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 10848916295ns | 10748796278ns | 10943766610ns | +4.63% |
| llvm_non_incremental | 10369294004ns | 10278380232ns | 10410661450ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 10848918708ns | 10848918708ns | +4.63% |
| llvm_non_incremental | 10369296352ns | 10369296352ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 10393762130ns | base | --- | [10303458432, 10410661450] | --- | --- | --- | --- |
| llvm_incremental | 10836289929ns | +489552296.1ns (+4.7%) | [+405830543, +543484034]ns | [10766692346, 10943766610] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 10406376297ns | +4.9% |
| 1 | 2 | 10278380232ns | +5.0% |
| 1 | 3 | 10393186973ns | +3.8% |
| 1 | 4 | 10328536633ns | +4.1% |
| 1 | 5 | 10394337287ns | +5.5% |
| 1 | 6 | 10414946603ns | +4.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.060 | ok |
| llvm_non_incremental | -0.438 | moderate- |

**Consistency summary:**

- **llvm_incremental**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 44121297.9ns | 10848916295.4ns | 0.4% |  |
| llvm_non_incremental | 29896056.7ns | 10369294004.2ns | 0.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 10748796278.3-10943766610.5 ns)
  10748796278.3 |########################################
  10758544794.9 |
  10768293311.5 |
  10778041828.1 |########################################
  10787790344.7 |########################################
  10797538861.3 |
  10807287378.0 |
  10817035894.6 |
  10826784411.2 |
  10836532927.8 |
  10846281444.4 |
  10856029961.0 |
  10865778477.6 |
  10875526994.2 |########################################
  10885275510.8 |
  10895024027.5 |
  10904772544.1 |
  10914521060.7 |########################################
  10924269577.3 |
  10934018093.9 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 10278380232.1-10410661450.0 ns)
  10278380232.1 |####################
  10284994293.0 |
  10291608353.9 |
  10298222414.8 |
  10304836475.7 |
  10311450536.6 |
  10318064597.5 |
  10324678658.4 |####################
  10331292719.3 |
  10337906780.2 |
  10344520841.1 |
  10351134902.0 |
  10357748962.9 |
  10364363023.8 |
  10370977084.7 |
  10377591145.6 |
  10384205206.5 |
  10390819267.4 |########################################
  10397433328.3 |
  10404047389.2 |####################
  (0 below, 1 above range)

```
