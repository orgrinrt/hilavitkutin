# fold_heavy, frames of tens of milliseconds, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_nway, head_tail) are a dead heat (<1%)

kernel_nway (744.77 us) and head_tail (744.89 us) differ by 0.02%, inside the noise, even though the wider field spreads 3.2%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail shows alternating (throttle bounce) (autocorr -0.67)

head_tail's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole field within 3.2% of the fastest

All 7 variants sit between 744.77 us and 768.79 us - a 3.2% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Fastest: kernel_nway** at 744765.6 ns median (-0.0% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.03x (fastest 744765.6 ns, slowest 768786.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 751377ns | 745172ns | 743917ns | 744903ns | 764818ns | base |
| head_tail_dyn | 756469ns | 748406ns | 743260ns | 747288ns | 776844ns | +0.68% |
| kernel_nway | 745090ns | 745188ns | 744573ns | 745097ns | 745339ns | -0.84% |
| kernel_seq | 749682ns | 745391ns | 743719ns | 744872ns | 759880ns | -0.23% |
| nway | 771774ns | 769260ns | 754656ns | 764878ns | 790677ns | +2.71% |
| sequential | 762710ns | 753599ns | 746927ns | 751413ns | 787547ns | +1.51% |
| std_parallel | 773905ns | 761688ns | 761021ns | 761514ns | 798932ns | +3.00% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 751023ns | 743615ns | 764276ns | base | 1.396 |
| head_tail_dyn | 756116ns | 743062ns | 776260ns | +0.68% | 1.387 |
| kernel_nway | 744783ns | 744271ns | 745146ns | -0.83% | 1.408 |
| kernel_seq | 749295ns | 743396ns | 759250ns | -0.23% | 1.399 |
| nway | 771257ns | 754417ns | 789911ns | +2.69% | 1.360 |
| sequential | 762281ns | 746719ns | 786792ns | +1.50% | 1.376 |
| std_parallel | 773465ns | 760625ns | 798260ns | +2.99% | 1.356 |

## Performance model

- Peak throughput: **1.411 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.408 | 99.8% |
| head_tail_dyn | 1.402 | 99.3% |
| kernel_nway | 1.408 | 99.8% |
| kernel_seq | 1.407 | 99.7% |
| nway | 1.364 | 96.7% |
| sequential | 1.392 | 98.6% |
| std_parallel | 1.377 | 97.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 751377ns | 751377ns | base |
| head_tail_dyn | 756469ns | 756469ns | +0.68% |
| kernel_nway | 745090ns | 745090ns | -0.84% |
| kernel_seq | 749682ns | 749682ns | -0.23% |
| nway | 771774ns | 771774ns | +2.71% |
| sequential | 762710ns | 762710ns | +1.51% |
| std_parallel | 773905ns | 773905ns | +3.00% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 744891ns | base | --- | [743901, 764276] | --- | --- | --- | --- |
| head_tail_dyn | 748167ns | no significant difference | [-16109, +31370]ns | [743922, 776260] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 744766ns | no significant difference | [-19354, +781]ns | [744438, 745146] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 745172ns | no significant difference | [-20438, +15349]ns | [743464, 759250] | no | 1.0000 | 1.0000 | 0 |
| nway | 768786ns | +16536.5ns (+2.2%) | [+10182, +33984]ns | [755073, 789911] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| sequential | 753333ns | no significant difference | [-16151, +41901]ns | [746719, 786792] | no | 1.0000 | 0.6875 | 0 |
| std_parallel | 761448ns | no significant difference | [-3333, +54359]ns | [760688, 798260] | no | 1.0000 | 0.6875 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 743901ns | +0.0% | +0.1% | +2.1% | +3.3% | +1.1% | +7.3% |
| 1 | 2 | 764276ns | -2.1% | -2.5% | -2.7% | +3.4% | -2.1% | -0.4% |
| 1 | 3 | 744891ns | +4.2% | -0.0% | -0.0% | +1.4% | +5.6% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.665 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.102 | ok |
| kernel_nway | -0.624 | HIGH- (thermal bounce) |
| kernel_seq | -0.200 | ok |
| nway | -0.565 | HIGH- (thermal bounce) |
| sequential | -0.221 | moderate- |
| std_parallel | -0.170 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 1/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 1/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 1/3, lost 2/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 750498.3ns | 751022.6ns | 99.9% | HIGH |
| head_tail_dyn | 755394.1ns | 756116.3ns | 99.9% | HIGH |
| kernel_nway | 744942.7ns | 744783.0ns | 100.0% | HIGH |
| kernel_seq | 750513.9ns | 749295.1ns | 100.2% | HIGH |
| nway | 771036.5ns | 771257.0ns | 100.0% | HIGH |
| sequential | 761595.5ns | 762281.2ns | 99.9% | HIGH |
| std_parallel | 776149.3ns | 773465.3ns | 100.3% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 743614.6-764276.1 ns)
  743614.6 |########################################
  744647.7 |#############
  745680.7 |
  746713.8 |
  747746.9 |
  748780.0 |
  749813.0 |
  750846.1 |
  751879.2 |
  752912.3 |
  753945.3 |
  754978.4 |
  756011.5 |
  757044.5 |
  758077.6 |
  759110.7 |
  760143.8 |
  761176.8 |#############
  762209.9 |
  763243.0 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 743062.5-776260.4 ns)
  743062.5 |####################
  744722.4 |########################################
  746382.3 |
  748042.2 |
  749702.1 |####################
  751362.0 |
  753021.9 |
  754681.8 |
  756341.7 |
  758001.6 |
  759661.5 |
  761321.4 |
  762981.3 |
  764641.2 |
  766301.1 |
  767961.0 |####################
  769620.9 |
  771280.8 |
  772940.7 |
  774600.6 |
  (0 below, 1 above range)

