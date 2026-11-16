# a buffered column's swap: plain floor vs Snapshot copy vs Exchange

4 variants, 18 samples per variant.
Baseline: **cs_plain**

## Highlights

Baseline for all deltas below: **cs_plain**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### cs_exch dominates: 26% faster than the next best (cs_plain)

cs_exch (3.13 ms) leads cs_plain (3.94 ms) by 26%, a clear separation rather than a photo finish. CV 12.4%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### cs_exch beats baseline by 23% (significant)

cs_exch is -897.42 us (23%) faster than baseline cs_plain, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### cs_exch is fastest but the noisiest (CV 12.4%)

cs_exch wins on median (3.13 ms) yet has the highest variance (CV 12.4%), while cs_snap is the steadiest (CV 6.1%, 4.50 ms).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### Two tiers: {cs_exch} vs {cs_plain, cs_snap_twin, cs_snap} (26% apart)

The field splits into a fast tier {cs_exch} and a slow tier {cs_plain, cs_snap_twin, cs_snap} with a 26% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

## Key findings

- **Fastest: cs_exch** at 3132825.0 ns median (-20.4% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 1.44x (fastest 3132825.0 ns, slowest 4497431.2 ns)

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
| cs_exch | 3028506ns | 3140158ns | 2287112ns | 3031135ns | 3395259ns | -22.29% |
| cs_plain | 3897139ns | 3944975ns | 3327092ns | 3869179ns | 4224101ns | base |
| cs_snap | 4483404ns | 4503740ns | 4077804ns | 4415491ns | 4788073ns | +15.04% |
| cs_snap_twin | 4475516ns | 4371621ns | 3907851ns | 4351353ns | 4945594ns | +14.84% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| cs_exch | 3022069ns | 2281689ns | 3388517ns | -22.32% | 0.003 |
| cs_plain | 3890272ns | 3320012ns | 4217011ns | base | 0.002 |
| cs_snap | 4476741ns | 4071424ns | 4781583ns | +15.08% | 0.002 |
| cs_snap_twin | 4469085ns | 3901751ns | 4938819ns | +14.88% | 0.002 |

## Performance model

- Peak throughput: **0.004 Gops/s** (cs_exch; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| cs_exch | 0.003 | 72.8% |
| cs_plain | 0.002 | 57.9% |
| cs_snap | 0.002 | 50.7% |
| cs_snap_twin | 0.002 | 52.3% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| cs_exch | 3028506ns | 3028506ns | -22.29% |
| cs_plain | 3897139ns | 3897139ns | base |
| cs_snap | 4483404ns | 4483404ns | +15.04% |
| cs_snap_twin | 4475516ns | 4475516ns | +14.84% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| cs_plain | 3937977ns | base | --- | [3716200, 4161921] | --- | --- | --- | --- |
| cs_exch | 3132825ns | -897424.9ns (-22.8%) | [-997244, -725467]ns | [2899927, 3255265] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap | 4497431ns | +406991.7ns (+10.3%) | [+277517, +834373]ns | [4304992, 4622306] | YES | 0.0000 | 0.0000 | 0 |
| cs_snap_twin | 4365931ns | +477881.3ns (+12.1%) | [+247717, +716690]ns | [4298196, 4505935] | YES | 0.0001 | 0.0001 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | cs_plain | cs_exch | cs_snap | cs_snap_twin |
|---|---|---|---|---|---|
| 1 | 1 | 3250321ns | -29.1% | +39.6% | +16.9% |
| 1 | 2 | 3914275ns | -19.3% | +24.8% | +10.9% |
| 1 | 3 | 4088971ns | -22.8% | +8.0% | +14.8% |
| 2 | 1 | 4158938ns | -25.9% | +3.2% | +8.7% |
| 2 | 2 | 3743462ns | -10.4% | +11.3% | +17.7% |
| 2 | 3 | 4232842ns | -25.5% | +6.9% | +2.2% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| cs_exch | -0.025 | ok |
| cs_plain | -0.075 | ok |
| cs_snap | 0.173 | ok |
| cs_snap_twin | 0.155 | ok |

**Consistency summary:**

- **cs_exch**: won 6/6, lost 0/6
- **cs_snap**: won 0/6, lost 6/6
- **cs_snap_twin**: won 0/6, lost 6/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| cs_exch | 46033841.4ns | 3022069.0ns | 1523.3% | HIGH |
| cs_plain | 46117819.7ns | 3890272.2ns | 1185.5% | HIGH |
| cs_snap | 46042297.2ns | 4476741.0ns | 1028.5% | HIGH |
| cs_snap_twin | 46798813.9ns | 4469084.7ns | 1047.2% | HIGH |

## Distribution (algo ns)

```
cs_exch (n=18, range 2281688.9-3388516.7 ns)
  2281688.9 |##########################
  2337030.3 |
  2392371.7 |
  2447713.1 |
  2503054.5 |
  2558395.8 |
  2613737.2 |
  2669078.6 |
  2724420.0 |
  2779761.4 |#############
  2835102.8 |
  2890444.2 |########################################
  2945785.6 |
  3001127.0 |
  3056468.3 |##########################
  3111809.7 |########################################
  3167151.1 |#############
  3222492.5 |
  3277833.9 |#############
  3333175.3 |##########################
  (1 below, 2 above range)

cs_plain (n=18, range 3320012.5-4217011.1 ns)
  3320012.5 |
  3364862.4 |
  3409712.3 |
  3454562.3 |#############
  3499412.2 |#############
  3544262.1 |
  3589112.1 |
  3633962.0 |
  3678811.9 |#############
  3723661.9 |##########################
  3768511.8 |
  3813361.7 |
  3858211.7 |
  3903061.6 |##########################
  3947911.5 |#############
  3992761.5 |
  4037611.4 |
  4082461.3 |#############
  4127311.3 |########################################
  4172161.2 |#############
  (2 below, 3 above range)

cs_snap (n=18, range 4071423.6-4781583.3 ns)
  4071423.6 |
  4106931.6 |
  4142439.6 |########################################
  4177947.6 |####################
  4213455.6 |
  4248963.6 |
  4284471.5 |########################################
  4319979.5 |
  4355487.5 |####################
  4390995.5 |####################
  4426503.5 |
  4462011.5 |####################
  4497519.5 |########################################
  4533027.4 |########################################
  4568535.4 |
  4604043.4 |
  4639551.4 |
  4675059.4 |####################
  4710567.4 |
  4746075.4 |####################
  (1 below, 3 above range)

cs_snap_twin (n=18, range 3901751.4-4938818.7 ns)
  3901751.4 |
  3953604.8 |
  4005458.1 |
  4057311.5 |
  4109164.9 |##########
  4161018.2 |
  4212871.6 |
  4264725.0 |####################
  4316578.3 |########################################
  4368431.7 |##############################
  4420285.1 |
  4472138.4 |####################
  4523991.8 |
  4575845.2 |
  4627698.5 |
  4679551.9 |##########
  4731405.3 |
  4783258.6 |
  4835112.0 |
  4886965.4 |
  (2 below, 3 above range)

```

## Diagnostics

- **cs_exch**: bridge=1465.7% of algo (FFI overhead may distort results)
- **cs_plain**: bridge=1166.5% of algo (FFI overhead may distort results)
- **cs_snap**: bridge=1019.6% of algo (FFI overhead may distort results)
- **cs_snap_twin**: bridge=1050.8% of algo (FFI overhead may distort results)
