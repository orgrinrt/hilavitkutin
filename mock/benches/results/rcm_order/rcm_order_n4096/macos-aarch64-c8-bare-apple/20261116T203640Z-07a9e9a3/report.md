# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 2.0% (fastest 8.40 ms to slowest 8.57 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

## Key findings

- **Fastest: rcm_half** at 8403160.4 ns median (-0.8% vs baseline)
- Spread: 1.02x (fastest 8403160.4 ns, slowest 8569285.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | A1-1 dispatch order: column-adjacency dose response across valid topo orders |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c304d312345678 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 8508435ns | 8477435ns | 8340846ns | 8459401ns | 8665781ns | base |
| rcm_half | 8434835ns | 8409965ns | 8308690ns | 8377493ns | 8583921ns | -0.87% |
| rcm_rev | 8572439ns | 8482971ns | 8299012ns | 8460471ns | 8877103ns | +0.75% |
| rcm_scr | 8607039ns | 8576177ns | 8342131ns | 8538393ns | 8842462ns | +1.16% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 8501673ns | 8334093ns | 8658685ns | base | 0.000 |
| rcm_half | 8428191ns | 8302321ns | 8577164ns | -0.86% | 0.000 |
| rcm_rev | 8565650ns | 8292749ns | 8870094ns | +0.75% | 0.000 |
| rcm_scr | 8600409ns | 8335617ns | 8835617ns | +1.16% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_rev; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 97.9% |
| rcm_half | 0.000 | 98.7% |
| rcm_rev | 0.000 | 97.8% |
| rcm_scr | 0.000 | 96.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 8508435ns | 8508435ns | base |
| rcm_half | 8434835ns | 8434835ns | -0.87% |
| rcm_rev | 8572439ns | 8572439ns | +0.75% |
| rcm_scr | 8607039ns | 8607039ns | +1.16% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 8470975ns | base | --- | [8410583, 8584456] | --- | --- | --- | --- |
| rcm_half | 8403160ns | no significant difference | [-165390, +47210]ns | [8333569, 8488315] | no | 0.3568 | 0.2379 | 0 |
| rcm_rev | 8476010ns | no significant difference | [-115100, +224052]ns | [8373717, 8698398] | no | 0.3568 | 0.2379 | 0 |
| rcm_scr | 8569285ns | no significant difference | [-127435, +189992]ns | [8501538, 8658683] | no | 0.4807 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 8440233ns | -1.0% | +0.4% | +1.8% |
| 1 | 2 | 8369379ns | +2.2% | +1.2% | +2.3% |
| 1 | 3 | 8365438ns | -0.0% | +2.4% | +6.5% |
| 2 | 1 | 8540242ns | -2.4% | -2.7% | -2.1% |
| 2 | 2 | 8465688ns | -2.0% | -1.3% | +1.1% |
| 2 | 3 | 8624646ns | -1.2% | +3.7% | -0.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.096 | ok |
| rcm_half | -0.292 | moderate- |
| rcm_rev | -0.169 | ok |
| rcm_scr | -0.475 | moderate- |

**Consistency summary:**

- **rcm_half**: won 4/6, lost 1/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 2/6, lost 4/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 178757148.6ns | 8501673.4ns | 2102.6% | HIGH |
| rcm_half | 178741819.7ns | 8428191.2ns | 2120.8% | HIGH |
| rcm_rev | 179303613.0ns | 8565650.2ns | 2093.3% | HIGH |
| rcm_scr | 179780844.2ns | 8600408.8ns | 2090.4% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 8334093.0-8658684.7 ns)
  8334093.0 |
  8350322.6 |#############
  8366552.2 |##########################
  8382781.8 |
  8399011.4 |
  8415241.0 |
  8431470.5 |#############
  8447700.1 |#############
  8463929.7 |########################################
  8480159.3 |#############
  8496388.9 |
  8512618.5 |
  8528848.0 |#############
  8545077.6 |
  8561307.2 |#############
  8577536.8 |
  8593766.4 |#############
  8609996.0 |##########################
  8626225.5 |
  8642455.1 |
  (2 below, 2 above range)

rcm_half (n=18, range 8302320.8-8577163.9 ns)
  8302320.8 |##########################
  8316063.0 |#############
  8329805.1 |#############
  8343547.3 |#############
  8357289.4 |#############
  8371031.6 |
  8384773.8 |
  8398515.9 |########################################
  8412258.1 |
  8426000.2 |#############
  8439742.4 |
  8453484.5 |#############
  8467226.7 |
  8480968.8 |
  8494711.0 |
  8508453.1 |#############
  8522195.3 |
  8535937.5 |
  8549679.6 |##########################
  8563421.8 |
  (2 below, 2 above range)

rcm_rev (n=18, range 8292748.6-8870093.7 ns)
  8292748.6 |########################################
  8321615.9 |
  8350483.1 |#############
  8379350.4 |#############
  8408217.7 |
  8437084.9 |#############
  8465952.2 |########################################
  8494819.4 |
  8523686.7 |
  8552553.9 |#############
  8581421.2 |#############
  8610288.4 |#############
  8639155.7 |
  8668022.9 |
  8696890.2 |
  8725757.5 |
  8754624.7 |#############
  8783492.0 |
  8812359.2 |
  8841226.5 |#############
  (1 below, 3 above range)

rcm_scr (n=18, range 8335616.7-8835616.7 ns)
  8335616.7 |########################################
  8360616.7 |
  8385616.7 |
  8410616.7 |
  8435616.7 |
  8460616.7 |#############
  8485616.7 |
  8510616.7 |#############
  8535616.7 |########################################
  8560616.7 |#############
  8585616.7 |########################################
  8610616.7 |
  8635616.7 |
  8660616.7 |
  8685616.7 |
  8710616.7 |##########################
  8735616.7 |
  8760616.7 |
  8785616.7 |
  8810616.7 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2109.0% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2126.5% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=2109.1% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=2087.2% of algo (FFI overhead may distort results)
