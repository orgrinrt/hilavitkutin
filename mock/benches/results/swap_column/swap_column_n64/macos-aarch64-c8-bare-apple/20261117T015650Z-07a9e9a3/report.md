# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Two tiers: {cs_exch, cs_plain} vs {cs_snap_twin, cs_snap} (25% apart)

The field splits into a fast tier {cs_exch, cs_plain} and a slow tier {cs_snap_twin, cs_snap} with a 25% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Speed leader cs_exch vs stability leader cs_plain (+2% speed for 1.1x steadier)

cs_exch is fastest (21.09 us, CV 3.2%); cs_plain gives up 1.6% median for 1.1x lower variance (CV 2.8%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: cs_exch** at 21087.5 ns median (-1.5% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.28x (fastest 21087.5 ns, slowest 27075.0 ns)

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
| cs_exch | 23972ns | 23565ns | 23232ns | 23549ns | 24976ns | -1.15% |
| cs_plain | 24250ns | 24046ns | 23524ns | 24010ns | 24973ns | base |
| cs_snap | 29988ns | 29681ns | 28903ns | 29615ns | 31092ns | +23.66% |
| cs_snap_twin | 31194ns | 29565ns | 28886ns | 29412ns | 35019ns | +28.63% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 21393ns | 20881ns | 22194ns | -1.44% | 0.003 |
| cs_plain | 21705ns | 21158ns | 22444ns | base | 0.003 |
| cs_snap | 27413ns | 26561ns | 28449ns | +26.30% | 0.002 |
| cs_snap_twin | 28288ns | 26333ns | 31689ns | +30.33% | 0.002 |

## Performance model

- Peak throughput: **0.003 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 64

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.003 | 99.0% |
| cs_plain | 0.003 | 97.5% |
| cs_snap | 0.002 | 77.1% |
| cs_snap_twin | 0.002 | 77.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 23972ns | 23972ns | -1.15% |
| cs_plain | 24250ns | 24250ns | base |
| cs_snap | 29988ns | 29988ns | +23.66% |
| cs_snap_twin | 31194ns | 31194ns | +28.63% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 21415ns | base | --- | [21262, 21971] | --- | --- | --- | --- |
| cs_exch | 21088ns | -275.0ns (-1.3%) | [-792, -40]ns | [20935, 21627] | YES (adj: no) | 0.0963 | 0.0963 | 0 |
| cs_snap | 27075ns | +5658.4ns (+26.4%) | [+4975, +5940]ns | [26796, 27425] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 26798ns | +5039.5ns (+23.5%) | [+4798, +6196]ns | [26404, 27785] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 21421ns | +3.6% | +35.7% | +22.9% |
| 1 | 2 | 21217ns | -1.5% | +26.2% | +47.7% |
| 1 | 3 | 21558ns | -3.1% | +26.9% | +22.5% |
| 2 | 1 | 22471ns | -6.4% | +20.0% | +19.5% |
| 2 | 2 | 21392ns | +3.4% | +32.4% | +25.0% |
| 2 | 3 | 21183ns | -0.2% | +26.9% | +31.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.229 | moderate- |
| cs_plain | -0.031 | ok |
| cs_snap | -0.437 | moderate- |
| cs_snap_twin | -0.429 | moderate- |

**Consistency summary:**

- **cs_exch**: won 4/6, lost 2/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 353364.1ns | 21393.3ns | 1651.8% | HIGH |
| cs_plain | 355401.6ns | 21704.9ns | 1637.4% | HIGH |
| cs_snap | 358596.5ns | 27412.7ns | 1308.1% | HIGH |
| cs_snap_twin | 361805.1ns | 28288.4ns | 1279.0% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 20880.5-22194.5 ns)
  20880.5 |########################################
  20946.2 |##########################
  21011.9 |##########################
  21077.6 |##########################
  21143.3 |#############
  21209.0 |#############
  21274.7 |
  21340.4 |
  21406.1 |
  21471.8 |
  21537.5 |
  21603.2 |
  21668.9 |
  21734.6 |
  21800.3 |
  21866.0 |
  21931.7 |#############
  21997.4 |
  22063.1 |#############
  22128.8 |#############
  (2 below, 2 above range)

cs_plain (n=18, range 21158.3-22443.8 ns)
  21158.3 |########################################
  21222.6 |
  21286.9 |#############
  21351.1 |########################################
  21415.4 |#############
  21479.7 |
  21544.0 |#############
  21608.2 |#############
  21672.5 |
  21736.8 |#############
  21801.0 |
  21865.3 |
  21929.6 |
  21993.9 |
  22058.1 |
  22122.4 |
  22186.7 |#############
  22250.9 |
  22315.2 |
  22379.5 |
  (2 below, 4 above range)

cs_snap (n=18, range 26561.1-28448.6 ns)
  26561.1 |
  26655.5 |########################################
  26749.9 |##########################
  26844.3 |#############
  26938.6 |##########################
  27033.0 |
  27127.4 |#############
  27221.7 |
  27316.1 |##########################
  27410.5 |##########################
  27504.9 |
  27599.2 |
  27693.6 |
  27788.0 |
  27882.4 |
  27976.7 |
  28071.1 |
  28165.5 |
  28259.9 |#############
  28354.2 |
  (1 below, 3 above range)

cs_snap_twin (n=18, range 26333.3-31688.9 ns)
  26333.3 |########################################
  26601.1 |#############
  26868.9 |######
  27136.7 |######
  27404.4 |
  27672.2 |#############
  27940.0 |
  28207.8 |######
  28475.6 |
  28743.3 |######
  29011.1 |
  29278.9 |
  29546.7 |
  29814.4 |
  30082.2 |
  30350.0 |
  30617.8 |
  30885.6 |
  31153.3 |######
  31421.1 |
  (2 below, 1 above range)

```

## Diagnostics

- **cs_exch**: bridge=1654.5% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1647.1% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1315.8% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1310.3% of algo (FFI overhead may distort results)
