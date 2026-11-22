# fold_light, W = 2

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail shows alternating (throttle bounce) (autocorr -0.56)

head_tail's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### No variant beats the baseline (head_tail)

The baseline head_tail is the fastest (72.28 us median); no rival improves on it (all deltas are >= 0).

_Why it matters:_ When nothing beats the baseline, the current choice stands; the contenders cost speed for whatever else they buy.

### Two tiers: {head_tail, kernel_nway, nway, std_parallel, kernel_seq, sequential} vs {head_tail_dyn} (38% apart)

The field splits into a fast tier {head_tail, kernel_nway, nway, std_parallel, kernel_seq, sequential} and a slow tier {head_tail_dyn} with a 38% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Baseline (head_tail) is the fastest** at 72275.0 ns median
- 5 variants significantly slower than baseline
- Spread: 1.90x (fastest 72275.0 ns, slowest 137518.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 2 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 74325ns | 72900ns | 71458ns | 72848ns | 77974ns | base |
| head_tail_dyn | 138366ns | 137871ns | 135164ns | 137680ns | 140996ns | +86.16% |
| kernel_nway | 72595ns | 73825ns | 65919ns | 73489ns | 74590ns | -2.33% |
| kernel_seq | 95595ns | 95292ns | 94642ns | 95253ns | 96585ns | +28.62% |
| nway | 79063ns | 78515ns | 77660ns | 78440ns | 80699ns | +6.37% |
| sequential | 100096ns | 99892ns | 98921ns | 99772ns | 101168ns | +34.67% |
| std_parallel | 85984ns | 84700ns | 76465ns | 82903ns | 95365ns | +15.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 73558ns | 70817ns | 76956ns | base | 14.255 |
| head_tail_dyn | 137865ns | 134601ns | 140370ns | +87.42% | 7.606 |
| kernel_nway | 71868ns | 65342ns | 73862ns | -2.30% | 14.590 |
| kernel_seq | 95324ns | 94454ns | 96165ns | +29.59% | 11.000 |
| nway | 78431ns | 77049ns | 80026ns | +6.62% | 13.369 |
| sequential | 99756ns | 98721ns | 100563ns | +35.62% | 10.511 |
| std_parallel | 85576ns | 76065ns | 94959ns | +16.34% | 12.253 |

## Performance model

- Peak throughput: **16.048 Gops/s** (kernel_nway; best 20% batches)
- Ops per call: 1048576

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 14.508 | 90.4% |
| head_tail_dyn | 7.625 | 47.5% |
| kernel_nway | 14.358 | 89.5% |
| kernel_seq | 11.025 | 68.7% |
| nway | 13.462 | 83.9% |
| sequential | 10.520 | 65.6% |
| std_parallel | 12.441 | 77.5% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 74325ns | 74325ns | base |
| head_tail_dyn | 138366ns | 138366ns | +86.16% |
| kernel_nway | 72595ns | 72595ns | -2.33% |
| kernel_seq | 95595ns | 95595ns | +28.62% |
| nway | 79063ns | 79063ns | +6.37% |
| sequential | 100096ns | 100096ns | +34.67% |
| std_parallel | 85984ns | 85984ns | +15.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 72275ns | base | --- | [72027, 73392] | --- | --- | --- | --- |
| head_tail_dyn | 137519ns | +65156.3ns (+90.2%) | [+63485, +67031]ns | [136906, 138850] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 73029ns | no significant difference | [-3110, +1233]ns | [72458, 73329] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 95110ns | +22685.4ns (+31.4%) | [+22017, +23271]ns | [94898, 95821] | YES | 0.0000 | 0.0000 | 0 |
| nway | 77890ns | +5710.4ns (+7.9%) | [+4552, +6229]ns | [77633, 78554] | YES | 0.0016 | 0.0013 | 0 |
| sequential | 99671ns | +27458.3ns (+38.0%) | [+26038, +27900]ns | [99210, 100248] | YES | 0.0000 | 0.0000 | 0 |
| std_parallel | 84283ns | +10718.8ns (+14.8%) | [+6633, +16673]ns | [79573, 90367] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 72038ns | +92.3% | +1.5% | +32.2% | +8.1% | +38.6% | +17.0% |
| 1 | 2 | 73627ns | +86.5% | -0.7% | +29.4% | +5.9% | +35.7% | +14.1% |
| 1 | 3 | 72665ns | +87.6% | +0.1% | +30.7% | +7.6% | +37.0% | +15.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.564 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.001 | ok |
| kernel_nway | -0.147 | ok |
| kernel_seq | -0.234 | moderate- |
| nway | -0.035 | ok |
| sequential | -0.277 | moderate- |
| std_parallel | -0.000 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 140431.2ns | 73558.1ns | 190.9% | HIGH |
| head_tail_dyn | 137571.5ns | 137865.3ns | 99.8% | HIGH |
| kernel_nway | 140131.3ns | 71867.6ns | 195.0% | HIGH |
| kernel_seq | 95419.4ns | 95324.3ns | 100.1% | HIGH |
| nway | 149562.0ns | 78430.6ns | 190.7% | HIGH |
| sequential | 100103.2ns | 99756.0ns | 100.3% | HIGH |
| std_parallel | 132584.7ns | 85576.4ns | 154.9% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 70816.7-76956.2 ns)
  70816.7 |
  71123.6 |
  71430.6 |##########
  71737.6 |##############################
  72044.6 |########################################
  72351.6 |##########
  72658.5 |##########
  72965.5 |
  73272.5 |####################
  73579.5 |
  73886.4 |
  74193.4 |
  74500.4 |
  74807.4 |####################
  75114.4 |
  75421.3 |
  75728.3 |
  76035.3 |
  76342.3 |
  76649.3 |
  (2 below, 2 above range)

