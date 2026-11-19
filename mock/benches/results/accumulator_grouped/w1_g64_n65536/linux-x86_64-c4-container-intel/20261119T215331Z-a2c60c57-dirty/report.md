# W = 1, G = 64

3 variants, 18 samples per variant.
Baseline: **grouped**

## Highlights

Baseline for all deltas below: **grouped**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (grouped) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline grouped has the worst median (337.76 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest per_core at 313.11 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### grouped shows alternating (throttle bounce) (autocorr -0.66)

grouped's per-pass series has lag-1 autocorrelation -0.66, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: per_core** at 313106.8 ns median (-7.3% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.08x (fastest 313106.8 ns, slowest 337764.4 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | W = 1, G = 64 |
| Passes | 3 |
| Runs per pass | 60 |
| Batch size | 10 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2100451752 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| grouped | 353843ns | 338398ns | 316975ns | 337316ns | 397068ns | base |
| per_core | 319605ns | 314251ns | 289722ns | 311553ns | 346625ns | -9.68% |
| sequential | 349020ns | 324476ns | 313393ns | 324242ns | 404001ns | -1.36% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| grouped | 352688ns | 316119ns | 395642ns | base | 0.186 |
| per_core | 318575ns | 289081ns | 345524ns | -9.67% | 0.206 |
| sequential | 347595ns | 311938ns | 402536ns | -1.44% | 0.189 |

## Performance model

- Peak throughput: **0.227 Gops/s** (per_core; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| grouped | 0.194 | 85.6% |
| per_core | 0.209 | 92.3% |
| sequential | 0.203 | 89.4% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| grouped | 353843ns | 353843ns | base |
| per_core | 319605ns | 319605ns | -9.68% |
| sequential | 349020ns | 349020ns | -1.36% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| grouped | 337764ns | base | --- | [333282, 343439] | --- | --- | --- | --- |
| per_core | 313107ns | -25737.7ns (-7.6%) | [-37373, -10444]ns | [305908, 326631] | YES | 0.0003 | 0.0001 | 0 |
| sequential | 323281ns | no significant difference | [-18584, +30597]ns | [316625, 371298] | no | 0.4807 | 0.4807 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | grouped | per_core | sequential |
|---|---|---|---|---|
| 1 | 1 | 340225ns | -9.2% | +4.6% |
| 1 | 2 | 333282ns | -8.2% | -5.0% |
| 1 | 3 | 341327ns | -0.6% | -4.7% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| grouped | -0.656 | HIGH- (thermal bounce) |
| per_core | -0.214 | moderate- |
| sequential | -0.301 | moderate- |

**Consistency summary:**

- **per_core**: won 3/3, lost 0/3
- **sequential**: won 2/3, lost 1/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| grouped | 367080.8ns | 352688.0ns | 104.1% | HIGH |
| per_core | 318351.9ns | 318574.6ns | 99.9% | HIGH |
| sequential | 363474.3ns | 347595.2ns | 104.6% | HIGH |

## Distribution (algo ns)

```
grouped (n=18, range 316118.7-395641.5 ns)
  316118.7 |
  320094.9 |
  324071.0 |##########
  328047.1 |
  332023.3 |########################################
  335999.4 |##############################
  339975.6 |##############################
  343951.7 |##########
  347927.8 |
  351904.0 |
  355880.1 |##########
  359856.3 |
  363832.4 |
  367808.5 |
  371784.7 |
  375760.8 |
  379736.9 |
  383713.1 |##########
  387689.2 |
  391665.4 |
  (2 below, 2 above range)

per_core (n=18, range 289080.8-345524.1 ns)
  289080.8 |####################
  291903.0 |####################
  294725.2 |
  297547.3 |
  300369.5 |####################
  303191.6 |########################################
  306013.8 |####################
  308836.0 |########################################
  311658.1 |
  314480.3 |####################
  317302.4 |########################################
  320124.6 |
  322946.8 |####################
  325768.9 |
  328591.1 |####################
  331413.3 |
  334235.4 |####################
  337057.6 |
  339879.7 |####################
  342701.9 |
  (1 below, 2 above range)

sequential (n=18, range 311938.2-402536.2 ns)
  311938.2 |##########################
  316468.1 |########################################
  320998.0 |########################################
  325527.9 |#############
  330057.8 |
  334587.7 |
  339117.6 |#############
  343647.5 |
  348177.4 |
  352707.3 |
  357237.2 |
  361767.1 |
  366297.0 |
  370826.9 |#############
  375356.8 |
  379886.7 |
  384416.6 |
  388946.5 |#############
  393476.4 |#############
  398006.3 |#############
  (2 below, 2 above range)

```

## Diagnostics

- **grouped**: bridge=103.4% of algo (FFI overhead may distort results)
- **per_core**: bridge=100.0% of algo (FFI overhead may distort results)
- **sequential**: bridge=105.2% of algo (FFI overhead may distort results)
