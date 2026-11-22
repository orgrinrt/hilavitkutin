# Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step

3 variants, 160 samples per variant.
Baseline: **inline_always**

## Highlights

Baseline for all deltas below: **inline_always**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (inline_default, inline_always) are a dead heat (<1%)

inline_default (1.23 us) and inline_always (1.23 us) differ by 0.02%, inside the noise, even though the wider field spreads 4.0%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Whole field within 4.0% of the fastest

All 3 variants sit between 1.23 us and 1.28 us - a 4.0% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: inline_default** at 1226.7 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.04x (fastest 1226.7 ns, slowest 1275.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Inline strategy: #[inline(always)] vs default vs #[inline(never)] per-record step |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x39f847b2c651de80 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| inline_always | 3331ns | 3327ns | 3321ns | 3327ns | 3351ns | base |
| inline_default | 3330ns | 3326ns | 3322ns | 3326ns | 3348ns | -0.02% |
| inline_never | 3369ns | 3359ns | 3352ns | 3359ns | 3416ns | +1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| inline_always | 1228ns | 1223ns | 1234ns | base | 0.834 |
| inline_default | 1228ns | 1223ns | 1236ns | +0.02% | 0.834 |
| inline_never | 1279ns | 1270ns | 1299ns | +4.19% | 0.801 |

## Performance model

- Peak throughput: **0.837 Gops/s** (inline_always; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| inline_always | 0.835 | 99.7% |
| inline_default | 0.835 | 99.7% |
| inline_never | 0.803 | 95.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| inline_always | 3331ns | 3331ns | base |
| inline_default | 3330ns | 3330ns | -0.02% |
| inline_never | 3369ns | 3369ns | +1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| inline_always | 1227ns | base | --- | [1226, 1228] | --- | --- | --- | --- |
| inline_default | 1227ns | no significant difference | [-1, +0]ns | [1226, 1227] | no | 0.6812 | 0.6812 | 12 |
| inline_never | 1275ns | +48.3ns (+3.9%) | [+48, +49]ns | [1275, 1276] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | inline_always | inline_default | inline_never |
|---|---|---|---|---|
| 1 | 1 | 1227ns | -0.0% | +3.9% |
| 1 | 2 | 1227ns | +0.0% | +3.9% |
| 1 | 3 | 1226ns | +0.0% | +4.0% |
| 1 | 4 | 1227ns | -0.0% | +3.8% |
| 1 | 5 | 1227ns | -0.1% | +4.0% |
| 1 | 6 | 1228ns | -0.0% | +3.9% |
| 1 | 7 | 1227ns | +0.0% | +4.0% |
| 1 | 8 | 1226ns | +0.0% | +5.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| inline_always | -0.230 | moderate- |
| inline_default | -0.437 | moderate- |
| inline_never | 0.004 | ok |

**Consistency summary:**

- **inline_default**: won 0/8, lost 0/8
- **inline_never**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| inline_always | 2.8ns | 1227.5ns | 0.2% |  |
| inline_default | 2.8ns | 1227.8ns | 0.2% |  |
| inline_never | 3.2ns | 1279.0ns | 0.2% |  |

## Distribution (algo ns)

```
inline_always (n=160, range 1222.8-1233.6 ns)
   1222.8 |################
   1223.3 |#############
   1223.9 |################
   1224.4 |#############
   1225.0 |##################################
   1225.5 |##################################
   1226.0 |##########################
   1226.6 |########################################
   1227.1 |#############################
   1227.7 |##################
   1228.2 |#############################
   1228.7 |#############
   1229.3 |##########################
   1229.8 |#####
   1230.4 |########################
   1230.9 |########
   1231.4 |#####
   1232.0 |#####
   1232.5 |#####
   1233.1 |########
  (12 below, 8 above range)

inline_default (n=160, range 1223.2-1235.6 ns)
   1223.2 |############
   1223.8 |##################
   1224.4 |###################################
   1225.0 |#####################
   1225.6 |########################################
   1226.3 |#########################
   1226.9 |################
   1227.5 |#################################
   1228.1 |#################################
   1228.7 |########
   1229.4 |##
   1230.0 |##############
   1230.6 |######
   1231.2 |######
   1231.8 |####
   1232.5 |########
   1233.1 |
   1233.7 |
   1234.3 |
   1234.9 |######
  (12 below, 8 above range)

inline_never (n=160, range 1269.7-1298.8 ns)
   1269.7 |###############
   1271.1 |##############################
   1272.6 |#################
   1274.1 |########################################
   1275.5 |###################################
   1277.0 |#################
   1278.4 |##########
   1279.9 |#######
   1281.3 |#######
   1282.8 |####
   1284.2 |
   1285.7 |#
   1287.1 |
   1288.6 |####
   1290.0 |#
   1291.5 |##
   1292.9 |
   1294.4 |#
   1295.8 |#
   1297.3 |
  (11 below, 11 above range)

```
