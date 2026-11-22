# fold_heavy, frames of tens of milliseconds, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.55)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.55, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (760.83 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Whole field within 2.3% of the fastest

All 7 variants sit between 760.83 us and 778.42 us - a 2.3% band - though some paired differences are still significant.

_Why it matters:_ Small but real gaps: worth taking only where this path is hot enough that a few percent compounds.

## Key findings

- **Baseline (head_tail) is the fastest** at 760833.3 ns median
- 4 variants significantly slower than baseline
- Spread: 1.02x (fastest 760833.3 ns, slowest 778416.7 ns)

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
| head_tail | 762896ns | 761146ns | 757698ns | 760517ns | 769062ns | base |
| head_tail_dyn | 790576ns | 767859ns | 764500ns | 766858ns | 839193ns | +3.63% |
| kernel_nway | 764688ns | 764380ns | 758594ns | 763788ns | 769083ns | +0.23% |
| kernel_seq | 761538ns | 761672ns | 757521ns | 760319ns | 765375ns | -0.18% |
| nway | 778049ns | 775714ns | 771490ns | 774545ns | 786583ns | +1.99% |
| sequential | 777326ns | 773536ns | 765552ns | 771503ns | 791948ns | +1.89% |
| std_parallel | 780939ns | 778979ns | 775073ns | 778462ns | 787589ns | +2.37% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 762604ns | 757500ns | 768760ns | base | 1.375 |
| head_tail_dyn | 788885ns | 763448ns | 835172ns | +3.45% | 1.329 |
| kernel_nway | 764372ns | 758271ns | 768760ns | +0.23% | 1.372 |
| kernel_seq | 761194ns | 757146ns | 765057ns | -0.18% | 1.378 |
| nway | 776672ns | 770865ns | 783599ns | +1.84% | 1.350 |
| sequential | 776851ns | 765281ns | 791297ns | +1.87% | 1.350 |
| std_parallel | 780455ns | 774583ns | 787094ns | +2.34% | 1.344 |

## Performance model

- Peak throughput: **1.385 Gops/s** (kernel_seq; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.378 | 99.5% |
| head_tail_dyn | 1.366 | 98.6% |
| kernel_nway | 1.372 | 99.1% |
| kernel_seq | 1.377 | 99.4% |
| nway | 1.353 | 97.7% |
| sequential | 1.356 | 97.9% |
| std_parallel | 1.347 | 97.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 762896ns | 762896ns | base |
| head_tail_dyn | 790576ns | 790576ns | +3.63% |
| kernel_nway | 764688ns | 764688ns | +0.23% |
| kernel_seq | 761538ns | 761538ns | -0.18% |
| nway | 778049ns | 778049ns | +1.99% |
| sequential | 777326ns | 777326ns | +1.89% |
| std_parallel | 780939ns | 780939ns | +2.37% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 760833ns | base | --- | [758219, 768760] | --- | --- | --- | --- |
| head_tail_dyn | 767583ns | +7203.1ns (+0.9%) | [+1495, +70146]ns | [763901, 835172] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| kernel_nway | 764021ns | no significant difference | [-4297, +6859]ns | [760333, 768760] | no | 0.8250 | 0.6875 | 0 |
| kernel_seq | 761359ns | no significant difference | [-10068, +6839]ns | [757167, 765057] | no | 1.0000 | 1.0000 | 0 |
| nway | 775099ns | +15416.7ns (+2.0%) | [+4021, +22766]ns | [771318, 783599] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| sequential | 773068ns | +12234.4ns (+1.6%) | [+7969, +22536]ns | [766188, 791297] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 778417ns | +17968.8ns (+2.4%) | [+15021, +20563]ns | [775854, 787094] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 759833ns | +0.5% | +0.5% | +0.5% | +2.3% | +1.4% | +2.3% |
| 1 | 2 | 759219ns | +7.5% | +0.3% | +0.3% | +2.7% | +1.2% | +2.3% |
| 1 | 3 | 768760ns | +2.3% | -0.1% | -1.3% | +0.5% | +2.9% | +2.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.201 | moderate- |
| head_tail_dyn | -0.547 | HIGH- (thermal bounce) |
| kernel_nway | -0.410 | moderate- |
| kernel_seq | -0.007 | ok |
| nway | -0.366 | moderate- |
| sequential | -0.214 | moderate- |
| std_parallel | -0.191 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 764599.0ns | 762604.2ns | 100.3% | HIGH |
| head_tail_dyn | 795048.6ns | 788885.4ns | 100.8% | HIGH |
| kernel_nway | 765501.7ns | 764371.5ns | 100.1% | HIGH |
| kernel_seq | 760640.6ns | 761194.4ns | 99.9% | HIGH |
| nway | 779062.5ns | 776671.9ns | 100.3% | HIGH |
| sequential | 771732.6ns | 776850.7ns | 99.3% | HIGH |
| std_parallel | 779394.1ns | 780454.9ns | 99.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 757500.0-768760.4 ns)
  757500.0 |########################################
  758063.0 |
  758626.0 |########################################
  759189.1 |
  759752.1 |
  760315.1 |########################################
  760878.1 |########################################
  761441.1 |
  762004.2 |
  762567.2 |
  763130.2 |
  763693.2 |
  764256.2 |
  764819.3 |
  765382.3 |
  765945.3 |
  766508.3 |
  767071.3 |
  767634.4 |
  768197.4 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 763447.9-835171.9 ns)
  763447.9 |########################################
  767034.1 |########################################
  770620.3 |
  774206.5 |
  777792.7 |
  781378.9 |
  784965.1 |
  788551.3 |
  792137.5 |
  795723.7 |
  799309.9 |
  802896.1 |####################
  806482.3 |
  810068.5 |
  813654.7 |
  817240.9 |
  820827.1 |
  824413.3 |
  827999.5 |
  831585.7 |
  (0 below, 1 above range)

