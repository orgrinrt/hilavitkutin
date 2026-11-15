# append, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 105% faster than the next best (head_tail_dyn)

sequential (12.35 us) leads head_tail_dyn (25.27 us) by 105%, a clear separation rather than a photo finish. CV 36.7%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential beats baseline by 44% (significant)

sequential is -11.45 us (44%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 12.9x slower than the field

std_parallel (158.76 us) is 12.9x the fastest (12.35 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Wide spread: slowest is 12.9x the fastest

Fastest sequential (12.35 us) to slowest std_parallel (158.76 us): 12.9x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### nway is inconsistent: worst-20% is 2.0x its best-20%

nway's best 20% of batches run at 66.73 us but its worst 20% at 130.33 us (2.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 12348.0 ns median (-52.7% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 12.86x (fastest 12348.0 ns, slowest 158758.1 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 27665ns | 27058ns | 22973ns | 26029ns | 32464ns | base |
| head_tail_dyn | 27756ns | 26094ns | 22841ns | 25543ns | 33534ns | +0.33% |
| kernel_nway | 73967ns | 71633ns | 60930ns | 67716ns | 89862ns | +167.37% |
| kernel_seq | 28305ns | 27677ns | 25375ns | 27024ns | 31691ns | +2.31% |
| nway | 97702ns | 92623ns | 67226ns | 85167ns | 131745ns | +253.17% |
| sequential | 15610ns | 12632ns | 11325ns | 12872ns | 21860ns | -43.57% |
| std_parallel | 165680ns | 159331ns | 138639ns | 157592ns | 191332ns | +498.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 27060ns | 22468ns | 31791ns | base | 0.151 |
| head_tail_dyn | 27221ns | 22456ns | 32965ns | +0.60% | 0.150 |
| kernel_nway | 73336ns | 60543ns | 89019ns | +171.01% | 0.056 |
| kernel_seq | 27498ns | 24777ns | 30743ns | +1.62% | 0.149 |
| nway | 96877ns | 66733ns | 130327ns | +258.01% | 0.042 |
| sequential | 15275ns | 11061ns | 21429ns | -43.55% | 0.268 |
| std_parallel | 164941ns | 138177ns | 190577ns | +509.54% | 0.025 |

## Performance model

- Peak throughput: **0.370 Gops/s** (sequential; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.157 | 42.3% |
| head_tail_dyn | 0.162 | 43.8% |
| kernel_nway | 0.058 | 15.5% |
| kernel_seq | 0.153 | 41.2% |
| nway | 0.045 | 12.0% |
| sequential | 0.332 | 89.6% |
| std_parallel | 0.026 | 7.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 27665ns | 27665ns | base |
| head_tail_dyn | 27756ns | 27756ns | +0.33% |
| kernel_nway | 73967ns | 73967ns | +167.37% |
| kernel_seq | 28305ns | 28305ns | +2.31% |
| nway | 97702ns | 97702ns | +253.17% |
| sequential | 15610ns | 15610ns | -43.57% |
| std_parallel | 165680ns | 165680ns | +498.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 26131ns | base | --- | [23861, 29171] | --- | --- | --- | --- |
| head_tail_dyn | 25271ns | no significant difference | [-4278, +5636]ns | [23493, 29347] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 71141ns | +43916.8ns (+168.1%) | [+33798, +60469]ns | [61900, 83566] | YES | 0.0000 | 0.0000 | 0 |
| kernel_seq | 26857ns | no significant difference | [-3008, +4122]ns | [25333, 27956] | no | 1.0000 | 1.0000 | 0 |
| nway | 91965ns | +58441.3ns (+223.6%) | [+50985, +73794]ns | [75154, 102350] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 12348ns | -11448.6ns (-43.8%) | [-13430, -10487]ns | [11913, 19512] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 158758ns | +135300.3ns (+517.8%) | [+125997, +151266]ns | [148907, 185214] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 22684ns | +28.6% | +281.3% | +22.3% | +220.4% | -46.8% | +599.9% |
| 1 | 2 | 26131ns | -8.2% | +172.7% | -0.8% | +273.7% | -55.3% | +440.0% |
| 1 | 3 | 31705ns | -26.9% | +92.1% | -17.0% | +229.1% | -32.0% | +489.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.012 | ok |
| head_tail_dyn | -0.099 | ok |
| kernel_nway | -0.008 | ok |
| kernel_seq | -0.295 | moderate- |
| nway | -0.067 | ok |
| sequential | -0.187 | ok |
| std_parallel | -0.419 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 0/3, lost 3/3
- **kernel_seq**: won 2/3, lost 1/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 49306.3ns | 27059.7ns | 182.2% | HIGH |
| head_tail_dyn | 49138.8ns | 27221.2ns | 180.5% | HIGH |
| kernel_nway | 97060.5ns | 73335.8ns | 132.4% | HIGH |
| kernel_seq | 49032.4ns | 27498.4ns | 178.3% | HIGH |
| nway | 129779.8ns | 96877.3ns | 134.0% | HIGH |
| sequential | 37846.6ns | 15274.9ns | 247.8% | HIGH |
| std_parallel | 193259.7ns | 164940.8ns | 117.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 22467.7-31790.6 ns)
  22467.7 |########################################
  22933.8 |
  23400.0 |#############
  23866.1 |#############
  24332.3 |#############
  24798.4 |
  25264.6 |#############
  25730.7 |#############
  26196.9 |#############
  26663.0 |
  27129.2 |
  27595.3 |#############
  28061.5 |
  28527.6 |#############
  28993.8 |##########################
  29459.9 |
  29926.1 |#############
  30392.2 |
  30858.4 |
  31324.5 |
  (1 below, 3 above range)

head_tail_dyn (n=18, range 22456.4-32965.0 ns)
  22456.4 |########################################
  22981.8 |#############
  23507.3 |##########################
  24032.7 |#############
  24558.1 |#############
  25083.5 |
  25609.0 |#############
  26134.4 |
  26659.8 |
  27185.2 |
  27710.7 |#############
  28236.1 |##########################
  28761.5 |
  29287.0 |
  29812.4 |########################################
  30337.8 |
  30863.2 |
  31388.7 |
  31914.1 |
  32439.5 |
  (1 below, 2 above range)

kernel_nway (n=18, range 60542.6-89019.3 ns)
  60542.6 |########################################
  61966.4 |####################
  63390.3 |
  64814.1 |
  66237.9 |
  67661.8 |##########
  69085.6 |##########
  70509.5 |
  71933.3 |##############################
  73357.1 |
  74781.0 |
  76204.8 |
  77628.6 |
  79052.5 |
  80476.3 |
  81900.1 |
  83324.0 |####################
  84747.8 |##########
  86171.6 |
  87595.5 |##########
  (1 below, 2 above range)

kernel_seq (n=18, range 24776.5-30743.3 ns)
  24776.5 |########################################
  25074.9 |
  25373.2 |####################
  25671.5 |
  25969.9 |##########
  26268.2 |
  26566.6 |##########
  26864.9 |##########
  27163.2 |##########
  27461.6 |##########
  27759.9 |##########
  28058.2 |##########
  28356.6 |####################
  28654.9 |
  28953.3 |
  29251.6 |##########
  29549.9 |
  29848.3 |
  30146.6 |
  30444.9 |
  (1 below, 1 above range)

nway (n=18, range 66733.3-130327.1 ns)
  66733.3 |####################
  69913.0 |########################################
  73092.7 |########################################
  76272.4 |####################
  79452.1 |####################
  82631.8 |
  85811.5 |
  88991.1 |####################
  92170.8 |########################################
  95350.5 |
  98530.2 |########################################
  101709.9 |####################
  104889.6 |####################
  108069.3 |####################
  111249.0 |
  114428.6 |
  117608.3 |
  120788.0 |####################
  123967.7 |
  127147.4 |
  (1 below, 1 above range)

sequential (n=18, range 11061.4-21429.3 ns)
  11061.4 |##########################
  11579.8 |########################################
  12098.2 |########################################
  12616.6 |
  13135.0 |
  13653.4 |#############
  14171.8 |
  14690.2 |#############
  15208.6 |
  15727.0 |
  16245.3 |
  16763.7 |
  17282.1 |
  17800.5 |
  18318.9 |
  18837.3 |
  19355.7 |#############
  19874.1 |#############
  20392.5 |
  20910.9 |
  (2 below, 4 above range)

std_parallel (n=18, range 138176.7-190577.0 ns)
  138176.7 |####################
  140796.7 |
  143416.7 |########################################
  146036.7 |
  148656.8 |
  151276.8 |####################
  153896.8 |########################################
  156516.8 |####################
  159136.8 |########################################
  161756.8 |
  164376.8 |
  166996.9 |
  169616.9 |
  172236.9 |
  174856.9 |
  177476.9 |####################
  180096.9 |
  182716.9 |####################
  185337.0 |########################################
  187957.0 |####################
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=184.9% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=20.2% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=189.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=125.8% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=179.5% of algo (FFI overhead may distort results)
- **nway**: CV=39.8% (high variance, measurements may be unstable)
- **nway**: bridge=126.3% of algo (FFI overhead may distort results)
- **sequential**: CV=29.6% (high variance, measurements may be unstable)
- **sequential**: bridge=285.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=118.1% of algo (FFI overhead may distort results)
