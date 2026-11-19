# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (5.73 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 4.79 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

## Key findings

- **Fastest: per_core** at 4785246.6 ns median (-16.5% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.20x (fastest 4785246.6 ns, slowest 5728395.5 ns)

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
| grouped | 5768269ns | 5730956ns | 5073519ns | 5623133ns | 6333348ns | base |
| per_core | 4981188ns | 4787845ns | 4605190ns | 4784781ns | 5463798ns | -13.65% |
| sequential | 5104103ns | 4884694ns | 4748389ns | 4855818ns | 5654389ns | -11.51% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 5765102ns | 5070717ns | 6329674ns | base | 0.182 |
| per_core | 4978514ns | 4603008ns | 5460824ns | -13.64% | 0.211 |
| sequential | 5101246ns | 4745977ns | 5651121ns | -11.52% | 0.206 |

## Performance model

- Peak throughput: **0.228 Gops/s** (per_core; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.183 | 80.4% |
| per_core | 0.219 | 96.2% |
| sequential | 0.215 | 94.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 5768269ns | 5768269ns | base |
| per_core | 4981188ns | 4981188ns | -13.65% |
| sequential | 5104103ns | 5104103ns | -11.51% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 5728395ns | base | --- | [5237741, 6266673] | --- | --- | --- | --- |
| per_core | 4785247ns | -843353.1ns (-14.7%) | [-984028, -482812]ns | [4665739, 5310726] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 4881939ns | -439599.9ns (-7.7%) | [-1317667, -296784]ns | [4788637, 5316728] | YES | 0.0075 | 0.0075 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 6343680ns | -15.0% | -24.0% |
| 1 | 2 | 5198444ns | -9.3% | -7.1% |
| 1 | 3 | 5657170ns | -16.5% | +2.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.430 | moderate- |
| per_core | -0.171 | ok |
| sequential | -0.163 | ok |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 5900280.1ns | 5765101.5ns | 102.3% | HIGH |
| per_core | 4958717.2ns | 4978513.8ns | 99.6% | HIGH |
| sequential | 5137149.3ns | 5101246.4ns | 100.7% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 5070717.3-6329673.9 ns)
  5070717.3 |####################
  5133665.2 |####################
  5196613.0 |########################################
  5259560.8 |
  5322508.6 |
  5385456.5 |
  5448404.3 |####################
  5511352.1 |
  5574300.0 |
  5637247.8 |########################################
  5700195.6 |
  5763143.4 |####################
  5826091.3 |
  5889039.1 |
  5951986.9 |
  6014934.7 |
  6077882.6 |####################
  6140830.4 |
  6203778.2 |########################################
  6266726.1 |####################
  (2 below, 4 above range)

per_core (n=18, range 4603008.4-5460823.7 ns)
  4603008.4 |##########################
  4645899.2 |########################################
  4688789.9 |
  4731680.7 |##########################
  4774571.5 |##########################
  4817462.2 |#############
  4860353.0 |
  4903243.8 |
  4946134.5 |
  4989025.3 |
  5031916.1 |
  5074806.8 |#############
  5117697.6 |
  5160588.4 |
  5203479.1 |#############
  5246369.9 |
  5289260.7 |
  5332151.4 |
  5375042.2 |##########################
  5417933.0 |
  (1 below, 3 above range)

sequential (n=18, range 4745977.2-5651120.6 ns)
  4745977.2 |########################################
  4791234.4 |####################
  4836491.5 |####################
  4881748.7 |####################
  4927005.9 |##########
  4972263.0 |
  5017520.2 |##########
  5062777.4 |
  5108034.5 |
  5153291.7 |
  5198548.9 |
  5243806.1 |
  5289063.2 |
  5334320.4 |
  5379577.6 |
  5424834.7 |
  5470091.9 |
  5515349.1 |
  5560606.2 |##########
  5605863.4 |
  (1 below, 4 above range)

```

## Diagnostics

- **grouped**: bridge=103.6% of algo (FFI overhead may distort results)
- **per_core**: bridge=99.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.8% of algo (FFI overhead may distort results)
