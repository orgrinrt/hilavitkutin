# build-static: every crate built into an empty target directory

2 variants, 6 samples per variant.
Baseline: **llvm_non_incremental**

## Highlights

Baseline for all deltas below: **llvm_non_incremental**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### llvm_incremental shows alternating (throttle bounce) (autocorr -0.82)

llvm_incremental's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (llvm_non_incremental)

The baseline llvm_non_incremental is the fastest (117220.40 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (llvm_non_incremental) is the fastest** at 117220401777.6 ns median
- 1 variant significantly slower than baseline
- Spread: 1.08x (fastest 117220401777.6 ns, slowest 126373237845.4 ns)

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
| Counter frequency | 2100246914 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| llvm_incremental | 125754169806ns | 126373242053ns | 119471424815ns | 124238460857ns | 131169105725ns | +7.85% |
| llvm_non_incremental | 116597542183ns | 117220406668ns | 111733918574ns | 116527240380ns | 119134806690ns | base |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| llvm_incremental | 125754165177ns | 119471419764ns | 131169100796ns | +7.85% |
| llvm_non_incremental | 116597537444ns | 111733913453ns | 119134802055ns | base |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| llvm_incremental | 125754169806ns | 125754169806ns | +7.85% |
| llvm_non_incremental | 116597542183ns | 116597542183ns | base |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| llvm_non_incremental | 117220401778ns | base | --- | [113437408500, 119134802055] | --- | --- | --- | --- |
| llvm_incremental | 126373237845ns | +10670124964.5ns (+9.1%) | [+2473582914, +14326175320]ns | [119720156889, 131169100796] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | llvm_non_incremental | llvm_incremental |
|---|---|---|---|
| 1 | 1 | 118468111668ns | +4.0% |
| 1 | 2 | 118544947406ns | +11.6% |
| 1 | 3 | 119724656704ns | +0.2% |
| 1 | 4 | 115972691887ns | +11.7% |
| 1 | 5 | 111733913453ns | +6.9% |
| 1 | 6 | 115140903546ns | +13.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| llvm_incremental | -0.823 | HIGH- (thermal bounce) |
| llvm_non_incremental | 0.414 | moderate+ |

**Consistency summary:**

- **llvm_incremental**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| llvm_incremental | 1569592139.8ns | 125754165176.8ns | 1.2% |  |
| llvm_non_incremental | 391482103.1ns | 116597537444.1ns | 0.3% |  |

## Distribution (algo ns)

```
llvm_incremental (n=6, range 119471419764.0-131169100795.6 ns)
  119471419764.0 |########################################
  120056303815.6 |
  120641187867.2 |
  121226071918.7 |
  121810955970.3 |
  122395840021.9 |
  122980724073.5 |####################
  123565608125.1 |
  124150492176.6 |
  124735376228.2 |
  125320260279.8 |
  125905144331.4 |
  126490028383.0 |
  127074912434.5 |
  127659796486.1 |
  128244680537.7 |
  128829564589.3 |
  129414448640.9 |####################
  129999332692.4 |####################
  130584216744.0 |
  (0 below, 1 above range)

llvm_non_incremental (n=6, range 111733913453.0-119134802055.1 ns)
  111733913453.0 |####################
  112103957883.1 |
  112474002313.2 |
  112844046743.3 |
  113214091173.4 |
  113584135603.5 |
  113954180033.6 |
  114324224463.7 |
  114694268893.8 |
  115064313323.9 |####################
  115434357754.0 |
  115804402184.1 |####################
  116174446614.2 |
  116544491044.3 |
  116914535474.4 |
  117284579904.5 |
  117654624334.6 |
  118024668764.7 |
  118394713194.8 |########################################
  118764757624.9 |
  (0 below, 1 above range)

```
