# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Key findings

- **Baseline (sb_none) is the fastest** at 9887.5 ns median
- 2 variants significantly slower than baseline
- Spread: 2.24x (fastest 9887.5 ns, slowest 22179.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 12363ns | 12558ns | 11249ns | 12279ns | 13046ns | base |
| sb_plan | 16735ns | 16817ns | 15494ns | 16484ns | 17731ns | +35.37% |
| sb_value | 25053ns | 25138ns | 22479ns | 24936ns | 26515ns | +102.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 9676ns | 8943ns | 10083ns | base | 0.007 |
| sb_plan | 13732ns | 13076ns | 14262ns | +41.92% | 0.005 |
| sb_value | 21962ns | 19910ns | 23097ns | +126.98% | 0.003 |

## Performance model

- Peak throughput: **0.007 Gops/s** (sb_none; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.006 | 90.4% |
| sb_plan | 0.005 | 64.0% |
| sb_value | 0.003 | 40.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 12363ns | 12363ns | base |
| sb_plan | 16735ns | 16735ns | +35.37% |
| sb_value | 25053ns | 25053ns | +102.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 9888ns | base | --- | [9410, 9965] | --- | --- | --- | --- |
| sb_plan | 13983ns | +4099.9ns (+41.5%) | [+3996, +4229]ns | [13092, 14165] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 22179ns | +12270.9ns (+124.1%) | [+11771, +12929]ns | [21533, 22758] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|
| 1 | 9358ns | +39.8% | +129.2% |
| 2 | 10242ns | +27.6% | +91.1% |
| 3 | 8762ns | +49.2% | +123.0% |
| 4 | 10117ns | +44.5% | +127.8% |
| 5 | 9950ns | +42.0% | +122.4% |
| 6 | 9829ns | +41.8% | +117.4% |
| 7 | 10088ns | +40.6% | +130.1% |
| 8 | 9975ns | +41.2% | +122.9% |
| 9 | 9821ns | +40.3% | +122.2% |
| 10 | 9021ns | +45.1% | +159.6% |
| 11 | 9046ns | +44.7% | +158.5% |
| 12 | 9046ns | +44.6% | +141.3% |
| 13 | 9954ns | +42.9% | +131.3% |
| 14 | 9946ns | +42.2% | +123.5% |
| 15 | 9462ns | +50.0% | +136.9% |
| 16 | 9950ns | +42.8% | +126.1% |
| 17 | 10125ns | +38.6% | +112.7% |
| 18 | 9471ns | +38.8% | +117.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.050 | ok |
| sb_plan | 0.397 | moderate+ |
| sb_value | 0.287 | moderate+ |

**Consistency summary:**

- **sb_plan**: won 0/18, lost 18/18
- **sb_value**: won 0/18, lost 18/18

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 386313.0ns | 9675.7ns | 3992.6% | HIGH |
| sb_plan | 391290.0ns | 13731.9ns | 2849.5% | HIGH |
| sb_value | 390770.1ns | 21961.8ns | 1779.3% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 8943.0-10083.4 ns)
   8943.0 |
   9000.0 |##############################
   9057.1 |
   9114.1 |
   9171.1 |
   9228.1 |
   9285.1 |
   9342.1 |##########
   9399.2 |
   9456.2 |####################
   9513.2 |
   9570.2 |
   9627.2 |
   9684.2 |
   9741.3 |
   9798.3 |####################
   9855.3 |
   9912.3 |########################################
   9969.3 |##########
  10026.3 |
  (1 below, 4 above range)

sb_plan (n=18, range 13076.4-14262.5 ns)
  13076.4 |########################################
  13135.7 |##########
  13195.0 |
  13254.3 |
  13313.6 |
  13372.9 |
  13432.2 |
  13491.5 |
  13550.8 |
  13610.1 |
  13669.4 |
  13728.7 |##########
  13788.0 |
  13847.4 |
  13906.7 |##########
  13966.0 |
  14025.3 |##########
  14084.6 |####################
  14143.9 |##############################
  14203.2 |####################
  (2 below, 1 above range)

sb_value (n=18, range 19909.7-23096.5 ns)
  19909.7 |
  20069.1 |
  20228.4 |
  20387.8 |
  20547.1 |####################
  20706.4 |
  20865.8 |
  21025.1 |
  21184.4 |
  21343.8 |########################################
  21503.1 |####################
  21662.5 |
  21821.8 |########################################
  21981.1 |####################
  22140.5 |########################################
  22299.8 |####################
  22459.2 |####################
  22618.5 |
  22777.8 |
  22937.2 |########################################
  (2 below, 3 above range)

```

## Diagnostics

- **sb_none**: bridge=3991.7% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=2843.1% of algo (FFI overhead may distort results)
- **sb_value**: bridge=1798.7% of algo (FFI overhead may distort results)
