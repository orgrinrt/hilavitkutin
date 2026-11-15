# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 102% faster than the next best (sat_checked)

sat_wrap (5.14 us) leads sat_checked (10.39 us) by 102%, a clear separation rather than a photo finish. CV 2.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 50% (significant)

sat_wrap is -5.17 us (50%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_saturate is an outlier: 2.0x slower than the field

sat_saturate (10.45 us) is 2.0x the fastest (5.14 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sat_saturate's edge over baseline is significant but tiny (22 ns, 0.21%)

sat_saturate differs from baseline sat_checked by 22 ns (0.21%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 5144.6 ns median (-50.5% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.03x (fastest 5144.6 ns, slowest 10451.2 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sat_checked | 12578ns | 12511ns | 12319ns | 12497ns | 13079ns | base |
| sat_saturate | 12661ns | 12572ns | 12316ns | 12591ns | 13217ns | +0.67% |
| sat_wrap | 7336ns | 7228ns | 7211ns | 7269ns | 7663ns | -41.67% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 10454ns | 10239ns | 10870ns | base | 1.567 |
| sat_saturate | 10529ns | 10239ns | 10991ns | +0.71% | 1.556 |
| sat_wrap | 5226ns | 5136ns | 5465ns | -50.01% | 3.135 |

## Performance model

- Peak throughput: **3.190 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.577 | 49.4% |
| sat_saturate | 1.568 | 49.1% |
| sat_wrap | 3.185 | 99.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 12578ns | 12578ns | base |
| sat_saturate | 12661ns | 12661ns | +0.67% |
| sat_wrap | 7336ns | 7336ns | -41.67% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 10392ns | base | --- | [10300, 10437] | --- | --- | --- | --- |
| sat_saturate | 10451ns | +22.0ns (+0.2%) | [+2, +84]ns | [10433, 10525] | YES | 0.0261 | 0.0261 | 1 |
| sat_wrap | 5145ns | -5168.3ns (-49.7%) | [-5279, -5134]ns | [5143, 5159] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 10245ns | +6.3% | -49.8% |
| 1 | 2 | 10600ns | +0.5% | -49.8% |
| 1 | 3 | 10244ns | +1.9% | -48.9% |
| 1 | 4 | 10271ns | -0.3% | -49.9% |
| 1 | 5 | 10491ns | +0.1% | -51.0% |
| 1 | 6 | 10574ns | -1.1% | -51.4% |
| 1 | 7 | 10545ns | -2.7% | -51.3% |
| 1 | 8 | 10254ns | +3.0% | -46.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.188 | ok |
| sat_saturate | 0.191 | ok |
| sat_wrap | -0.098 | ok |

**Consistency summary:**

- **sat_saturate**: won 3/8, lost 5/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.9ns | 10454.3ns | 0.0% |  |
| sat_saturate | 2.8ns | 10528.5ns | 0.0% |  |
| sat_wrap | 2.7ns | 5226.1ns | 0.1% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 10238.8-10869.7 ns)
  10238.8 |########################################
  10270.3 |#############
  10301.9 |######
  10333.4 |
  10365.0 |###
  10396.5 |#####
  10428.1 |#############
  10459.6 |####
  10491.2 |####
  10522.7 |#####
  10554.3 |#
  10585.8 |###########
  10617.3 |#
  10648.9 |#
  10680.4 |#
  10712.0 |
  10743.5 |
  10775.1 |
  10806.6 |
  10838.2 |
  (14 below, 19 above range)

sat_saturate (n=160, range 10239.2-10990.6 ns)
  10239.2 |########################################
  10276.8 |################
  10314.4 |##########
  10351.9 |###
  10389.5 |#
  10427.1 |##############################
  10464.6 |#######
  10502.2 |#########
  10539.8 |######
  10577.3 |####################
  10614.9 |#######
  10652.5 |####
  10690.0 |###
  10727.6 |####
  10765.2 |
  10802.7 |####
  10840.3 |
  10877.9 |
  10915.5 |#
  10953.0 |#################################
  (17 below, 9 above range)

sat_wrap (n=160, range 5135.6-5464.7 ns)
   5135.6 |########################################
   5152.1 |#####
   5168.5 |#
   5185.0 |#
   5201.4 |#
   5217.9 |###
   5234.4 |#####
   5250.8 |#
   5267.3 |#
   5283.7 |
   5300.2 |###
   5316.6 |##
   5333.1 |
   5349.6 |
   5366.0 |#
   5382.5 |
   5398.9 |
   5415.4 |
   5431.8 |
   5448.3 |
  (18 below, 21 above range)

```
