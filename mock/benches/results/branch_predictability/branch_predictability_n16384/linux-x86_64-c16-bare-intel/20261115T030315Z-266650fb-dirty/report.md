# Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common

3 variants, 160 samples per variant.
Baseline: **bp_balanced**

## Highlights

Baseline for all deltas below: **bp_balanced**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (bp_balanced) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline bp_balanced has the worst median (18.27 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest bp_skew_low at 18.27 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### bp_skew_low is fastest but the noisiest (CV 7.9%)

bp_skew_low wins on median (18.27 us) yet has the highest variance (CV 7.9%), while bp_balanced is the steadiest (CV 0.0%, 18.27 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (1 ns) is smaller than the fastest variant's own run-to-run std-dev (1.44 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader bp_skew_low vs stability leader bp_balanced (+0% speed for 229.0x steadier)

bp_skew_low is fastest (18.27 us, CV 7.9%); bp_balanced gives up 0.0% median for 229.0x lower variance (CV 0.0%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

### Whole field within 0.0% of the fastest

All 3 variants sit between 18.27 us and 18.27 us - a 0.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

### bp_skew_high's edge over baseline is significant but tiny (-1 ns, 0.00%)

bp_skew_high differs from baseline bp_balanced by -1 ns (0.00%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: bp_skew_low** at 18267.4 ns median (-0.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.00x (fastest 18267.4 ns, slowest 18268.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Branch predictability: skewed-rare vs balanced 50/50 vs skewed-common |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x1b41e7c35fa28d06 |
| Counter frequency | 2112760617 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| bp_balanced | 20167ns | 20162ns | 20155ns | 20162ns | 20193ns | base |
| bp_skew_high | 19735ns | 20160ns | 17869ns | 20161ns | 20327ns | -2.14% |
| bp_skew_low | 19252ns | 20160ns | 16492ns | 19857ns | 20198ns | -4.54% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| bp_balanced | 18267ns | 18259ns | 18274ns | base | 0.897 |
| bp_skew_high | 17879ns | 16186ns | 18408ns | -2.13% | 0.916 |
| bp_skew_low | 17438ns | 14941ns | 18284ns | -4.54% | 0.940 |

## Performance model

- Peak throughput: **1.097 Gops/s** (bp_skew_low; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| bp_balanced | 0.897 | 81.8% |
| bp_skew_high | 0.897 | 81.8% |
| bp_skew_low | 0.897 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| bp_balanced | 20167ns | 20167ns | base |
| bp_skew_high | 19735ns | 19735ns | -2.14% |
| bp_skew_low | 19252ns | 19252ns | -4.54% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| bp_balanced | 18268ns | base | --- | [18268, 18268] | --- | --- | --- | --- |
| bp_skew_high | 18268ns | -0.7ns (-0.0%) | [-1, -0]ns | [18268, 18268] | YES | 0.0042 | 0.0042 | 1 |
| bp_skew_low | 18267ns | -1.4ns (-0.0%) | [-2, -1]ns | [18267, 18268] | YES | 0.0000 | 0.0000 | 2 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | bp_balanced | bp_skew_high | bp_skew_low |
|---|---|---|---|---|
| 1 | 1 | 18267ns | -18.2% | -18.2% |
| 1 | 2 | 18268ns | +0.0% | +0.0% |
| 1 | 3 | 18268ns | -0.0% | -18.2% |
| 1 | 4 | 18269ns | +0.0% | -0.0% |
| 1 | 5 | 18268ns | -0.0% | -0.0% |
| 1 | 6 | 18269ns | -0.0% | -0.0% |
| 1 | 7 | 18268ns | -0.0% | -0.0% |
| 1 | 8 | 18268ns | +0.0% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| bp_balanced | 0.264 | moderate+ |
| bp_skew_high | -0.018 | ok |
| bp_skew_low | -0.208 | moderate- |

**Consistency summary:**

- **bp_skew_high**: won 1/8, lost 0/8
- **bp_skew_low**: won 2/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| bp_balanced | 24.3ns | 18267.4ns | 0.1% |  |
| bp_skew_high | 23.7ns | 17878.5ns | 0.1% |  |
| bp_skew_low | 23.0ns | 17438.1ns | 0.1% |  |

## Distribution (algo ns)

```
bp_balanced (n=160, range 18258.7-18273.7 ns)
  18258.7 |#
  18259.5 |#
  18260.2 |
  18260.9 |#
  18261.7 |
  18262.4 |
  18263.2 |
  18263.9 |
  18264.7 |
  18265.4 |##
  18266.2 |###############
  18266.9 |#################################
  18267.7 |########################################
  18268.4 |#################################
  18269.2 |##############################
  18269.9 |####
  18270.7 |#########
  18271.4 |##
  18272.2 |#
  18272.9 |
  (24 below, 8 above range)

bp_skew_high (n=160, range 16186.2-18407.8 ns)
  16186.2 |
  16297.2 |
  16408.3 |
  16519.4 |
  16630.5 |
  16741.6 |
  16852.6 |
  16963.7 |
  17074.8 |
  17185.9 |
  17297.0 |
  17408.1 |
  17519.1 |
  17630.2 |
  17741.3 |
  17852.4 |
  17963.5 |
  18074.5 |
  18185.6 |########################################
  18296.7 |
  (20 below, 1 above range)

bp_skew_low (n=160, range 14941.5-18283.8 ns)
  14941.5 |#######
  15108.6 |
  15275.7 |
  15442.8 |
  15609.9 |
  15777.1 |
  15944.2 |
  16111.3 |
  16278.4 |
  16445.5 |
  16612.6 |
  16779.8 |
  16946.9 |
  17114.0 |
  17281.1 |
  17448.2 |
  17615.3 |
  17782.5 |
  17949.6 |
  18116.7 |########################################
  (19 below, 3 above range)

```
