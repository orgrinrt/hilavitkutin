# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Key findings

- **Baseline (layout_aos) is the fastest** at 2.5 ns median
- Spread: 1.16x (fastest 2.5 ns, slowest 2.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 3765ns | 3424ns | 3064ns | 3447ns | 5419ns | base |
| layout_soa | 3576ns | 3436ns | 3086ns | 3480ns | 4354ns | -5.02% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 3ns | 1ns | 4ns | base | 97.154 |
| layout_soa | 3ns | 1ns | 5ns | +7.59% | 90.300 |

## Performance model

- Peak throughput: **232.068 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 256

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 102.400 | 44.1% |
| layout_soa | 88.276 | 38.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 3765ns | 3765ns | base |
| layout_soa | 3576ns | 3576ns | -5.02% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 2ns | base | --- | [2, 3] | --- | --- | --- | --- |
| layout_soa | 3ns | no significant difference | [+0, +0]ns | [2, 3] | no | 0.3088 | 0.3088 | **21** (13%, HIGH) |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | layout_aos | layout_soa |
|---|---|---|
| 1 | 4ns | +35.1% |
| 2 | 4ns | -43.2% |
| 3 | 3ns | -27.6% |
| 4 | 2ns | -16.0% |
| 5 | 3ns | -24.2% |
| 6 | 3ns | -41.4% |
| 7 | 2ns | +0.0% |
| 8 | 3ns | +13.8% |
| 9 | 3ns | -63.6% |
| 10 | 3ns | +0.0% |
| 11 | 2ns | +32.0% |
| 12 | 4ns | -59.5% |
| 13 | 2ns | -16.0% |
| 14 | 1ns | -33.3% |
| 15 | 6ns | -63.8% |
| 16 | 3ns | +0.0% |
| 17 | 4ns | -21.6% |
| 18 | 2ns | +84.0% |
| 19 | 2ns | +38.1% |
| 20 | 2ns | +70.6% |
| 21 | 2ns | -32.0% |
| 22 | 3ns | -27.6% |
| 23 | 4ns | -40.5% |
| 24 | 2ns | +0.0% |
| 25 | 3ns | +0.0% |
| 26 | 1ns | +250.0% |
| 27 | 2ns | +0.0% |
| 28 | 6ns | -36.2% |
| 29 | 2ns | -19.0% |
| 30 | 1ns | +0.0% |
| 31 | 4ns | -40.5% |
| 32 | 3ns | +44.8% |
| 33 | 4ns | +59.5% |
| 34 | 2ns | +23.5% |
| 35 | 2ns | +176.2% |
| 36 | 3ns | +58.6% |
| 37 | 3ns | +13.8% |
| 38 | 2ns | +100.0% |
| 39 | 5ns | -28.3% |
| 40 | 4ns | -21.4% |
| 41 | 2ns | +100.0% |
| 42 | 2ns | +32.0% |
| 43 | 5ns | -50.0% |
| 44 | 1ns | +75.0% |
| 45 | 3ns | +12.1% |
| 46 | 2ns | +70.6% |
| 47 | 2ns | -52.9% |
| 48 | 4ns | +9.5% |
| 49 | 3ns | -36.4% |
| 50 | 1ns | +312.5% |
| 51 | 3ns | -13.8% |
| 52 | 2ns | -32.0% |
| 53 | 4ns | -40.5% |
| 54 | 2ns | +47.1% |
| 55 | 1ns | +162.5% |
| 56 | 6ns | -43.1% |
| 57 | 4ns | +0.0% |
| 58 | 4ns | -54.1% |
| 59 | 3ns | +27.6% |
| 60 | 6ns | -36.2% |
| 61 | 2ns | +38.1% |
| 62 | 2ns | +16.0% |
| 63 | 2ns | -32.0% |
| 64 | 0ns | +0.0% |
| 65 | 1ns | +362.5% |
| 66 | 2ns | +0.0% |
| 67 | 1ns | +383.3% |
| 68 | 2ns | +16.0% |
| 69 | 0ns | +825.0% |
| 70 | 1ns | +262.5% |
| 71 | 1ns | +175.0% |
| 72 | 3ns | +12.1% |
| 73 | 1ns | +250.0% |
| 74 | 1ns | +75.0% |
| 75 | 1ns | +0.0% |
| 76 | 2ns | +138.1% |
| 77 | 2ns | +0.0% |
| 78 | 1ns | +50.0% |
| 79 | 1ns | +250.0% |
| 80 | 2ns | +19.0% |
| 81 | 3ns | +27.6% |
| 82 | 1ns | +141.7% |
| 83 | 5ns | -45.7% |
| 84 | 4ns | -43.2% |
| 85 | 3ns | +39.4% |
| 86 | 4ns | -21.6% |
| 87 | 3ns | +27.6% |
| 88 | 5ns | -50.0% |
| 89 | 2ns | +32.0% |
| 90 | 3ns | -41.4% |
| 91 | 3ns | +13.8% |
| 92 | 4ns | -78.4% |
| 93 | 4ns | +35.1% |
| 94 | 3ns | +0.0% |
| 95 | 2ns | -32.0% |
| 96 | 5ns | -34.0% |
| 97 | 2ns | +47.1% |
| 98 | 2ns | +0.0% |
| 99 | 3ns | +27.3% |
| 100 | 4ns | +56.8% |
| 101 | 2ns | +138.1% |
| 102 | 4ns | -21.6% |
| 103 | 2ns | +19.0% |
| 104 | 1ns | -100.0% |
| 105 | 3ns | +27.3% |
| 106 | 5ns | -54.3% |
| 107 | 2ns | +0.0% |
| 108 | 4ns | -71.4% |
| 109 | 2ns | +19.0% |
| 110 | 3ns | +0.0% |
| 111 | 3ns | -13.8% |
| 112 | 2ns | +70.6% |
| 113 | 2ns | +217.6% |
| 114 | 1ns | +75.0% |
| 115 | 2ns | -52.0% |
| 116 | 4ns | -50.0% |
| 117 | 2ns | +100.0% |
| 118 | 4ns | -43.2% |
| 119 | 3ns | -13.8% |
| 120 | 4ns | -21.4% |
| 121 | 2ns | +0.0% |
| 122 | 1ns | +175.0% |
| 123 | 1ns | -66.7% |
| 124 | 2ns | -52.9% |
| 125 | 2ns | -52.9% |
| 126 | 2ns | -52.0% |
| 127 | 2ns | -19.0% |
| 128 | 2ns | -52.9% |
| 129 | 2ns | +48.0% |
| 130 | 2ns | -52.9% |
| 131 | 5ns | -58.0% |
| 132 | 3ns | -41.4% |
| 133 | 2ns | -76.5% |
| 134 | 1ns | +0.0% |
| 135 | 2ns | +0.0% |
| 136 | 0ns | +425.0% |
| 137 | 1ns | +50.0% |
| 138 | 3ns | -27.6% |
| 139 | 2ns | +94.1% |
| 140 | 2ns | -68.0% |
| 141 | 2ns | +76.2% |
| 142 | 4ns | -21.6% |
| 143 | 2ns | +48.0% |
| 144 | 2ns | +0.0% |
| 145 | 2ns | +170.6% |
| 146 | 1ns | +108.3% |
| 147 | 2ns | +19.0% |
| 148 | 3ns | +87.9% |
| 149 | 5ns | -8.7% |
| 150 | 1ns | +208.3% |
| 151 | 2ns | +119.0% |
| 152 | 2ns | +38.1% |
| 153 | 2ns | +57.1% |
| 154 | 2ns | +0.0% |
| 155 | 4ns | -10.8% |
| 156 | 4ns | +45.9% |
| 157 | 3ns | +0.0% |
| 158 | 4ns | -11.9% |
| 159 | 2ns | +32.0% |
| 160 | 2ns | +94.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.110 | ok |
| layout_soa | 0.098 | ok |

**Consistency summary:**

- **layout_soa**: won 63/160, lost 75/160

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.4ns | 2.6ns | 129.5% | HIGH |
| layout_soa | 3.7ns | 2.8ns | 130.4% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 1.1-4.4 ns)
      1.1 |##############################
      1.3 |
      1.4 |
      1.6 |######################################
      1.8 |
      1.9 |
      2.1 |####################################
      2.3 |
      2.4 |########################################
      2.6 |
      2.8 |##############################
      2.9 |
      3.1 |
      3.3 |###########################
      3.4 |
      3.6 |###########################
      3.7 |
      3.9 |
      4.1 |####################
      4.2 |
  (10 below, 12 above range)

layout_soa (n=160, range 1.2-4.7 ns)
      1.2 |############
      1.4 |
      1.5 |#########################
      1.7 |
      1.9 |
      2.1 |######################################
      2.2 |
      2.4 |##############################
      2.6 |
      2.8 |######################
      2.9 |
      3.1 |########################################
      3.3 |
      3.5 |
      3.7 |########################
      3.8 |
      4.0 |
      4.2 |############
      4.4 |
      4.5 |#########
  (12 below, 13 above range)

```

## Diagnostics

- **layout_aos**: CV=45.5% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **layout_aos**: bridge=132.0% of algo (FFI overhead may distort results)
- **layout_soa**: CV=44.8% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 4.0x (possible bimodal distribution)
- **layout_soa**: bridge=113.8% of algo (FFI overhead may distort results)