kernel_nway (n=6, range 758270.8-768760.4 ns)
  758270.8 |########################################
  758795.3 |
  759319.8 |
  759844.2 |
  760368.7 |
  760893.2 |
  761417.7 |
  761942.2 |########################################
  762466.7 |
  762991.1 |########################################
  763515.6 |
  764040.1 |
  764564.6 |########################################
  765089.1 |########################################
  765613.6 |
  766138.0 |
  766662.5 |
  767187.0 |
  767711.5 |
  768236.0 |
  (0 below, 1 above range)

kernel_seq (n=6, range 757145.8-765057.3 ns)
  757145.8 |########################################
  757541.4 |
  757937.0 |
  758332.5 |
  758728.1 |
  759123.7 |
  759519.2 |
  759914.8 |####################
  760310.4 |
  760706.0 |
  761101.6 |
  761497.1 |
  761892.7 |
  762288.3 |####################
  762683.9 |
  763079.4 |
  763475.0 |
  763870.6 |
  764266.2 |####################
  764661.7 |
  (0 below, 1 above range)

nway (n=6, range 770864.6-783598.9 ns)
  770864.6 |########################################
  771501.3 |########################################
  772138.0 |
  772774.8 |
  773411.5 |
  774048.2 |
  774684.9 |########################################
  775321.6 |########################################
  775958.3 |
  776595.1 |
  777231.8 |
  777868.5 |
  778505.2 |
  779141.9 |
  779778.6 |
  780415.4 |
  781052.1 |
  781688.8 |
  782325.5 |
  782962.2 |########################################
  (0 below, 1 above range)

sequential (n=6, range 765281.2-791296.9 ns)
  765281.2 |########################################
  766582.0 |########################################
  767882.8 |
  769183.6 |
  770484.3 |
  771785.1 |########################################
  773085.9 |########################################
  774386.7 |
  775687.5 |
  776988.3 |
  778289.0 |
  779589.8 |
  780890.6 |
  782191.4 |
  783492.2 |########################################
  784793.0 |
  786093.8 |
  787394.5 |
  788695.3 |
  789996.1 |
  (0 below, 1 above range)

std_parallel (n=6, range 774583.3-787093.8 ns)
  774583.3 |####################
  775208.8 |
  775834.3 |
  776459.9 |
  777085.4 |########################################
  777710.9 |
  778336.4 |
  778962.0 |####################
  779587.5 |
  780213.0 |
  780838.5 |
  781464.0 |
  782089.6 |
  782715.1 |
  783340.6 |
  783966.1 |
  784591.7 |
  785217.2 |
  785842.7 |####################
  786468.2 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=99.9% of algo (FFI overhead may distort results)
- **nway**: bridge=100.6% of algo (FFI overhead may distort results)
- **sequential**: bridge=99.4% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=100.2% of algo (FFI overhead may distort results)
