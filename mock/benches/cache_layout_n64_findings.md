# AoS vs SoA (column-store) layout under partial-field iteration

2 variants, 160 samples per variant.
Baseline: **layout_aos**

## Key findings

- **Baseline (layout_aos) is the fastest** at 1.2 ns median
- Spread: 1.00x (fastest 1.2 ns, slowest 1.2 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| layout_aos | 3878ns | 3843ns | 3147ns | 3721ns | 5077ns | base |
| layout_soa | 3744ns | 3592ns | 3126ns | 3651ns | 4640ns | -3.45% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| layout_aos | 1ns | 0ns | 2ns | base | 51.796 |
| layout_soa | 1ns | 0ns | 2ns | +0.81% | 51.380 |

## Performance model

- Peak throughput: **256.000 Gops/s** (layout_aos; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| layout_aos | 53.333 | 20.8% |
| layout_soa | 53.333 | 20.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| layout_aos | 3878ns | 3878ns | base |
| layout_soa | 3744ns | 3744ns | -3.45% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| layout_aos | 1ns | base | --- | [1, 1] | --- | --- | --- | --- |
| layout_soa | 1ns | no significant difference | [+0, +0]ns | [1, 1] | no | 1.0000 | 1.0000 | **25** (16%, HIGH) |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | layout_aos | layout_soa |
|---|---|---|
| 1 | 2ns | -84.0% |
| 2 | 1ns | +41.7% |
| 3 | 2ns | +0.0% |
| 4 | 0ns | +100.0% |
| 5 | 0ns | +0.0% |
| 6 | 1ns | +162.5% |
| 7 | 2ns | -61.9% |
| 8 | 2ns | -42.9% |
| 9 | 2ns | +0.0% |
| 10 | 0ns | +825.0% |
| 11 | 0ns | +325.0% |
| 12 | 1ns | +0.0% |
| 13 | 1ns | -66.7% |
| 14 | 1ns | +0.0% |
| 15 | 2ns | +100.0% |
| 16 | 2ns | +0.0% |
| 17 | 1ns | +108.3% |
| 18 | 1ns | -100.0% |
| 19 | 1ns | +41.7% |
| 20 | 2ns | -29.4% |
| 21 | 0ns | +200.0% |
| 22 | 2ns | +0.0% |
| 23 | 2ns | -29.4% |
| 24 | 0ns | +0.0% |
| 25 | 1ns | +0.0% |
| 26 | 0ns | +200.0% |
| 27 | 1ns | -33.3% |
| 28 | 1ns | -100.0% |
| 29 | 2ns | +19.0% |
| 30 | 2ns | +23.5% |
| 31 | 0ns | +200.0% |
| 32 | 2ns | -81.0% |
| 33 | 2ns | -42.9% |
| 34 | 2ns | -68.0% |
| 35 | 1ns | -50.0% |
| 36 | 2ns | -42.9% |
| 37 | 2ns | -42.9% |
| 38 | 0ns | +325.0% |
| 39 | 2ns | -100.0% |
| 40 | 1ns | +262.5% |
| 41 | 1ns | -33.3% |
| 42 | 1ns | +0.0% |
| 43 | 3ns | -27.6% |
| 44 | 1ns | +0.0% |
| 45 | 0ns | +100.0% |
| 46 | 1ns | +41.7% |
| 47 | 1ns | +50.0% |
| 48 | 2ns | -42.9% |
| 49 | 1ns | +212.5% |
| 50 | 2ns | +70.6% |
| 51 | 1ns | +0.0% |
| 52 | 0ns | +0.0% |
| 53 | 4ns | -59.5% |
| 54 | 1ns | +75.0% |
| 55 | 1ns | +41.7% |
| 56 | 0ns | +100.0% |
| 57 | 2ns | -42.9% |
| 58 | 1ns | -33.3% |
| 59 | 0ns | +200.0% |
| 60 | 2ns | -52.9% |
| 61 | 0ns | +0.0% |
| 62 | 2ns | -61.9% |
| 63 | 1ns | +41.7% |
| 64 | 1ns | +50.0% |
| 65 | 1ns | -50.0% |
| 66 | 0ns | +325.0% |
| 67 | 0ns | +0.0% |
| 68 | 1ns | -33.3% |
| 69 | 0ns | +100.0% |
| 70 | 0ns | +325.0% |
| 71 | 1ns | +312.5% |
| 72 | 1ns | +0.0% |
| 73 | 1ns | +112.5% |
| 74 | 0ns | +325.0% |
| 75 | 1ns | -50.0% |
| 76 | 2ns | +19.0% |
| 77 | 2ns | -68.0% |
| 78 | 1ns | +0.0% |
| 79 | 3ns | -86.2% |
| 80 | 2ns | -19.0% |
| 81 | 3ns | -100.0% |
| 82 | 2ns | -52.9% |
| 83 | 2ns | -52.9% |
| 84 | 0ns | +0.0% |
| 85 | 1ns | +162.5% |
| 86 | 0ns | +0.0% |
| 87 | 2ns | -52.9% |
| 88 | 0ns | -100.0% |
| 89 | 1ns | +0.0% |
| 90 | 2ns | -81.0% |
| 91 | 2ns | +0.0% |
| 92 | 2ns | -76.5% |
| 93 | 1ns | -100.0% |
| 94 | 0ns | +0.0% |
| 95 | 0ns | +0.0% |
| 96 | 3ns | -72.4% |
| 97 | 2ns | -100.0% |
| 98 | 1ns | -33.3% |
| 99 | 1ns | +50.0% |
| 100 | 2ns | -100.0% |
| 101 | 1ns | +0.0% |
| 102 | 1ns | +0.0% |
| 103 | 1ns | +0.0% |
| 104 | 2ns | +23.5% |
| 105 | 1ns | -50.0% |
| 106 | 2ns | -68.0% |
| 107 | 4ns | -54.1% |
| 108 | 0ns | +100.0% |
| 109 | 0ns | -100.0% |
| 110 | 1ns | +50.0% |
| 111 | 1ns | +162.5% |
| 112 | 0ns | +0.0% |
| 113 | 1ns | +362.5% |
| 114 | 2ns | -68.0% |
| 115 | 1ns | +0.0% |
| 116 | 0ns | +100.0% |
| 117 | 1ns | +50.0% |
| 118 | 1ns | +41.7% |
| 119 | 2ns | -29.4% |
| 120 | 2ns | -16.0% |
| 121 | 2ns | -61.9% |
| 122 | 2ns | -29.4% |
| 123 | 0ns | +325.0% |
| 124 | 1ns | -100.0% |
| 125 | 0ns | +325.0% |
| 126 | 1ns | -33.3% |
| 127 | 1ns | +75.0% |
| 128 | 0ns | +0.0% |
| 129 | 1ns | -50.0% |
| 130 | 1ns | +50.0% |
| 131 | 2ns | -19.0% |
| 132 | 1ns | +0.0% |
| 133 | 3ns | -48.5% |
| 134 | 3ns | -100.0% |
| 135 | 1ns | +0.0% |
| 136 | 0ns | +725.0% |
| 137 | 1ns | +212.5% |
| 138 | 0ns | +200.0% |
| 139 | 2ns | -42.9% |
| 140 | 3ns | -72.4% |
| 141 | 2ns | -42.9% |
| 142 | 1ns | -100.0% |
| 143 | 1ns | +0.0% |
| 144 | 0ns | +525.0% |
| 145 | 1ns | +0.0% |
| 146 | 0ns | +0.0% |
| 147 | 1ns | -50.0% |
| 148 | 1ns | -66.7% |
| 149 | 0ns | +325.0% |
| 150 | 1ns | -33.3% |
| 151 | 0ns | +100.0% |
| 152 | 1ns | +112.5% |
| 153 | 2ns | -32.0% |
| 154 | 0ns | +0.0% |
| 155 | 1ns | -33.3% |
| 156 | 2ns | -81.0% |
| 157 | 1ns | +0.0% |
| 158 | 0ns | +325.0% |
| 159 | 0ns | +0.0% |
| 160 | 2ns | +19.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| layout_aos | 0.097 | ok |
| layout_soa | 0.048 | ok |

**Consistency summary:**

- **layout_soa**: won 67/160, lost 56/160

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| layout_aos | 3.5ns | 1.2ns | 279.5% | HIGH |
| layout_soa | 3.2ns | 1.2ns | 257.3% | HIGH |

## Distribution (algo ns)

```
layout_aos (n=160, range 0.3-2.5 ns)
      0.3 |
      0.4 |################################
      0.5 |
      0.6 |
      0.7 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |###################################
      1.3 |
      1.4 |
      1.5 |
      1.6 |####################
      1.7 |
      1.8 |
      1.9 |
      2.0 |###########################
      2.2 |
      2.3 |
      2.4 |
  (12 below, 16 above range)

layout_soa (n=160, range 0.3-2.4 ns)
      0.3 |###############
      0.4 |
      0.5 |
      0.6 |
      0.7 |########################################
      0.8 |
      0.9 |
      1.0 |
      1.1 |##################################
      1.3 |
      1.4 |
      1.5 |
      1.6 |
      1.7 |############################
      1.8 |
      1.9 |
      2.0 |
      2.1 |############
      2.2 |
      2.3 |
  (12 below, 14 above range)

```

## Diagnostics

- **layout_aos**: CV=67.8% (high variance, measurements may be unstable)
- **layout_aos**: worst_20/best_20 = 10.0x (possible bimodal distribution)
- **layout_aos**: bridge=275.0% of algo (FFI overhead may distort results)
- **layout_soa**: CV=63.3% (high variance, measurements may be unstable)
- **layout_soa**: worst_20/best_20 = 8.1x (possible bimodal distribution)
- **layout_soa**: bridge=241.7% of algo (FFI overhead may distort results)
