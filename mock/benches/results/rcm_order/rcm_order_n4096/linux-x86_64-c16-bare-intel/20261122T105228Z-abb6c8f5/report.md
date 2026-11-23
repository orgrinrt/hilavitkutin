# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.5% (fastest 18.96 ms to slowest 19.06 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (97.94 us) is smaller than the fastest variant's own run-to-run std-dev (169.49 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_rev** at 18961402.8 ns median (-0.1% vs baseline)
- Spread: 1.01x (fastest 18961402.8 ns, slowest 19059341.3 ns)

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
| rcm_adj | 18934407ns | 18990435ns | 18711897ns | 18921703ns | 19064717ns | base |
| rcm_half | 19047103ns | 19062984ns | 18677601ns | 19006251ns | 19293134ns | +0.60% |
| rcm_rev | 18974700ns | 18965205ns | 18747185ns | 18932440ns | 19151847ns | +0.21% |
| rcm_scr | 18943617ns | 18977368ns | 18762634ns | 18923698ns | 19063988ns | +0.05% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 18930706ns | 18708173ns | 19060979ns | base | 0.000 |
| rcm_half | 19043319ns | 18673692ns | 19289341ns | +0.59% | 0.000 |
| rcm_rev | 18970893ns | 18743347ns | 19148128ns | +0.21% | 0.000 |
| rcm_scr | 18939871ns | 18758751ns | 19060223ns | +0.05% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 4096

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.4% |
| rcm_half | 0.000 | 98.0% |
| rcm_rev | 0.000 | 98.5% |
| rcm_scr | 0.000 | 98.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 18934407ns | 18934407ns | base |
| rcm_half | 19047103ns | 19047103ns | +0.60% |
| rcm_rev | 18974700ns | 18974700ns | +0.21% |
| rcm_scr | 18943617ns | 18943617ns | +0.05% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 18986858ns | base | --- | [18805017, 19035797] | --- | --- | --- | --- |
| rcm_half | 19059341ns | no significant difference | [-26936, +130980]ns | [18930475, 19118914] | no | 0.3568 | 0.2379 | 0 |
| rcm_rev | 18961403ns | no significant difference | [-90114, +61181]ns | [18869632, 19053719] | no | 0.4807 | 0.4807 | 0 |
| rcm_scr | 18973752ns | no significant difference | [-12055, +60411]ns | [18835890, 19023254] | no | 0.3568 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 18757905ns | -0.5% | +1.5% | +0.0% |
| 1 | 2 | 18696360ns | +0.9% | +0.3% | +1.0% |
| 1 | 3 | 19078407ns | -0.4% | -0.5% | -1.3% |
| 2 | 1 | 19030455ns | +0.3% | -0.8% | +0.2% |
| 2 | 2 | 18939595ns | +1.4% | +0.6% | +0.3% |
| 2 | 3 | 19035884ns | +0.1% | +0.2% | -0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | 0.177 | ok |
| rcm_half | 0.424 | moderate+ |
| rcm_rev | -0.238 | moderate- |
| rcm_scr | 0.196 | ok |

**Consistency summary:**

- **rcm_half**: won 2/6, lost 4/6
- **rcm_rev**: won 2/6, lost 4/6
- **rcm_scr**: won 1/6, lost 3/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 122874187.8ns | 18930706.4ns | 649.1% | HIGH |
| rcm_half | 123173808.0ns | 19043319.4ns | 646.8% | HIGH |
| rcm_rev | 122942804.0ns | 18970892.9ns | 648.1% | HIGH |
| rcm_scr | 123222286.2ns | 18939870.5ns | 650.6% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 18708172.7-19060978.9 ns)
  18708172.7 |
  18725813.0 |######
  18743453.3 |######
  18761093.6 |######
  18778733.9 |
  18796374.2 |
  18814014.6 |
  18831654.9 |######
  18849295.2 |
  18866935.5 |
  18884575.8 |
  18902216.1 |
  18919856.4 |######
  18937496.7 |#############
  18955137.0 |
  18972777.3 |
  18990417.6 |
  19008057.9 |
  19025698.3 |########################################
  19043338.6 |
  (2 below, 3 above range)

rcm_half (n=18, range 18673691.8-19289340.9 ns)
  18673691.8 |
  18704474.2 |#############
  18735256.7 |
  18766039.1 |
  18796821.6 |#############
  18827604.1 |
  18858386.5 |#############
  18889169.0 |
  18919951.4 |
  18950733.9 |
  18981516.4 |##########################
  19012298.8 |
  19043081.3 |########################################
  19073863.7 |########################################
  19104646.2 |
  19135428.6 |##########################
  19166211.1 |
  19196993.6 |##########################
  19227776.0 |
  19258558.5 |
  (2 below, 1 above range)

rcm_rev (n=18, range 18743346.5-19148128.5 ns)
  18743346.5 |########################################
  18763585.6 |
  18783824.7 |####################
  18804063.8 |
  18824302.9 |
  18844542.0 |
  18864781.1 |########################################
  18885020.2 |
  18905259.3 |####################
  18925498.4 |########################################
  18945737.5 |
  18965976.6 |####################
  18986215.7 |
  19006454.8 |####################
  19026693.9 |####################
  19046933.0 |########################################
  19067172.1 |####################
  19087411.2 |####################
  19107650.3 |
  19127889.4 |
  (1 below, 2 above range)

rcm_scr (n=18, range 18758751.3-19060223.3 ns)
  18758751.3 |####################
  18773824.9 |####################
  18788898.5 |
  18803972.1 |
  18819045.7 |####################
  18834119.3 |########################################
  18849192.9 |
  18864266.5 |
  18879340.1 |####################
  18894413.7 |
  18909487.3 |
  18924560.9 |
  18939634.5 |####################
  18954708.1 |
  18969781.7 |########################################
  18984855.3 |####################
  18999928.9 |####################
  19015002.5 |########################################
  19030076.1 |####################
  19045149.7 |
  (1 below, 3 above range)

```

## Diagnostics

- **rcm_adj**: bridge=646.9% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=644.1% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=646.8% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=647.8% of algo (FFI overhead may distort results)
