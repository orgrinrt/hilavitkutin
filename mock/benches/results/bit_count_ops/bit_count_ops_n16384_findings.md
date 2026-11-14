# Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop

3 variants, 160 samples per variant.
Baseline: **ctz_debruijn**

## Highlights

Baseline for all deltas below: **ctz_debruijn**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### ctz_loop is an outlier: 4.3x slower than the field

ctz_loop (2.14 us) is 4.3x the fastest (500 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### ctz_intrinsic is fastest but the noisiest (CV 6.2%)

ctz_intrinsic wins on median (500 ns) yet has the highest variance (CV 6.2%), while ctz_loop is the steadiest (CV 5.5%, 2.14 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### ctz_intrinsic shows alternating (throttle bounce) (autocorr -0.57)

ctz_intrinsic's per-pass series has lag-1 autocorrelation -0.57, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.3x the fastest

Fastest ctz_intrinsic (500 ns) to slowest ctz_loop (2.14 us): 4.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: ctz_intrinsic** at 499.6 ns median (-2.4% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 4.29x (fastest 499.6 ns, slowest 2141.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Trailing-zero count: std intrinsic vs De Bruijn lookup vs shift-loop |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6f1038d492c5ab07 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| ctz_debruijn | 2701ns | 2661ns | 2551ns | 2666ns | 2954ns | base |
| ctz_intrinsic | 2676ns | 2618ns | 2549ns | 2637ns | 2923ns | -0.90% |
| ctz_loop | 4391ns | 4362ns | 4074ns | 4382ns | 4738ns | +62.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| ctz_debruijn | 521ns | 489ns | 573ns | base | 31.433 |
| ctz_intrinsic | 515ns | 487ns | 568ns | -1.22% | 31.821 |
| ctz_loop | 2160ns | 1998ns | 2339ns | +314.49% | 7.584 |

## Performance model

- Peak throughput: **33.611 Gops/s** (ctz_intrinsic; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| ctz_debruijn | 32.022 | 95.3% |
| ctz_intrinsic | 32.794 | 97.6% |
| ctz_loop | 7.649 | 22.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| ctz_debruijn | 2701ns | 2701ns | base |
| ctz_intrinsic | 2676ns | 2676ns | -0.90% |
| ctz_loop | 4391ns | 4391ns | +62.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| ctz_debruijn | 512ns | base | --- | [507, 522] | --- | --- | --- | --- |
| ctz_intrinsic | 500ns | no significant difference | [-6, +0]ns | [496, 506] | no | 0.0652 | 0.0652 | 4 |
| ctz_loop | 2142ns | +1625.2ns (+317.6%) | [+1619, +1662]ns | [2117, 2176] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | ctz_debruijn | ctz_intrinsic | ctz_loop |
|---|---|---|---|---|
| 1 | 1 | 493ns | -0.1% | +329.3% |
| 1 | 2 | 497ns | -0.7% | +325.4% |
| 1 | 3 | 504ns | +4.8% | +346.4% |
| 1 | 4 | 498ns | -0.6% | +331.2% |
| 1 | 5 | 566ns | -5.8% | +252.5% |
| 1 | 6 | 508ns | -3.2% | +330.1% |
| 1 | 7 | 530ns | +7.2% | +294.9% |
| 1 | 8 | 530ns | -7.3% | +307.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| ctz_debruijn | -0.072 | ok |
| ctz_intrinsic | -0.570 | HIGH- (thermal bounce) |
| ctz_loop | -0.329 | moderate- |

**Consistency summary:**

- **ctz_intrinsic**: won 6/8, lost 2/8
- **ctz_loop**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| ctz_debruijn | 2.9ns | 521.2ns | 0.6% |  |
| ctz_intrinsic | 2.8ns | 514.9ns | 0.5% |  |
| ctz_loop | 3.4ns | 2160.5ns | 0.2% |  |

## Distribution (algo ns)

```
ctz_debruijn (n=160, range 489.4-573.4 ns)
    489.4 |########################################
    493.6 |#############################
    497.8 |################################
    502.0 |##################################
    506.2 |########################
    510.4 |##########################
    514.6 |########
    518.8 |##########
    523.0 |################################
    527.2 |#####################################
    531.4 |#############
    535.6 |########
    539.8 |
    544.0 |
    548.2 |
    552.4 |##
    556.6 |##
    560.8 |#####################
    565.0 |#####################
    569.2 |#############
  (16 below, 10 above range)

ctz_intrinsic (n=160, range 487.5-567.9 ns)
    487.5 |########################################
    491.5 |################################
    495.5 |##############
    499.5 |############
    503.5 |######
    507.6 |
    511.6 |###
    515.6 |##
    519.6 |#########
    523.6 |##########
    527.7 |###########
    531.7 |#######
    535.7 |###
    539.7 |
    543.7 |
    547.8 |
    551.8 |
    555.8 |##
    559.8 |#
    563.9 |#######
  (12 below, 19 above range)

ctz_loop (n=160, range 1997.9-2338.6 ns)
   1997.9 |#########
   2015.0 |#
   2032.0 |#
   2049.0 |#########
   2066.1 |###
   2083.1 |##########
   2100.1 |########################################
   2117.2 |##########
   2134.2 |###############
   2151.2 |#
   2168.3 |##################
   2185.3 |##########
   2202.3 |#######
   2219.4 |#########
   2236.4 |#
   2253.5 |#######
   2270.5 |#############
   2287.5 |#
   2304.6 |############
   2321.6 |####
  (21 below, 15 above range)

```
