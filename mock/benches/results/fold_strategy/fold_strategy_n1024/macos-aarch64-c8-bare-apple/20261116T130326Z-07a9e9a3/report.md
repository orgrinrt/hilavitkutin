# Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way

3 variants, 160 samples per variant.
Baseline: **fold_paired**

## Highlights

Baseline for all deltas below: **fold_paired**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### fold_quad dominates: 29% faster than the next best (fold_paired)

fold_quad (52 ns) leads fold_paired (67 ns) by 29%, a clear separation rather than a photo finish. CV 7.1%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### fold_quad beats baseline by 23% (significant)

fold_quad is -15 ns (23%) faster than baseline fold_paired, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### fold_sequential is an outlier: 2.0x slower than the field

fold_sequential (106 ns) is 2.0x the fastest (52 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### fold_quad is fastest but the noisiest (CV 7.1%)

fold_quad wins on median (52 ns) yet has the highest variance (CV 7.1%), while fold_sequential is the steadiest (CV 2.3%, 106 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### fold_paired shows alternating (throttle bounce) (autocorr -0.58)

fold_paired's per-pass series has lag-1 autocorrelation -0.58, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: fold_quad** at 52.1 ns median (-22.4% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.04x (fastest 52.1 ns, slowest 106.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Fold ILP: sequential single-acc vs paired 2-way vs quad 4-way |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x239fb80547ae61c2 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fold_paired | 2152ns | 2149ns | 2145ns | 2149ns | 2168ns | base |
| fold_quad | 2132ns | 2128ns | 2123ns | 2128ns | 2151ns | -0.95% |
| fold_sequential | 2210ns | 2207ns | 2203ns | 2208ns | 2226ns | +2.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fold_paired | 68ns | 63ns | 73ns | base | 15.146 |
| fold_quad | 52ns | 47ns | 58ns | -22.69% | 19.592 |
| fold_sequential | 107ns | 103ns | 110ns | +57.66% | 9.607 |

## Performance model

- Peak throughput: **21.561 Gops/s** (fold_quad; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fold_paired | 15.261 | 70.8% |
| fold_quad | 19.655 | 91.2% |
| fold_sequential | 9.642 | 44.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fold_paired | 2152ns | 2152ns | base |
| fold_quad | 2132ns | 2132ns | -0.95% |
| fold_sequential | 2210ns | 2210ns | +2.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fold_paired | 67ns | base | --- | [67, 68] | --- | --- | --- | --- |
| fold_quad | 52ns | -15.2ns (-22.7%) | [-16, -14]ns | [51, 53] | YES | 0.0000 | 0.0000 | 0 |
| fold_sequential | 106ns | +38.8ns (+57.8%) | [+39, +40]ns | [106, 107] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | fold_paired | fold_quad | fold_sequential |
|---|---|---|---|---|
| 1 | 1 | 68ns | -20.6% | +57.9% |
| 1 | 2 | 67ns | -21.5% | +58.7% |
| 1 | 3 | 68ns | -25.5% | +55.8% |
| 1 | 4 | 67ns | -23.3% | +58.6% |
| 1 | 5 | 67ns | -23.8% | +59.1% |
| 1 | 6 | 68ns | -24.2% | +57.7% |
| 1 | 7 | 67ns | -21.5% | +58.7% |
| 1 | 8 | 67ns | -21.8% | +58.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fold_paired | -0.583 | HIGH- (thermal bounce) |
| fold_quad | 0.263 | moderate+ |
| fold_sequential | -0.099 | ok |

**Consistency summary:**

- **fold_quad**: won 8/8, lost 0/8
- **fold_sequential**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fold_paired | 2.6ns | 67.6ns | 3.9% |  |
| fold_quad | 2.5ns | 52.3ns | 4.8% |  |
| fold_sequential | 2.7ns | 106.6ns | 2.6% |  |

## Distribution (algo ns)

```
fold_paired (n=160, range 62.7-73.0 ns)
     62.7 |############
     63.2 |###############
     63.7 |##################
     64.2 |##################
     64.8 |########################
     65.3 |#####################
     65.8 |########################################
     66.3 |###########################
     66.8 |####################################
     67.3 |#####################
     67.8 |#################################
     68.4 |########################
     68.9 |###############
     69.4 |############
     69.9 |########################
     70.4 |##################
     70.9 |#########
     71.5 |###
     72.0 |###############
     72.5 |############
  (12 below, 16 above range)

fold_quad (n=160, range 47.5-57.7 ns)
     47.5 |##########
     48.0 |###
     48.5 |##########
     49.0 |##############
     49.5 |########################################
     50.0 |#########
     50.5 |############
     51.1 |#########
     51.6 |#######
     52.1 |#####################
     52.6 |############
     53.1 |#########
     53.6 |#######
     54.1 |#########################
     54.6 |#######
     55.1 |#
     55.6 |############
     56.2 |#######
     56.7 |##################
     57.2 |#
  (13 below, 13 above range)

fold_sequential (n=160, range 103.3-110.3 ns)
    103.3 |
    103.7 |################################
    104.0 |########################
    104.4 |##################
    104.7 |################
    105.1 |##################
    105.4 |
    105.8 |########################################
    106.1 |##################################
    106.5 |########################
    106.8 |################
    107.2 |##########################
    107.5 |
    107.9 |##########################
    108.2 |#############
    108.6 |########
    108.9 |##################
    109.3 |#############
    109.6 |
    109.9 |################
  (15 below, 15 above range)

```
