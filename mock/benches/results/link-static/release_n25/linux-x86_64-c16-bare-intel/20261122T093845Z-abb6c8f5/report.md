# link-static: the fixture's release profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (129.84 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 67.63 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 92% faster than the next best (default_relink)

default_replay (67.63 ms) leads default_relink (129.84 ms) by 92%, a clear separation rather than a photo finish. CV 14.2%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 51% (significant)

default_replay is -66.21 ms (51%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### default_replay is fastest but the noisiest (CV 14.2%)

default_replay wins on median (67.63 ms) yet has the highest variance (CV 14.2%), while default_relink is the steadiest (CV 13.8%, 129.84 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### default_relink shows alternating (throttle bounce) (autocorr -0.55)

default_relink's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: default_replay** at 67632303.8 ns median (-47.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.92x (fastest 67632303.8 ns, slowest 129839266.0 ns)

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
| Counter frequency | 2112767148 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 137131805ns | 129841959ns | 117249257ns | 127700583ns | 161219912ns | base |
| default_replay | 69002025ns | 67634591ns | 58019819ns | 65263052ns | 80101587ns | -49.68% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 137129221ns | 117247087ns | 161217256ns | base |
| default_replay | 68999004ns | 58016387ns | 80097702ns | -49.68% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 137131805ns | 137131805ns | base |
| default_replay | 69002025ns | 69002025ns | -49.68% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 129839266ns | base | --- | [120331141, 161217256] | --- | --- | --- | --- |
| default_replay | 67632304ns | -66207016.9ns (-51.0%) | [-94710998, -43472636]ns | [59267007, 80097702] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 129784942ns | -55.3% |
| 1 | 2 | 129893590ns | -42.7% |
| 1 | 3 | 123415196ns | -49.1% |
| 1 | 4 | 155568060ns | -61.1% |
| 1 | 5 | 117247087ns | -26.9% |
| 1 | 6 | 166866452ns | -56.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.553 | HIGH- (thermal bounce) |
| default_replay | -0.225 | moderate- |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 55930785.3ns | 137129221.3ns | 40.8% | HIGH |
| default_replay | 63419271.7ns | 68999004.3ns | 91.9% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 117247086.8-161217256.4 ns)
  117247086.8 |####################
  119445595.3 |
  121644103.8 |####################
  123842612.2 |
  126041120.7 |
  128239629.2 |########################################
  130438137.7 |
  132636646.2 |
  134835154.6 |
  137033663.1 |
  139232171.6 |
  141430680.1 |
  143629188.6 |
  145827697.0 |
  148026205.5 |
  150224714.0 |
  152423222.5 |
  154621731.0 |####################
  156820239.4 |
  159018747.9 |
  (0 below, 1 above range)

default_replay (n=6, range 58016386.6-80097702.0 ns)
  58016386.6 |########################################
  59120452.4 |
  60224518.1 |########################################
  61328583.9 |
  62432649.7 |########################################
  63536715.5 |
  64640781.2 |
  65744847.0 |
  66848912.8 |
  67952978.6 |
  69057044.3 |
  70161110.1 |
  71265175.9 |
  72369241.6 |########################################
  73473307.4 |########################################
  74577373.2 |
  75681439.0 |
  76785504.7 |
  77889570.5 |
  78993636.3 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=43.4% of algo (FFI overhead may distort results)
- **default_replay**: bridge=95.9% of algo (FFI overhead may distort results)
