# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_plain dominates: 56% faster than the next best (cs_exch)

cs_plain (452.32 us) leads cs_exch (707.11 us) by 56%, a clear separation rather than a photo finish. CV 12.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_snap is an outlier: 2.2x slower than the field

cs_snap (991.33 us) is 2.2x the fastest (452.32 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### cs_plain is fastest but the noisiest (CV 12.0%)

cs_plain wins on median (452.32 us) yet has the highest variance (CV 12.0%), while cs_snap is the steadiest (CV 5.8%, 991.33 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### No variant beats the baseline (cs_plain)

The baseline cs_plain is the fastest (452.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

## Key findings

- **Baseline (cs_plain) is the fastest** at 452323.3 ns median
- 3 variants significantly slower than baseline
- Spread: 2.19x (fastest 452323.3 ns, slowest 991329.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 2112802069 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 716044ns | 709860ns | 635910ns | 688046ns | 798108ns | +55.25% |
| cs_plain | 461211ns | 454747ns | 397514ns | 443167ns | 520123ns | base |
| cs_snap | 993227ns | 994812ns | 881855ns | 992723ns | 1049669ns | +115.35% |
| cs_snap_twin | 949598ns | 922121ns | 803750ns | 914597ns | 1075024ns | +105.89% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 713248ns | 633065ns | 795302ns | +55.52% | 0.001 |
| cs_plain | 458625ns | 395054ns | 517556ns | base | 0.002 |
| cs_snap | 989909ns | 878524ns | 1046286ns | +115.84% | 0.001 |
| cs_snap_twin | 946279ns | 800461ns | 1071596ns | +106.33% | 0.001 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_plain; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.001 | 55.9% |
| cs_plain | 0.002 | 87.3% |
| cs_snap | 0.001 | 39.9% |
| cs_snap_twin | 0.001 | 43.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 716044ns | 716044ns | +55.25% |
| cs_plain | 461211ns | 461211ns | base |
| cs_snap | 993227ns | 993227ns | +115.35% |
| cs_snap_twin | 949598ns | 949598ns | +105.89% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 452323ns | base | --- | [430923, 458058] | --- | --- | --- | --- |
| cs_exch | 707112ns | +240899.6ns (+53.3%) | [+210634, +276692]ns | [644471, 756680] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 991329ns | +530472.7ns (+117.3%) | [+475844, +586948]ns | [980988, 1026832] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 919000ns | +481839.6ns (+106.5%) | [+450640, +530188]ns | [904797, 1015910] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 439683ns | +55.2% | +140.1% | +101.1% |
| 1 | 2 | 457836ns | +58.5% | +117.2% | +133.4% |
| 1 | 3 | 423335ns | +51.7% | +109.0% | +145.5% |
| 2 | 1 | 453066ns | +81.6% | +116.7% | +102.3% |
| 2 | 2 | 564535ns | +33.4% | +80.6% | +62.7% |
| 2 | 3 | 399711ns | +60.0% | +160.7% | +128.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.295 | moderate- |
| cs_plain | -0.396 | moderate- |
| cs_snap | 0.130 | ok |
| cs_snap_twin | 0.038 | ok |

**Consistency summary:**

- **cs_exch**: won 0/6, lost 6/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 3853108.4ns | 713248.4ns | 540.2% | HIGH |
| cs_plain | 3850268.2ns | 458625.5ns | 839.5% | HIGH |
| cs_snap | 3862781.0ns | 989908.9ns | 390.2% | HIGH |
| cs_snap_twin | 3855579.4ns | 946278.8ns | 407.4% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 633064.9-795301.6 ns)
  633064.9 |########################################
  641176.8 |########################################
  649288.6 |
  657400.4 |
  665512.3 |
  673624.1 |
  681735.9 |##########################
  689847.8 |
  697959.6 |
  706071.4 |
  714183.2 |
  722295.1 |#############
  730406.9 |
  738518.7 |#############
  746630.6 |##########################
  754742.4 |#############
  762854.2 |
  770966.1 |
  779077.9 |#############
  787189.7 |
  (1 below, 3 above range)

cs_plain (n=18, range 395054.0-517556.1 ns)
  395054.0 |########################################
  401179.1 |
  407304.2 |
  413429.3 |
  419554.4 |#############
  425679.5 |
  431804.6 |
  437929.7 |##########################
  444054.8 |#############
  450179.9 |########################################
  456305.0 |########################################
  462430.1 |
  468555.3 |
  474680.4 |
  480805.5 |
  486930.6 |
  493055.7 |#############
  499180.8 |
  505305.9 |
  511431.0 |
  (1 below, 3 above range)

cs_snap (n=18, range 878523.8-1046285.7 ns)
  878523.8 |##########
  886911.9 |##########
  895300.0 |
  903688.1 |
  912076.2 |
  920464.2 |
  928852.3 |
  937240.4 |
  945628.5 |
  954016.6 |
  962404.7 |
  970792.8 |##########
  979180.9 |########################################
  987569.0 |####################
  995957.1 |
  1004345.2 |##########
  1012733.3 |####################
  1021121.4 |
  1029509.5 |##########
  1037897.6 |##########
  (1 below, 3 above range)

cs_snap_twin (n=18, range 800460.9-1071596.1 ns)
  800460.9 |
  814017.6 |
  827574.4 |######
  841131.2 |
  854687.9 |
  868244.7 |
  881801.4 |######
  895358.2 |#############
  908915.0 |########################################
  922471.7 |
  936028.5 |
  949585.3 |
  963142.0 |
  976698.8 |
  990255.6 |######
  1003812.3 |
  1017369.1 |
  1030925.8 |######
  1044482.6 |######
  1058039.4 |######
  (2 below, 2 above range)

```

## Diagnostics

- **cs_exch**: bridge=544.4% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=850.4% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=388.4% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=418.2% of algo (FFI overhead may distort results)
