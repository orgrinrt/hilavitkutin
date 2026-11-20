# link-static: the fixture's dev profile

2 variants, 6 samples per variant.
Baseline: **default_relink**

## Highlights

Baseline for all deltas below: **default_relink**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (default_relink) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline default_relink has the worst median (510.24 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest default_replay at 295.63 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### default_replay dominates: 73% faster than the next best (default_relink)

default_replay (295.63 ms) leads default_relink (510.24 ms) by 73%, a clear separation rather than a photo finish. CV 2.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### default_replay beats baseline by 42% (significant)

default_replay is -215.62 ms (42%) faster than baseline default_relink, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

## Key findings

- **Fastest: default_replay** at 295631423.4 ns median (-42.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.73x (fastest 295631423.4 ns, slowest 510238704.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | link-static: the fixture's dev profile |
| Passes | 6 |
| Runs per pass | 1 |
| Batch size | 1 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x6c6b737461740001 |
| Counter frequency | 2100314120 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| default_relink | 514166992ns | 510242897ns | 473742104ns | 504730390ns | 548534338ns | base |
| default_replay | 297169208ns | 295635799ns | 288721524ns | 294046323ns | 306077376ns | -42.20% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean |
|---|---|---|---|---|
| default_relink | 514162984ns | 473737945ns | 548530664ns | base |
| default_replay | 297165033ns | 288717653ns | 306072927ns | -42.20% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| default_relink | 514166992ns | 514166992ns | base |
| default_replay | 297169208ns | 297169208ns | -42.20% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| default_relink | 510238705ns | base | --- | [483719584, 548530664] | --- | --- | --- | --- |
| default_replay | 295631423ns | -215624655.1ns (-42.3%) | [-252437210, -182931990]ns | [289790748, 306072927] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | default_relink | default_replay |
|---|---|---|---|
| 1 | 1 | 512918984ns | -43.3% |
| 1 | 2 | 534341069ns | -46.0% |
| 1 | 3 | 493701224ns | -40.7% |
| 1 | 4 | 562720258ns | -46.1% |
| 1 | 5 | 507558425ns | -41.2% |
| 1 | 6 | 473737945ns | -34.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| default_relink | -0.305 | moderate- |
| default_replay | 0.276 | moderate+ |

**Consistency summary:**

- **default_replay**: won 6/6, lost 0/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| default_relink | 309555324.3ns | 514162984.2ns | 60.2% | HIGH |
| default_replay | 319327068.3ns | 297165032.6ns | 107.5% | HIGH |

## Distribution (algo ns)

```
default_relink (n=6, range 473737945.0-548530663.5 ns)
  473737945.0 |########################################
  477477580.9 |
  481217216.9 |
  484956852.8 |
  488696488.7 |
  492436124.6 |########################################
  496175760.6 |
  499915396.5 |
  503655032.4 |
  507394668.3 |########################################
  511134304.3 |########################################
  514873940.2 |
  518613576.1 |
  522353212.1 |
  526092848.0 |
  529832483.9 |
  533572119.8 |########################################
  537311755.8 |
  541051391.7 |
  544791027.6 |
  (0 below, 1 above range)

default_replay (n=6, range 288717653.1-306072926.7 ns)
  288717653.1 |########################################
  289585416.8 |
  290453180.5 |########################################
  291320944.1 |
  292188707.8 |########################################
  293056471.5 |
  293924235.2 |
  294791998.9 |
  295659762.5 |
  296527526.2 |
  297395289.9 |
  298263053.6 |########################################
  299130817.3 |
  299998580.9 |
  300866344.6 |
  301734108.3 |
  302601872.0 |########################################
  303469635.7 |
  304337399.3 |
  305205163.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **default_relink**: bridge=60.8% of algo (FFI overhead may distort results)
- **default_replay**: bridge=107.0% of algo (FFI overhead may distort results)
