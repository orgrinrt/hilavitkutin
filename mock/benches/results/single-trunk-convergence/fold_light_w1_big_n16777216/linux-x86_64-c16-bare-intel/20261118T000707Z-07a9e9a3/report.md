# fold_light, far above cache, W = 1

7 variants, 6 samples per variant.
Baseline: **head_tail**

## Highlights

Baseline for all deltas below: **head_tail**. (Deltas are paired `variant - baseline` medians; `*` marks a CI that excludes zero.)

### sequential dominates: 17% faster than the next best (kernel_nway)

sequential (3.52 ms) leads kernel_nway (4.13 ms) by 17%, a clear separation rather than a photo finish. CV 8.9%.

_Why it matters:_ A dominant, well-separated winner is a safe default pick for this workload shape.

### sequential shows alternating (throttle bounce) (autocorr -0.67)

sequential's per-pass series has lag-1 autocorrelation -0.67, indicating alternating (throttle bounce). Its timing may not be at steady state.

_Why it matters:_ Autocorrelated samples violate the independence the CIs assume; the interval is optimistic until the drift is warmed out or cooled down.

## Key findings

- **Fastest: sequential** at 3517454.5 ns median (-15.3% vs baseline)
- 1 variant significantly slower than baseline
- Spread: 1.39x (fastest 3517454.5 ns, slowest 4904584.3 ns)

## Methodology

| Parameter | Value |
|---|---|
| Workload | fold_light, far above cache, W = 1 |
| Passes | 3 |
| Runs per pass | 8 |
| Batch size | 4 |
| Harness runs | 1 |
| Cooldown schedule | 0ms |
| Master seed | 0x5eed000000000001 |
| Counter frequency | 2112769393 Hz |
| Drift correction | none |

## End-to-end (all cooldowns combined)

| Variant | mean | median | best 20% | mid 60% | worst 20% | Δ mean |
|---|---|---|---|---|---|---|
| head_tail | 4039844ns | 4155949ns | 3558176ns | 4077732ns | 4223846ns | base |
| head_tail_dyn | 4168123ns | 4186202ns | 4111257ns | 4167481ns | 4197517ns | +3.18% |
| kernel_nway | 4140534ns | 4130497ns | 4107746ns | 4124162ns | 4181487ns | +2.49% |
| kernel_seq | 4205307ns | 4184556ns | 4102824ns | 4178318ns | 4297032ns | +4.10% |
| nway | 4002874ns | 4163130ns | 3376548ns | 4067282ns | 4219423ns | -0.92% |
| sequential | 3732400ns | 3518871ns | 3501254ns | 3514444ns | 4174909ns | -7.61% |
| std_parallel | 5246843ns | 4908295ns | 4507553ns | 4865910ns | 6187889ns | +29.88% |

## Function-under-test only (all cooldowns combined)

| Variant | mean | best 20% | worst 20% | Δ mean | throughput (Gops/s) |
|---|---|---|---|---|---|
| head_tail | 4034790ns | 3555984ns | 4215741ns | base | 4.158 |
| head_tail_dyn | 4162322ns | 4109343ns | 4189450ns | +3.16% | 4.031 |
| kernel_nway | 4135415ns | 4106043ns | 4172026ns | +2.49% | 4.057 |
| kernel_seq | 4199803ns | 4092423ns | 4291887ns | +4.09% | 3.995 |
| nway | 4001242ns | 3375278ns | 4217674ns | -0.83% | 4.193 |
| sequential | 3731130ns | 3500132ns | 4173592ns | -7.53% | 4.497 |
| std_parallel | 5243224ns | 4504127ns | 6184628ns | +29.95% | 3.200 |

## Performance model

- Peak throughput: **4.971 Gops/s** (nway; best 20% batches)
- Ops per call: 16777216

| Variant | Gops/s (median) | % of peak |
|---|---|---|
| head_tail | 4.042 | 81.3% |
| head_tail_dyn | 4.015 | 80.8% |
| kernel_nway | 4.066 | 81.8% |
| kernel_seq | 4.014 | 80.8% |
| nway | 4.032 | 81.1% |
| sequential | 4.770 | 96.0% |
| std_parallel | 3.421 | 68.8% |

## Per-cooldown breakdown (e2e mean)