kernel_nway (n=6, range 744270.8-745145.9 ns)
  744270.8 |########################################
  744314.6 |
  744358.3 |
  744402.1 |
  744445.8 |
  744489.6 |
  744533.3 |
  744577.1 |########################################
  744620.8 |########################################
  744664.6 |
  744708.3 |
  744752.1 |
  744795.8 |
  744839.6 |
  744883.3 |########################################
  744927.1 |
  744970.8 |
  745014.6 |
  745058.3 |########################################
  745102.1 |
  (0 below, 1 above range)

kernel_seq (n=6, range 743395.8-759250.0 ns)
  743395.8 |########################################
  744188.5 |####################
  744981.2 |
  745773.9 |####################
  746566.6 |
  747359.4 |
  748152.1 |
  748944.8 |
  749737.5 |
  750530.2 |####################
  751322.9 |
  752115.6 |
  752908.3 |
  753701.0 |
  754493.7 |
  755286.4 |
  756079.2 |
  756871.9 |
  757664.6 |
  758457.3 |
  (0 below, 1 above range)

nway (n=6, range 754416.7-789911.4 ns)
  754416.7 |########################################
  756191.4 |
  757966.2 |####################
  759740.9 |
  761515.6 |
  763290.4 |
  765065.1 |
  766839.9 |
  768614.6 |
  770389.3 |
  772164.1 |
  773938.8 |
  775713.5 |
  777488.3 |
  779263.0 |####################
  781037.8 |
  782812.5 |
  784587.2 |####################
  786362.0 |
  788136.7 |
  (0 below, 1 above range)

sequential (n=6, range 746718.8-786791.7 ns)
  746718.8 |########################################
  748722.4 |####################
  750726.1 |
  752729.7 |
  754733.4 |
  756737.0 |####################
  758740.7 |
  760744.3 |
  762747.9 |
  764751.6 |
  766755.2 |
  768758.9 |
  770762.5 |
  772766.2 |
  774769.8 |
  776773.4 |####################
  778777.1 |
  780780.7 |
  782784.4 |
  784788.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 760625.0-798260.4 ns)
  760625.0 |########################################
  762506.8 |
  764388.5 |
  766270.3 |
  768152.1 |
  770033.8 |
  771915.6 |
  773797.4 |
  775679.2 |
  777560.9 |
  779442.7 |
  781324.5 |
  783206.2 |
  785088.0 |
  786969.8 |##########
  788851.6 |
  790733.3 |
  792615.1 |
  794496.9 |
  796378.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.7% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.4% of algo (FFI overhead may distort results)
