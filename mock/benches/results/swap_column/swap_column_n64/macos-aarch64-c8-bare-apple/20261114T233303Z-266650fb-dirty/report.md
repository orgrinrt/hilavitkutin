# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (cs_exch, cs_plain) are a dead heat (<1%)

cs_exch (21.75 us) and cs_plain (21.80 us) differ by 0.26%, inside the noise, even though the wider field spreads 26.3%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

## Key findings

- **Fastest: cs_exch** at 21747.9 ns median (-0.3% vs baseline)
- 2 variants significantly slower than baseline
- Spread: 1.26x (fastest 21747.9 ns, slowest 27460.5 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | a buffered column's swap: plain floor vs Snapshot copy vs Exchange |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000004 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| cs_exch | 24324ns | 24298ns | 23199ns | 24158ns | 25137ns | -0.37% |
| cs_plain | 24414ns | 24235ns | 23599ns | 24157ns | 25208ns | base |
| cs_snap | 29632ns | 29677ns | 28747ns | 29460ns | 30333ns | +21.37% |
| cs_snap_twin | 30200ns | 30131ns | 28721ns | 29724ns | 31653ns | +23.70% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 21882ns | 21049ns | 22628ns | -0.47% | 0.003 |
| cs_plain | 21986ns | 21225ns | 22701ns | base | 0.003 |
| cs_snap | 27203ns | 26457ns | 27911ns | +23.73% | 0.002 |
| cs_snap_twin | 27553ns | 26521ns | 28601ns | +25.32% | 0.002 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.003 | 96.8% |
| cs_plain | 0.003 | 96.5% |
| cs_snap | 0.002 | 77.8% |
| cs_snap_twin | 0.002 | 76.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 24324ns | 24324ns | -0.37% |
| cs_plain | 24414ns | 24414ns | base |
| cs_snap | 29632ns | 29632ns | +21.37% |
| cs_snap_twin | 30200ns | 30200ns | +23.70% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 21804ns | base | --- | [21675, 22142] | --- | --- | --- | --- |
| cs_exch | 21748ns | no significant difference | [-723, +306]ns | [21485, 22008] | no | 0.8145 | 0.8145 | 0 |
| cs_snap | 27069ns | +5260.4ns (+24.1%) | [+4875, +5535]ns | [26773, 27494] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 27460ns | +5366.6ns (+24.6%) | [+4960, +6085]ns | [26694, 28340] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 21733ns | -0.8% | +23.5% | +22.9% |
| 1 | 2 | 21817ns | -0.3% | +22.6% | +21.6% |
| 1 | 3 | 21192ns | +1.6% | +26.4% | +32.8% |
| 2 | 1 | 21792ns | +3.2% | +25.6% | +22.9% |
| 2 | 2 | 22112ns | -0.4% | +23.8% | +26.3% |
| 2 | 3 | 22171ns | -4.8% | +19.7% | +29.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.116 | ok |
| cs_plain | 0.177 | ok |
| cs_snap | -0.011 | ok |
| cs_snap_twin | -0.039 | ok |

**Consistency summary:**

- **cs_exch**: won 4/6, lost 2/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 352677.1ns | 21881.9ns | 1611.7% | HIGH |
| cs_plain | 353111.6ns | 21986.1ns | 1606.1% | HIGH |
| cs_snap | 353233.3ns | 27203.0ns | 1298.5% | HIGH |
| cs_snap_twin | 355674.5ns | 27553.3ns | 1290.9% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 21048.6-22628.5 ns)
  21048.6 |########################################
  21127.6 |
  21206.6 |
  21285.6 |
  21364.6 |########################################
  21443.6 |
  21522.6 |########################################
  21601.5 |####################
  21680.5 |########################################
  21759.5 |
  21838.5 |####################
  21917.5 |########################################
  21996.5 |####################
  22075.5 |
  22154.5 |####################
  22233.5 |
  22312.5 |
  22391.5 |
  22470.5 |####################
  22549.5 |
  (1 below, 2 above range)

cs_plain (n=18, range 21225.0-22700.7 ns)
  21225.0 |
  21298.8 |####################
  21372.6 |
  21446.4 |####################
  21520.1 |
  21593.9 |####################
  21667.7 |########################################
  21741.5 |########################################
  21815.3 |########################################
  21889.1 |
  21962.8 |####################
  22036.6 |
  22110.4 |########################################
  22184.2 |
  22258.0 |
  22331.8 |####################
  22405.5 |
  22479.3 |
  22553.1 |
  22626.9 |
  (2 below, 3 above range)

cs_snap (n=18, range 26457.0-27911.1 ns)
  26457.0 |#############
  26529.7 |#############
  26602.4 |
  26675.1 |
  26747.8 |########################################
  26820.5 |########################################
  26893.2 |
  26965.9 |
  27038.6 |
  27111.3 |
  27184.0 |#############
  27256.7 |
  27329.4 |########################################
  27402.2 |
  27474.9 |
  27547.6 |#############
  27620.3 |
  27693.0 |
  27765.7 |
  27838.4 |
  (1 below, 4 above range)

cs_snap_twin (n=18, range 26520.8-28600.7 ns)
  26520.8 |##########################
  26624.8 |########################################
  26728.8 |#############
  26832.8 |
  26936.8 |#############
  27040.8 |
  27144.8 |#############
  27248.8 |
  27352.8 |
  27456.8 |
  27560.8 |
  27664.8 |#############
  27768.8 |
  27872.8 |#############
  27976.8 |
  28080.7 |#############
  28184.7 |
  28288.7 |##########################
  28392.7 |#############
  28496.7 |
  (1 below, 3 above range)

```

## Diagnostics

- **cs_exch**: bridge=1621.6% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1625.1% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1297.3% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1283.7% of algo (FFI overhead may distort results)
