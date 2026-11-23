# Load alignment strategy: aligned ptr cast vs read_unaligned vs from_le_bytes pack

3 variants, 160 samples per variant.
Baseline: **load_aligned**

## Highlights

Baseline for all deltas below: **load_aligned**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (load_aligned) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline load_aligned has the worst median (2.29 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest load_unaligned at 2.29 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.0% (fastest 2.29 us to slowest 2.29 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### load_unaligned is fastest but the noisiest (CV 19.3%)

load_unaligned wins on median (2.29 us) yet has the highest variance (CV 19.3%), while load_aligned is the steadiest (CV 17.1%, 2.29 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (0 ns) is smaller than the fastest variant's own run-to-run std-dev (441 ns); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### load_unaligned's comparison is tie-heavy (11% tied pairs)

11% of paired samples for load_unaligned are exact ties vs baseline, weakening the sign test - the timer resolution may be coarser than the effect.

_Why it matters:_ A high tie rate means the difference is at or below measurement resolution; trust it less and consider a heavier workload per call.

### Speed leader load_unaligned vs stability leader load_aligned (+0% speed for 1.1x steadier)

load_unaligned is fastest (2.29 us, CV 19.3%); load_aligned gives up 0.0% median for 1.1x lower variance (CV 17.1%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: load_unaligned** at 2287.1 ns median (-0.0% vs baseline)
- Spread: 1.00x (fastest 2287.1 ns, slowest 2287.3 ns)

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
| Counter frequency | 2112580123 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| load_aligned | 3972ns | 4164ns | 3407ns | 3972ns | 4538ns | base |
| load_byte_pack | 3911ns | 4161ns | 3407ns | 3825ns | 4671ns | -1.56% |
| load_unaligned | 3929ns | 4163ns | 3407ns | 3825ns | 4762ns | -1.10% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| load_aligned | 2178ns | 1871ns | 2477ns | base | 7.523 |
| load_byte_pack | 2144ns | 1871ns | 2550ns | -1.56% | 7.642 |
| load_unaligned | 2153ns | 1871ns | 2602ns | -1.12% | 7.608 |

## Performance model

- Peak throughput: **8.755 Gops/s** (load_unaligned; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| load_aligned | 7.163 | 81.8% |
| load_byte_pack | 7.163 | 81.8% |
| load_unaligned | 7.164 | 81.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| load_aligned | 3972ns | 3972ns | base |
| load_byte_pack | 3911ns | 3911ns | -1.56% |
| load_unaligned | 3929ns | 3929ns | -1.10% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| load_aligned | 2287ns | base | --- | [2287, 2287] | --- | --- | --- | --- |
| load_byte_pack | 2287ns | no significant difference | [-9, +0]ns | [1883, 2287] | no | 0.2015 | 0.2015 | 3 |
| load_unaligned | 2287ns | no significant difference | [-0, +0]ns | [1881, 2287] | no | 0.2015 | 0.1534 | **18** (11%, HIGH) |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | load_aligned | load_byte_pack | load_unaligned |
|---|---|---|---|---|
| 1 | 1 | 2288ns | -18.2% | +0.0% |
| 1 | 2 | 2287ns | -18.2% | +0.0% |
| 1 | 3 | 2287ns | +0.0% | +0.0% |
| 1 | 4 | 2288ns | -18.2% | -18.2% |
| 1 | 5 | 1872ns | +22.2% | -0.0% |
| 1 | 6 | 1872ns | +0.0% | -0.0% |
| 1 | 7 | 2288ns | -0.0% | -0.0% |
| 1 | 8 | 1872ns | +22.2% | -0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| load_aligned | 0.058 | ok |
| load_byte_pack | -0.375 | moderate- |
| load_unaligned | 0.125 | ok |

**Consistency summary:**

- **load_byte_pack**: won 3/8, lost 2/8
- **load_unaligned**: won 1/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| load_aligned | 22.7ns | 2177.9ns | 1.0% |  |
| load_byte_pack | 22.4ns | 2143.9ns | 1.0% |  |
| load_unaligned | 22.4ns | 2153.5ns | 1.0% |  |

## Distribution (algo ns)

```
load_aligned (n=160, range 1871.5-2476.8 ns)
   1871.5 |#######################
   1901.7 |
   1932.0 |
   1962.3 |
   1992.5 |
   2022.8 |
   2053.1 |
   2083.3 |
   2113.6 |
   2143.8 |
   2174.1 |
   2204.4 |
   2234.6 |
   2264.9 |########################################
   2295.2 |###########
   2325.4 |
   2355.7 |
   2386.0 |
   2416.2 |
   2446.5 |
  (12 below, 2 above range)

load_byte_pack (n=160, range 1871.5-2550.4 ns)
   1871.5 |################################
   1905.4 |
   1939.3 |
   1973.3 |
   2007.2 |
   2041.2 |
   2075.1 |
   2109.1 |
   2143.0 |
   2177.0 |
   2210.9 |
   2244.9 |
   2278.8 |########################################
   2312.7 |
   2346.7 |
   2380.6 |
   2414.6 |
   2448.5 |
   2482.5 |
   2516.4 |
  (11 below, 5 above range)

load_unaligned (n=160, range 1871.4-2601.9 ns)
   1871.4 |#############################
   1908.0 |
   1944.5 |
   1981.0 |
   2017.5 |
   2054.1 |
   2090.6 |
   2127.1 |
   2163.6 |
   2200.1 |
   2236.7 |
   2273.2 |########################################
   2309.7 |
   2346.2 |
   2382.7 |
   2419.3 |
   2455.8 |
   2492.3 |
   2528.8 |
   2565.4 |
  (17 below, 5 above range)

```

## Diagnostics

- **load_unaligned**: CV=20.5% (high variance, measurements may be unstable)
