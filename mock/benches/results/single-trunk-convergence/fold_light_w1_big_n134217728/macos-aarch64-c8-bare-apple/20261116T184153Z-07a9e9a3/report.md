# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### kernel_seq shows alternating (throttle bounce) (autocorr -0.67)

kernel_seq's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 13699359.4 ns median (-7.8% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.13x (fastest 13699359.4 ns, slowest 15483427.1 ns)

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
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 14890972ns | 14868635ns | 13471802ns | 14673431ns | 15926870ns | base |
| head_tail_dyn | 14591811ns | 14608802ns | 13142531ns | 14533910ns | 15403302ns | -2.01% |
| kernel_nway | 13836240ns | 13702062ns | 13418042ns | 13654368ns | 14318146ns | -7.08% |
| kernel_seq | 15286418ns | 15487578ns | 14291583ns | 15213372ns | 15893406ns | +2.66% |
| nway | 14880314ns | 14859135ns | 13926417ns | 14611431ns | 15760588ns | -0.07% |
| sequential | 14470116ns | 14675562ns | 13216490ns | 14411965ns | 15184156ns | -2.83% |
| std_parallel | 14634245ns | 14588615ns | 13683438ns | 14420941ns | 15429604ns | -1.72% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 14887405ns | 13468438ns | 15923536ns | base | 9.016 |
| head_tail_dyn | 14588569ns | 13139281ns | 15399516ns | -2.01% | 9.200 |
| kernel_nway | 13833250ns | 13415312ns | 14314714ns | -7.08% | 9.703 |
| kernel_seq | 15282795ns | 14289708ns | 15889927ns | +2.66% | 8.782 |
| nway | 14876887ns | 13922323ns | 15756943ns | -0.07% | 9.022 |
| sequential | 14466849ns | 13213333ns | 15180911ns | -2.82% | 9.278 |
| std_parallel | 14632023ns | 13681135ns | 15427609ns | -1.72% | 9.173 |

## Performance model

- Peak throughput: **10.215 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 134217728

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 9.029 | 88.4% |
| head_tail_dyn | 9.189 | 90.0% |
| kernel_nway | 9.797 | 95.9% |
| kernel_seq | 8.668 | 84.9% |
| nway | 9.034 | 88.4% |
| sequential | 9.148 | 89.5% |
| std_parallel | 9.202 | 90.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 14890972ns | 14890972ns | base |
| head_tail_dyn | 14591811ns | 14591811ns | -2.01% |
| kernel_nway | 13836240ns | 13836240ns | -7.08% |
| kernel_seq | 15286418ns | 15286418ns | +2.66% |
| nway | 14880314ns | 14880314ns | -0.07% |
| sequential | 14470116ns | 14470116ns | -2.83% |
| std_parallel | 14634245ns | 14634245ns | -1.72% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14865422ns | base | --- | [13873255, 15923536] | --- | --- | --- | --- |
| head_tail_dyn | 14605620ns | no significant difference | [-1903099, +1121104]ns | [13760573, 15399516] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 13699359ns | -981062.5ns (-6.6%) | [-2081818, -99583]ns | [13485677, 14314714] | YES (adj: no) | 0.1875 | 0.0313 | 0 |
| kernel_seq | 15483427ns | no significant difference | [-668224, +1043615]ns | [14475031, 15889927] | no | 0.6563 | 0.2188 | 0 |
| nway | 14856469ns | no significant difference | [-1544729, +1126443]ns | [14017250, 15756943] | no | 1.0000 | 0.6875 | 0 |
| sequential | 14672583ns | no significant difference | [-2116620, +1023318]ns | [13547052, 15180911] | no | 1.0000 | 0.6875 | 0 |
| std_parallel | 14586203ns | no significant difference | [-1338057, +786484]ns | [13882255, 15427609] | no | 1.0000 | 1.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 14630500ns | +5.3% | -5.3% | +7.1% | +7.7% | +3.6% | -1.5% |
| 1 | 2 | 15023932ns | -3.1% | -9.4% | -3.7% | -6.7% | -5.8% | -2.9% |
| 1 | 3 | 15007781ns | -8.0% | -6.5% | +4.6% | -1.0% | -6.0% | -0.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.188 | ok |
| head_tail_dyn | -0.001 | ok |
| kernel_nway | -0.556 | HIGH- (thermal bounce) |
| kernel_seq | -0.666 | HIGH- (thermal bounce) |
| nway | -0.488 | moderate- |
| sequential | -0.143 | ok |
| std_parallel | -0.018 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 2/3, lost 1/3
- **kernel_nway**: won 3/3, lost 0/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 2/3, lost 1/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 3/3, lost 0/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 14691994.8ns | 14887404.5ns | 98.7% | HIGH |
| head_tail_dyn | 14660625.0ns | 14588569.4ns | 100.5% | HIGH |
| kernel_nway | 15109633.7ns | 13833250.0ns | 109.2% | HIGH |
| kernel_seq | 14159888.9ns | 15282795.1ns | 92.7% | HIGH |
| nway | 14683159.7ns | 14876887.2ns | 98.7% | HIGH |
| sequential | 14650012.2ns | 14466849.0ns | 101.3% | HIGH |
| std_parallel | 14940656.2ns | 14632022.6ns | 102.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 13468437.5-15923536.4 ns)
  13468437.5 |########################################
  13591192.4 |
  13713947.4 |
  13836702.3 |
  13959457.3 |
  14082212.2 |
  14204967.2 |########################################
  14327722.1 |
  14450477.1 |
  14573232.0 |
  14695987.0 |########################################
  14818741.9 |
  14941496.9 |########################################
  15064251.8 |
  15187006.8 |########################################
  15309761.7 |
  15432516.7 |
  15555271.6 |
  15678026.6 |
  15800781.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 13139281.2-15399515.6 ns)
  13139281.2 |########################################
  13252292.9 |
  13365304.6 |
  13478316.4 |
  13591328.1 |
  13704339.8 |
  13817351.5 |
  13930363.2 |
  14043375.0 |
  14156386.7 |
  14269398.4 |########################################
  14382410.1 |########################################
  14495421.8 |
  14608433.6 |
  14721445.3 |########################################
  14834457.0 |
  14947468.7 |
  15060480.4 |
  15173492.2 |########################################
  15286503.9 |
  (0 below, 1 above range)

