# Seq collection member: live-stream vs snapshot-copy across payload size

2 variants, 120 samples per variant.
Baseline: **seq_live**

## Highlights

Baseline for all deltas below: **seq_live**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (seq_live) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline seq_live has the worst median (997.52 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest seq_snapshot at 867.33 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### seq_snapshot dominates: 15% faster than the next best (seq_live)

seq_snapshot (867.33 us) leads seq_live (997.52 us) by 15%, a clear separation rather than a photo finish. CV 5.6%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### seq_live shows warm-up / thermal drift (autocorr +0.65)

seq_live's per-pass series has lag-1 autocorrelation +0.65, indicating warm-up / thermal drift. Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: seq_snapshot** at 867326.2 ns median (-13.1% vs baseline)
- 1 variant significantly faster than baseline
- Spread: 1.15x (fastest 867326.2 ns, slowest 997520.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | Seq collection member: live-stream vs snapshot-copy across payload size |
| Passes | 5 |
| Runs per pass | 2000 |
| Batch size | 250 |
| Harness runs | 3 |
| Cooldown schedule | 0ms |
| Master seed | 0x123456789abcdef0 |
| Counter frequency | 2112730567 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| seq_live | 969191ns | 997601ns | 829729ns | 997763ns | 1022937ns | base |
| seq_snapshot | 885948ns | 867433ns | 866277ns | 870932ns | 950666ns | -8.59% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| seq_live | 969108ns | 829658ns | 1022846ns | base | 0.068 |
| seq_snapshot | 885832ns | 866166ns | 950540ns | -8.59% | 0.074 |

## Performance model

- Peak throughput: **0.079 Gops/s** (seq_live; best 20% batches)
- Ops per call: 65536

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| seq_live | 0.066 | 83.2% |
| seq_snapshot | 0.076 | 95.7% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| seq_live | 969191ns | 969191ns | base |
| seq_snapshot | 885948ns | 885948ns | -8.59% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| seq_live | 997521ns | base | --- | [997307, 997822] | --- | --- | --- | --- |
| seq_snapshot | 867326ns | -128365.2ns (-12.9%) | [-130262, -117567]ns | [867054, 868832] | YES | 0.0000 | 0.0000 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | seq_live | seq_snapshot |
|---|---|---|---|
| 1 | 1 | 816542ns | +6.2% |
| 1 | 2 | 816687ns | +6.1% |
| 1 | 3 | 817683ns | +9.6% |
| 1 | 4 | 997611ns | -13.1% |
| 1 | 5 | 997195ns | -10.9% |
| 2 | 1 | 997117ns | -13.1% |
| 2 | 2 | 1032975ns | -16.0% |
| 2 | 3 | 997907ns | -13.2% |
| 2 | 4 | 997421ns | -12.1% |
| 2 | 5 | 997861ns | -13.1% |
| 3 | 1 | 1008832ns | +4.8% |
| 3 | 2 | 997923ns | -12.0% |
| 3 | 3 | 998039ns | -11.2% |
| 3 | 4 | 997614ns | -13.1% |
| 3 | 5 | 998112ns | -13.1% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| seq_live | 0.649 | HIGH+ (drift/warm-up) |
| seq_snapshot | -0.104 | ok |

**Consistency summary:**

- **seq_snapshot**: won 11/15, lost 4/15

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| seq_live | 97435.5ns | 969107.8ns | 10.1% | HIGH |
| seq_snapshot | 111239.9ns | 885832.2ns | 12.6% | HIGH |

## Distribution (algo ns)

```
seq_live (n=120, range 829658.3-1022846.5 ns)
  829658.3 |
  839317.7 |#
  848977.1 |
  858636.5 |
  868295.9 |
  877955.3 |
  887614.7 |
  897274.1 |
  906933.5 |
  916593.0 |
  926252.4 |
  935911.8 |
  945571.2 |
  955230.6 |
  964890.0 |
  974549.4 |
  984208.8 |
  993868.2 |########################################
  1003527.6 |#
  1013187.1 |####
  (19 below, 9 above range)

seq_snapshot (n=120, range 866165.9-950539.7 ns)
  866165.9 |########################################
  870384.6 |#
  874603.3 |####
  878821.9 |##
  883040.6 |
  887259.3 |
  891478.0 |####
  895696.7 |#####
  899915.4 |
  904134.1 |
  908352.8 |
  912571.5 |
  916790.2 |
  921008.8 |
  925227.5 |
  929446.2 |
  933664.9 |
  937883.6 |
  942102.3 |
  946321.0 |
  (11 below, 8 above range)

```

## Diagnostics

- **seq_live**: autocorrelation=0.65 (measurement drift or warm-up artifact)
- **seq_live**: bridge=10.1% of algo (FFI overhead may distort results)
- **seq_snapshot**: bridge=12.6% of algo (FFI overhead may distort results)
