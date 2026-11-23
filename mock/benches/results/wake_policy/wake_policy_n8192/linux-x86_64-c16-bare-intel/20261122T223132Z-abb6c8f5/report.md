# frame wake policy: park-immediately vs bounded spin pre-roll

7 variants, 18 samples per variant.
Baseline: **wp_isb8**

## Highlights

Baseline for all deltas below: **wp_isb8**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### Baseline (wp_isb8) is the SLOWEST variant; every rival beats it

The declared/defaulted baseline wp_isb8 has the worst median (78.99 us). Every delta is therefore measured against the worst performer, which flatters all rivals and compresses the differences that matter among them (e.g. fastest wp_nh2k at 59.49 us).

_Why it matters:_ A baseline picked by accident (often the first variant to run / sort) silently skews every comparison. Re-baseline via `[bench.<name>.normalise]` on a representative variant.

### wp_nh2k is fastest but the noisiest (CV 47.0%)

wp_nh2k wins on median (59.49 us) yet has the highest variance (CV 47.0%), while wp_nh8k is the steadiest (CV 32.9%, 78.75 us).

_Why it matters:_ For latency-sensitive or tail-bound paths, the steadier variant can beat the faster-on-average one; weigh peak vs consistency.

### wp_nh8k shows alternating (throttle bounce) (autocorr -0.52)

wp_nh8k's per-pass series has lag-1 autocorrelation -0.52, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

### Whole-field spread is below the measurement noise floor

The fastest-to-slowest gap (19.50 us) is smaller than the fastest variant's own run-to-run std-dev (27.94 us); the ranking is inside the noise.

_Why it matters:_ When the spread is below resolution, any apparent ordering is likely noise; increase work per call before trusting a winner.

### wp_isb8 is inconsistent: worst-20% is 2.4x its best-20%

wp_isb8's best 20% of batches run at 49.66 us but its worst 20% at 121.18 us (2.4x) - a bimodal or bursty profile the median hides.

_Why it matters:_ A fat tail matters for latency budgets even when the median looks fine; a steadier variant may serve better under load.

## Key findings

