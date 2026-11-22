# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Top two (rcm_half, rcm_adj) are a dead heat (<1%)

rcm_half (34.72 ms) and rcm_adj (34.82 ms) differ by 0.27%, inside the noise, even though the wider field spreads 5.1%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### rcm_scr shows alternating (throttle bounce) (autocorr -0.74)

rcm_scr's per-pass series has lag-1 autocorrelation -0.74, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### rcm_rev is inconsistent: worst-20% is 1.6x its best-20%

rcm_rev's best 20% of batches run at 34.56 ms but its worst 20% at 55.83 ms (1.6x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: rcm_half** at 34722181.2 ns median (-0.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.05x (fastest 34722181.2 ns, slowest 36492925.0 ns)

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
| rcm_adj | 37872613ns | 34825300ns | 34441736ns | 34803650ns | 44191497ns | base |
| rcm_half | 34972732ns | 34730044ns | 34363340ns | 34664546ns | 35739708ns | -7.66% |
| rcm_rev | 42291992ns | 36501952ns | 34569218ns | 35832449ns | 55842693ns | +11.67% |
| rcm_scr | 37822886ns | 35087608ns | 34394271ns | 34909123ns | 43907840ns | -0.13% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 37863992ns | 34433976ns | 44182390ns | base | 0.000 |
| rcm_half | 34964319ns | 34354596ns | 35731467ns | -7.66% | 0.000 |
| rcm_rev | 42282784ns | 34561111ns | 55832291ns | +11.67% | 0.000 |
| rcm_scr | 37814941ns | 34387115ns | 43898658ns | -0.13% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_half; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 98.7% |
| rcm_half | 0.000 | 98.9% |
| rcm_rev | 0.000 | 94.1% |
| rcm_scr | 0.000 | 97.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 37872613ns | 37872613ns | base |
| rcm_half | 34972732ns | 34972732ns | -7.66% |
| rcm_rev | 42291992ns | 42291992ns | +11.67% |
| rcm_scr | 37822886ns | 37822886ns | -0.13% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 34816885ns | base | --- | [34572017, 35654050] | --- | --- | --- | --- |
| rcm_half | 34722181ns | no significant difference | [-539948, +474890]ns | [34509900, 35064533] | no | 1.0000 | 1.0000 | 0 |
| rcm_rev | 36492925ns | no significant difference | [-53065, +2879210]ns | [34890900, 39891608] | no | 0.2888 | 0.0963 | 0 |
| rcm_scr | 35079848ns | no significant difference | [-468981, +303217]ns | [34718548, 35344456] | no | 1.0000 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 34845725ns | -0.7% | +4.9% | +0.8% |
| 1 | 2 | 45597379ns | -24.1% | +79.9% | -23.9% |
| 1 | 3 | 34704158ns | +1.1% | +0.9% | +1.6% |
| 2 | 1 | 34779350ns | -1.2% | +7.1% | +0.1% |
| 2 | 2 | 34464558ns | +1.5% | +1.1% | +2.8% |
| 2 | 3 | 34542042ns | +1.5% | +8.8% | +0.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.208 | moderate- |
| rcm_half | -0.399 | moderate- |
| rcm_rev | -0.252 | moderate- |
| rcm_scr | -0.739 | HIGH- (thermal bounce) |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 0/6, lost 6/6
- **rcm_scr**: won 1/6, lost 5/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 726821227.1ns | 37863992.1ns | 1919.6% | HIGH |
| rcm_half | 717985755.5ns | 34964319.2ns | 2053.5% | HIGH |
| rcm_rev | 730901374.5ns | 42282784.3ns | 1728.6% | HIGH |
| rcm_scr | 720129300.2ns | 37814941.4ns | 1904.4% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 34433976.4-44182389.6 ns)
  34433976.4 |########################################
  34921397.0 |########
  35408817.7 |########
  35896238.4 |
  36383659.0 |
  36871079.7 |
  37358500.3 |
  37845921.0 |
  38333341.7 |
  38820762.3 |
  39308183.0 |
  39795603.6 |
  40283024.3 |
  40770445.0 |
  41257865.6 |
  41745286.3 |
  42232707.0 |####
  42720127.6 |
  43207548.3 |
  43694968.9 |
  (1 below, 3 above range)

rcm_half (n=18, range 34354595.9-35731467.4 ns)
  34354595.9 |##########################
  34423439.4 |##########################
  34492283.0 |#############
  34561126.6 |########################################
  34629970.2 |
  34698813.7 |
  34767657.3 |#############
  34836500.9 |#############
  34905344.5 |
  34974188.0 |#############
  35043031.6 |##########################
  35111875.2 |
  35180718.8 |
  35249562.3 |#############
  35318405.9 |
  35387249.5 |
  35456093.1 |
  35524936.6 |
  35593780.2 |
  35662623.8 |
  (1 below, 3 above range)

rcm_rev (n=18, range 34561111.1-55832291.0 ns)
  34561111.1 |########################################
  35624670.1 |###########
  36688229.1 |###########
  37751788.1 |
  38815347.1 |#####
  39878906.1 |#####
  40942465.1 |
  42006024.1 |
  43069583.1 |###########
  44133142.1 |
  45196701.0 |
  46260260.0 |
  47323819.0 |
  48387378.0 |
  49450937.0 |
  50514496.0 |
  51578055.0 |
  52641614.0 |
  53705173.0 |
  54768732.0 |
  (1 below, 2 above range)

rcm_scr (n=18, range 34387115.3-43898658.3 ns)
  34387115.3 |########################################
  34862692.4 |#################################
  35338269.6 |##########################
  35813846.7 |
  36289423.9 |
  36765001.0 |
  37240578.2 |
  37716155.3 |
  38191732.5 |
  38667309.7 |
  39142886.8 |
  39618464.0 |
  40094041.1 |
  40569618.3 |
  41045195.4 |
  41520772.6 |
  41996349.7 |
  42471926.9 |
  42947504.0 |
  43423081.2 |
  (2 below, 1 above range)

```

## Diagnostics

- **rcm_adj**: bridge=2062.9% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=2067.0% of algo (FFI overhead may distort results)
- **rcm_rev**: CV=35.6% (high variance, measurements may be unstable)
- **rcm_rev**: bridge=1968.2% of algo (FFI overhead may distort results)
- **rcm_scr**: CV=30.9% (high variance, measurements may be unstable)
- **rcm_scr**: bridge=2046.3% of algo (FFI overhead may distort results)
