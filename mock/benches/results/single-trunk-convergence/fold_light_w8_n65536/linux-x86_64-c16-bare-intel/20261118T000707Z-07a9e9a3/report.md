# fold_light, W = 8

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 38% faster than the next best (kernel_nway)

sequential (16.19 us) leads kernel_nway (22.26 us) by 38%, a clear separation rather than a photo finish. CV 103.0%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### kernel_nway beats baseline by 25% (significant)

kernel_nway is -7.68 us (25%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### std_parallel is an outlier: 4.6x slower than the field

std_parallel (75.14 us) is 4.6x the fastest (16.19 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 103.0%)

sequential wins on median (16.19 us) yet has the highest variance (CV 103.0%), while nway is the steadiest (CV 11.8%, 47.39 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 4.6x the fastest

Fastest sequential (16.19 us) to slowest std_parallel (75.14 us): 4.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 4.0x its best-20%

sequential's best 20% of batches run at 12.07 us but its worst 20% at 48.12 us (4.0x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 16186.0 ns median (-46.9% vs baseline)
- 1 variant significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 4.64x (fastest 16186.0 ns, slowest 75144.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, W = 8 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 33111ns | 31246ns | 25875ns | 30531ns | 40599ns | base |
| head_tail_dyn | 44805ns | 44592ns | 31940ns | 41750ns | 55820ns | +35.32% |
| kernel_nway | 24214ns | 23327ns | 20799ns | 22835ns | 27990ns | -26.87% |
| kernel_seq | 35918ns | 37462ns | 24274ns | 34455ns | 43936ns | +8.48% |
| nway | 49694ns | 48292ns | 46317ns | 47875ns | 54110ns | +50.08% |
| sequential | 26489ns | 16397ns | 12218ns | 16387ns | 48776ns | -20.00% |
| std_parallel | 86627ns | 75374ns | 69568ns | 73974ns | 114136ns | +161.62% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 32158ns | 25225ns | 39391ns | base | 2.038 |
| head_tail_dyn | 43838ns | 31485ns | 54281ns | +36.32% | 1.495 |
| kernel_nway | 23262ns | 20200ns | 26622ns | -27.66% | 2.817 |
| kernel_seq | 35050ns | 23836ns | 42661ns | +8.99% | 1.870 |
| nway | 49103ns | 45803ns | 53515ns | +52.69% | 1.335 |
| sequential | 26139ns | 12070ns | 48117ns | -18.72% | 2.507 |
| std_parallel | 86174ns | 69334ns | 113254ns | +167.97% | 0.761 |

## Performance model

- Peak throughput: **5.430 Gops/s** (sequential; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.149 | 39.6% |
| head_tail_dyn | 1.504 | 27.7% |
| kernel_nway | 2.944 | 54.2% |
| kernel_seq | 1.773 | 32.7% |
| nway | 1.383 | 25.5% |
| sequential | 4.049 | 74.6% |
| std_parallel | 0.872 | 16.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 33111ns | 33111ns | base |
| head_tail_dyn | 44805ns | 44805ns | +35.32% |
| kernel_nway | 24214ns | 24214ns | -26.87% |
| kernel_seq | 35918ns | 35918ns | +8.48% |
| nway | 49694ns | 49694ns | +50.08% |
| sequential | 26489ns | 26489ns | -20.00% |
| std_parallel | 86627ns | 86627ns | +161.62% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 30502ns | base | --- | [28464, 32180] | --- | --- | --- | --- |
| head_tail_dyn | 43565ns | +14431.1ns (+47.3%) | [+4792, +15942]ns | [37779, 45310] | YES | 0.0003 | 0.0001 | 0 |
| kernel_nway | 22261ns | -7679.1ns (-25.2%) | [-10691, -4499]ns | [21478, 24053] | YES | 0.0020 | 0.0013 | 0 |
| kernel_seq | 36964ns | no significant difference | [-2050, +8364]ns | [27963, 40106] | no | 0.4807 | 0.4807 | 0 |
| nway | 47387ns | +18086.6ns (+59.3%) | [+15710, +19562]ns | [46384, 49468] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 16186ns | no significant difference | [-15557, +7620]ns | [12312, 39666] | no | 0.2855 | 0.2379 | 0 |
| std_parallel | 75145ns | +45485.8ns (+149.1%) | [+41806, +57296]ns | [70245, 86642] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 31521ns | +40.6% | -30.8% | +26.7% | +56.8% | -6.1% | +124.9% |
| 1 | 2 | 31915ns | +8.1% | -33.5% | +22.7% | +48.5% | -60.4% | +136.0% |
| 1 | 3 | 28464ns | +53.2% | -14.7% | -11.2% | +63.9% | -33.5% | +186.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.230 | moderate- |
| head_tail_dyn | -0.664 | HIGH- (thermal bounce) |
| kernel_nway | -0.278 | moderate- |
| kernel_seq | -0.140 | ok |
| nway | -0.045 | ok |
| sequential | -0.409 | moderate- |
| std_parallel | -0.006 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 3/3, lost 0/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 30710.4ns | 32158.4ns | 95.5% | HIGH |
| head_tail_dyn | 44300.4ns | 43838.3ns | 101.1% | HIGH |
| kernel_nway | 24649.7ns | 23262.3ns | 106.0% | HIGH |
| kernel_seq | 35497.9ns | 35049.6ns | 101.3% | HIGH |
| nway | 49471.7ns | 49103.1ns | 100.8% | HIGH |
| sequential | 27147.3ns | 26138.9ns | 103.9% | HIGH |
| std_parallel | 87921.3ns | 86173.9ns | 102.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 25225.4-39390.6 ns)
  25225.4 |#############
  25933.7 |
  26641.9 |
  27350.2 |#############
  28058.4 |########################################
  28766.7 |
  29475.0 |##########################
  30183.2 |
  30891.5 |########################################
  31599.7 |#############
  32308.0 |#############
  33016.3 |#############
  33724.5 |
  34432.8 |#############
  35141.0 |#############
  35849.3 |
  36557.6 |
  37265.8 |
  37974.1 |
  38682.3 |
  (2 below, 1 above range)

head_tail_dyn (n=18, range 31485.3-54280.5 ns)
  31485.3 |#############
  32625.1 |#############
  33764.8 |
  34904.6 |#############
  36044.3 |
  37184.1 |
  38323.9 |
  39463.6 |#############
  40603.4 |#############
  41743.1 |#############
  42882.9 |##########################
  44022.7 |########################################
  45162.4 |#############
  46302.2 |
  47442.0 |
  48581.7 |#############
  49721.5 |#############
  50861.2 |
  52001.0 |
  53140.8 |
  (2 below, 2 above range)

kernel_nway (n=18, range 20199.6-26622.3 ns)
  20199.6 |
  20520.8 |##########################
  20841.9 |#############
  21163.0 |#############
  21484.2 |########################################
  21805.3 |#############
  22126.4 |
  22447.6 |#############
  22768.7 |#############
  23089.8 |
  23411.0 |
  23732.1 |##########################
  24053.2 |#############
  24374.4 |#############
  24695.5 |#############
  25016.6 |
  25337.8 |
  25658.9 |
  25980.0 |
  26301.2 |
  (1 below, 2 above range)

kernel_seq (n=18, range 23836.0-42661.2 ns)
  23836.0 |########################################
  24777.3 |
  25718.5 |########################################
  26659.8 |
  27601.1 |
  28542.3 |####################
  29483.6 |
  30424.8 |
  31366.1 |
  32307.4 |
  33248.6 |####################
  34189.9 |
  35131.1 |####################
  36072.4 |########################################
  37013.6 |
  37954.9 |####################
  38896.2 |####################
  39837.4 |########################################
  40778.7 |
  41719.9 |####################
  (1 below, 3 above range)

nway (n=18, range 45802.8-53515.4 ns)
  45802.8 |########################################
  46188.5 |##########################
  46574.1 |##########################
  46959.7 |#############
  47345.4 |
  47731.0 |#############
  48116.6 |
  48502.2 |
  48887.9 |########################################
  49273.5 |
  49659.1 |##########################
  50044.8 |#############
  50430.4 |#############
  50816.0 |
  51201.7 |
  51587.3 |
  51972.9 |
  52358.5 |
  52744.2 |
  53129.8 |
  (1 below, 1 above range)

sequential (n=18, range 12070.3-48117.2 ns)
  12070.3 |########################################
  13872.6 |
  15675.0 |
  17477.3 |
  19279.7 |#####
  21082.0 |
  22884.4 |
  24686.7 |###########
  26489.1 |
  28291.4 |
  30093.8 |
  31896.1 |
  33698.5 |
  35500.8 |
  37303.2 |
  39105.5 |#####
  40907.9 |#####
  42710.2 |#####
  44512.6 |
  46314.9 |
  (2 below, 3 above range)

std_parallel (n=18, range 69333.6-113254.1 ns)
  69333.6 |########################################
  71529.6 |########
  73725.6 |########################
  75921.7 |########
  78117.7 |
  80313.7 |########
  82509.7 |
  84705.8 |########
  86901.8 |########
  89097.8 |
  91293.8 |
  93489.9 |########
  95685.9 |
  97881.9 |
  100077.9 |
  102274.0 |
  104470.0 |
  106666.0 |
  108862.0 |
  111058.1 |
  (1 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=28.6% (high variance, measurements may be unstable)
- **head_tail**: bridge=100.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: CV=24.9% (high variance, measurements may be unstable)
- **head_tail_dyn**: bridge=100.9% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.7% of algo (FFI overhead may distort results)
- **kernel_seq**: CV=21.0% (high variance, measurements may be unstable)
- **kernel_seq**: bridge=101.7% of algo (FFI overhead may distort results)
- **nway**: bridge=101.5% of algo (FFI overhead may distort results)
- **sequential**: CV=63.8% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **sequential**: bridge=109.0% of algo (FFI overhead may distort results)
- **std_parallel**: CV=28.4% (high variance, measurements may be unstable)
- **std_parallel**: bridge=100.1% of algo (FFI overhead may distort results)
