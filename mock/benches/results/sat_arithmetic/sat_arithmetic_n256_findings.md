# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (184 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 107 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 68% faster than the next best (sat_saturate)

sat_wrap (107 ns) leads sat_saturate (180 ns) by 68%, a clear separation rather than a photo finish. CV 8.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 42% (significant)

sat_wrap is -78 ns (42%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_saturate's edge over baseline is significant but tiny (-1 ns, 0.68%)

sat_saturate differs from baseline sat_checked by -1 ns (0.68%) - statistically real (CI excludes zero) but small enough to be practically irrelevant.

_Why it matters:_ Statistical significance is not practical significance: a measurable-but-tiny gap should not drive a decision.

## Key findings

- **Fastest: sat_wrap** at 106.9 ns median (-42.0% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.72x (fastest 106.9 ns, slowest 184.2 ns)

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
| sat_checked | 2422ns | 2396ns | 2245ns | 2425ns | 2589ns | base |
| sat_saturate | 2377ns | 2386ns | 2232ns | 2355ns | 2589ns | -1.85% |
| sat_wrap | 2292ns | 2314ns | 2162ns | 2269ns | 2494ns | -5.35% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 187ns | 171ns | 204ns | base | 1.372 |
| sat_saturate | 184ns | 169ns | 210ns | -1.17% | 1.388 |
| sat_wrap | 108ns | 99ns | 121ns | -41.93% | 2.362 |

## Performance model

- Peak throughput: **2.586 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.390 | 53.7% |
| sat_saturate | 1.425 | 55.1% |
| sat_wrap | 2.395 | 92.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2422ns | 2422ns | base |
| sat_saturate | 2377ns | 2377ns | -1.85% |
| sat_wrap | 2292ns | 2292ns | -5.35% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 184ns | base | --- | [182, 188] | --- | --- | --- | --- |
| sat_saturate | 180ns | -1.2ns (-0.7%) | [-2, -0]ns | [177, 182] | YES | 0.0117 | 0.0117 | 8 |
| sat_wrap | 107ns | -77.5ns (-42.1%) | [-81, -75]ns | [106, 108] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 173ns | -0.2% | -35.1% |
| 1 | 2 | 199ns | -2.9% | -48.1% |
| 1 | 3 | 189ns | -7.3% | -42.0% |
| 1 | 4 | 196ns | -11.5% | -45.5% |
| 1 | 5 | 190ns | -0.0% | -37.5% |
| 1 | 6 | 180ns | +8.7% | -41.7% |
| 1 | 7 | 181ns | -3.7% | -42.6% |
| 1 | 8 | 179ns | +5.3% | -37.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.009 | ok |
| sat_saturate | -0.355 | moderate- |
| sat_wrap | -0.416 | moderate- |

**Consistency summary:**

- **sat_saturate**: won 5/8, lost 2/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.7ns | 186.6ns | 1.4% |  |
| sat_saturate | 2.5ns | 184.4ns | 1.4% |  |
| sat_wrap | 2.2ns | 108.4ns | 2.0% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 171.4-203.9 ns)
    171.4 |#####
    173.1 |########################################
    174.7 |#############
    176.3 |##################
    177.9 |##########################
    179.5 |#####################
    181.2 |#############################
    182.8 |########################
    184.4 |##################
    186.0 |########
    187.7 |########
    189.3 |################
    190.9 |################
    192.5 |#####
    194.2 |##########
    195.8 |#############
    197.4 |##################
    199.0 |########################
    200.7 |##########################
    202.3 |##########
  (15 below, 12 above range)

sat_saturate (n=160, range 169.1-209.9 ns)
    169.1 |#######################
    171.1 |#########################
    173.2 |########################################
    175.2 |###############################
    177.3 |################
    179.3 |###########################
    181.3 |##############
    183.4 |############
    185.4 |########
    187.5 |############
    189.5 |########
    191.5 |######
    193.6 |############
    195.6 |########
    197.7 |########
    199.7 |################
    201.8 |##############
    203.8 |############
    205.8 |####
    207.9 |
  (13 below, 2 above range)

sat_wrap (n=160, range 99.0-120.9 ns)
     99.0 |##################
    100.1 |######
    101.2 |########################
    102.3 |########################
    103.4 |#################################
    104.5 |####################################
    105.6 |########################################
    106.7 |#################################
    107.8 |#################################
    108.9 |##################
    109.9 |#####################
    111.0 |###########################
    112.1 |##################
    113.2 |######
    114.3 |###############
    115.4 |###############
    116.5 |#########
    117.6 |
    118.7 |###
    119.8 |##################
  (17 below, 11 above range)

```
