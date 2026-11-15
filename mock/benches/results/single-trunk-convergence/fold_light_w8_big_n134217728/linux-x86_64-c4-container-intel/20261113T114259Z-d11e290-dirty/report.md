# fold_light, far above cache, W = 8

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### nway beats baseline by 50% (significant)

nway is -23.40 ms (50%) faster than baseline head_tail, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### sequential is an outlier: 3.4x slower than the field

sequential (84.71 ms) is 3.4x the fastest (25.27 ms), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (kernel_nway, nway) are a dead heat (<1%)

kernel_nway (25.27 ms) and nway (25.30 ms) differ by 0.14%, inside the noise, even though the wider field spreads 235.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### head_tail_dyn shows alternating (throttle bounce) (autocorr -0.66)

head_tail_dyn's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 3.4x the fastest

Fastest kernel_nway (25.27 ms) to slowest sequential (84.71 ms): 3.4x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

## Key findings

- **Fastest: kernel_nway** at 25266414.5 ns median (-46.3% vs baseline)
- 3 variants significantly faster than baseline
- 3 variants significantly slower than baseline
- Spread: 3.35x (fastest 25266414.5 ns, slowest 84711457.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 8 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100242360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 48109229ns | 47036040ns | 44775049ns | 46644044ns | 51974096ns | base |
| head_tail_dyn | 55403382ns | 54983800ns | 54186556ns | 54885216ns | 56789043ns | +15.16% |
| kernel_nway | 25460370ns | 25272335ns | 24313618ns | 24970356ns | 26768768ns | -47.08% |
| kernel_seq | 84769261ns | 83722501ns | 81030503ns | 83172913ns | 89033161ns | +76.20% |
| nway | 25015248ns | 25306047ns | 22515571ns | 25119327ns | 26108967ns | -48.00% |
| sequential | 84476497ns | 84715153ns | 81685843ns | 83752830ns | 86957324ns | +75.59% |
| std_parallel | 29279195ns | 27743469ns | 26832387ns | 27644706ns | 32954333ns | -39.14% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 48102444ns | 44770243ns | 51967284ns | base | 2.790 |
| head_tail_dyn | 55397196ns | 54181739ns | 56783063ns | +15.17% | 2.423 |
| kernel_nway | 25454458ns | 24306926ns | 26762679ns | -47.08% | 5.273 |
| kernel_seq | 84762159ns | 81025219ns | 89024830ns | +76.21% | 1.583 |
| nway | 25011295ns | 22511579ns | 26105022ns | -48.00% | 5.366 |
| sequential | 84472899ns | 81682409ns | 86953726ns | +75.61% | 1.589 |
| std_parallel | 29275031ns | 26829249ns | 32950060ns | -39.14% | 4.585 |

## Performance model

- Peak throughput: **5.962 Gops/s** (nway; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 2.854 | 47.9% |
| head_tail_dyn | 2.441 | 40.9% |
| kernel_nway | 5.312 | 89.1% |
| kernel_seq | 1.603 | 26.9% |
| nway | 5.305 | 89.0% |
| sequential | 1.584 | 26.6% |
| std_parallel | 4.839 | 81.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 48109229ns | 48109229ns | base |
| head_tail_dyn | 55403382ns | 55403382ns | +15.16% |
| kernel_nway | 25460370ns | 25460370ns | -47.08% |
| kernel_seq | 84769261ns | 84769261ns | +76.20% |
| nway | 25015248ns | 25015248ns | -48.00% |
| sequential | 84476497ns | 84476497ns | +75.59% |
| std_parallel | 29279195ns | 29279195ns | -39.14% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 47027644ns | base | --- | [45312404, 51967284] | --- | --- | --- | --- |
| head_tail_dyn | 54977277ns | +7806677.6ns (+16.6%) | [+3579876, +10497701]ns | [54431247, 56783063] | YES | 0.0313 | 0.0313 | 0 |
| kernel_nway | 25266414ns | -22425384.4ns (-47.7%) | [-25204605, -20313970]ns | [24334279, 26762679] | YES | 0.0313 | 0.0313 | 0 |
| kernel_seq | 83715991ns | +34853165.8ns (+74.1%) | [+31748707, +43377271]ns | [81545656, 89024830] | YES | 0.0313 | 0.0313 | 0 |
| nway | 25302018ns | -23400800.6ns (-49.8%) | [-26032560, -19840087]ns | [23626844, 26105022] | YES | 0.0313 | 0.0313 | 0 |
| sequential | 84711458ns | +35061022.2ns (+74.6%) | [+33177375, +40872966]ns | [81753512, 86953726] | YES | 0.0313 | 0.0313 | 0 |
| std_parallel | 27738192ns | -19299790.0ns (-41.0%) | [-24484951, -12697498]ns | [27136841, 32950060] | YES | 0.0313 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 45647558ns | +20.1% | -46.0% | +95.0% | -44.8% | +89.5% | -27.8% |
| 1 | 2 | 48816084ns | +16.3% | -45.3% | +68.4% | -48.2% | +71.5% | -44.3% |
| 1 | 3 | 49843690ns | +9.5% | -49.8% | +66.6% | -50.8% | +66.9% | -44.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.053 | ok |
| head_tail_dyn | -0.660 | HIGH- (thermal bounce) |
| kernel_nway | -0.648 | HIGH- (thermal bounce) |
| kernel_seq | -0.236 | moderate- |
| nway | -0.227 | moderate- |
| sequential | -0.088 | ok |
| std_parallel | -0.216 | moderate- |

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
| head_tail | 47638089.8ns | 48102444.1ns | 99.0% | HIGH |
| head_tail_dyn | 55687895.1ns | 55397195.5ns | 100.5% | HIGH |
| kernel_nway | 24937594.2ns | 25454457.7ns | 98.0% | HIGH |
| kernel_seq | 86362408.8ns | 84762158.9ns | 101.9% | HIGH |
| nway | 25015553.8ns | 25011294.7ns | 100.0% | HIGH |
| sequential | 85063804.6ns | 84472898.6ns | 100.7% | HIGH |
| std_parallel | 29635110.3ns | 29275031.0ns | 101.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 44770243.4-51967284.0 ns)
  44770243.4 |########################################
  45130095.4 |
  45489947.5 |
  45849799.5 |########################################
  46209651.5 |########################################
  46569503.6 |
  46929355.6 |
  47289207.6 |########################################
  47649059.7 |
  48008911.7 |
  48368763.7 |
  48728615.8 |
  49088467.8 |
  49448319.8 |
  49808171.9 |
  50168023.9 |
  50527875.9 |
  50887728.0 |
  51247580.0 |
  51607432.0 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 54181738.6-56783063.2 ns)
  54181738.6 |####################
  54311804.8 |
  54441871.1 |
  54571937.3 |####################
  54702003.5 |
  54832069.8 |
  54962136.0 |########################################
  55092202.2 |
  55222268.4 |
  55352334.7 |
  55482400.9 |
  55612467.1 |
  55742533.4 |
  55872599.6 |
  56002665.8 |
  56132732.1 |
  56262798.3 |
  56392864.5 |
  56522930.7 |
  56652997.0 |####################
  (0 below, 1 above range)

