# link-static: the fixture's release profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (161.64 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 72.02 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 124% faster than the next best (default_relink)

default_replay (72.02 ms) leads default_relink (161.64 ms) by 124%, a clear separation rather than a photo finish. CV 5.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 56% (significant)

default_replay is -90.36 ms (56%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_relink shows alternating (throttle bounce) (autocorr -0.69)

default_relink's per-pass series has lag-1 autocorrelation -0.69, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: default_replay** at 72020726.6 ns median (-55.4% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 2.24x (fastest 72020726.6 ns, slowest 161638329.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's release profile |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c6b737461740001 |
| Counter frequency | 2100641458 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 168071054ns | 161642854ns | 152023854ns | 160451819ns | 187523508ns | base |
| default_replay | 73575320ns | 72025595ns | 69968850ns | 71366511ns | 78691769ns | -56.22% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 168066202ns | 152020182ns | 187517389ns | base |
| default_replay | 73570690ns | 69964635ns | 78686896ns | -56.23% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 168071054ns | 168071054ns | base |
| default_replay | 73575320ns | 73575320ns | -56.22% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 161638330ns | base | --- | [155042886, 187517389] | --- | --- | --- | --- |
| default_replay | 72020727ns | -90359306.8ns (-55.9%) | [-112222917, -80904312]ns | [70004446, 78686896] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 158065591ns | -50.5% |
| 1 | 2 | 197046425ns | -63.7% |
| 1 | 3 | 152020182ns | -54.0% |
| 1 | 4 | 177988352ns | -55.6% |
| 1 | 5 | 160519066ns | -54.8% |
| 1 | 6 | 162757594ns | -57.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.687 | HIGH- (thermal bounce) |
| default_replay | -0.290 | moderate- |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 66323080.7ns | 168066201.7ns | 39.5% | HIGH |
| default_replay | 67141379.2ns | 73570689.6ns | 91.3% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 152020181.9-187517388.8 ns)
  152020181.9 |########################################
  153795042.2 |
  155569902.6 |
  157344762.9 |########################################
  159119623.3 |########################################
  160894483.6 |
  162669344.0 |########################################
  164444204.3 |
  166219064.6 |
  167993925.0 |
  169768785.3 |
  171543645.7 |
  173318506.0 |
  175093366.4 |
  176868226.7 |########################################
  178643087.0 |
  180417947.4 |
  182192807.7 |
  183967668.1 |
  185742528.4 |
  (0 below, 1 above range)

default_replay (n=6, range 69964634.6-78686896.2 ns)
  69964634.6 |########################################
  70400747.7 |
  70836860.8 |
  71272973.8 |####################
  71709086.9 |
  72145200.0 |####################
  72581313.1 |
  73017426.2 |
  73453539.2 |
  73889652.3 |
  74325765.4 |
  74761878.5 |
  75197991.6 |
  75634104.6 |
  76070217.7 |
  76506330.8 |
  76942443.9 |
  77378557.0 |
  77814670.0 |
  78250783.1 |####################
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=40.7% of algo (FFI overhead may distort results)
- **default_replay**: bridge=91.4% of algo (FFI overhead may distort results)
