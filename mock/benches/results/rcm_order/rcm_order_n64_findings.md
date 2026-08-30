# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Key findings

- **Fastest: rcm_half** at 95195.8 ns median (-5.0% vs baseline)
- Spread: 1.05x (fastest 95195.8 ns, slowest 100170.9 ns)

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 103117ns | 104479ns | 87450ns | 99900ns | 115776ns | base |
| rcm_half | 104122ns | 99542ns | 87247ns | 96345ns | 124226ns | +0.98% |
| rcm_rev | 106943ns | 102125ns | 85504ns | 96615ns | 133153ns | +3.71% |
| rcm_scr | 104081ns | 100604ns | 86063ns | 98565ns | 121366ns | +0.94% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 98466ns | 84306ns | 110434ns | base | 0.001 |
| rcm_half | 99818ns | 83612ns | 119065ns | +1.37% | 0.001 |
| rcm_rev | 102628ns | 82542ns | 127909ns | +4.23% | 0.001 |
| rcm_scr | 100220ns | 83214ns | 116596ns | +1.78% | 0.001 |

## Performance model

- Peak throughput: **0.001 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.001 | 82.4% |
| rcm_half | 0.001 | 86.7% |
| rcm_rev | 0.001 | 84.9% |
| rcm_scr | 0.001 | 85.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 103117ns | 103117ns | base |
| rcm_half | 104122ns | 104122ns | +0.98% |
| rcm_rev | 106943ns | 106943ns | +3.71% |
| rcm_scr | 104081ns | 104081ns | +0.94% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 100171ns | base | --- | [88469, 105579] | --- | --- | --- | --- |
| rcm_half | 95196ns | no significant difference | [-10071, +8356]ns | [88325, 106942] | no | 1.0000 | 0.8145 | 0 |
| rcm_rev | 97219ns | no significant difference | [-11290, +13346]ns | [84942, 104644] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 96877ns | no significant difference | [-4167, +5917]ns | [90419, 108148] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop e2e, Δ vs baseline)

| Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|
| 1 | 107250ns | -17.1% | -22.7% | -6.0% |
| 2 | 96979ns | -11.6% | -15.6% | +2.2% |
| 3 | 82996ns | +15.9% | +46.6% | +14.0% |
| 4 | 115046ns | -5.9% | -15.4% | -25.7% |
| 5 | 109962ns | -3.8% | +11.0% | -1.7% |
| 6 | 109462ns | -14.9% | +61.4% | +0.0% |
| 7 | 116975ns | +20.6% | +17.8% | +0.7% |
| 8 | 86912ns | +24.3% | +11.7% | +41.0% |
| 9 | 97417ns | +3.6% | -7.7% | +36.9% |
| 10 | 103029ns | -21.8% | +4.2% | -20.6% |
| 11 | 85200ns | +3.0% | +19.3% | -0.7% |
| 12 | 84721ns | +35.8% | +20.3% | -1.8% |
| 13 | 89071ns | +52.7% | -6.8% | +1.2% |
| 14 | 103908ns | -8.6% | -19.8% | +4.2% |
| 15 | 86450ns | +2.2% | -1.7% | +8.7% |
| 16 | 90488ns | -6.0% | +4.7% | +18.6% |
| 17 | 103604ns | -17.7% | -20.0% | -12.5% |
| 18 | 102925ns | -7.3% | -2.6% | -10.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.134 | ok |
| rcm_half | 0.238 | moderate+ |
| rcm_rev | 0.472 | moderate+ |
| rcm_scr | 0.358 | moderate+ |

**Consistency summary:**

- **rcm_half**: won 10/18, lost 8/18
- **rcm_rev**: won 9/18, lost 9/18
- **rcm_scr**: won 8/18, lost 9/18

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 2944046.1ns | 98466.4ns | 2989.9% | HIGH |
| rcm_half | 2925456.0ns | 99818.3ns | 2930.8% | HIGH |
| rcm_rev | 2987230.8ns | 102628.0ns | 2910.7% | HIGH |
| rcm_scr | 2941112.5ns | 100219.9ns | 2934.7% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 84305.5-110434.0 ns)
  84305.5 |##########################
  85612.0 |##########################
  86918.4 |
  88224.8 |#############
  89531.2 |#############
  90837.7 |
  92144.1 |
  93450.5 |
  94756.9 |
  96063.4 |#############
  97369.8 |#############
  98676.2 |
  99982.6 |
  101289.0 |
  102595.5 |########################################
  103901.9 |#############
  105208.3 |
  106514.7 |#############
  107821.2 |
  109127.6 |##########################
  (1 below, 2 above range)

rcm_half (n=18, range 83612.5-119065.3 ns)
  83612.5 |##########################
  85385.1 |#############
  87157.8 |########################################
  88930.4 |
  90703.1 |
  92475.7 |#############
  94248.3 |##########################
  96021.0 |#############
  97793.6 |
  99566.3 |#############
  101338.9 |
  103111.5 |
  104884.2 |#############
  106656.8 |##########################
  108429.4 |
  110202.1 |
  111974.7 |
  113747.4 |#############
  115520.0 |
  117292.6 |
  (1 below, 2 above range)

rcm_rev (n=18, range 82541.7-127909.0 ns)
  82541.7 |########################################
  84810.0 |##########
  87078.4 |
  89346.8 |##########
  91615.1 |
  93883.5 |##########
  96151.9 |####################
  98420.2 |##########
  100688.6 |####################
  102957.0 |
  105225.4 |##########
  107493.7 |
  109762.1 |
  112030.5 |
  114298.8 |
  116567.2 |
  118835.6 |
  121103.9 |####################
  123372.3 |
  125640.7 |
  (1 below, 2 above range)

rcm_scr (n=18, range 83213.9-116595.8 ns)
  83213.9 |##########################
  84883.0 |#############
  86552.1 |
  88221.2 |
  89890.3 |##########################
  91559.4 |#############
  93228.5 |##########################
  94897.6 |
  96566.7 |
  98235.8 |#############
  99904.9 |#############
  101574.0 |
  103243.1 |
  104912.2 |
  106581.3 |########################################
  108250.4 |#############
  109919.5 |
  111588.6 |
  113257.7 |
  114926.8 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2899.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=3042.1% of algo (FFI overhead may distort results)
- **rcm_rev**: CV=23.0% (high variance, measurements may be unstable)
- **rcm_rev**: bridge=3022.0% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2965.9% of algo (FFI overhead may distort results)
