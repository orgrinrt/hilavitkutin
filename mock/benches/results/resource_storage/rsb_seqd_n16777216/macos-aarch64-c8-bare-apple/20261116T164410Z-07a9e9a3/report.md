# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### seq_live dominates: 152% faster than the next best (seq_snapshot)

seq_live (1.47 ms) leads seq_snapshot (3.70 ms) by 152%, a clear separation rather than a photo finish. CV 15.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### No variant beats the baseline (seq_live)

The baseline seq_live is the fastest (1.47 ms median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (seq_live) is the fastest** at 1469627.2 ns median
- 1 variant significantly slower than baseline
- Spread: 2.52x (fastest 1469627.2 ns, slowest 3700629.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 1647816ns | 1471265ns | 1466421ns | 1602818ns | 1964204ns | base |
| seq_snapshot | 4232659ns | 3701104ns | 3686466ns | 4123883ns | 5105183ns | +156.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 1646010ns | 1464758ns | 1962153ns | base | 10.193 |
| seq_snapshot | 4231624ns | 3685949ns | 5103327ns | +157.08% | 3.965 |

## Performance model

- Peak throughput: **11.454 Gops/s** (seq_live; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 11.416 | 99.7% |
| seq_snapshot | 4.534 | 39.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 1647816ns | 1647816ns | base |
| seq_snapshot | 4232659ns | 4232659ns | +156.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 1469627ns | base | --- | [1468320, 1472267] | --- | --- | --- | --- |
| seq_snapshot | 3700630ns | +2233250.0ns (+152.0%) | [+2230164, +2394872]ns | [3697754, 4133770] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 1919728ns | +163.6% |
| 1 | 2 | 1924026ns | +159.0% |
| 1 | 3 | 1465601ns | +151.9% |
| 1 | 4 | 1466252ns | +151.5% |
| 1 | 5 | 1467812ns | +151.9% |
| 2 | 1 | 1467788ns | +152.0% |
| 2 | 2 | 1466277ns | +151.8% |
| 2 | 3 | 1468679ns | +151.4% |
| 2 | 4 | 1941976ns | +158.2% |
| 2 | 5 | 1917799ns | +163.1% |
| 3 | 1 | 1468441ns | +151.9% |
| 3 | 2 | 1912229ns | +161.3% |
| 3 | 3 | 1919608ns | +162.4% |
| 3 | 4 | 1470167ns | +151.7% |
| 3 | 5 | 1465906ns | +151.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | 0.235 | moderate+ |
| seq_snapshot | 0.238 | moderate+ |

**Consistency summary:**

- **seq_snapshot**: won 0/15, lost 15/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 27876954.0ns | 1646010.0ns | 1693.6% | HIGH |
| seq_snapshot | 27896177.3ns | 4231623.7ns | 659.2% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 1464758.3-1962152.8 ns)
  1464758.3 |########################################
  1489628.1 |
  1514497.8 |
  1539367.5 |
  1564237.2 |
  1589106.9 |
  1613976.7 |
  1638846.4 |
  1663716.1 |
  1688585.8 |
  1713455.6 |
  1738325.3 |
  1763195.0 |
  1788064.7 |
  1812934.4 |
  1837804.2 |
  1862673.9 |#
  1887543.6 |######
  1912413.3 |#########
  1937283.0 |##
  (8 below, 12 above range)

seq_snapshot (n=120, range 3685949.2-5103326.9 ns)
  3685949.2 |########################################
  3756818.1 |
  3827687.0 |
  3898555.9 |
  3969424.7 |
  4040293.6 |
  4111162.5 |#
  4182031.4 |
  4252900.3 |
  4323769.2 |
  4394638.1 |
  4465507.0 |
  4536375.8 |
  4607244.7 |
  4678113.6 |
  4748982.5 |
  4819851.4 |
  4890720.3 |####
  4961589.2 |##########
  5032458.1 |##########
  (11 below, 8 above range)

```

## Diagnostics

- **seq_live**: bridge=1892.3% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=751.7% of algo (FFI overhead may distort results)
