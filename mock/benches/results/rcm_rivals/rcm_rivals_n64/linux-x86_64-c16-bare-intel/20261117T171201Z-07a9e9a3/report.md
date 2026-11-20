# ordering theories on a grid: RCM vs spectral vs greedy vs row-major

5 variants, 18 samples per variant.
Baseline: **riv_rcm**

## Highlights

Baseline for all deltas below: **riv_rcm**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.5% (fastest 459.32 us to slowest 466.15 us).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### riv_rcm is fastest but the noisiest (CV 7.6%)

riv_rcm wins on median (459.32 us) yet has the highest variance (CV 7.6%), while riv_snake is the steadiest (CV 3.8%, 463.91 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### riv_rowm shows alternating (throttle bounce) (autocorr -0.59)

riv_rowm's per-pass series has lag-1 autocorrelation -0.59, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (6.84 us) is smaller than the fastest variant's own run-to-run std-dev (35.05 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### No variant beats the baseline (riv_rcm)

The baseline riv_rcm is the fastest (459.32 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Speed leader riv_rcm vs stability leader riv_snake (+1% speed for 2.0x steadier)

riv_rcm is fastest (459.32 us, CV 7.6%); riv_snake gives up 1.0% median for 2.0x lower variance (CV 3.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Baseline (riv_rcm) is the fastest** at 459315.3 ns median
- Spread: 1.01x (fastest 459315.3 ns, slowest 466151.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | ordering theories on a grid: RCM vs spectral vs greedy vs row-major |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345679 |
| Counter frequency | 2112768073 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| riv_rcm | 458799ns | 461709ns | 415968ns | 447615ns | 496990ns | base |
| riv_rowm | 448465ns | 465102ns | 415075ns | 445573ns | 469498ns | -2.25% |
| riv_scr | 455004ns | 468396ns | 417163ns | 452689ns | 477396ns | -0.83% |
| riv_snake | 459686ns | 466091ns | 421491ns | 464701ns | 471260ns | +0.19% |
| riv_spec | 478701ns | 462567ns | 456745ns | 462147ns | 514510ns | +4.34% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| riv_rcm | 456437ns | 413655ns | 494564ns | base | 0.000 |
| riv_rowm | 446331ns | 412972ns | 467385ns | -2.21% | 0.000 |
| riv_scr | 452761ns | 414853ns | 475180ns | -0.81% | 0.000 |
| riv_snake | 457560ns | 419395ns | 469155ns | +0.25% | 0.000 |
| riv_spec | 476498ns | 454586ns | 512156ns | +4.40% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (riv_rowm; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| riv_rcm | 0.000 | 89.9% |
| riv_rowm | 0.000 | 89.2% |
| riv_scr | 0.000 | 88.6% |
| riv_snake | 0.000 | 89.0% |
| riv_spec | 0.000 | 89.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| riv_rcm | 458799ns | 458799ns | base |
| riv_rowm | 448465ns | 448465ns | -2.25% |
| riv_scr | 455004ns | 455004ns | -0.83% |
| riv_snake | 459686ns | 459686ns | +0.19% |
| riv_spec | 478701ns | 478701ns | +4.34% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| riv_rcm | 459315ns | base | --- | [417801, 468320] | --- | --- | --- | --- |
| riv_rowm | 462985ns | no significant difference | [-43734, +8640]ns | [416873, 466863] | no | 1.0000 | 1.0000 | 0 |
| riv_scr | 466152ns | no significant difference | [-42160, +44370]ns | [421733, 472704] | no | 1.0000 | 1.0000 | 0 |
| riv_snake | 463907ns | no significant difference | [-40240, +46570]ns | [461087, 465545] | no | 1.0000 | 0.4807 | 0 |
| riv_spec | 460511ns | no significant difference | [-4859, +40503]ns | [458483, 507214] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | riv_rcm | riv_rowm | riv_scr | riv_snake | riv_spec |
|---|---|---|---|---|---|---|
| 1 | 1 | 459976ns | +1.7% | -9.7% | +1.0% | -1.1% |
| 1 | 2 | 416646ns | -0.0% | +10.8% | +11.6% | +22.2% |
| 1 | 3 | 414389ns | +11.5% | +12.6% | +11.5% | +10.6% |
| 2 | 1 | 455366ns | +2.3% | -7.5% | +3.6% | +0.7% |
| 2 | 2 | 467321ns | -11.3% | +1.7% | -10.3% | -1.2% |
| 2 | 3 | 518671ns | -10.3% | -8.8% | -10.6% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| riv_rcm | 0.295 | moderate+ |
| riv_rowm | -0.593 | HIGH- (thermal bounce) |
| riv_scr | -0.248 | moderate- |
| riv_snake | -0.345 | moderate- |
| riv_spec | -0.330 | moderate- |

**Consistency summary:**

- **riv_rowm**: won 2/6, lost 3/6
- **riv_scr**: won 3/6, lost 3/6
- **riv_snake**: won 2/6, lost 4/6
- **riv_spec**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| riv_rcm | 2878850.9ns | 456436.7ns | 630.7% | HIGH |
| riv_rowm | 2871219.3ns | 446330.7ns | 643.3% | HIGH |
| riv_scr | 2873578.3ns | 452761.2ns | 634.7% | HIGH |
| riv_snake | 2872502.8ns | 457560.4ns | 627.8% | HIGH |
| riv_spec | 2874720.9ns | 476498.2ns | 603.3% | HIGH |

## Distribution (algo ns)

```
riv_rcm (n=18, range 413655.1-494564.2 ns)
  413655.1 |########################################
  417700.6 |##########
  421746.0 |
  425791.5 |
  429836.9 |
  433882.4 |
  437927.8 |
  441973.3 |
  446018.7 |
  450064.2 |##########
  454109.7 |##########
  458155.1 |####################
  462200.6 |##########
  466246.0 |##############################
  470291.5 |
  474336.9 |
  478382.4 |
  482427.8 |##########
  486473.3 |
  490518.8 |
  (1 below, 3 above range)

riv_rowm (n=18, range 412971.7-467384.9 ns)
  412971.7 |########
  415692.4 |########################
  418413.0 |
  421133.7 |
  423854.3 |########
  426575.0 |
  429295.7 |
  432016.3 |
  434737.0 |
  437457.7 |
  440178.3 |
  442899.0 |
  445619.6 |
  448340.3 |
  451061.0 |
  453781.6 |
  456502.3 |########
  459222.9 |
  461943.6 |################
  464664.3 |########################################
  (2 below, 3 above range)

riv_scr (n=18, range 414853.4-475180.1 ns)
  414853.4 |########################################
  417869.7 |
  420886.1 |##########################
  423902.4 |
  426918.7 |
  429935.1 |
  432951.4 |
  435967.7 |
  438984.1 |
  442000.4 |
  445016.7 |
  448033.1 |
  451049.4 |
  454065.7 |
  457082.1 |
  460098.4 |##########################
  463114.7 |#############
  466131.1 |##########################
  469147.4 |#############
  472163.7 |########################################
  (1 below, 3 above range)

riv_snake (n=18, range 419394.6-469155.4 ns)
  419394.6 |##########
  421882.6 |
  424370.7 |
  426858.7 |
  429346.8 |
  431834.8 |
  434322.8 |
  436810.9 |
  439298.9 |
  441786.9 |
  444275.0 |
  446763.0 |
  449251.1 |
  451739.1 |
  454227.1 |
  456715.2 |##########
  459203.2 |####################
  461691.3 |########################################
  464179.3 |########################################
  466667.3 |##########
  (2 below, 3 above range)

riv_spec (n=18, range 454586.2-512156.4 ns)
  454586.2 |########################
  457464.7 |########################################
  460343.2 |################
  463221.7 |########
  466100.2 |
  468978.7 |
  471857.2 |
  474735.8 |
  477614.3 |
  480492.8 |
  483371.3 |
  486249.8 |
  489128.3 |
  492006.8 |
  494885.3 |
  497763.8 |
  500642.3 |
  503520.8 |
  506399.3 |################
  509277.8 |################
  (1 below, 2 above range)

```

## Diagnostics

- **riv_rcm**: bridge=624.9% of algo (FFI overhead may distort results)
- **riv_rowm**: bridge=620.0% of algo (FFI overhead may distort results)
- **riv_scr**: bridge=616.0% of algo (FFI overhead may distort results)
- **riv_snake**: bridge=618.7% of algo (FFI overhead may distort results)
- **riv_spec**: bridge=623.5% of algo (FFI overhead may distort results)
