# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (rcm_adj) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline rcm_adj has the worst median (37.98 ms). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest rcm_half at 37.83 ms).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 37.83 ms to slowest 37.98 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (149.35 us) is smaller than the fastest variant's own run-to-run std-dev (577.40 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_half** at 37831446.0 ns median (-0.4% vs baseline)
- Spread: 1.00x (fastest 37831446.0 ns, slowest 37980797.6 ns)

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
| Counter frequency | 2112776360 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| rcm_adj | 37985721ns | 37984579ns | 37773011ns | 37949161ns | 38146914ns | base |
| rcm_half | 37969368ns | 37835571ns | 37530623ns | 37784071ns | 38466686ns | -0.04% |
| rcm_rev | 38042826ns | 37954759ns | 37822497ns | 37905146ns | 38359511ns | +0.15% |
| rcm_scr | 38109402ns | 37934698ns | 37663773ns | 37885897ns | 38667474ns | +0.33% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 37981959ns | 37769209ns | 38143147ns | base | 0.000 |
| rcm_half | 37965459ns | 37526787ns | 38462659ns | -0.04% | 0.000 |
| rcm_rev | 38038959ns | 37818651ns | 38355579ns | +0.15% | 0.000 |
| rcm_scr | 38105547ns | 37659961ns | 38663565ns | +0.33% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.8% |
| rcm_half | 0.000 | 99.2% |
| rcm_rev | 0.000 | 98.9% |
| rcm_scr | 0.000 | 98.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 37985721ns | 37985721ns | base |
| rcm_half | 37969368ns | 37969368ns | -0.04% |
| rcm_rev | 38042826ns | 38042826ns | +0.15% |
| rcm_scr | 38109402ns | 38109402ns | +0.33% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 37980798ns | base | --- | [37900544, 38106693] | --- | --- | --- | --- |
| rcm_half | 37831446ns | no significant difference | [-339150, +110001]ns | [37640718, 38014846] | no | 1.0000 | 0.4807 | 0 |
| rcm_rev | 37951041ns | no significant difference | [-146218, +68556]ns | [37843931, 37985872] | no | 1.0000 | 1.0000 | 0 |
| rcm_scr | 37930986ns | no significant difference | [-227983, +175915]ns | [37815746, 38178439] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 38169100ns | -0.5% | -0.9% | -0.6% |
| 1 | 2 | 37987079ns | +0.3% | -0.3% | -0.4% |
| 1 | 3 | 37958061ns | -1.0% | +0.8% | -0.8% |
| 2 | 1 | 38002693ns | -1.3% | -0.0% | +0.7% |
| 2 | 2 | 37771957ns | +0.2% | +0.2% | +0.2% |
| 2 | 3 | 37873682ns | +1.1% | +0.2% | +2.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.155 | ok |
| rcm_half | 0.144 | ok |
| rcm_rev | -0.060 | ok |
| rcm_scr | -0.151 | ok |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 2/6, lost 3/6
- **rcm_scr**: won 3/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 246075942.1ns | 37981958.9ns | 647.9% | HIGH |
| rcm_half | 246030675.2ns | 37965459.3ns | 648.0% | HIGH |
| rcm_rev | 246258226.2ns | 38038959.3ns | 647.4% | HIGH |
| rcm_scr | 246019963.2ns | 38105546.8ns | 645.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 37769208.9-38143147.3 ns)
  37769208.9 |#############
  37787905.9 |
  37806602.8 |#############
  37825299.7 |
  37843996.6 |
  37862693.5 |#############
  37881390.4 |#############
  37900087.4 |#############
  37918784.3 |#############
  37937481.2 |
  37956178.1 |#############
  37974875.0 |########################################
  37993572.0 |#############
  38012268.9 |
  38030965.8 |
  38049662.7 |
  38068359.6 |
  38087056.5 |
  38105753.5 |##########################
  38124450.4 |
  (1 below, 4 above range)

rcm_half (n=18, range 37526787.3-38462659.3 ns)
  37526787.3 |##########################
  37573580.9 |#############
  37620374.5 |#############
  37667168.1 |
  37713961.7 |
  37760755.3 |#############
  37807548.9 |########################################
  37854342.5 |
  37901136.1 |
  37947929.7 |########################################
  37994723.3 |#############
  38041516.9 |
  38088310.5 |#############
  38135104.1 |
  38181897.7 |#############
  38228691.3 |
  38275484.9 |#############
  38322278.5 |
  38369072.1 |
  38415865.7 |
  (2 below, 1 above range)

rcm_rev (n=18, range 37818651.5-38355579.4 ns)
  37818651.5 |########################################
  37845497.9 |#############
  37872344.3 |##########################
  37899190.7 |
  37926037.1 |##########################
  37952883.5 |##########################
  37979729.9 |########################################
  38006576.2 |
  38033422.6 |
  38060269.0 |
  38087115.4 |
  38113961.8 |
  38140808.2 |#############
  38167654.6 |
  38194501.0 |
  38221347.4 |
  38248193.8 |#############
  38275040.2 |
  38301886.6 |
  38328733.0 |
  (2 below, 1 above range)

rcm_scr (n=18, range 37659961.4-38663564.8 ns)
  37659961.4 |#############
  37710141.6 |
  37760321.8 |##########################
  37810501.9 |########################################
  37860682.1 |
  37910862.3 |########################################
  37961042.4 |
  38011222.6 |#############
  38061402.8 |
  38111582.9 |#############
  38161763.1 |
  38211943.3 |#############
  38262123.5 |#############
  38312303.6 |
  38362483.8 |
  38412664.0 |
  38462844.1 |
  38513024.3 |
  38563204.5 |
  38613384.6 |
  (2 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=647.1% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=649.3% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=647.5% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=647.8% of algo (FFI overhead may distort results)