| Variant | 0ms | avg | Δ avg |
|---|---|---|---|
| head_tail | 4039844ns | 4039844ns | base |
| head_tail_dyn | 4168123ns | 4168123ns | +3.18% |
| kernel_nway | 4140534ns | 4140534ns | +2.49% |
| kernel_seq | 4205307ns | 4205307ns | +4.10% |
| nway | 4002874ns | 4002874ns | -0.92% |
| sequential | 3732400ns | 3732400ns | -7.61% |
| std_parallel | 5246843ns | 5246843ns | +29.88% |

## Statistical comparison (algo, 95% bootstrap CI)

| Variant | median | Δ median | Δ CI | 95% CI | sig? | adj. p | sign p | ties |
|---|---|---|---|---|---|---|---|---|
| head_tail | 4150970ns | base | --- | [3737660, 4215741] | --- | --- | --- | --- |
| head_tail_dyn | 4178600ns | no significant difference | [-37141, +415576]ns | [4118915, 4189450] | no | 1.0000 | 1.0000 | 0 |
| kernel_nway | 4126274ns | no significant difference | [-76496, +374885]ns | [4107945, 4172026] | no | 1.0000 | 1.0000 | 0 |
| kernel_seq | 4179204ns | no significant difference | [-72991, +527540]ns | [4128317, 4291887] | no | 1.0000 | 0.6875 | 0 |
| nway | 4161385ns | no significant difference | [-275834, +143439]ns | [3624666, 4217674] | no | 1.0000 | 0.6875 | 0 |
| sequential | 3517455ns | no significant difference | [-668470, +273092]ns | [3502343, 4173592] | no | 0.6563 | 0.2188 | 0 |
| std_parallel | 4904584ns | +777964.5ns (+18.7%) | [+588100, +2259236]ns | [4640459, 6184628] | YES (adj: no) | 0.1875 | 0.0313 | 0 |

## Per-pass consistency (nonstop algo, Δ vs baseline)

| Run | Pass | head_tail | head_tail_dyn | kernel_nway | kernel_seq | nway | sequential | std_parallel |
|---|---|---|---|---|---|---|---|---|
| 1 | 1 | 4052359ns | +3.4% | +2.6% | +3.4% | +2.7% | -13.3% | +14.5% |
| 1 | 2 | 3900500ns | +6.2% | +5.8% | +9.7% | -7.1% | +7.0% | +28.5% |
| 1 | 3 | 4151512ns | +0.0% | -0.7% | -0.6% | +1.6% | -15.6% | +46.4% |

**Autocorrelation (lag-1) per-pass series:**

| Variant | r₁ | note |
|---|---|---|
| head_tail | -0.564 | HIGH- (thermal bounce) |
| head_tail_dyn | -0.291 | moderate- |
| kernel_nway | -0.116 | ok |
| kernel_seq | -0.549 | HIGH- (thermal bounce) |
| nway | -0.662 | HIGH- (thermal bounce) |
| sequential | -0.667 | HIGH- (thermal bounce) |
| std_parallel | -0.049 | ok |

**Consistency summary:**

- **head_tail_dyn**: won 0/3, lost 2/3
- **kernel_nway**: won 1/3, lost 2/3
- **kernel_seq**: won 1/3, lost 2/3
- **nway**: won 1/3, lost 2/3
- **sequential**: won 2/3, lost 1/3
- **std_parallel**: won 0/3, lost 3/3

## Bridge overhead per variant

| Variant | mean bridge | algo mean | bridge % | flag |
|---|---|---|---|---|
| head_tail | 4139635.5ns | 4034790.4ns | 102.6% | HIGH |
| head_tail_dyn | 4175471.2ns | 4162321.5ns | 100.3% | HIGH |
| kernel_nway | 4141536.7ns | 4135415.0ns | 100.1% | HIGH |
| kernel_seq | 4221328.0ns | 4199802.7ns | 100.5% | HIGH |
| nway | 4021662.8ns | 4001241.5ns | 100.5% | HIGH |
| sequential | 3738395.1ns | 3731130.0ns | 100.2% | HIGH |
| std_parallel | 5089734.5ns | 5243223.8ns | 97.1% | HIGH |

## Distribution (algo ns)

