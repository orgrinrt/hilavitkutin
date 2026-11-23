# A1-1 dispatch order: column-adjacency dose response across valid topo orders

4 variants, 18 samples per variant.
Baseline: **rcm_adj**

## Highlights

Baseline for all deltas below: **rcm_adj**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### All variants are a statistical tie (no significant difference)

No variant's paired difference vs the baseline is significant, and the whole field sits within 0.4% (fastest 75.35 ms to slowest 75.62 ms).

_Why it matters:_ When nothing separates, pick on a secondary axis (stability, code simplicity, memory) - performance does not decide it here.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (267.69 us) is smaller than the fastest variant's own run-to-run std-dev (479.19 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

## Key findings

- **Fastest: rcm_scr** at 75352756.6 ns median (-0.1% vs baseline)
- Spread: 1.00x (fastest 75352756.6 ns, slowest 75620443.6 ns)

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
| rcm_adj | 76061389ns | 75425042ns | 75238392ns | 75416962ns | 77439527ns | base |
| rcm_half | 75815695ns | 75624834ns | 75160590ns | 75536446ns | 76562120ns | -0.32% |
| rcm_rev | 75835281ns | 75502988ns | 75138219ns | 75458398ns | 76749138ns | -0.30% |
| rcm_scr | 75407311ns | 75357170ns | 74784549ns | 75261671ns | 75937152ns | -0.86% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| rcm_adj | 76056770ns | 75233813ns | 77434772ns | base | 0.000 |
| rcm_half | 75811169ns | 75156024ns | 76557603ns | -0.32% | 0.000 |
| rcm_rev | 75830816ns | 75133751ns | 76744633ns | -0.30% | 0.000 |
| rcm_scr | 75402714ns | 74779798ns | 75932579ns | -0.86% | 0.000 |

## Performance model

- Peak throughput: **0.000 Gops/s** (rcm_scr; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| rcm_adj | 0.000 | 99.2% |
| rcm_half | 0.000 | 98.9% |
| rcm_rev | 0.000 | 99.0% |
| rcm_scr | 0.000 | 99.2% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| rcm_adj | 76061389ns | 76061389ns | base |
| rcm_half | 75815695ns | 75815695ns | -0.32% |
| rcm_rev | 75835281ns | 75835281ns | -0.30% |
| rcm_scr | 75407311ns | 75407311ns | -0.86% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| rcm_adj | 75420413ns | base | --- | [75360706, 76269782] | --- | --- | --- | --- |
| rcm_half | 75620444ns | no significant difference | [-654406, +324922]ns | [75332505, 75970033] | no | 0.8145 | 0.8145 | 0 |
| rcm_rev | 75498572ns | no significant difference | [-236958, +155942]ns | [75400839, 75676081] | no | 0.7137 | 0.2379 | 0 |
| rcm_scr | 75352757ns | no significant difference | [-1167672, +127103]ns | [75108345, 75590296] | no | 0.8145 | 0.8145 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | rcm_adj | rcm_half | rcm_rev | rcm_scr |
|---|---|---|---|---|---|
| 1 | 1 | 75478919ns | +0.3% | -0.1% | -0.6% |
| 1 | 2 | 75412510ns | +0.6% | +0.4% | +0.3% |
| 1 | 3 | 75334942ns | +0.2% | +0.2% | +0.6% |
| 2 | 1 | 77171804ns | -1.5% | -2.1% | -2.5% |
| 2 | 2 | 75331460ns | -0.2% | +0.6% | -0.8% |
| 2 | 3 | 75386470ns | -0.3% | -0.3% | +0.0% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| rcm_adj | -0.296 | moderate- |
| rcm_half | -0.089 | ok |
| rcm_rev | -0.443 | moderate- |
| rcm_scr | 0.026 | ok |

**Consistency summary:**

- **rcm_half**: won 3/6, lost 3/6
- **rcm_rev**: won 2/6, lost 3/6
- **rcm_scr**: won 3/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| rcm_adj | 493501344.3ns | 76056769.8ns | 648.9% | HIGH |
| rcm_half | 492004430.8ns | 75811168.8ns | 649.0% | HIGH |
| rcm_rev | 492516504.0ns | 75830816.1ns | 649.5% | HIGH |
| rcm_scr | 491428533.5ns | 75402714.2ns | 651.7% | HIGH |

## Distribution (algo ns)

```
rcm_adj (n=18, range 75233812.7-77434772.1 ns)
  75233812.7 |################################
  75343860.7 |########################################
  75453908.6 |################
  75563956.6 |
  75674004.6 |########
  75784052.6 |
  75894100.5 |
  76004148.5 |
  76114196.5 |
  76224244.4 |
  76334292.4 |
  76444340.4 |
  76554388.3 |
  76664436.3 |
  76774484.3 |########
  76884532.3 |
  76994580.2 |########
  77104628.2 |########
  77214676.2 |
  77324724.1 |
  (1 below, 2 above range)

rcm_half (n=18, range 75156023.8-76557602.7 ns)
  75156023.8 |########################################
  75226102.8 |
  75296181.7 |
  75366260.7 |
  75436339.6 |##############################
  75506418.5 |##########
  75576497.5 |
  75646576.4 |
  75716655.4 |##########
  75786734.3 |##########
  75856813.2 |####################
  75926892.2 |
  75996971.1 |##############################
  76067050.1 |
  76137129.0 |
  76207208.0 |
  76277286.9 |
  76347365.8 |
  76417444.8 |
  76487523.7 |
  (1 below, 2 above range)

rcm_rev (n=18, range 75133750.7-76744632.9 ns)
  75133750.7 |####################
  75214294.8 |##########
  75294838.9 |
  75375383.0 |########################################
  75455927.1 |####################
  75536471.2 |####################
  75617015.3 |##########
  75697559.4 |####################
  75778103.5 |
  75858647.7 |
  75939191.8 |
  76019735.9 |
  76100280.0 |
  76180824.1 |
  76261368.2 |
  76341912.3 |
  76422456.4 |
  76503000.5 |
  76583544.6 |
  76664088.8 |
  (1 below, 3 above range)

rcm_scr (n=18, range 74779797.7-75932579.1 ns)
  74779797.7 |####################
  74837436.8 |
  74895075.9 |
  74952714.9 |
  75010354.0 |########################################
  75067993.1 |
  75125632.2 |
  75183271.2 |########################################
  75240910.3 |####################
  75298549.4 |####################
  75356188.4 |########################################
  75413827.5 |####################
  75471466.6 |
  75529105.6 |####################
  75586744.7 |
  75644383.8 |########################################
  75702022.9 |
  75759661.9 |####################
  75817301.0 |
  75874940.1 |
  (2 below, 2 above range)

```

## Diagnostics

- **rcm_adj**: bridge=653.1% of algo (FFI overhead may distort results)
- **rcm_half**: bridge=649.9% of algo (FFI overhead may distort results)
- **rcm_rev**: bridge=650.8% of algo (FFI overhead may distort results)
- **rcm_scr**: bridge=652.1% of algo (FFI overhead may distort results)
