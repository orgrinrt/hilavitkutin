# append, far above cache, W = 2

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### std_parallel beats baseline by 48% (significant)

std_parallel is -6.70 ms (48%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.2x slower than the field

sequential (23.35 ms) is 3.2x the fastest (7.22 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### head_tail shows alternating (throttle bounce) (autocorr -0.66)

head_tail's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.2x the fastest

Fastest std_parallel (7.22 ms) to slowest sequential (23.35 ms): 3.2x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: std_parallel** at 7219579.3 ns median (-47.8% vs baseline)
- 3 variants significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 3.23x (fastest 7219579.3 ns, slowest 23345819.6 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 2 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100342298 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14029332ns | 13844482ns | 13193549ns | 13713358ns | 14921184ns | base |
| head_tail_dyn | 14419860ns | 14462840ns | 13801950ns | 14261500ns | 14966354ns | +2.78% |
| kernel_nway | 7974630ns | 7819668ns | 7098015ns | 7662720ns | 8880802ns | -43.16% |
| kernel_seq | 13944958ns | 13855224ns | 13529438ns | 13764602ns | 14423254ns | -0.60% |
| nway | 11347675ns | 10774840ns | 9978276ns | 10639772ns | 13094229ns | -19.11% |
| sequential | 23353367ns | 23350571ns | 22821487ns | 23269303ns | 23745403ns | +66.46% |
| std_parallel | 7220752ns | 7224839ns | 6791666ns | 7181119ns | 7494743ns | -48.53% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14022927ns | 13188689ns | 14915408ns | base | 1.196 |
| head_tail_dyn | 14413948ns | 13797619ns | 14960065ns | +2.79% | 1.164 |
| kernel_nway | 7968854ns | 7093870ns | 8875152ns | -43.17% | 2.105 |
| kernel_seq | 13938745ns | 13524747ns | 14417046ns | -0.60% | 1.204 |
| nway | 11343280ns | 9974472ns | 13089971ns | -19.11% | 1.479 |
| sequential | 23348689ns | 22816788ns | 23740596ns | +66.50% | 0.719 |
| std_parallel | 7215631ns | 6788311ns | 7488506ns | -48.54% | 2.325 |

## Performance model

- Peak throughput: **2.471 Gops/s** (std_parallel; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.212 | 49.1% |
| head_tail_dyn | 1.161 | 47.0% |
| kernel_nway | 2.147 | 86.9% |
| kernel_seq | 1.211 | 49.0% |
| nway | 1.558 | 63.0% |
| sequential | 0.719 | 29.1% |
| std_parallel | 2.324 | 94.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14029332ns | 14029332ns | base |
| head_tail_dyn | 14419860ns | 14419860ns | +2.78% |
| kernel_nway | 7974630ns | 7974630ns | -43.16% |
| kernel_seq | 13944958ns | 13944958ns | -0.60% |
| nway | 11347675ns | 11347675ns | -19.11% |
| sequential | 23353367ns | 23353367ns | +66.46% |
| std_parallel | 7220752ns | 7220752ns | -48.53% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 13838687ns | base | --- | [13314686, 14915408] | --- | --- | --- | --- |
| head_tail_dyn | 14456796ns | no significant difference | [-151206, +950867]ns | [13824983, 14960065] | no | 0.8250 | 0.6875 | 0 |
| kernel_nway | 7813835ns | -6478744.4ns (-46.8%) | [-7243941, -4439534]ns | [7217575, 8875152] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| kernel_seq | 13849003ns | no significant difference | [-539453, +269152]ns | [13550187, 14417046] | no | 1.0000 | 1.0000 | 0 |
| nway | 10770240ns | -3336297.3ns (-24.1%) | [-4145169, -557474]ns | [10169631, 13089971] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| sequential | 23345820ns | +9400286.1ns (+67.9%) | [+8347796, +10229202]ns | [22959651, 23740596] | YES (adj: no) | 0.0625 | 0.0313 | 0 |
| std_parallel | 7219579ns | -6702871.8ns (-48.4%) | [-7691311, -6027706]ns | [6938807, 7488506] | YES (adj: no) | 0.0625 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 13647445ns | +1.3% | -43.0% | +1.2% | -4.1% | +70.1% | -49.1% |
| 1 | 2 | 14915408ns | +0.3% | -48.6% | -3.6% | -27.8% | +56.0% | -51.6% |
| 1 | 3 | 13505929ns | +7.0% | -37.4% | +0.9% | -24.7% | +74.5% | -44.6% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.661 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.461 | moderate- |
| kernel_nway | -0.243 | moderate- |
| kernel_seq | -0.626 | HIGH- (thermal bounce) |
| nway | -0.069 | ok |
| sequential | -0.095 | ok |
| std_parallel | -0.001 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 3/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 3/3, lost 0/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 98267653.2ns | 14022927.2ns | 700.8% | HIGH |
| head_tail_dyn | 98836172.5ns | 14413947.9ns | 685.7% | HIGH |
| kernel_nway | 93548186.4ns | 7968853.8ns | 1173.9% | HIGH |
| kernel_seq | 99505804.0ns | 13938745.5ns | 713.9% | HIGH |
| nway | 98586209.8ns | 11343280.5ns | 869.1% | HIGH |
| sequential | 107256971.0ns | 23348688.5ns | 459.4% | HIGH |
| std_parallel | 91493860.0ns | 7215630.8ns | 1268.0% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 13188689.1-14915408.4 ns)
  13188689.1 |####################
  13275025.1 |
  13361361.0 |####################
  13447697.0 |
  13534033.0 |
  13620368.9 |
  13706704.9 |
  13793040.9 |########################################
  13879376.8 |
  13965712.8 |
  14052048.8 |
  14138384.7 |
  14224720.7 |
  14311056.6 |
  14397392.6 |
  14483728.6 |
  14570064.5 |
  14656400.5 |
  14742736.5 |
  14829072.4 |####################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 13797619.0-14960064.6 ns)
  13797619.0 |########################################
  13855741.3 |
  13913863.6 |
  13971985.8 |
  14030108.1 |
  14088230.4 |
  14146352.7 |
  14204475.0 |
  14262597.2 |####################
  14320719.5 |
  14378841.8 |
  14436964.1 |
  14495086.4 |
  14553208.6 |
  14611330.9 |####################
  14669453.2 |####################
  14727575.5 |
  14785697.8 |
  14843820.0 |
  14901942.3 |
  (0 below, 1 above range)

kernel_nway (n=6, range 7093870.2-8875151.6 ns)
  7093870.2 |########################################
  7182934.3 |
  7271998.3 |########################################
  7361062.4 |
  7450126.5 |
  7539190.6 |########################################
  7628254.6 |
  7717318.7 |
  7806382.8 |
  7895446.9 |
  7984510.9 |########################################
  8073575.0 |
  8162639.1 |
  8251703.1 |
  8340767.2 |
  8429831.3 |########################################
  8518895.4 |
  8607959.4 |
  8697023.5 |
  8786087.6 |
  (0 below, 1 above range)

kernel_seq (n=6, range 13524747.2-14417045.9 ns)
  13524747.2 |########################################
  13569362.1 |########################################
  13613977.1 |
  13658592.0 |
  13703206.9 |########################################
  13747821.9 |
  13792436.8 |
  13837051.7 |
  13881666.7 |
  13926281.6 |
  13970896.5 |########################################
  14015511.5 |########################################
  14060126.4 |
  14104741.4 |
  14149356.3 |
  14193971.2 |
  14238586.2 |
  14283201.1 |
  14327816.0 |
  14372431.0 |
  (0 below, 1 above range)

nway (n=6, range 9974471.7-13089970.7 ns)
  9974471.7 |########################################
  10130246.6 |
  10286021.6 |########################################
  10441796.5 |
  10597571.5 |########################################
  10753346.4 |
  10909121.4 |########################################
  11064896.3 |
  11220671.3 |
  11376446.2 |
  11532221.2 |
  11687996.1 |
  11843771.1 |
  11999546.0 |########################################
  12155321.0 |
  12311095.9 |
  12466870.9 |
  12622645.8 |
  12778420.8 |
  12934195.8 |
  (0 below, 1 above range)

sequential (n=6, range 22816787.7-23740595.5 ns)
  22816787.7 |########################################
  22862978.1 |
  22909168.5 |
  22955358.9 |
  23001549.3 |
  23047739.6 |
  23093930.0 |########################################
  23140120.4 |
  23186310.8 |
  23232501.2 |
  23278691.6 |########################################
  23324882.0 |
  23371072.4 |########################################
  23417262.8 |
  23463453.2 |
  23509643.6 |
  23555833.9 |
  23602024.3 |
  23648214.7 |
  23694405.1 |########################################
  (0 below, 1 above range)

std_parallel (n=6, range 6788310.8-7488505.7 ns)
  6788310.8 |####################
  6823320.5 |
  6858330.3 |
  6893340.0 |
  6928349.8 |
  6963359.5 |
  6998369.3 |
  7033379.0 |
  7068388.7 |########################################
  7103398.5 |
  7138408.2 |
  7173418.0 |
  7208427.7 |
  7243437.5 |
  7278447.2 |
  7313456.9 |####################
  7348466.7 |####################
  7383476.4 |
  7418486.2 |
  7453495.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=701.5% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=680.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=1193.5% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=713.1% of algo (FFI overhead may distort results)
- **nway**: bridge=910.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=456.9% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=1260.6% of algo (FFI overhead may distort results)
