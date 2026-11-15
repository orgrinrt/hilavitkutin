# Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst

3 variants, 160 samples per variant.
Baseline: **acc_atomic_relaxed**

## Highlights

Baseline for all deltas below: **acc_atomic_relaxed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (acc_atomic_relaxed) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline acc_atomic_relaxed has the worst median (18.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest acc_plain at 18.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### acc_plain is fastest but the noisiest (CV 8.8%)

acc_plain wins on median (18.27 us) yet has the highest variance (CV 8.8%), while acc_atomic_relaxed is the steadiest (CV 0.3%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (2 ns) is smaller than the fastest variant's own run-to-run std-dev (1.61 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader acc_plain vs stability leader acc_atomic_relaxed (+0% speed for 28.6x steadier)

acc_plain is fastest (18.27 us, CV 8.8%); acc_atomic_relaxed gives up 0.0% median for 28.6x lower variance (CV 0.3%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 18.27 us and 18.27 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### acc_plain's edge over baseline is significant but tiny (-4 ns, 0.02%)

acc_plain differs from baseline acc_atomic_relaxed by -4 ns (0.02%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: acc_plain** at 18266.3 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 18266.3 ns, slowest 18268.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Accumulator typing cost: plain u64 vs AtomicU64 Relaxed vs AtomicU64 SeqCst |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4823d76519bef0a3 |
| Counter frequency | 2112779839 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 20160ns | 20148ns | 20143ns | 20149ns | 20211ns | base |
| acc_atomic_seqcst | 19699ns | 20146ns | 17860ns | 20147ns | 20196ns | -2.29% |
| acc_plain | 18781ns | 20147ns | 16480ns | 19084ns | 20171ns | -6.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| acc_atomic_relaxed | 18272ns | 18257ns | 18301ns | base | 0.897 |
| acc_atomic_seqcst | 17855ns | 16187ns | 18290ns | -2.28% | 0.918 |
| acc_plain | 17020ns | 14939ns | 18271ns | -6.85% | 0.963 |

## Performance model

- Peak throughput: **1.097 Gops/s** (acc_plain; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| acc_atomic_relaxed | 0.897 | 81.8% |
| acc_atomic_seqcst | 0.897 | 81.8% |
| acc_plain | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| acc_atomic_relaxed | 20160ns | 20160ns | base |
| acc_atomic_seqcst | 19699ns | 19699ns | -2.29% |
| acc_plain | 18781ns | 18781ns | -6.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| acc_atomic_relaxed | 18268ns | base | --- | [18268, 18268] | --- | --- | --- | --- |
| acc_atomic_seqcst | 18268ns | no significant difference | [-1, +0]ns | [18267, 18268] | no | 0.0924 | 0.0924 | 4 |
| acc_plain | 18266ns | -3.7ns (-0.0%) | [-12, -1]ns | [18257, 18267] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | acc_atomic_relaxed | acc_atomic_seqcst | acc_plain |
|---|---|---|---|---|
| 1 | 1 | 18267ns | +0.0% | -18.2% |
| 1 | 2 | 18268ns | -0.0% | -18.2% |
| 1 | 3 | 18269ns | -0.0% | -0.0% |
| 1 | 4 | 18268ns | -0.0% | +0.0% |
| 1 | 5 | 18268ns | +0.0% | -18.2% |
| 1 | 6 | 18268ns | +0.0% | +0.0% |
| 1 | 7 | 18269ns | -0.0% | -0.0% |
| 1 | 8 | 18261ns | -18.2% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| acc_atomic_relaxed | -0.158 | ok |
| acc_atomic_seqcst | -0.018 | ok |
| acc_plain | 0.058 | ok |

**Consistency summary:**

- **acc_atomic_seqcst**: won 1/8, lost 0/8
- **acc_plain**: won 3/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| acc_atomic_relaxed | 23.8ns | 18272.2ns | 0.1% |  |
| acc_atomic_seqcst | 23.3ns | 17855.1ns | 0.1% |  |
| acc_plain | 22.3ns | 17020.5ns | 0.1% |  |

## Distribution (algo ns)

```
acc_atomic_relaxed (n=160, range 18256.6-18300.6 ns)
  18256.6 |##########
  18258.8 |
  18261.0 |
  18263.2 |
  18265.4 |######################
  18267.6 |########################################
  18269.8 |########
  18272.0 |#
  18274.2 |##
  18276.4 |#
  18278.6 |
  18280.8 |
  18283.0 |
  18285.2 |
  18287.4 |
  18289.6 |
  18291.8 |
  18294.0 |
  18296.2 |
  18298.4 |
  (15 below, 2 above range)

acc_atomic_seqcst (n=160, range 16187.5-18290.4 ns)
  16187.5 |
  16292.6 |
  16397.7 |
  16502.9 |
  16608.0 |
  16713.2 |
  16818.3 |
  16923.5 |
  17028.6 |
  17133.8 |
  17238.9 |
  17344.1 |
  17449.2 |
  17554.4 |
  17659.5 |
  17764.7 |
  17869.8 |
  17975.0 |
  18080.1 |
  18185.3 |########################################
  (20 below, 4 above range)

acc_plain (n=160, range 14939.1-18270.5 ns)
  14939.1 |###############
  15105.6 |
  15272.2 |
  15438.8 |
  15605.4 |
  15771.9 |
  15938.5 |
  16105.1 |
  16271.7 |
  16438.2 |
  16604.8 |
  16771.4 |
  16937.9 |
  17104.5 |
  17271.1 |
  17437.7 |
  17604.2 |
  17770.8 |
  17937.4 |
  18104.0 |########################################
  (24 below, 8 above range)

```
