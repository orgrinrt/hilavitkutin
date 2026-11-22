# Column iteration patterns: sequential vs strided vs data-dependent gather

3 variants, 160 samples per variant.
Baseline: **iter_gather**

## Highlights

Baseline for all deltas below: **iter_gather**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (iter_gather) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline iter_gather has the worst median (249 ns). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest iter_strided at 3 ns).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### iter_strided dominates: 3182% faster than the next best (iter_seq)

iter_strided (3 ns) leads iter_seq (108 ns) by 3182%, a clear separation rather than a photo finish. CV 33.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### iter_strided beats baseline by 99% (significant)

iter_strided is -246 ns (99%) faster than baseline iter_gather, with a CI that excludes zero.

_Why it matters:_ A large, significant improvement over the current baseline is a concrete reason to switch.

### iter_gather is an outlier: 75.5x slower than the field

iter_gather (249 ns) is 75.5x the fastest (3 ns), well off the pack.

_Why it matters:_ A >2x outlier is almost never the right choice; if it is intentional (e.g. it buys correctness), say so explicitly.

### iter_strided is fastest but the noisiest (CV 33.9%)

iter_strided wins on median (3 ns) yet has the highest variance (CV 33.9%), while iter_gather is the steadiest (CV 2.2%, 249 ns).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### iter_strided shows alternating (throttle bounce) (autocorr -0.56)

iter_strided's per-pass series has lag-1 autocorrelation -0.56, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Wide spread: slowest is 75.5x the fastest

Fastest iter_strided (3 ns) to slowest iter_gather (249 ns): 75.5x. The strategy choice matters a lot for this workload.

_Why it matters:_ A wide field means the strategy is load-bearing here; getting it right (or wrong) has large consequences.

### iter_strided is inconsistent: worst-20% is 2.4x its best-20%

iter_strided's best 20% of batches run at 2 ns but its worst 20% at 5 ns (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: iter_strided** at 3.3 ns median (-98.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 75.52x (fastest 3.3 ns, slowest 249.2 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Column iteration patterns: sequential vs strided vs data-dependent gather |
| Passes | 8 |
| Runs per pass | 2000 |
| Batch size | 100 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x3a5c8e91b2d4f607 |
| Counter frequency | 24000000 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| iter_gather | 2361ns | 2339ns | 2335ns | 2342ns | 2447ns | base |
| iter_seq | 2212ns | 2209ns | 2203ns | 2209ns | 2231ns | -6.32% |
| iter_strided | 2100ns | 2096ns | 2090ns | 2096ns | 2121ns | -11.07% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| iter_gather | 250ns | 244ns | 259ns | base | 4.089 |
| iter_seq | 108ns | 104ns | 112ns | -56.87% | 9.480 |
| iter_strided | 3ns | 2ns | 5ns | -98.61% | 293.410 |

## Performance model

- Peak throughput: **480.469 Gops/s** (iter_strided; best 20% batches)
- Ops per call: 1024

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| iter_gather | 4.109 | 0.9% |
| iter_seq | 9.455 | 2.0% |
| iter_strided | 310.303 | 64.6% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| iter_gather | 2361ns | 2361ns | base |
| iter_seq | 2212ns | 2212ns | -6.32% |
| iter_strided | 2100ns | 2100ns | -11.07% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| iter_gather | 249ns | base | --- | [249, 250] | --- | --- | --- | --- |
| iter_seq | 108ns | -141.2ns (-56.7%) | [-142, -140]ns | [108, 109] | YES | 0.0000 | 0.0000 | 0 |
| iter_strided | 3ns | -245.8ns (-98.6%) | [-246, -245]ns | [3, 4] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | iter_gather | iter_seq | iter_strided |
|---|---|---|---|---|
| 1 | 1 | 251ns | -56.8% | -98.8% |
| 1 | 2 | 250ns | -56.7% | -98.4% |
| 1 | 3 | 248ns | -56.4% | -98.8% |
| 1 | 4 | 260ns | -58.5% | -98.7% |
| 1 | 5 | 249ns | -56.3% | -98.7% |
| 1 | 6 | 249ns | -56.5% | -98.7% |
| 1 | 7 | 247ns | -56.2% | -98.6% |
| 1 | 8 | 248ns | -56.7% | -98.8% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| iter_gather | -0.163 | ok |
| iter_seq | -0.362 | moderate- |
| iter_strided | -0.559 | HIGH- (thermal bounce) |

**Consistency summary:**

- **iter_seq**: won 8/8, lost 0/8
- **iter_strided**: won 8/8, lost 0/8

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| iter_gather | 2.6ns | 250.4ns | 1.0% |  |
| iter_seq | 2.8ns | 108.0ns | 2.6% |  |
| iter_strided | 13.7ns | 3.5ns | 393.2% | HIGH |

## Distribution (algo ns)

```
iter_gather (n=160, range 244.5-259.5 ns)
    244.5 |########################
    245.2 |########################
    246.0 |##################################
    246.7 |#############
    247.5 |##########################
    248.2 |#####################################
    249.0 |#####################################
    249.7 |#####################################
    250.5 |########################################
    251.2 |##########
    252.0 |#####################
    252.7 |########
    253.5 |########
    254.2 |##
    255.0 |##
    255.7 |##
    256.5 |#####
    257.2 |##########
    258.0 |########
    258.7 |##########
  (12 below, 11 above range)

iter_seq (n=160, range 103.7-111.8 ns)
    103.7 |###
    104.1 |##########
    104.5 |#######
    104.9 |#######
    105.3 |##################
    105.7 |################################
    106.1 |################################
    106.6 |################################
    107.0 |##############
    107.4 |#############################
    107.8 |########################################
    108.2 |#####################
    108.6 |####################################
    109.0 |################################
    109.4 |########################################
    109.8 |#####################
    110.2 |################################
    110.6 |####################################
    111.0 |##################
    111.4 |##################
  (16 below, 10 above range)

iter_strided (n=160, range 2.1-5.1 ns)
      2.1 |
      2.3 |
      2.4 |#########################
      2.6 |
      2.7 |
      2.9 |########################################
      3.0 |
      3.2 |#############################
      3.3 |
      3.5 |
      3.6 |#######################
      3.8 |
      3.9 |
      4.1 |##################
      4.2 |
      4.4 |
      4.5 |###########################
      4.7 |
      4.8 |
      5.0 |##########
  (17 below, 8 above range)

```

## Diagnostics

- **iter_strided**: CV=32.1% (high variance, measurements may be unstable)
- **iter_strided**: bridge=418.2% of algo (FFI overhead may distort results)
