# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (7.85 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 4.57 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 41% faster than the next best (sat_saturate)

sat_wrap (4.57 us) leads sat_saturate (6.43 us) by 41%, a clear separation rather than a photo finish. CV 12.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -3.29 us (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_wrap is fastest but the noisiest (CV 12.8%)

sat_wrap wins on median (4.57 us) yet has the highest variance (CV 12.8%), while sat_checked is the steadiest (CV 10.0%, 7.85 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

## Key findings

- **Fastest: sat_wrap** at 4569.6 ns median (-41.8% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.72x (fastest 4569.6 ns, slowest 7849.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Overflow strategy: wrapping_add vs saturating_add vs checked_add |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x52b9de7c318fa064 |
| Counter frequency | 2112805263 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 9372ns | 9743ns | 7965ns | 9598ns | 10100ns | base |
| sat_saturate | 8304ns | 7978ns | 7945ns | 7979ns | 9636ns | -11.39% |
| sat_wrap | 6213ns | 6460ns | 5286ns | 6366ns | 6678ns | -33.71% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 7551ns | 6418ns | 8138ns | base | 2.170 |
| sat_saturate | 6690ns | 6400ns | 7751ns | -11.40% | 2.449 |
| sat_wrap | 4404ns | 3741ns | 4769ns | -41.67% | 3.720 |

## Performance model

- Peak throughput: **4.379 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 2.087 | 47.7% |
| sat_saturate | 2.548 | 58.2% |
| sat_wrap | 3.585 | 81.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 9372ns | 9372ns | base |
| sat_saturate | 8304ns | 8304ns | -11.39% |
| sat_wrap | 6213ns | 6213ns | -33.71% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 7849ns | base | --- | [7842, 7858] | --- | --- | --- | --- |
| sat_saturate | 6431ns | -1400.3ns (-17.8%) | [-1411, -1374]ns | [6428, 6436] | YES | 0.0000 | 0.0000 | 0 |
| sat_wrap | 4570ns | -3290.6ns (-41.9%) | [-3300, -3282]ns | [4570, 4570] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 7885ns | -18.5% | -42.0% |
| 1 | 2 | 7861ns | -18.2% | -41.7% |
| 1 | 3 | 6425ns | -0.0% | -28.9% |
| 1 | 4 | 7864ns | -18.4% | -52.5% |
| 1 | 5 | 7844ns | -18.1% | -52.3% |
| 1 | 6 | 7878ns | -0.4% | -42.0% |
| 1 | 7 | 6423ns | +0.2% | -28.8% |
| 1 | 8 | 7851ns | -17.9% | -41.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.376 | moderate- |
| sat_saturate | -0.156 | ok |
| sat_wrap | 0.295 | moderate+ |

**Consistency summary:**

- **sat_saturate**: won 6/8, lost 1/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 23.1ns | 7551.2ns | 0.3% |  |
| sat_saturate | 20.4ns | 6690.4ns | 0.3% |  |
| sat_wrap | 22.9ns | 4404.4ns | 0.5% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 6418.2-8138.1 ns)
   6418.2 |############
   6504.2 |
   6590.2 |
   6676.2 |
   6762.2 |
   6848.2 |
   6934.2 |
   7020.2 |
   7106.2 |
   7192.2 |
   7278.2 |
   7364.1 |
   7450.1 |
   7536.1 |
   7622.1 |
   7708.1 |
   7794.1 |########################################
   7880.1 |#############
   7966.1 |
   8052.1 |
  (13 below, 3 above range)

sat_saturate (n=160, range 6399.6-7751.1 ns)
   6399.6 |########################################
   6467.2 |#
   6534.8 |
   6602.4 |
   6669.9 |
   6737.5 |
   6805.1 |
   6872.7 |
   6940.2 |
   7007.8 |
   7075.4 |
   7143.0 |
   7210.5 |
   7278.1 |
   7345.7 |
   7413.3 |
   7480.8 |
   7548.4 |
   7616.0 |
   7683.6 |
  (13 below, 23 above range)

sat_wrap (n=160, range 3741.2-4768.7 ns)
   3741.2 |#####
   3792.5 |
   3843.9 |
   3895.3 |
   3946.7 |
   3998.0 |
   4049.4 |
   4100.8 |
   4152.2 |
   4203.6 |
   4254.9 |
   4306.3 |
   4357.7 |
   4409.1 |
   4460.5 |
   4511.8 |
   4563.2 |########################################
   4614.6 |
   4666.0 |
   4717.3 |
  (25 below, 2 above range)

```
