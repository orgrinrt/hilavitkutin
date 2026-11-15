# Intra-resource member gather at M=64: contiguous blob vs scattered columns

2 variants, 120 samples per variant.
Baseline: **wide_blob**

## Key findings

- **Baseline (wide_blob) is the fastest** at 4366.8 ns median
- 1 variant significantly slower than baseline
- Spread: 1.87x (fastest 4366.8 ns, slowest 8179.4 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wide_blob | 4476ns | 4404ns | 4210ns | 4387ns | 5011ns | base |
| wide_decomposed | 8975ns | 8220ns | 8015ns | 8261ns | 12079ns | +100.50% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wide_blob | 4437ns | 4173ns | 4963ns | base | 3.693 |
| wide_decomposed | 8897ns | 7976ns | 11848ns | +100.52% | 1.842 |

## Performance model

- Peak throughput: **3.926 Gops/s** (wide_blob; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wide_blob | 3.752 | 95.6% |
| wide_decomposed | 2.003 | 51.0% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wide_blob | 4476ns | 4476ns | base |
| wide_decomposed | 8975ns | 8975ns | +100.50% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wide_blob | 4367ns | base | --- | [4263, 4386] | --- | --- | --- | --- |
| wide_decomposed | 8179ns | +3878.4ns (+88.8%) | [+3826, +3919]ns | [8115, 8338] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | wide_blob | wide_decomposed |
|---|---|---|
| 1 | 4198ns | +95.5% |
| 2 | 4176ns | +92.9% |
| 3 | 4162ns | +93.1% |
| 4 | 4228ns | +89.9% |
| 5 | 4233ns | +91.1% |
| 6 | 4259ns | +93.6% |
| 7 | 4380ns | +83.4% |
| 8 | 4544ns | +77.4% |
| 9 | 4722ns | +90.6% |
| 10 | 4795ns | +88.4% |
| 11 | 4724ns | +89.5% |
| 12 | 4666ns | +79.3% |
| 13 | 4382ns | +93.5% |
| 14 | 4388ns | +90.8% |
| 15 | 4381ns | +90.5% |
| 16 | 4380ns | +90.1% |
| 17 | 4720ns | +74.4% |
| 18 | 4870ns | +64.4% |
| 19 | 4719ns | +72.5% |
| 20 | 4928ns | +61.7% |
| 21 | 4690ns | +72.6% |
| 22 | 4432ns | +80.5% |
| 23 | 4407ns | +88.6% |
| 24 | 4424ns | +79.7% |
| 25 | 4246ns | +89.9% |
| 26 | 4348ns | +89.3% |
| 27 | 4199ns | +93.4% |
| 28 | 4161ns | +91.9% |
| 29 | 4171ns | +100.4% |
| 30 | 4164ns | +92.2% |
| 31 | 4180ns | +91.7% |
| 32 | 4162ns | +92.1% |
| 33 | 4239ns | +87.7% |
| 34 | 4267ns | +101.8% |
| 35 | 4303ns | +99.8% |
| 36 | 4205ns | +93.2% |
| 37 | 4175ns | +95.6% |
| 38 | 4170ns | +92.1% |
| 39 | 4175ns | +97.8% |
| 40 | 4180ns | +92.5% |
| 41 | 4186ns | +93.8% |
| 42 | 4204ns | +93.8% |
| 43 | 4173ns | +99.6% |
| 44 | 4245ns | +93.0% |
| 45 | 4361ns | +89.8% |
| 46 | 4725ns | +71.3% |
| 47 | 4724ns | +77.0% |
| 48 | 4319ns | +93.4% |
| 49 | 6282ns | +33.7% |
| 50 | 4386ns | +102.6% |
| 51 | 4423ns | +89.3% |
| 52 | 4499ns | +86.9% |
| 53 | 5375ns | +55.8% |
| 54 | 4385ns | +92.9% |
| 55 | 5955ns | +51.2% |
| 56 | 6223ns | +34.8% |
| 57 | 4434ns | +672.0% |
| 58 | 4481ns | +448.4% |
| 59 | 4597ns | +314.0% |
| 60 | 4405ns | +340.1% |
| 61 | 4641ns | +147.7% |
| 62 | 4423ns | +127.7% |
| 63 | 4524ns | +156.1% |
| 64 | 5020ns | +177.9% |
| 65 | 4746ns | +76.4% |
| 66 | 4726ns | +77.1% |
| 67 | 4722ns | +82.1% |
| 68 | 4746ns | +70.5% |
| 69 | 4462ns | +82.1% |
| 70 | 4759ns | +77.2% |
| 71 | 4468ns | +80.5% |
| 72 | 4379ns | +91.3% |
| 73 | 4237ns | +88.7% |
| 74 | 4200ns | +89.4% |
| 75 | 4206ns | +89.3% |
| 76 | 4719ns | +70.0% |
| 77 | 4531ns | +86.5% |
| 78 | 4421ns | +91.6% |
| 79 | 4404ns | +92.8% |
| 80 | 4378ns | +86.5% |
| 81 | 4748ns | +67.6% |
| 82 | 4647ns | +74.2% |
| 83 | 4482ns | +90.0% |
| 84 | 4762ns | +70.4% |
| 85 | 4716ns | +77.7% |
| 86 | 4449ns | +97.7% |
| 87 | 4186ns | +101.0% |
| 88 | 4166ns | +101.1% |
| 89 | 4226ns | +92.8% |
| 90 | 4240ns | +88.8% |
| 91 | 4216ns | +99.7% |
| 92 | 4375ns | +96.6% |
| 93 | 4379ns | +90.9% |
| 94 | 4284ns | +86.4% |
| 95 | 4417ns | +80.0% |
| 96 | 4180ns | +92.8% |
| 97 | 4163ns | +94.0% |
| 98 | 4179ns | +94.2% |
| 99 | 4242ns | +89.3% |
| 100 | 4372ns | +82.0% |
| 101 | 4305ns | +87.5% |
| 102 | 4386ns | +81.5% |
| 103 | 4311ns | +90.2% |
| 104 | 4354ns | +85.4% |
| 105 | 4186ns | +92.2% |
| 106 | 4190ns | +92.7% |
| 107 | 4162ns | +113.7% |
| 108 | 4164ns | +114.0% |
| 109 | 4288ns | +98.7% |
| 110 | 4240ns | +101.6% |
| 111 | 4187ns | +91.0% |
| 112 | 4224ns | +92.1% |
| 113 | 4162ns | +91.2% |
| 114 | 4229ns | +88.3% |
| 115 | 4197ns | +89.4% |
| 116 | 4235ns | +102.2% |
| 117 | 4264ns | +90.9% |
| 118 | 4226ns | +87.8% |
| 119 | 4228ns | +89.2% |
| 120 | 4200ns | +90.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wide_blob | 0.423 | moderate+ |
| wide_decomposed | 0.622 | HIGH+ (drift/warm-up) |

**Consistency summary:**

- **wide_decomposed**: won 0/120, lost 120/120

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wide_blob | 815.9ns | 4436.8ns | 18.4% | HIGH |
| wide_decomposed | 1020.9ns | 8896.9ns | 11.5% | HIGH |

## Distribution (algo ns)

```
wide_blob (n=120, range 4173.4-4963.2 ns)
   4173.4 |########################################
   4212.9 |################################
   4252.4 |##########
   4291.9 |########
   4331.4 |######
   4370.9 |################################
   4410.3 |################
   4449.8 |########
   4489.3 |####
   4528.8 |####
   4568.3 |##
   4607.8 |####
   4647.3 |##
   4686.8 |######################
   4726.3 |##########
   4765.8 |##
   4805.3 |
   4844.7 |##
   4884.2 |
   4923.7 |##
  (12 below, 5 above range)

wide_decomposed (n=120, range 7975.9-11848.2 ns)
   7975.9 |########################################
   8169.5 |#############
   8363.1 |###################
   8556.7 |####
   8750.3 |###
   8944.0 |###
   9137.6 |
   9331.2 |
   9524.8 |
   9718.4 |
   9912.1 |
  10105.7 |
  10299.3 |
  10492.9 |
  10686.5 |
  10880.2 |
  11073.8 |
  11267.4 |
  11461.0 |#
  11654.6 |
  (13 below, 5 above range)

```

## Diagnostics

- **wide_blob**: bridge=18.2% of algo (FFI overhead may distort results)
- **wide_decomposed**: CV=35.5% (high variance, measurements may be unstable)
- **wide_decomposed**: autocorrelation=0.62 (measurement drift or warm-up artifact)
- **wide_decomposed**: bridge=11.8% of algo (FFI overhead may distort results)