kernel_nway (n=6, range 24306926.5-26762679.1 ns)
  24306926.5 |########################################
  24429714.1 |
  24552501.8 |
  24675289.4 |
  24798077.0 |####################
  24920864.7 |
  25043652.3 |
  25166439.9 |
  25289227.6 |
  25412015.2 |
  25534802.8 |####################
  25657590.5 |####################
  25780378.1 |
  25903165.7 |
  26025953.4 |
  26148741.0 |
  26271528.6 |
  26394316.3 |
  26517103.9 |
  26639891.5 |
  (0 below, 1 above range)

kernel_seq (n=6, range 81025218.8-89024829.9 ns)
  81025218.8 |########################################
  81425199.4 |
  81825179.9 |########################################
  82225160.5 |
  82625141.0 |
  83025121.6 |########################################
  83425102.1 |
  83825082.7 |########################################
  84225063.2 |
  84625043.8 |
  85025024.3 |
  85425004.9 |
  85824985.5 |
  86224966.0 |
  86624946.6 |########################################
  87024927.1 |
  87424907.7 |
  87824888.2 |
  88224868.8 |
  88624849.3 |
  (0 below, 1 above range)

nway (n=6, range 22511579.2-26105022.1 ns)
  22511579.2 |####################
  22691251.3 |
  22870923.5 |
  23050595.6 |
  23230267.8 |
  23409939.9 |
  23589612.1 |
  23769284.2 |
  23948956.4 |
  24128628.5 |
  24308300.7 |
  24487972.8 |
  24667645.0 |####################
  24847317.1 |
  25026989.3 |
  25206661.4 |########################################
  25386333.6 |
  25566005.7 |####################
  25745677.9 |
  25925350.0 |
  (0 below, 1 above range)

sequential (n=6, range 81682409.2-86953726.4 ns)
  81682409.2 |########################################
  81945975.1 |
  82209540.9 |
  82473106.8 |
  82736672.6 |
  83000238.5 |
  83263804.4 |
  83527370.2 |
  83790936.1 |
  84054501.9 |
  84318067.8 |####################
  84581633.7 |
  84845199.5 |####################
  85108765.4 |
  85372331.2 |
  85635897.1 |####################
  85899463.0 |
  86163028.8 |
  86426594.7 |
  86690160.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 26829249.0-32950060.5 ns)
  26829249.0 |####################
  27135289.6 |
  27441330.1 |########################################
  27747370.7 |####################
  28053411.3 |
  28359451.9 |
  28665492.4 |
  28971533.0 |
  29277573.6 |
  29583614.2 |####################
  29889654.8 |
  30195695.3 |
  30501735.9 |
  30807776.5 |
  31113817.1 |
  31419857.6 |
  31725898.2 |
  32031938.8 |
  32337979.4 |
  32644019.9 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.7% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=101.5% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=98.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.7% of algo (FFI overhead may distort results)
- **nway**: bridge=99.5% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.1% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=103.7% of algo (FFI overhead may distort results)
