# Bench Results Index

Generated from the runs under `results/`, one directory per run at `<bench>/<cell>/<machine class>/<run>/`, and regenerated from them on every run. Each run holds its samples, meta, report and summary.

## atomic_inc_strategy/atomic_inc_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T020533Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 8.34 us | 801.46x |
| inc_fetch_add | 4.69 us | 450.56x |
| inc_load_store | 10 ns | 1.00x |

- [20261115T020533Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T020533Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T021047Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 146.50 us | 13952.07x |
| inc_fetch_add | 82.17 us | 7825.81x |
| inc_load_store | 10 ns | 1.00x |

- [20261115T021047Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T021047Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T205820Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inc_load_store | 0 ns | 1.00x |

- [20260511T015108Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T015108Z-88d91ab/report.md)
- [20261114T205819Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T205819Z-266650fb-dirty/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T020258Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 1.88 us | 221.12x |
| inc_fetch_add | 1.06 us | 125.16x |
| inc_load_store | 8 ns | 1.00x |

- [20261115T020258Z-266650fb](atomic_inc_strategy/atomic_inc_strategy_n256/linux-x86_64-c16-bare-intel/20261115T020258Z-266650fb/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T020807Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 29.96 us | 2880.84x |
| inc_fetch_add | 16.82 us | 1617.45x |
| inc_load_store | 10 ns | 1.00x |

- [20261115T020807Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T020807Z-266650fb-dirty/report.md)

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

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214037Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n1024/macos-aarch64-c8-bare-apple/20260510T214037Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n16384

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214043Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n16384/macos-aarch64-c8-bare-apple/20260510T214043Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n256

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214035Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n256/macos-aarch64-c8-bare-apple/20260510T214035Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n4096

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214039Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n4096/macos-aarch64-c8-bare-apple/20260510T214039Z-f6ee560/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n64

### macos-aarch64-c8-bare-apple, 1 run(s)

- [20260510T214034Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n64/macos-aarch64-c8-bare-apple/20260510T214034Z-f6ee560/report.md)

## atomic_vs_plain/atomic_vs_plain_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T021656Z-266650fb`

| variant | median | vs best |
|---|---|---|
| acc_plain | 1.15 us | 1.00x |

- [20261115T021655Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/linux-x86_64-c16-bare-intel/20261115T021655Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T022201Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 18.27 us | 1.00x |
| acc_atomic_seqcst | 18.27 us | 1.00x |
| acc_plain | 18.27 us | 1.00x |

- [20261115T022201Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n16384/linux-x86_64-c16-bare-intel/20261115T022201Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T021420Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 292 ns | 1.00x |
| acc_atomic_seqcst | 292 ns | 1.00x |
| acc_plain | 292 ns | 1.00x |

- [20261115T021420Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n256/linux-x86_64-c16-bare-intel/20261115T021420Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T021927Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 4.57 us | 1.00x |
| acc_atomic_seqcst | 4.57 us | 1.00x |
| acc_plain | 4.57 us | 1.00x |

- [20261115T021927Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n4096/linux-x86_64-c16-bare-intel/20261115T021927Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T022738Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 52 ns | 1.00x |
| ctz_intrinsic | 52 ns | 1.00x |
| ctz_loop | 124 ns | 2.39x |

- [20261115T022738Z-266650fb-dirty](bit_count_ops/bit_count_ops_n1024/linux-x86_64-c16-bare-intel/20261115T022738Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T023242Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 621 ns | 1.00x |
| ctz_intrinsic | 620 ns | 1.00x |
| ctz_loop | 2.47 us | 3.99x |

- [20261115T023242Z-266650fb-dirty](bit_count_ops/bit_count_ops_n16384/linux-x86_64-c16-bare-intel/20261115T023242Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T022503Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 20 ns | 1.21x |
| ctz_intrinsic | 16 ns | 1.00x |
| ctz_loop | 38 ns | 2.28x |

- [20261115T022503Z-266650fb-dirty](bit_count_ops/bit_count_ops_n256/linux-x86_64-c16-bare-intel/20261115T022503Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T023010Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 180 ns | 1.00x |
| ctz_intrinsic | 180 ns | 1.00x |
| ctz_loop | 671 ns | 3.73x |

- [20261115T023010Z-266650fb-dirty](bit_count_ops/bit_count_ops_n4096/linux-x86_64-c16-bare-intel/20261115T023010Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T023819Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 152 ns | 1.00x |
| bc_iter | 152 ns | 1.00x |
| bc_unchecked | 153 ns | 1.00x |

- [20261115T023819Z-266650fb-dirty](bounds_check/bounds_check_n1024/linux-x86_64-c16-bare-intel/20261115T023819Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024323Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 2.29 us | 1.00x |
| bc_iter | 2.29 us | 1.00x |
| bc_unchecked | 2.29 us | 1.00x |

- [20261115T024323Z-266650fb-dirty](bounds_check/bounds_check_n16384/linux-x86_64-c16-bare-intel/20261115T024323Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T023548Z-266650fb`

| variant | median | vs best |
|---|---|---|
| bc_unchecked | 37 ns | 1.00x |

- [20261115T023544Z-266650fb-dirty](bounds_check/bounds_check_n256/linux-x86_64-c16-bare-intel/20261115T023544Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024051Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 580 ns | 1.00x |
| bc_iter | 580 ns | 1.00x |
| bc_unchecked | 580 ns | 1.00x |

- [20261115T024051Z-266650fb-dirty](bounds_check/bounds_check_n4096/linux-x86_64-c16-bare-intel/20261115T024051Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024902Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.71 us | 1.99x |
| branch_if | 861 ns | 1.00x |

- [20261115T024902Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/linux-x86_64-c16-bare-intel/20261115T024902Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210137Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.72 us | 1.00x |
| branch_if | 1.72 us | 1.00x |

- [20260510T213318Z-b042648](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20260510T213318Z-b042648/report.md)
- [20261114T210137Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20261114T210137Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025232Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 27.44 us | 1.00x |
| branch_if | 29.93 us | 1.09x |

- [20261115T025232Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/linux-x86_64-c16-bare-intel/20261115T025232Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210143Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 26.15 us | 1.00x |
| branch_if | 26.09 us | 1.00x |

- [20260510T213324Z-b042648](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20260510T213324Z-b042648/report.md)
- [20261114T210143Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20261114T210143Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024719Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 429 ns | 1.94x |
| branch_if | 221 ns | 1.00x |

- [20261115T024719Z-266650fb-dirty](branch_pattern/branch_pattern_n256/linux-x86_64-c16-bare-intel/20261115T024719Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210136Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 445 ns | 1.12x |
| branch_if | 396 ns | 1.00x |

- [20260510T213317Z-b042648](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20260510T213317Z-b042648/report.md)
- [20261114T210136Z-266650fb-dirty](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20261114T210136Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025046Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.29 us | 2.00x |
| branch_if | 3.15 us | 1.00x |

- [20261115T025046Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/linux-x86_64-c16-bare-intel/20261115T025046Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210139Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.72 us | 1.00x |
| branch_if | 6.72 us | 1.00x |

- [20260510T213320Z-b042648](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20260510T213320Z-b042648/report.md)
- [20261114T210139Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20261114T210139Z-266650fb-dirty/report.md)

## branch_pattern/branch_pattern_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024533Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 112 ns | 1.79x |
| branch_if | 63 ns | 1.00x |

- [20261115T024533Z-266650fb-dirty](branch_pattern/branch_pattern_n64/linux-x86_64-c16-bare-intel/20261115T024533Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210134Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 93 ns | 1.03x |
| branch_if | 90 ns | 1.00x |

- [20260510T213316Z-b042648](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20260510T213316Z-b042648/report.md)
- [20261114T210134Z-266650fb-dirty](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20261114T210134Z-266650fb-dirty/report.md)

## branch_predictability/branch_predictability_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025809Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 1.15 us | 1.00x |
| bp_skew_high | 1.15 us | 1.00x |
| bp_skew_low | 1.15 us | 1.00x |

- [20261115T025809Z-266650fb-dirty](branch_predictability/branch_predictability_n1024/linux-x86_64-c16-bare-intel/20261115T025809Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030315Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 18.27 us | 1.00x |
| bp_skew_high | 18.27 us | 1.00x |
| bp_skew_low | 18.27 us | 1.00x |

- [20261115T030315Z-266650fb-dirty](branch_predictability/branch_predictability_n16384/linux-x86_64-c16-bare-intel/20261115T030315Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025534Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 292 ns | 1.00x |
| bp_skew_high | 292 ns | 1.00x |
| bp_skew_low | 292 ns | 1.00x |

- [20261115T025534Z-266650fb-dirty](branch_predictability/branch_predictability_n256/linux-x86_64-c16-bare-intel/20261115T025534Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030041Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 4.57 us | 1.22x |
| bp_skew_high | 3.75 us | 1.00x |
| bp_skew_low | 4.57 us | 1.22x |

- [20261115T030041Z-266650fb-dirty](branch_predictability/branch_predictability_n4096/linux-x86_64-c16-bare-intel/20261115T030041Z-266650fb-dirty/report.md)

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

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 16864.64 ms | 1.08x |
| llvm_non_incremental | 15672.26 ms | 1.00x |

- [20261116T063249Z-f04daa0a-dirty](build-static/clean_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/leaf_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 3379.88 ms | 1.00x |
| llvm_non_incremental | 5451.34 ms | 1.61x |

- [20261116T063249Z-f04daa0a-dirty](build-static/leaf_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/mid_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 693.37 ms | 1.00x |
| llvm_non_incremental | 5582.15 ms | 8.05x |

- [20261116T063249Z-f04daa0a-dirty](build-static/mid_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## build-static/root_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 1119.72 ms | 1.00x |
| llvm_non_incremental | 14310.33 ms | 12.78x |

- [20261116T063249Z-f04daa0a-dirty](build-static/root_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

## cache_layout/cache_layout_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030857Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 28 ns | 1.00x |
| layout_soa | 28 ns | 1.00x |

- [20261115T030857Z-266650fb-dirty](cache_layout/cache_layout_n1024/linux-x86_64-c16-bare-intel/20261115T030857Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210300Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 13 ns | 1.03x |
| layout_soa | 12 ns | 1.00x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210300Z-266650fb-dirty](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20261114T210300Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031223Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 333 ns | 1.00x |
| layout_soa | 333 ns | 1.00x |

- [20261115T031223Z-266650fb-dirty](cache_layout/cache_layout_n16384/linux-x86_64-c16-bare-intel/20261115T031223Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210303Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 371 ns | 1.00x |
| layout_soa | 386 ns | 1.04x |

- [20260510T214108Z-f6ee560](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20260510T214108Z-f6ee560/report.md)
- [20261114T210303Z-266650fb-dirty](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20261114T210303Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030714Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 12 ns | 1.01x |
| layout_soa | 12 ns | 1.00x |

- [20261115T030714Z-266650fb-dirty](cache_layout/cache_layout_n256/linux-x86_64-c16-bare-intel/20261115T030714Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210259Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 1 ns | 1.50x |
| layout_soa | 1 ns | 1.00x |

- [20260510T214105Z-f6ee560](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20260510T214105Z-f6ee560/report.md)
- [20261114T210259Z-266650fb-dirty](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20261114T210259Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031040Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 90 ns | 1.01x |
| layout_soa | 89 ns | 1.00x |

- [20261115T031040Z-266650fb-dirty](cache_layout/cache_layout_n4096/linux-x86_64-c16-bare-intel/20261115T031040Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210301Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 65 ns | 1.00x |
| layout_soa | 69 ns | 1.06x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210301Z-266650fb-dirty](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20261114T210301Z-266650fb-dirty/report.md)

## cache_layout/cache_layout_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030527Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 10 ns | 1.00x |
| layout_soa | 10 ns | 1.00x |

- [20261115T030527Z-266650fb-dirty](cache_layout/cache_layout_n64/linux-x86_64-c16-bare-intel/20261115T030527Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210257Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 0 ns | 1.00x |
| layout_soa | 0 ns | 1.00x |

- [20260510T214104Z-f6ee560](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20260510T214104Z-f6ee560/report.md)
- [20261114T210257Z-266650fb-dirty](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20261114T210257Z-266650fb-dirty/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n128

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032537Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 746 ns | 1.02x |
| dep_graph_dense | 23.42 us | 31.93x |
| dep_graph_hybrid | 734 ns | 1.00x |

- [20261115T032537Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n128/linux-x86_64-c16-bare-intel/20261115T032537Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031800Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 89 ns | 1.00x |
| dep_graph_dense | 439 ns | 4.93x |
| dep_graph_hybrid | 447 ns | 5.02x |

- [20261115T031800Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n16/linux-x86_64-c16-bare-intel/20261115T031800Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032812Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 1.49 us | 1.02x |
| dep_graph_dense | 92.67 us | 63.39x |
| dep_graph_hybrid | 1.46 us | 1.00x |

- [20261115T032812Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n256/linux-x86_64-c16-bare-intel/20261115T032812Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032032Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 193 ns | 1.00x |
| dep_graph_dense | 1.60 us | 8.29x |
| dep_graph_hybrid | 208 ns | 1.08x |

- [20261115T032032Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n32/linux-x86_64-c16-bare-intel/20261115T032032Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032304Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 377 ns | 1.00x |
| dep_graph_dense | 6.02 us | 15.96x |
| dep_graph_hybrid | 404 ns | 1.07x |

- [20261115T032304Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n64/linux-x86_64-c16-bare-intel/20261115T032304Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031525Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 43 ns | 1.00x |
| dep_graph_dense | 124 ns | 2.88x |
| dep_graph_hybrid | 125 ns | 2.89x |

- [20261115T031525Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n8/linux-x86_64-c16-bare-intel/20261115T031525Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033621Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 150 ns | 1.00x |
| dispatch_runtime | 193 ns | 1.29x |
| dispatch_volatile | 182 ns | 1.22x |

- [20261115T033621Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n1024/linux-x86_64-c16-bare-intel/20261115T033621Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034125Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2.29 us | 1.00x |
| dispatch_runtime | 11.44 us | 5.00x |
| dispatch_volatile | 2.85 us | 1.25x |

- [20261115T034125Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n16384/linux-x86_64-c16-bare-intel/20261115T034125Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033349Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 43 ns | 1.05x |
| dispatch_runtime | 41 ns | 1.00x |
| dispatch_volatile | 46 ns | 1.13x |

- [20261115T033349Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n256/linux-x86_64-c16-bare-intel/20261115T033349Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033853Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 578 ns | 1.00x |
| dispatch_runtime | 741 ns | 1.28x |
| dispatch_volatile | 718 ns | 1.24x |

- [20261115T033853Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n4096/linux-x86_64-c16-bare-intel/20261115T033853Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033114Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 16 ns | 1.02x |
| dispatch_runtime | 16 ns | 1.00x |
| dispatch_volatile | 16 ns | 1.03x |

- [20261115T033114Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n64/linux-x86_64-c16-bare-intel/20261115T033114Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034930Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 150 ns | 1.00x |
| dispatch_direct | 150 ns | 1.00x |
| dispatch_trait | 150 ns | 1.00x |

- [20261115T034930Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/linux-x86_64-c16-bare-intel/20261115T034930Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T210516Z-266650fb`

| variant | median | vs best |
|---|---|---|
| dispatch_trait | 123 ns | 1.00x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210515Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20261114T210515Z-266650fb-dirty/report.md)

## dispatch_static/dispatch_static_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T035434Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2.29 us | 1.00x |
| dispatch_direct | 2.29 us | 1.00x |
| dispatch_trait | 2.29 us | 1.00x |

- [20261115T035434Z-266650fb-dirty](dispatch_static/dispatch_static_n16384/linux-x86_64-c16-bare-intel/20261115T035434Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034658Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 43 ns | 1.00x |
| dispatch_direct | 43 ns | 1.00x |
| dispatch_trait | 43 ns | 1.00x |

- [20261115T034658Z-266650fb-dirty](dispatch_static/dispatch_static_n256/linux-x86_64-c16-bare-intel/20261115T034658Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T035202Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 578 ns | 1.00x |
| dispatch_direct | 578 ns | 1.00x |
| dispatch_trait | 578 ns | 1.00x |

- [20261115T035202Z-266650fb-dirty](dispatch_static/dispatch_static_n4096/linux-x86_64-c16-bare-intel/20261115T035202Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034423Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 16 ns | 1.00x |
| dispatch_direct | 16 ns | 1.00x |
| dispatch_trait | 16 ns | 1.00x |

- [20261115T034423Z-266650fb-dirty](dispatch_static/dispatch_static_n64/linux-x86_64-c16-bare-intel/20261115T034423Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040242Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 196 ns | 1.66x |
| ema_neon | 197 ns | 1.67x |
| ema_scalar | 118 ns | 1.00x |

- [20261115T040242Z-266650fb-dirty](ema_axis_h/ema_axis_h_n1024/linux-x86_64-c16-bare-intel/20261115T040242Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040747Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 2.93 us | 1.66x |
| ema_neon | 2.94 us | 1.67x |
| ema_scalar | 1.76 us | 1.00x |

- [20261115T040747Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/linux-x86_64-c16-bare-intel/20261115T040747Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T210606Z-266650fb`

| variant | median | vs best |
|---|---|---|
| ema_scalar | 1.69 us | 1.00x |

- [20260510T204031Z-3c0d0d5](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20260510T204031Z-3c0d0d5/report.md)
- [20261114T210605Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20261114T210605Z-266650fb-dirty/report.md)

## ema_axis_h/ema_axis_h_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040011Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 59 ns | 1.62x |
| ema_neon | 59 ns | 1.62x |
| ema_scalar | 36 ns | 1.00x |

- [20261115T040011Z-266650fb-dirty](ema_axis_h/ema_axis_h_n256/linux-x86_64-c16-bare-intel/20261115T040011Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040514Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 742 ns | 1.66x |
| ema_neon | 744 ns | 1.67x |
| ema_scalar | 446 ns | 1.00x |

- [20261115T040514Z-266650fb-dirty](ema_axis_h/ema_axis_h_n4096/linux-x86_64-c16-bare-intel/20261115T040514Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T035736Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 13 ns | 1.06x |
| ema_neon | 12 ns | 1.00x |
| ema_scalar | 15 ns | 1.20x |

- [20261115T035736Z-266650fb-dirty](ema_axis_h/ema_axis_h_n64/linux-x86_64-c16-bare-intel/20261115T035736Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041327Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 2.00 us | 2.32x |
| ema_form_intshift | 863 ns | 1.00x |
| ema_form_q32 | 1.43 us | 1.66x |

- [20261115T041327Z-266650fb-dirty](ema_formulation/ema_formulation_n1024/linux-x86_64-c16-bare-intel/20261115T041327Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041834Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 31.96 us | 2.33x |
| ema_form_intshift | 13.70 us | 1.00x |
| ema_form_q32 | 22.83 us | 1.67x |

- [20261115T041834Z-266650fb-dirty](ema_formulation/ema_formulation_n16384/linux-x86_64-c16-bare-intel/20261115T041834Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041052Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 509 ns | 2.29x |
| ema_form_intshift | 222 ns | 1.00x |
| ema_form_q32 | 363 ns | 1.64x |

- [20261115T041052Z-266650fb-dirty](ema_formulation/ema_formulation_n256/linux-x86_64-c16-bare-intel/20261115T041052Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041559Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 8.00 us | 2.33x |
| ema_form_intshift | 3.43 us | 1.00x |
| ema_form_q32 | 5.72 us | 1.67x |

- [20261115T041559Z-266650fb-dirty](ema_formulation/ema_formulation_n4096/linux-x86_64-c16-bare-intel/20261115T041559Z-266650fb-dirty/report.md)

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

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013654Z-23953983](engine_vs_std/accumulator_n1048576/linux-x86_64-c4-container-intel/20261114T013654Z-23953983/report.md)

## engine_vs_std/accumulator_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013321Z-c894a779](engine_vs_std/accumulator_n4096/linux-x86_64-c4-container-intel/20261114T013321Z-c894a779/report.md)

## engine_vs_std/accumulator_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013855Z-23953983](engine_vs_std/accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T013855Z-23953983/report.md)

## engine_vs_std/accumulator_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013507Z-23953983](engine_vs_std/accumulator_n65536/linux-x86_64-c4-container-intel/20261114T013507Z-23953983/report.md)

## engine_vs_std/branching_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014141Z-23953983](engine_vs_std/branching_n1048576/linux-x86_64-c4-container-intel/20261114T014141Z-23953983/report.md)

## engine_vs_std/branching_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013949Z-23953983](engine_vs_std/branching_n4096/linux-x86_64-c4-container-intel/20261114T013949Z-23953983/report.md)

## engine_vs_std/branching_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014245Z-23953983](engine_vs_std/branching_n4194304/linux-x86_64-c4-container-intel/20261114T014245Z-23953983/report.md)

## engine_vs_std/branching_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014044Z-23953983](engine_vs_std/branching_n65536/linux-x86_64-c4-container-intel/20261114T014044Z-23953983/report.md)

## engine_vs_std/element_wise_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014531Z-23953983](engine_vs_std/element_wise_n1048576/linux-x86_64-c4-container-intel/20261114T014531Z-23953983/report.md)

## engine_vs_std/element_wise_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014339Z-23953983](engine_vs_std/element_wise_n4096/linux-x86_64-c4-container-intel/20261114T014339Z-23953983/report.md)

## engine_vs_std/element_wise_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014634Z-23953983](engine_vs_std/element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014634Z-23953983/report.md)

## engine_vs_std/element_wise_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014434Z-23953983](engine_vs_std/element_wise_n65536/linux-x86_64-c4-container-intel/20261114T014434Z-23953983/report.md)

## engine_vs_std/startup_accumulator_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014741Z-23953983](engine_vs_std/startup_accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T014741Z-23953983/report.md)

## engine_vs_std/startup_branching_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014850Z-23953983](engine_vs_std/startup_branching_n4194304/linux-x86_64-c4-container-intel/20261114T014850Z-23953983/report.md)

## engine_vs_std/startup_element_wise_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014957Z-23953983](engine_vs_std/startup_element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014957Z-23953983/report.md)

## engine_vs_std/startup_wide_parallel_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015143Z-23953983](engine_vs_std/startup_wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T015143Z-23953983/report.md)

## engine_vs_std/wide_parallel_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015712Z-23953983](engine_vs_std/wide_parallel_n1048576/linux-x86_64-c4-container-intel/20261114T015712Z-23953983/report.md)

## engine_vs_std/wide_parallel_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015326Z-23953983](engine_vs_std/wide_parallel_n4096/linux-x86_64-c4-container-intel/20261114T015326Z-23953983/report.md)

## engine_vs_std/wide_parallel_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T020005Z-23953983](engine_vs_std/wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T020005Z-23953983/report.md)

## engine_vs_std/wide_parallel_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015511Z-23953983](engine_vs_std/wide_parallel_n65536/linux-x86_64-c4-container-intel/20261114T015511Z-23953983/report.md)

## fiber_theory/fiber_theory_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042302Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 95 ns | 1.00x |
| fib_spectral | 72.54 us | 761.22x |

- [20261115T042302Z-266650fb-dirty](fiber_theory/fiber_theory_n16/linux-x86_64-c16-bare-intel/20261115T042302Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210756Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 96 ns | 1.00x |
| fib_spectral | 64.19 us | 667.28x |

- [20260719T190052Z-e4cd1757-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20260719T190052Z-e4cd1757-dirty/report.md)
- [20261114T210756Z-266650fb-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20261114T210756Z-266650fb-dirty/report.md)

## fiber_theory/fiber_theory_n32

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042449Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 191 ns | 1.00x |
| fib_spectral | 86.81 us | 455.22x |

- [20261115T042449Z-266650fb-dirty](fiber_theory/fiber_theory_n32/linux-x86_64-c16-bare-intel/20261115T042449Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210800Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 191 ns | 1.00x |
| fib_spectral | 80.28 us | 419.41x |

- [20260719T190056Z-e4cd1757-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20260719T190056Z-e4cd1757-dirty/report.md)
- [20261114T210800Z-266650fb-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20261114T210800Z-266650fb-dirty/report.md)

## fiber_theory/fiber_theory_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042637Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 401 ns | 1.00x |
| fib_spectral | 113.57 us | 282.93x |

- [20261115T042637Z-266650fb-dirty](fiber_theory/fiber_theory_n64/linux-x86_64-c16-bare-intel/20261115T042637Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210807Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 392 ns | 1.00x |
| fib_spectral | 128.20 us | 326.63x |

- [20260719T190101Z-e4cd1757-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20260719T190101Z-e4cd1757-dirty/report.md)
- [20261114T210807Z-266650fb-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20261114T210807Z-266650fb-dirty/report.md)

## fiber_theory/fiber_theory_n8

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042113Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 50 ns | 1.00x |
| fib_spectral | 16.08 us | 320.29x |

- [20261115T042113Z-266650fb-dirty](fiber_theory/fiber_theory_n8/linux-x86_64-c16-bare-intel/20261115T042113Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210752Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 36 ns | 1.00x |
| fib_spectral | 16.63 us | 459.44x |

- [20260719T190048Z-e4cd1757-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20260719T190048Z-e4cd1757-dirty/report.md)
- [20261114T210752Z-266650fb-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20261114T210752Z-266650fb-dirty/report.md)

## float_width/float_width_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043214Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 2.57 us | 4.44x |
| float_f64 | 2.53 us | 4.38x |
| float_i64_scaled | 578 ns | 1.00x |

- [20261115T043214Z-266650fb-dirty](float_width/float_width_n1024/linux-x86_64-c16-bare-intel/20261115T043214Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043721Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 40.51 us | 4.43x |
| float_f64 | 40.51 us | 4.43x |
| float_i64_scaled | 9.14 us | 1.00x |

- [20261115T043721Z-266650fb-dirty](float_width/float_width_n16384/linux-x86_64-c16-bare-intel/20261115T043721Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042939Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 527 ns | 3.49x |
| float_f64 | 532 ns | 3.52x |
| float_i64_scaled | 151 ns | 1.00x |

- [20261115T042939Z-266650fb-dirty](float_width/float_width_n256/linux-x86_64-c16-bare-intel/20261115T042939Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043446Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 10.41 us | 4.55x |
| float_f64 | 10.55 us | 4.61x |
| float_i64_scaled | 2.29 us | 1.00x |

- [20261115T043446Z-266650fb-dirty](float_width/float_width_n4096/linux-x86_64-c16-bare-intel/20261115T043446Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T044118Z-266650fb`

| variant | median | vs best |
|---|---|---|
| fpt_table_16 | 178 ns | 1.00x |

- [20261115T044117Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/linux-x86_64-c16-bare-intel/20261115T044117Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210922Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 236 ns | 1.00x |
| fpt_table_16 | 328 ns | 1.39x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210922Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20261114T210922Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044445Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 12.99 us | 1.00x |
| fpt_table_16 | 15.12 us | 1.16x |

- [20261115T044445Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/linux-x86_64-c16-bare-intel/20261115T044445Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210926Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 10.26 us | 1.00x |
| fpt_table_16 | 11.37 us | 1.11x |

- [20260511T051112Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20260511T051112Z-9b76747/report.md)
- [20261114T210926Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20261114T210926Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043931Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 34 ns | 1.13x |
| fpt_table_16 | 30 ns | 1.00x |

- [20261115T043931Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/linux-x86_64-c16-bare-intel/20261115T043931Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210920Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 71 ns | 1.58x |
| fpt_table_16 | 45 ns | 1.00x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210920Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20261114T210920Z-266650fb-dirty/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044301Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 590 ns | 1.00x |
| fpt_table_16 | 1.99 us | 3.36x |

- [20261115T044301Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/linux-x86_64-c16-bare-intel/20261115T044301Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T210923Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 841 ns | 1.00x |
| fpt_table_16 | 1.30 us | 1.55x |

- [20260511T051110Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20260511T051110Z-9b76747/report.md)
- [20261114T210923Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20261114T210923Z-266650fb-dirty/report.md)

## fold_strategy/fold_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045022Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 79 ns | 1.76x |
| fold_quad | 45 ns | 1.00x |
| fold_sequential | 150 ns | 3.31x |

- [20261115T045022Z-266650fb-dirty](fold_strategy/fold_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T045022Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045526Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 1.15 us | 1.98x |
| fold_quad | 580 ns | 1.00x |
| fold_sequential | 2.29 us | 3.94x |

- [20261115T045526Z-266650fb-dirty](fold_strategy/fold_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T045526Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044747Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 26 ns | 1.23x |
| fold_quad | 21 ns | 1.00x |
| fold_sequential | 43 ns | 2.03x |

- [20261115T044747Z-266650fb-dirty](fold_strategy/fold_strategy_n256/linux-x86_64-c16-bare-intel/20261115T044747Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045253Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 294 ns | 1.93x |
| fold_quad | 152 ns | 1.00x |
| fold_sequential | 578 ns | 3.79x |

- [20261115T045253Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T045253Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211007Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 331 ns | 1.68x |
| fold_quad | 197 ns | 1.00x |
| fold_sequential | 617 ns | 3.14x |

- [20260511T023055Z-7ed697d](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T023055Z-7ed697d/report.md)
- [20261114T211007Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211007Z-266650fb-dirty/report.md)

## frame_wide/heavy_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151355Z-c1eee3f5-dirty](frame_wide/heavy_n65536/linux-x86_64-c4-container-intel/20261113T151355Z-c1eee3f5-dirty/report.md)

## frame_wide/light_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151715Z-c1eee3f5-dirty](frame_wide/light_n65536/linux-x86_64-c4-container-intel/20261113T151715Z-c1eee3f5-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050123Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 97 ns | 1.00x |
| fxpmul_precise | 151 ns | 1.55x |

- [20261115T050123Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T050123Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211059Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 102 ns | 1.00x |
| fxpmul_precise | 111 ns | 1.10x |

- [20260510T213422Z-b042648](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213422Z-b042648/report.md)
- [20261114T211059Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211059Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050449Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.43 us | 1.00x |
| fxpmul_precise | 2.29 us | 1.60x |

- [20261115T050449Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T050449Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211102Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.70 us | 1.00x |
| fxpmul_precise | 1.98 us | 1.17x |

- [20260510T213424Z-b042648](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213424Z-b042648/report.md)
- [20261114T211102Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211102Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045940Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 30 ns | 1.00x |
| fxpmul_precise | 44 ns | 1.44x |

- [20261115T045940Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/linux-x86_64-c16-bare-intel/20261115T045940Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211058Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 19 ns | 1.00x |
| fxpmul_precise | 27 ns | 1.43x |

- [20260510T213421Z-b042648](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213421Z-b042648/report.md)
- [20261114T211058Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211058Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050306Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 364 ns | 1.00x |
| fxpmul_precise | 578 ns | 1.59x |

- [20261115T050306Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T050306Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211101Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 401 ns | 1.00x |
| fxpmul_precise | 501 ns | 1.25x |

- [20260510T213423Z-b042648](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213423Z-b042648/report.md)
- [20261114T211101Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211101Z-266650fb-dirty/report.md)

## fxpmul_strategy/fxpmul_strategy_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045753Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 14 ns | 1.00x |
| fxpmul_precise | 17 ns | 1.25x |

- [20261115T045753Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/linux-x86_64-c16-bare-intel/20261115T045753Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261114T211056Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 3 ns | 1.00x |
| fxpmul_precise | 3 ns | 1.00x |

- [20260510T213420Z-b042648](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213420Z-b042648/report.md)
- [20261114T211056Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211056Z-266650fb-dirty/report.md)

## hash_algos/hash_algos_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051258Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 1.15 us | 6.03x |
| hash_sip | 202 ns | 1.06x |
| hash_xxh | 190 ns | 1.00x |

- [20261115T051258Z-266650fb-dirty](hash_algos/hash_algos_n1024/linux-x86_64-c16-bare-intel/20261115T051258Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051803Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 18.27 us | 6.39x |
| hash_sip | 3.07 us | 1.07x |
| hash_xxh | 2.86 us | 1.00x |

- [20261115T051803Z-266650fb-dirty](hash_algos/hash_algos_n16384/linux-x86_64-c16-bare-intel/20261115T051803Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051026Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 292 ns | 5.65x |
| hash_sip | 58 ns | 1.13x |
| hash_xxh | 52 ns | 1.00x |

- [20261115T051026Z-266650fb-dirty](hash_algos/hash_algos_n256/linux-x86_64-c16-bare-intel/20261115T051026Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051530Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 4.57 us | 6.30x |
| hash_sip | 782 ns | 1.08x |
| hash_xxh | 725 ns | 1.00x |

- [20261115T051530Z-266650fb-dirty](hash_algos/hash_algos_n4096/linux-x86_64-c16-bare-intel/20261115T051530Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050751Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 78 ns | 4.19x |
| hash_sip | 22 ns | 1.21x |
| hash_xxh | 19 ns | 1.00x |

- [20261115T050751Z-266650fb-dirty](hash_algos/hash_algos_n64/linux-x86_64-c16-bare-intel/20261115T050751Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T052342Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 1.15 us | 1.00x |
| inline_default | 1.15 us | 1.00x |
| inline_never | 1.15 us | 1.00x |

- [20261115T052342Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T052342Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T211227Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inline_default | 1.31 us | 1.00x |
| inline_never | 1.32 us | 1.00x |

- [20260511T003101Z-63d18f0](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T003101Z-63d18f0/report.md)
- [20261114T211226Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211226Z-266650fb-dirty/report.md)

## inline_strategy/inline_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T052848Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 18.27 us | 1.00x |
| inline_default | 18.27 us | 1.00x |
| inline_never | 18.27 us | 1.00x |

- [20261115T052848Z-266650fb-dirty](inline_strategy/inline_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T052848Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T052107Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 292 ns | 1.00x |
| inline_default | 292 ns | 1.00x |
| inline_never | 293 ns | 1.00x |

- [20261115T052107Z-266650fb-dirty](inline_strategy/inline_strategy_n256/linux-x86_64-c16-bare-intel/20261115T052107Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T052615Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inline_never | 4.57 us | 1.00x |

- [20261115T052614Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T052614Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053425Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 297 ns | 11.80x |
| iter_seq | 150 ns | 5.93x |
| iter_strided | 25 ns | 1.00x |

- [20261115T053425Z-266650fb-dirty](iter_patterns/iter_patterns_n1024/linux-x86_64-c16-bare-intel/20261115T053425Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053929Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 4.63 us | 15.86x |
| iter_seq | 2.29 us | 7.83x |
| iter_strided | 292 ns | 1.00x |

- [20261115T053929Z-266650fb-dirty](iter_patterns/iter_patterns_n16384/linux-x86_64-c16-bare-intel/20261115T053929Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053150Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 82 ns | 7.10x |
| iter_seq | 43 ns | 3.71x |
| iter_strided | 12 ns | 1.00x |

- [20261115T053150Z-266650fb-dirty](iter_patterns/iter_patterns_n256/linux-x86_64-c16-bare-intel/20261115T053150Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053657Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 1.16 us | 14.85x |
| iter_seq | 577 ns | 7.36x |
| iter_strided | 78 ns | 1.00x |

- [20261115T053657Z-266650fb-dirty](iter_patterns/iter_patterns_n4096/linux-x86_64-c16-bare-intel/20261115T053657Z-266650fb-dirty/report.md)

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

### linux-x86_64-c4-container-intel, 2 run(s)

Latest: `20261119T134226Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| default_relink | 162.10 ms | 2.03x |
| default_replay | 79.97 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/dev_n25/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)
- [20261119T134226Z-07a9e9a3](link-static/dev_n25/linux-x86_64-c4-container-intel/20261119T134226Z-07a9e9a3/report.md)

## link-static/dev_n97

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T083526Z-b2bbf4ba`

| variant | median | vs best |
|---|---|---|
| default_relink | 524.41 ms | 1.64x |
| default_replay | 319.64 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/dev_n97/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)

## link-static/release_n25

### linux-x86_64-c4-container-intel, 2 run(s)

Latest: `20261119T134226Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| default_relink | 171.70 ms | 2.56x |
| default_replay | 67.19 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/release_n25/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)
- [20261119T134226Z-07a9e9a3](link-static/release_n25/linux-x86_64-c4-container-intel/20261119T134226Z-07a9e9a3/report.md)

## load_alignment/load_alignment_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T054506Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 150 ns | 1.00x |
| load_byte_pack | 150 ns | 1.00x |
| load_unaligned | 150 ns | 1.00x |

- [20261115T054506Z-266650fb-dirty](load_alignment/load_alignment_n1024/linux-x86_64-c16-bare-intel/20261115T054506Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055010Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 2.29 us | 1.00x |
| load_byte_pack | 2.29 us | 1.00x |
| load_unaligned | 2.29 us | 1.00x |

- [20261115T055010Z-266650fb-dirty](load_alignment/load_alignment_n16384/linux-x86_64-c16-bare-intel/20261115T055010Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T054231Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 43 ns | 1.00x |
| load_byte_pack | 43 ns | 1.00x |
| load_unaligned | 43 ns | 1.00x |

- [20261115T054231Z-266650fb-dirty](load_alignment/load_alignment_n256/linux-x86_64-c16-bare-intel/20261115T054231Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T054738Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 578 ns | 1.00x |
| load_byte_pack | 578 ns | 1.00x |
| load_unaligned | 578 ns | 1.00x |

- [20261115T054738Z-266650fb-dirty](load_alignment/load_alignment_n4096/linux-x86_64-c16-bare-intel/20261115T054738Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055547Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 106 ns | 1.00x |
| match_64_arms | 181 ns | 1.70x |
| match_8_arms | 161 ns | 1.51x |

- [20261115T055547Z-266650fb-dirty](match_arm_count/match_arm_count_n1024/linux-x86_64-c16-bare-intel/20261115T055547Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060052Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 1.58 us | 1.00x |
| match_64_arms | 11.31 us | 7.17x |
| match_8_arms | 11.37 us | 7.21x |

- [20261115T060052Z-266650fb-dirty](match_arm_count/match_arm_count_n16384/linux-x86_64-c16-bare-intel/20261115T060052Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055312Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 33 ns | 1.08x |
| match_64_arms | 39 ns | 1.29x |
| match_8_arms | 30 ns | 1.00x |

- [20261115T055312Z-266650fb-dirty](match_arm_count/match_arm_count_n256/linux-x86_64-c16-bare-intel/20261115T055312Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055819Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 404 ns | 1.00x |
| match_64_arms | 747 ns | 1.85x |
| match_8_arms | 663 ns | 1.64x |

- [20261115T055819Z-266650fb-dirty](match_arm_count/match_arm_count_n4096/linux-x86_64-c16-bare-intel/20261115T055819Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060900Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 1.23 us | 1.00x |
| memcpy_naive | 1.23 us | 1.00x |
| memcpy_unrolled | 1.23 us | 1.00x |

- [20261115T060900Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n1024/linux-x86_64-c16-bare-intel/20261115T060900Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061407Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 18.27 us | 1.00x |
| memcpy_naive | 18.27 us | 1.00x |
| memcpy_unrolled | 18.27 us | 1.00x |

- [20261115T061407Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n16384/linux-x86_64-c16-bare-intel/20261115T061407Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060629Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 376 ns | 1.00x |
| memcpy_naive | 375 ns | 1.00x |
| memcpy_unrolled | 376 ns | 1.00x |

- [20261115T060629Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n256/linux-x86_64-c16-bare-intel/20261115T060629Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061133Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 4.25 us | 1.00x |
| memcpy_naive | 4.66 us | 1.10x |
| memcpy_unrolled | 4.66 us | 1.10x |

- [20261115T061133Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n4096/linux-x86_64-c16-bare-intel/20261115T061133Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060354Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 78 ns | 1.00x |
| memcpy_naive | 78 ns | 1.00x |
| memcpy_unrolled | 78 ns | 1.00x |

- [20261115T060354Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n64/linux-x86_64-c16-bare-intel/20261115T060354Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061944Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 51 ns | 1.01x |
| minmax_branchless | 50 ns | 1.00x |
| minmax_intrinsic | 52 ns | 1.02x |

- [20261115T061944Z-266650fb-dirty](minmax_strategy/minmax_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T061944Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T062448Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 720 ns | 1.08x |
| minmax_branchless | 670 ns | 1.00x |
| minmax_intrinsic | 720 ns | 1.07x |

- [20261115T062448Z-266650fb-dirty](minmax_strategy/minmax_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T062448Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061709Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 34 ns | 1.00x |
| minmax_branchless | 34 ns | 1.00x |
| minmax_intrinsic | 34 ns | 1.00x |

- [20261115T061709Z-266650fb-dirty](minmax_strategy/minmax_strategy_n256/linux-x86_64-c16-bare-intel/20261115T061709Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T062215Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 185 ns | 1.00x |
| minmax_branchless | 188 ns | 1.01x |
| minmax_intrinsic | 188 ns | 1.01x |

- [20261115T062215Z-266650fb-dirty](minmax_strategy/minmax_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T062215Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063029Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 2.93 us | 2.41x |
| mod_pow2_const | 1.22 us | 1.00x |
| mod_var | 6.29 us | 5.16x |

- [20261115T063029Z-266650fb-dirty](modulo_strategy/modulo_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T063029Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063541Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 57.06 us | 2.94x |
| mod_pow2_const | 19.41 us | 1.00x |
| mod_var | 100.43 us | 5.18x |

- [20261115T063541Z-266650fb-dirty](modulo_strategy/modulo_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T063541Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T062754Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 898 ns | 2.90x |
| mod_pow2_const | 310 ns | 1.00x |
| mod_var | 1.58 us | 5.09x |

- [20261115T062754Z-266650fb-dirty](modulo_strategy/modulo_strategy_n256/linux-x86_64-c16-bare-intel/20261115T062754Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063303Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 14.27 us | 2.94x |
| mod_pow2_const | 4.85 us | 1.00x |
| mod_var | 25.11 us | 5.18x |

- [20261115T063303Z-266650fb-dirty](modulo_strategy/modulo_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T063303Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064350Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 1.80 us | 19.36x |
| popcount_std | 93 ns | 1.00x |
| popcount_swar | 189 ns | 2.03x |

- [20261115T064350Z-266650fb-dirty](popcount_strategy/popcount_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T064350Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064856Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 37.29 us | 29.04x |
| popcount_std | 1.28 us | 1.00x |
| popcount_swar | 2.73 us | 2.13x |

- [20261115T064856Z-266650fb-dirty](popcount_strategy/popcount_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T064856Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064118Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 354 ns | 10.83x |
| popcount_std | 33 ns | 1.00x |
| popcount_swar | 67 ns | 2.05x |

- [20261115T064118Z-266650fb-dirty](popcount_strategy/popcount_strategy_n256/linux-x86_64-c16-bare-intel/20261115T064118Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064622Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 8.04 us | 24.30x |
| popcount_std | 331 ns | 1.00x |
| popcount_swar | 693 ns | 2.09x |

- [20261115T064622Z-266650fb-dirty](popcount_strategy/popcount_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T064622Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063843Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 91 ns | 5.96x |
| popcount_std | 15 ns | 1.00x |
| popcount_swar | 22 ns | 1.42x |

- [20261115T063843Z-266650fb-dirty](popcount_strategy/popcount_strategy_n64/linux-x86_64-c16-bare-intel/20261115T063843Z-266650fb-dirty/report.md)

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

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20260719T162211Z-2c337b67`

| variant | median | vs best |
|---|---|---|
| riv_scr | 8.27 ms | 1.00x |
| riv_snake | 8.31 ms | 1.00x |
| riv_spec | 8.34 ms | 1.01x |

- [20260719T162210Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n2048/macos-aarch64-c8-bare-apple/20260719T162210Z-2c337b67-dirty/report.md)

## rcm_rivals/rcm_rivals_n256

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T090031Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 2.79 us | 1.10x |
| reg_bound_record | 18.11 us | 7.16x |
| reg_bound_strided | 5.62 us | 2.22x |
| reg_bound_units | 2.77 us | 1.10x |
| reg_typed | 3.02 us | 1.19x |
| reg_typed_twin | 2.53 us | 1.00x |

- [20261115T090031Z-266650fb-dirty](registration_binding/registration_binding_n16384/linux-x86_64-c16-bare-intel/20261115T090031Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T091356Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 48.57 us | 1.02x |
| reg_bound_record | 292.09 us | 6.12x |
| reg_bound_strided | 72.48 us | 1.52x |
| reg_bound_units | 48.12 us | 1.01x |
| reg_typed | 47.70 us | 1.00x |
| reg_typed_twin | 47.69 us | 1.00x |

- [20261115T091356Z-266650fb-dirty](registration_binding/registration_binding_n262144/linux-x86_64-c16-bare-intel/20261115T091356Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T085521Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 761 ns | 1.23x |
| reg_bound_record | 5.53 us | 8.91x |
| reg_bound_strided | 1.14 us | 1.83x |
| reg_bound_units | 629 ns | 1.01x |
| reg_typed | 620 ns | 1.00x |
| reg_typed_twin | 620 ns | 1.00x |

- [20261115T085521Z-266650fb-dirty](registration_binding/registration_binding_n4096/linux-x86_64-c16-bare-intel/20261115T085521Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T090613Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 12.35 us | 1.03x |
| reg_bound_record | 89.13 us | 7.46x |
| reg_bound_strided | 22.09 us | 1.85x |
| reg_bound_units | 12.21 us | 1.02x |
| reg_typed | 11.96 us | 1.00x |
| reg_typed_twin | 11.95 us | 1.00x |

- [20261115T090613Z-266650fb-dirty](registration_binding/registration_binding_n65536/linux-x86_64-c16-bare-intel/20261115T090613Z-266650fb-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T081013Z-83f6f25-dirty](registration_binding/registration_binding_n65536/linux-x86_64-c4-container-intel/20261113T081013Z-83f6f25-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261114T232023Z-266650fb`

| variant | median | vs best |
|---|---|---|
| reg_typed_twin | 4.36 us | 1.00x |

- [20261114T232022Z-266650fb-dirty](registration_binding/registration_binding_n65536/macos-aarch64-c8-bare-apple/20261114T232022Z-266650fb-dirty/report.md)

## resource_storage/rsb_bwide_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182437Z-9d530d12](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20260619T182437Z-9d530d12/report.md)
- [20260702T102749Z-677b60f8](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20260702T102749Z-677b60f8/report.md)

## resource_storage/rsb_bwide_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182434Z-9d530d12](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20260619T182434Z-9d530d12/report.md)
- [20260702T102747Z-677b60f8](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20260702T102747Z-677b60f8/report.md)

## resource_storage/rsb_bwide_n4194304

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182613Z-9d530d12](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20260619T182613Z-9d530d12/report.md)
- [20260702T102929Z-677b60f8](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20260702T102929Z-677b60f8/report.md)

## resource_storage/rsb_clean_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183622Z-9d530d12](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20260619T183622Z-9d530d12/report.md)
- [20260702T103426Z-677b60f8](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20260702T103426Z-677b60f8/report.md)

## resource_storage/rsb_clean_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183639Z-9d530d12](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20260619T183639Z-9d530d12/report.md)
- [20260702T103444Z-677b60f8](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20260702T103444Z-677b60f8/report.md)

## resource_storage/rsb_clean_n262144

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T184039Z-9d530d12](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20260619T184039Z-9d530d12/report.md)
- [20260702T103856Z-a2410c71](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20260702T103856Z-a2410c71/report.md)

## resource_storage/rsb_heavy_thrash_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183150Z-9d530d12](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20260619T183150Z-9d530d12/report.md)
- [20260702T102934Z-677b60f8](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20260702T102934Z-677b60f8/report.md)

## resource_storage/rsb_heavy_thrash_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183213Z-9d530d12](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20260619T183213Z-9d530d12/report.md)
- [20260702T102953Z-677b60f8](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20260702T102953Z-677b60f8/report.md)

## resource_storage/rsb_heavy_thrash_n262144

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183619Z-9d530d12](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20260619T183619Z-9d530d12/report.md)
- [20260702T103423Z-677b60f8](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20260702T103423Z-677b60f8/report.md)

## resource_storage/rsb_light_thrash_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182616Z-9d530d12](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20260619T182616Z-9d530d12/report.md)
- [20260702T102311Z-677b60f8](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20260702T102311Z-677b60f8/report.md)

## resource_storage/rsb_light_thrash_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T182633Z-9d530d12](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20260619T182633Z-9d530d12/report.md)
- [20260702T102329Z-677b60f8](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20260702T102329Z-677b60f8/report.md)

## resource_storage/rsb_light_thrash_n262144

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T183144Z-9d530d12](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20260619T183144Z-9d530d12/report.md)
- [20260702T102742Z-677b60f8](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20260702T102742Z-677b60f8/report.md)

## resource_storage/rsb_seqd_n1048576

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T184658Z-9d530d12](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20260619T184658Z-9d530d12/report.md)
- [20260702T104547Z-a2410c71](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20260702T104547Z-a2410c71/report.md)

## resource_storage/rsb_seqd_n16777216

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T202925Z-9d530d12](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20260619T202925Z-9d530d12/report.md)
- [20260702T121947Z-a2410c71](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20260702T121947Z-a2410c71/report.md)

## resource_storage/rsb_seqd_n4194304

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T190539Z-9d530d12](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20260619T190539Z-9d530d12/report.md)
- [20260702T110508Z-a2410c71](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20260702T110508Z-a2410c71/report.md)

## resource_storage/rsb_seqd_n65536

### macos-aarch64-c8-bare-apple, 2 run(s)

- [20260619T184145Z-9d530d12](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20260619T184145Z-9d530d12/report.md)
- [20260702T104015Z-a2410c71](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20260702T104015Z-a2410c71/report.md)

## rotate_strategy/rotate_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T091951Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 65 ns | 1.00x |
| rot_manual_shifts | 79 ns | 1.22x |
| rot_variable_amount | 181 ns | 2.80x |

- [20261115T091951Z-266650fb-dirty](rotate_strategy/rotate_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T091951Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T092455Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 1.15 us | 1.00x |
| rot_manual_shifts | 1.15 us | 1.00x |
| rot_variable_amount | 3.43 us | 2.99x |

- [20261115T092455Z-266650fb-dirty](rotate_strategy/rotate_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T092455Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T091717Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 21 ns | 1.00x |
| rot_manual_shifts | 25 ns | 1.22x |
| rot_variable_amount | 59 ns | 2.81x |

- [20261115T091717Z-266650fb-dirty](rotate_strategy/rotate_strategy_n256/linux-x86_64-c16-bare-intel/20261115T091717Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T092224Z-266650fb`

| variant | median | vs best |
|---|---|---|
| rot_manual_shifts | 293 ns | 1.00x |
| rot_variable_amount | 862 ns | 2.94x |

- [20261115T092223Z-266650fb-dirty](rotate_strategy/rotate_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T092223Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T093033Z-266650fb`

| variant | median | vs best |
|---|---|---|
| sat_saturate | 504 ns | 1.71x |
| sat_wrap | 295 ns | 1.00x |

- [20261115T093032Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n1024/linux-x86_64-c16-bare-intel/20261115T093032Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T093537Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 7.80 us | 1.71x |
| sat_saturate | 7.80 us | 1.71x |
| sat_wrap | 4.57 us | 1.00x |

- [20261115T093537Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n16384/linux-x86_64-c16-bare-intel/20261115T093537Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T092757Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 140 ns | 1.72x |
| sat_saturate | 140 ns | 1.72x |
| sat_wrap | 81 ns | 1.00x |

- [20261115T092757Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n256/linux-x86_64-c16-bare-intel/20261115T092757Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T093304Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 1.97 us | 1.71x |
| sat_saturate | 1.97 us | 1.72x |
| sat_wrap | 1.15 us | 1.00x |

- [20261115T093304Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n4096/linux-x86_64-c16-bare-intel/20261115T093304Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T094114Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 19 ns | 1.00x |
| shr_loop_invariant | 20 ns | 1.03x |
| shr_per_iter | 94 ns | 4.90x |

- [20261115T094114Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n1024/linux-x86_64-c16-bare-intel/20261115T094114Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T094618Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 131 ns | 1.00x |
| shr_loop_invariant | 203 ns | 1.56x |
| shr_per_iter | 1.33 us | 10.22x |

- [20261115T094618Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n16384/linux-x86_64-c16-bare-intel/20261115T094618Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T093839Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 12 ns | 1.00x |
| shr_loop_invariant | 13 ns | 1.06x |
| shr_per_iter | 26 ns | 2.14x |

- [20261115T093839Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n256/linux-x86_64-c16-bare-intel/20261115T093839Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T094346Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 50 ns | 1.00x |
| shr_loop_invariant | 61 ns | 1.23x |
| shr_per_iter | 342 ns | 6.89x |

- [20261115T094346Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n4096/linux-x86_64-c16-bare-intel/20261115T094346Z-266650fb-dirty/report.md)

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

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091914Z-ed2bf53-dirty](single-trunk-convergence/append_w1_big_n16777216/linux-x86_64-c4-container-intel/20261113T091914Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w1_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091545Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n1048576/linux-x86_64-c4-container-intel/20261113T091545Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w1_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091048Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n4096/linux-x86_64-c4-container-intel/20261113T091048Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w1_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091304Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n65536/linux-x86_64-c4-container-intel/20261113T091304Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/append_w2_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T100114Z-74174bb](single-trunk-convergence/append_w2_big_n16777216/linux-x86_64-c4-container-intel/20261113T100114Z-74174bb/report.md)

## single-trunk-convergence/append_w2_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095753Z-74174bb](single-trunk-convergence/append_w2_n1048576/linux-x86_64-c4-container-intel/20261113T095753Z-74174bb/report.md)

## single-trunk-convergence/append_w2_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095257Z-74174bb](single-trunk-convergence/append_w2_n4096/linux-x86_64-c4-container-intel/20261113T095257Z-74174bb/report.md)

## single-trunk-convergence/append_w2_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095513Z-74174bb](single-trunk-convergence/append_w2_n65536/linux-x86_64-c4-container-intel/20261113T095513Z-74174bb/report.md)

## single-trunk-convergence/append_w4_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T111219Z-f12bdc4](single-trunk-convergence/append_w4_big_n16777216/linux-x86_64-c4-container-intel/20261113T111219Z-f12bdc4/report.md)

## single-trunk-convergence/append_w4_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110856Z-f12bdc4](single-trunk-convergence/append_w4_n1048576/linux-x86_64-c4-container-intel/20261113T110856Z-f12bdc4/report.md)

## single-trunk-convergence/append_w4_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110400Z-f12bdc4](single-trunk-convergence/append_w4_n4096/linux-x86_64-c4-container-intel/20261113T110400Z-f12bdc4/report.md)

## single-trunk-convergence/append_w4_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110615Z-f12bdc4](single-trunk-convergence/append_w4_n65536/linux-x86_64-c4-container-intel/20261113T110615Z-f12bdc4/report.md)

## single-trunk-convergence/append_w8_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T120018Z-d11e290-dirty](single-trunk-convergence/append_w8_big_n16777216/linux-x86_64-c4-container-intel/20261113T120018Z-d11e290-dirty/report.md)

## single-trunk-convergence/append_w8_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115657Z-d11e290-dirty](single-trunk-convergence/append_w8_n1048576/linux-x86_64-c4-container-intel/20261113T115657Z-d11e290-dirty/report.md)

## single-trunk-convergence/append_w8_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115200Z-d11e290-dirty](single-trunk-convergence/append_w8_n4096/linux-x86_64-c4-container-intel/20261113T115200Z-d11e290-dirty/report.md)

## single-trunk-convergence/append_w8_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115416Z-d11e290-dirty](single-trunk-convergence/append_w8_n65536/linux-x86_64-c4-container-intel/20261113T115416Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_heavy_w1_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090834Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_big_n1048576/linux-x86_64-c4-container-intel/20261113T090834Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_heavy_w1_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090400Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_n4096/linux-x86_64-c4-container-intel/20261113T090400Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_heavy_w1_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090616Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_n65536/linux-x86_64-c4-container-intel/20261113T090616Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_heavy_w2_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095043Z-74174bb](single-trunk-convergence/fold_heavy_w2_big_n1048576/linux-x86_64-c4-container-intel/20261113T095043Z-74174bb/report.md)

## single-trunk-convergence/fold_heavy_w2_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094611Z-74174bb](single-trunk-convergence/fold_heavy_w2_n4096/linux-x86_64-c4-container-intel/20261113T094611Z-74174bb/report.md)

## single-trunk-convergence/fold_heavy_w2_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094827Z-74174bb](single-trunk-convergence/fold_heavy_w2_n65536/linux-x86_64-c4-container-intel/20261113T094827Z-74174bb/report.md)

## single-trunk-convergence/fold_heavy_w4_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110146Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_big_n1048576/linux-x86_64-c4-container-intel/20261113T110146Z-f12bdc4/report.md)

## single-trunk-convergence/fold_heavy_w4_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105714Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_n4096/linux-x86_64-c4-container-intel/20261113T105714Z-f12bdc4/report.md)

## single-trunk-convergence/fold_heavy_w4_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105929Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_n65536/linux-x86_64-c4-container-intel/20261113T105929Z-f12bdc4/report.md)

## single-trunk-convergence/fold_heavy_w8_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114945Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_big_n1048576/linux-x86_64-c4-container-intel/20261113T114945Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_heavy_w8_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114513Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_n4096/linux-x86_64-c4-container-intel/20261113T114513Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_heavy_w8_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114729Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_n65536/linux-x86_64-c4-container-intel/20261113T114729Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w1_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090146Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_big_n134217728/linux-x86_64-c4-container-intel/20261113T090146Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085554Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_big_n16777216/linux-x86_64-c4-container-intel/20261113T085554Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085316Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n1048576/linux-x86_64-c4-container-intel/20261113T085316Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084844Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n4096/linux-x86_64-c4-container-intel/20261113T084844Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w1_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085057Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n65536/linux-x86_64-c4-container-intel/20261113T085057Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/fold_light_w2_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094358Z-74174bb](single-trunk-convergence/fold_light_w2_big_n134217728/linux-x86_64-c4-container-intel/20261113T094358Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093857Z-74174bb](single-trunk-convergence/fold_light_w2_big_n16777216/linux-x86_64-c4-container-intel/20261113T093857Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093622Z-74174bb](single-trunk-convergence/fold_light_w2_n1048576/linux-x86_64-c4-container-intel/20261113T093622Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093149Z-74174bb](single-trunk-convergence/fold_light_w2_n4096/linux-x86_64-c4-container-intel/20261113T093149Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w2_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093403Z-74174bb](single-trunk-convergence/fold_light_w2_n65536/linux-x86_64-c4-container-intel/20261113T093403Z-74174bb/report.md)

## single-trunk-convergence/fold_light_w4_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105500Z-f12bdc4](single-trunk-convergence/fold_light_w4_big_n134217728/linux-x86_64-c4-container-intel/20261113T105500Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104952Z-f12bdc4](single-trunk-convergence/fold_light_w4_big_n16777216/linux-x86_64-c4-container-intel/20261113T104952Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104719Z-f12bdc4](single-trunk-convergence/fold_light_w4_n1048576/linux-x86_64-c4-container-intel/20261113T104719Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104247Z-f12bdc4](single-trunk-convergence/fold_light_w4_n4096/linux-x86_64-c4-container-intel/20261113T104247Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w4_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104501Z-f12bdc4](single-trunk-convergence/fold_light_w4_n65536/linux-x86_64-c4-container-intel/20261113T104501Z-f12bdc4/report.md)

## single-trunk-convergence/fold_light_w8_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114259Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_big_n134217728/linux-x86_64-c4-container-intel/20261113T114259Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113804Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_big_n16777216/linux-x86_64-c4-container-intel/20261113T113804Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113530Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n1048576/linux-x86_64-c4-container-intel/20261113T113530Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113057Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n4096/linux-x86_64-c4-container-intel/20261113T113057Z-d11e290-dirty/report.md)

## single-trunk-convergence/fold_light_w8_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113312Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n65536/linux-x86_64-c4-container-intel/20261113T113312Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w1_n1024

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083547Z-e486b5f-dirty](single-trunk-convergence/short_w1_n1024/linux-x86_64-c2-container-intel/20261113T083547Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084629Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n1024/linux-x86_64-c4-container-intel/20261113T084629Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w1_n16

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T082904Z-e486b5f-dirty](single-trunk-convergence/short_w1_n16/linux-x86_64-c2-container-intel/20261113T082904Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T083944Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n16/linux-x86_64-c4-container-intel/20261113T083944Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w1_n256

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083333Z-e486b5f-dirty](single-trunk-convergence/short_w1_n256/linux-x86_64-c2-container-intel/20261113T083333Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084415Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n256/linux-x86_64-c4-container-intel/20261113T084415Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w1_n64

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083120Z-e486b5f-dirty](single-trunk-convergence/short_w1_n64/linux-x86_64-c2-container-intel/20261113T083120Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084201Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n64/linux-x86_64-c4-container-intel/20261113T084201Z-ed2bf53-dirty/report.md)

## single-trunk-convergence/short_w2_n1024

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092935Z-74174bb](single-trunk-convergence/short_w2_n1024/linux-x86_64-c4-container-intel/20261113T092935Z-74174bb/report.md)

## single-trunk-convergence/short_w2_n16

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092249Z-74174bb](single-trunk-convergence/short_w2_n16/linux-x86_64-c4-container-intel/20261113T092249Z-74174bb/report.md)

## single-trunk-convergence/short_w2_n256

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092721Z-74174bb](single-trunk-convergence/short_w2_n256/linux-x86_64-c4-container-intel/20261113T092721Z-74174bb/report.md)

## single-trunk-convergence/short_w2_n64

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092506Z-74174bb](single-trunk-convergence/short_w2_n64/linux-x86_64-c4-container-intel/20261113T092506Z-74174bb/report.md)

## single-trunk-convergence/short_w4_n1024

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104033Z-f12bdc4](single-trunk-convergence/short_w4_n1024/linux-x86_64-c4-container-intel/20261113T104033Z-f12bdc4/report.md)

## single-trunk-convergence/short_w4_n16

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103346Z-f12bdc4](single-trunk-convergence/short_w4_n16/linux-x86_64-c4-container-intel/20261113T103346Z-f12bdc4/report.md)

## single-trunk-convergence/short_w4_n256

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103818Z-f12bdc4](single-trunk-convergence/short_w4_n256/linux-x86_64-c4-container-intel/20261113T103818Z-f12bdc4/report.md)

## single-trunk-convergence/short_w4_n64

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103604Z-f12bdc4](single-trunk-convergence/short_w4_n64/linux-x86_64-c4-container-intel/20261113T103604Z-f12bdc4/report.md)

## single-trunk-convergence/short_w8_n1024

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112843Z-d11e290-dirty](single-trunk-convergence/short_w8_n1024/linux-x86_64-c4-container-intel/20261113T112843Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w8_n16

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112154Z-d11e290-dirty](single-trunk-convergence/short_w8_n16/linux-x86_64-c4-container-intel/20261113T112154Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w8_n256

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112627Z-d11e290-dirty](single-trunk-convergence/short_w8_n256/linux-x86_64-c4-container-intel/20261113T112627Z-d11e290-dirty/report.md)

## single-trunk-convergence/short_w8_n64

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112412Z-d11e290-dirty](single-trunk-convergence/short_w8_n64/linux-x86_64-c4-container-intel/20261113T112412Z-d11e290-dirty/report.md)

## swap_band/swap_band_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T095229Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 9.90 us | 1.00x |
| sb_plan | 12.61 us | 1.27x |
| sb_value | 235.25 us | 23.77x |

- [20261115T095229Z-266650fb-dirty](swap_band/swap_band_n1024/linux-x86_64-c16-bare-intel/20261115T095229Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T095023Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 7.90 us | 1.00x |
| sb_plan | 10.11 us | 1.28x |
| sb_value | 16.20 us | 2.05x |

- [20261115T095023Z-266650fb-dirty](swap_band/swap_band_n64/linux-x86_64-c16-bare-intel/20261115T095023Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T095526Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 11.02 us | 1.00x |
| sb_plan | 13.92 us | 1.26x |
| sb_value | 5.17 ms | 468.55x |

- [20261115T095526Z-266650fb-dirty](swap_band/swap_band_n8192/linux-x86_64-c16-bare-intel/20261115T095526Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T100307Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 707.51 us | 1.57x |
| cs_plain | 451.50 us | 1.00x |
| cs_snap | 952.53 us | 2.11x |
| cs_snap_twin | 1.02 ms | 2.26x |

- [20261115T100307Z-266650fb-dirty](swap_column/swap_column_n1024/linux-x86_64-c16-bare-intel/20261115T100307Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T100024Z-266650fb`

| variant | median | vs best |
|---|---|---|
| cs_plain | 16.47 us | 1.00x |
| cs_snap | 21.77 us | 1.32x |
| cs_snap_twin | 22.61 us | 1.37x |

- [20261115T100020Z-266650fb-dirty](swap_column/swap_column_n64/linux-x86_64-c16-bare-intel/20261115T100020Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T100719Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| cs_exch | 7.15 ms | 1.14x |
| cs_plain | 6.29 ms | 1.00x |
| cs_snap | 9.09 ms | 1.45x |
| cs_snap_twin | 9.11 ms | 1.45x |

- [20261115T100719Z-266650fb-dirty](swap_column/swap_column_n8192/linux-x86_64-c16-bare-intel/20261115T100719Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T101140Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sc_v1k | 16 ns | 1.62x |
| sc_v64b | 10 ns | 1.00x |
| sc_v64k | 2.16 us | 213.46x |

- [20261115T101140Z-266650fb-dirty](swap_cost/swap_cost_n64/linux-x86_64-c16-bare-intel/20261115T101140Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261114T233754Z-266650fb`

| variant | median | vs best |
|---|---|---|
| sc_v64k | 1.71 us | 1.00x |

- [20260719T185012Z-8c3a949d-dirty](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20260719T185012Z-8c3a949d-dirty/report.md)
- [20261114T233753Z-266650fb-dirty](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20261114T233753Z-266650fb-dirty/report.md)

## wake_policy/wake_policy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T102359Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 27.39 us | 1.25x |
| wp_lp0 | 30.51 us | 1.39x |
| wp_nh128 | 25.51 us | 1.16x |
| wp_nh2k | 41.44 us | 1.89x |
| wp_nh8k | 24.03 us | 1.10x |
| wp_park | 24.76 us | 1.13x |
| wp_spin | 21.94 us | 1.00x |

- [20261115T102359Z-266650fb-dirty](wake_policy/wake_policy_n1024/linux-x86_64-c16-bare-intel/20261115T102359Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T101936Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 30.45 us | 2.10x |
| wp_lp0 | 23.89 us | 1.64x |
| wp_nh128 | 19.72 us | 1.36x |
| wp_nh2k | 24.34 us | 1.67x |
| wp_nh8k | 17.19 us | 1.18x |
| wp_park | 16.44 us | 1.13x |
| wp_spin | 14.53 us | 1.00x |

- [20261115T101936Z-266650fb-dirty](wake_policy/wake_policy_n64/linux-x86_64-c16-bare-intel/20261115T101936Z-266650fb-dirty/report.md)

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

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T102820Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 59.65 us | 1.05x |
| wp_lp0 | 70.14 us | 1.24x |
| wp_nh128 | 67.90 us | 1.20x |
| wp_nh2k | 78.82 us | 1.39x |
| wp_nh8k | 70.17 us | 1.24x |
| wp_park | 65.03 us | 1.15x |
| wp_spin | 56.73 us | 1.00x |

- [20261115T102820Z-266650fb-dirty](wake_policy/wake_policy_n8192/linux-x86_64-c16-bare-intel/20261115T102820Z-266650fb-dirty/report.md)

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
