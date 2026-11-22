# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (rcm_adj) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline rcm_adj has the worst median (9.63 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rcm_rev at 9.53 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 1.0% (fastest 9.53 ms to slowest 9.63 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### rcm_scr shows alternating (throttle bounce) (autocorr -0.51)

rcm_scr's per-pass series has lag-1 autocorrelation -0.51, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (97.10 us) is smaller than the fastest variant's own run-to-run std-dev (351.40 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### Speed leader rcm_rev vs stability leader rcm_scr (+0% speed for 2.4x steadier)

rcm_rev is fastest (9.53 ms, CV 3.7%); rcm_scr gives up 0.3% median for 2.4x lower variance (CV 1.5%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: rcm_rev** at 9532543.9 ns median (-1.0% vs baseline)
- Spread: 1.01x (fastest 9532543.9 ns, slowest 9629639.0 ns)

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
| Counter frequency | 2112757625 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 9639886ns | 9633557ns | 9277043ns | 9587786ns | 9899457ns | base |
| rcm_half | 9536168ns | 9568571ns | 9251140ns | 9508573ns | 9720075ns | -1.08% |
| rcm_rev | 9685769ns | 9536652ns | 9444242ns | 9526455ns | 10045504ns | +0.48% |
| rcm_scr | 9579599ns | 9563738ns | 9408425ns | 9526675ns | 9744572ns | -0.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 9635908ns | 9273052ns | 9895524ns | base | 0.000 |
| rcm_half | 9532159ns | 9247105ns | 9716111ns | -1.08% | 0.000 |
| rcm_rev | 9681699ns | 9439969ns | 10041299ns | +0.48% | 0.000 |
| rcm_scr | 9575753ns | 9404434ns | 9740732ns | -0.62% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 2048

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 96.0% |
| rcm_half | 0.000 | 96.7% |
| rcm_rev | 0.000 | 97.0% |
| rcm_scr | 0.000 | 96.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 9639886ns | 9639886ns | base |
| rcm_half | 9536168ns | 9536168ns | -1.08% |
| rcm_rev | 9685769ns | 9685769ns | +0.48% |
| rcm_scr | 9579599ns | 9579599ns | -0.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 9629639ns | base | --- | [9531678, 9703676] | --- | --- | --- | --- |
| rcm_half | 9564518ns | no significant difference | [-241196, +37350]ns | [9413501, 9672276] | no | 0.2888 | 0.0963 | 0 |
| rcm_rev | 9532544ns | no significant difference | [-93290, +169203]ns | [9497625, 9600867] | no | 0.8145 | 0.8145 | 0 |
| rcm_scr | 9559780ns | no significant difference | [-178978, +101280]ns | [9456435, 9628780] | no | 0.7210 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 9679876ns | -0.6% | +1.4% | -0.8% |
| 1 | 2 | 9586434ns | -2.0% | -1.0% | -1.9% |
| 1 | 3 | 9267068ns | +2.7% | +1.9% | +3.7% |
| 2 | 1 | 9504755ns | +2.1% | +7.6% | -0.6% |
| 2 | 2 | 9756197ns | -1.0% | -2.4% | +0.9% |
| 2 | 3 | 9636538ns | -3.9% | -0.8% | -1.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.116 | ok |
| rcm_half | -0.167 | ok |
| rcm_rev | -0.400 | moderate- |
| rcm_scr | -0.515 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rcm_half**: won 4/6, lost 2/6
- **rcm_rev**: won 3/6, lost 3/6
- **rcm_scr**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 61663346.5ns | 9635908.4ns | 639.9% | HIGH |
| rcm_half | 61848409.4ns | 9532159.2ns | 648.8% | HIGH |
| rcm_rev | 61685657.9ns | 9681699.4ns | 637.1% | HIGH |
| rcm_scr | 61343557.4ns | 9575752.7ns | 640.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 9273051.6-9895524.5 ns)
  9273051.6 |#############
  9304175.3 |
  9335298.9 |
  9366422.6 |
  9397546.2 |
  9428669.8 |#############
  9459793.5 |
  9490917.1 |#############
  9522040.8 |
  9553164.4 |#############
  9584288.1 |##########################
  9615411.7 |########################################
  9646535.3 |#############
  9677659.0 |#############
  9708782.6 |#############
  9739906.3 |#############
  9771029.9 |
  9802153.6 |
  9833277.2 |
  9864400.8 |
  (2 below, 3 above range)

rcm_half (n=18, range 9247105.0-9716110.5 ns)
  9247105.0 |#############
  9270555.3 |#############
  9294005.6 |
  9317455.9 |
  9340906.1 |
  9364356.4 |#############
  9387806.7 |#############
  9411257.0 |#############
  9434707.2 |
  9458157.5 |#############
  9481607.8 |#############
  9505058.1 |#############
  9528508.3 |
  9551958.6 |
  9575408.9 |
  9598859.2 |#############
  9622309.4 |##########################
  9645759.7 |#############
  9669210.0 |#############
  9692660.3 |########################################
  (1 below, 1 above range)

rcm_rev (n=18, range 9439968.7-10041299.4 ns)
  9439968.7 |##########
  9470035.3 |##############################
  9500101.8 |########################################
  9530168.3 |####################
  9560234.9 |##########
  9590301.4 |####################
  9620367.9 |
  9650434.5 |
  9680501.0 |
  9710567.5 |
  9740634.1 |
  9770700.6 |
  9800767.1 |##########
  9830833.7 |
  9860900.2 |
  9890966.7 |
  9921033.3 |
  9951099.8 |
  9981166.3 |
  10011232.9 |
  (1 below, 3 above range)

rcm_scr (n=18, range 9404434.0-9740731.8 ns)
  9404434.0 |####################
  9421248.9 |####################
  9438063.7 |####################
  9454878.6 |####################
  9471693.5 |
  9488508.4 |####################
  9505323.3 |
  9522138.2 |########################################
  9538953.1 |
  9555768.0 |
  9572582.9 |####################
  9589397.8 |####################
  9606212.7 |########################################
  9623027.5 |####################
  9639842.4 |####################
  9656657.3 |
  9673472.2 |
  9690287.1 |
  9707102.0 |
  9723916.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=638.1% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=642.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=643.8% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=641.6% of algo (FFI overhead may distort results)
