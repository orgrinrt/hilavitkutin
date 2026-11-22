# fold_heavy, frames of tens of milliseconds, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_nway beats baseline by 50% (significant)

kernel_nway is -216.39 us (50%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.4x slower than the field

sequential (760.34 us) is 3.4x the fastest (220.79 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest kernel_nway (220.79 us) to slowest sequential (760.34 us): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 220791.6 ns median (-49.4% vs baseline)
- 3 variants significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 3.44x (fastest 220791.6 ns, slowest 760343.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_heavy, frames of tens of milliseconds, W = 8 |
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
| head_tail | 437700ns | 437188ns | 427948ns | 436948ns | 443703ns | base |
| head_tail_dyn | 442096ns | 442005ns | 439469ns | 441493ns | 444312ns | +1.00% |
| kernel_nway | 225559ns | 221771ns | 206938ns | 218458ns | 245521ns | -48.47% |
| kernel_seq | 755984ns | 746000ns | 744885ns | 745795ns | 776818ns | +72.72% |
| nway | 242701ns | 232443ns | 231125ns | 232007ns | 264531ns | -44.55% |
| sequential | 768394ns | 760714ns | 749312ns | 759323ns | 791542ns | +75.55% |
| std_parallel | 276910ns | 260880ns | 256656ns | 260188ns | 312120ns | -36.74% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 436957ns | 427250ns | 442964ns | base | 2.400 |
| head_tail_dyn | 441266ns | 437573ns | 443547ns | +0.99% | 2.376 |
| kernel_nway | 224655ns | 206073ns | 244609ns | -48.59% | 4.668 |
| kernel_seq | 755259ns | 744458ns | 775224ns | +72.85% | 1.388 |
| nway | 241797ns | 230500ns | 263297ns | -44.66% | 4.337 |
| sequential | 767906ns | 749104ns | 790911ns | +75.74% | 1.366 |
| std_parallel | 276316ns | 256208ns | 311391ns | -36.76% | 3.795 |

## Performance model

- Peak throughput: **5.088 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.402 | 47.2% |
| head_tail_dyn | 2.375 | 46.7% |
| kernel_nway | 4.749 | 93.3% |
| kernel_seq | 1.406 | 27.6% |
| nway | 4.528 | 89.0% |
| sequential | 1.379 | 27.1% |
| std_parallel | 4.028 | 79.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 437700ns | 437700ns | base |
| head_tail_dyn | 442096ns | 442096ns | +1.00% |
| kernel_nway | 225559ns | 225559ns | -48.47% |
| kernel_seq | 755984ns | 755984ns | +72.72% |
| nway | 242701ns | 242701ns | -44.55% |
| sequential | 768394ns | 768394ns | +75.55% |
| std_parallel | 276910ns | 276910ns | -36.74% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 436542ns | base | --- | [431365, 442964] | --- | --- | --- | --- |
| head_tail_dyn | 441469ns | no significant difference | [-36, +9115]ns | [438781, 443547] | no | 0.2188 | 0.2188 | 0 |
| kernel_nway | 220792ns | -216390.6ns (-49.6%) | [-228047, -192469]ns | [208562, 244609] | YES | 0.0375 | 0.0313 | 0 |
| kernel_seq | 745729ns | +309187.5ns (+70.8%) | [+301859, +343859]ns | [744823, 775224] | YES | 0.0375 | 0.0313 | 0 |
| nway | 231583ns | -203666.6ns (-46.7%) | [-206859, -174953]ns | [230510, 263297] | YES | 0.0375 | 0.0313 | 0 |
| sequential | 760344ns | +321864.6ns (+73.7%) | [+311438, +359547]ns | [752464, 790911] | YES | 0.0375 | 0.0313 | 0 |
| std_parallel | 260333ns | -171031.2ns (-39.2%) | [-179318, -131573]ns | [257224, 311391] | YES | 0.0375 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 442964ns | +0.1% | -45.7% | +68.1% | -40.8% | +70.3% | -29.7% |
| 1 | 2 | 431365ns | +1.7% | -47.9% | +79.7% | -46.5% | +83.4% | -39.6% |
| 1 | 3 | 436542ns | +1.1% | -52.2% | +70.8% | -46.8% | +73.7% | -41.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.463 | moderate- |
| head_tail_dyn | -0.548 | HIGH- (thermal bounce) |
| kernel_nway | -0.000 | ok |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.192 | ok |
| sequential | -0.660 | HIGH- (thermal bounce) |
| std_parallel | -0.138 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 441463.5ns | 436956.6ns | 101.0% | HIGH |
| head_tail_dyn | 443378.5ns | 441265.6ns | 100.5% | HIGH |
| kernel_nway | 224640.6ns | 224654.5ns | 100.0% | HIGH |
| kernel_seq | 756362.9ns | 755258.7ns | 100.1% | HIGH |
| nway | 242357.6ns | 241796.9ns | 100.2% | HIGH |
| sequential | 765569.4ns | 767906.2ns | 99.7% | HIGH |
| std_parallel | 280114.6ns | 276316.0ns | 101.4% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 427250.0-442963.5 ns)
  427250.0 |########################################
  428035.7 |
  428821.4 |
  429607.0 |
  430392.7 |
  431178.4 |
  431964.1 |
  432749.7 |
  433535.4 |
  434321.1 |
  435106.8 |########################################
  435892.5 |########################################
  436678.1 |########################################
  437463.8 |
  438249.5 |
  439035.2 |########################################
  439820.8 |
  440606.5 |
  441392.2 |
  442177.9 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 437572.9-443546.8 ns)
  437572.9 |####################
  437871.6 |
  438170.3 |
  438469.0 |
  438767.7 |
  439066.4 |
  439365.1 |
  439663.8 |
  439962.5 |####################
  440261.2 |####################
  440559.9 |
  440858.6 |
  441157.3 |
  441456.0 |
  441754.7 |
  442053.4 |
  442352.1 |########################################
  442650.8 |
  442949.5 |
  443248.2 |
  (0 below, 1 above range)

kernel_nway (n=6, range 206072.9-244609.4 ns)
  206072.9 |####################
  207999.7 |
  209926.5 |####################
  211853.4 |
  213780.2 |
  215707.0 |
  217633.9 |
  219560.7 |########################################
  221487.5 |
  223414.3 |
  225341.2 |
  227268.0 |####################
  229194.8 |
  231121.6 |
  233048.5 |
  234975.3 |
  236902.1 |
  238828.9 |
  240755.8 |
  242682.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 744458.3-775223.9 ns)
  744458.3 |########################################
  745996.6 |
  747534.9 |
  749073.1 |
  750611.4 |
  752149.7 |
  753688.0 |
  755226.3 |
  756764.6 |
  758302.8 |
  759841.1 |
  761379.4 |
  762917.7 |
  764456.0 |
  765994.3 |
  767532.5 |
  769070.8 |
  770609.1 |
  772147.4 |##########
  773685.7 |
  (0 below, 1 above range)