- **Fastest: wp_nh2k** at 59486.4 ns median (-24.7% vs baseline)
- 2 variants significantly faster than baseline
- Spread: 1.33x (fastest 59486.4 ns, slowest 78990.8 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | frame wake policy: park-immediately vs bounded spin pre-roll |
| Passes | 3 |
| Runs per pass | 30 |
| Batch size | 10 |
| Harness runs | 2 |
| Cooldown schedule | 0ms |
| Master seed | 0x51c3510700000003 |
| Counter frequency | 2112740287 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| wp_isb8 | 92835ns | 86413ns | 56509ns | 79635ns | 130798ns | base |
| wp_lp0 | 82879ns | 68900ns | 55250ns | 69246ns | 117143ns | -10.72% |
| wp_nh128 | 83548ns | 73575ns | 52759ns | 71610ns | 116850ns | -10.00% |
| wp_nh2k | 80308ns | 67711ns | 55790ns | 65198ns | 115232ns | -13.49% |
| wp_nh8k | 84009ns | 85817ns | 51098ns | 73996ns | 115486ns | -9.51% |
| wp_park | 81556ns | 72088ns | 53744ns | 69336ns | 113794ns | -12.15% |
| wp_spin | 82664ns | 72116ns | 55022ns | 65885ns | 121653ns | -10.96% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| wp_isb8 | 84621ns | 49656ns | 121182ns | base | 0.097 |
| wp_lp0 | 75117ns | 48182ns | 107795ns | -11.23% | 0.109 |
| wp_nh128 | 75918ns | 46943ns | 107905ns | -10.28% | 0.108 |
| wp_nh2k | 72146ns | 49439ns | 105390ns | -14.74% | 0.114 |
| wp_nh8k | 76748ns | 45337ns | 107365ns | -9.30% | 0.107 |
| wp_park | 74466ns | 48439ns | 105312ns | -12.00% | 0.110 |
| wp_spin | 75171ns | 48085ns | 112789ns | -11.17% | 0.109 |

## Performance model

- Peak throughput: **0.181 Gops/s** (wp_nh8k; best 20% batches)
- Ops per call: 8192

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| wp_isb8 | 0.104 | 57.4% |
| wp_lp0 | 0.133 | 73.4% |
| wp_nh128 | 0.124 | 68.9% |
| wp_nh2k | 0.138 | 76.2% |
| wp_nh8k | 0.104 | 57.6% |
| wp_park | 0.124 | 68.5% |
| wp_spin | 0.125 | 68.9% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| wp_isb8 | 92835ns | 92835ns | base |
| wp_lp0 | 82879ns | 82879ns | -10.72% |
| wp_nh128 | 83548ns | 83548ns | -10.00% |
| wp_nh2k | 80308ns | 80308ns | -13.49% |
| wp_nh8k | 84009ns | 84009ns | -9.51% |
| wp_park | 81556ns | 81556ns | -12.15% |
| wp_spin | 82664ns | 82664ns | -10.96% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| wp_isb8 | 78991ns | base | --- | [57285, 96930] | --- | --- | --- | --- |
| wp_lp0 | 61728ns | no significant difference | [-28696, +5385]ns | [56554, 93277] | no | 0.4807 | 0.4807 | 0 |
| wp_nh128 | 65830ns | no significant difference | [-26308, +256]ns | [52595, 95998] | no | 0.2855 | 0.0963 | 0 |
| wp_nh2k | 59486ns | no significant difference | [-25465, +1020]ns | [53139, 75454] | no | 0.2855 | 0.2379 | 0 |
| wp_nh8k | 78751ns | -6677.9ns (-8.5%) | [-17564, -1046]ns | [51730, 92157] | YES (adj: no) | 0.2855 | 0.0963 | 0 |
| wp_park | 66210ns | no significant difference | [-19423, +1584]ns | [53416, 89519] | no | 0.2855 | 0.2379 | 0 |
| wp_spin | 65767ns | no significant difference | [-32010, +8791]ns | [50148, 98469] | no | 0.2855 | 0.2379 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | wp_isb8 | wp_lp0 | wp_nh128 | wp_nh2k | wp_nh8k | wp_park | wp_spin |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 92170ns | -39.9% | -9.6% | -43.4% | -12.4% | -46.4% | +18.2% |
| 1 | 2 | 93763ns | -45.7% | -41.4% | -28.9% | -15.0% | -37.7% | -21.6% |
| 1 | 3 | 73969ns | -0.2% | +29.9% | -14.3% | +10.1% | -21.0% | -32.9% |
| 2 | 1 | 54264ns | +6.0% | +76.8% | +6.7% | +43.4% | +16.8% | +28.9% |
| 2 | 2 | 82825ns | -3.4% | -39.3% | -34.5% | +30.9% | +5.5% | -38.6% |
| 2 | 3 | 51275ns | +25.2% | -4.6% | +2.0% | -9.0% | +34.6% | -0.3% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| wp_isb8 | -0.013 | ok |
| wp_lp0 | -0.279 | moderate- |
| wp_nh128 | -0.016 | ok |
| wp_nh2k | 0.104 | ok |
| wp_nh8k | -0.515 | HIGH- (thermal bounce) |
| wp_park | 0.254 | moderate+ |
| wp_spin | 0.124 | ok |

**Consistency summary:**

- **wp_lp0**: won 4/6, lost 2/6
- **wp_nh128**: won 4/6, lost 2/6
- **wp_nh2k**: won 4/6, lost 2/6
- **wp_nh8k**: won 3/6, lost 3/6
- **wp_park**: won 3/6, lost 3/6
- **wp_spin**: won 4/6, lost 2/6

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| wp_isb8 | 34414.5ns | 84620.6ns | 40.7% | HIGH |
| wp_lp0 | 32777.4ns | 75117.1ns | 43.6% | HIGH |
| wp_nh128 | 32695.3ns | 75917.8ns | 43.1% | HIGH |
| wp_nh2k | 35137.6ns | 72145.6ns | 48.7% | HIGH |
| wp_nh8k | 31143.3ns | 76748.1ns | 40.6% | HIGH |
| wp_park | 29880.6ns | 74465.9ns | 40.1% | HIGH |
| wp_spin | 32738.0ns | 75171.0ns | 43.6% | HIGH |

## Distribution (algo ns)

```
wp_isb8 (n=18, range 49656.0-121182.0 ns)
  49656.0 |########################################
  53232.3 |#############
  56808.6 |#############
  60384.9 |#############
  63961.2 |
  67537.5 |
  71113.8 |#############
  74690.1 |#############
  78266.4 |
  81842.7 |#############
  85419.0 |
  88995.3 |#############
  92571.6 |#############
  96147.9 |##########################
  99724.2 |#############
  103300.5 |
  106876.8 |
  110453.1 |#############
  114029.4 |
  117605.7 |
  (1 below, 2 above range)

wp_lp0 (n=18, range 48182.3-107794.6 ns)
  48182.3 |#############
  51162.9 |
  54143.6 |########################################
  57124.2 |########################################
  60104.8 |
  63085.4 |#############
  66066.0 |
  69046.6 |
  72027.2 |#############
  75007.9 |
  77988.5 |#############
  80969.1 |
  83949.7 |
  86930.3 |
  89910.9 |
  92891.5 |##########################
  95872.2 |
  98852.8 |
  101833.4 |
  104814.0 |
  (2 below, 4 above range)

wp_nh128 (n=18, range 46943.1-107905.4 ns)
  46943.1 |########################################
  49991.2 |####################
  53039.4 |####################
  56087.5 |####################
  59135.6 |####################
  62183.7 |####################
  65231.8 |####################
  68279.9 |
  71328.0 |
  74376.2 |
  77424.3 |
  80472.4 |####################
  83520.5 |
  86568.6 |
  89616.7 |
  92664.8 |####################
  95713.0 |########################################
  98761.1 |
  101809.2 |########################################
  104857.3 |
  (2 below, 2 above range)

wp_nh2k (n=18, range 49438.7-105389.6 ns)
  49438.7 |##########################
  52236.2 |########################################
  55033.8 |#############
  57831.3 |#############
  60628.9 |##########################
  63426.4 |
  66223.9 |#############
  69021.5 |
  71819.0 |#############
  74616.6 |#############
  77414.1 |
  80211.7 |
  83009.2 |
  85806.8 |
  88604.3 |
  91401.9 |
  94199.4 |
  96997.0 |
  99794.5 |
  102592.1 |#############
  (2 below, 3 above range)

wp_nh8k (n=18, range 45337.1-107364.6 ns)
  45337.1 |##########################
  48438.5 |#############
  51539.8 |#############
  54641.2 |#############
  57742.6 |
  60844.0 |
  63945.3 |
  67046.7 |
  70148.1 |
  73249.5 |#############
  76350.8 |#############
  79452.2 |########################################
  82553.6 |
  85654.9 |
  88756.3 |#############
  91857.7 |
  94959.1 |#############
  98060.4 |
  101161.8 |
  104263.2 |
  (2 below, 4 above range)

wp_park (n=18, range 48439.4-105312.2 ns)
  48439.4 |########################################
  51283.0 |########################################
  54126.7 |####################
  56970.3 |########################################
  59814.0 |
  62657.6 |####################
  65501.2 |
  68344.9 |####################
  71188.5 |
  74032.1 |####################
  76875.8 |####################
  79719.4 |
  82563.1 |
  85406.7 |####################
  88250.3 |
  91094.0 |####################
  93937.6 |
  96781.2 |
  99624.9 |####################
  102468.5 |####################
  (1 below, 2 above range)

wp_spin (n=18, range 48084.6-112789.4 ns)
  48084.6 |########################################
  51319.9 |
  54555.1 |
  57790.3 |
  61025.6 |#####
  64260.8 |
  67496.1 |#####
  70731.3 |#####
  73966.5 |#####
  77201.8 |
  80437.0 |
  83672.2 |
  86907.5 |#####
  90142.7 |
  93377.9 |
  96613.2 |
  99848.4 |
  103083.7 |
  106318.9 |#####
  109554.1 |#####
  (1 below, 3 above range)

```

## Diagnostics

- **wp_isb8**: CV=39.0% (high variance, measurements may be unstable)
- **wp_isb8**: bridge=42.8% of algo (FFI overhead may distort results)
- **wp_lp0**: CV=33.4% (high variance, measurements may be unstable)
- **wp_lp0**: bridge=53.5% of algo (FFI overhead may distort results)
- **wp_nh128**: CV=35.2% (high variance, measurements may be unstable)
- **wp_nh128**: bridge=50.3% of algo (FFI overhead may distort results)
- **wp_nh2k**: CV=38.7% (high variance, measurements may be unstable)
- **wp_nh2k**: bridge=58.6% of algo (FFI overhead may distort results)
- **wp_nh8k**: CV=33.7% (high variance, measurements may be unstable)
- **wp_nh8k**: bridge=38.5% of algo (FFI overhead may distort results)
- **wp_park**: CV=34.2% (high variance, measurements may be unstable)
- **wp_park**: bridge=42.7% of algo (FFI overhead may distort results)
- **wp_spin**: CV=38.7% (high variance, measurements may be unstable)
- **wp_spin**: bridge=50.2% of algo (FFI overhead may distort results)
