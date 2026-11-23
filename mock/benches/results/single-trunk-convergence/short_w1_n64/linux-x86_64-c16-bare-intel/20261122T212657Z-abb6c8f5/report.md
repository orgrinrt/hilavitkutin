# fold_light, short fiber (1 to 4 morsels), W = 1

7 variants, 18 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 14% faster than the next best (head_tail)

sequential (10.04 us) leads head_tail (11.49 us) by 14%, a clear separation rather than a photo finish. CV 78.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### nway is an outlier: 3.6x slower than the field

nway (35.97 us) is 3.6x the fastest (10.04 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### sequential is fastest but the noisiest (CV 78.9%)

sequential wins on median (10.04 us) yet has the highest variance (CV 78.9%), while std_parallel is the steadiest (CV 11.0%, 34.25 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### kernel_nway shows alternating (throttle bounce) (autocorr -0.66)

kernel_nway's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Two tiers: {sequential, head_tail, kernel_seq, head_tail_dyn, kernel_nway} vs {std_parallel, nway} (118% apart)

The field splits into a fast tier {sequential, head_tail, kernel_seq, head_tail_dyn, kernel_nway} and a slow tier {std_parallel, nway} with a 118% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 3.6x the fastest

Fastest sequential (10.04 us) to slowest nway (35.97 us): 3.6x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### sequential is inconsistent: worst-20% is 3.5x its best-20%

sequential's best 20% of batches run at 6.75 us but its worst 20% at 23.54 us (3.5x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: sequential** at 10041.7 ns median (-12.6% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 3.58x (fastest 10041.7 ns, slowest 35966.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, short fiber (1 to 4 morsels), W = 1 |
| Passes | 3 |
| Runs per pass | 150 |
| Batch size | 25 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112789863 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 11539ns | 12131ns | 7220ns | 10970ns | 14552ns | base |
| head_tail_dyn | 16416ns | 15235ns | 13188ns | 15113ns | 19983ns | +42.26% |
| kernel_nway | 16222ns | 16448ns | 10194ns | 15240ns | 20710ns | +40.59% |
| kernel_seq | 13470ns | 13973ns | 9753ns | 12968ns | 16080ns | +16.73% |
| nway | 37109ns | 36511ns | 28882ns | 36096ns | 42743ns | +221.59% |
| sequential | 13619ns | 10284ns | 6886ns | 8901ns | 24061ns | +18.02% |
| std_parallel | 35730ns | 34764ns | 30683ns | 34323ns | 40364ns | +209.64% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 10757ns | 6757ns | 13425ns | base | 0.006 |
| head_tail_dyn | 15421ns | 12580ns | 18685ns | +43.36% | 0.004 |
| kernel_nway | 15224ns | 9679ns | 19303ns | +41.53% | 0.004 |
| kernel_seq | 12599ns | 9102ns | 14989ns | +17.13% | 0.005 |
| nway | 36565ns | 28465ns | 42118ns | +239.93% | 0.002 |
| sequential | 13324ns | 6750ns | 23539ns | +23.87% | 0.005 |
| std_parallel | 35101ns | 30245ns | 39564ns | +226.32% | 0.002 |

## Performance model

- Peak throughput: **0.009 Gops/s** (sequential; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 0.006 | 58.8% |
| head_tail_dyn | 0.005 | 47.6% |
| kernel_nway | 0.004 | 43.1% |
| kernel_seq | 0.005 | 51.0% |
| nway | 0.002 | 18.8% |
| sequential | 0.006 | 67.2% |
| std_parallel | 0.002 | 19.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 11539ns | 11539ns | base |
| head_tail_dyn | 16416ns | 16416ns | +42.26% |
| kernel_nway | 16222ns | 16222ns | +40.59% |
| kernel_seq | 13470ns | 13470ns | +16.73% |
| nway | 37109ns | 37109ns | +221.59% |
| sequential | 13619ns | 13619ns | +18.02% |
| std_parallel | 35730ns | 35730ns | +209.64% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 11486ns | base | --- | [8610, 13060] | --- | --- | --- | --- |
| head_tail_dyn | 14192ns | +2187.9ns (+19.0%) | [+1063, +9810]ns | [13651, 16555] | YES | 0.0000 | 0.0000 | 0 |
| kernel_nway | 15680ns | no significant difference | [-523, +7791]ns | [12198, 18313] | no | 0.2855 | 0.2379 | 0 |
| kernel_seq | 13238ns | +2019.8ns (+17.6%) | [+458, +2662]ns | [11273, 13707] | YES | 0.0000 | 0.0000 | 0 |
| nway | 35967ns | +24783.1ns (+215.8%) | [+21916, +29896]ns | [33871, 40272] | YES | 0.0000 | 0.0000 | 0 |
| sequential | 10042ns | no significant difference | [-3921, +3842]ns | [6988, 16916] | no | 0.4807 | 0.4807 | 0 |
| std_parallel | 34248ns | +24301.8ns (+211.6%) | [+21181, +27147]ns | [32168, 37130] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 7117ns | +166.6% | +143.7% | +39.3% | +465.9% | +41.5% | +400.6% |
| 1 | 2 | 13060ns | +5.3% | -18.7% | +2.0% | +169.8% | -8.4% | +179.3% |
| 1 | 3 | 12005ns | +11.6% | +50.6% | +23.8% | +161.3% | -41.6% | +166.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.271 | moderate- |
| head_tail_dyn | -0.135 | ok |
| kernel_nway | -0.661 | HIGH- (thermal bounce) |
| kernel_seq | -0.030 | ok |
| nway | -0.004 | ok |
| sequential | -0.416 | moderate- |
| std_parallel | -0.278 | moderate- |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 11398.6ns | 10756.8ns | 106.0% | HIGH |
| head_tail_dyn | 16048.2ns | 15421.1ns | 104.1% | HIGH |
| kernel_nway | 15851.2ns | 15223.7ns | 104.1% | HIGH |
| kernel_seq | 13070.1ns | 12599.1ns | 103.7% | HIGH |
| nway | 38605.8ns | 36565.2ns | 105.6% | HIGH |
| sequential | 13662.4ns | 13324.1ns | 102.5% | HIGH |
| std_parallel | 35985.7ns | 35101.3ns | 102.5% | HIGH |

## Distribution (algo ns)

```
head_tail (n=18, range 6756.9-13425.4 ns)
   6756.9 |##########################
   7090.3 |#############
   7423.7 |#############
   7757.1 |
   8090.6 |
   8424.0 |
   8757.4 |
   9090.8 |
   9424.3 |
   9757.7 |#############
  10091.1 |#############
  10424.6 |
  10758.0 |#############
  11091.4 |#############
  11424.8 |
  11758.3 |########################################
  12091.7 |
  12425.1 |
  12758.5 |#############
  13092.0 |########################################
  (1 below, 2 above range)

head_tail_dyn (n=18, range 12580.0-18684.9 ns)
  12580.0 |##########################
  12885.3 |
  13190.5 |
  13495.8 |########################################
  13801.0 |##########################
  14106.2 |##########################
  14411.5 |
  14716.7 |#############
  15022.0 |
  15327.2 |
  15632.5 |#############
  15937.7 |
  16242.9 |#############
  16548.2 |#############
  16853.4 |
  17158.7 |
  17463.9 |
  17769.2 |
  18074.4 |
  18379.6 |
  (1 below, 4 above range)

kernel_nway (n=18, range 9679.1-19302.6 ns)
   9679.1 |
  10160.2 |####################
  10641.4 |####################
  11122.6 |
  11603.8 |####################
  12084.9 |
  12566.1 |####################
  13047.3 |####################
  13528.5 |
  14009.6 |
  14490.8 |
  14972.0 |####################
  15453.2 |########################################
  15934.3 |####################
  16415.5 |
  16896.7 |
  17377.9 |
  17859.0 |########################################
  18340.2 |####################
  18821.4 |####################
  (2 below, 3 above range)

kernel_seq (n=18, range 9102.2-14988.9 ns)
   9102.2 |
   9396.5 |#############
   9690.9 |
   9985.2 |#############
  10279.5 |
  10573.9 |
  10868.2 |
  11162.5 |##########################
  11456.9 |##########################
  11751.2 |
  12045.5 |
  12339.9 |
  12634.2 |
  12928.5 |#############
  13222.9 |##########################
  13517.2 |########################################
  13811.5 |#############
  14105.9 |
  14400.2 |
  14694.5 |
  (2 below, 3 above range)

nway (n=18, range 28464.9-42117.5 ns)
  28464.9 |
  29147.5 |####################
  29830.1 |
  30512.8 |
  31195.4 |
  31878.0 |
  32560.7 |####################
  33243.3 |####################
  33925.9 |####################
  34608.6 |####################
  35291.2 |########################################
  35973.8 |####################
  36656.5 |
  37339.1 |####################
  38021.7 |
  38704.4 |
  39387.0 |########################################
  40069.6 |####################
  40752.3 |####################
  41434.9 |
  (2 below, 3 above range)

sequential (n=18, range 6750.4-23538.9 ns)
   6750.4 |########################################
   7589.9 |
   8429.3 |
   9268.7 |################################
  10108.1 |########
  10947.5 |
  11787.0 |
  12626.4 |
  13465.8 |
  14305.2 |
  15144.7 |
  15984.1 |
  16823.5 |################
  17662.9 |
  18502.3 |
  19341.8 |
  20181.2 |
  21020.6 |
  21860.0 |########
  22699.5 |
  (2 below, 3 above range)

std_parallel (n=18, range 30245.2-39563.8 ns)
  30245.2 |
  30711.2 |
  31177.1 |#############
  31643.0 |##########################
  32108.9 |#############
  32574.9 |
  33040.8 |
  33506.7 |#############
  33972.7 |########################################
  34438.6 |
  34904.5 |
  35370.5 |##########################
  35836.4 |
  36302.3 |
  36768.2 |#############
  37234.2 |#############
  37700.1 |
  38166.0 |#############
  38632.0 |
  39097.9 |
  (2 below, 3 above range)

```

## Diagnostics

- **head_tail**: CV=24.0% (high variance, measurements may be unstable)
- **head_tail**: bridge=105.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=102.5% of algo (FFI overhead may distort results)
- **kernel_nway**: CV=24.5% (high variance, measurements may be unstable)
- **kernel_nway**: bridge=101.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=103.8% of algo (FFI overhead may distort results)
- **nway**: bridge=102.4% of algo (FFI overhead may distort results)
- **sequential**: CV=59.5% (high variance, measurements may be unstable)
- **sequential**: worst_20/best_20 = 3.5x (possible bimodal distribution)
- **sequential**: bridge=102.2% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.0% of algo (FFI overhead may distort results)
