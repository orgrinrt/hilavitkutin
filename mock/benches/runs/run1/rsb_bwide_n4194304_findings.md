# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Key findings

- **Baseline (wide_blob) is the fastest** at 47.5 ns median
- 1 variant significantly slower than baseline
- Spread: 3.03x (fastest 47.5 ns, slowest 144.1 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 89ns | 84ns | 81ns | 85ns | 110ns | base |
| wide_decomposed | 188ns | 184ns | 176ns | 186ns | 207ns | +112.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 48ns | 46ns | 52ns | base | 87175.502 |
| wide_decomposed | 148ns | 139ns | 162ns | +208.00% | 28303.397 |

## Performance model

- Peak throughput: **91997.163 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 4194304

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 88301.137 | 96.0% |
| wide_decomposed | 29106.898 | 31.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 89ns | 89ns | base |
| wide_decomposed | 188ns | 188ns | +112.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 48ns | base | --- | [47, 48] | --- | --- | --- | --- |
| wide_decomposed | 144ns | +96.8ns (+203.7%) | [+96, +98]ns | [143, 146] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide_blob | wide_decomposed |
|---|---|---|
| 1 | 46ns | +240.2% |
| 2 | 46ns | +241.9% |
| 3 | 48ns | +231.6% |
| 4 | 48ns | +231.6% |
| 5 | 47ns | +234.1% |
| 6 | 49ns | +207.6% |
| 7 | 50ns | +189.5% |
| 8 | 43ns | +237.2% |
| 9 | 47ns | +204.1% |
| 10 | 48ns | +194.2% |
| 11 | 45ns | +217.9% |
| 12 | 47ns | +206.9% |
| 13 | 48ns | +198.5% |
| 14 | 45ns | +222.1% |
| 15 | 49ns | +187.6% |
| 16 | 46ns | +203.0% |
| 17 | 46ns | +205.9% |
| 18 | 46ns | +205.9% |
| 19 | 46ns | +205.3% |
| 20 | 46ns | +203.2% |
| 21 | 47ns | +198.3% |
| 22 | 48ns | +201.7% |
| 23 | 47ns | +206.0% |
| 24 | 48ns | +203.6% |
| 25 | 48ns | +206.7% |
| 26 | 47ns | +206.6% |
| 27 | 48ns | +206.2% |
| 28 | 48ns | +220.5% |
| 29 | 50ns | +197.2% |
| 30 | 49ns | +195.7% |
| 31 | 48ns | +206.7% |
| 32 | 49ns | +202.3% |
| 33 | 44ns | +216.2% |
| 34 | 47ns | +201.7% |
| 35 | 48ns | +191.3% |
| 36 | 50ns | +181.5% |
| 37 | 54ns | +163.6% |
| 38 | 47ns | +350.0% |
| 39 | 46ns | +260.6% |
| 40 | 47ns | +190.9% |
| 41 | 46ns | +210.4% |
| 42 | 48ns | +192.4% |
| 43 | 48ns | +193.5% |
| 44 | 46ns | +212.9% |
| 45 | 48ns | +192.7% |
| 46 | 48ns | +196.8% |
| 47 | 47ns | +196.6% |
| 48 | 47ns | +203.0% |
| 49 | 51ns | +175.4% |
| 50 | 54ns | +193.9% |
| 51 | 53ns | +194.7% |
| 52 | 52ns | +205.0% |
| 53 | 52ns | +202.5% |
| 54 | 52ns | +199.4% |
| 55 | 54ns | +195.7% |
| 56 | 52ns | +204.1% |
| 57 | 47ns | +199.4% |
| 58 | 49ns | +194.7% |
| 59 | 47ns | +216.0% |
| 60 | 49ns | +197.1% |
| 61 | 47ns | +199.6% |
| 62 | 47ns | +194.9% |
| 63 | 48ns | +190.4% |
| 64 | 46ns | +197.4% |
| 65 | 46ns | +211.7% |
| 66 | 46ns | +205.6% |
| 67 | 48ns | +200.4% |
| 68 | 46ns | +208.6% |
| 69 | 48ns | +195.5% |
| 70 | 47ns | +319.7% |
| 71 | 46ns | +203.9% |
| 72 | 46ns | +206.7% |
| 73 | 52ns | +170.8% |
| 74 | 46ns | +215.8% |
| 75 | 48ns | +201.6% |
| 76 | 48ns | +196.0% |
| 77 | 52ns | +180.5% |
| 78 | 48ns | +207.4% |
| 79 | 50ns | +193.2% |
| 80 | 50ns | +191.0% |
| 81 | 53ns | +198.7% |
| 82 | 53ns | +198.7% |
| 83 | 53ns | +197.0% |
| 84 | 53ns | +195.5% |
| 85 | 54ns | +189.1% |
| 86 | 53ns | +196.6% |
| 87 | 53ns | +198.3% |
| 88 | 52ns | +202.5% |
| 89 | 47ns | +192.9% |
| 90 | 48ns | +184.8% |
| 91 | 47ns | +185.5% |
| 92 | 47ns | +196.6% |
| 93 | 49ns | +186.4% |
| 94 | 45ns | +207.1% |
| 95 | 46ns | +203.0% |
| 96 | 46ns | +200.4% |
| 97 | 45ns | +247.2% |
| 98 | 47ns | +230.4% |
| 99 | 47ns | +235.5% |
| 100 | 47ns | +234.0% |
| 101 | 50ns | +219.7% |
| 102 | 48ns | +226.1% |
| 103 | 46ns | +240.9% |
| 104 | 47ns | +239.8% |
| 105 | 47ns | +238.0% |
| 106 | 52ns | +205.0% |
| 107 | 48ns | +229.6% |
| 108 | 47ns | +232.3% |
| 109 | 47ns | +232.6% |
| 110 | 48ns | +231.9% |
| 111 | 46ns | +240.0% |
| 112 | 46ns | +237.6% |
| 113 | 48ns | +199.2% |
| 114 | 51ns | +178.5% |
| 115 | 46ns | +208.3% |
| 116 | 46ns | +212.5% |
| 117 | 47ns | +204.1% |
| 118 | 48ns | +193.8% |
| 119 | 44ns | +220.9% |
| 120 | 46ns | +203.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | 0.555 | HIGH+ (drift/warm-up) |
| wide_decomposed | 0.380 | moderate+ |

**Consistency summary:**

- **wide_decomposed**: won 0/120, lost 120/120

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 787.8ns | 48.1ns | 1637.4% | HIGH |
| wide_decomposed | 975.8ns | 148.2ns | 658.5% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 45.6-52.3 ns)
     45.6 |#############
     45.9 |#############
     46.3 |##################################
     46.6 |#####################
     46.9 |#####################################
     47.3 |########################################
     47.6 |################
     47.9 |#############
     48.3 |################
     48.6 |#############
     48.9 |##
     49.3 |##
     49.6 |########
     49.9 |#####
     50.3 |##
     50.6 |##
     51.0 |
     51.3 |##
     51.6 |##########
     52.0 |
  (9 below, 15 above range)

wide_decomposed (n=120, range 138.9-162.4 ns)
    138.9 |#########################
    140.0 |################################
    141.2 |#########################
    142.4 |##############################
    143.6 |##############
    144.8 |#########
    145.9 |################
    147.1 |#######
    148.3 |####
    149.5 |##
    150.6 |
    151.8 |
    153.0 |
    154.2 |##
    155.3 |##############
    156.5 |########################################
    157.7 |#########################
    158.9 |####
    160.1 |
    161.2 |
  (8 below, 3 above range)

```

## Diagnostics

- **wide_blob**: autocorrelation=0.55 (measurement drift or warm-up artifact)
- **wide_blob**: bridge=1607.4% of algo (FFI overhead may distort results)
- **wide_decomposed**: bridge=664.3% of algo (FFI overhead may distort results)
