# Fn-pointer table vs match dispatch for 16-variant hot loop

2 variants, 160 samples per variant.
Baseline: **fpt_match_16**

## Highlights

Baseline for all deltas below: **fpt_match_16**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fpt_match_16 dominates: 16% faster than the next best (fpt_table_16)

fpt_match_16 (12.99 us) leads fpt_table_16 (15.12 us) by 16%, a clear separation rather than a photo finish. CV 9.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fpt_match_16 is fastest but the noisiest (CV 9.3%)

fpt_match_16 wins on median (12.99 us) yet has the highest variance (CV 9.3%), while fpt_table_16 is the steadiest (CV 3.8%, 15.12 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (fpt_match_16)

The baseline fpt_match_16 is the fastest (12.99 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (fpt_match_16) is the fastest** at 12992.9 ns median
- 1 variant significantly slower than baseline
- Spread: 1.16x (fastest 12992.9 ns, slowest 15123.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fn-pointer table vs match dispatch for 16-variant hot loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x478ab261e0d9f352 |
| Counter frequency | 2112822230 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fpt_match_16 | 14577ns | 14888ns | 12384ns | 14757ns | 16231ns | base |
| fpt_table_16 | 17061ns | 17017ns | 16278ns | 17035ns | 17920ns | +17.04% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fpt_match_16 | 12761ns | 10834ns | 14315ns | base | 1.284 |
| fpt_table_16 | 15161ns | 14383ns | 15998ns | +18.81% | 1.081 |

## Performance model

- Peak throughput: **1.512 Gops/s** (fpt_match_16; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fpt_match_16 | 1.261 | 83.4% |
| fpt_table_16 | 1.083 | 71.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fpt_match_16 | 14577ns | 14577ns | base |
| fpt_table_16 | 17061ns | 17061ns | +17.04% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fpt_match_16 | 12993ns | base | --- | [12801, 13108] | --- | --- | --- | --- |
| fpt_table_16 | 15124ns | +2104.9ns (+16.2%) | [+1806, +2263]ns | [15043, 15238] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fpt_match_16 | fpt_table_16 |
|---|---|---|---|
| 1 | 1 | 10966ns | +37.3% |
| 1 | 2 | 13897ns | +11.1% |
| 1 | 3 | 10700ns | +44.3% |
| 1 | 4 | 13001ns | +12.5% |
| 1 | 5 | 12627ns | +18.7% |
| 1 | 6 | 13331ns | +14.6% |
| 1 | 7 | 13303ns | +8.6% |
| 1 | 8 | 12964ns | +18.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fpt_match_16 | -0.489 | moderate- |
| fpt_table_16 | -0.345 | moderate- |

**Consistency summary:**

- **fpt_table_16**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fpt_match_16 | 26.3ns | 12760.7ns | 0.2% |  |
| fpt_table_16 | 27.5ns | 15160.8ns | 0.2% |  |

## Distribution (algo ns)

```
fpt_match_16 (n=160, range 10833.5-14314.8 ns)
  10833.5 |######################
  11007.6 |####
  11181.6 |########
  11355.7 |######
  11529.8 |
  11703.8 |####
  11877.9 |
  12052.0 |########
  12226.0 |########
  12400.1 |#################
  12574.2 |########################
  12748.2 |######################
  12922.3 |########################################
  13096.3 |######################
  13270.4 |#################################
  13444.5 |#################
  13618.5 |#############
  13792.6 |########
  13966.7 |#############
  14140.7 |#############
  (17 below, 12 above range)

fpt_table_16 (n=160, range 14383.2-15997.7 ns)
  14383.2 |##################
  14463.9 |############
  14544.7 |###
  14625.4 |###############
  14706.1 |####################################
  14786.8 |########################
  14867.5 |############
  14948.3 |###########################
  15029.0 |####################################
  15109.7 |########################
  15190.4 |##############################
  15271.2 |############
  15351.9 |########################################
  15432.6 |##################
  15513.3 |############
  15594.0 |###########################
  15674.8 |######
  15755.5 |##################
  15836.2 |#########
  15916.9 |############
  (17 below, 13 above range)

```