```
head_tail (n=6, range 3555984.4-4215741.2 ns)
  3555984.4 |####################
  3588972.2 |
  3621960.1 |
  3654947.9 |
  3687935.8 |
  3720923.6 |
  3753911.5 |
  3786899.3 |
  3819887.1 |
  3852875.0 |
  3885862.8 |
  3918850.7 |####################
  3951838.5 |
  3984826.4 |
  4017814.2 |
  4050802.0 |
  4083789.9 |####################
  4116777.7 |
  4149765.6 |
  4182753.4 |########################################
  (0 below, 1 above range)

head_tail_dyn (n=6, range 4109342.6-4189449.8 ns)
  4109342.6 |####################
  4113348.0 |
  4117353.3 |
  4121358.7 |
  4125364.0 |####################
  4129369.4 |
  4133374.7 |
  4137380.1 |
  4141385.5 |
  4145390.8 |
  4149396.2 |
  4153401.5 |
  4157406.9 |
  4161412.2 |
  4165417.6 |
  4169423.0 |
  4173428.3 |
  4177433.7 |########################################
  4181439.0 |####################
  4185444.4 |
  (0 below, 1 above range)

kernel_nway (n=6, range 4106043.0-4172026.2 ns)
  4106043.0 |########################################
  4109342.2 |########################################
  4112641.3 |
  4115940.5 |########################################
  4119239.6 |
  4122538.8 |
  4125838.0 |
  4129137.1 |
  4132436.3 |########################################
  4135735.5 |
  4139034.6 |
  4142333.8 |########################################
  4145633.0 |
  4148932.1 |
  4152231.3 |
  4155530.4 |
  4158829.6 |
  4162128.8 |
  4165427.9 |
  4168727.1 |
  (0 below, 1 above range)

kernel_seq (n=6, range 4092423.3-4291886.9 ns)
  4092423.3 |####################
  4102396.5 |
  4112369.7 |
  4122342.8 |
  4132316.0 |
  4142289.2 |
  4152262.4 |
  4162235.6 |########################################
  4172208.8 |
  4182181.9 |
  4192155.1 |####################
  4202128.3 |
  4212101.5 |####################
  4222074.7 |
  4232047.9 |
  4242021.0 |
  4251994.2 |
  4261967.4 |
  4271940.6 |
  4281913.8 |
  (0 below, 1 above range)

nway (n=6, range 3375278.4-4217673.9 ns)
  3375278.4 |####################
  3417398.2 |
  3459518.0 |
  3501637.7 |
  3543757.5 |
  3585877.3 |
  3627997.0 |
  3670116.8 |
  3712236.6 |
  3754356.4 |
  3796476.2 |
  3838595.9 |####################
  3880715.7 |
  3922835.5 |
  3964955.2 |
  4007075.0 |
  4049194.8 |
  4091314.6 |####################
  4133434.4 |
  4175554.1 |########################################
  (0 below, 1 above range)

sequential (n=6, range 3500132.2-4173592.0 ns)
  3500132.2 |########################################
  3533805.2 |
  3567478.2 |
  3601151.2 |
  3634824.2 |
  3668497.1 |
  3702170.1 |
  3735843.1 |
  3769516.1 |
  3803189.1 |
  3836862.1 |
  3870535.1 |
  3904208.1 |
  3937881.0 |
  3971554.0 |
  4005227.0 |
  4038900.0 |
  4072573.0 |
  4106246.0 |
  4139919.0 |##########
  (0 below, 1 above range)

std_parallel (n=6, range 4504126.8-6184627.9 ns)
  4504126.8 |####################
  4588151.9 |
  4672176.9 |
  4756202.0 |####################
  4840227.0 |########################################
  4924252.1 |
  5008277.1 |
  5092302.2 |####################
  5176327.2 |
  5260352.3 |
  5344377.3 |
  5428402.4 |
  5512427.5 |
  5596452.5 |
  5680477.6 |
  5764502.6 |
  5848527.7 |
  5932552.7 |
  6016577.8 |
  6100602.8 |
  (0 below, 1 above range)

```

## Diagnostics

- **head_tail**: bridge=100.1% of algo (FFI overhead may distort results)
- **head_tail_dyn**: bridge=100.2% of algo (FFI overhead may distort results)
- **kernel_nway**: bridge=100.1% of algo (FFI overhead may distort results)
- **kernel_seq**: bridge=100.8% of algo (FFI overhead may distort results)
- **nway**: bridge=100.2% of algo (FFI overhead may distort results)
- **sequential**: bridge=100.3% of algo (FFI overhead may distort results)
- **std_parallel**: bridge=101.1% of algo (FFI overhead may distort results)
