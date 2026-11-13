# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (kernel_seq, nway) are a dead heat (<1%)

kernel_seq (83.48 ms) and nway (84.14 ms) differ by 0.80%, inside the noise, even though the wider field spreads 7.6%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_seq** at 83476428.8 ns median (-4.9% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 83476428.8 ns, slowest 89797201.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100350026 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 87033291ns | 87764653ns | 84134805ns | 87056910ns | 88447108ns | base |
| head_tail_dyn | 87523948ns | 87426303ns | 84264519ns | 86468787ns | 90736403ns | +0.56% |
| kernel_nway | 87302056ns | 86577215ns | 82195703ns | 86067847ns | 91706547ns | +0.31% |
| kernel_seq | 83798611ns | 83483291ns | 81380990ns | 82939452ns | 86296159ns | -3.72% |
| nway | 83797111ns | 84148874ns | 81099523ns | 83746272ns | 85222162ns | -3.72% |
| sequential | 86547559ns | 87574889ns | 83269031ns | 86198183ns | 88710886ns | -0.56% |
| std_parallel | 88470921ns | 89802084ns | 83222235ns | 88540599ns | 90990747ns | +1.65% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 87026864ns | 84129631ns | 88440678ns | base | 1.542 |
| head_tail_dyn | 87516414ns | 84259412ns | 90730868ns | +0.56% | 1.534 |
| kernel_nway | 87294837ns | 82190244ns | 91697737ns | +0.31% | 1.538 |
| kernel_seq | 83791989ns | 81373440ns | 86289493ns | -3.72% | 1.602 |
| nway | 83792886ns | 81095446ns | 85218126ns | -3.72% | 1.602 |
| sequential | 86543845ns | 83265606ns | 88707164ns | -0.56% | 1.551 |
| std_parallel | 88465984ns | 83218176ns | 90984638ns | +1.65% | 1.517 |

## Performance model

- Peak throughput: **1.655 Gops/s** (nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.529 | 92.4% |
| head_tail_dyn | 1.535 | 92.8% |
| kernel_nway | 1.550 | 93.7% |
| kernel_seq | 1.608 | 97.1% |
| nway | 1.595 | 96.4% |
| sequential | 1.533 | 92.6% |
| std_parallel | 1.495 | 90.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 87033291ns | 87033291ns | base |
| head_tail_dyn | 87523948ns | 87523948ns | +0.56% |
| kernel_nway | 87302056ns | 87302056ns | +0.31% |
| kernel_seq | 83798611ns | 83798611ns | -3.72% |
| nway | 83797111ns | 83797111ns | -3.72% |
| sequential | 86547559ns | 86547559ns | -0.56% |
| std_parallel | 88470921ns | 88470921ns | +1.65% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 87756856ns | base | --- | [84883057, 88440678] | --- | --- | --- | --- |
| head_tail_dyn | 87415388ns | no significant difference | [-2290031, +3743384]ns | [84402986, 90730868] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 86570809ns | no significant difference | [-2720285, +3834937]ns | [83615966, 91697737] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 83476429ns | no significant difference | [-6337038, +820869]ns | [81610046, 86289493] | no | 0.4375 | 0.2188 | 0 |
| nway | 84144560ns | -3667837.3ns (-4.2%) | [-4280979, -1753118]ns | [82015972, 85218126] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| sequential | 87570966ns | no significant difference | [-2880651, +1909908]ns | [83353406, 88707164] | no | 0.8250 | 0.6875 | 0 |
| std_parallel | 89797202ns | no significant difference | [-1617945, +3479068]ns | [84616112, 90984638] | no | 0.4375 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 86797256ns | +2.2% | +4.4% | -2.7% | -3.9% | +2.2% | +3.8% |
| 1 | 2 | 86336252ns | -2.2% | -3.2% | -1.2% | -2.9% | -0.7% | +2.8% |
| 1 | 3 | 87947084ns | +1.7% | -0.4% | -7.2% | -4.3% | -3.2% | -1.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.346 | moderate- |
| head_tail_dyn | -0.657 | HIGH- (thermal bounce) |
| kernel_nway | -0.546 | HIGH- (thermal bounce) |
| kernel_seq | -0.298 | moderate- |
| nway | -0.016 | ok |
| sequential | -0.085 | ok |
| std_parallel | -0.011 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 3/3, lost 0/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 1/3, lost 2/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 86133916.6ns | 87026864.0ns | 99.0% | HIGH |
| head_tail_dyn | 88153152.2ns | 87516413.8ns | 100.7% | HIGH |
| kernel_nway | 87221160.8ns | 87294837.4ns | 99.9% | HIGH |
| kernel_seq | 85758351.2ns | 83791989.2ns | 102.3% | HIGH |
| nway | 83766941.5ns | 83792886.1ns | 100.0% | HIGH |
| sequential | 86930736.5ns | 86543845.3ns | 100.4% | HIGH |
| std_parallel | 86444408.1ns | 88465983.9ns | 97.7% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 84129630.7-88440678.2 ns)
  84129630.7 |########################################
  84345183.1 |
  84560735.4 |
  84776287.8 |
  84991840.2 |
  85207392.6 |
  85422944.9 |########################################
  85638497.3 |
  85854049.7 |
  86069602.1 |
  86285154.4 |
  86500706.8 |
  86716259.2 |
  86931811.5 |
  87147363.9 |
  87362916.3 |########################################
  87578468.7 |
  87794021.0 |########################################
  88009573.4 |
  88225125.8 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 84259411.9-90730867.8 ns)
  84259411.9 |########################################
  84582984.7 |
  84906557.5 |
  85230130.3 |
  85553703.1 |
  85877275.9 |
  86200848.7 |
  86524421.5 |
  86847994.3 |####################
  87171567.1 |
  87495139.8 |
  87818712.6 |####################
  88142285.4 |
  88465858.2 |
  88789431.0 |
  89113003.8 |
  89436576.6 |####################
  89760149.4 |
  90083722.2 |
  90407295.0 |
  (0 below, 1 above range)