nway (n=6, range 230500.0-263296.9 ns)
  230500.0 |########################################
  232139.8 |##########
  233779.7 |
  235419.5 |
  237059.4 |
  238699.2 |
  240339.1 |
  241978.9 |
  243618.8 |
  245258.6 |
  246898.5 |
  248538.3 |
  250178.1 |
  251818.0 |
  253457.8 |
  255097.7 |
  256737.5 |
  258377.4 |
  260017.2 |
  261657.1 |
  (0 below, 1 above range)

sequential (n=6, range 749104.2-790911.4 ns)
  749104.2 |####################
  751194.6 |
  753284.9 |
  755375.3 |####################
  757465.6 |
  759556.0 |########################################
  761646.4 |
  763736.7 |
  765827.1 |
  767917.5 |
  770007.8 |
  772098.2 |
  774188.5 |
  776278.9 |
  778369.3 |
  780459.6 |
  782550.0 |
  784640.4 |
  786730.7 |
  788821.1 |####################
  (0 below, 1 above range)

std_parallel (n=6, range 256208.3-311390.6 ns)
  256208.3 |########################################
  258967.4 |########################################
  261726.5 |
  264485.6 |
  267244.8 |
  270003.9 |
  272763.0 |
  275522.1 |
  278281.2 |
  281040.3 |
  283799.4 |
  286558.6 |
  289317.7 |
  292076.8 |
  294835.9 |
  297595.0 |
  300354.1 |
  303113.3 |
  305872.4 |####################
  308631.5 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.4% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.7% of algo (FFI overhead may distort results)
- **nway**: bridge=101.3% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.4% of algo (FFI overhead may distort results)
