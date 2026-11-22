# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### rcm_adj shows alternating (throttle bounce) (autocorr -0.82)

rcm_adj's per-pass series has lag-1 autocorrelation -0.82, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (rcm_adj)

The baseline rcm_adj is the fastest (312.96 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 1.3% of the fastest

All 4 variants sit between 312.96 us and 317.14 us - a 1.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (rcm_adj) is the fastest** at 312962.5 ns median
- 3 variants significantly slower than baseline
- Spread: 1.01x (fastest 312962.5 ns, slowest 317135.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 320163ns | 319731ns | 316325ns | 319098ns | 323681ns | base |
| rcm_half | 324864ns | 322158ns | 316432ns | 320985ns | 334897ns | +1.47% |
| rcm_rev | 324772ns | 322198ns | 317508ns | 321913ns | 332692ns | +1.44% |
| rcm_scr | 324338ns | 324040ns | 321069ns | 323600ns | 327079ns | +1.30% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 313374ns | 309311ns | 316870ns | base | 0.001 |
| rcm_half | 318395ns | 310424ns | 328149ns | +1.60% | 0.001 |
| rcm_rev | 317715ns | 311583ns | 324673ns | +1.39% | 0.001 |
| rcm_scr | 317108ns | 314190ns | 319678ns | +1.19% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_adj; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 98.8% |
| rcm_half | 0.001 | 97.9% |
| rcm_rev | 0.001 | 98.1% |
| rcm_scr | 0.001 | 97.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 320163ns | 320163ns | base |
| rcm_half | 324864ns | 324864ns | +1.47% |
| rcm_rev | 324772ns | 324772ns | +1.44% |
| rcm_scr | 324338ns | 324338ns | +1.30% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 312962ns | base | --- | [311644, 315331] | --- | --- | --- | --- |
| rcm_half | 315912ns | +2950.0ns (+0.9%) | [+1023, +6956]ns | [312948, 321752] | YES | 0.0113 | 0.0075 | 0 |
| rcm_rev | 315452ns | +1733.4ns (+0.6%) | [+267, +4965]ns | [314894, 316573] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| rcm_scr | 317135ns | +3745.8ns (+1.2%) | [+2198, +6012]ns | [315446, 318794] | YES | 0.0113 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 313433ns | +1.0% | +0.7% | +0.6% |
| 1 | 2 | 310162ns | +3.2% | +2.0% | +2.2% |
| 1 | 3 | 317283ns | +2.7% | -1.4% | +0.6% |
| 2 | 1 | 309862ns | +0.5% | +2.3% | +2.5% |
| 2 | 2 | 315683ns | -0.5% | -0.3% | -0.3% |
| 2 | 3 | 312912ns | +0.9% | +0.6% | +0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.825 | HIGH- (thermal bounce) |
| rcm_half | -0.062 | ok |
| rcm_rev | -0.692 | HIGH- (thermal bounce) |
| rcm_scr | 0.211 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 1/6, lost 5/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 11178773.1ns | 313374.1ns | 3567.2% | HIGH |
| rcm_half | 11178021.1ns | 318394.9ns | 3510.7% | HIGH |
| rcm_rev | 11182379.6ns | 317715.3ns | 3519.6% | HIGH |
| rcm_scr | 11177915.3ns | 317107.9ns | 3525.0% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 309311.1-316870.1 ns)
  309311.1 |
  309689.1 |########################################
  310067.0 |####################
  310445.0 |
  310822.9 |
  311200.9 |####################
  311578.8 |####################
  311956.8 |########################################
  312334.7 |
  312712.7 |########################################
  313090.6 |####################
  313468.6 |
  313846.5 |
  314224.5 |####################
  314602.4 |
  314980.4 |####################
  315358.3 |########################################
  315736.3 |####################
  316114.2 |
  316492.2 |
  (1 below, 2 above range)

rcm_half (n=18, range 310423.6-328148.6 ns)
  310423.6 |##########
  311309.9 |##########
  312196.1 |##########
  313082.4 |##########
  313968.6 |####################
  314854.9 |
  315741.1 |########################################
  316627.4 |
  317513.6 |
  318399.9 |
  319286.1 |##########
  320172.4 |
  321058.6 |
  321944.9 |
  322831.1 |##########
  323717.4 |
  324603.6 |
  325489.9 |##########
  326376.1 |
  327262.4 |
  (2 below, 3 above range)

rcm_rev (n=18, range 311583.3-324672.9 ns)
  311583.3 |######
  312237.8 |######
  312892.3 |
  313546.8 |######
  314201.2 |
  314855.7 |########################################
  315510.2 |#############
  316164.7 |######
  316819.2 |#############
  317473.6 |
  318128.1 |
  318782.6 |######
  319437.1 |######
  320091.6 |
  320746.0 |
  321400.5 |
  322055.0 |
  322709.5 |
  323364.0 |
  324018.4 |
  (1 below, 1 above range)

rcm_scr (n=18, range 314190.3-319678.5 ns)
  314190.3 |
  314464.7 |########################################
  314739.1 |
  315013.5 |####################
  315287.9 |########################################
  315562.3 |########################################
  315836.7 |
  316111.1 |
  316385.5 |
  316660.0 |####################
  316934.4 |
  317208.8 |####################
  317483.2 |####################
  317757.6 |####################
  318032.0 |
  318306.4 |
  318580.8 |####################
  318855.2 |####################
  319129.6 |####################
  319404.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=3572.1% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3539.3% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=3543.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=3525.7% of algo (FFI overhead may distort results)