kernel_nway (n=6, range 82190244.0-91697736.7 ns)
  82190244.0 |########################################
  82665618.6 |
  83140993.3 |
  83616367.9 |
  84091742.5 |
  84567117.2 |########################################
  85042491.8 |
  85517866.4 |########################################
  85993241.1 |
  86468615.7 |
  86943990.3 |########################################
  87419365.0 |
  87894739.6 |########################################
  88370114.3 |
  88845488.9 |
  89320863.5 |
  89796238.2 |
  90271612.8 |
  90746987.4 |
  91222362.1 |
  (0 below, 1 above range)

kernel_seq (n=6, range 81373439.8-86289492.7 ns)
  81373439.8 |########################################
  81619242.4 |########################################
  81865045.1 |
  82110847.7 |
  82356650.4 |
  82602453.0 |
  82848255.7 |########################################
  83094058.3 |
  83339860.9 |
  83585663.6 |
  83831466.2 |########################################
  84077268.9 |
  84323071.5 |
  84568874.2 |
  84814676.8 |
  85060479.4 |########################################
  85306282.1 |
  85552084.7 |
  85797887.4 |
  86043690.0 |
  (0 below, 1 above range)

nway (n=6, range 81095445.6-85218126.2 ns)
  81095445.6 |########################################
  81301579.6 |
  81507713.7 |
  81713847.7 |
  81919981.7 |
  82126115.8 |
  82332249.8 |
  82538383.8 |
  82744517.9 |########################################
  82950651.9 |
  83156785.9 |
  83362920.0 |
  83569054.0 |
  83775188.0 |########################################
  83981322.1 |
  84187456.1 |########################################
  84393590.1 |
  84599724.2 |########################################
  84805858.2 |
  85011992.2 |
  (0 below, 1 above range)

sequential (n=6, range 83265605.9-88707163.8 ns)
  83265605.9 |########################################
  83537683.8 |
  83809761.7 |
  84081839.6 |
  84353917.5 |
  84625995.4 |
  84898073.3 |
  85170151.2 |
  85442229.1 |
  85714307.0 |
  85986384.9 |
  86258462.8 |
  86530540.7 |
  86802618.6 |####################
  87074696.5 |
  87346774.4 |
  87618852.3 |
  87890930.2 |
  88163008.1 |########################################
  88435086.0 |
  (0 below, 1 above range)

std_parallel (n=6, range 83218175.8-90984637.7 ns)
  83218175.8 |########################################
  83606498.9 |
  83994822.0 |
  84383145.1 |
  84771468.2 |
  85159791.3 |
  85548114.4 |
  85936437.5 |########################################
  86324760.6 |
  86713083.7 |
  87101406.8 |
  87489729.8 |
  87878052.9 |
  88266376.0 |
  88654699.1 |
  89043022.2 |
  89431345.3 |########################################
  89819668.4 |########################################
  90207991.5 |########################################
  90596314.6 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=98.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=101.2% of algo (FFI overhead may distort results)
- **nway**: bridge=99.8% of algo (FFI overhead may distort results)
- **sequential**: bridge=98.5% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=95.9% of algo (FFI overhead may distort results)
