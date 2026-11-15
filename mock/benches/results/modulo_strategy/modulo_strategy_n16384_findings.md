# Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv)

3 variants, 160 samples per variant.
Baseline: **mod_npow2_const**

## Highlights

Baseline for all deltas below: **mod_npow2_const**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### mod_pow2_const dominates: 121% faster than the next best (mod_npow2_const)

mod_pow2_const (25.59 us) leads mod_npow2_const (56.44 us) by 121%, a clear separation rather than a photo finish. CV 2.8%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### mod_pow2_const beats baseline by 55% (significant)

mod_pow2_const is -30.83 us (55%) faster than baseline mod_npow2_const, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### mod_var is an outlier: 2.8x slower than the field

mod_var (71.63 us) is 2.8x the fastest (25.59 us), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

## Key findings

- **Fastest: mod_pow2_const** at 25592.5 ns median (-54.7% vs baseline)
- 1 variant significantly faster than baseline
- 1 variant significantly slower than baseline
- Spread: 2.80x (fastest 25592.5 ns, slowest 71626.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Modulo strategy: const pow2 (AND mask) vs const non-pow2 (magic mul) vs opaque divisor (full udiv) |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x4dd582c1e7fa3b96 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| mod_npow2_const | 58790ns | 58565ns | 58472ns | 58642ns | 59552ns | base |
| mod_pow2_const | 27988ns | 27704ns | 27631ns | 27777ns | 28976ns | -52.39% |
| mod_var | 74299ns | 73778ns | 73668ns | 73843ns | 76296ns | +26.38% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| mod_npow2_const | 56612ns | 56360ns | 57245ns | base | 0.289 |
| mod_pow2_const | 25854ns | 25531ns | 26771ns | -54.33% | 0.634 |
| mod_var | 72120ns | 71542ns | 74052ns | +27.39% | 0.227 |

## Performance model

- Peak throughput: **0.642 Gops/s** (mod_pow2_const; best 20% batches)
- Ops per call: 16384

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| mod_npow2_const | 0.290 | 45.2% |
| mod_pow2_const | 0.640 | 99.8% |
| mod_var | 0.229 | 35.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| mod_npow2_const | 58790ns | 58790ns | base |
| mod_pow2_const | 27988ns | 27988ns | -52.39% |
| mod_var | 74299ns | 74299ns | +26.38% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| mod_npow2_const | 56438ns | base | --- | [56416, 56457] | --- | --- | --- | --- |
| mod_pow2_const | 25592ns | -30832.3ns (-54.6%) | [-30857, -30801]ns | [25578, 25629] | YES | 0.0000 | 0.0000 | 0 |
| mod_var | 71626ns | +15195.4ns (+26.9%) | [+15167, +15219]ns | [71606, 71645] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | mod_npow2_const | mod_pow2_const | mod_var |
|---|---|---|---|---|
| 1 | 1 | 56428ns | -54.7% | +26.9% |
| 1 | 2 | 56900ns | -55.0% | +25.9% |
| 1 | 3 | 56874ns | -54.6% | +31.5% |
| 1 | 4 | 56418ns | -54.6% | +27.0% |
| 1 | 5 | 56424ns | -54.1% | +27.0% |
| 1 | 6 | 56445ns | -54.8% | +26.9% |
| 1 | 7 | 56389ns | -54.6% | +27.0% |
| 1 | 8 | 56399ns | -54.7% | +26.9% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| mod_npow2_const | 0.303 | moderate+ |
| mod_pow2_const | -0.304 | moderate- |
| mod_var | -0.150 | ok |

**Consistency summary:**

- **mod_pow2_const**: won 8/8, lost 0/8
- **mod_var**: won 0/8, lost 8/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| mod_npow2_const | 3.2ns | 56611.7ns | 0.0% |  |
| mod_pow2_const | 3.3ns | 25853.7ns | 0.0% |  |
| mod_var | 3.7ns | 72119.9ns | 0.0% |  |

## Distribution (algo ns)

```
mod_npow2_const (n=160, range 56360.4-57244.6 ns)
  56360.4 |########################################
  56404.6 |#############################
  56448.8 |########
  56493.1 |#######
  56537.3 |#
  56581.5 |##
  56625.7 |
  56669.9 |####
  56714.1 |##
  56758.3 |####
  56802.5 |#
  56846.7 |###
  56890.9 |###
  56935.2 |###
  56979.4 |##
  57023.6 |#
  57067.8 |#
  57112.0 |#
  57156.2 |#
  57200.4 |
  (15 below, 7 above range)

mod_pow2_const (n=160, range 25530.6-26771.0 ns)
  25530.6 |########################################
  25592.6 |##########
  25654.7 |####
  25716.7 |#####
  25778.7 |###
  25840.7 |#
  25902.7 |##
  25964.7 |
  26026.8 |##
  26088.8 |##
  26150.8 |#
  26212.8 |#
  26274.8 |
  26336.9 |
  26398.9 |###
  26460.9 |###
  26522.9 |
  26584.9 |
  26647.0 |#
  26709.0 |
  (17 below, 6 above range)

mod_var (n=160, range 71541.9-74052.4 ns)
  71541.9 |########################################
  71667.4 |########
  71792.9 |####
  71918.5 |##
  72044.0 |#
  72169.5 |
  72295.0 |#
  72420.6 |
  72546.1 |#
  72671.6 |
  72797.2 |
  72922.7 |
  73048.2 |#
  73173.7 |
  73299.3 |
  73424.8 |
  73550.3 |
  73675.8 |
  73801.4 |
  73926.9 |
  (15 below, 13 above range)

```
