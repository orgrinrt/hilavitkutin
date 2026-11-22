# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (sat_checked) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline sat_checked has the worst median (177 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest sat_wrap at 104 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### sat_wrap dominates: 69% faster than the next best (sat_saturate)

sat_wrap (104 ns) leads sat_saturate (176 ns) by 69%, a clear separation rather than a photo finish. CV 3.5%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 41% (significant)

sat_wrap is -72 ns (41%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_checked shows alternating (throttle bounce) (autocorr -0.56)

sat_checked's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sat_wrap** at 104.2 ns median (-41.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.70x (fastest 104.2 ns, slowest 176.7 ns)

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
| sat_checked | 2298ns | 2288ns | 2283ns | 2290ns | 2335ns | base |
| sat_saturate | 2296ns | 2290ns | 2284ns | 2291ns | 2323ns | -0.08% |
| sat_wrap | 2223ns | 2218ns | 2212ns | 2218ns | 2251ns | -3.24% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 176ns | 170ns | 181ns | base | 1.454 |
| sat_saturate | 176ns | 170ns | 181ns | -0.17% | 1.457 |
| sat_wrap | 104ns | 99ns | 109ns | -41.02% | 2.466 |

## Performance model

- Peak throughput: **2.598 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.449 | 55.8% |
| sat_saturate | 1.453 | 55.9% |
| sat_wrap | 2.457 | 94.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2298ns | 2298ns | base |
| sat_saturate | 2296ns | 2296ns | -0.08% |
| sat_wrap | 2223ns | 2223ns | -3.24% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 177ns | base | --- | [176, 178] | --- | --- | --- | --- |
| sat_saturate | 176ns | no significant difference | [-1, +0]ns | [176, 177] | no | 0.0903 | 0.0903 | 6 |
| sat_wrap | 104ns | -72.5ns (-41.0%) | [-73, -72]ns | [103, 105] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 176ns | -0.1% | -40.7% |
| 1 | 2 | 177ns | +0.0% | -40.9% |
| 1 | 3 | 176ns | +0.1% | -40.4% |
| 1 | 4 | 177ns | -0.6% | -41.0% |
| 1 | 5 | 176ns | +0.0% | -40.9% |
| 1 | 6 | 177ns | -0.3% | -41.4% |
| 1 | 7 | 176ns | -0.1% | -40.0% |
| 1 | 8 | 176ns | +0.1% | -41.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.560 | HIGH- (thermal bounce) |
| sat_saturate | -0.305 | moderate- |
| sat_wrap | -0.546 | HIGH- (thermal bounce) |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 2/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.7ns | 176.0ns | 1.5% |  |
| sat_saturate | 2.6ns | 175.7ns | 1.5% |  |
| sat_wrap | 2.0ns | 103.8ns | 1.9% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 170.3-181.2 ns)
    170.3 |##########
    170.8 |########
    171.4 |##########
    171.9 |##################
    172.4 |########################
    173.0 |################
    173.5 |#####
    174.1 |##########################
    174.6 |##########
    175.2 |#####
    175.7 |#############################
    176.3 |##################
    176.8 |################
    177.3 |#############################
    177.9 |########################################
    178.4 |################
    179.0 |########
    179.5 |##########################
    180.1 |################
    180.6 |##################
  (15 below, 12 above range)

sat_saturate (n=160, range 170.1-180.6 ns)
    170.1 |###########
    170.6 |######
    171.1 |#############
    171.7 |####################
    172.2 |##
    172.7 |########
    173.2 |#############
    173.8 |#################
    174.3 |###########
    174.8 |########
    175.3 |#################
    175.9 |####################
    176.4 |#################
    176.9 |###############
    177.4 |########################################
    178.0 |#############
    178.5 |####################
    179.0 |###############
    179.5 |###########
    180.0 |########
  (14 below, 14 above range)

sat_wrap (n=160, range 98.5-108.5 ns)
     98.5 |############
     99.0 |########################
     99.5 |########################
    100.0 |###########################
    100.5 |#########
    101.0 |############
    101.5 |#########
    102.0 |#####################
    102.5 |#####################
    103.0 |###############
    103.5 |###############
    104.0 |###############
    104.5 |########################
    105.0 |##################
    105.5 |########################################
    106.0 |###########################
    106.5 |#####################
    107.0 |########################################
    107.5 |############
    108.0 |#####################
  (13 below, 12 above range)

```
