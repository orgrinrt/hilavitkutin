# W = 4, G = 8

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (20.41 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 18.35 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### grouped shows alternating (throttle bounce) (autocorr -0.51)

grouped's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 18347.4 ns median (-10.1% vs baseline)
- Spread: 1.11x (fastest 18347.4 ns, slowest 20413.0 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 4, G = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112787067 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 21311ns | 21068ns | 17704ns | 20102ns | 24929ns | base |
| per_core | 19761ns | 19129ns | 17320ns | 18884ns | 22298ns | -7.27% |
| sequential | 22968ns | 21025ns | 18839ns | 20868ns | 28184ns | +7.78% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 20385ns | 17077ns | 23752ns | base | 0.201 |
| per_core | 18821ns | 16658ns | 21082ns | -7.67% | 0.218 |
| sequential | 22051ns | 18365ns | 26820ns | +8.17% | 0.186 |

## Performance model

- Peak throughput: **0.246 Gops/s** (per_core; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.201 | 81.6% |
| per_core | 0.223 | 90.8% |
| sequential | 0.201 | 81.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 21311ns | 21311ns | base |
| per_core | 19761ns | 19761ns | -7.27% |
| sequential | 22968ns | 22968ns | +7.78% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 20413ns | base | --- | [17604, 22295] | --- | --- | --- | --- |
| per_core | 18347ns | no significant difference | [-3687, +330]ns | [17800, 19558] | no | 0.2379 | 0.2379 | 0 |
| sequential | 20388ns | no significant difference | [-1709, +1736]ns | [19331, 21998] | no | 0.2379 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 23448ns | -12.9% | -13.2% |
| 1 | 2 | 17230ns | +6.0% | +8.1% |
| 1 | 3 | 20413ns | -17.6% | +8.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.508 | HIGH- (thermal bounce) |
| per_core | -0.009 | ok |
| sequential | -0.488 | moderate- |

**Consistency summary:**

- **per_core**: won 2/3, lost 1/3
- **sequential**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 26321.0ns | 20385.1ns | 129.1% | HIGH |
| per_core | 20731.5ns | 18821.1ns | 110.2% | HIGH |
| sequential | 22912.2ns | 22051.3ns | 103.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 17076.8-23751.9 ns)
  17076.8 |########################################
  17410.6 |#############
  17744.3 |#############
  18078.1 |
  18411.8 |
  18745.6 |##########################
  19079.4 |
  19413.1 |
  19746.9 |#############
  20080.6 |
  20414.4 |
  20748.1 |########################################
  21081.9 |
  21415.6 |
  21749.4 |#############
  22083.1 |
  22416.9 |#############
  22750.7 |
  23084.4 |#############
  23418.2 |#############
  (1 below, 2 above range)

per_core (n=18, range 16658.5-21082.1 ns)
  16658.5 |##########################
  16879.7 |#############
  17100.9 |
  17322.0 |
  17543.2 |#############
  17764.4 |##########################
  17985.6 |#############
  18206.8 |########################################
  18428.0 |#############
  18649.1 |#############
  18870.3 |
  19091.5 |
  19312.7 |
  19533.9 |
  19755.0 |
  19976.2 |
  20197.4 |#############
  20418.6 |#############
  20639.8 |#############
  20861.0 |
  (1 below, 2 above range)

sequential (n=18, range 18365.2-26820.4 ns)
  18365.2 |########################
  18787.9 |########
  19210.7 |
  19633.5 |########
  20056.2 |########################################
  20479.0 |
  20901.7 |
  21324.5 |
  21747.3 |########################
  22170.0 |########
  22592.8 |
  23015.5 |
  23438.3 |
  23861.1 |########
  24283.8 |
  24706.6 |
  25129.4 |
  25552.1 |
  25974.9 |########
  26397.6 |
  (1 below, 1 above range)

```

## Diagnostics

- **grouped**: bridge=127.8% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.5% of algo (FFI overhead may distort results)
- **sequential**: CV=26.2% (high variance, measurements may be unstable)
- **sequential**: bridge=102.8% of algo (FFI overhead may distort results)
