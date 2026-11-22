# Overflow strategy: wrapping_add vs saturating_add vs checked_add

3 variants, 160 samples per variant.
Baseline: **sat_checked**

## Highlights

Baseline for all deltas below: **sat_checked**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sat_wrap dominates: 70% faster than the next best (sat_checked)

sat_wrap (103 ns) leads sat_checked (175 ns) by 70%, a clear separation rather than a photo finish. CV 4.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sat_wrap beats baseline by 41% (significant)

sat_wrap is -72 ns (41%) faster than baseline sat_checked, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sat_saturate shows warm-up / thermal drift (autocorr +0.57)

sat_saturate's per-pass series has lag-1 autocorrelation +0.57, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sat_wrap** at 103.3 ns median (-41.0% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.70x (fastest 103.3 ns, slowest 175.8 ns)

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
| sat_checked | 2306ns | 2304ns | 2240ns | 2296ns | 2403ns | base |
| sat_saturate | 2310ns | 2300ns | 2242ns | 2302ns | 2404ns | +0.18% |
| sat_wrap | 2224ns | 2211ns | 2172ns | 2216ns | 2303ns | -3.55% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sat_checked | 176ns | 168ns | 184ns | base | 1.456 |
| sat_saturate | 177ns | 168ns | 189ns | +0.76% | 1.445 |
| sat_wrap | 103ns | 97ns | 109ns | -41.27% | 2.478 |

## Performance model

- Peak throughput: **2.633 Gops/s** (sat_wrap; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sat_checked | 1.461 | 55.5% |
| sat_saturate | 1.456 | 55.3% |
| sat_wrap | 2.478 | 94.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sat_checked | 2306ns | 2306ns | base |
| sat_saturate | 2310ns | 2310ns | +0.18% |
| sat_wrap | 2224ns | 2224ns | -3.55% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sat_checked | 175ns | base | --- | [174, 177] | --- | --- | --- | --- |
| sat_saturate | 176ns | no significant difference | [-1, +2]ns | [175, 177] | no | 0.6909 | 0.6909 | 2 |
| sat_wrap | 103ns | -71.7ns (-40.9%) | [-73, -71]ns | [102, 104] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sat_checked | sat_saturate | sat_wrap |
|---|---|---|---|---|
| 1 | 1 | 181ns | +0.6% | -43.3% |
| 1 | 2 | 174ns | +4.4% | -41.3% |
| 1 | 3 | 172ns | +6.8% | -39.6% |
| 1 | 4 | 184ns | -4.9% | -45.1% |
| 1 | 5 | 174ns | -0.5% | -40.9% |
| 1 | 6 | 179ns | -3.1% | -40.1% |
| 1 | 7 | 173ns | +1.0% | -40.4% |
| 1 | 8 | 175ns | -1.2% | -38.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sat_checked | -0.506 | HIGH- (thermal bounce) |
| sat_saturate | 0.573 | HIGH+ (drift/warm-up) |
| sat_wrap | -0.126 | ok |

**Consistency summary:**

- **sat_saturate**: won 4/8, lost 4/8
- **sat_wrap**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sat_checked | 2.7ns | 175.9ns | 1.5% |  |
| sat_saturate | 2.5ns | 177.2ns | 1.4% |  |
| sat_wrap | 2.1ns | 103.3ns | 2.0% |  |

## Distribution (algo ns)

```
sat_checked (n=160, range 168.4-183.8 ns)
    168.4 |###############
    169.2 |######
    169.9 |##################
    170.7 |###############
    171.5 |#####################
    172.2 |####################################
    173.0 |############
    173.8 |########################################
    174.6 |#################################
    175.3 |##################
    176.1 |#################################
    176.9 |##################
    177.7 |##################
    178.4 |##############################
    179.2 |######
    180.0 |######
    180.8 |##############################
    181.5 |########################
    182.3 |###############
    183.1 |###
  (15 below, 13 above range)

sat_saturate (n=160, range 168.5-188.7 ns)
    168.5 |########
    169.5 |############################
    170.5 |###########
    171.5 |######################
    172.5 |##################################
    173.5 |######################
    174.5 |########################################
    175.6 |#########################
    176.6 |########################################
    177.6 |##############
    178.6 |###########
    179.6 |#########################
    180.6 |############################
    181.6 |####################
    182.6 |########
    183.6 |##############
    184.6 |########
    185.6 |###########
    186.7 |#################
    187.7 |#####
  (16 below, 4 above range)

sat_wrap (n=160, range 97.2-109.2 ns)
     97.2 |#############
     97.8 |#############
     98.4 |################
     99.0 |########################
     99.6 |##########
    100.2 |##########################
    100.8 |##
    101.4 |#############
    102.0 |########################################
    102.6 |#############
    103.2 |########################
    103.8 |#############
    104.4 |################################
    105.0 |########################
    105.6 |#############
    106.2 |##########
    106.8 |##########################
    107.4 |#####################
    108.0 |#####
    108.6 |################
  (13 below, 12 above range)

```

## Diagnostics

- **sat_saturate**: autocorrelation=0.57 (measurement drift or warm-up artifact)
