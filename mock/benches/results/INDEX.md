# Bench Results Index

Generated from the runs under `results/`, one directory per run at `<bench>/<cell>/<machine class>/<run>/`, and regenerated from them on every run. Each run holds its samples, meta, report and summary.

## atomic_inc_strategy/atomic_inc_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T094555Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 7.51 us | 872.86x |
| inc_fetch_add | 4.39 us | 510.09x |
| inc_load_store | 9 ns | 1.00x |

- [20261115T020533Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T020533Z-266650fb-dirty/report.md)
- [20261117T094555Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T094555Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205811Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 2.54 us | 6345.25x |
| inc_fetch_add | 2.21 us | 5527.00x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015102Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T015102Z-88d91ab/report.md)
- [20261114T205811Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T205811Z-266650fb-dirty/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T094555Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 119.86 us | 11307.65x |
| inc_fetch_add | 67.24 us | 6343.64x |
| inc_load_store | 11 ns | 1.00x |

- [20261115T021047Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T021047Z-266650fb-dirty/report.md)
- [20261117T094555Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T094555Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T205820Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inc_load_store | 0 ns | 1.00x |

- [20260511T015108Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T015108Z-88d91ab/report.md)
- [20261114T205819Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T205819Z-266650fb-dirty/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T094555Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 1.88 us | 221.13x |
| inc_fetch_add | 1.30 us | 152.93x |
| inc_load_store | 8 ns | 1.00x |

- [20261115T020258Z-266650fb](atomic_inc_strategy/atomic_inc_strategy_n256/linux-x86_64-c16-bare-intel/20261115T020258Z-266650fb/report.md)
- [20261117T094555Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n256/linux-x86_64-c16-bare-intel/20261117T094555Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205809Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 687 ns | 1.00x |
| inc_fetch_add | 535 ns | 1.00x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015101Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20260511T015101Z-88d91ab/report.md)
- [20261114T205809Z-266650fb](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20261114T205809Z-266650fb/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T094555Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 30.57 us | 3513.45x |
| inc_fetch_add | 16.82 us | 1933.61x |
| inc_load_store | 9 ns | 1.00x |

- [20261115T020807Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T020807Z-266650fb-dirty/report.md)
- [20261117T094555Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T094555Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205814Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 9.77 us | 1.00x |
| inc_fetch_add | 8.48 us | 1.00x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015104Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T015104Z-88d91ab/report.md)
- [20261114T205814Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T205814Z-266650fb-dirty/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T095653Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 896 ns | 1.34x |
| atomic_relaxed | 673 ns | 1.01x |
| atomic_seqcst | 670 ns | 1.00x |

- [20261117T095653Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n1024/linux-x86_64-c16-bare-intel/20261117T095653Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214037Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n1024/macos-aarch64-c8-bare-apple/20260510T214037Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T095653Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 14.27 us | 1.67x |
| atomic_relaxed | 8.55 us | 1.00x |
| atomic_seqcst | 8.54 us | 1.00x |

- [20261117T095653Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n16384/linux-x86_64-c16-bare-intel/20261117T095653Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214043Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n16384/macos-aarch64-c8-bare-apple/20260510T214043Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T095653Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 225 ns | 1.31x |
| atomic_relaxed | 172 ns | 1.00x |
| atomic_seqcst | 172 ns | 1.00x |

- [20261117T095653Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n256/linux-x86_64-c16-bare-intel/20261117T095653Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214035Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n256/macos-aarch64-c8-bare-apple/20260510T214035Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T095653Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 3.58 us | 1.65x |
| atomic_relaxed | 2.30 us | 1.06x |
| atomic_seqcst | 2.16 us | 1.00x |

- [20261117T095653Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n4096/linux-x86_64-c16-bare-intel/20261117T095653Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214039Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n4096/macos-aarch64-c8-bare-apple/20260510T214039Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T095653Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 76 ns | 1.47x |
| atomic_relaxed | 52 ns | 1.00x |
| atomic_seqcst | 52 ns | 1.00x |

- [20261117T095653Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n64/linux-x86_64-c16-bare-intel/20261117T095653Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214034Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n64/macos-aarch64-c8-bare-apple/20260510T214034Z-f6ee560/report.md)

## atomic_vs_plain/atomic_vs_plain_n1024

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261117T101015Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 939 ns | 1.00x |
| acc_atomic_seqcst | 939 ns | 1.00x |
| acc_plain | 939 ns | 1.00x |

- [20261115T021655Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/linux-x86_64-c16-bare-intel/20261115T021655Z-266650fb-dirty/report.md)
- [20261117T101015Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n1024/linux-x86_64-c16-bare-intel/20261117T101015Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205932Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 1.31 us | 1.01x |
| acc_atomic_seqcst | 1.31 us | 1.01x |
| acc_plain | 1.30 us | 1.00x |

- [20260511T013000Z-a410443](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20260511T013000Z-a410443/report.md)
- [20261114T205932Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20261114T205932Z-266650fb-dirty/report.md)

## atomic_vs_plain/atomic_vs_plain_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T101015Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 18.26 us | 1.00x |
| acc_atomic_seqcst | 18.27 us | 1.00x |
| acc_plain | 18.27 us | 1.00x |

- [20261115T022201Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n16384/linux-x86_64-c16-bare-intel/20261115T022201Z-266650fb-dirty/report.md)
- [20261117T101015Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n16384/linux-x86_64-c16-bare-intel/20261117T101015Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205940Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 20.46 us | 1.00x |
| acc_atomic_seqcst | 20.50 us | 1.00x |
| acc_plain | 20.48 us | 1.00x |

- [20260511T013005Z-a410443](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20260511T013005Z-a410443/report.md)
- [20261114T205940Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20261114T205940Z-266650fb-dirty/report.md)

## atomic_vs_plain/atomic_vs_plain_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T101015Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 250 ns | 1.00x |
| acc_atomic_seqcst | 408 ns | 1.63x |
| acc_plain | 250 ns | 1.00x |

- [20261115T021420Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n256/linux-x86_64-c16-bare-intel/20261115T021420Z-266650fb-dirty/report.md)
- [20261117T101015Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n256/linux-x86_64-c16-bare-intel/20261117T101015Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205930Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 308 ns | 1.13x |
| acc_atomic_seqcst | 272 ns | 1.00x |
| acc_plain | 305 ns | 1.12x |

- [20260511T012959Z-a410443](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20260511T012959Z-a410443/report.md)
- [20261114T205930Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20261114T205930Z-266650fb-dirty/report.md)

## atomic_vs_plain/atomic_vs_plain_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T101015Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 3.75 us | 1.00x |
| acc_atomic_seqcst | 4.57 us | 1.22x |
| acc_plain | 4.57 us | 1.22x |

- [20261115T021927Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n4096/linux-x86_64-c16-bare-intel/20261115T021927Z-266650fb-dirty/report.md)
- [20261117T101015Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n4096/linux-x86_64-c16-bare-intel/20261117T101015Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T205935Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 5.28 us | 1.00x |
| acc_atomic_seqcst | 5.38 us | 1.02x |
| acc_plain | 5.42 us | 1.03x |

- [20260511T013001Z-a410443](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20260511T013001Z-a410443/report.md)
- [20261114T205935Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20261114T205935Z-266650fb-dirty/report.md)

## bit_count_ops/bit_count_ops_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T102106Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 52 ns | 1.00x |
| ctz_intrinsic | 52 ns | 1.01x |
| ctz_loop | 112 ns | 2.18x |

- [20261115T022738Z-266650fb-dirty](bit_count_ops/bit_count_ops_n1024/linux-x86_64-c16-bare-intel/20261115T022738Z-266650fb-dirty/report.md)
- [20261117T102106Z-07a9e9a3](bit_count_ops/bit_count_ops_n1024/linux-x86_64-c16-bare-intel/20261117T102106Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210018Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 64 ns | 1.00x |
| ctz_intrinsic | 65 ns | 1.03x |
| ctz_loop | 158 ns | 2.47x |

- [20260511T021038Z-315383f](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20260511T021038Z-315383f/report.md)
- [20261114T210018Z-266650fb-dirty](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20261114T210018Z-266650fb-dirty/report.md)

## bit_count_ops/bit_count_ops_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T102106Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 616 ns | 1.18x |
| ctz_intrinsic | 524 ns | 1.00x |
| ctz_loop | 2.51 us | 4.79x |

- [20261115T023242Z-266650fb-dirty](bit_count_ops/bit_count_ops_n16384/linux-x86_64-c16-bare-intel/20261115T023242Z-266650fb-dirty/report.md)
- [20261117T102106Z-07a9e9a3](bit_count_ops/bit_count_ops_n16384/linux-x86_64-c16-bare-intel/20261117T102106Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210022Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 512 ns | 1.02x |
| ctz_intrinsic | 500 ns | 1.00x |
| ctz_loop | 2.14 us | 4.29x |

- [20260511T021040Z-315383f](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20260511T021040Z-315383f/report.md)
- [20261114T210022Z-266650fb-dirty](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20261114T210022Z-266650fb-dirty/report.md)

## bit_count_ops/bit_count_ops_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T102106Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 20 ns | 1.00x |
| ctz_intrinsic | 20 ns | 1.00x |
| ctz_loop | 38 ns | 1.89x |

- [20261115T022503Z-266650fb-dirty](bit_count_ops/bit_count_ops_n256/linux-x86_64-c16-bare-intel/20261115T022503Z-266650fb-dirty/report.md)
- [20261117T102106Z-07a9e9a3](bit_count_ops/bit_count_ops_n256/linux-x86_64-c16-bare-intel/20261117T102106Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210015Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 20 ns | 1.00x |
| ctz_intrinsic | 25 ns | 1.22x |
| ctz_loop | 55 ns | 2.69x |

- [20260511T021037Z-315383f](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20260511T021037Z-315383f/report.md)
- [20261114T210015Z-266650fb-dirty](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20261114T210015Z-266650fb-dirty/report.md)

## bit_count_ops/bit_count_ops_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T102106Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 177 ns | 1.00x |
| ctz_intrinsic | 179 ns | 1.01x |
| ctz_loop | 668 ns | 3.77x |

- [20261115T023010Z-266650fb-dirty](bit_count_ops/bit_count_ops_n4096/linux-x86_64-c16-bare-intel/20261115T023010Z-266650fb-dirty/report.md)
- [20261117T102106Z-07a9e9a3](bit_count_ops/bit_count_ops_n4096/linux-x86_64-c16-bare-intel/20261117T102106Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210020Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 169 ns | 1.06x |
| ctz_intrinsic | 159 ns | 1.00x |
| ctz_loop | 542 ns | 3.42x |

- [20260511T021039Z-315383f](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20260511T021039Z-315383f/report.md)
- [20261114T210020Z-266650fb-dirty](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20261114T210020Z-266650fb-dirty/report.md)

## bounds_check/bounds_check_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T103154Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 152 ns | 1.00x |
| bc_iter | 152 ns | 1.00x |
| bc_unchecked | 152 ns | 1.00x |

- [20261115T023819Z-266650fb-dirty](bounds_check/bounds_check_n1024/linux-x86_64-c16-bare-intel/20261115T023819Z-266650fb-dirty/report.md)
- [20261117T103154Z-07a9e9a3](bounds_check/bounds_check_n1024/linux-x86_64-c16-bare-intel/20261117T103154Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210100Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 40 ns | 1.04x |
| bc_iter | 38 ns | 1.00x |
| bc_unchecked | 41 ns | 1.07x |

- [20260511T005008Z-f10ba65](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20260511T005008Z-f10ba65/report.md)
- [20261114T210100Z-266650fb-dirty](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20261114T210100Z-266650fb-dirty/report.md)

## bounds_check/bounds_check_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T103154Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 2.29 us | 1.22x |
| bc_iter | 1.88 us | 1.00x |
| bc_unchecked | 2.29 us | 1.22x |

- [20261115T024323Z-266650fb-dirty](bounds_check/bounds_check_n16384/linux-x86_64-c16-bare-intel/20261115T024323Z-266650fb-dirty/report.md)
- [20261117T103154Z-07a9e9a3](bounds_check/bounds_check_n16384/linux-x86_64-c16-bare-intel/20261117T103154Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210104Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 379 ns | 1.01x |
| bc_iter | 375 ns | 1.00x |
| bc_unchecked | 673 ns | 1.79x |

- [20260511T005013Z-f10ba65](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20260511T005013Z-f10ba65/report.md)
- [20261114T210104Z-266650fb-dirty](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20261114T210104Z-266650fb-dirty/report.md)

## bounds_check/bounds_check_n256

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261117T103154Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 46 ns | 1.00x |
| bc_iter | 46 ns | 1.01x |
| bc_unchecked | 45 ns | 1.00x |

- [20261115T023544Z-266650fb-dirty](bounds_check/bounds_check_n256/linux-x86_64-c16-bare-intel/20261115T023544Z-266650fb-dirty/report.md)
- [20261117T103154Z-07a9e9a3](bounds_check/bounds_check_n256/linux-x86_64-c16-bare-intel/20261117T103154Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210058Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 5 ns | 1.00x |
| bc_iter | 5 ns | 1.08x |
| bc_unchecked | 6 ns | 1.16x |

- [20260511T005007Z-f10ba65](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20260511T005007Z-f10ba65/report.md)
- [20261114T210058Z-266650fb-dirty](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20261114T210058Z-266650fb-dirty/report.md)

## bounds_check/bounds_check_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T103154Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 580 ns | 1.00x |
| bc_iter | 580 ns | 1.00x |
| bc_unchecked | 580 ns | 1.00x |

- [20261115T024051Z-266650fb-dirty](bounds_check/bounds_check_n4096/linux-x86_64-c16-bare-intel/20261115T024051Z-266650fb-dirty/report.md)
- [20261117T103154Z-07a9e9a3](bounds_check/bounds_check_n4096/linux-x86_64-c16-bare-intel/20261117T103154Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210102Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 103 ns | 1.00x |
| bc_iter | 104 ns | 1.01x |
| bc_unchecked | 150 ns | 1.45x |

- [20260511T005009Z-f10ba65](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20260511T005009Z-f10ba65/report.md)
- [20261114T210102Z-266650fb-dirty](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20261114T210102Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T104240Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.41 us | 1.97x |
| branch_if | 715 ns | 1.00x |

- [20261115T024902Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/linux-x86_64-c16-bare-intel/20261115T024902Z-266650fb-dirty/report.md)
- [20261117T104240Z-07a9e9a3](branch_pattern/branch_pattern_n1024/linux-x86_64-c16-bare-intel/20261117T104240Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210137Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.72 us | 1.00x |
| branch_if | 1.72 us | 1.00x |

- [20260510T213318Z-b042648](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20260510T213318Z-b042648/report.md)
- [20261114T210137Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20261114T210137Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T104240Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 22.47 us | 1.00x |
| branch_if | 25.58 us | 1.14x |

- [20261115T025232Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/linux-x86_64-c16-bare-intel/20261115T025232Z-266650fb-dirty/report.md)
- [20261117T104240Z-07a9e9a3](branch_pattern/branch_pattern_n16384/linux-x86_64-c16-bare-intel/20261117T104240Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210143Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 26.15 us | 1.00x |
| branch_if | 26.09 us | 1.00x |

- [20260510T213324Z-b042648](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20260510T213324Z-b042648/report.md)
- [20261114T210143Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20261114T210143Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T104240Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 425 ns | 1.93x |
| branch_if | 221 ns | 1.00x |

- [20261115T024719Z-266650fb-dirty](branch_pattern/branch_pattern_n256/linux-x86_64-c16-bare-intel/20261115T024719Z-266650fb-dirty/report.md)
- [20261117T104240Z-07a9e9a3](branch_pattern/branch_pattern_n256/linux-x86_64-c16-bare-intel/20261117T104240Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210136Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 445 ns | 1.12x |
| branch_if | 396 ns | 1.00x |

- [20260510T213317Z-b042648](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20260510T213317Z-b042648/report.md)
- [20261114T210136Z-266650fb-dirty](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20261114T210136Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T104240Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.91 us | 2.02x |
| branch_if | 3.42 us | 1.00x |

- [20261115T025046Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/linux-x86_64-c16-bare-intel/20261115T025046Z-266650fb-dirty/report.md)
- [20261117T104240Z-07a9e9a3](branch_pattern/branch_pattern_n4096/linux-x86_64-c16-bare-intel/20261117T104240Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210139Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.72 us | 1.00x |
| branch_if | 6.72 us | 1.00x |

- [20260510T213320Z-b042648](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20260510T213320Z-b042648/report.md)
- [20261114T210139Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20261114T210139Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T104240Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 112 ns | 1.79x |
| branch_if | 63 ns | 1.00x |

- [20261115T024533Z-266650fb-dirty](branch_pattern/branch_pattern_n64/linux-x86_64-c16-bare-intel/20261115T024533Z-266650fb-dirty/report.md)
- [20261117T104240Z-07a9e9a3](branch_pattern/branch_pattern_n64/linux-x86_64-c16-bare-intel/20261117T104240Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210134Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 93 ns | 1.03x |
| branch_if | 90 ns | 1.00x |

- [20260510T213316Z-b042648](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20260510T213316Z-b042648/report.md)
- [20261114T210134Z-266650fb-dirty](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20261114T210134Z-266650fb-dirty/report.md)

## branch_predictability/branch_predictability_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T105203Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 1.15 us | 1.21x |
| bp_skew_high | 1.15 us | 1.21x |
| bp_skew_low | 948 ns | 1.00x |

- [20261115T025809Z-266650fb-dirty](branch_predictability/branch_predictability_n1024/linux-x86_64-c16-bare-intel/20261115T025809Z-266650fb-dirty/report.md)
- [20261117T105203Z-07a9e9a3](branch_predictability/branch_predictability_n1024/linux-x86_64-c16-bare-intel/20261117T105203Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210220Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 1.27 us | 1.00x |
| bp_skew_high | 1.34 us | 1.06x |
| bp_skew_low | 1.34 us | 1.06x |

- [20260511T011011Z-c61ad63](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20260511T011011Z-c61ad63/report.md)
- [20261114T210220Z-266650fb-dirty](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20261114T210220Z-266650fb-dirty/report.md)

## branch_predictability/branch_predictability_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T105203Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 18.27 us | 1.01x |
| bp_skew_high | 18.04 us | 1.00x |
| bp_skew_low | 18.27 us | 1.01x |

- [20261115T030315Z-266650fb-dirty](branch_predictability/branch_predictability_n16384/linux-x86_64-c16-bare-intel/20261115T030315Z-266650fb-dirty/report.md)
- [20261117T105203Z-07a9e9a3](branch_predictability/branch_predictability_n16384/linux-x86_64-c16-bare-intel/20261117T105203Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210227Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 20.53 us | 1.00x |
| bp_skew_high | 20.55 us | 1.00x |
| bp_skew_low | 20.54 us | 1.00x |

- [20260511T011016Z-c61ad63](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20260511T011016Z-c61ad63/report.md)
- [20261114T210227Z-266650fb-dirty](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20261114T210227Z-266650fb-dirty/report.md)

## branch_predictability/branch_predictability_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T105203Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 293 ns | 1.00x |
| bp_skew_high | 293 ns | 1.00x |
| bp_skew_low | 292 ns | 1.00x |

- [20261115T025534Z-266650fb-dirty](branch_predictability/branch_predictability_n256/linux-x86_64-c16-bare-intel/20261115T025534Z-266650fb-dirty/report.md)
- [20261117T105203Z-07a9e9a3](branch_predictability/branch_predictability_n256/linux-x86_64-c16-bare-intel/20261117T105203Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210218Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 333 ns | 1.00x |
| bp_skew_high | 332 ns | 1.00x |
| bp_skew_low | 332 ns | 1.00x |

- [20260511T011010Z-c61ad63](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20260511T011010Z-c61ad63/report.md)
- [20261114T210218Z-266650fb-dirty](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20261114T210218Z-266650fb-dirty/report.md)

## branch_predictability/branch_predictability_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T105203Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 4.57 us | 1.00x |
| bp_skew_high | 4.57 us | 1.00x |
| bp_skew_low | 4.57 us | 1.00x |

- [20261115T030041Z-266650fb-dirty](branch_predictability/branch_predictability_n4096/linux-x86_64-c16-bare-intel/20261115T030041Z-266650fb-dirty/report.md)
- [20261117T105203Z-07a9e9a3](branch_predictability/branch_predictability_n4096/linux-x86_64-c16-bare-intel/20261117T105203Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210223Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 5.47 us | 1.03x |
| bp_skew_high | 5.30 us | 1.00x |
| bp_skew_low | 5.42 us | 1.02x |

- [20260511T011012Z-c61ad63](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20260511T011012Z-c61ad63/report.md)
- [20261114T210223Z-266650fb-dirty](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20261114T210223Z-266650fb-dirty/report.md)

## build-static/clean_n25

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 10836.29 ms | 1.04x |
| llvm_non_incremental | 10393.76 ms | 1.00x |

- [20261118T031806Z-07a9e9a3](build-static/clean_n25/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 16864.64 ms | 1.08x |
| llvm_non_incremental | 15672.26 ms | 1.00x |

- [20261116T063249Z-f04daa0a-dirty](build-static/clean_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/clean_n97

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 71875.08 ms | 1.06x |
| llvm_non_incremental | 68073.80 ms | 1.00x |

- [20261118T031806Z-07a9e9a3](build-static/clean_n97/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

## build-static/leaf_n25

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 2372.08 ms | 1.00x |
| llvm_non_incremental | 3669.55 ms | 1.55x |

- [20261118T031806Z-07a9e9a3](build-static/leaf_n25/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 3379.88 ms | 1.00x |
| llvm_non_incremental | 5451.34 ms | 1.61x |

- [20261116T063249Z-f04daa0a-dirty](build-static/leaf_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/leaf_n97

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 2639.11 ms | 1.00x |
| llvm_non_incremental | 3920.12 ms | 1.49x |

- [20261118T031806Z-07a9e9a3](build-static/leaf_n97/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

## build-static/mid_n25

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 376.53 ms | 1.00x |
| llvm_non_incremental | 4863.22 ms | 12.92x |

- [20261118T031806Z-07a9e9a3](build-static/mid_n25/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 693.37 ms | 1.00x |
| llvm_non_incremental | 5582.15 ms | 8.05x |

- [20261116T063249Z-f04daa0a-dirty](build-static/mid_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/mid_n97

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 1564.03 ms | 1.00x |
| llvm_non_incremental | 19538.46 ms | 12.49x |

- [20261118T031806Z-07a9e9a3](build-static/mid_n97/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

## build-static/root_n25

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 666.89 ms | 1.00x |
| llvm_non_incremental | 8761.35 ms | 13.14x |

- [20261118T031806Z-07a9e9a3](build-static/root_n25/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 1119.72 ms | 1.00x |
| llvm_non_incremental | 14310.33 ms | 12.78x |

- [20261116T063249Z-f04daa0a-dirty](build-static/root_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/root_n97

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T031806Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 3976.99 ms | 1.00x |
| llvm_non_incremental | 49961.76 ms | 12.56x |

- [20261118T031806Z-07a9e9a3](build-static/root_n97/linux-x86_64-c16-bare-intel/20261118T031806Z-07a9e9a3/report.md)

## cache_layout/cache_layout_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T110252Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 28 ns | 1.00x |
| layout_soa | 28 ns | 1.00x |

- [20261115T030857Z-266650fb-dirty](cache_layout/cache_layout_n1024/linux-x86_64-c16-bare-intel/20261115T030857Z-266650fb-dirty/report.md)
- [20261117T110252Z-07a9e9a3](cache_layout/cache_layout_n1024/linux-x86_64-c16-bare-intel/20261117T110252Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210300Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 13 ns | 1.03x |
| layout_soa | 12 ns | 1.00x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210300Z-266650fb-dirty](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20261114T210300Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T110252Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 333 ns | 1.00x |
| layout_soa | 334 ns | 1.00x |

- [20261115T031223Z-266650fb-dirty](cache_layout/cache_layout_n16384/linux-x86_64-c16-bare-intel/20261115T031223Z-266650fb-dirty/report.md)
- [20261117T110252Z-07a9e9a3](cache_layout/cache_layout_n16384/linux-x86_64-c16-bare-intel/20261117T110252Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210303Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 371 ns | 1.00x |
| layout_soa | 386 ns | 1.04x |

- [20260510T214108Z-f6ee560](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20260510T214108Z-f6ee560/report.md)
- [20261114T210303Z-266650fb-dirty](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20261114T210303Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T110252Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 12 ns | 1.01x |
| layout_soa | 12 ns | 1.00x |

- [20261115T030714Z-266650fb-dirty](cache_layout/cache_layout_n256/linux-x86_64-c16-bare-intel/20261115T030714Z-266650fb-dirty/report.md)
- [20261117T110252Z-07a9e9a3](cache_layout/cache_layout_n256/linux-x86_64-c16-bare-intel/20261117T110252Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210259Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 1 ns | 1.50x |
| layout_soa | 1 ns | 1.00x |

- [20260510T214105Z-f6ee560](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20260510T214105Z-f6ee560/report.md)
- [20261114T210259Z-266650fb-dirty](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20261114T210259Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T110252Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 90 ns | 1.01x |
| layout_soa | 89 ns | 1.00x |

- [20261115T031040Z-266650fb-dirty](cache_layout/cache_layout_n4096/linux-x86_64-c16-bare-intel/20261115T031040Z-266650fb-dirty/report.md)
- [20261117T110252Z-07a9e9a3](cache_layout/cache_layout_n4096/linux-x86_64-c16-bare-intel/20261117T110252Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210301Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 65 ns | 1.00x |
| layout_soa | 69 ns | 1.06x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210301Z-266650fb-dirty](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20261114T210301Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T110252Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 10 ns | 1.01x |
| layout_soa | 10 ns | 1.00x |

- [20261115T030527Z-266650fb-dirty](cache_layout/cache_layout_n64/linux-x86_64-c16-bare-intel/20261115T030527Z-266650fb-dirty/report.md)
- [20261117T110252Z-07a9e9a3](cache_layout/cache_layout_n64/linux-x86_64-c16-bare-intel/20261117T110252Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210257Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 0 ns | 1.00x |
| layout_soa | 0 ns | 1.00x |

- [20260510T214104Z-f6ee560](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20260510T214104Z-f6ee560/report.md)
- [20261114T210257Z-266650fb-dirty](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20261114T210257Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n128

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T111209Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 612 ns | 1.00x |
| dep_graph_dense | 23.43 us | 38.31x |
| dep_graph_hybrid | 724 ns | 1.18x |

- [20261115T032537Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n128/linux-x86_64-c16-bare-intel/20261115T032537Z-266650fb-dirty/report.md)
- [20261117T111209Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n128/linux-x86_64-c16-bare-intel/20261117T111209Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210349Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 806 ns | 1.00x |
| dep_graph_dense | 26.53 us | 32.92x |
| dep_graph_hybrid | 808 ns | 1.00x |

- [20260510T212550Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20260510T212550Z-771392d/report.md)
- [20261114T210349Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20261114T210349Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n16

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T111209Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 89 ns | 1.00x |
| dep_graph_dense | 439 ns | 4.94x |
| dep_graph_hybrid | 447 ns | 5.02x |

- [20261115T031800Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n16/linux-x86_64-c16-bare-intel/20261115T031800Z-266650fb-dirty/report.md)
- [20261117T111209Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n16/linux-x86_64-c16-bare-intel/20261117T111209Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210341Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 54 ns | 1.00x |
| dep_graph_dense | 904 ns | 16.81x |
| dep_graph_hybrid | 452 ns | 8.39x |

- [20260510T212544Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20260510T212544Z-771392d/report.md)
- [20261114T210341Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20261114T210341Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T111209Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 1.49 us | 1.02x |
| dep_graph_dense | 87.96 us | 60.24x |
| dep_graph_hybrid | 1.46 us | 1.00x |

- [20261115T032812Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n256/linux-x86_64-c16-bare-intel/20261115T032812Z-266650fb-dirty/report.md)
- [20261117T111209Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n256/linux-x86_64-c16-bare-intel/20261117T111209Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210355Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 1.66 us | 1.05x |
| dep_graph_dense | 104.06 us | 65.87x |
| dep_graph_hybrid | 1.58 us | 1.00x |

- [20260510T212557Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20260510T212557Z-771392d/report.md)
- [20261114T210355Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20261114T210355Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n32

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T111209Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 193 ns | 1.00x |
| dep_graph_dense | 1.61 us | 8.30x |
| dep_graph_hybrid | 202 ns | 1.05x |

- [20261115T032032Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n32/linux-x86_64-c16-bare-intel/20261115T032032Z-266650fb-dirty/report.md)
- [20261117T111209Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n32/linux-x86_64-c16-bare-intel/20261117T111209Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210343Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 158 ns | 1.00x |
| dep_graph_dense | 2.26 us | 14.25x |
| dep_graph_hybrid | 165 ns | 1.04x |

- [20260510T212545Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20260510T212545Z-771392d/report.md)
- [20261114T210343Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20261114T210343Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T111209Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 376 ns | 1.00x |
| dep_graph_dense | 6.03 us | 16.01x |
| dep_graph_hybrid | 404 ns | 1.07x |

- [20261115T032304Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n64/linux-x86_64-c16-bare-intel/20261115T032304Z-266650fb-dirty/report.md)
- [20261117T111209Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n64/linux-x86_64-c16-bare-intel/20261117T111209Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210346Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 382 ns | 1.03x |
| dep_graph_dense | 7.49 us | 20.16x |
| dep_graph_hybrid | 372 ns | 1.00x |

- [20260510T212547Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20260510T212547Z-771392d/report.md)
- [20261114T210346Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20261114T210346Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n8

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T111209Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 43 ns | 1.00x |
| dep_graph_dense | 122 ns | 2.84x |
| dep_graph_hybrid | 124 ns | 2.88x |

- [20261115T031525Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n8/linux-x86_64-c16-bare-intel/20261115T031525Z-266650fb-dirty/report.md)
- [20261117T111209Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n8/linux-x86_64-c16-bare-intel/20261117T111209Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210339Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 27 ns | 1.00x |
| dep_graph_dense | 581 ns | 21.42x |
| dep_graph_hybrid | 110 ns | 4.04x |

- [20260510T212543Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20260510T212543Z-771392d/report.md)
- [20261114T210339Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20261114T210339Z-266650fb-dirty/report.md)

## dispatch_dynamic/dispatch_dynamic_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T112807Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 150 ns | 1.00x |
| dispatch_runtime | 181 ns | 1.21x |
| dispatch_volatile | 182 ns | 1.21x |

- [20261115T033621Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n1024/linux-x86_64-c16-bare-intel/20261115T033621Z-266650fb-dirty/report.md)
- [20261117T112807Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n1024/linux-x86_64-c16-bare-intel/20261117T112807Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210435Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 124 ns | 1.00x |
| dispatch_runtime | 319 ns | 2.58x |
| dispatch_volatile | 166 ns | 1.34x |

- [20260510T204023Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20260510T204023Z-3c0d0d5/report.md)
- [20261114T210435Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20261114T210435Z-266650fb-dirty/report.md)

## dispatch_dynamic/dispatch_dynamic_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T112807Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2.29 us | 1.00x |
| dispatch_runtime | 11.45 us | 5.00x |
| dispatch_volatile | 2.85 us | 1.25x |

- [20261115T034125Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n16384/linux-x86_64-c16-bare-intel/20261115T034125Z-266650fb-dirty/report.md)
- [20261117T112807Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n16384/linux-x86_64-c16-bare-intel/20261117T112807Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210440Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2.60 us | 1.00x |
| dispatch_runtime | 9.35 us | 3.60x |
| dispatch_volatile | 2.65 us | 1.02x |

- [20260510T204026Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20260510T204026Z-3c0d0d5/report.md)
- [20261114T210440Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20261114T210440Z-266650fb-dirty/report.md)

## dispatch_dynamic/dispatch_dynamic_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T112807Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 43 ns | 1.05x |
| dispatch_runtime | 41 ns | 1.00x |
| dispatch_volatile | 47 ns | 1.15x |

- [20261115T033349Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n256/linux-x86_64-c16-bare-intel/20261115T033349Z-266650fb-dirty/report.md)
- [20261117T112807Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n256/linux-x86_64-c16-bare-intel/20261117T112807Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210433Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 15 ns | 1.00x |
| dispatch_runtime | 47 ns | 3.24x |
| dispatch_volatile | 47 ns | 3.23x |

- [20260510T204022Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20260510T204022Z-3c0d0d5/report.md)
- [20261114T210433Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20261114T210433Z-266650fb-dirty/report.md)

## dispatch_dynamic/dispatch_dynamic_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T112807Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 578 ns | 1.00x |
| dispatch_runtime | 743 ns | 1.29x |
| dispatch_volatile | 717 ns | 1.24x |

- [20261115T033853Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n4096/linux-x86_64-c16-bare-intel/20261115T033853Z-266650fb-dirty/report.md)
- [20261117T112807Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n4096/linux-x86_64-c16-bare-intel/20261117T112807Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210437Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 636 ns | 1.00x |
| dispatch_runtime | 1.17 us | 1.83x |
| dispatch_volatile | 677 ns | 1.06x |

- [20260510T204024Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20260510T204024Z-3c0d0d5/report.md)
- [20261114T210437Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20261114T210437Z-266650fb-dirty/report.md)

## dispatch_dynamic/dispatch_dynamic_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T112807Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 16 ns | 1.05x |
| dispatch_runtime | 15 ns | 1.00x |
| dispatch_volatile | 16 ns | 1.06x |

- [20261115T033114Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n64/linux-x86_64-c16-bare-intel/20261115T033114Z-266650fb-dirty/report.md)
- [20261117T112807Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n64/linux-x86_64-c16-bare-intel/20261117T112807Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210430Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2 ns | 1.00x |
| dispatch_runtime | 10 ns | 4.57x |
| dispatch_volatile | 8 ns | 3.57x |

- [20260510T204021Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20260510T204021Z-3c0d0d5/report.md)
- [20261114T210430Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20261114T210430Z-266650fb-dirty/report.md)

## dispatch_static/dispatch_static_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T114128Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 150 ns | 1.00x |
| dispatch_direct | 150 ns | 1.00x |
| dispatch_trait | 150 ns | 1.00x |

- [20261115T034930Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/linux-x86_64-c16-bare-intel/20261115T034930Z-266650fb-dirty/report.md)
- [20261117T114128Z-07a9e9a3](dispatch_static/dispatch_static_n1024/linux-x86_64-c16-bare-intel/20261117T114128Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T210516Z-266650fb`

| variant | median | vs best |
|---|---|---|
| dispatch_trait | 123 ns | 1.00x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210515Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20261114T210515Z-266650fb-dirty/report.md)

## dispatch_static/dispatch_static_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T114128Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2.29 us | 1.22x |
| dispatch_direct | 1.88 us | 1.00x |
| dispatch_trait | 2.29 us | 1.22x |

- [20261115T035434Z-266650fb-dirty](dispatch_static/dispatch_static_n16384/linux-x86_64-c16-bare-intel/20261115T035434Z-266650fb-dirty/report.md)
- [20261117T114128Z-07a9e9a3](dispatch_static/dispatch_static_n16384/linux-x86_64-c16-bare-intel/20261117T114128Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210520Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2.57 us | 1.00x |
| dispatch_direct | 2.68 us | 1.04x |
| dispatch_trait | 2.61 us | 1.01x |

- [20260510T204037Z-3c0d0d5](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20260510T204037Z-3c0d0d5/report.md)
- [20261114T210520Z-266650fb-dirty](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20261114T210520Z-266650fb-dirty/report.md)

## dispatch_static/dispatch_static_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T114128Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 43 ns | 1.00x |
| dispatch_direct | 43 ns | 1.00x |
| dispatch_trait | 43 ns | 1.00x |

- [20261115T034658Z-266650fb-dirty](dispatch_static/dispatch_static_n256/linux-x86_64-c16-bare-intel/20261115T034658Z-266650fb-dirty/report.md)
- [20261117T114128Z-07a9e9a3](dispatch_static/dispatch_static_n256/linux-x86_64-c16-bare-intel/20261117T114128Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210513Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 15 ns | 1.08x |
| dispatch_direct | 14 ns | 1.01x |
| dispatch_trait | 14 ns | 1.00x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210513Z-266650fb-dirty](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20261114T210513Z-266650fb-dirty/report.md)

## dispatch_static/dispatch_static_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T114128Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 578 ns | 1.22x |
| dispatch_direct | 473 ns | 1.00x |
| dispatch_trait | 577 ns | 1.22x |

- [20261115T035202Z-266650fb-dirty](dispatch_static/dispatch_static_n4096/linux-x86_64-c16-bare-intel/20261115T035202Z-266650fb-dirty/report.md)
- [20261117T114128Z-07a9e9a3](dispatch_static/dispatch_static_n4096/linux-x86_64-c16-bare-intel/20261117T114128Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210517Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 630 ns | 1.00x |
| dispatch_direct | 648 ns | 1.03x |
| dispatch_trait | 633 ns | 1.00x |

- [20260510T204035Z-3c0d0d5](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20260510T204035Z-3c0d0d5/report.md)
- [20261114T210517Z-266650fb-dirty](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20261114T210517Z-266650fb-dirty/report.md)

## dispatch_static/dispatch_static_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T114128Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 16 ns | 1.00x |
| dispatch_direct | 16 ns | 1.01x |
| dispatch_trait | 16 ns | 1.01x |

- [20261115T034423Z-266650fb-dirty](dispatch_static/dispatch_static_n64/linux-x86_64-c16-bare-intel/20261115T034423Z-266650fb-dirty/report.md)
- [20261117T114128Z-07a9e9a3](dispatch_static/dispatch_static_n64/linux-x86_64-c16-bare-intel/20261117T114128Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210511Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2 ns | 1.00x |
| dispatch_direct | 2 ns | 1.00x |
| dispatch_trait | 2 ns | 1.00x |

- [20260510T204033Z-3c0d0d5](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20260510T204033Z-3c0d0d5/report.md)
- [20261114T210511Z-266650fb-dirty](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20261114T210511Z-266650fb-dirty/report.md)

## ema_axis_h/ema_axis_h_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T115451Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 196 ns | 1.66x |
| ema_neon | 197 ns | 1.67x |
| ema_scalar | 118 ns | 1.00x |

- [20261115T040242Z-266650fb-dirty](ema_axis_h/ema_axis_h_n1024/linux-x86_64-c16-bare-intel/20261115T040242Z-266650fb-dirty/report.md)
- [20261117T115451Z-07a9e9a3](ema_axis_h/ema_axis_h_n1024/linux-x86_64-c16-bare-intel/20261117T115451Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210600Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 392 ns | 8.65x |
| ema_neon | 45 ns | 1.00x |
| ema_scalar | 95 ns | 2.09x |

- [20260510T204029Z-3c0d0d5](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20260510T204029Z-3c0d0d5/report.md)
- [20261114T210600Z-266650fb-dirty](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20261114T210600Z-266650fb-dirty/report.md)

## ema_axis_h/ema_axis_h_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T115451Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 2.93 us | 1.66x |
| ema_neon | 2.93 us | 1.67x |
| ema_scalar | 1.76 us | 1.00x |

- [20261115T040747Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/linux-x86_64-c16-bare-intel/20261115T040747Z-266650fb-dirty/report.md)
- [20261117T115451Z-07a9e9a3](ema_axis_h/ema_axis_h_n16384/linux-x86_64-c16-bare-intel/20261117T115451Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T210606Z-266650fb`

| variant | median | vs best |
|---|---|---|
| ema_scalar | 1.69 us | 1.00x |

- [20260510T204031Z-3c0d0d5](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20260510T204031Z-3c0d0d5/report.md)
- [20261114T210605Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20261114T210605Z-266650fb-dirty/report.md)

## ema_axis_h/ema_axis_h_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T115451Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 59 ns | 1.97x |
| ema_neon | 59 ns | 1.98x |
| ema_scalar | 30 ns | 1.00x |

- [20261115T040011Z-266650fb-dirty](ema_axis_h/ema_axis_h_n256/linux-x86_64-c16-bare-intel/20261115T040011Z-266650fb-dirty/report.md)
- [20261117T115451Z-07a9e9a3](ema_axis_h/ema_axis_h_n256/linux-x86_64-c16-bare-intel/20261117T115451Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210558Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 88 ns | 72.92x |
| ema_neon | 1 ns | 1.00x |
| ema_scalar | 25 ns | 20.83x |

- [20260510T204028Z-3c0d0d5](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20260510T204028Z-3c0d0d5/report.md)
- [20261114T210558Z-266650fb-dirty](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20261114T210558Z-266650fb-dirty/report.md)

## ema_axis_h/ema_axis_h_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T115451Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 742 ns | 1.90x |
| ema_neon | 614 ns | 1.57x |
| ema_scalar | 390 ns | 1.00x |

- [20261115T040514Z-266650fb-dirty](ema_axis_h/ema_axis_h_n4096/linux-x86_64-c16-bare-intel/20261115T040514Z-266650fb-dirty/report.md)
- [20261117T115451Z-07a9e9a3](ema_axis_h/ema_axis_h_n4096/linux-x86_64-c16-bare-intel/20261117T115451Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210602Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 1.70 us | 5.47x |
| ema_neon | 310 ns | 1.00x |
| ema_scalar | 395 ns | 1.27x |

- [20260510T204030Z-3c0d0d5](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20260510T204030Z-3c0d0d5/report.md)
- [20261114T210602Z-266650fb-dirty](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20261114T210602Z-266650fb-dirty/report.md)

## ema_axis_h/ema_axis_h_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T115451Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 13 ns | 1.06x |
| ema_neon | 12 ns | 1.00x |
| ema_scalar | 15 ns | 1.20x |

- [20261115T035736Z-266650fb-dirty](ema_axis_h/ema_axis_h_n64/linux-x86_64-c16-bare-intel/20261115T035736Z-266650fb-dirty/report.md)
- [20261117T115451Z-07a9e9a3](ema_axis_h/ema_axis_h_n64/linux-x86_64-c16-bare-intel/20261117T115451Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210556Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 0 ns | 1.00x |
| ema_neon | 0 ns | 1.00x |
| ema_scalar | 3 ns | 7.25x |

- [20260510T204028Z-3c0d0d5](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20260510T204028Z-3c0d0d5/report.md)
- [20261114T210556Z-266650fb-dirty](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20261114T210556Z-266650fb-dirty/report.md)

## ema_formulation/ema_formulation_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T120812Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 2.00 us | 2.32x |
| ema_form_intshift | 863 ns | 1.00x |
| ema_form_q32 | 1.43 us | 1.66x |

- [20261115T041327Z-266650fb-dirty](ema_formulation/ema_formulation_n1024/linux-x86_64-c16-bare-intel/20261115T041327Z-266650fb-dirty/report.md)
- [20261117T120812Z-07a9e9a3](ema_formulation/ema_formulation_n1024/linux-x86_64-c16-bare-intel/20261117T120812Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210643Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 2.24 us | 1.73x |
| ema_form_intshift | 1.29 us | 1.00x |
| ema_form_q32 | 1.93 us | 1.50x |

- [20260510T233132Z-5dcdb36](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20260510T233132Z-5dcdb36/report.md)
- [20261114T210643Z-266650fb-dirty](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20261114T210643Z-266650fb-dirty/report.md)

## ema_formulation/ema_formulation_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T120812Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 26.15 us | 1.91x |
| ema_form_intshift | 13.70 us | 1.00x |
| ema_form_q32 | 22.83 us | 1.67x |

- [20261115T041834Z-266650fb-dirty](ema_formulation/ema_formulation_n16384/linux-x86_64-c16-bare-intel/20261115T041834Z-266650fb-dirty/report.md)
- [20261117T120812Z-07a9e9a3](ema_formulation/ema_formulation_n16384/linux-x86_64-c16-bare-intel/20261117T120812Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210651Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 35.71 us | 1.74x |
| ema_form_intshift | 20.46 us | 1.00x |
| ema_form_q32 | 30.68 us | 1.50x |

- [20260510T233138Z-5dcdb36](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20260510T233138Z-5dcdb36/report.md)
- [20261114T210651Z-266650fb-dirty](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20261114T210651Z-266650fb-dirty/report.md)

## ema_formulation/ema_formulation_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T120812Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 509 ns | 2.29x |
| ema_form_intshift | 222 ns | 1.00x |
| ema_form_q32 | 363 ns | 1.64x |

- [20261115T041052Z-266650fb-dirty](ema_formulation/ema_formulation_n256/linux-x86_64-c16-bare-intel/20261115T041052Z-266650fb-dirty/report.md)
- [20261117T120812Z-07a9e9a3](ema_formulation/ema_formulation_n256/linux-x86_64-c16-bare-intel/20261117T120812Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210641Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 439 ns | 1.48x |
| ema_form_intshift | 296 ns | 1.00x |
| ema_form_q32 | 474 ns | 1.60x |

- [20260510T233131Z-5dcdb36](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20260510T233131Z-5dcdb36/report.md)
- [20261114T210641Z-266650fb-dirty](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20261114T210641Z-266650fb-dirty/report.md)

## ema_formulation/ema_formulation_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T120812Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 8.00 us | 2.33x |
| ema_form_intshift | 3.43 us | 1.00x |
| ema_form_q32 | 5.71 us | 1.66x |

- [20261115T041559Z-266650fb-dirty](ema_formulation/ema_formulation_n4096/linux-x86_64-c16-bare-intel/20261115T041559Z-266650fb-dirty/report.md)
- [20261117T120812Z-07a9e9a3](ema_formulation/ema_formulation_n4096/linux-x86_64-c16-bare-intel/20261117T120812Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210646Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 9.10 us | 1.77x |
| ema_form_intshift | 5.15 us | 1.00x |
| ema_form_q32 | 7.63 us | 1.48x |

- [20260510T233134Z-5dcdb36](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20260510T233134Z-5dcdb36/report.md)
- [20261114T210646Z-266650fb-dirty](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20261114T210646Z-266650fb-dirty/report.md)

## engine_vs_std/accumulator_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 870.94 us | 2.19x |
| evs_engine_par | 770.84 us | 1.93x |
| evs_std | 398.38 us | 1.00x |
| evs_std_par | 516.89 us | 1.30x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/accumulator_n1048576/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013654Z-23953983](engine_vs_std/accumulator_n1048576/linux-x86_64-c4-container-intel/20261114T013654Z-23953983/report.md)

## engine_vs_std/accumulator_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 39.28 us | 6.16x |
| evs_engine_par | 110.43 us | 17.31x |
| evs_std | 6.38 us | 1.00x |
| evs_std_par | 46.80 us | 7.33x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/accumulator_n4096/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013321Z-c894a779](engine_vs_std/accumulator_n4096/linux-x86_64-c4-container-intel/20261114T013321Z-c894a779/report.md)

## engine_vs_std/accumulator_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 3.50 ms | 2.06x |
| evs_engine_par | 1.79 ms | 1.05x |
| evs_std | 1.84 ms | 1.08x |
| evs_std_par | 1.70 ms | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/accumulator_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013855Z-23953983](engine_vs_std/accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T013855Z-23953983/report.md)

## engine_vs_std/accumulator_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 57.29 us | 2.79x |
| evs_engine_par | 253.99 us | 12.36x |
| evs_std | 20.55 us | 1.00x |
| evs_std_par | 194.05 us | 9.44x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/accumulator_n65536/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013507Z-23953983](engine_vs_std/accumulator_n65536/linux-x86_64-c4-container-intel/20261114T013507Z-23953983/report.md)

## engine_vs_std/branching_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 951.53 us | 1.73x |
| evs_std | 548.70 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/branching_n1048576/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014141Z-23953983](engine_vs_std/branching_n1048576/linux-x86_64-c4-container-intel/20261114T014141Z-23953983/report.md)

## engine_vs_std/branching_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 31.23 us | 4.73x |
| evs_std | 6.61 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/branching_n4096/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013949Z-23953983](engine_vs_std/branching_n4096/linux-x86_64-c4-container-intel/20261114T013949Z-23953983/report.md)

## engine_vs_std/branching_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 4.48 ms | 2.05x |
| evs_std | 2.19 ms | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/branching_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014245Z-23953983](engine_vs_std/branching_n4194304/linux-x86_64-c4-container-intel/20261114T014245Z-23953983/report.md)

## engine_vs_std/branching_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 35.50 us | 1.21x |
| evs_std | 29.27 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/branching_n65536/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014044Z-23953983](engine_vs_std/branching_n65536/linux-x86_64-c4-container-intel/20261114T014044Z-23953983/report.md)

## engine_vs_std/element_wise_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 396.88 us | 1.07x |
| evs_std | 370.27 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/element_wise_n1048576/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014531Z-23953983](engine_vs_std/element_wise_n1048576/linux-x86_64-c4-container-intel/20261114T014531Z-23953983/report.md)

## engine_vs_std/element_wise_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 6.06 us | 1.09x |
| evs_std | 5.54 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/element_wise_n4096/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014339Z-23953983](engine_vs_std/element_wise_n4096/linux-x86_64-c4-container-intel/20261114T014339Z-23953983/report.md)

## engine_vs_std/element_wise_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 1.94 ms | 1.00x |
| evs_std | 2.18 ms | 1.12x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/element_wise_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014634Z-23953983](engine_vs_std/element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014634Z-23953983/report.md)

## engine_vs_std/element_wise_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 20.67 us | 1.01x |
| evs_std | 20.37 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/element_wise_n65536/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014434Z-23953983](engine_vs_std/element_wise_n65536/linux-x86_64-c4-container-intel/20261114T014434Z-23953983/report.md)

## engine_vs_std/startup_accumulator_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 2.40 ms | 10.67x |
| evs_std_startup | 225.36 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/startup_accumulator_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014741Z-23953983](engine_vs_std/startup_accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T014741Z-23953983/report.md)

## engine_vs_std/startup_branching_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 8.92 ms | 40.13x |
| evs_std_startup | 222.33 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/startup_branching_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014850Z-23953983](engine_vs_std/startup_branching_n4194304/linux-x86_64-c4-container-intel/20261114T014850Z-23953983/report.md)

## engine_vs_std/startup_element_wise_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 10.80 ms | 45.64x |
| evs_std_startup | 236.66 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/startup_element_wise_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014957Z-23953983](engine_vs_std/startup_element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014957Z-23953983/report.md)

## engine_vs_std/startup_wide_parallel_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 17.13 ms | 17.22x |
| evs_std_startup | 995.24 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/startup_wide_parallel_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015143Z-23953983](engine_vs_std/startup_wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T015143Z-23953983/report.md)

## engine_vs_std/wide_parallel_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 10.97 ms | 2.26x |
| evs_engine_par | 4.85 ms | 1.00x |
| evs_std | 10.72 ms | 2.21x |
| evs_std_par | 8.50 ms | 1.75x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/wide_parallel_n1048576/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015712Z-23953983](engine_vs_std/wide_parallel_n1048576/linux-x86_64-c4-container-intel/20261114T015712Z-23953983/report.md)

## engine_vs_std/wide_parallel_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 56.79 us | 1.08x |
| evs_engine_par | 265.14 us | 5.03x |
| evs_std | 52.76 us | 1.00x |
| evs_std_par | 209.00 us | 3.96x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/wide_parallel_n4096/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015326Z-23953983](engine_vs_std/wide_parallel_n4096/linux-x86_64-c4-container-intel/20261114T015326Z-23953983/report.md)

## engine_vs_std/wide_parallel_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 41.50 ms | 1.90x |
| evs_engine_par | 21.86 ms | 1.00x |
| evs_std | 43.03 ms | 1.97x |
| evs_std_par | 24.29 ms | 1.11x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/wide_parallel_n4194304/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T020005Z-23953983](engine_vs_std/wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T020005Z-23953983/report.md)

## engine_vs_std/wide_parallel_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T121923Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 665.93 us | 1.07x |
| evs_engine_par | 1.45 ms | 2.32x |
| evs_std | 667.76 us | 1.07x |
| evs_std_par | 623.09 us | 1.00x |

- [20261117T121923Z-07a9e9a3](engine_vs_std/wide_parallel_n65536/linux-x86_64-c16-bare-intel/20261117T121923Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015511Z-23953983](engine_vs_std/wide_parallel_n65536/linux-x86_64-c4-container-intel/20261114T015511Z-23953983/report.md)

## fiber_theory/fiber_theory_n16

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T124716Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 96 ns | 1.00x |
| fib_spectral | 65.80 us | 687.54x |

- [20261115T042302Z-266650fb-dirty](fiber_theory/fiber_theory_n16/linux-x86_64-c16-bare-intel/20261115T042302Z-266650fb-dirty/report.md)
- [20261117T124716Z-07a9e9a3](fiber_theory/fiber_theory_n16/linux-x86_64-c16-bare-intel/20261117T124716Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210756Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 96 ns | 1.00x |
| fib_spectral | 64.19 us | 667.28x |

- [20260719T190052Z-e4cd1757-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20260719T190052Z-e4cd1757-dirty/report.md)
- [20261114T210756Z-266650fb-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20261114T210756Z-266650fb-dirty/report.md)

## fiber_theory/fiber_theory_n32

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T124716Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 192 ns | 1.00x |
| fib_spectral | 85.62 us | 444.77x |

- [20261115T042449Z-266650fb-dirty](fiber_theory/fiber_theory_n32/linux-x86_64-c16-bare-intel/20261115T042449Z-266650fb-dirty/report.md)
- [20261117T124716Z-07a9e9a3](fiber_theory/fiber_theory_n32/linux-x86_64-c16-bare-intel/20261117T124716Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210800Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 191 ns | 1.00x |
| fib_spectral | 80.28 us | 419.41x |

- [20260719T190056Z-e4cd1757-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20260719T190056Z-e4cd1757-dirty/report.md)
- [20261114T210800Z-266650fb-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20261114T210800Z-266650fb-dirty/report.md)

## fiber_theory/fiber_theory_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T124716Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 348 ns | 1.00x |
| fib_spectral | 114.19 us | 328.60x |

- [20261115T042637Z-266650fb-dirty](fiber_theory/fiber_theory_n64/linux-x86_64-c16-bare-intel/20261115T042637Z-266650fb-dirty/report.md)
- [20261117T124716Z-07a9e9a3](fiber_theory/fiber_theory_n64/linux-x86_64-c16-bare-intel/20261117T124716Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210807Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 392 ns | 1.00x |
| fib_spectral | 128.20 us | 326.63x |

- [20260719T190101Z-e4cd1757-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20260719T190101Z-e4cd1757-dirty/report.md)
- [20261114T210807Z-266650fb-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20261114T210807Z-266650fb-dirty/report.md)

## fiber_theory/fiber_theory_n8

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T124716Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 50 ns | 1.00x |
| fib_spectral | 16.04 us | 317.58x |

- [20261115T042113Z-266650fb-dirty](fiber_theory/fiber_theory_n8/linux-x86_64-c16-bare-intel/20261115T042113Z-266650fb-dirty/report.md)
- [20261117T124716Z-07a9e9a3](fiber_theory/fiber_theory_n8/linux-x86_64-c16-bare-intel/20261117T124716Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210752Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 36 ns | 1.00x |
| fib_spectral | 16.63 us | 459.44x |

- [20260719T190048Z-e4cd1757-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20260719T190048Z-e4cd1757-dirty/report.md)
- [20261114T210752Z-266650fb-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20261114T210752Z-266650fb-dirty/report.md)

## float_width/float_width_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T125503Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 2.57 us | 4.44x |
| float_f64 | 2.08 us | 3.60x |
| float_i64_scaled | 578 ns | 1.00x |

- [20261115T043214Z-266650fb-dirty](float_width/float_width_n1024/linux-x86_64-c16-bare-intel/20261115T043214Z-266650fb-dirty/report.md)
- [20261117T125503Z-07a9e9a3](float_width/float_width_n1024/linux-x86_64-c16-bare-intel/20261117T125503Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210846Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 262 ns | 1.00x |
| float_f64 | 332 ns | 1.27x |
| float_i64_scaled | 479 ns | 1.83x |

- [20260511T041156Z-e203deb](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20260511T041156Z-e203deb/report.md)
- [20261114T210846Z-266650fb-dirty](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20261114T210846Z-266650fb-dirty/report.md)

## float_width/float_width_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T125503Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 40.51 us | 4.43x |
| float_f64 | 40.51 us | 4.43x |
| float_i64_scaled | 9.14 us | 1.00x |

- [20261115T043721Z-266650fb-dirty](float_width/float_width_n16384/linux-x86_64-c16-bare-intel/20261115T043721Z-266650fb-dirty/report.md)
- [20261117T125503Z-07a9e9a3](float_width/float_width_n16384/linux-x86_64-c16-bare-intel/20261117T125503Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210851Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 5.22 us | 1.00x |
| float_f64 | 5.21 us | 1.00x |
| float_i64_scaled | 6.45 us | 1.24x |

- [20260511T041159Z-e203deb](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20260511T041159Z-e203deb/report.md)
- [20261114T210851Z-266650fb-dirty](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20261114T210851Z-266650fb-dirty/report.md)

## float_width/float_width_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T125503Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 535 ns | 4.33x |
| float_f64 | 649 ns | 5.26x |
| float_i64_scaled | 124 ns | 1.00x |

- [20261115T042939Z-266650fb-dirty](float_width/float_width_n256/linux-x86_64-c16-bare-intel/20261115T042939Z-266650fb-dirty/report.md)
- [20261117T125503Z-07a9e9a3](float_width/float_width_n256/linux-x86_64-c16-bare-intel/20261117T125503Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210843Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 50 ns | 1.00x |
| float_f64 | 75 ns | 1.50x |
| float_i64_scaled | 134 ns | 2.67x |

- [20260511T041155Z-e203deb](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20260511T041155Z-e203deb/report.md)
- [20261114T210843Z-266650fb-dirty](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20261114T210843Z-266650fb-dirty/report.md)

## float_width/float_width_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T125503Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 10.41 us | 4.55x |
| float_f64 | 10.55 us | 4.61x |
| float_i64_scaled | 2.29 us | 1.00x |

- [20261115T043446Z-266650fb-dirty](float_width/float_width_n4096/linux-x86_64-c16-bare-intel/20261115T043446Z-266650fb-dirty/report.md)
- [20261117T125503Z-07a9e9a3](float_width/float_width_n4096/linux-x86_64-c16-bare-intel/20261117T125503Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210848Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 1.28 us | 1.00x |
| float_f64 | 1.36 us | 1.06x |
| float_i64_scaled | 1.74 us | 1.36x |

- [20260511T041157Z-e203deb](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20260511T041157Z-e203deb/report.md)
- [20261114T210848Z-266650fb-dirty](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20261114T210848Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n1024

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261117T130553Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 180 ns | 1.03x |
| fpt_table_16 | 176 ns | 1.00x |

- [20261115T044117Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/linux-x86_64-c16-bare-intel/20261115T044117Z-266650fb-dirty/report.md)
- [20261117T130553Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n1024/linux-x86_64-c16-bare-intel/20261117T130553Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210922Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 236 ns | 1.00x |
| fpt_table_16 | 328 ns | 1.39x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210922Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20261114T210922Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T130553Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 13.26 us | 1.00x |
| fpt_table_16 | 15.61 us | 1.18x |

- [20261115T044445Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/linux-x86_64-c16-bare-intel/20261115T044445Z-266650fb-dirty/report.md)
- [20261117T130553Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n16384/linux-x86_64-c16-bare-intel/20261117T130553Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210926Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 10.26 us | 1.00x |
| fpt_table_16 | 11.37 us | 1.11x |

- [20260511T051112Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20260511T051112Z-9b76747/report.md)
- [20261114T210926Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20261114T210926Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T130553Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 30 ns | 1.00x |
| fpt_table_16 | 30 ns | 1.00x |

- [20261115T043931Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/linux-x86_64-c16-bare-intel/20261115T043931Z-266650fb-dirty/report.md)
- [20261117T130553Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n256/linux-x86_64-c16-bare-intel/20261117T130553Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210920Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 71 ns | 1.58x |
| fpt_table_16 | 45 ns | 1.00x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210920Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20261114T210920Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T130553Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 590 ns | 1.00x |
| fpt_table_16 | 1.78 us | 3.02x |

- [20261115T044301Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/linux-x86_64-c16-bare-intel/20261115T044301Z-266650fb-dirty/report.md)
- [20261117T130553Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n4096/linux-x86_64-c16-bare-intel/20261117T130553Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210923Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 841 ns | 1.00x |
| fpt_table_16 | 1.30 us | 1.55x |

- [20260511T051110Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20260511T051110Z-9b76747/report.md)
- [20261114T210923Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20261114T210923Z-266650fb-dirty/report.md)

## fold_strategy/fold_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T131328Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 79 ns | 1.76x |
| fold_quad | 45 ns | 1.00x |
| fold_sequential | 150 ns | 3.31x |

- [20261115T045022Z-266650fb-dirty](fold_strategy/fold_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T045022Z-266650fb-dirty/report.md)
- [20261117T131328Z-07a9e9a3](fold_strategy/fold_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T131328Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211005Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 73 ns | 1.26x |
| fold_quad | 58 ns | 1.00x |
| fold_sequential | 113 ns | 1.93x |

- [20260511T023054Z-7ed697d](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T023054Z-7ed697d/report.md)
- [20261114T211005Z-266650fb-dirty](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211005Z-266650fb-dirty/report.md)

## fold_strategy/fold_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T131328Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 1.15 us | 1.98x |
| fold_quad | 580 ns | 1.00x |
| fold_sequential | 2.29 us | 3.94x |

- [20261115T045526Z-266650fb-dirty](fold_strategy/fold_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T045526Z-266650fb-dirty/report.md)
- [20261117T131328Z-07a9e9a3](fold_strategy/fold_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T131328Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211009Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 1.29 us | 1.83x |
| fold_quad | 704 ns | 1.00x |
| fold_sequential | 2.68 us | 3.81x |

- [20260511T023056Z-7ed697d](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T023056Z-7ed697d/report.md)
- [20261114T211009Z-266650fb-dirty](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211009Z-266650fb-dirty/report.md)

## fold_strategy/fold_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T131328Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 26 ns | 1.23x |
| fold_quad | 21 ns | 1.00x |
| fold_sequential | 43 ns | 2.03x |

- [20261115T044747Z-266650fb-dirty](fold_strategy/fold_strategy_n256/linux-x86_64-c16-bare-intel/20261115T044747Z-266650fb-dirty/report.md)
- [20261117T131328Z-07a9e9a3](fold_strategy/fold_strategy_n256/linux-x86_64-c16-bare-intel/20261117T131328Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211003Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 13 ns | 1.00x |
| fold_quad | 15 ns | 1.16x |
| fold_sequential | 16 ns | 1.23x |

- [20260511T023053Z-7ed697d](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20260511T023053Z-7ed697d/report.md)
- [20261114T211003Z-266650fb-dirty](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211003Z-266650fb-dirty/report.md)

## fold_strategy/fold_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T131328Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 294 ns | 1.93x |
| fold_quad | 152 ns | 1.00x |
| fold_sequential | 577 ns | 3.79x |

- [20261115T045253Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T045253Z-266650fb-dirty/report.md)
- [20261117T131328Z-07a9e9a3](fold_strategy/fold_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T131328Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211007Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 331 ns | 1.68x |
| fold_quad | 197 ns | 1.00x |
| fold_sequential | 617 ns | 3.14x |

- [20260511T023055Z-7ed697d](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T023055Z-7ed697d/report.md)
- [20261114T211007Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211007Z-266650fb-dirty/report.md)

## frame_wide/heavy_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 32.26 ms | 4.07x |
| engine_par | 35.62 ms | 4.50x |
| hand | 38.84 ms | 4.90x |
| hand_par | 7.92 ms | 1.00x |
| rust_nostd | 31.77 ms | 4.01x |
| rust_std | 34.65 ms | 4.37x |
| rust_std_par | 8.19 ms | 1.03x |

- [20261117T132523Z-07a9e9a3](frame_wide/heavy_n1048576/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

## frame_wide/heavy_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 509.32 ms | 5.04x |
| engine_par | 448.18 ms | 4.43x |
| hand | 620.70 ms | 6.14x |
| hand_par | 101.14 ms | 1.00x |
| rust_nostd | 509.07 ms | 5.03x |
| rust_std | 577.66 ms | 5.71x |
| rust_std_par | 140.04 ms | 1.38x |

- [20261117T132523Z-07a9e9a3](frame_wide/heavy_n16777216/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

## frame_wide/heavy_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 127.27 ms | 4.05x |
| engine_par | 117.67 ms | 3.74x |
| hand | 155.27 ms | 4.94x |
| hand_par | 36.95 ms | 1.18x |
| rust_nostd | 127.16 ms | 4.04x |
| rust_std | 149.73 ms | 4.76x |
| rust_std_par | 31.44 ms | 1.00x |

- [20261117T132523Z-07a9e9a3](frame_wide/heavy_n4194304/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

## frame_wide/heavy_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 2.01 ms | 2.76x |
| engine_par | 5.10 ms | 7.01x |
| hand | 2.43 ms | 3.34x |
| hand_par | 1.74 ms | 2.39x |
| rust_nostd | 2.01 ms | 2.76x |
| rust_std | 2.03 ms | 2.79x |
| rust_std_par | 727.95 us | 1.00x |

- [20261117T132523Z-07a9e9a3](frame_wide/heavy_n65536/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151355Z-c1eee3f5-dirty](frame_wide/heavy_n65536/linux-x86_64-c4-container-intel/20261113T151355Z-c1eee3f5-dirty/report.md)

## frame_wide/light_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 4.86 ms | 5.53x |
| engine_par | 5.26 ms | 5.98x |
| hand | 1.08 ms | 1.23x |
| hand_par | 878.80 us | 1.00x |
| rust_nostd | 6.01 ms | 6.84x |
| rust_std | 6.19 ms | 7.04x |
| rust_std_par | 4.58 ms | 5.21x |

- [20261117T132523Z-07a9e9a3](frame_wide/light_n1048576/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

## frame_wide/light_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 74.58 ms | 4.65x |
| engine_par | 71.05 ms | 4.43x |
| hand | 16.04 ms | 1.00x |
| hand_par | 19.89 ms | 1.24x |
| rust_nostd | 89.71 ms | 5.59x |
| rust_std | 144.17 ms | 8.99x |
| rust_std_par | 125.48 ms | 7.82x |

- [20261117T132523Z-07a9e9a3](frame_wide/light_n16777216/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

## frame_wide/light_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 18.49 ms | 4.58x |
| engine_par | 19.62 ms | 4.86x |
| hand | 4.04 ms | 1.00x |
| hand_par | 5.00 ms | 1.24x |
| rust_nostd | 22.24 ms | 5.51x |
| rust_std | 27.73 ms | 6.87x |
| rust_std_par | 26.51 ms | 6.57x |

- [20261117T132523Z-07a9e9a3](frame_wide/light_n4194304/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

## frame_wide/light_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T132523Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 121.97 us | 2.30x |
| engine_par | 890.28 us | 16.82x |
| hand | 52.93 us | 1.00x |
| hand_par | 307.42 us | 5.81x |
| rust_nostd | 115.25 us | 2.18x |
| rust_std | 111.65 us | 2.11x |
| rust_std_par | 485.71 us | 9.18x |

- [20261117T132523Z-07a9e9a3](frame_wide/light_n65536/linux-x86_64-c16-bare-intel/20261117T132523Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151715Z-c1eee3f5-dirty](frame_wide/light_n65536/linux-x86_64-c4-container-intel/20261113T151715Z-c1eee3f5-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T140950Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 97 ns | 1.00x |
| fxpmul_precise | 151 ns | 1.55x |

- [20261115T050123Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T050123Z-266650fb-dirty/report.md)
- [20261117T140950Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T140950Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211059Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 102 ns | 1.00x |
| fxpmul_precise | 111 ns | 1.10x |

- [20260510T213422Z-b042648](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213422Z-b042648/report.md)
- [20261114T211059Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211059Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T140950Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.43 us | 1.00x |
| fxpmul_precise | 2.29 us | 1.60x |

- [20261115T050449Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T050449Z-266650fb-dirty/report.md)
- [20261117T140950Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T140950Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211102Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.70 us | 1.00x |
| fxpmul_precise | 1.98 us | 1.17x |

- [20260510T213424Z-b042648](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213424Z-b042648/report.md)
- [20261114T211102Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211102Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T140950Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 25 ns | 1.00x |
| fxpmul_precise | 44 ns | 1.75x |

- [20261115T045940Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/linux-x86_64-c16-bare-intel/20261115T045940Z-266650fb-dirty/report.md)
- [20261117T140950Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n256/linux-x86_64-c16-bare-intel/20261117T140950Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211058Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 19 ns | 1.00x |
| fxpmul_precise | 27 ns | 1.43x |

- [20260510T213421Z-b042648](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213421Z-b042648/report.md)
- [20261114T211058Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211058Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T140950Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 364 ns | 1.00x |
| fxpmul_precise | 578 ns | 1.59x |

- [20261115T050306Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T050306Z-266650fb-dirty/report.md)
- [20261117T140950Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T140950Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211101Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 401 ns | 1.00x |
| fxpmul_precise | 501 ns | 1.25x |

- [20260510T213423Z-b042648](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213423Z-b042648/report.md)
- [20261114T211101Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211101Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T140950Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 11 ns | 1.00x |
| fxpmul_precise | 14 ns | 1.24x |

- [20261115T045753Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/linux-x86_64-c16-bare-intel/20261115T045753Z-266650fb-dirty/report.md)
- [20261117T140950Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n64/linux-x86_64-c16-bare-intel/20261117T140950Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211056Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 3 ns | 1.00x |
| fxpmul_precise | 3 ns | 1.00x |

- [20260510T213420Z-b042648](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213420Z-b042648/report.md)
- [20261114T211056Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211056Z-266650fb-dirty/report.md)

## hash_algos/hash_algos_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T141908Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 944 ns | 4.99x |
| hash_sip | 201 ns | 1.06x |
| hash_xxh | 189 ns | 1.00x |

- [20261115T051258Z-266650fb-dirty](hash_algos/hash_algos_n1024/linux-x86_64-c16-bare-intel/20261115T051258Z-266650fb-dirty/report.md)
- [20261117T141908Z-07a9e9a3](hash_algos/hash_algos_n1024/linux-x86_64-c16-bare-intel/20261117T141908Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211142Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 1.31 us | 5.58x |
| hash_sip | 287 ns | 1.22x |
| hash_xxh | 235 ns | 1.00x |

- [20260510T213309Z-b042648](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20260510T213309Z-b042648/report.md)
- [20261114T211142Z-266650fb-dirty](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20261114T211142Z-266650fb-dirty/report.md)

## hash_algos/hash_algos_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T141908Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 18.26 us | 6.39x |
| hash_sip | 3.09 us | 1.08x |
| hash_xxh | 2.86 us | 1.00x |

- [20261115T051803Z-266650fb-dirty](hash_algos/hash_algos_n16384/linux-x86_64-c16-bare-intel/20261115T051803Z-266650fb-dirty/report.md)
- [20261117T141908Z-07a9e9a3](hash_algos/hash_algos_n16384/linux-x86_64-c16-bare-intel/20261117T141908Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211148Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 20.44 us | 4.50x |
| hash_sip | 4.54 us | 1.00x |
| hash_xxh | 4.77 us | 1.05x |

- [20260510T213314Z-b042648](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20260510T213314Z-b042648/report.md)
- [20261114T211148Z-266650fb-dirty](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20261114T211148Z-266650fb-dirty/report.md)

## hash_algos/hash_algos_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T141908Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 292 ns | 5.66x |
| hash_sip | 58 ns | 1.12x |
| hash_xxh | 52 ns | 1.00x |

- [20261115T051026Z-266650fb-dirty](hash_algos/hash_algos_n256/linux-x86_64-c16-bare-intel/20261115T051026Z-266650fb-dirty/report.md)
- [20261117T141908Z-07a9e9a3](hash_algos/hash_algos_n256/linux-x86_64-c16-bare-intel/20261117T141908Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211140Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 288 ns | 8.05x |
| hash_sip | 61 ns | 1.70x |
| hash_xxh | 36 ns | 1.00x |

- [20260510T213307Z-b042648](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20260510T213307Z-b042648/report.md)
- [20261114T211140Z-266650fb-dirty](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20261114T211140Z-266650fb-dirty/report.md)

## hash_algos/hash_algos_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T141908Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 3.75 us | 5.17x |
| hash_sip | 786 ns | 1.08x |
| hash_xxh | 725 ns | 1.00x |

- [20261115T051530Z-266650fb-dirty](hash_algos/hash_algos_n4096/linux-x86_64-c16-bare-intel/20261115T051530Z-266650fb-dirty/report.md)
- [20261117T141908Z-07a9e9a3](hash_algos/hash_algos_n4096/linux-x86_64-c16-bare-intel/20261117T141908Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211145Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 5.26 us | 4.59x |
| hash_sip | 1.19 us | 1.04x |
| hash_xxh | 1.15 us | 1.00x |

- [20260510T213311Z-b042648](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20260510T213311Z-b042648/report.md)
- [20261114T211145Z-266650fb-dirty](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20261114T211145Z-266650fb-dirty/report.md)

## hash_algos/hash_algos_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T141908Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 78 ns | 4.21x |
| hash_sip | 22 ns | 1.22x |
| hash_xxh | 18 ns | 1.00x |

- [20261115T050751Z-266650fb-dirty](hash_algos/hash_algos_n64/linux-x86_64-c16-bare-intel/20261115T050751Z-266650fb-dirty/report.md)
- [20261117T141908Z-07a9e9a3](hash_algos/hash_algos_n64/linux-x86_64-c16-bare-intel/20261117T141908Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211138Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 39 ns | 9.24x |
| hash_sip | 9 ns | 2.10x |
| hash_xxh | 4 ns | 1.00x |

- [20260510T213306Z-b042648](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20260510T213306Z-b042648/report.md)
- [20261114T211138Z-266650fb-dirty](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20261114T211138Z-266650fb-dirty/report.md)

## inline_strategy/inline_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T143229Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 1.15 us | 1.22x |
| inline_default | 939 ns | 1.00x |
| inline_never | 1.15 us | 1.23x |

- [20261115T052342Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T052342Z-266650fb-dirty/report.md)
- [20261117T143229Z-07a9e9a3](inline_strategy/inline_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T143229Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T211227Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inline_default | 1.31 us | 1.00x |
| inline_never | 1.32 us | 1.00x |

- [20260511T003101Z-63d18f0](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T003101Z-63d18f0/report.md)
- [20261114T211226Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211226Z-266650fb-dirty/report.md)

## inline_strategy/inline_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T143229Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 18.27 us | 1.22x |
| inline_default | 14.95 us | 1.00x |
| inline_never | 14.95 us | 1.00x |

- [20261115T052848Z-266650fb-dirty](inline_strategy/inline_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T052848Z-266650fb-dirty/report.md)
- [20261117T143229Z-07a9e9a3](inline_strategy/inline_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T143229Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211234Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 20.46 us | 1.00x |
| inline_default | 20.49 us | 1.00x |
| inline_never | 20.52 us | 1.00x |

- [20260511T003107Z-63d18f0](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T003107Z-63d18f0/report.md)
- [20261114T211234Z-266650fb-dirty](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211234Z-266650fb-dirty/report.md)

## inline_strategy/inline_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T143229Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 292 ns | 1.22x |
| inline_default | 240 ns | 1.00x |
| inline_never | 293 ns | 1.22x |

- [20261115T052107Z-266650fb-dirty](inline_strategy/inline_strategy_n256/linux-x86_64-c16-bare-intel/20261115T052107Z-266650fb-dirty/report.md)
- [20261117T143229Z-07a9e9a3](inline_strategy/inline_strategy_n256/linux-x86_64-c16-bare-intel/20261117T143229Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211224Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 276 ns | 1.00x |
| inline_default | 295 ns | 1.07x |
| inline_never | 336 ns | 1.22x |

- [20260511T003100Z-63d18f0](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20260511T003100Z-63d18f0/report.md)
- [20261114T211224Z-266650fb-dirty](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211224Z-266650fb-dirty/report.md)

## inline_strategy/inline_strategy_n4096

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261117T143229Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 3.75 us | 1.00x |
| inline_default | 3.75 us | 1.00x |
| inline_never | 3.75 us | 1.00x |

- [20261115T052614Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T052614Z-266650fb-dirty/report.md)
- [20261117T143229Z-07a9e9a3](inline_strategy/inline_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T143229Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211229Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 5.42 us | 1.02x |
| inline_default | 5.43 us | 1.02x |
| inline_never | 5.32 us | 1.00x |

- [20260511T003103Z-63d18f0](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T003103Z-63d18f0/report.md)
- [20261114T211229Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211229Z-266650fb-dirty/report.md)

## iter_patterns/iter_patterns_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T144320Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 297 ns | 11.75x |
| iter_seq | 149 ns | 5.91x |
| iter_strided | 25 ns | 1.00x |

- [20261115T053425Z-266650fb-dirty](iter_patterns/iter_patterns_n1024/linux-x86_64-c16-bare-intel/20261115T053425Z-266650fb-dirty/report.md)
- [20261117T144320Z-07a9e9a3](iter_patterns/iter_patterns_n1024/linux-x86_64-c16-bare-intel/20261117T144320Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211312Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 265 ns | 71.73x |
| iter_seq | 118 ns | 31.76x |
| iter_strided | 4 ns | 1.00x |

- [20260510T231055Z-b4fbd1a](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20260510T231055Z-b4fbd1a/report.md)
- [20261114T211312Z-266650fb-dirty](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20261114T211312Z-266650fb-dirty/report.md)

## iter_patterns/iter_patterns_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T144320Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 4.63 us | 15.85x |
| iter_seq | 2.29 us | 7.83x |
| iter_strided | 292 ns | 1.00x |

- [20261115T053929Z-266650fb-dirty](iter_patterns/iter_patterns_n16384/linux-x86_64-c16-bare-intel/20261115T053929Z-266650fb-dirty/report.md)
- [20261117T144320Z-07a9e9a3](iter_patterns/iter_patterns_n16384/linux-x86_64-c16-bare-intel/20261117T144320Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211316Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 4.73 us | 16.96x |
| iter_seq | 2.63 us | 9.43x |
| iter_strided | 279 ns | 1.00x |

- [20260510T231058Z-b4fbd1a](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20260510T231058Z-b4fbd1a/report.md)
- [20261114T211316Z-266650fb-dirty](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20261114T211316Z-266650fb-dirty/report.md)

## iter_patterns/iter_patterns_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T144320Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 82 ns | 8.57x |
| iter_seq | 35 ns | 3.69x |
| iter_strided | 10 ns | 1.00x |

- [20261115T053150Z-266650fb-dirty](iter_patterns/iter_patterns_n256/linux-x86_64-c16-bare-intel/20261115T053150Z-266650fb-dirty/report.md)
- [20261117T144320Z-07a9e9a3](iter_patterns/iter_patterns_n256/linux-x86_64-c16-bare-intel/20261117T144320Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211310Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 47 ns | 58.38x |
| iter_seq | 17 ns | 21.38x |
| iter_strided | 1 ns | 1.00x |

- [20260510T231054Z-b4fbd1a](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20260510T231054Z-b4fbd1a/report.md)
- [20261114T211310Z-266650fb-dirty](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20261114T211310Z-266650fb-dirty/report.md)

## iter_patterns/iter_patterns_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T144320Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 1.17 us | 14.86x |
| iter_seq | 577 ns | 7.36x |
| iter_strided | 78 ns | 1.00x |

- [20261115T053657Z-266650fb-dirty](iter_patterns/iter_patterns_n4096/linux-x86_64-c16-bare-intel/20261115T053657Z-266650fb-dirty/report.md)
- [20261117T144320Z-07a9e9a3](iter_patterns/iter_patterns_n4096/linux-x86_64-c16-bare-intel/20261117T144320Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211314Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 1.17 us | 21.51x |
| iter_seq | 629 ns | 11.57x |
| iter_strided | 54 ns | 1.00x |

- [20260510T231056Z-b4fbd1a](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20260510T231056Z-b4fbd1a/report.md)
- [20261114T211314Z-266650fb-dirty](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20261114T211314Z-266650fb-dirty/report.md)

## link-static/dev_n25

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T065415Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| default_relink | 107.94 ms | 2.13x |
| default_replay | 50.59 ms | 1.00x |

- [20261118T065415Z-07a9e9a3](link-static/dev_n25/linux-x86_64-c16-bare-intel/20261118T065415Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T083526Z-b2bbf4ba`

| variant | median | vs best |
|---|---|---|
| default_relink | 148.57 ms | 2.57x |
| default_replay | 57.84 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/dev_n25/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)

## link-static/dev_n97

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T083526Z-b2bbf4ba`

| variant | median | vs best |
|---|---|---|
| default_relink | 524.41 ms | 1.64x |
| default_replay | 319.64 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/dev_n97/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)

## link-static/release_n25

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T065415Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| default_relink | 103.15 ms | 2.16x |
| default_replay | 47.66 ms | 1.00x |

- [20261118T065415Z-07a9e9a3](link-static/release_n25/linux-x86_64-c16-bare-intel/20261118T065415Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T083526Z-b2bbf4ba`

| variant | median | vs best |
|---|---|---|
| default_relink | 137.90 ms | 2.21x |
| default_replay | 62.37 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/release_n25/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)

## load_alignment/load_alignment_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T145408Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 123 ns | 1.00x |
| load_byte_pack | 170 ns | 1.38x |
| load_unaligned | 150 ns | 1.22x |

- [20261115T054506Z-266650fb-dirty](load_alignment/load_alignment_n1024/linux-x86_64-c16-bare-intel/20261115T054506Z-266650fb-dirty/report.md)
- [20261117T145408Z-07a9e9a3](load_alignment/load_alignment_n1024/linux-x86_64-c16-bare-intel/20261117T145408Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211354Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 113 ns | 1.00x |
| load_byte_pack | 115 ns | 1.02x |
| load_unaligned | 121 ns | 1.07x |

- [20260511T001004Z-1fc2e8d](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20260511T001004Z-1fc2e8d/report.md)
- [20261114T211354Z-266650fb-dirty](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20261114T211354Z-266650fb-dirty/report.md)

## load_alignment/load_alignment_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T145408Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 1.87 us | 1.00x |
| load_byte_pack | 1.87 us | 1.00x |
| load_unaligned | 1.88 us | 1.00x |

- [20261115T055010Z-266650fb-dirty](load_alignment/load_alignment_n16384/linux-x86_64-c16-bare-intel/20261115T055010Z-266650fb-dirty/report.md)
- [20261117T145408Z-07a9e9a3](load_alignment/load_alignment_n16384/linux-x86_64-c16-bare-intel/20261117T145408Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211359Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 2.61 us | 1.00x |
| load_byte_pack | 2.65 us | 1.02x |
| load_unaligned | 2.69 us | 1.03x |

- [20260511T001006Z-1fc2e8d](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20260511T001006Z-1fc2e8d/report.md)
- [20261114T211359Z-266650fb-dirty](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20261114T211359Z-266650fb-dirty/report.md)

## load_alignment/load_alignment_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T145408Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 35 ns | 1.00x |
| load_byte_pack | 43 ns | 1.22x |
| load_unaligned | 43 ns | 1.22x |

- [20261115T054231Z-266650fb-dirty](load_alignment/load_alignment_n256/linux-x86_64-c16-bare-intel/20261115T054231Z-266650fb-dirty/report.md)
- [20261117T145408Z-07a9e9a3](load_alignment/load_alignment_n256/linux-x86_64-c16-bare-intel/20261117T145408Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211352Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 15 ns | 1.00x |
| load_byte_pack | 16 ns | 1.03x |
| load_unaligned | 15 ns | 1.00x |

- [20260511T001003Z-1fc2e8d](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20260511T001003Z-1fc2e8d/report.md)
- [20261114T211352Z-266650fb-dirty](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20261114T211352Z-266650fb-dirty/report.md)

## load_alignment/load_alignment_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T145408Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 484 ns | 1.00x |
| load_byte_pack | 577 ns | 1.19x |
| load_unaligned | 517 ns | 1.07x |

- [20261115T054738Z-266650fb-dirty](load_alignment/load_alignment_n4096/linux-x86_64-c16-bare-intel/20261115T054738Z-266650fb-dirty/report.md)
- [20261117T145408Z-07a9e9a3](load_alignment/load_alignment_n4096/linux-x86_64-c16-bare-intel/20261117T145408Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211357Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 616 ns | 1.00x |
| load_byte_pack | 631 ns | 1.02x |
| load_unaligned | 630 ns | 1.02x |

- [20260511T001005Z-1fc2e8d](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20260511T001005Z-1fc2e8d/report.md)
- [20261114T211357Z-266650fb-dirty](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20261114T211357Z-266650fb-dirty/report.md)

## match_arm_count/match_arm_count_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T150457Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 105 ns | 1.00x |
| match_64_arms | 181 ns | 1.73x |
| match_8_arms | 134 ns | 1.27x |

- [20261115T055547Z-266650fb-dirty](match_arm_count/match_arm_count_n1024/linux-x86_64-c16-bare-intel/20261115T055547Z-266650fb-dirty/report.md)
- [20261117T150457Z-07a9e9a3](match_arm_count/match_arm_count_n1024/linux-x86_64-c16-bare-intel/20261117T150457Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211437Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 107 ns | 1.00x |
| match_64_arms | 249 ns | 2.33x |
| match_8_arms | 169 ns | 1.58x |

- [20260511T035129Z-4802e6f](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20260511T035129Z-4802e6f/report.md)
- [20261114T211437Z-266650fb-dirty](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20261114T211437Z-266650fb-dirty/report.md)

## match_arm_count/match_arm_count_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T150457Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 1.30 us | 1.00x |
| match_64_arms | 9.59 us | 7.36x |
| match_8_arms | 11.68 us | 8.95x |

- [20261115T060052Z-266650fb-dirty](match_arm_count/match_arm_count_n16384/linux-x86_64-c16-bare-intel/20261115T060052Z-266650fb-dirty/report.md)
- [20261117T150457Z-07a9e9a3](match_arm_count/match_arm_count_n16384/linux-x86_64-c16-bare-intel/20261117T150457Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211443Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 1.40 us | 1.00x |
| match_64_arms | 12.03 us | 8.62x |
| match_8_arms | 2.65 us | 1.89x |

- [20260511T035132Z-4802e6f](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20260511T035132Z-4802e6f/report.md)
- [20261114T211443Z-266650fb-dirty](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20261114T211443Z-266650fb-dirty/report.md)

## match_arm_count/match_arm_count_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T150457Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 33 ns | 1.18x |
| match_64_arms | 39 ns | 1.41x |
| match_8_arms | 28 ns | 1.00x |

- [20261115T055312Z-266650fb-dirty](match_arm_count/match_arm_count_n256/linux-x86_64-c16-bare-intel/20261115T055312Z-266650fb-dirty/report.md)
- [20261117T150457Z-07a9e9a3](match_arm_count/match_arm_count_n256/linux-x86_64-c16-bare-intel/20261117T150457Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211435Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 28 ns | 1.00x |
| match_64_arms | 76 ns | 2.72x |
| match_8_arms | 100 ns | 3.58x |

- [20260511T035128Z-4802e6f](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20260511T035128Z-4802e6f/report.md)
- [20261114T211435Z-266650fb-dirty](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20261114T211435Z-266650fb-dirty/report.md)

## match_arm_count/match_arm_count_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T150457Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 396 ns | 1.00x |
| match_64_arms | 746 ns | 1.88x |
| match_8_arms | 666 ns | 1.68x |

- [20261115T055819Z-266650fb-dirty](match_arm_count/match_arm_count_n4096/linux-x86_64-c16-bare-intel/20261115T055819Z-266650fb-dirty/report.md)
- [20261117T150457Z-07a9e9a3](match_arm_count/match_arm_count_n4096/linux-x86_64-c16-bare-intel/20261117T150457Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211439Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 371 ns | 1.00x |
| match_64_arms | 941 ns | 2.54x |
| match_8_arms | 676 ns | 1.82x |

- [20260511T035130Z-4802e6f](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20260511T035130Z-4802e6f/report.md)
- [20261114T211439Z-266650fb-dirty](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20261114T211439Z-266650fb-dirty/report.md)

## memcpy_patterns/memcpy_patterns_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T151549Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 1.23 us | 1.00x |
| memcpy_naive | 1.23 us | 1.00x |
| memcpy_unrolled | 1.23 us | 1.00x |

- [20261115T060900Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n1024/linux-x86_64-c16-bare-intel/20261115T060900Z-266650fb-dirty/report.md)
- [20261117T151549Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n1024/linux-x86_64-c16-bare-intel/20261117T151549Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211523Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 1.50 us | 1.02x |
| memcpy_naive | 1.52 us | 1.04x |
| memcpy_unrolled | 1.47 us | 1.00x |

- [20260510T214729Z-f222646](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20260510T214729Z-f222646/report.md)
- [20261114T211523Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20261114T211523Z-266650fb-dirty/report.md)

## memcpy_patterns/memcpy_patterns_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T151549Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 18.27 us | 1.00x |
| memcpy_naive | 18.27 us | 1.00x |
| memcpy_unrolled | 18.27 us | 1.00x |

- [20261115T061407Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n16384/linux-x86_64-c16-bare-intel/20261115T061407Z-266650fb-dirty/report.md)
- [20261117T151549Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n16384/linux-x86_64-c16-bare-intel/20261117T151549Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211530Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 20.45 us | 1.00x |
| memcpy_naive | 20.47 us | 1.00x |
| memcpy_unrolled | 20.47 us | 1.00x |

- [20260510T214734Z-f222646](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20260510T214734Z-f222646/report.md)
- [20261114T211530Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20261114T211530Z-266650fb-dirty/report.md)

## memcpy_patterns/memcpy_patterns_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T151549Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 376 ns | 1.00x |
| memcpy_naive | 375 ns | 1.00x |
| memcpy_unrolled | 376 ns | 1.00x |

- [20261115T060629Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n256/linux-x86_64-c16-bare-intel/20261115T060629Z-266650fb-dirty/report.md)
- [20261117T151549Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n256/linux-x86_64-c16-bare-intel/20261117T151549Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211521Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 488 ns | 1.00x |
| memcpy_naive | 491 ns | 1.01x |
| memcpy_unrolled | 491 ns | 1.01x |

- [20260510T214728Z-f222646](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20260510T214728Z-f222646/report.md)
- [20261114T211521Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20261114T211521Z-266650fb-dirty/report.md)

## memcpy_patterns/memcpy_patterns_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T151549Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 4.35 us | 1.02x |
| memcpy_naive | 4.25 us | 1.00x |
| memcpy_unrolled | 4.66 us | 1.10x |

- [20261115T061133Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n4096/linux-x86_64-c16-bare-intel/20261115T061133Z-266650fb-dirty/report.md)
- [20261117T151549Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n4096/linux-x86_64-c16-bare-intel/20261117T151549Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211526Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 5.62 us | 1.02x |
| memcpy_naive | 5.63 us | 1.02x |
| memcpy_unrolled | 5.50 us | 1.00x |

- [20260510T214730Z-f222646](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20260510T214730Z-f222646/report.md)
- [20261114T211526Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20261114T211526Z-266650fb-dirty/report.md)

## memcpy_patterns/memcpy_patterns_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T151549Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 78 ns | 1.22x |
| memcpy_naive | 64 ns | 1.00x |
| memcpy_unrolled | 78 ns | 1.22x |

- [20261115T060354Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n64/linux-x86_64-c16-bare-intel/20261115T060354Z-266650fb-dirty/report.md)
- [20261117T151549Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n64/linux-x86_64-c16-bare-intel/20261117T151549Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211518Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 40 ns | 1.02x |
| memcpy_naive | 40 ns | 1.01x |
| memcpy_unrolled | 39 ns | 1.00x |

- [20260510T214727Z-f222646](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20260510T214727Z-f222646/report.md)
- [20261114T211518Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20261114T211518Z-266650fb-dirty/report.md)

## minmax_strategy/minmax_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T152912Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 52 ns | 1.03x |
| minmax_branchless | 50 ns | 1.00x |
| minmax_intrinsic | 52 ns | 1.03x |

- [20261115T061944Z-266650fb-dirty](minmax_strategy/minmax_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T061944Z-266650fb-dirty/report.md)
- [20261117T152912Z-07a9e9a3](minmax_strategy/minmax_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T152912Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211609Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 42 ns | 1.00x |
| minmax_branchless | 42 ns | 1.01x |
| minmax_intrinsic | 42 ns | 1.00x |

- [20260511T043126Z-da11af0](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T043126Z-da11af0/report.md)
- [20261114T211609Z-266650fb-dirty](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211609Z-266650fb-dirty/report.md)

## minmax_strategy/minmax_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T152912Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 675 ns | 1.02x |
| minmax_branchless | 664 ns | 1.00x |
| minmax_intrinsic | 708 ns | 1.07x |

- [20261115T062448Z-266650fb-dirty](minmax_strategy/minmax_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T062448Z-266650fb-dirty/report.md)
- [20261117T152912Z-07a9e9a3](minmax_strategy/minmax_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T152912Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211613Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 401 ns | 1.00x |
| minmax_branchless | 411 ns | 1.03x |
| minmax_intrinsic | 407 ns | 1.02x |

- [20260511T043128Z-da11af0](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T043128Z-da11af0/report.md)
- [20261114T211613Z-266650fb-dirty](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211613Z-266650fb-dirty/report.md)

## minmax_strategy/minmax_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T152912Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 34 ns | 1.21x |
| minmax_branchless | 28 ns | 1.00x |
| minmax_intrinsic | 34 ns | 1.22x |

- [20261115T061709Z-266650fb-dirty](minmax_strategy/minmax_strategy_n256/linux-x86_64-c16-bare-intel/20261115T061709Z-266650fb-dirty/report.md)
- [20261117T152912Z-07a9e9a3](minmax_strategy/minmax_strategy_n256/linux-x86_64-c16-bare-intel/20261117T152912Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211607Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 24 ns | 1.08x |
| minmax_branchless | 24 ns | 1.05x |
| minmax_intrinsic | 22 ns | 1.00x |

- [20260511T043125Z-da11af0](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20260511T043125Z-da11af0/report.md)
- [20261114T211607Z-266650fb-dirty](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211607Z-266650fb-dirty/report.md)

## minmax_strategy/minmax_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T152912Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 185 ns | 1.04x |
| minmax_branchless | 179 ns | 1.00x |
| minmax_intrinsic | 192 ns | 1.07x |

- [20261115T062215Z-266650fb-dirty](minmax_strategy/minmax_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T062215Z-266650fb-dirty/report.md)
- [20261117T152912Z-07a9e9a3](minmax_strategy/minmax_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T152912Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211611Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 117 ns | 1.00x |
| minmax_branchless | 123 ns | 1.05x |
| minmax_intrinsic | 117 ns | 1.00x |

- [20260511T043127Z-da11af0](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T043127Z-da11af0/report.md)
- [20261114T211611Z-266650fb-dirty](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211611Z-266650fb-dirty/report.md)

## modulo_strategy/modulo_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T154001Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 2.93 us | 2.41x |
| mod_pow2_const | 1.22 us | 1.00x |
| mod_var | 6.28 us | 5.15x |

- [20261115T063029Z-266650fb-dirty](modulo_strategy/modulo_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T063029Z-266650fb-dirty/report.md)
- [20261117T154001Z-07a9e9a3](modulo_strategy/modulo_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T154001Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211652Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 3.71 us | 2.24x |
| mod_pow2_const | 1.66 us | 1.00x |
| mod_var | 4.50 us | 2.71x |

- [20260510T234953Z-4ccf554](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T234953Z-4ccf554/report.md)
- [20261114T211652Z-266650fb-dirty](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211652Z-266650fb-dirty/report.md)

## modulo_strategy/modulo_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T154001Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 57.06 us | 3.59x |
| mod_pow2_const | 15.88 us | 1.00x |
| mod_var | 100.43 us | 6.32x |

- [20261115T063541Z-266650fb-dirty](modulo_strategy/modulo_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T063541Z-266650fb-dirty/report.md)
- [20261117T154001Z-07a9e9a3](modulo_strategy/modulo_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T154001Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211705Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 56.44 us | 2.21x |
| mod_pow2_const | 25.59 us | 1.00x |
| mod_var | 71.63 us | 2.80x |

- [20260510T235003Z-4ccf554](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T235003Z-4ccf554/report.md)
- [20261114T211705Z-266650fb-dirty](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211705Z-266650fb-dirty/report.md)

## modulo_strategy/modulo_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T154001Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 740 ns | 2.92x |
| mod_pow2_const | 254 ns | 1.00x |
| mod_var | 1.58 us | 6.22x |

- [20261115T062754Z-266650fb-dirty](modulo_strategy/modulo_strategy_n256/linux-x86_64-c16-bare-intel/20261115T062754Z-266650fb-dirty/report.md)
- [20261117T154001Z-07a9e9a3](modulo_strategy/modulo_strategy_n256/linux-x86_64-c16-bare-intel/20261117T154001Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211650Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 877 ns | 2.37x |
| mod_pow2_const | 370 ns | 1.00x |
| mod_var | 1.08 us | 2.92x |

- [20260510T234952Z-4ccf554](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20260510T234952Z-4ccf554/report.md)
- [20261114T211650Z-266650fb-dirty](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211650Z-266650fb-dirty/report.md)

## modulo_strategy/modulo_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T154001Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 13.33 us | 2.75x |
| mod_pow2_const | 4.85 us | 1.00x |
| mod_var | 25.12 us | 5.18x |

- [20261115T063303Z-266650fb-dirty](modulo_strategy/modulo_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T063303Z-266650fb-dirty/report.md)
- [20261117T154001Z-07a9e9a3](modulo_strategy/modulo_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T154001Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211656Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 14.32 us | 2.18x |
| mod_pow2_const | 6.57 us | 1.00x |
| mod_var | 18.17 us | 2.77x |

- [20260510T234956Z-4ccf554](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T234956Z-4ccf554/report.md)
- [20261114T211656Z-266650fb-dirty](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211656Z-266650fb-dirty/report.md)

## popcount_strategy/popcount_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T155057Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 1.51 us | 16.27x |
| popcount_std | 93 ns | 1.00x |
| popcount_swar | 188 ns | 2.02x |

- [20261115T064350Z-266650fb-dirty](popcount_strategy/popcount_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T064350Z-266650fb-dirty/report.md)
- [20261117T155057Z-07a9e9a3](popcount_strategy/popcount_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T155057Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211746Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 158 ns | 4.47x |
| popcount_std | 35 ns | 1.00x |
| popcount_swar | 155 ns | 4.38x |

- [20260510T213414Z-b042648](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213414Z-b042648/report.md)
- [20261114T211746Z-266650fb-dirty](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211746Z-266650fb-dirty/report.md)

## popcount_strategy/popcount_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T155057Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 37.20 us | 28.97x |
| popcount_std | 1.28 us | 1.00x |
| popcount_swar | 2.72 us | 2.12x |

- [20261115T064856Z-266650fb-dirty](popcount_strategy/popcount_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T064856Z-266650fb-dirty/report.md)
- [20261117T155057Z-07a9e9a3](popcount_strategy/popcount_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T155057Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211751Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 2.08 us | 7.20x |
| popcount_std | 289 ns | 1.00x |
| popcount_swar | 2.03 us | 7.02x |

- [20260510T213418Z-b042648](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213418Z-b042648/report.md)
- [20261114T211751Z-266650fb-dirty](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211751Z-266650fb-dirty/report.md)

## popcount_strategy/popcount_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T155057Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 342 ns | 10.50x |
| popcount_std | 33 ns | 1.00x |
| popcount_swar | 67 ns | 2.06x |

- [20261115T064118Z-266650fb-dirty](popcount_strategy/popcount_strategy_n256/linux-x86_64-c16-bare-intel/20261115T064118Z-266650fb-dirty/report.md)
- [20261117T155057Z-07a9e9a3](popcount_strategy/popcount_strategy_n256/linux-x86_64-c16-bare-intel/20261117T155057Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211744Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 42 ns | 10.02x |
| popcount_std | 4 ns | 1.00x |
| popcount_swar | 41 ns | 9.67x |

- [20260510T213413Z-b042648](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213413Z-b042648/report.md)
- [20261114T211744Z-266650fb-dirty](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211744Z-266650fb-dirty/report.md)

## popcount_strategy/popcount_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T155057Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 8.04 us | 24.28x |
| popcount_std | 331 ns | 1.00x |
| popcount_swar | 692 ns | 2.09x |

- [20261115T064622Z-266650fb-dirty](popcount_strategy/popcount_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T064622Z-266650fb-dirty/report.md)
- [20261117T155057Z-07a9e9a3](popcount_strategy/popcount_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T155057Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211749Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 530 ns | 5.74x |
| popcount_std | 92 ns | 1.00x |
| popcount_swar | 526 ns | 5.70x |

- [20260510T213416Z-b042648](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213416Z-b042648/report.md)
- [20261114T211749Z-266650fb-dirty](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211749Z-266650fb-dirty/report.md)

## popcount_strategy/popcount_strategy_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T155057Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 91 ns | 5.95x |
| popcount_std | 15 ns | 1.00x |
| popcount_swar | 22 ns | 1.41x |

- [20261115T063843Z-266650fb-dirty](popcount_strategy/popcount_strategy_n64/linux-x86_64-c16-bare-intel/20261115T063843Z-266650fb-dirty/report.md)
- [20261117T155057Z-07a9e9a3](popcount_strategy/popcount_strategy_n64/linux-x86_64-c16-bare-intel/20261117T155057Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211742Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 5 ns | 12.50x |
| popcount_std | 0 ns | 1.00x |
| popcount_swar | 5 ns | 12.50x |

- [20260510T213412Z-b042648](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213412Z-b042648/report.md)
- [20261114T211742Z-266650fb-dirty](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211742Z-266650fb-dirty/report.md)

## rcm_order/rcm_order_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 4.78 ms | 1.03x |
| rcm_half | 4.95 ms | 1.06x |
| rcm_rev | 4.67 ms | 1.00x |
| rcm_scr | 4.89 ms | 1.05x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n1024/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T150703Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 2.24 ms | 1.04x |
| rcm_half | 2.27 ms | 1.06x |
| rcm_rev | 2.15 ms | 1.00x |
| rcm_scr | 2.34 ms | 1.09x |

- [20260719T150703Z-2c337b67-dirty](rcm_order/rcm_order_n1024/macos-aarch64-c8-bare-apple/20260719T150703Z-2c337b67-dirty/report.md)

## rcm_order/rcm_order_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 75.51 ms | 1.00x |
| rcm_half | 75.61 ms | 1.00x |
| rcm_rev | 75.54 ms | 1.00x |
| rcm_scr | 75.91 ms | 1.01x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n16384/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T160923Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 37.54 ms | 1.01x |
| rcm_half | 37.11 ms | 1.00x |
| rcm_rev | 40.77 ms | 1.10x |
| rcm_scr | 37.79 ms | 1.02x |

- [20260719T160923Z-2c337b67-dirty](rcm_order/rcm_order_n16384/macos-aarch64-c8-bare-apple/20260719T160923Z-2c337b67-dirty/report.md)

## rcm_order/rcm_order_n2048

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 9.63 ms | 1.01x |
| rcm_half | 9.56 ms | 1.00x |
| rcm_rev | 9.53 ms | 1.00x |
| rcm_scr | 9.56 ms | 1.00x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n2048/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T151110Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 4.54 ms | 1.00x |
| rcm_half | 4.76 ms | 1.05x |
| rcm_rev | 4.54 ms | 1.00x |
| rcm_scr | 4.62 ms | 1.02x |

- [20260719T151110Z-2c337b67-dirty](rcm_order/rcm_order_n2048/macos-aarch64-c8-bare-apple/20260719T151110Z-2c337b67-dirty/report.md)

## rcm_order/rcm_order_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 998.83 us | 1.00x |
| rcm_half | 1.01 ms | 1.01x |
| rcm_rev | 1.04 ms | 1.04x |
| rcm_scr | 1.06 ms | 1.06x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n256/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T150459Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 394.46 us | 1.05x |
| rcm_half | 412.73 us | 1.10x |
| rcm_rev | 374.54 us | 1.00x |
| rcm_scr | 400.38 us | 1.07x |

- [20260719T150459Z-2c337b67-dirty](rcm_order/rcm_order_n256/macos-aarch64-c8-bare-apple/20260719T150459Z-2c337b67-dirty/report.md)

## rcm_order/rcm_order_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 19.35 ms | 1.01x |
| rcm_half | 19.12 ms | 1.00x |
| rcm_rev | 19.13 ms | 1.00x |
| rcm_scr | 19.09 ms | 1.00x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n4096/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T151919Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 9.23 ms | 1.04x |
| rcm_half | 8.86 ms | 1.00x |
| rcm_rev | 8.99 ms | 1.01x |
| rcm_scr | 9.01 ms | 1.02x |

- [20260719T151919Z-2c337b67-dirty](rcm_order/rcm_order_n4096/macos-aarch64-c8-bare-apple/20260719T151919Z-2c337b67-dirty/report.md)

## rcm_order/rcm_order_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 125.81 us | 1.01x |
| rcm_half | 128.84 us | 1.03x |
| rcm_rev | 124.70 us | 1.00x |
| rcm_scr | 134.84 us | 1.08x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n64/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T150428Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 102.92 us | 1.08x |
| rcm_half | 95.41 us | 1.00x |
| rcm_rev | 97.32 us | 1.02x |
| rcm_scr | 99.14 us | 1.04x |

- [20260719T150428Z-2c337b67-dirty](rcm_order/rcm_order_n64/macos-aarch64-c8-bare-apple/20260719T150428Z-2c337b67-dirty/report.md)

## rcm_order/rcm_order_n8192

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T160446Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 37.75 ms | 1.01x |
| rcm_half | 37.79 ms | 1.01x |
| rcm_rev | 37.63 ms | 1.00x |
| rcm_scr | 37.54 ms | 1.00x |

- [20261117T160446Z-07a9e9a3](rcm_order/rcm_order_n8192/linux-x86_64-c16-bare-intel/20261117T160446Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T153530Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 18.29 ms | 1.00x |
| rcm_half | 18.57 ms | 1.02x |
| rcm_rev | 18.99 ms | 1.04x |
| rcm_scr | 18.26 ms | 1.00x |

- [20260719T153530Z-2c337b67-dirty](rcm_order/rcm_order_n8192/macos-aarch64-c8-bare-apple/20260719T153530Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 9.73 ms | 1.03x |
| riv_rowm | 9.47 ms | 1.00x |
| riv_scr | 9.55 ms | 1.01x |
| riv_snake | 9.45 ms | 1.00x |
| riv_spec | 9.53 ms | 1.01x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n1024/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T161426Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 3.86 ms | 1.03x |
| riv_rowm | 3.76 ms | 1.00x |
| riv_scr | 3.95 ms | 1.05x |
| riv_snake | 3.80 ms | 1.01x |
| riv_spec | 3.77 ms | 1.00x |

- [20260719T161426Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n1024/macos-aarch64-c8-bare-apple/20260719T161426Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 149.46 ms | 1.00x |
| riv_rowm | 148.91 ms | 1.00x |
| riv_scr | 149.05 ms | 1.00x |
| riv_snake | 149.16 ms | 1.00x |
| riv_spec | 149.20 ms | 1.00x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n16384/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T181527Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 106.11 ms | 1.00x |
| riv_rowm | 176.58 ms | 1.66x |
| riv_scr | 110.35 ms | 1.04x |
| riv_snake | 108.51 ms | 1.02x |
| riv_spec | 124.67 ms | 1.17x |

- [20260719T181527Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n16384/macos-aarch64-c8-bare-apple/20260719T181527Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n2048

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 19.08 ms | 1.02x |
| riv_rowm | 18.68 ms | 1.00x |
| riv_scr | 19.13 ms | 1.02x |
| riv_snake | 18.82 ms | 1.01x |
| riv_spec | 18.97 ms | 1.02x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n2048/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20260719T162211Z-2c337b67`

| variant | median | vs best |
|---|---|---|
| riv_scr | 8.27 ms | 1.00x |
| riv_snake | 8.31 ms | 1.00x |
| riv_spec | 8.34 ms | 1.01x |

- [20260719T162210Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n2048/macos-aarch64-c8-bare-apple/20260719T162210Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 2.58 ms | 1.10x |
| riv_rowm | 2.35 ms | 1.00x |
| riv_scr | 2.44 ms | 1.04x |
| riv_snake | 2.40 ms | 1.02x |
| riv_spec | 2.53 ms | 1.08x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n256/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T161037Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 876.86 us | 1.00x |
| riv_rowm | 907.75 us | 1.04x |
| riv_scr | 920.12 us | 1.05x |
| riv_snake | 872.60 us | 1.00x |
| riv_spec | 875.08 us | 1.00x |

- [20260719T161037Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n256/macos-aarch64-c8-bare-apple/20260719T161037Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 37.62 ms | 1.01x |
| riv_rowm | 37.55 ms | 1.00x |
| riv_scr | 37.53 ms | 1.00x |
| riv_snake | 37.48 ms | 1.00x |
| riv_spec | 37.41 ms | 1.00x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n4096/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T163719Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 16.03 ms | 1.00x |
| riv_rowm | 16.10 ms | 1.00x |
| riv_scr | 16.26 ms | 1.01x |
| riv_snake | 16.10 ms | 1.00x |
| riv_spec | 16.16 ms | 1.01x |

- [20260719T163719Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n4096/macos-aarch64-c8-bare-apple/20260719T163719Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 459.32 us | 1.00x |
| riv_rowm | 462.98 us | 1.01x |
| riv_scr | 466.15 us | 1.01x |
| riv_snake | 463.91 us | 1.01x |
| riv_spec | 460.51 us | 1.00x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n64/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T160939Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 190.89 us | 1.01x |
| riv_rowm | 195.05 us | 1.04x |
| riv_scr | 191.46 us | 1.02x |
| riv_snake | 189.53 us | 1.01x |
| riv_spec | 188.20 us | 1.00x |

- [20260719T160939Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n64/macos-aarch64-c8-bare-apple/20260719T160939Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n8192

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T171201Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 74.55 ms | 1.00x |
| riv_rowm | 75.12 ms | 1.01x |
| riv_scr | 74.64 ms | 1.00x |
| riv_snake | 74.56 ms | 1.00x |
| riv_spec | 74.91 ms | 1.00x |

- [20261117T171201Z-07a9e9a3](rcm_rivals/rcm_rivals_n8192/linux-x86_64-c16-bare-intel/20261117T171201Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20260719T170734Z-2c337b67-dirty`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 31.88 ms | 1.01x |
| riv_rowm | 31.72 ms | 1.00x |
| riv_scr | 31.80 ms | 1.00x |
| riv_snake | 31.87 ms | 1.00x |
| riv_spec | 31.92 ms | 1.01x |

- [20260719T170734Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n8192/macos-aarch64-c8-bare-apple/20260719T170734Z-2c337b67-dirty/report.md)

## registration_binding/registration_binding_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T190802Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 3.10 us | 1.22x |
| reg_bound_record | 22.14 us | 8.72x |
| reg_bound_strided | 5.17 us | 2.04x |
| reg_bound_units | 2.58 us | 1.02x |
| reg_typed | 2.57 us | 1.01x |
| reg_typed_twin | 2.54 us | 1.00x |

- [20261115T090031Z-266650fb-dirty](registration_binding/registration_binding_n16384/linux-x86_64-c16-bare-intel/20261115T090031Z-266650fb-dirty/report.md)
- [20261117T190802Z-07a9e9a3](registration_binding/registration_binding_n16384/linux-x86_64-c16-bare-intel/20261117T190802Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T080428Z-83f6f25-dirty](registration_binding/registration_binding_n16384/linux-x86_64-c4-container-intel/20261113T080428Z-83f6f25-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T231941Z-266650fb`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 848 ns | 1.00x |
| reg_bound_record | 21.88 us | 25.89x |
| reg_bound_strided | 4.97 us | 5.88x |
| reg_bound_units | 863 ns | 1.02x |
| reg_typed | 845 ns | 1.00x |
| reg_typed_twin | 845 ns | 1.00x |

- [20261114T231940Z-266650fb-dirty](registration_binding/registration_binding_n16384/macos-aarch64-c8-bare-apple/20261114T231940Z-266650fb-dirty/report.md)

## registration_binding/registration_binding_n262144

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T190802Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 48.33 us | 1.22x |
| reg_bound_record | 293.26 us | 7.43x |
| reg_bound_strided | 72.20 us | 1.83x |
| reg_bound_units | 39.72 us | 1.01x |
| reg_typed | 39.49 us | 1.00x |
| reg_typed_twin | 47.67 us | 1.21x |

- [20261115T091356Z-266650fb-dirty](registration_binding/registration_binding_n262144/linux-x86_64-c16-bare-intel/20261115T091356Z-266650fb-dirty/report.md)
- [20261117T190802Z-07a9e9a3](registration_binding/registration_binding_n262144/linux-x86_64-c16-bare-intel/20261117T190802Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T081835Z-83f6f25-dirty](registration_binding/registration_binding_n262144/linux-x86_64-c4-container-intel/20261113T081835Z-83f6f25-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T232255Z-266650fb`

| variant | median | vs best |
|---|---|---|
| reg_bound_record | 352.85 us | 21.32x |
| reg_bound_strided | 79.58 us | 4.81x |
| reg_bound_units | 17.69 us | 1.07x |
| reg_typed | 16.56 us | 1.00x |
| reg_typed_twin | 16.55 us | 1.00x |

- [20261114T232254Z-266650fb-dirty](registration_binding/registration_binding_n262144/macos-aarch64-c8-bare-apple/20261114T232254Z-266650fb-dirty/report.md)

## registration_binding/registration_binding_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T190802Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 719 ns | 1.16x |
| reg_bound_record | 4.60 us | 7.44x |
| reg_bound_strided | 1.14 us | 1.84x |
| reg_bound_units | 626 ns | 1.01x |
| reg_typed | 618 ns | 1.00x |
| reg_typed_twin | 622 ns | 1.01x |

- [20261115T085521Z-266650fb-dirty](registration_binding/registration_binding_n4096/linux-x86_64-c16-bare-intel/20261115T085521Z-266650fb-dirty/report.md)
- [20261117T190802Z-07a9e9a3](registration_binding/registration_binding_n4096/linux-x86_64-c16-bare-intel/20261117T190802Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T075919Z-83f6f25-dirty](registration_binding/registration_binding_n4096/linux-x86_64-c4-container-intel/20261113T075919Z-83f6f25-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T231928Z-266650fb`

| variant | median | vs best |
|---|---|---|
| reg_bound_strided | 1.27 us | 5.86x |
| reg_bound_units | 217 ns | 1.00x |
| reg_typed | 236 ns | 1.09x |
| reg_typed_twin | 233 ns | 1.07x |

- [20261114T231927Z-266650fb-dirty](registration_binding/registration_binding_n4096/macos-aarch64-c8-bare-apple/20261114T231927Z-266650fb-dirty/report.md)

## registration_binding/registration_binding_n65536

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T190802Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 11.97 us | 1.00x |
| reg_bound_record | 89.02 us | 7.47x |
| reg_bound_strided | 22.04 us | 1.85x |
| reg_bound_units | 12.58 us | 1.06x |
| reg_typed | 12.36 us | 1.04x |
| reg_typed_twin | 11.92 us | 1.00x |

- [20261115T090613Z-266650fb-dirty](registration_binding/registration_binding_n65536/linux-x86_64-c16-bare-intel/20261115T090613Z-266650fb-dirty/report.md)
- [20261117T190802Z-07a9e9a3](registration_binding/registration_binding_n65536/linux-x86_64-c16-bare-intel/20261117T190802Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T081013Z-83f6f25-dirty](registration_binding/registration_binding_n65536/linux-x86_64-c4-container-intel/20261113T081013Z-83f6f25-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T232023Z-266650fb`

| variant | median | vs best |
|---|---|---|
| reg_typed_twin | 4.36 us | 1.00x |

- [20261114T232022Z-266650fb-dirty](registration_binding/registration_binding_n65536/macos-aarch64-c8-bare-apple/20261114T232022Z-266650fb-dirty/report.md)

## resource_storage/false_sharing_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fs_packed | 1.63 ms | 1.38x |
| fs_padded | 1.18 ms | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/false_sharing_n1024/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

## resource_storage/false_sharing_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fs_packed | 7.20 ms | 1.30x |
| fs_padded | 5.53 ms | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/false_sharing_n16384/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

## resource_storage/false_sharing_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fs_packed | 23.43 ms | 1.27x |
| fs_padded | 18.44 ms | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/false_sharing_n65536/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

## resource_storage/rsb_bwide_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wide_blob | 10.75 us | 1.00x |
| wide_decomposed | 11.63 us | 1.08x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_bwide_n16384/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182437Z-9d530d12](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20260619T182437Z-9d530d12/report.md)
- [20260702T102749Z-677b60f8](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20260702T102749Z-677b60f8/report.md)

## resource_storage/rsb_bwide_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wide_blob | 53.81 us | 1.13x |
| wide_decomposed | 47.64 us | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_bwide_n256/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182434Z-9d530d12](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20260619T182434Z-9d530d12/report.md)
- [20260702T102747Z-677b60f8](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20260702T102747Z-677b60f8/report.md)

## resource_storage/rsb_bwide_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wide_blob | 183 ns | 1.00x |
| wide_decomposed | 192 ns | 1.05x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_bwide_n4194304/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182613Z-9d530d12](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20260619T182613Z-9d530d12/report.md)
- [20260702T102929Z-677b60f8](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20260702T102929Z-677b60f8/report.md)

## resource_storage/rsb_clean_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 118 ns | 1.01x |
| v1_snapshot | 118 ns | 1.01x |
| v2_decomposed | 117 ns | 1.01x |
| v3_shapebound | 116 ns | 1.00x |
| v4_erased | 118 ns | 1.01x |
| v5_handletable | 117 ns | 1.01x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_clean_n1024/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183622Z-9d530d12](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20260619T183622Z-9d530d12/report.md)
- [20260702T103426Z-677b60f8](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20260702T103426Z-677b60f8/report.md)

## resource_storage/rsb_clean_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 1.63 us | 1.22x |
| v1_snapshot | 1.35 us | 1.00x |
| v2_decomposed | 1.64 us | 1.22x |
| v3_shapebound | 1.35 us | 1.01x |
| v4_erased | 1.64 us | 1.22x |
| v5_handletable | 1.34 us | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_clean_n16384/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183639Z-9d530d12](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20260619T183639Z-9d530d12/report.md)
- [20260702T103444Z-677b60f8](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20260702T103444Z-677b60f8/report.md)

## resource_storage/rsb_clean_n262144

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 25.92 us | 1.00x |
| v1_snapshot | 25.94 us | 1.00x |
| v2_decomposed | 25.98 us | 1.00x |
| v3_shapebound | 25.96 us | 1.00x |
| v4_erased | 25.94 us | 1.00x |
| v5_handletable | 25.94 us | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_clean_n262144/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T184039Z-9d530d12](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20260619T184039Z-9d530d12/report.md)
- [20260702T103856Z-a2410c71](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20260702T103856Z-a2410c71/report.md)

## resource_storage/rsb_heavy_thrash_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 118 ns | 1.23x |
| v1_snapshot | 117 ns | 1.22x |
| v2_decomposed | 96 ns | 1.01x |
| v3_shapebound | 96 ns | 1.00x |
| v4_erased | 117 ns | 1.22x |
| v5_handletable | 118 ns | 1.23x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_heavy_thrash_n1024/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183150Z-9d530d12](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20260619T183150Z-9d530d12/report.md)
- [20260702T102934Z-677b60f8](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20260702T102934Z-677b60f8/report.md)

## resource_storage/rsb_heavy_thrash_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 1.35 us | 1.01x |
| v1_snapshot | 1.64 us | 1.22x |
| v2_decomposed | 1.35 us | 1.00x |
| v3_shapebound | 1.35 us | 1.00x |
| v4_erased | 1.35 us | 1.00x |
| v5_handletable | 1.64 us | 1.22x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_heavy_thrash_n16384/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183213Z-9d530d12](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20260619T183213Z-9d530d12/report.md)
- [20260702T102953Z-677b60f8](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20260702T102953Z-677b60f8/report.md)

## resource_storage/rsb_heavy_thrash_n262144

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 25.94 us | 1.04x |
| v1_snapshot | 25.92 us | 1.04x |
| v2_decomposed | 25.96 us | 1.04x |
| v3_shapebound | 25.95 us | 1.04x |
| v4_erased | 25.00 us | 1.00x |
| v5_handletable | 25.94 us | 1.04x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_heavy_thrash_n262144/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183619Z-9d530d12](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20260619T183619Z-9d530d12/report.md)
- [20260702T103423Z-677b60f8](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20260702T103423Z-677b60f8/report.md)

## resource_storage/rsb_light_thrash_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 118 ns | 1.21x |
| v1_snapshot | 117 ns | 1.20x |
| v2_decomposed | 117 ns | 1.21x |
| v3_shapebound | 117 ns | 1.20x |
| v4_erased | 97 ns | 1.00x |
| v5_handletable | 118 ns | 1.22x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_light_thrash_n1024/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182616Z-9d530d12](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20260619T182616Z-9d530d12/report.md)
- [20260702T102311Z-677b60f8](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20260702T102311Z-677b60f8/report.md)

## resource_storage/rsb_light_thrash_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 1.64 us | 1.22x |
| v1_snapshot | 1.64 us | 1.22x |
| v2_decomposed | 1.64 us | 1.22x |
| v3_shapebound | 1.35 us | 1.01x |
| v4_erased | 1.64 us | 1.22x |
| v5_handletable | 1.34 us | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_light_thrash_n16384/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182633Z-9d530d12](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20260619T182633Z-9d530d12/report.md)
- [20260702T102329Z-677b60f8](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20260702T102329Z-677b60f8/report.md)

## resource_storage/rsb_light_thrash_n262144

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 25.95 us | 1.12x |
| v1_snapshot | 25.93 us | 1.12x |
| v2_decomposed | 23.15 us | 1.00x |
| v3_shapebound | 25.96 us | 1.12x |
| v4_erased | 25.93 us | 1.12x |
| v5_handletable | 25.95 us | 1.12x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_light_thrash_n262144/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183144Z-9d530d12](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20260619T183144Z-9d530d12/report.md)
- [20260702T102742Z-677b60f8](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20260702T102742Z-677b60f8/report.md)

## resource_storage/rsb_seqd_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 821.78 us | 1.00x |
| seq_snapshot | 1.49 ms | 1.82x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_seqd_n1048576/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T184658Z-9d530d12](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20260619T184658Z-9d530d12/report.md)
- [20260702T104547Z-a2410c71](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20260702T104547Z-a2410c71/report.md)

## resource_storage/rsb_seqd_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 4.59 ms | 1.00x |
| seq_snapshot | 14.22 ms | 3.10x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_seqd_n16777216/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T202925Z-9d530d12](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20260619T202925Z-9d530d12/report.md)
- [20260702T121947Z-a2410c71](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20260702T121947Z-a2410c71/report.md)

## resource_storage/rsb_seqd_n4194304

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 1.10 ms | 1.00x |
| seq_snapshot | 3.68 ms | 3.34x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_seqd_n4194304/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T190539Z-9d530d12](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20260619T190539Z-9d530d12/report.md)
- [20260702T110508Z-a2410c71](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20260702T110508Z-a2410c71/report.md)

## resource_storage/rsb_seqd_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261117T193232Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 997.20 us | 1.15x |
| seq_snapshot | 866.75 us | 1.00x |

- [20261117T193232Z-07a9e9a3](resource_storage/rsb_seqd_n65536/linux-x86_64-c16-bare-intel/20261117T193232Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T184145Z-9d530d12](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20260619T184145Z-9d530d12/report.md)
- [20260702T104015Z-a2410c71](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20260702T104015Z-a2410c71/report.md)

## rotate_strategy/rotate_strategy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T233420Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 79 ns | 1.00x |
| rot_manual_shifts | 79 ns | 1.00x |
| rot_variable_amount | 220 ns | 2.80x |

- [20261115T091951Z-266650fb-dirty](rotate_strategy/rotate_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T091951Z-266650fb-dirty/report.md)
- [20261117T233420Z-07a9e9a3](rotate_strategy/rotate_strategy_n1024/linux-x86_64-c16-bare-intel/20261117T233420Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232350Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 74 ns | 1.00x |
| rot_manual_shifts | 74 ns | 1.01x |
| rot_variable_amount | 247 ns | 3.34x |

- [20260511T031129Z-cfbc15c](rotate_strategy/rotate_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T031129Z-cfbc15c/report.md)
- [20261114T232350Z-266650fb-dirty](rotate_strategy/rotate_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T232350Z-266650fb-dirty/report.md)

## rotate_strategy/rotate_strategy_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T233420Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 1.15 us | 1.22x |
| rot_manual_shifts | 939 ns | 1.00x |
| rot_variable_amount | 2.80 us | 2.99x |

- [20261115T092455Z-266650fb-dirty](rotate_strategy/rotate_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T092455Z-266650fb-dirty/report.md)
- [20261117T233420Z-07a9e9a3](rotate_strategy/rotate_strategy_n16384/linux-x86_64-c16-bare-intel/20261117T233420Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232355Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 1.28 us | 1.00x |
| rot_manual_shifts | 1.28 us | 1.00x |
| rot_variable_amount | 3.96 us | 3.09x |

- [20260511T031131Z-cfbc15c](rotate_strategy/rotate_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T031131Z-cfbc15c/report.md)
- [20261114T232355Z-266650fb-dirty](rotate_strategy/rotate_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T232355Z-266650fb-dirty/report.md)

## rotate_strategy/rotate_strategy_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T233420Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 21 ns | 1.00x |
| rot_manual_shifts | 21 ns | 1.00x |
| rot_variable_amount | 59 ns | 2.81x |

- [20261115T091717Z-266650fb-dirty](rotate_strategy/rotate_strategy_n256/linux-x86_64-c16-bare-intel/20261115T091717Z-266650fb-dirty/report.md)
- [20261117T233420Z-07a9e9a3](rotate_strategy/rotate_strategy_n256/linux-x86_64-c16-bare-intel/20261117T233420Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232348Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 6 ns | 1.07x |
| rot_manual_shifts | 5 ns | 1.00x |
| rot_variable_amount | 50 ns | 9.30x |

- [20260511T031128Z-cfbc15c](rotate_strategy/rotate_strategy_n256/macos-aarch64-c8-bare-apple/20260511T031128Z-cfbc15c/report.md)
- [20261114T232348Z-266650fb-dirty](rotate_strategy/rotate_strategy_n256/macos-aarch64-c8-bare-apple/20261114T232348Z-266650fb-dirty/report.md)

## rotate_strategy/rotate_strategy_n4096

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261117T233420Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 293 ns | 1.00x |
| rot_manual_shifts | 293 ns | 1.00x |
| rot_variable_amount | 862 ns | 2.94x |

- [20261115T092223Z-266650fb-dirty](rotate_strategy/rotate_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T092223Z-266650fb-dirty/report.md)
- [20261117T233420Z-07a9e9a3](rotate_strategy/rotate_strategy_n4096/linux-x86_64-c16-bare-intel/20261117T233420Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232352Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 314 ns | 1.02x |
| rot_manual_shifts | 309 ns | 1.00x |
| rot_variable_amount | 1.01 us | 3.28x |

- [20260511T031130Z-cfbc15c](rotate_strategy/rotate_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T031130Z-cfbc15c/report.md)
- [20261114T232352Z-266650fb-dirty](rotate_strategy/rotate_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T232352Z-266650fb-dirty/report.md)

## sat_arithmetic/sat_arithmetic_n1024

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261117T234509Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 506 ns | 1.71x |
| sat_saturate | 509 ns | 1.72x |
| sat_wrap | 295 ns | 1.00x |

- [20261115T093032Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n1024/linux-x86_64-c16-bare-intel/20261115T093032Z-266650fb-dirty/report.md)
- [20261117T234509Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n1024/linux-x86_64-c16-bare-intel/20261117T234509Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232433Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 698 ns | 1.92x |
| sat_saturate | 699 ns | 1.92x |
| sat_wrap | 363 ns | 1.00x |

- [20260511T025001Z-15b5fe0](sat_arithmetic/sat_arithmetic_n1024/macos-aarch64-c8-bare-apple/20260511T025001Z-15b5fe0/report.md)
- [20261114T232433Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n1024/macos-aarch64-c8-bare-apple/20261114T232433Z-266650fb-dirty/report.md)

## sat_arithmetic/sat_arithmetic_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T234509Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 7.85 us | 1.72x |
| sat_saturate | 6.43 us | 1.41x |
| sat_wrap | 4.57 us | 1.00x |

- [20261115T093537Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n16384/linux-x86_64-c16-bare-intel/20261115T093537Z-266650fb-dirty/report.md)
- [20261117T234509Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n16384/linux-x86_64-c16-bare-intel/20261117T234509Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232439Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 10.39 us | 2.02x |
| sat_saturate | 10.45 us | 2.03x |
| sat_wrap | 5.14 us | 1.00x |

- [20260511T025005Z-15b5fe0](sat_arithmetic/sat_arithmetic_n16384/macos-aarch64-c8-bare-apple/20260511T025005Z-15b5fe0/report.md)
- [20261114T232439Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n16384/macos-aarch64-c8-bare-apple/20261114T232439Z-266650fb-dirty/report.md)

## sat_arithmetic/sat_arithmetic_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T234509Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 117 ns | 1.75x |
| sat_saturate | 117 ns | 1.75x |
| sat_wrap | 67 ns | 1.00x |

- [20261115T092757Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n256/linux-x86_64-c16-bare-intel/20261115T092757Z-266650fb-dirty/report.md)
- [20261117T234509Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n256/linux-x86_64-c16-bare-intel/20261117T234509Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232431Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 184 ns | 1.72x |
| sat_saturate | 180 ns | 1.68x |
| sat_wrap | 107 ns | 1.00x |

- [20260511T025000Z-15b5fe0](sat_arithmetic/sat_arithmetic_n256/macos-aarch64-c8-bare-apple/20260511T025000Z-15b5fe0/report.md)
- [20261114T232431Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n256/macos-aarch64-c8-bare-apple/20261114T232431Z-266650fb-dirty/report.md)

## sat_arithmetic/sat_arithmetic_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T234509Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 1.97 us | 1.72x |
| sat_saturate | 1.62 us | 1.41x |
| sat_wrap | 1.15 us | 1.00x |

- [20261115T093304Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n4096/linux-x86_64-c16-bare-intel/20261115T093304Z-266650fb-dirty/report.md)
- [20261117T234509Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n4096/linux-x86_64-c16-bare-intel/20261117T234509Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232435Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 2.75 us | 2.05x |
| sat_saturate | 2.66 us | 1.98x |
| sat_wrap | 1.34 us | 1.00x |

- [20260511T025002Z-15b5fe0](sat_arithmetic/sat_arithmetic_n4096/macos-aarch64-c8-bare-apple/20260511T025002Z-15b5fe0/report.md)
- [20261114T232435Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n4096/macos-aarch64-c8-bare-apple/20261114T232435Z-266650fb-dirty/report.md)

## shift_amount_origin/shift_amount_origin_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T235600Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 19 ns | 1.00x |
| shr_loop_invariant | 20 ns | 1.03x |
| shr_per_iter | 95 ns | 4.93x |

- [20261115T094114Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n1024/linux-x86_64-c16-bare-intel/20261115T094114Z-266650fb-dirty/report.md)
- [20261117T235600Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n1024/linux-x86_64-c16-bare-intel/20261117T235600Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232517Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 5 ns | 1.00x |
| shr_loop_invariant | 6 ns | 1.07x |
| shr_per_iter | 66 ns | 12.26x |

- [20260511T033044Z-4f44ee4](shift_amount_origin/shift_amount_origin_n1024/macos-aarch64-c8-bare-apple/20260511T033044Z-4f44ee4/report.md)
- [20261114T232517Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n1024/macos-aarch64-c8-bare-apple/20261114T232517Z-266650fb-dirty/report.md)

## shift_amount_origin/shift_amount_origin_n16384

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T235600Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 136 ns | 1.00x |
| shr_loop_invariant | 203 ns | 1.49x |
| shr_per_iter | 1.33 us | 9.79x |

- [20261115T094618Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n16384/linux-x86_64-c16-bare-intel/20261115T094618Z-266650fb-dirty/report.md)
- [20261117T235600Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n16384/linux-x86_64-c16-bare-intel/20261117T235600Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232522Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 195 ns | 1.00x |
| shr_loop_invariant | 205 ns | 1.05x |
| shr_per_iter | 710 ns | 3.64x |

- [20260511T033046Z-4f44ee4](shift_amount_origin/shift_amount_origin_n16384/macos-aarch64-c8-bare-apple/20260511T033046Z-4f44ee4/report.md)
- [20261114T232522Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n16384/macos-aarch64-c8-bare-apple/20261114T232522Z-266650fb-dirty/report.md)

## shift_amount_origin/shift_amount_origin_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T235600Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 12 ns | 1.00x |
| shr_loop_invariant | 13 ns | 1.07x |
| shr_per_iter | 26 ns | 2.11x |

- [20261115T093839Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n256/linux-x86_64-c16-bare-intel/20261115T093839Z-266650fb-dirty/report.md)
- [20261117T235600Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n256/linux-x86_64-c16-bare-intel/20261117T235600Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232515Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 1 ns | 1.00x |
| shr_loop_invariant | 2 ns | 3.12x |
| shr_per_iter | 7 ns | 8.87x |

- [20260511T033043Z-4f44ee4](shift_amount_origin/shift_amount_origin_n256/macos-aarch64-c8-bare-apple/20260511T033043Z-4f44ee4/report.md)
- [20261114T232515Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n256/macos-aarch64-c8-bare-apple/20261114T232515Z-266650fb-dirty/report.md)

## shift_amount_origin/shift_amount_origin_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261117T235600Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 48 ns | 1.00x |
| shr_loop_invariant | 60 ns | 1.24x |
| shr_per_iter | 342 ns | 7.06x |

- [20261115T094346Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n4096/linux-x86_64-c16-bare-intel/20261115T094346Z-266650fb-dirty/report.md)
- [20261117T235600Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n4096/linux-x86_64-c16-bare-intel/20261117T235600Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232519Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 70 ns | 1.00x |
| shr_loop_invariant | 70 ns | 1.01x |
| shr_per_iter | 193 ns | 2.78x |

- [20260511T033045Z-4f44ee4](shift_amount_origin/shift_amount_origin_n4096/macos-aarch64-c8-bare-apple/20260511T033045Z-4f44ee4/report.md)
- [20261114T232519Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n4096/macos-aarch64-c8-bare-apple/20261114T232519Z-266650fb-dirty/report.md)

## single-trunk-convergence/append_w1_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.91 ms | 1.04x |
| head_tail_dyn | 9.90 ms | 1.04x |
| kernel_nway | 9.90 ms | 1.04x |
| kernel_seq | 9.50 ms | 1.00x |
| nway | 14.77 ms | 1.56x |
| sequential | 14.15 ms | 1.49x |
| std_parallel | 10.71 ms | 1.13x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w1_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091914Z-ed2bf53-dirty](single-trunk-convergence/append_w1_big_n16777216/linux-x86_64-c4-container-intel/20261113T091914Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w1_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 586.75 us | 1.00x |
| head_tail_dyn | 615.80 us | 1.05x |
| kernel_nway | 583.99 us | 1.00x |
| kernel_seq | 624.52 us | 1.07x |
| nway | 985.12 us | 1.69x |
| sequential | 815.49 us | 1.40x |
| std_parallel | 891.21 us | 1.53x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w1_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091545Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n1048576/linux-x86_64-c4-container-intel/20261113T091545Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w1_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 14.23 us | 1.00x |
| head_tail_dyn | 16.38 us | 1.15x |
| kernel_nway | 14.95 us | 1.05x |
| kernel_seq | 14.34 us | 1.01x |
| nway | 42.42 us | 2.98x |
| sequential | 19.47 us | 1.37x |
| std_parallel | 41.10 us | 2.89x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w1_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091048Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n4096/linux-x86_64-c4-container-intel/20261113T091048Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w1_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 29.49 us | 1.00x |
| head_tail_dyn | 37.15 us | 1.26x |
| kernel_nway | 38.94 us | 1.32x |
| kernel_seq | 40.15 us | 1.36x |
| nway | 67.92 us | 2.30x |
| sequential | 56.89 us | 1.93x |
| std_parallel | 114.68 us | 3.89x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w1_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091304Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n65536/linux-x86_64-c4-container-intel/20261113T091304Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w2_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.92 ms | 1.40x |
| head_tail_dyn | 9.38 ms | 1.32x |
| kernel_nway | 7.11 ms | 1.00x |
| kernel_seq | 9.87 ms | 1.39x |
| nway | 22.58 ms | 3.18x |
| sequential | 13.95 ms | 1.96x |
| std_parallel | 7.61 ms | 1.07x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w2_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T100114Z-74174bb](single-trunk-convergence/append_w2_big_n16777216/linux-x86_64-c4-container-intel/20261113T100114Z-74174bb/report.md)

## single-trunk-convergence/append_w2_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 551.32 us | 1.07x |
| head_tail_dyn | 618.12 us | 1.20x |
| kernel_nway | 925.66 us | 1.80x |
| kernel_seq | 589.13 us | 1.14x |
| nway | 2.54 ms | 4.93x |
| sequential | 858.65 us | 1.67x |
| std_parallel | 515.18 us | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w2_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095753Z-74174bb](single-trunk-convergence/append_w2_n1048576/linux-x86_64-c4-container-intel/20261113T095753Z-74174bb/report.md)

## single-trunk-convergence/append_w2_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 17.69 us | 1.58x |
| head_tail_dyn | 16.42 us | 1.47x |
| kernel_nway | 14.57 us | 1.30x |
| kernel_seq | 18.45 us | 1.65x |
| nway | 48.22 us | 4.32x |
| sequential | 11.17 us | 1.00x |
| std_parallel | 57.16 us | 5.12x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w2_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095257Z-74174bb](single-trunk-convergence/append_w2_n4096/linux-x86_64-c4-container-intel/20261113T095257Z-74174bb/report.md)

## single-trunk-convergence/append_w2_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 29.25 us | 1.09x |
| head_tail_dyn | 40.16 us | 1.50x |
| kernel_nway | 26.76 us | 1.00x |
| kernel_seq | 28.88 us | 1.08x |
| nway | 68.22 us | 2.55x |
| sequential | 56.96 us | 2.13x |
| std_parallel | 49.56 us | 1.85x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w2_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095513Z-74174bb](single-trunk-convergence/append_w2_n65536/linux-x86_64-c4-container-intel/20261113T095513Z-74174bb/report.md)

## single-trunk-convergence/append_w4_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 10.02 ms | 1.48x |
| head_tail_dyn | 9.65 ms | 1.42x |
| kernel_nway | 12.05 ms | 1.78x |
| kernel_seq | 9.43 ms | 1.39x |
| nway | 16.56 ms | 2.44x |
| sequential | 14.04 ms | 2.07x |
| std_parallel | 6.78 ms | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w4_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T111219Z-f12bdc4](single-trunk-convergence/append_w4_big_n16777216/linux-x86_64-c4-container-intel/20261113T111219Z-f12bdc4/report.md)

## single-trunk-convergence/append_w4_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 549.34 us | 1.51x |
| head_tail_dyn | 553.94 us | 1.52x |
| kernel_nway | 1.35 ms | 3.71x |
| kernel_seq | 628.46 us | 1.72x |
| nway | 3.02 ms | 8.27x |
| sequential | 845.35 us | 2.32x |
| std_parallel | 364.46 us | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w4_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110856Z-f12bdc4](single-trunk-convergence/append_w4_n1048576/linux-x86_64-c4-container-intel/20261113T110856Z-f12bdc4/report.md)

## single-trunk-convergence/append_w4_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 15.32 us | 1.07x |
| head_tail_dyn | 15.17 us | 1.06x |
| kernel_nway | 14.77 us | 1.03x |
| kernel_seq | 15.10 us | 1.05x |
| nway | 44.26 us | 3.09x |
| sequential | 14.32 us | 1.00x |
| std_parallel | 73.92 us | 5.16x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w4_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110400Z-f12bdc4](single-trunk-convergence/append_w4_n4096/linux-x86_64-c4-container-intel/20261113T110400Z-f12bdc4/report.md)

## single-trunk-convergence/append_w4_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 39.42 us | 1.14x |
| head_tail_dyn | 34.90 us | 1.01x |
| kernel_nway | 44.05 us | 1.27x |
| kernel_seq | 34.56 us | 1.00x |
| nway | 68.94 us | 1.99x |
| sequential | 56.96 us | 1.65x |
| std_parallel | 55.20 us | 1.60x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w4_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110615Z-f12bdc4](single-trunk-convergence/append_w4_n65536/linux-x86_64-c4-container-intel/20261113T110615Z-f12bdc4/report.md)

## single-trunk-convergence/append_w8_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.98 ms | 1.56x |
| head_tail_dyn | 9.94 ms | 1.56x |
| kernel_nway | 8.17 ms | 1.28x |
| kernel_seq | 9.68 ms | 1.52x |
| nway | 6.80 ms | 1.07x |
| sequential | 14.00 ms | 2.19x |
| std_parallel | 6.38 ms | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w8_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T120018Z-d11e290-dirty](single-trunk-convergence/append_w8_big_n16777216/linux-x86_64-c4-container-intel/20261113T120018Z-d11e290-dirty/report.md)

## single-trunk-convergence/append_w8_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 563.50 us | 1.38x |
| head_tail_dyn | 553.66 us | 1.36x |
| kernel_nway | 684.10 us | 1.68x |
| kernel_seq | 607.37 us | 1.49x |
| nway | 1.55 ms | 3.80x |
| sequential | 809.89 us | 1.99x |
| std_parallel | 407.10 us | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w8_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115657Z-d11e290-dirty](single-trunk-convergence/append_w8_n1048576/linux-x86_64-c4-container-intel/20261113T115657Z-d11e290-dirty/report.md)

## single-trunk-convergence/append_w8_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 16.09 us | 1.54x |
| head_tail_dyn | 17.13 us | 1.64x |
| kernel_nway | 21.18 us | 2.03x |
| kernel_seq | 16.79 us | 1.61x |
| nway | 57.33 us | 5.49x |
| sequential | 10.43 us | 1.00x |
| std_parallel | 97.72 us | 9.36x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w8_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115200Z-d11e290-dirty](single-trunk-convergence/append_w8_n4096/linux-x86_64-c4-container-intel/20261113T115200Z-d11e290-dirty/report.md)

## single-trunk-convergence/append_w8_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 40.97 us | 1.23x |
| head_tail_dyn | 41.79 us | 1.26x |
| kernel_nway | 59.57 us | 1.79x |
| kernel_seq | 33.29 us | 1.00x |
| nway | 223.36 us | 6.71x |
| sequential | 56.85 us | 1.71x |
| std_parallel | 264.52 us | 7.95x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/append_w8_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115416Z-d11e290-dirty](single-trunk-convergence/append_w8_n65536/linux-x86_64-c4-container-intel/20261113T115416Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_heavy_w1_big_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 3.15 ms | 1.03x |
| head_tail_dyn | 3.19 ms | 1.04x |
| kernel_nway | 3.18 ms | 1.04x |
| kernel_seq | 3.14 ms | 1.03x |
| nway | 3.20 ms | 1.05x |
| sequential | 3.05 ms | 1.00x |
| std_parallel | 6.81 ms | 2.23x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w1_big_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090834Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_big_n1048576/linux-x86_64-c4-container-intel/20261113T090834Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_heavy_w1_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 43.70 us | 1.02x |
| head_tail_dyn | 44.85 us | 1.05x |
| kernel_nway | 46.65 us | 1.09x |
| kernel_seq | 42.91 us | 1.00x |
| nway | 75.23 us | 1.75x |
| sequential | 54.29 us | 1.27x |
| std_parallel | 100.83 us | 2.35x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w1_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090400Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_n4096/linux-x86_64-c4-container-intel/20261113T090400Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_heavy_w1_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 193.60 us | 1.00x |
| head_tail_dyn | 193.74 us | 1.00x |
| kernel_nway | 193.67 us | 1.00x |
| kernel_seq | 192.91 us | 1.00x |
| nway | 242.64 us | 1.26x |
| sequential | 195.35 us | 1.01x |
| std_parallel | 612.89 us | 3.18x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w1_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090616Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_n65536/linux-x86_64-c4-container-intel/20261113T090616Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_heavy_w2_big_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 1.77 ms | 1.00x |
| head_tail_dyn | 2.15 ms | 1.21x |
| kernel_nway | 1.78 ms | 1.01x |
| kernel_seq | 3.17 ms | 1.79x |
| nway | 1.84 ms | 1.04x |
| sequential | 3.05 ms | 1.72x |
| std_parallel | 3.97 ms | 2.24x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w2_big_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095043Z-74174bb](single-trunk-convergence/fold_heavy_w2_big_n1048576/linux-x86_64-c4-container-intel/20261113T095043Z-74174bb/report.md)

## single-trunk-convergence/fold_heavy_w2_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 49.46 us | 2.62x |
| head_tail_dyn | 35.77 us | 1.90x |
| kernel_nway | 36.25 us | 1.92x |
| kernel_seq | 43.69 us | 2.32x |
| nway | 74.46 us | 3.95x |
| sequential | 18.87 us | 1.00x |
| std_parallel | 94.63 us | 5.02x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w2_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094611Z-74174bb](single-trunk-convergence/fold_heavy_w2_n4096/linux-x86_64-c4-container-intel/20261113T094611Z-74174bb/report.md)

## single-trunk-convergence/fold_heavy_w2_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 115.52 us | 1.00x |
| head_tail_dyn | 121.65 us | 1.05x |
| kernel_nway | 116.65 us | 1.01x |
| kernel_seq | 195.20 us | 1.69x |
| nway | 153.51 us | 1.33x |
| sequential | 195.41 us | 1.69x |
| std_parallel | 264.21 us | 2.29x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w2_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094827Z-74174bb](single-trunk-convergence/fold_heavy_w2_n65536/linux-x86_64-c4-container-intel/20261113T094827Z-74174bb/report.md)

## single-trunk-convergence/fold_heavy_w4_big_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.09 ms | 1.18x |
| head_tail_dyn | 1.76 ms | 1.00x |
| kernel_nway | 2.12 ms | 1.21x |
| kernel_seq | 3.15 ms | 1.79x |
| nway | 2.16 ms | 1.23x |
| sequential | 3.05 ms | 1.73x |
| std_parallel | 2.13 ms | 1.21x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w4_big_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110146Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_big_n1048576/linux-x86_64-c4-container-intel/20261113T110146Z-f12bdc4/report.md)

## single-trunk-convergence/fold_heavy_w4_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 50.82 us | 1.45x |
| head_tail_dyn | 42.31 us | 1.21x |
| kernel_nway | 35.01 us | 1.00x |
| kernel_seq | 46.16 us | 1.32x |
| nway | 63.23 us | 1.81x |
| sequential | 36.73 us | 1.05x |
| std_parallel | 96.53 us | 2.76x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w4_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105714Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_n4096/linux-x86_64-c4-container-intel/20261113T105714Z-f12bdc4/report.md)

## single-trunk-convergence/fold_heavy_w4_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 140.25 us | 1.09x |
| head_tail_dyn | 138.47 us | 1.07x |
| kernel_nway | 129.12 us | 1.00x |
| kernel_seq | 195.67 us | 1.52x |
| nway | 159.20 us | 1.23x |
| sequential | 195.42 us | 1.51x |
| std_parallel | 181.16 us | 1.40x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w4_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105929Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_n65536/linux-x86_64-c4-container-intel/20261113T105929Z-f12bdc4/report.md)

## single-trunk-convergence/fold_heavy_w8_big_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 1.76 ms | 1.00x |
| head_tail_dyn | 1.80 ms | 1.02x |
| kernel_nway | 1.81 ms | 1.03x |
| kernel_seq | 3.17 ms | 1.80x |
| nway | 2.05 ms | 1.17x |
| sequential | 3.02 ms | 1.72x |
| std_parallel | 2.87 ms | 1.63x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w8_big_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114945Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_big_n1048576/linux-x86_64-c4-container-intel/20261113T114945Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_heavy_w8_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 43.53 us | 1.37x |
| head_tail_dyn | 31.66 us | 1.00x |
| kernel_nway | 34.00 us | 1.07x |
| kernel_seq | 48.68 us | 1.54x |
| nway | 54.98 us | 1.74x |
| sequential | 36.67 us | 1.16x |
| std_parallel | 128.52 us | 4.06x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w8_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114513Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_n4096/linux-x86_64-c4-container-intel/20261113T114513Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_heavy_w8_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 123.37 us | 1.34x |
| head_tail_dyn | 122.23 us | 1.32x |
| kernel_nway | 92.36 us | 1.00x |
| kernel_seq | 194.14 us | 2.10x |
| nway | 172.49 us | 1.87x |
| sequential | 195.60 us | 2.12x |
| std_parallel | 294.70 us | 3.19x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_heavy_w8_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114729Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_n65536/linux-x86_64-c4-container-intel/20261113T114729Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w1_big_n134217728

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 30.94 ms | 1.17x |
| head_tail_dyn | 30.79 ms | 1.16x |
| kernel_nway | 30.83 ms | 1.16x |
| kernel_seq | 30.85 ms | 1.17x |
| nway | 31.09 ms | 1.17x |
| sequential | 26.47 ms | 1.00x |
| std_parallel | 30.65 ms | 1.16x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w1_big_n134217728/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090146Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_big_n134217728/linux-x86_64-c4-container-intel/20261113T090146Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.15 ms | 1.18x |
| head_tail_dyn | 4.18 ms | 1.19x |
| kernel_nway | 4.13 ms | 1.17x |
| kernel_seq | 4.18 ms | 1.19x |
| nway | 4.16 ms | 1.18x |
| sequential | 3.52 ms | 1.00x |
| std_parallel | 4.90 ms | 1.39x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w1_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085554Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_big_n16777216/linux-x86_64-c4-container-intel/20261113T085554Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 114.87 us | 1.08x |
| head_tail_dyn | 107.27 us | 1.01x |
| kernel_nway | 124.08 us | 1.17x |
| kernel_seq | 106.17 us | 1.00x |
| nway | 110.91 us | 1.04x |
| sequential | 107.64 us | 1.01x |
| std_parallel | 284.15 us | 2.68x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w1_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085316Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n1048576/linux-x86_64-c4-container-intel/20261113T085316Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 15.65 us | 1.19x |
| head_tail_dyn | 14.63 us | 1.11x |
| kernel_nway | 19.56 us | 1.48x |
| kernel_seq | 13.17 us | 1.00x |
| nway | 45.05 us | 3.42x |
| sequential | 27.33 us | 2.07x |
| std_parallel | 42.82 us | 3.25x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w1_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084844Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n4096/linux-x86_64-c4-container-intel/20261113T084844Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 35.83 us | 1.61x |
| head_tail_dyn | 30.30 us | 1.36x |
| kernel_nway | 38.47 us | 1.73x |
| kernel_seq | 32.20 us | 1.44x |
| nway | 57.18 us | 2.57x |
| sequential | 22.29 us | 1.00x |
| std_parallel | 67.83 us | 3.04x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w1_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085057Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n65536/linux-x86_64-c4-container-intel/20261113T085057Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w2_big_n134217728

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 19.81 ms | 1.00x |
| head_tail_dyn | 27.07 ms | 1.37x |
| kernel_nway | 19.74 ms | 1.00x |
| kernel_seq | 29.48 ms | 1.49x |
| nway | 19.79 ms | 1.00x |
| sequential | 27.98 ms | 1.42x |
| std_parallel | 20.22 ms | 1.02x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w2_big_n134217728/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094358Z-74174bb](single-trunk-convergence/fold_light_w2_big_n134217728/linux-x86_64-c4-container-intel/20261113T094358Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.88 ms | 1.00x |
| head_tail_dyn | 6.99 ms | 2.43x |
| kernel_nway | 3.01 ms | 1.05x |
| kernel_seq | 4.19 ms | 1.45x |
| nway | 4.82 ms | 1.67x |
| sequential | 4.16 ms | 1.44x |
| std_parallel | 2.89 ms | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w2_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093857Z-74174bb](single-trunk-convergence/fold_light_w2_big_n16777216/linux-x86_64-c4-container-intel/20261113T093857Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 103.91 us | 1.20x |
| head_tail_dyn | 160.10 us | 1.85x |
| kernel_nway | 102.59 us | 1.19x |
| kernel_seq | 102.38 us | 1.18x |
| nway | 86.56 us | 1.00x |
| sequential | 106.17 us | 1.23x |
| std_parallel | 179.68 us | 2.08x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w2_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093622Z-74174bb](single-trunk-convergence/fold_light_w2_n1048576/linux-x86_64-c4-container-intel/20261113T093622Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 15.39 us | 1.04x |
| head_tail_dyn | 14.75 us | 1.00x |
| kernel_nway | 14.99 us | 1.02x |
| kernel_seq | 16.61 us | 1.13x |
| nway | 50.09 us | 3.40x |
| sequential | 25.08 us | 1.70x |
| std_parallel | 50.27 us | 3.41x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w2_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093149Z-74174bb](single-trunk-convergence/fold_light_w2_n4096/linux-x86_64-c4-container-intel/20261113T093149Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 25.87 us | 1.92x |
| head_tail_dyn | 50.60 us | 3.76x |
| kernel_nway | 27.91 us | 2.08x |
| kernel_seq | 28.68 us | 2.13x |
| nway | 59.12 us | 4.40x |
| sequential | 13.45 us | 1.00x |
| std_parallel | 72.23 us | 5.37x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w2_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093403Z-74174bb](single-trunk-convergence/fold_light_w2_n65536/linux-x86_64-c4-container-intel/20261113T093403Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w4_big_n134217728

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 21.82 ms | 1.44x |
| head_tail_dyn | 52.36 ms | 3.45x |
| kernel_nway | 15.19 ms | 1.00x |
| kernel_seq | 31.13 ms | 2.05x |
| nway | 31.20 ms | 2.05x |
| sequential | 27.82 ms | 1.83x |
| std_parallel | 17.10 ms | 1.13x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w4_big_n134217728/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105500Z-f12bdc4](single-trunk-convergence/fold_light_w4_big_n134217728/linux-x86_64-c4-container-intel/20261113T105500Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 3.62 ms | 1.66x |
| head_tail_dyn | 3.79 ms | 1.74x |
| kernel_nway | 3.89 ms | 1.79x |
| kernel_seq | 4.17 ms | 1.92x |
| nway | 4.30 ms | 1.98x |
| sequential | 4.14 ms | 1.90x |
| std_parallel | 2.18 ms | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w4_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104952Z-f12bdc4](single-trunk-convergence/fold_light_w4_big_n16777216/linux-x86_64-c4-container-intel/20261113T104952Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 98.27 us | 1.47x |
| head_tail_dyn | 193.47 us | 2.89x |
| kernel_nway | 111.96 us | 1.67x |
| kernel_seq | 115.66 us | 1.73x |
| nway | 67.05 us | 1.00x |
| sequential | 101.81 us | 1.52x |
| std_parallel | 180.67 us | 2.69x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w4_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104719Z-f12bdc4](single-trunk-convergence/fold_light_w4_n1048576/linux-x86_64-c4-container-intel/20261113T104719Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 12.89 us | 1.00x |
| head_tail_dyn | 18.49 us | 1.43x |
| kernel_nway | 15.97 us | 1.24x |
| kernel_seq | 13.30 us | 1.03x |
| nway | 49.54 us | 3.84x |
| sequential | 23.74 us | 1.84x |
| std_parallel | 87.33 us | 6.77x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w4_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104247Z-f12bdc4](single-trunk-convergence/fold_light_w4_n4096/linux-x86_64-c4-container-intel/20261113T104247Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 28.10 us | 1.64x |
| head_tail_dyn | 43.83 us | 2.55x |
| kernel_nway | 24.16 us | 1.41x |
| kernel_seq | 34.06 us | 1.98x |
| nway | 58.73 us | 3.42x |
| sequential | 17.16 us | 1.00x |
| std_parallel | 84.80 us | 4.94x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w4_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104501Z-f12bdc4](single-trunk-convergence/fold_light_w4_n65536/linux-x86_64-c4-container-intel/20261113T104501Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w8_big_n134217728

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 20.24 ms | 1.55x |
| head_tail_dyn | 46.30 ms | 3.54x |
| kernel_nway | 13.09 ms | 1.00x |
| kernel_seq | 30.41 ms | 2.32x |
| nway | 15.65 ms | 1.20x |
| sequential | 26.42 ms | 2.02x |
| std_parallel | 14.24 ms | 1.09x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w8_big_n134217728/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114259Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_big_n134217728/linux-x86_64-c4-container-intel/20261113T114259Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_big_n16777216

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 6.05 ms | 3.25x |
| head_tail_dyn | 3.38 ms | 1.82x |
| kernel_nway | 3.46 ms | 1.86x |
| kernel_seq | 4.16 ms | 2.24x |
| nway | 4.43 ms | 2.38x |
| sequential | 4.14 ms | 2.23x |
| std_parallel | 1.86 ms | 1.00x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w8_big_n16777216/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113804Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_big_n16777216/linux-x86_64-c4-container-intel/20261113T113804Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_n1048576

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 103.39 us | 1.25x |
| head_tail_dyn | 184.80 us | 2.24x |
| kernel_nway | 82.67 us | 1.00x |
| kernel_seq | 106.33 us | 1.29x |
| nway | 121.26 us | 1.47x |
| sequential | 107.95 us | 1.31x |
| std_parallel | 178.19 us | 2.16x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w8_n1048576/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113530Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n1048576/linux-x86_64-c4-container-intel/20261113T113530Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 14.45 us | 1.00x |
| head_tail_dyn | 15.73 us | 1.09x |
| kernel_nway | 19.04 us | 1.32x |
| kernel_seq | 14.44 us | 1.00x |
| nway | 54.16 us | 3.75x |
| sequential | 25.98 us | 1.80x |
| std_parallel | 74.99 us | 5.19x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w8_n4096/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113057Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n4096/linux-x86_64-c4-container-intel/20261113T113057Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_n65536

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 30.50 us | 1.88x |
| head_tail_dyn | 43.56 us | 2.69x |
| kernel_nway | 22.26 us | 1.38x |
| kernel_seq | 36.96 us | 2.28x |
| nway | 47.39 us | 2.93x |
| sequential | 16.19 us | 1.00x |
| std_parallel | 75.14 us | 4.64x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/fold_light_w8_n65536/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113312Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n65536/linux-x86_64-c4-container-intel/20261113T113312Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w1_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 11.86 us | 1.22x |
| head_tail_dyn | 12.14 us | 1.25x |
| kernel_nway | 10.36 us | 1.07x |
| kernel_seq | 9.69 us | 1.00x |
| nway | 31.81 us | 3.28x |
| sequential | 11.46 us | 1.18x |
| std_parallel | 36.61 us | 3.78x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w1_n1024/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083547Z-e486b5f-dirty](single-trunk-convergence/short_w1_n1024/linux-x86_64-c2-container-intel/20261113T083547Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084629Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n1024/linux-x86_64-c4-container-intel/20261113T084629Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w1_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 15.33 us | 1.81x |
| head_tail_dyn | 10.90 us | 1.29x |
| kernel_nway | 13.17 us | 1.55x |
| kernel_seq | 11.80 us | 1.39x |
| nway | 34.40 us | 4.06x |
| sequential | 8.48 us | 1.00x |
| std_parallel | 37.02 us | 4.37x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w1_n16/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T082904Z-e486b5f-dirty](single-trunk-convergence/short_w1_n16/linux-x86_64-c2-container-intel/20261113T082904Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T083944Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n16/linux-x86_64-c4-container-intel/20261113T083944Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w1_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 11.56 us | 1.62x |
| head_tail_dyn | 10.45 us | 1.46x |
| kernel_nway | 12.07 us | 1.69x |
| kernel_seq | 9.56 us | 1.34x |
| nway | 33.06 us | 4.63x |
| sequential | 7.14 us | 1.00x |
| std_parallel | 34.03 us | 4.77x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w1_n256/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083333Z-e486b5f-dirty](single-trunk-convergence/short_w1_n256/linux-x86_64-c2-container-intel/20261113T083333Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084415Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n256/linux-x86_64-c4-container-intel/20261113T084415Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w1_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 10.43 us | 1.46x |
| head_tail_dyn | 10.97 us | 1.54x |
| kernel_nway | 11.78 us | 1.65x |
| kernel_seq | 12.95 us | 1.82x |
| nway | 34.98 us | 4.91x |
| sequential | 7.12 us | 1.00x |
| std_parallel | 38.45 us | 5.40x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w1_n64/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083120Z-e486b5f-dirty](single-trunk-convergence/short_w1_n64/linux-x86_64-c2-container-intel/20261113T083120Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084201Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n64/linux-x86_64-c4-container-intel/20261113T084201Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w2_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 14.01 us | 1.95x |
| head_tail_dyn | 13.13 us | 1.83x |
| kernel_nway | 12.93 us | 1.80x |
| kernel_seq | 12.43 us | 1.73x |
| nway | 33.81 us | 4.72x |
| sequential | 7.17 us | 1.00x |
| std_parallel | 49.80 us | 6.95x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w2_n1024/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092935Z-74174bb](single-trunk-convergence/short_w2_n1024/linux-x86_64-c4-container-intel/20261113T092935Z-74174bb/report.md)

## single-trunk-convergence/short_w2_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 12.24 us | 1.18x |
| head_tail_dyn | 10.79 us | 1.04x |
| kernel_nway | 12.47 us | 1.21x |
| kernel_seq | 10.33 us | 1.00x |
| nway | 37.23 us | 3.60x |
| sequential | 14.27 us | 1.38x |
| std_parallel | 48.16 us | 4.66x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w2_n16/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092249Z-74174bb](single-trunk-convergence/short_w2_n16/linux-x86_64-c4-container-intel/20261113T092249Z-74174bb/report.md)

## single-trunk-convergence/short_w2_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 13.40 us | 1.20x |
| head_tail_dyn | 13.76 us | 1.23x |
| kernel_nway | 11.17 us | 1.00x |
| kernel_seq | 11.29 us | 1.01x |
| nway | 32.83 us | 2.94x |
| sequential | 16.96 us | 1.52x |
| std_parallel | 48.46 us | 4.34x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w2_n256/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092721Z-74174bb](single-trunk-convergence/short_w2_n256/linux-x86_64-c4-container-intel/20261113T092721Z-74174bb/report.md)

## single-trunk-convergence/short_w2_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 11.63 us | 1.14x |
| head_tail_dyn | 13.24 us | 1.29x |
| kernel_nway | 11.26 us | 1.10x |
| kernel_seq | 10.23 us | 1.00x |
| nway | 37.42 us | 3.66x |
| sequential | 16.79 us | 1.64x |
| std_parallel | 48.41 us | 4.73x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w2_n64/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092506Z-74174bb](single-trunk-convergence/short_w2_n64/linux-x86_64-c4-container-intel/20261113T092506Z-74174bb/report.md)

## single-trunk-convergence/short_w4_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 13.04 us | 1.60x |
| head_tail_dyn | 11.15 us | 1.37x |
| kernel_nway | 13.40 us | 1.65x |
| kernel_seq | 12.65 us | 1.55x |
| nway | 40.54 us | 4.98x |
| sequential | 8.14 us | 1.00x |
| std_parallel | 61.44 us | 7.55x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w4_n1024/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104033Z-f12bdc4](single-trunk-convergence/short_w4_n1024/linux-x86_64-c4-container-intel/20261113T104033Z-f12bdc4/report.md)

## single-trunk-convergence/short_w4_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 12.64 us | 1.25x |
| head_tail_dyn | 13.26 us | 1.31x |
| kernel_nway | 14.57 us | 1.44x |
| kernel_seq | 10.09 us | 1.00x |
| nway | 42.00 us | 4.16x |
| sequential | 12.60 us | 1.25x |
| std_parallel | 42.32 us | 4.19x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w4_n16/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103346Z-f12bdc4](single-trunk-convergence/short_w4_n16/linux-x86_64-c4-container-intel/20261113T103346Z-f12bdc4/report.md)

## single-trunk-convergence/short_w4_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 11.43 us | 1.16x |
| head_tail_dyn | 13.27 us | 1.35x |
| kernel_nway | 15.94 us | 1.62x |
| kernel_seq | 9.86 us | 1.00x |
| nway | 43.52 us | 4.42x |
| sequential | 12.90 us | 1.31x |
| std_parallel | 56.10 us | 5.69x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w4_n256/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103818Z-f12bdc4](single-trunk-convergence/short_w4_n256/linux-x86_64-c4-container-intel/20261113T103818Z-f12bdc4/report.md)

## single-trunk-convergence/short_w4_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 12.20 us | 1.11x |
| head_tail_dyn | 10.98 us | 1.00x |
| kernel_nway | 15.84 us | 1.44x |
| kernel_seq | 11.77 us | 1.07x |
| nway | 42.12 us | 3.83x |
| sequential | 12.58 us | 1.15x |
| std_parallel | 44.40 us | 4.04x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w4_n64/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103604Z-f12bdc4](single-trunk-convergence/short_w4_n64/linux-x86_64-c4-container-intel/20261113T103604Z-f12bdc4/report.md)

## single-trunk-convergence/short_w8_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 12.64 us | 1.32x |
| head_tail_dyn | 15.25 us | 1.60x |
| kernel_nway | 18.58 us | 1.94x |
| kernel_seq | 10.96 us | 1.15x |
| nway | 49.89 us | 5.22x |
| sequential | 9.56 us | 1.00x |
| std_parallel | 73.47 us | 7.68x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w8_n1024/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112843Z-d11e290-dirty](single-trunk-convergence/short_w8_n1024/linux-x86_64-c4-container-intel/20261113T112843Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w8_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 14.03 us | 2.01x |
| head_tail_dyn | 13.81 us | 1.98x |
| kernel_nway | 17.95 us | 2.57x |
| kernel_seq | 13.51 us | 1.94x |
| nway | 53.76 us | 7.70x |
| sequential | 6.98 us | 1.00x |
| std_parallel | 70.16 us | 10.05x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w8_n16/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112154Z-d11e290-dirty](single-trunk-convergence/short_w8_n16/linux-x86_64-c4-container-intel/20261113T112154Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w8_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 11.85 us | 1.14x |
| head_tail_dyn | 10.38 us | 1.00x |
| kernel_nway | 17.61 us | 1.70x |
| kernel_seq | 13.94 us | 1.34x |
| nway | 48.76 us | 4.70x |
| sequential | 18.33 us | 1.77x |
| std_parallel | 71.20 us | 6.86x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w8_n256/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112627Z-d11e290-dirty](single-trunk-convergence/short_w8_n256/linux-x86_64-c4-container-intel/20261113T112627Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w8_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261118T000707Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 10.77 us | 1.00x |
| head_tail_dyn | 11.43 us | 1.06x |
| kernel_nway | 17.45 us | 1.62x |
| kernel_seq | 11.12 us | 1.03x |
| nway | 47.37 us | 4.40x |
| sequential | 14.44 us | 1.34x |
| std_parallel | 67.06 us | 6.22x |

- [20261118T000707Z-07a9e9a3](single-trunk-convergence/short_w8_n64/linux-x86_64-c16-bare-intel/20261118T000707Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112412Z-d11e290-dirty](single-trunk-convergence/short_w8_n64/linux-x86_64-c4-container-intel/20261113T112412Z-d11e290-dirty/report.md)

## swap_band/swap_band_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T024217Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sb_none | 8.29 us | 1.00x |
| sb_plan | 11.48 us | 1.39x |
| sb_value | 282.01 us | 34.01x |

- [20261115T095229Z-266650fb-dirty](swap_band/swap_band_n1024/linux-x86_64-c16-bare-intel/20261115T095229Z-266650fb-dirty/report.md)
- [20261118T024217Z-07a9e9a3](swap_band/swap_band_n1024/linux-x86_64-c16-bare-intel/20261118T024217Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232820Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 10.04 us | 1.00x |
| sb_plan | 12.35 us | 1.23x |
| sb_value | 155.06 us | 15.45x |

- [20260719T185027Z-8c3a949d-dirty](swap_band/swap_band_n1024/macos-aarch64-c8-bare-apple/20260719T185027Z-8c3a949d-dirty/report.md)
- [20261114T232820Z-266650fb-dirty](swap_band/swap_band_n1024/macos-aarch64-c8-bare-apple/20261114T232820Z-266650fb-dirty/report.md)

## swap_band/swap_band_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T024217Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sb_none | 6.61 us | 1.00x |
| sb_plan | 9.30 us | 1.41x |
| sb_value | 16.59 us | 2.51x |

- [20261115T095023Z-266650fb-dirty](swap_band/swap_band_n64/linux-x86_64-c16-bare-intel/20261115T095023Z-266650fb-dirty/report.md)
- [20261118T024217Z-07a9e9a3](swap_band/swap_band_n64/linux-x86_64-c16-bare-intel/20261118T024217Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T232808Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 10.02 us | 1.00x |
| sb_plan | 12.53 us | 1.25x |
| sb_value | 17.82 us | 1.78x |

- [20260719T185015Z-8c3a949d-dirty](swap_band/swap_band_n64/macos-aarch64-c8-bare-apple/20260719T185015Z-8c3a949d-dirty/report.md)
- [20261114T232808Z-266650fb-dirty](swap_band/swap_band_n64/macos-aarch64-c8-bare-apple/20261114T232808Z-266650fb-dirty/report.md)

## swap_band/swap_band_n8192

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T024217Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sb_none | 9.40 us | 1.00x |
| sb_plan | 12.49 us | 1.33x |
| sb_value | 5.19 ms | 551.69x |

- [20261115T095526Z-266650fb-dirty](swap_band/swap_band_n8192/linux-x86_64-c16-bare-intel/20261115T095526Z-266650fb-dirty/report.md)
- [20261118T024217Z-07a9e9a3](swap_band/swap_band_n8192/linux-x86_64-c16-bare-intel/20261118T024217Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T232949Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 12.92 us | 1.00x |
| sb_plan | 15.24 us | 1.18x |
| sb_value | 2.54 ms | 196.70x |

- [20260719T185155Z-8c3a949d-dirty](swap_band/swap_band_n8192/macos-aarch64-c8-bare-apple/20260719T185155Z-8c3a949d-dirty/report.md)
- [20261114T232949Z-266650fb-dirty](swap_band/swap_band_n8192/macos-aarch64-c8-bare-apple/20261114T232949Z-266650fb-dirty/report.md)

## swap_column/swap_column_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T025000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| cs_exch | 707.11 us | 1.56x |
| cs_plain | 452.32 us | 1.00x |
| cs_snap | 991.33 us | 2.19x |
| cs_snap_twin | 919.00 us | 2.03x |

- [20261115T100307Z-266650fb-dirty](swap_column/swap_column_n1024/linux-x86_64-c16-bare-intel/20261115T100307Z-266650fb-dirty/report.md)
- [20261118T025000Z-07a9e9a3](swap_column/swap_column_n1024/linux-x86_64-c16-bare-intel/20261118T025000Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261113T190452Z-5e6887f-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 725.26 us | 1.24x |
| cs_plain | 584.76 us | 1.00x |
| cs_snap | 1.09 ms | 1.87x |
| cs_snap_twin | 1.01 ms | 1.72x |

- [20261113T190452Z-5e6887f-dirty](swap_column/swap_column_n1024/linux-x86_64-c4-container-intel/20261113T190452Z-5e6887f-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261114T233320Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 210.39 us | 1.01x |
| cs_plain | 209.26 us | 1.00x |
| cs_snap | 301.32 us | 1.44x |
| cs_snap_twin | 307.54 us | 1.47x |

- [20261114T233320Z-266650fb-dirty](swap_column/swap_column_n1024/macos-aarch64-c8-bare-apple/20261114T233320Z-266650fb-dirty/report.md)

## swap_column/swap_column_n64

### linux-x86_64-c16-bare-intel, 3 run(s)

Latest: `20261118T025000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| cs_exch | 17.74 us | 1.08x |
| cs_plain | 16.42 us | 1.00x |
| cs_snap | 22.68 us | 1.38x |
| cs_snap_twin | 22.38 us | 1.36x |

- [20261115T100020Z-266650fb-dirty](swap_column/swap_column_n64/linux-x86_64-c16-bare-intel/20261115T100020Z-266650fb-dirty/report.md)
- [20261118T025000Z-07a9e9a3](swap_column/swap_column_n64/linux-x86_64-c16-bare-intel/20261118T025000Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261113T190201Z-5e6887f-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 25.26 us | 1.11x |
| cs_plain | 22.78 us | 1.00x |
| cs_snap | 49.87 us | 2.19x |
| cs_snap_twin | 30.05 us | 1.32x |

- [20261113T190201Z-5e6887f-dirty](swap_column/swap_column_n64/linux-x86_64-c4-container-intel/20261113T190201Z-5e6887f-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261114T233303Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 21.75 us | 1.00x |
| cs_plain | 21.80 us | 1.00x |
| cs_snap | 27.07 us | 1.24x |
| cs_snap_twin | 27.46 us | 1.26x |

- [20261114T233303Z-266650fb-dirty](swap_column/swap_column_n64/macos-aarch64-c8-bare-apple/20261114T233303Z-266650fb-dirty/report.md)

## swap_column/swap_column_n8192

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T025000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| cs_exch | 7.26 ms | 1.16x |
| cs_plain | 6.27 ms | 1.00x |
| cs_snap | 9.08 ms | 1.45x |
| cs_snap_twin | 9.15 ms | 1.46x |

- [20261115T100719Z-266650fb-dirty](swap_column/swap_column_n8192/linux-x86_64-c16-bare-intel/20261115T100719Z-266650fb-dirty/report.md)
- [20261118T025000Z-07a9e9a3](swap_column/swap_column_n8192/linux-x86_64-c16-bare-intel/20261118T025000Z-07a9e9a3/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261113T190946Z-5e6887f-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 11.54 ms | 1.14x |
| cs_plain | 10.16 ms | 1.00x |
| cs_snap | 16.22 ms | 1.60x |
| cs_snap_twin | 15.61 ms | 1.54x |

- [20261113T190946Z-5e6887f-dirty](swap_column/swap_column_n8192/linux-x86_64-c4-container-intel/20261113T190946Z-5e6887f-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261114T233524Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 2.25 ms | 1.00x |
| cs_plain | 3.18 ms | 1.41x |
| cs_snap | 3.29 ms | 1.46x |
| cs_snap_twin | 3.26 ms | 1.45x |

- [20261114T233524Z-266650fb-dirty](swap_column/swap_column_n8192/macos-aarch64-c8-bare-apple/20261114T233524Z-266650fb-dirty/report.md)

## swap_cost/swap_cost_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T030011Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sc_v1k | 16 ns | 1.61x |
| sc_v64b | 10 ns | 1.00x |
| sc_v64k | 2.14 us | 212.30x |

- [20261115T101140Z-266650fb-dirty](swap_cost/swap_cost_n64/linux-x86_64-c16-bare-intel/20261115T101140Z-266650fb-dirty/report.md)
- [20261118T030011Z-07a9e9a3](swap_cost/swap_cost_n64/linux-x86_64-c16-bare-intel/20261118T030011Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261114T233754Z-266650fb`

| variant | median | vs best |
|---|---|---|
| sc_v64k | 1.71 us | 1.00x |

- [20260719T185012Z-8c3a949d-dirty](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20260719T185012Z-8c3a949d-dirty/report.md)
- [20261114T233753Z-266650fb-dirty](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20261114T233753Z-266650fb-dirty/report.md)

## wake_policy/wake_policy_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T030337Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 20.26 us | 1.18x |
| wp_lp0 | 19.79 us | 1.15x |
| wp_nh128 | 20.62 us | 1.20x |
| wp_nh2k | 22.42 us | 1.31x |
| wp_nh8k | 26.01 us | 1.52x |
| wp_park | 18.28 us | 1.07x |
| wp_spin | 17.15 us | 1.00x |

- [20261115T102359Z-266650fb-dirty](wake_policy/wake_policy_n1024/linux-x86_64-c16-bare-intel/20261115T102359Z-266650fb-dirty/report.md)
- [20261118T030337Z-07a9e9a3](wake_policy/wake_policy_n1024/linux-x86_64-c16-bare-intel/20261118T030337Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 5 run(s)

Latest: `20261114T234250Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 9.45 us | 1.08x |
| wp_lp0 | 9.03 us | 1.03x |
| wp_nh128 | 8.99 us | 1.03x |
| wp_nh2k | 12.17 us | 1.39x |
| wp_nh8k | 17.71 us | 2.03x |
| wp_park | 8.74 us | 1.00x |
| wp_spin | 13.57 us | 1.55x |

- [20260719T201051Z-a12070de-dirty](wake_policy/wake_policy_n1024/macos-aarch64-c8-bare-apple/20260719T201051Z-a12070de-dirty/report.md)
- [20261114T234250Z-266650fb-dirty](wake_policy/wake_policy_n1024/macos-aarch64-c8-bare-apple/20261114T234250Z-266650fb-dirty/report.md)

## wake_policy/wake_policy_n64

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T030337Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 17.62 us | 3.33x |
| wp_lp0 | 17.24 us | 3.26x |
| wp_nh128 | 22.13 us | 4.18x |
| wp_nh2k | 17.55 us | 3.31x |
| wp_nh8k | 5.29 us | 1.00x |
| wp_park | 24.98 us | 4.72x |
| wp_spin | 21.77 us | 4.11x |

- [20261115T101936Z-266650fb-dirty](wake_policy/wake_policy_n64/linux-x86_64-c16-bare-intel/20261115T101936Z-266650fb-dirty/report.md)
- [20261118T030337Z-07a9e9a3](wake_policy/wake_policy_n64/linux-x86_64-c16-bare-intel/20261118T030337Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 5 run(s)

Latest: `20261114T234248Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 5.55 us | 1.09x |
| wp_lp0 | 5.07 us | 1.00x |
| wp_nh128 | 6.17 us | 1.22x |
| wp_nh2k | 9.66 us | 1.91x |
| wp_nh8k | 10.89 us | 2.15x |
| wp_park | 5.14 us | 1.01x |
| wp_spin | 10.64 us | 2.10x |

- [20260719T201050Z-a12070de-dirty](wake_policy/wake_policy_n64/macos-aarch64-c8-bare-apple/20260719T201050Z-a12070de-dirty/report.md)
- [20261114T234248Z-266650fb-dirty](wake_policy/wake_policy_n64/macos-aarch64-c8-bare-apple/20261114T234248Z-266650fb-dirty/report.md)

## wake_policy/wake_policy_n8192

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261118T030337Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 60.26 us | 1.11x |
| wp_lp0 | 78.49 us | 1.45x |
| wp_nh128 | 65.90 us | 1.22x |
| wp_nh2k | 71.15 us | 1.31x |
| wp_nh8k | 65.86 us | 1.22x |
| wp_park | 82.05 us | 1.52x |
| wp_spin | 54.15 us | 1.00x |

- [20261115T102820Z-266650fb-dirty](wake_policy/wake_policy_n8192/linux-x86_64-c16-bare-intel/20261115T102820Z-266650fb-dirty/report.md)
- [20261118T030337Z-07a9e9a3](wake_policy/wake_policy_n8192/linux-x86_64-c16-bare-intel/20261118T030337Z-07a9e9a3/report.md)

### macos-aarch64-c8-bare-apple, 5 run(s)

Latest: `20261114T234254Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 30.41 us | 1.07x |
| wp_lp0 | 28.48 us | 1.00x |
| wp_nh128 | 34.09 us | 1.20x |
| wp_nh2k | 33.90 us | 1.19x |
| wp_nh8k | 41.65 us | 1.46x |
| wp_park | 33.34 us | 1.17x |
| wp_spin | 34.88 us | 1.22x |

- [20260719T201053Z-a12070de-dirty](wake_policy/wake_policy_n8192/macos-aarch64-c8-bare-apple/20260719T201053Z-a12070de-dirty/report.md)
- [20261114T234254Z-266650fb-dirty](wake_policy/wake_policy_n8192/macos-aarch64-c8-bare-apple/20261114T234254Z-266650fb-dirty/report.md)
