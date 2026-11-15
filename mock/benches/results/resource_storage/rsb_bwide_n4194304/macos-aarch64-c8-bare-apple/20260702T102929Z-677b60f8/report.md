# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Key findings

- **Baseline (wide_blob) is the fastest** at 47.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.06x (fastest 47.5 ns, slowest 145.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 109ns | 101ns | 99ns | 101ns | 142ns | base |
| wide_decomposed | 207ns | 196ns | 188ns | 195ns | 264ns | +90.48% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 48ns | 47ns | 50ns | base | 87504.386 |
| wide_decomposed | 154ns | 138ns | 200ns | +221.71% | 27199.533 |

## Performance model

- Peak throughput: **90200.086 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 88301.137 | 97.9% |
| wide_decomposed | 28846.657 | 32.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 109ns | 109ns | base |
| wide_decomposed | 207ns | 207ns | +90.48% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 48ns | base | --- | [47, 48] | --- | --- | --- | --- |
| wide_decomposed | 145ns | +97.5ns (+205.3%) | [+96, +98]ns | [143, 146] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide_blob | wide_decomposed |
|---|---|---|
| 1 | 47ns | +214.0% |
| 2 | 47ns | +208.9% |
| 3 | 48ns | +207.4% |
| 4 | 48ns | +203.8% |
| 5 | 47ns | +212.1% |
| 6 | 47ns | +212.4% |
| 7 | 48ns | +221.1% |
| 8 | 48ns | +298.5% |
| 9 | 49ns | +189.2% |
| 10 | 49ns | +187.0% |
| 11 | 49ns | +189.5% |
| 12 | 50ns | +197.6% |
| 13 | 49ns | +196.3% |
| 14 | 49ns | +202.0% |
| 15 | 49ns | +194.5% |
| 16 | 49ns | +195.7% |
| 17 | 48ns | +211.3% |
| 18 | 48ns | +209.9% |
| 19 | 48ns | +208.5% |
| 20 | 48ns | +208.2% |
| 21 | 47ns | +213.0% |
| 22 | 47ns | +210.4% |
| 23 | 48ns | +206.9% |
| 24 | 48ns | +208.2% |
| 25 | 47ns | +212.0% |
| 26 | 47ns | +209.2% |
| 27 | 47ns | +208.0% |
| 28 | 48ns | +208.3% |
| 29 | 48ns | +207.5% |
| 30 | 47ns | +207.6% |
| 31 | 47ns | +210.0% |
| 32 | 48ns | +207.8% |
| 33 | 47ns | +195.3% |
| 34 | 47ns | +201.5% |
| 35 | 47ns | +197.9% |
| 36 | 47ns | +203.8% |
| 37 | 47ns | +206.6% |
| 38 | 47ns | +208.8% |
| 39 | 47ns | +202.8% |
| 40 | 47ns | +202.5% |
| 41 | 50ns | +194.3% |
| 42 | 49ns | +870.8% |
| 43 | 50ns | +528.8% |
| 44 | 50ns | +570.9% |
| 45 | 50ns | +301.0% |
| 46 | 49ns | +398.0% |
| 47 | 49ns | +367.8% |
| 48 | 64ns | +421.4% |
| 49 | 47ns | +213.1% |
| 50 | 47ns | +206.6% |
| 51 | 47ns | +209.6% |
| 52 | 47ns | +212.0% |
| 53 | 48ns | +208.6% |
| 54 | 47ns | +207.6% |
| 55 | 48ns | +206.5% |
| 56 | 47ns | +210.1% |
| 57 | 46ns | +202.8% |
| 58 | 47ns | +197.6% |
| 59 | 47ns | +197.6% |
| 60 | 46ns | +197.8% |
| 61 | 46ns | +201.7% |
| 62 | 46ns | +208.3% |
| 63 | 46ns | +202.4% |
| 64 | 47ns | +194.5% |
| 65 | 50ns | +179.6% |
| 66 | 49ns | +194.3% |
| 67 | 50ns | +187.5% |
| 68 | 49ns | +192.2% |
| 69 | 49ns | +190.4% |
| 70 | 49ns | +188.8% |
| 71 | 49ns | +187.0% |
| 72 | 49ns | +186.4% |
| 73 | 48ns | +259.8% |
| 74 | 48ns | +209.4% |
| 75 | 47ns | +206.6% |
| 76 | 48ns | +200.8% |
| 77 | 48ns | +201.0% |
| 78 | 48ns | +197.3% |
| 79 | 48ns | +194.2% |
| 80 | 47ns | +200.2% |
| 81 | 48ns | +189.1% |
| 82 | 47ns | +192.8% |
| 83 | 46ns | +198.3% |
| 84 | 47ns | +192.1% |
| 85 | 47ns | +192.9% |
| 86 | 47ns | +189.0% |
| 87 | 47ns | +193.2% |
| 88 | 47ns | +193.4% |
| 89 | 46ns | +209.3% |
| 90 | 48ns | +188.7% |
| 91 | 48ns | +186.7% |
| 92 | 49ns | +180.5% |
| 93 | 49ns | +190.0% |
| 94 | 47ns | +194.9% |
| 95 | 49ns | +189.1% |
| 96 | 50ns | +179.5% |
| 97 | 50ns | +215.9% |
| 98 | 48ns | +221.0% |
| 99 | 47ns | +233.6% |
| 100 | 47ns | +240.9% |
| 101 | 47ns | +237.3% |
| 102 | 48ns | +226.6% |
| 103 | 47ns | +230.5% |
| 104 | 48ns | +229.1% |
| 105 | 49ns | +202.5% |
| 106 | 49ns | +203.7% |
| 107 | 49ns | +200.2% |
| 108 | 49ns | +199.4% |
| 109 | 49ns | +198.2% |
| 110 | 49ns | +195.7% |
| 111 | 49ns | +198.2% |
| 112 | 50ns | +198.4% |
| 113 | 47ns | +195.5% |
| 114 | 47ns | +193.8% |
| 115 | 46ns | +213.8% |
| 116 | 46ns | +200.9% |
| 117 | 46ns | +198.9% |
| 118 | 47ns | +196.4% |
| 119 | 47ns | +197.6% |
| 120 | 47ns | +195.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | 0.252 | moderate+ |
| wide_decomposed | 0.542 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide_decomposed**: won 0/120, lost 120/120

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 137.0ns | 47.9ns | 285.8% | HIGH |
| wide_decomposed | 332.7ns | 154.2ns | 215.8% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 46.5-50.1 ns)
     46.5 |
     46.7 |########################################
     46.9 |################
     47.0 |##############
     47.2 |#########################
     47.4 |############
     47.6 |##################
     47.8 |####
     47.9 |##########
     48.1 |####
     48.3 |##
     48.5 |####
     48.6 |#####################
     48.8 |##########
     49.0 |
     49.2 |#######################
     49.4 |########
     49.5 |####
     49.7 |
     49.9 |##
  (10 below, 4 above range)

wide_decomposed (n=120, range 138.4-199.9 ns)
    138.4 |############################
    141.5 |############
    144.5 |########################################
    147.6 |#######
    150.7 |
    153.8 |##
    156.8 |####
    159.9 |
    163.0 |
    166.0 |
    169.1 |
    172.2 |
    175.3 |
    178.3 |
    181.4 |
    184.5 |
    187.6 |
    190.6 |
    193.7 |
    196.8 |
  (11 below, 6 above range)

```

## Diagnostics

- **wide_blob**: bridge=282.6% of algo (FFI overhead may distort results)
- **wide_decomposed**: CV=28.1% (high variance, measurements may be unstable)
- **wide_decomposed**: autocorrelation=0.54 (measurement drift or warm-up artifact)
- **wide_decomposed**: bridge=212.9% of algo (FFI overhead may distort results)
