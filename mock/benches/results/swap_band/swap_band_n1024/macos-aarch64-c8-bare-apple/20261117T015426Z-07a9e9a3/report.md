# next-frame asymmetry: clean skip vs value-swap cone vs plan band

3 variants, 18 samples per variant.
Baseline: **sb_none**

## Highlights

Baseline for all deltas below: **sb_none**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sb_none dominates: 23% faster than the next best (sb_plan)

sb_none (9.97 us) leads sb_plan (12.26 us) by 23%, a clear separation rather than a photo finish. CV 1.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sb_value is an outlier: 15.7x slower than the field

sb_value (156.61 us) is 15.7x the fastest (9.97 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### No variant beats the baseline (sb_none)

The baseline sb_none is the fastest (9.97 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Wide spread: slowest is 15.7x the fastest

Fastest sb_none (9.97 us) to slowest sb_value (156.61 us): 15.7x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Baseline (sb_none) is the fastest** at 9975.0 ns median
- 2 variants significantly slower than baseline
- Spread: 15.70x (fastest 9975.0 ns, slowest 156606.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | next-frame asymmetry: clean skip vs value-swap cone vs plan band |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000002 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| sb_none | 14004ns | 14000ns | 13428ns | 13884ns | 14471ns | base |
| sb_plan | 16729ns | 16567ns | 15828ns | 16537ns | 17469ns | +19.46% |
| sb_value | 167143ns | 161375ns | 157414ns | 160327ns | 182231ns | +1093.56% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| sb_none | 10055ns | 9944ns | 10234ns | base | 0.102 |
| sb_plan | 12364ns | 12233ns | 12596ns | +22.96% | 0.083 |
| sb_value | 162553ns | 152828ns | 177346ns | +1516.62% | 0.006 |

## Performance model

- Peak throughput: **0.103 Gops/s** (sb_none; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| sb_none | 0.103 | 99.7% |
| sb_plan | 0.084 | 81.1% |
| sb_value | 0.007 | 6.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| sb_none | 14004ns | 14004ns | base |
| sb_plan | 16729ns | 16729ns | +19.46% |
| sb_value | 167143ns | 167143ns | +1093.56% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| sb_none | 9975ns | base | --- | [9960, 10017] | --- | --- | --- | --- |
| sb_plan | 12258ns | +2275.0ns (+22.8%) | [+2238, +2312]ns | [12246, 12275] | YES | 0.0000 | 0.0000 | 0 |
| sb_value | 156606ns | +146462.5ns (+1468.3%) | [+144971, +148706]ns | [154862, 158783] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | sb_none | sb_plan | sb_value |
|---|---|---|---|---|
| 1 | 1 | 10021ns | +22.2% | +1469.9% |
| 1 | 2 | 9950ns | +22.9% | +1440.2% |
| 1 | 3 | 9962ns | +23.1% | +1461.6% |
| 2 | 1 | 9971ns | +22.9% | +1659.1% |
| 2 | 2 | 10221ns | +19.9% | +1436.7% |
| 2 | 3 | 9954ns | +25.0% | +1456.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| sb_none | -0.294 | moderate- |
| sb_plan | -0.004 | ok |
| sb_value | -0.149 | ok |

**Consistency summary:**

- **sb_plan**: won 0/6, lost 6/6
- **sb_value**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| sb_none | 5594625.2ns | 10055.1ns | 55639.7% | HIGH |
| sb_plan | 5583666.4ns | 12363.9ns | 45161.1% | HIGH |
| sb_value | 5604126.6ns | 162552.5ns | 3447.6% | HIGH |

## Distribution (algo ns)

```
sb_none (n=18, range 9944.4-10234.0 ns)
   9944.4 |########################################
   9958.9 |########################################
   9973.4 |##########
   9987.9 |####################
  10002.4 |##########
  10016.8 |##########
  10031.3 |
  10045.8 |
  10060.3 |
  10074.8 |
  10089.2 |
  10103.7 |
  10118.2 |
  10132.7 |
  10147.2 |##########
  10161.6 |
  10176.1 |
  10190.6 |
  10205.1 |
  10219.6 |##########
  (1 below, 2 above range)

sb_plan (n=18, range 12233.3-12595.8 ns)
  12233.3 |########################################
  12251.5 |##########################
  12269.6 |#############
  12287.7 |
  12305.8 |
  12324.0 |######
  12342.1 |
  12360.2 |
  12378.3 |
  12396.5 |
  12414.6 |
  12432.7 |######
  12450.8 |
  12469.0 |
  12487.1 |
  12505.2 |
  12523.3 |
  12541.5 |
  12559.6 |
  12577.7 |
  (2 below, 2 above range)

sb_value (n=18, range 152827.8-177345.8 ns)
  152827.8 |##########################
  154053.7 |########################################
  155279.6 |########################################
  156505.5 |########################################
  157731.4 |#############
  158957.3 |#############
  160183.2 |
  161409.1 |
  162635.0 |
  163860.9 |
  165086.8 |
  166312.7 |
  167538.6 |
  168764.5 |#############
  169990.4 |
  171216.3 |
  172442.2 |
  173668.1 |
  174894.0 |#############
  176119.9 |
  (1 below, 2 above range)

```

## Diagnostics

- **sb_none**: bridge=55895.9% of algo (FFI overhead may distort results)
- **sb_plan**: bridge=45462.5% of algo (FFI overhead may distort results)
- **sb_value**: bridge=3561.5% of algo (FFI overhead may distort results)
