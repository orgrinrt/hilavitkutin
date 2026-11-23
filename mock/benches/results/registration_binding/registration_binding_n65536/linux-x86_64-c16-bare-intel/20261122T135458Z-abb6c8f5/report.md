# Registration: typed binding vs a run-time binding table over a column workload

6 variants, 160 samples per variant.
Baseline: **reg_typed**

## Highlights

Baseline for all deltas below: **reg_typed**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### reg_bound_record is an outlier: 7.3x slower than the field

reg_bound_record (88.75 us) is 7.3x the fastest (12.13 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### Top two (reg_bound_units, reg_typed_twin) are a dead heat (<1%)

reg_bound_units (12.13 us) and reg_typed_twin (12.20 us) differ by 0.55%, inside the noise, even though the wider field spreads 631.5%.

_Why it matters:_ Do not over-fit to the nominal leader when the runner-up is within measurement noise; either is a fine pick.

### Two tiers: {reg_bound_units, reg_typed_twin, reg_typed, reg_bound_columns, reg_bound_strided} vs {reg_bound_record} (279% apart)

The field splits into a fast tier {reg_bound_units, reg_typed_twin, reg_typed, reg_bound_columns, reg_bound_strided} and a slow tier {reg_bound_record} with a 279% jump between them - a qualitative difference, not a gradient.

_Why it matters:_ A tier split usually reflects a mechanism boundary (branchless vs branch, cached vs not); the tier, not the exact rank, is the finding.

### Wide spread: slowest is 7.3x the fastest

Fastest reg_bound_units (12.13 us) to slowest reg_bound_record (88.75 us): 7.3x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### Speed leader reg_bound_units vs stability leader reg_typed_twin (+1% speed for 3.1x steadier)

reg_bound_units is fastest (12.13 us, CV 5.3%); reg_typed_twin gives up 0.5% median for 3.1x lower variance (CV 1.7%).

_Why it matters:_ The pick depends on priority: peak throughput vs predictable latency. Both are defensible; name which the workload needs.

## Key findings

- **Fastest: reg_bound_units** at 12132.4 ns median (-1.0% vs baseline)
- 1 variant significantly faster than baseline
- 2 variants significantly slower than baseline
- Spread: 7.32x (fastest 12132.4 ns, slowest 88749.9 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Registration: typed binding vs a run-time binding table over a column workload |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x71c40e2b95a3d86f |
| Counter frequency | 2112791800 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| reg_bound_columns | 13960ns | 14250ns | 12339ns | 14210ns | 14832ns | -1.76% |
| reg_bound_record | 89004ns | 90717ns | 80587ns | 90829ns | 91945ns | +526.33% |
| reg_bound_strided | 24418ns | 25399ns | 21866ns | 24868ns | 25624ns | +71.84% |
| reg_bound_units | 13935ns | 14067ns | 12740ns | 14146ns | 14499ns | -1.94% |
| reg_typed | 14210ns | 14200ns | 13878ns | 14155ns | 14709ns | base |
| reg_typed_twin | 14112ns | 14134ns | 13861ns | 14107ns | 14379ns | -0.69% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| reg_bound_columns | 12052ns | 10622ns | 12833ns | -1.66% | 5.438 |
| reg_bound_record | 87076ns | 78841ns | 89961ns | +610.50% | 0.753 |
| reg_bound_strided | 22503ns | 20130ns | 23656ns | +83.61% | 2.912 |
| reg_bound_units | 12038ns | 11023ns | 12548ns | -1.77% | 5.444 |
| reg_typed | 12256ns | 11940ns | 12712ns | base | 5.347 |
| reg_typed_twin | 12172ns | 11925ns | 12435ns | -0.68% | 5.384 |

## Performance model

- Peak throughput: **6.170 Gops/s** (reg_bound_columns; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| reg_bound_columns | 5.326 | 86.3% |
| reg_bound_record | 0.738 | 12.0% |
| reg_bound_strided | 2.797 | 45.3% |
| reg_bound_units | 5.402 | 87.6% |
| reg_typed | 5.345 | 86.6% |
| reg_typed_twin | 5.372 | 87.1% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| reg_bound_columns | 13960ns | 13960ns | -1.76% |
| reg_bound_record | 89004ns | 89004ns | +526.33% |
| reg_bound_strided | 24418ns | 24418ns | +71.84% |
| reg_bound_units | 13935ns | 13935ns | -1.94% |
| reg_typed | 14210ns | 14210ns | base |
| reg_typed_twin | 14112ns | 14112ns | -0.69% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| reg_typed | 12260ns | base | --- | [12245, 12268] | --- | --- | --- | --- |
| reg_bound_columns | 12304ns | no significant difference | [-41, +46]ns | [12298, 12345] | no | 0.2356 | 0.2356 | 0 |
| reg_bound_record | 88750ns | +76678.1ns (+625.4%) | [+76654, +76714]ns | [88723, 88912] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_strided | 23431ns | +11052.0ns (+90.1%) | [+10112, +11084]ns | [22484, 23467] | YES | 0.0000 | 0.0000 | 0 |
| reg_bound_units | 12132ns | -132.9ns (-1.1%) | [-158, -79]ns | [12110, 12166] | YES | 0.0001 | 0.0000 | 0 |
| reg_typed_twin | 12199ns | no significant difference | [-51, +1]ns | [12002, 12218] | no | 0.0859 | 0.0687 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | reg_typed | reg_bound_columns | reg_bound_record | reg_bound_strided | reg_bound_units | reg_typed_twin |
|---|---|---|---|---|---|---|---|
| 1 | 1 | 11959ns | +0.4% | +640.7% | +96.4% | +3.7% | +0.2% |
| 1 | 2 | 12273ns | -1.2% | +623.2% | +91.1% | -1.3% | +1.4% |
| 1 | 3 | 12447ns | -0.8% | +617.3% | +89.3% | -0.4% | -4.2% |
| 1 | 4 | 12274ns | -20.0% | +494.8% | +91.3% | -1.6% | -2.5% |
| 1 | 5 | 11982ns | +4.4% | +640.1% | +82.5% | -13.2% | +3.6% |
| 1 | 6 | 12016ns | -0.1% | +642.3% | +59.8% | +4.8% | +1.6% |
| 1 | 7 | 12260ns | +2.6% | +622.7% | +78.7% | -1.0% | -0.7% |
| 1 | 8 | 12466ns | -1.3% | +615.0% | +88.7% | -3.0% | -0.5% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| reg_bound_columns | -0.300 | moderate- |
| reg_bound_record | -0.174 | ok |
| reg_bound_strided | 0.354 | moderate+ |
| reg_bound_units | -0.264 | moderate- |
| reg_typed | 0.165 | ok |
| reg_typed_twin | -0.340 | moderate- |

**Consistency summary:**

- **reg_bound_columns**: won 4/8, lost 3/8
- **reg_bound_record**: won 0/8, lost 8/8
- **reg_bound_strided**: won 0/8, lost 8/8
- **reg_bound_units**: won 6/8, lost 2/8
- **reg_typed_twin**: won 4/8, lost 4/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| reg_bound_columns | 153487.1ns | 12051.9ns | 1273.6% | HIGH |
| reg_bound_record | 152836.2ns | 87075.8ns | 175.5% | HIGH |
| reg_bound_strided | 152721.5ns | 22502.7ns | 678.7% | HIGH |
| reg_bound_units | 152716.9ns | 12038.1ns | 1268.6% | HIGH |
| reg_typed | 157007.0ns | 12255.5ns | 1281.1% | HIGH |
| reg_typed_twin | 156149.4ns | 12172.5ns | 1282.8% | HIGH |

## Distribution (algo ns)

```
reg_bound_columns (n=160, range 10621.9-12833.2 ns)
  10621.9 |
  10732.5 |
  10843.1 |
  10953.6 |
  11064.2 |
  11174.8 |
  11285.3 |
  11395.9 |
  11506.4 |
  11617.0 |
  11727.6 |
  11838.1 |
  11948.7 |#################################
  12059.3 |
  12169.8 |#
  12280.4 |########################################
  12391.0 |######
  12501.5 |#################################
  12612.1 |####
  12722.6 |
  (20 below, 1 above range)

reg_bound_record (n=160, range 78841.0-89960.9 ns)
  78841.0 |
  79397.0 |
  79953.0 |
  80509.0 |
  81065.0 |
  81621.0 |
  82177.0 |
  82733.0 |
  83289.0 |
  83845.0 |
  84401.0 |
  84957.0 |
  85513.0 |
  86069.0 |
  86625.0 |
  87181.0 |
  87737.0 |
  88292.9 |########################################
  88848.9 |####################################
  89404.9 |
  (20 below, 5 above range)

reg_bound_strided (n=160, range 20130.3-23655.6 ns)
  20130.3 |
  20306.5 |
  20482.8 |
  20659.1 |
  20835.3 |
  21011.6 |
  21187.9 |
  21364.1 |
  21540.4 |####
  21716.7 |#############
  21892.9 |#########
  22069.2 |##
  22245.5 |##
  22421.7 |##
  22598.0 |
  22774.3 |
  22950.5 |
  23126.8 |
  23303.1 |#################
  23479.3 |########################################
  (20 below, 7 above range)

reg_bound_units (n=160, range 11022.8-12547.9 ns)
  11022.8 |
  11099.0 |
  11175.3 |
  11251.5 |
  11327.8 |
  11404.0 |
  11480.3 |
  11556.6 |
  11632.8 |
  11709.1 |
  11785.3 |
  11861.6 |
  11937.9 |
  12014.1 |#########################
  12090.4 |########################################
  12166.6 |###
  12242.9 |
  12319.2 |###########
  12395.4 |#########################
  12471.7 |
  (20 below, 21 above range)

reg_typed (n=160, range 11939.6-12712.3 ns)
  11939.6 |################################
  11978.2 |##############
  12016.9 |
  12055.5 |
  12094.1 |##
  12132.8 |
  12171.4 |
  12210.0 |###############
  12248.7 |########################################
  12287.3 |#########
  12325.9 |#
  12364.6 |#
  12403.2 |#####
  12441.9 |############################
  12480.5 |#####
  12519.1 |##
  12557.8 |
  12596.4 |
  12635.0 |
  12673.7 |
  (13 below, 2 above range)

reg_typed_twin (n=160, range 11925.3-12435.0 ns)
  11925.3 |###################
  11950.8 |##################################
  11976.3 |#############################
  12001.7 |########
  12027.2 |
  12052.7 |
  12078.2 |
  12103.7 |
  12129.2 |#
  12154.7 |#
  12180.1 |####################
  12205.6 |######################
  12231.1 |###
  12256.6 |
  12282.1 |
  12307.6 |
  12333.0 |
  12358.5 |
  12384.0 |########################################
  12409.5 |######################################
  (19 below, 14 above range)

```

## Diagnostics

- **reg_bound_columns**: bridge=1269.2% of algo (FFI overhead may distort results)
- **reg_bound_record**: bridge=175.9% of algo (FFI overhead may distort results)
- **reg_bound_strided**: bridge=666.6% of algo (FFI overhead may distort results)
- **reg_bound_units**: bridge=1287.0% of algo (FFI overhead may distort results)
- **reg_typed**: bridge=1273.4% of algo (FFI overhead may distort results)
- **reg_typed_twin**: bridge=1279.7% of algo (FFI overhead may distort results)
