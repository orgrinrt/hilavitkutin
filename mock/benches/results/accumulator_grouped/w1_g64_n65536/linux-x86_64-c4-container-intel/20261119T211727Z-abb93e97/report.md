# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (380.58 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 307.55 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### per_core dominates: 15% faster than the next best (sequential)

per_core (307.55 us) leads sequential (353.94 us) by 15%, a clear separation rather than a photo finish. CV 3.3%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### per_core shows alternating (throttle bounce) (autocorr -0.53)

per_core's per-pass series has lag-1 autocorrelation -0.53, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 307545.4 ns median (-19.2% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.24x (fastest 307545.4 ns, slowest 380578.7 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100372148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 375125ns | 381804ns | 328431ns | 375626ns | 397721ns | base |
| per_core | 308418ns | 309170ns | 294703ns | 305287ns | 319971ns | -17.78% |
| sequential | 360374ns | 355274ns | 292990ns | 341452ns | 422451ns | -3.93% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 373930ns | 326767ns | 396543ns | base | 0.175 |
| per_core | 306952ns | 293543ns | 318467ns | -17.91% | 0.214 |
| sequential | 358973ns | 292305ns | 420449ns | -4.00% | 0.183 |

## Performance model

- Peak throughput: **0.224 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.172 | 76.8% |
| per_core | 0.213 | 95.0% |
| sequential | 0.185 | 82.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 375125ns | 375125ns | base |
| per_core | 308418ns | 308418ns | -17.78% |
| sequential | 360374ns | 360374ns | -3.93% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 380579ns | base | --- | [367553, 389984] | --- | --- | --- | --- |
| per_core | 307545ns | -67722.0ns (-17.8%) | [-79772, -56835]ns | [298932, 312903] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 353944ns | no significant difference | [-58660, +29857]ns | [313029, 398939] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 388495ns | -19.5% | +4.0% |
| 1 | 2 | 379975ns | -21.7% | -22.1% |
| 1 | 3 | 377374ns | -18.9% | -6.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.057 | ok |
| per_core | -0.533 | HIGH- (thermal bounce) |
| sequential | -0.524 | HIGH- (thermal bounce) |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 386674.0ns | 373929.9ns | 103.4% | HIGH |
| per_core | 305024.8ns | 306952.1ns | 99.4% | HIGH |
| sequential | 355141.2ns | 358973.5ns | 98.9% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 326767.4-396542.8 ns)
  326767.4 |
  330256.2 |
  333744.9 |
  337233.7 |#############
  340722.5 |
  344211.2 |
  347700.0 |
  351188.8 |#############
  354677.5 |
  358166.3 |
  361655.1 |
  365143.8 |##########################
  368632.6 |
  372121.4 |##########################
  375610.1 |
  379098.9 |##########################
  382587.7 |
  386076.4 |########################################
  389565.2 |##########################
  393054.0 |#############
  (2 below, 2 above range)

per_core (n=18, range 293542.7-318466.6 ns)
  293542.7 |
  294788.9 |
  296035.1 |########################################
  297281.3 |####################
  298527.5 |####################
  299773.7 |####################
  301019.9 |
  302266.1 |
  303512.3 |####################
  304758.5 |
  306004.7 |
  307250.9 |########################################
  308497.1 |####################
  309743.3 |####################
  310989.5 |####################
  312235.7 |
  313481.9 |########################################
  314728.0 |
  315974.2 |
  317220.4 |
  (2 below, 3 above range)

sequential (n=18, range 292305.3-420448.8 ns)
  292305.3 |########################################
  298712.5 |
  305119.6 |#############
  311526.8 |
  317934.0 |#############
  324341.2 |#############
  330748.3 |#############
  337155.5 |
  343562.7 |
  349969.9 |#############
  356377.0 |#############
  362784.2 |
  369191.4 |#############
  375598.6 |
  382005.7 |
  388412.9 |#############
  394820.1 |##########################
  401227.2 |
  407634.4 |##########################
  414041.6 |
  (1 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=105.4% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.2% of algo (FFI overhead may distort results)
