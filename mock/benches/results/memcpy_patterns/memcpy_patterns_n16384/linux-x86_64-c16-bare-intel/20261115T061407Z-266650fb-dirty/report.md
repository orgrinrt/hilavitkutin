# Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop

3 variants, 160 samples per variant.
Baseline: **memcpy_intrinsic**

## Highlights

Baseline for all deltas below: **memcpy_intrinsic**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (memcpy_intrinsic) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline memcpy_intrinsic has the worst median (18.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest memcpy_naive at 18.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### memcpy_naive is fastest but the noisiest (CV 9.4%)

memcpy_naive wins on median (18.27 us) yet has the highest variance (CV 9.4%), while memcpy_intrinsic is the steadiest (CV 0.1%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (1.72 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader memcpy_naive vs stability leader memcpy_intrinsic (+0% speed for 74.2x steadier)

memcpy_naive is fastest (18.27 us, CV 9.4%); memcpy_intrinsic gives up 0.0% median for 74.2x lower variance (CV 0.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 18.27 us and 18.27 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### memcpy_naive's edge over baseline is significant but tiny (-2 ns, 0.01%)

memcpy_naive differs from baseline memcpy_intrinsic by -2 ns (0.01%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: memcpy_naive** at 18267.0 ns median (-0.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.00x (fastest 18267.0 ns, slowest 18268.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Memory copy patterns: intrinsic vs unrolled u64 vs naive byte loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4646464650504040 |
| Counter frequency | 2112632730 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| memcpy_intrinsic | 20159ns | 20151ns | 20146ns | 20151ns | 20193ns | base |
| memcpy_naive | 18823ns | 20148ns | 16481ns | 19086ns | 20380ns | -6.62% |
| memcpy_unrolled | 19703ns | 20151ns | 17860ns | 20152ns | 20199ns | -2.26% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| memcpy_intrinsic | 18269ns | 18257ns | 18283ns | base | 0.897 |
| memcpy_naive | 17062ns | 14936ns | 18478ns | -6.61% | 0.960 |
| memcpy_unrolled | 17856ns | 16187ns | 18291ns | -2.26% | 0.918 |

## Performance model

- Peak throughput: **1.097 Gops/s** (memcpy_naive; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| memcpy_intrinsic | 0.897 | 81.8% |
| memcpy_naive | 0.897 | 81.8% |
| memcpy_unrolled | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| memcpy_intrinsic | 20159ns | 20159ns | base |
| memcpy_naive | 18823ns | 18823ns | -6.62% |
| memcpy_unrolled | 19703ns | 19703ns | -2.26% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| memcpy_intrinsic | 18268ns | base | --- | [18268, 18268] | --- | --- | --- | --- |
| memcpy_naive | 18267ns | -1.5ns (-0.0%) | [-11, -1]ns | [18257, 18268] | YES | 0.0000 | 0.0000 | 1 |
| memcpy_unrolled | 18268ns | no significant difference | [-1, +0]ns | [18267, 18268] | no | 0.4278 | 0.4278 | 1 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | memcpy_intrinsic | memcpy_naive | memcpy_unrolled |
|---|---|---|---|---|
| 1 | 1 | 18267ns | +0.0% | +0.0% |
| 1 | 2 | 18268ns | -0.0% | -0.0% |
| 1 | 3 | 18268ns | -0.0% | -0.0% |
| 1 | 4 | 18268ns | +0.0% | -0.0% |
| 1 | 5 | 18268ns | -18.2% | +0.0% |
| 1 | 6 | 18267ns | -18.2% | -18.2% |
| 1 | 7 | 18268ns | -18.2% | +0.0% |
| 1 | 8 | 18268ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| memcpy_intrinsic | -0.332 | moderate- |
| memcpy_naive | 0.392 | moderate+ |
| memcpy_unrolled | -0.161 | ok |

**Consistency summary:**

- **memcpy_naive**: won 3/8, lost 0/8
- **memcpy_unrolled**: won 1/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| memcpy_intrinsic | 24.5ns | 18268.8ns | 0.1% |  |
| memcpy_naive | 22.9ns | 17061.7ns | 0.1% |  |
| memcpy_unrolled | 24.2ns | 17855.7ns | 0.1% |  |

## Distribution (algo ns)

```
memcpy_intrinsic (n=160, range 18256.7-18283.4 ns)
  18256.7 |#######
  18258.0 |##
  18259.4 |
  18260.7 |
  18262.0 |
  18263.4 |
  18264.7 |
  18266.0 |################
  18267.4 |########################################
  18268.7 |###############
  18270.1 |#######
  18271.4 |####
  18272.7 |
  18274.1 |
  18275.4 |
  18276.7 |#
  18278.1 |#
  18279.4 |
  18280.7 |
  18282.1 |
  (17 below, 2 above range)

memcpy_naive (n=160, range 14936.4-18478.4 ns)
  14936.4 |######################
  15113.5 |
  15290.6 |
  15467.7 |
  15644.8 |
  15821.9 |
  15999.0 |
  16176.1 |
  16353.2 |
  16530.3 |
  16707.4 |
  16884.5 |
  17061.6 |
  17238.7 |
  17415.8 |
  17592.9 |
  17770.0 |
  17947.1 |
  18124.2 |########################################
  18301.3 |
  (5 below, 1 above range)

memcpy_unrolled (n=160, range 16186.5-18290.9 ns)
  16186.5 |
  16291.7 |
  16397.0 |
  16502.2 |
  16607.4 |
  16712.6 |
  16817.8 |
  16923.1 |
  17028.3 |
  17133.5 |
  17238.7 |
  17343.9 |
  17449.2 |
  17554.4 |
  17659.6 |
  17764.8 |
  17870.1 |
  17975.3 |
  18080.5 |
  18185.7 |########################################
  (20 below, 3 above range)

```