kernel_nway (n=6, range 13415312.5-14314713.5 ns)
  13415312.5 |####################
  13460282.6 |
  13505252.6 |
  13550222.7 |########################################
  13595192.7 |
  13640162.8 |
  13685132.8 |
  13730102.8 |
  13775072.9 |####################
  13820042.9 |
  13865013.0 |
  13909983.1 |
  13954953.1 |
  13999923.2 |
  14044893.2 |
  14089863.2 |####################
  14134833.3 |
  14179803.3 |
  14224773.4 |
  14269743.4 |
  (0 below, 1 above range)

kernel_seq (n=6, range 14289708.3-15889927.1 ns)
  14289708.3 |########################################
  14369719.2 |
  14449730.2 |
  14529741.1 |
  14609752.1 |########################################
  14689763.0 |
  14769773.9 |
  14849784.9 |
  14929795.8 |
  15009806.7 |
  15089817.7 |
  15169828.6 |
  15249839.6 |
  15329850.5 |
  15409861.4 |########################################
  15489872.4 |########################################
  15569883.3 |
  15649894.2 |
  15729905.2 |
  15809916.1 |########################################
  (0 below, 1 above range)

nway (n=6, range 13922322.9-15756942.7 ns)
  13922322.9 |########################################
  14014053.9 |
  14105784.9 |########################################
  14197515.9 |
  14289246.9 |
  14380977.8 |
  14472708.8 |
  14564439.8 |########################################
  14656170.8 |
  14747901.8 |
  14839632.8 |
  14931363.8 |
  15023094.8 |########################################
  15114825.8 |
  15206556.8 |
  15298287.8 |
  15390018.7 |
  15481749.7 |
  15573480.7 |
  15665211.7 |########################################
  (0 below, 1 above range)

sequential (n=6, range 13213333.3-15180911.4 ns)
  13213333.3 |########################################
  13311712.2 |
  13410091.1 |
  13508470.0 |
  13606848.9 |
  13705227.8 |
  13803606.7 |########################################
  13901985.7 |
  14000364.6 |
  14098743.5 |
  14197122.4 |
  14295501.3 |
  14393880.2 |########################################
  14492259.1 |
  14590638.0 |
  14689016.9 |
  14787395.8 |
  14885774.7 |########################################
  14984153.6 |########################################
  15082532.5 |
  (0 below, 1 above range)

std_parallel (n=6, range 13681135.4-15427609.4 ns)
  13681135.4 |####################
  13768459.1 |
  13855782.8 |
  13943106.5 |
  14030430.2 |####################
  14117753.9 |
  14205077.6 |
  14292401.3 |
  14379725.0 |
  14467048.7 |
  14554372.4 |########################################
  14641696.1 |
  14729019.8 |
  14816343.5 |
  14903667.2 |
  14990990.9 |
  15078314.6 |####################
  15165638.3 |
  15252962.0 |
  15340285.7 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=97.0% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=98.8% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=111.7% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=89.2% of algo (FFI overhead may distort results)
- **nway**: bridge=99.7% of algo (FFI overhead may distort results)
- **sequential**: bridge=101.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.7% of algo (FFI overhead may distort results)