head_tail_dyn (n=18, range 134601.4-140370.1 ns)
  134601.4 |
  134889.8 |
  135178.2 |
  135466.7 |#############
  135755.1 |#############
  136043.6 |
  136332.0 |
  136620.4 |##########################
  136908.9 |##########################
  137197.3 |##########################
  137485.8 |##########################
  137774.2 |
  138062.6 |#############
  138351.1 |
  138639.5 |#############
  138927.9 |########################################
  139216.4 |
  139504.8 |
  139793.3 |
  140081.7 |
  (1 below, 2 above range)

kernel_nway (n=18, range 65341.7-73861.8 ns)
  65341.7 |
  65767.7 |
  66193.7 |
  66619.7 |
  67045.7 |
  67471.7 |
  67897.7 |
  68323.7 |
  68749.7 |
  69175.7 |
  69601.7 |######
  70027.7 |
  70453.7 |
  70879.8 |######
  71305.8 |
  71731.8 |
  72157.8 |#############
  72583.8 |#############
  73009.8 |########################################
  73435.8 |####################
  (2 below, 1 above range)

kernel_seq (n=18, range 94454.1-96164.6 ns)
  94454.1 |
  94539.7 |#############
  94625.2 |#############
  94710.7 |
  94796.2 |#############
  94881.7 |
  94967.3 |########################################
  95052.8 |##########################
  95138.3 |
  95223.8 |
  95309.4 |#############
  95394.9 |
  95480.4 |#############
  95565.9 |#############
  95651.4 |
  95737.0 |
  95822.5 |
  95908.0 |
  95993.5 |#############
  96079.1 |##########################
  (2 below, 2 above range)

nway (n=18, range 77048.6-80026.4 ns)
  77048.6 |
  77197.5 |########################################
  77346.4 |
  77495.3 |#############
  77644.2 |##########################
  77793.0 |########################################
  77941.9 |#############
  78090.8 |
  78239.7 |
  78388.6 |#############
  78537.5 |##########################
  78686.4 |#############
  78835.3 |
  78984.2 |#############
  79133.0 |
  79281.9 |
  79430.8 |
  79579.7 |#############
  79728.6 |
  79877.5 |
  (1 below, 1 above range)

sequential (n=18, range 98720.9-100562.5 ns)
  98720.9 |
  98812.9 |####################
  98905.0 |
  98997.1 |####################
  99089.2 |####################
  99181.3 |####################
  99273.4 |####################
  99365.4 |
  99457.5 |
  99549.6 |####################
  99641.7 |########################################
  99733.8 |
  99825.9 |
  99917.9 |
  100010.0 |####################
  100102.1 |####################
  100194.2 |####################
  100286.3 |####################
  100378.4 |########################################
  100470.4 |
  (2 below, 2 above range)

std_parallel (n=18, range 76065.3-94959.0 ns)
  76065.3 |
  77010.0 |####################
  77954.7 |####################
  78899.4 |####################
  79844.0 |########################################
  80788.7 |
  81733.4 |####################
  82678.1 |####################
  83622.8 |
  84567.5 |####################
  85512.2 |####################
  86456.8 |####################
  87401.5 |
  88346.2 |
  89290.9 |####################
  90235.6 |####################
  91180.3 |
  92125.0 |####################
  93069.6 |####################
  94014.3 |
  (2 below, 2 above range)

```

## Diagnostics

- **head_tail**: bridge=197.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=99.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=195.0% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.0% of algo (FFI overhead may distort results)
- **nway**: bridge=190.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=161.8% of algo (FFI overhead may distort results)
