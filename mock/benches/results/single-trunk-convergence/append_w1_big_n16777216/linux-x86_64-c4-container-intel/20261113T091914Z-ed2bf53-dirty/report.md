# append, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### head_tail shows alternating (throttle bounce) (autocorr -0.60)

head_tail's per-pass series has lag-1 autocorrelation -0.60, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: kernel_nway** at 14822777.7 ns median (-1.1% vs baseline)
- 4 variants significantly slower than baseline
- Spread: 1.80x (fastest 14822777.7 ns, slowest 26726867.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | append, far above cache, W = 1 |
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
| head_tail | 15097825ns | 14998128ns | 14326949ns | 14818064ns | 15902904ns | base |
| head_tail_dyn | 15303423ns | 15256717ns | 14316712ns | 14950258ns | 16326526ns | +1.36% |
| kernel_nway | 14845240ns | 14827685ns | 14563461ns | 14780911ns | 15082624ns | -1.67% |
| kernel_seq | 17135733ns | 16949098ns | 15022528ns | 16461521ns | 19203653ns | +13.50% |
| nway | 23282605ns | 22753306ns | 20594303ns | 22186694ns | 26270622ns | +54.21% |
| sequential | 26920531ns | 26732104ns | 23739538ns | 25876788ns | 30076643ns | +78.31% |
| std_parallel | 16845072ns | 16964212ns | 15161283ns | 16401458ns | 18352389ns | +11.57% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 15090696ns | 14322054ns | 15895880ns | base | 1.112 |
| head_tail_dyn | 15297218ns | 14309051ns | 16320252ns | +1.37% | 1.097 |
| kernel_nway | 14838953ns | 14555864ns | 15076214ns | -1.67% | 1.131 |
| kernel_seq | 17129232ns | 15014679ns | 19196860ns | +13.51% | 0.979 |
| nway | 23277365ns | 20589137ns | 26265260ns | +54.25% | 0.721 |
| sequential | 26913749ns | 23734342ns | 30066939ns | +78.35% | 0.623 |
| std_parallel | 16838424ns | 15154328ns | 18346182ns | +11.58% | 0.996 |

## Performance model

- Peak throughput: **1.172 Gops/s** (head_tail_dyn; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 1.119 | 95.5% |
| head_tail_dyn | 1.100 | 93.8% |
| kernel_nway | 1.132 | 96.5% |
| kernel_seq | 0.990 | 84.4% |
| nway | 0.738 | 62.9% |
| sequential | 0.628 | 53.5% |
| std_parallel | 0.989 | 84.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 15097825ns | 15097825ns | base |
| head_tail_dyn | 15303423ns | 15303423ns | +1.36% |
| kernel_nway | 14845240ns | 14845240ns | -1.67% |
| kernel_seq | 17135733ns | 17135733ns | +13.50% |
| nway | 23282605ns | 23282605ns | +54.21% |
| sequential | 26920531ns | 26920531ns | +78.31% |
| std_parallel | 16845072ns | 16845072ns | +11.57% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 14989857ns | base | --- | [14386352, 15895880] | --- | --- | --- | --- |
| head_tail_dyn | 15250865ns | no significant difference | [-1119662, +1330396]ns | [14320536, 16320252] | no | 0.8250 | 0.6875 | 0 |
| kernel_nway | 14822778ns | no significant difference | [-1073859, +422984]ns | [14617869, 15076214] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 16944130ns | +1682933.1ns (+11.2%) | [+122956, +4309718]ns | [15246706, 19196860] | YES (adj: no) | 0.3281 | 0.2188 | 0 |
| nway | 22747691ns | +7706682.7ns (+51.4%) | [+5643452, +11209871]ns | [20819143, 26265260] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| sequential | 26726867ns | +11432952.5ns (+76.3%) | [+8369062, +15667146]ns | [23947442, 30066939] | YES (adj: no) | 0.0938 | 0.0313 | 0 |
| std_parallel | 16958753ns | +1806138.5ns (+12.0%) | [+102898, +3334149]ns | [15210339, 18346182] | YES (adj: no) | 0.3281 | 0.2188 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 15226842ns | -6.0% | -3.6% | +26.1% | +42.4% | +82.3% | +20.3% |
| 1 | 2 | 14464091ns | +7.8% | +2.0% | +14.0% | +68.7% | +84.6% | +14.2% |
| 1 | 3 | 15581155ns | +2.6% | -3.2% | +0.8% | +52.3% | +68.6% | +0.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.602 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.056 | ok |
| kernel_nway | -0.075 | ok |
| kernel_seq | -0.061 | ok |
| nway | -0.318 | moderate- |
| sequential | -0.038 | ok |
| std_parallel | -0.029 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 1/3, lost 2/3
- **kernel_nway**: won 2/3, lost 1/3
- **kernel_seq**: won 0/3, lost 3/3
- **nway**: won 0/3, lost 3/3
- **sequential**: won 0/3, lost 3/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 100424291.0ns | 15090696.0ns | 665.5% | HIGH |
| head_tail_dyn | 101895459.4ns | 15297217.6ns | 666.1% | HIGH |
| kernel_nway | 102668328.5ns | 14838953.4ns | 691.9% | HIGH |
| kernel_seq | 108944157.9ns | 17129231.9ns | 636.0% | HIGH |
| nway | 110171954.4ns | 23277364.7ns | 473.3% | HIGH |
| sequential | 115120412.4ns | 26913749.5ns | 427.7% | HIGH |
| std_parallel | 106460724.5ns | 16838424.4ns | 632.2% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 14322054.2-15895879.8 ns)
  14322054.2 |####################
  14400745.5 |########################################
  14479436.8 |
  14558128.0 |
  14636819.3 |
  14715510.6 |
  14794201.9 |
  14872893.2 |
  14951584.4 |
  15030275.7 |
  15108967.0 |
  15187658.3 |
  15266349.6 |
  15345040.8 |
  15423732.1 |####################
  15502423.4 |
  15581114.7 |
  15659806.0 |####################
  15738497.2 |
  15817188.5 |
  (0 below, 1 above range)

head_tail_dyn (n=6, range 14309050.9-16320252.2 ns)
  14309050.9 |########################################
  14409611.0 |
  14510171.0 |
  14610731.1 |
  14711291.2 |
  14811851.2 |
  14912411.3 |
  15012971.3 |
  15113531.4 |
  15214091.5 |########################################
  15314651.5 |
  15415211.6 |
  15515771.7 |
  15616331.7 |
  15716891.8 |
  15817451.8 |####################
  15918011.9 |
  16018572.0 |
  16119132.0 |
  16219692.1 |
  (0 below, 1 above range)

kernel_nway (n=6, range 14555863.8-15076213.6 ns)
  14555863.8 |########################################
  14581881.3 |
  14607898.8 |
  14633916.3 |
  14659933.8 |########################################
  14685951.2 |
  14711968.7 |
  14737986.2 |
  14764003.7 |
  14790021.2 |########################################
  14816038.7 |########################################
  14842056.2 |
  14868073.7 |
  14894091.2 |
  14920108.7 |
  14946126.2 |
  14972143.6 |
  14998161.1 |
  15024178.6 |
  15050196.1 |########################################
  (0 below, 1 above range)

kernel_seq (n=6, range 15014678.8-19196860.1 ns)
  15014678.8 |########################################
  15223787.9 |
  15432896.9 |########################################
  15642006.0 |
  15851115.1 |
  16060224.1 |
  16269333.2 |########################################
  16478442.3 |
  16687551.3 |
  16896660.4 |
  17105769.5 |
  17314878.5 |########################################
  17523987.6 |
  17733096.7 |
  17942205.7 |
  18151314.8 |
  18360423.9 |########################################
  18569532.9 |
  18778642.0 |
  18987751.1 |
  (0 below, 1 above range)

nway (n=6, range 20589136.6-26265260.5 ns)
  20589136.6 |########################################
  20872942.8 |########################################
  21156749.0 |
  21440555.2 |
  21724361.4 |
  22008167.6 |
  22291973.8 |########################################
  22575780.0 |
  22859586.2 |
  23143392.4 |########################################
  23427198.6 |
  23711004.7 |
  23994810.9 |
  24278617.1 |
  24562423.3 |
  24846229.5 |
  25130035.7 |
  25413841.9 |########################################
  25697648.1 |
  25981454.3 |
  (0 below, 1 above range)

sequential (n=6, range 23734342.3-30066939.2 ns)
  23734342.3 |########################################
  24050972.1 |########################################
  24367602.0 |########################################
  24684231.8 |
  25000861.7 |
  25317491.5 |
  25634121.4 |
  25950751.2 |
  26267381.1 |
  26584010.9 |
  26900640.8 |
  27217270.6 |
  27533900.5 |
  27850530.3 |
  28167160.2 |
  28483790.0 |
  28800419.9 |########################################
  29117049.7 |########################################
  29433679.6 |
  29750309.4 |
  (0 below, 1 above range)

std_parallel (n=6, range 15154327.5-18346181.8 ns)
  15154327.5 |########################################
  15313920.2 |
  15473512.9 |
  15633105.6 |
  15792698.3 |
  15952291.1 |
  16111883.8 |####################
  16271476.5 |
  16431069.2 |
  16590661.9 |
  16750254.6 |
  16909847.3 |
  17069440.1 |
  17229032.8 |
  17388625.5 |
  17548218.2 |####################
  17707810.9 |####################
  17867403.6 |
  18026996.3 |
  18186589.0 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=666.4% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=666.6% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=700.9% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=634.8% of algo (FFI overhead may distort results)
- **nway**: bridge=481.4% of algo (FFI overhead may distort results)
- **sequential**: bridge=431.6% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=594.7% of algo (FFI overhead may distort results)
