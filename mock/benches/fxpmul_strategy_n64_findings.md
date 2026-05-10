# Fixed-point mul: Precise (i128 intermediate) vs Hot (i64 truncate)

2 variants, 160 samples per variant.
Baseline: **fxpmul_hot**

## Key findings

- **Baseline (fxpmul_hot) is the fastest** at 4.2 ns median
- 1 variant significantly slower than baseline
- Spread: 1.19x (fastest 4.2 ns, slowest 5.0 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| fxpmul_hot | 3440ns | 3074ns | 3057ns | 3333ns | 4145ns | base |
| fxpmul_precise | 3195ns | 3067ns | 2924ns | 3069ns | 3845ns | -7.12% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| fxpmul_hot | 4ns | 2ns | 6ns | base | 15.191 |
| fxpmul_precise | 5ns | 3ns | 7ns | +20.12% | 12.647 |

## Performance model

- Peak throughput: **26.089 Gops/s** (fxpmul_hot; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| fxpmul_hot | 15.238 | 58.4% |
| fxpmul_precise | 12.800 | 49.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| fxpmul_hot | 3440ns | 3440ns | base |
| fxpmul_precise | 3195ns | 3195ns | -7.12% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| fxpmul_hot | 4ns | base | --- | [4, 5] | --- | --- | --- | --- |
| fxpmul_precise | 5ns | +0.9ns (+21.4%) | [+0, +1]ns | [5, 5] | YES | 0.0000 | 0.0000 | 12 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | fxpmul_hot | fxpmul_precise |
|---|---|---|
| 1 | 4ns | +0.0% |
| 2 | 5ns | +7.4% |
| 3 | 5ns | -26.0% |
| 4 | 5ns | +16.0% |
| 5 | 2ns | +100.0% |
| 6 | 3ns | +0.0% |
| 7 | 4ns | +13.5% |
| 8 | 3ns | +27.6% |
| 9 | 5ns | +31.5% |
| 10 | 3ns | +39.4% |
| 11 | 2ns | +147.1% |
| 12 | 5ns | -26.0% |
| 13 | 2ns | +32.0% |
| 14 | 3ns | +113.8% |
| 15 | 3ns | +27.3% |
| 16 | 2ns | +294.1% |
| 17 | 6ns | -25.8% |
| 18 | 2ns | +168.0% |
| 19 | 4ns | +9.5% |
| 20 | 5ns | +0.0% |
| 21 | 5ns | +54.3% |
| 22 | 4ns | -21.6% |
| 23 | 6ns | -19.4% |
| 24 | 4ns | +24.3% |
| 25 | 4ns | +38.1% |
| 26 | 4ns | +35.1% |
| 27 | 4ns | -43.2% |
| 28 | 3ns | +44.8% |
| 29 | 5ns | +42.0% |
| 30 | 5ns | -14.8% |
| 31 | 5ns | -14.8% |
| 32 | 2ns | +32.0% |
| 33 | 5ns | +50.0% |
| 34 | 3ns | +127.3% |
| 35 | 5ns | -8.0% |
| 36 | 4ns | -40.5% |
| 37 | 2ns | +194.1% |
| 38 | 5ns | +0.0% |
| 39 | 4ns | +81.1% |
| 40 | 4ns | +35.1% |
| 41 | 5ns | +14.8% |
| 42 | 3ns | +58.6% |
| 43 | 3ns | +51.5% |
| 44 | 5ns | -34.0% |
| 45 | 4ns | +0.0% |
| 46 | 3ns | +0.0% |
| 47 | 3ns | +44.8% |
| 48 | 5ns | -14.8% |
| 49 | 5ns | -50.0% |
| 50 | 4ns | +38.1% |
| 51 | 5ns | +34.8% |
| 52 | 5ns | -34.0% |
| 53 | 4ns | +38.1% |
| 54 | 8ns | -50.7% |
| 55 | 5ns | -8.0% |
| 56 | 4ns | +19.0% |
| 57 | 4ns | +13.5% |
| 58 | 6ns | +8.1% |
| 59 | 6ns | -20.7% |
| 60 | 5ns | +0.0% |
| 61 | 8ns | -17.3% |
| 62 | 4ns | +67.6% |
| 63 | 6ns | -27.6% |
| 64 | 4ns | -67.6% |
| 65 | 5ns | -8.7% |
| 66 | 5ns | -38.9% |
| 67 | 4ns | +56.8% |
| 68 | 5ns | -16.0% |
| 69 | 7ns | +6.0% |
| 70 | 5ns | +38.9% |
| 71 | 4ns | +35.1% |
| 72 | 5ns | +34.8% |
| 73 | 5ns | +0.0% |
| 74 | 7ns | -31.3% |
| 75 | 3ns | +27.3% |
| 76 | 6ns | -27.6% |
| 77 | 4ns | -21.6% |
| 78 | 5ns | +14.8% |
| 79 | 4ns | +56.8% |
| 80 | 4ns | +35.1% |
| 81 | 5ns | -7.4% |
| 82 | 5ns | +0.0% |
| 83 | 5ns | +0.0% |
| 84 | 4ns | +35.1% |
| 85 | 5ns | +8.7% |
| 86 | 3ns | +87.9% |
| 87 | 2ns | +100.0% |
| 88 | 3ns | +27.3% |
| 89 | 4ns | +56.8% |
| 90 | 7ns | +16.9% |
| 91 | 4ns | -59.5% |
| 92 | 2ns | +116.0% |
| 93 | 5ns | +7.4% |
| 94 | 3ns | +75.8% |
| 95 | 2ns | +116.0% |
| 96 | 5ns | +24.0% |
| 97 | 5ns | -45.7% |
| 98 | 4ns | +35.1% |
| 99 | 5ns | +7.4% |
| 100 | 3ns | +51.5% |
| 101 | 5ns | +71.7% |
| 102 | 5ns | -8.7% |
| 103 | 3ns | +75.8% |
| 104 | 4ns | +38.1% |
| 105 | 3ns | +27.3% |
| 106 | 4ns | +9.5% |
| 107 | 6ns | -36.2% |
| 108 | 6ns | -25.8% |
| 109 | 4ns | +47.6% |
| 110 | 6ns | -53.2% |
| 111 | 6ns | +65.5% |
| 112 | 5ns | -14.8% |
| 113 | 6ns | +14.5% |
| 114 | 1ns | +250.0% |
| 115 | 5ns | -8.0% |
| 116 | 5ns | +0.0% |
| 117 | 4ns | +13.5% |
| 118 | 4ns | +102.7% |
| 119 | 5ns | +0.0% |
| 120 | 3ns | -12.1% |
| 121 | 2ns | +138.1% |
| 122 | 2ns | +132.0% |
| 123 | 4ns | -31.0% |
| 124 | 3ns | +75.8% |
| 125 | 2ns | +148.0% |
| 126 | 4ns | +9.5% |
| 127 | 5ns | -50.0% |
| 128 | 6ns | -36.2% |
| 129 | 5ns | -22.2% |
| 130 | 4ns | +28.6% |
| 131 | 5ns | -7.4% |
| 132 | 4ns | +28.6% |
| 133 | 4ns | +13.5% |
| 134 | 3ns | +100.0% |
| 135 | 5ns | +45.7% |
| 136 | 2ns | +168.0% |
| 137 | 3ns | +115.2% |
| 138 | 3ns | +39.4% |
| 139 | 1ns | +175.0% |
| 140 | 1ns | +525.0% |
| 141 | 4ns | +56.8% |
| 142 | 5ns | +45.7% |
| 143 | 4ns | +9.5% |
| 144 | 4ns | +35.1% |
| 145 | 2ns | +70.6% |
| 146 | 5ns | +26.1% |
| 147 | 2ns | +238.1% |
| 148 | 4ns | +24.3% |
| 149 | 3ns | +13.8% |
| 150 | 3ns | +158.6% |
| 151 | 5ns | +34.8% |
| 152 | 4ns | +59.5% |
| 153 | 5ns | +8.0% |
| 154 | 6ns | +15.5% |
| 155 | 7ns | -25.4% |
| 156 | 3ns | +127.3% |
| 157 | 5ns | +16.0% |
| 158 | 4ns | +47.6% |
| 159 | 7ns | -13.4% |
| 160 | 3ns | +127.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| fxpmul_hot | 0.093 | ok |
| fxpmul_precise | -0.051 | ok |

**Consistency summary:**

- **fxpmul_precise**: won 43/160, lost 105/160

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| fxpmul_hot | 4.9ns | 4.2ns | 115.8% | HIGH |
| fxpmul_precise | 5.0ns | 5.1ns | 99.0% | HIGH |

## Distribution (algo ns)

```
fxpmul_hot (n=160, range 2.5-6.0 ns)
      2.5 |################
      2.6 |
      2.8 |##############
      3.0 |
      3.2 |############################
      3.3 |
      3.5 |
      3.7 |########################################
      3.9 |
      4.0 |############################
      4.2 |
      4.4 |
      4.6 |###########################
      4.7 |
      4.9 |############################
      5.1 |
      5.3 |#########################
      5.5 |
      5.6 |###########
      5.8 |
  (9 below, 13 above range)

fxpmul_precise (n=160, range 3.1-7.0 ns)
      3.1 |############
      3.3 |
      3.5 |###########
      3.7 |
      3.9 |
      4.1 |##############################
      4.3 |
      4.5 |########################################
      4.7 |
      4.9 |###################################
      5.1 |
      5.3 |#########
      5.5 |
      5.7 |################################
      5.9 |
      6.1 |####################
      6.3 |
      6.5 |
      6.7 |##############
      6.9 |
  (14 below, 17 above range)

```

## Diagnostics

- **fxpmul_hot**: CV=30.4% (high variance, measurements may be unstable)
- **fxpmul_hot**: bridge=109.5% of algo (FFI overhead may distort results)
- **fxpmul_precise**: CV=27.7% (high variance, measurements may be unstable)
- **fxpmul_precise**: bridge=92.0% of algo (FFI overhead may distort results)
