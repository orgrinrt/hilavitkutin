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

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195330Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 2.40 us | 1.00x |
| inc_fetch_add | 2.09 us | 1.00x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015102Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T015102Z-88d91ab/report.md)
- [20261114T205811Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T205811Z-266650fb-dirty/report.md)
- [20261116T195330Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T195330Z-07a9e9a3/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T021047Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 146.50 us | 13952.07x |
| inc_fetch_add | 82.17 us | 7825.81x |
| inc_load_store | 10 ns | 1.00x |

- [20261115T021047Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T021047Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T195330Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 38.10 us | 95255.75x |
| inc_fetch_add | 33.58 us | 83959.50x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015108Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T015108Z-88d91ab/report.md)
- [20261114T205819Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T205819Z-266650fb-dirty/report.md)
- [20261116T195330Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T195330Z-07a9e9a3/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T020258Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 1.88 us | 221.12x |
| inc_fetch_add | 1.06 us | 125.16x |
| inc_load_store | 8 ns | 1.00x |

- [20261115T020258Z-266650fb](atomic_inc_strategy/atomic_inc_strategy_n256/linux-x86_64-c16-bare-intel/20261115T020258Z-266650fb/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195330Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 608 ns | 1519.75x |
| inc_fetch_add | 493 ns | 1233.25x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015101Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20260511T015101Z-88d91ab/report.md)
- [20261114T205809Z-266650fb](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20261114T205809Z-266650fb/report.md)
- [20261116T195330Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20261116T195330Z-07a9e9a3/report.md)

## atomic_inc_strategy/atomic_inc_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T020807Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 29.96 us | 2880.84x |
| inc_fetch_add | 16.82 us | 1617.45x |
| inc_load_store | 10 ns | 1.00x |

- [20261115T020807Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T020807Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195330Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 9.60 us | 23990.50x |
| inc_fetch_add | 8.37 us | 20925.00x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015104Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T015104Z-88d91ab/report.md)
- [20261114T205814Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T205814Z-266650fb-dirty/report.md)
- [20261116T195330Z-07a9e9a3](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T195330Z-07a9e9a3/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T195414Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 826 ns | 3.60x |
| atomic_relaxed | 230 ns | 1.00x |
| atomic_seqcst | 830 ns | 3.62x |

- [20260510T214037Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n1024/macos-aarch64-c8-bare-apple/20260510T214037Z-f6ee560/report.md)
- [20261116T195414Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n1024/macos-aarch64-c8-bare-apple/20261116T195414Z-07a9e9a3/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T195414Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 14.02 us | 3.29x |
| atomic_relaxed | 4.27 us | 1.00x |
| atomic_seqcst | 14.03 us | 3.29x |

- [20260510T214043Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n16384/macos-aarch64-c8-bare-apple/20260510T214043Z-f6ee560/report.md)
- [20261116T195414Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n16384/macos-aarch64-c8-bare-apple/20261116T195414Z-07a9e9a3/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T195414Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 162 ns | 4.73x |
| atomic_relaxed | 34 ns | 1.00x |
| atomic_seqcst | 167 ns | 4.87x |

- [20260510T214035Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n256/macos-aarch64-c8-bare-apple/20260510T214035Z-f6ee560/report.md)
- [20261116T195414Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n256/macos-aarch64-c8-bare-apple/20261116T195414Z-07a9e9a3/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n4096

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T195414Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 3.46 us | 3.35x |
| atomic_relaxed | 1.03 us | 1.00x |
| atomic_seqcst | 3.47 us | 3.36x |

- [20260510T214039Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n4096/macos-aarch64-c8-bare-apple/20260510T214039Z-f6ee560/report.md)
- [20261116T195414Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n4096/macos-aarch64-c8-bare-apple/20261116T195414Z-07a9e9a3/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n64

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T195414Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 7 ns | 1.86x |
| atomic_relaxed | 4 ns | 1.00x |
| atomic_seqcst | 4 ns | 1.00x |

- [20260510T214034Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n64/macos-aarch64-c8-bare-apple/20260510T214034Z-f6ee560/report.md)
- [20261116T195414Z-07a9e9a3](atomic_ordering_cost/atomic_ordering_cost_n64/macos-aarch64-c8-bare-apple/20261116T195414Z-07a9e9a3/report.md)

## atomic_vs_plain/atomic_vs_plain_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T021656Z-266650fb`

| variant | median | vs best |
|---|---|---|
| acc_plain | 1.15 us | 1.00x |

- [20261115T021655Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/linux-x86_64-c16-bare-intel/20261115T021655Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 1.23 us | 1.00x |
| acc_atomic_seqcst | 1.23 us | 1.00x |
| acc_plain | 1.23 us | 1.00x |

- [20260511T013000Z-a410443](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20260511T013000Z-a410443/report.md)
- [20261114T205932Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20261114T205932Z-266650fb-dirty/report.md)
- [20261116T195458Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20261116T195458Z-07a9e9a3/report.md)

## atomic_vs_plain/atomic_vs_plain_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T022201Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 18.27 us | 1.00x |
| acc_atomic_seqcst | 18.27 us | 1.00x |
| acc_plain | 18.27 us | 1.00x |

- [20261115T022201Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n16384/linux-x86_64-c16-bare-intel/20261115T022201Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 20.44 us | 1.00x |
| acc_atomic_seqcst | 20.42 us | 1.00x |
| acc_plain | 20.41 us | 1.00x |

- [20260511T013005Z-a410443](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20260511T013005Z-a410443/report.md)
- [20261114T205940Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20261114T205940Z-266650fb-dirty/report.md)
- [20261116T195458Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20261116T195458Z-07a9e9a3/report.md)

## atomic_vs_plain/atomic_vs_plain_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T021420Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 292 ns | 1.00x |
| acc_atomic_seqcst | 292 ns | 1.00x |
| acc_plain | 292 ns | 1.00x |

- [20261115T021420Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n256/linux-x86_64-c16-bare-intel/20261115T021420Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 269 ns | 1.00x |
| acc_atomic_seqcst | 269 ns | 1.00x |
| acc_plain | 270 ns | 1.00x |

- [20260511T012959Z-a410443](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20260511T012959Z-a410443/report.md)
- [20261114T205930Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20261114T205930Z-266650fb-dirty/report.md)
- [20261116T195458Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20261116T195458Z-07a9e9a3/report.md)

## atomic_vs_plain/atomic_vs_plain_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T021927Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 4.57 us | 1.00x |
| acc_atomic_seqcst | 4.57 us | 1.00x |
| acc_plain | 4.57 us | 1.00x |

- [20261115T021927Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n4096/linux-x86_64-c16-bare-intel/20261115T021927Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 5.07 us | 1.00x |
| acc_atomic_seqcst | 5.06 us | 1.00x |
| acc_plain | 5.06 us | 1.00x |

- [20260511T013001Z-a410443](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20260511T013001Z-a410443/report.md)
- [20261114T205935Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20261114T205935Z-266650fb-dirty/report.md)
- [20261116T195458Z-07a9e9a3](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20261116T195458Z-07a9e9a3/report.md)

## bit_count_ops/bit_count_ops_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T022738Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 52 ns | 1.00x |
| ctz_intrinsic | 52 ns | 1.00x |
| ctz_loop | 124 ns | 2.39x |

- [20261115T022738Z-266650fb-dirty](bit_count_ops/bit_count_ops_n1024/linux-x86_64-c16-bare-intel/20261115T022738Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195542Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 59 ns | 1.00x |
| ctz_intrinsic | 60 ns | 1.01x |
| ctz_loop | 148 ns | 2.51x |

- [20260511T021038Z-315383f](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20260511T021038Z-315383f/report.md)
- [20261114T210018Z-266650fb-dirty](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20261114T210018Z-266650fb-dirty/report.md)
- [20261116T195542Z-07a9e9a3](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20261116T195542Z-07a9e9a3/report.md)

## bit_count_ops/bit_count_ops_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T023242Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 621 ns | 1.00x |
| ctz_intrinsic | 620 ns | 1.00x |
| ctz_loop | 2.47 us | 3.99x |

- [20261115T023242Z-266650fb-dirty](bit_count_ops/bit_count_ops_n16384/linux-x86_64-c16-bare-intel/20261115T023242Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195542Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 498 ns | 1.01x |
| ctz_intrinsic | 493 ns | 1.00x |
| ctz_loop | 2.00 us | 4.06x |

- [20260511T021040Z-315383f](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20260511T021040Z-315383f/report.md)
- [20261114T210022Z-266650fb-dirty](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20261114T210022Z-266650fb-dirty/report.md)
- [20261116T195542Z-07a9e9a3](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20261116T195542Z-07a9e9a3/report.md)

## bit_count_ops/bit_count_ops_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T022503Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 20 ns | 1.21x |
| ctz_intrinsic | 16 ns | 1.00x |
| ctz_loop | 38 ns | 2.28x |

- [20261115T022503Z-266650fb-dirty](bit_count_ops/bit_count_ops_n256/linux-x86_64-c16-bare-intel/20261115T022503Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195542Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 23 ns | 1.02x |
| ctz_intrinsic | 23 ns | 1.00x |
| ctz_loop | 51 ns | 2.22x |

- [20260511T021037Z-315383f](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20260511T021037Z-315383f/report.md)
- [20261114T210015Z-266650fb-dirty](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20261114T210015Z-266650fb-dirty/report.md)
- [20261116T195542Z-07a9e9a3](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20261116T195542Z-07a9e9a3/report.md)

## bit_count_ops/bit_count_ops_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T023010Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 180 ns | 1.00x |
| ctz_intrinsic | 180 ns | 1.00x |
| ctz_loop | 671 ns | 3.73x |

- [20261115T023010Z-266650fb-dirty](bit_count_ops/bit_count_ops_n4096/linux-x86_64-c16-bare-intel/20261115T023010Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195542Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 152 ns | 1.01x |
| ctz_intrinsic | 151 ns | 1.00x |
| ctz_loop | 522 ns | 3.46x |

- [20260511T021039Z-315383f](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20260511T021039Z-315383f/report.md)
- [20261114T210020Z-266650fb-dirty](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20261114T210020Z-266650fb-dirty/report.md)
- [20261116T195542Z-07a9e9a3](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20261116T195542Z-07a9e9a3/report.md)

## bounds_check/bounds_check_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T023819Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 152 ns | 1.00x |
| bc_iter | 152 ns | 1.00x |
| bc_unchecked | 153 ns | 1.00x |

- [20261115T023819Z-266650fb-dirty](bounds_check/bounds_check_n1024/linux-x86_64-c16-bare-intel/20261115T023819Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195622Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 36 ns | 1.00x |
| bc_iter | 37 ns | 1.03x |
| bc_unchecked | 38 ns | 1.04x |

- [20260511T005008Z-f10ba65](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20260511T005008Z-f10ba65/report.md)
- [20261114T210100Z-266650fb-dirty](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20261114T210100Z-266650fb-dirty/report.md)
- [20261116T195622Z-07a9e9a3](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20261116T195622Z-07a9e9a3/report.md)

## bounds_check/bounds_check_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024323Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 2.29 us | 1.00x |
| bc_iter | 2.29 us | 1.00x |
| bc_unchecked | 2.29 us | 1.00x |

- [20261115T024323Z-266650fb-dirty](bounds_check/bounds_check_n16384/linux-x86_64-c16-bare-intel/20261115T024323Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195622Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 352 ns | 1.01x |
| bc_iter | 350 ns | 1.00x |
| bc_unchecked | 623 ns | 1.78x |

- [20260511T005013Z-f10ba65](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20260511T005013Z-f10ba65/report.md)
- [20261114T210104Z-266650fb-dirty](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20261114T210104Z-266650fb-dirty/report.md)
- [20261116T195622Z-07a9e9a3](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20261116T195622Z-07a9e9a3/report.md)

## bounds_check/bounds_check_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T023548Z-266650fb`

| variant | median | vs best |
|---|---|---|
| bc_unchecked | 37 ns | 1.00x |

- [20261115T023544Z-266650fb-dirty](bounds_check/bounds_check_n256/linux-x86_64-c16-bare-intel/20261115T023544Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195622Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 5 ns | 1.00x |
| bc_iter | 5 ns | 1.09x |
| bc_unchecked | 5 ns | 1.09x |

- [20260511T005007Z-f10ba65](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20260511T005007Z-f10ba65/report.md)
- [20261114T210058Z-266650fb-dirty](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20261114T210058Z-266650fb-dirty/report.md)
- [20261116T195622Z-07a9e9a3](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20261116T195622Z-07a9e9a3/report.md)

## bounds_check/bounds_check_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024051Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bc_index | 580 ns | 1.00x |
| bc_iter | 580 ns | 1.00x |
| bc_unchecked | 580 ns | 1.00x |

- [20261115T024051Z-266650fb-dirty](bounds_check/bounds_check_n4096/linux-x86_64-c16-bare-intel/20261115T024051Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195622Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bc_index | 92 ns | 1.00x |
| bc_iter | 93 ns | 1.01x |
| bc_unchecked | 142 ns | 1.53x |

- [20260511T005009Z-f10ba65](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20260511T005009Z-f10ba65/report.md)
- [20261114T210102Z-266650fb-dirty](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20261114T210102Z-266650fb-dirty/report.md)
- [20261116T195622Z-07a9e9a3](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20261116T195622Z-07a9e9a3/report.md)

## branch_pattern/branch_pattern_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024902Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.71 us | 1.99x |
| branch_if | 861 ns | 1.00x |

- [20261115T024902Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/linux-x86_64-c16-bare-intel/20261115T024902Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195700Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.61 us | 1.00x |
| branch_if | 1.61 us | 1.00x |

- [20260510T213318Z-b042648](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20260510T213318Z-b042648/report.md)
- [20261114T210137Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20261114T210137Z-266650fb-dirty/report.md)
- [20261116T195700Z-07a9e9a3](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20261116T195700Z-07a9e9a3/report.md)

## branch_pattern/branch_pattern_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025232Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 27.44 us | 1.00x |
| branch_if | 29.93 us | 1.09x |

- [20261115T025232Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/linux-x86_64-c16-bare-intel/20261115T025232Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195700Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 26.07 us | 1.00x |
| branch_if | 26.07 us | 1.00x |

- [20260510T213324Z-b042648](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20260510T213324Z-b042648/report.md)
- [20261114T210143Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20261114T210143Z-266650fb-dirty/report.md)
- [20261116T195700Z-07a9e9a3](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20261116T195700Z-07a9e9a3/report.md)

## branch_pattern/branch_pattern_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024719Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 429 ns | 1.94x |
| branch_if | 221 ns | 1.00x |

- [20261115T024719Z-266650fb-dirty](branch_pattern/branch_pattern_n256/linux-x86_64-c16-bare-intel/20261115T024719Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195700Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 390 ns | 1.00x |
| branch_if | 392 ns | 1.00x |

- [20260510T213317Z-b042648](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20260510T213317Z-b042648/report.md)
- [20261114T210136Z-266650fb-dirty](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20261114T210136Z-266650fb-dirty/report.md)
- [20261116T195700Z-07a9e9a3](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20261116T195700Z-07a9e9a3/report.md)

## branch_pattern/branch_pattern_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025046Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.29 us | 2.00x |
| branch_if | 3.15 us | 1.00x |

- [20261115T025046Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/linux-x86_64-c16-bare-intel/20261115T025046Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195700Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.51 us | 1.00x |
| branch_if | 6.50 us | 1.00x |

- [20260510T213320Z-b042648](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20260510T213320Z-b042648/report.md)
- [20261114T210139Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20261114T210139Z-266650fb-dirty/report.md)
- [20261116T195700Z-07a9e9a3](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20261116T195700Z-07a9e9a3/report.md)

## branch_pattern/branch_pattern_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024533Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 112 ns | 1.79x |
| branch_if | 63 ns | 1.00x |

- [20261115T024533Z-266650fb-dirty](branch_pattern/branch_pattern_n64/linux-x86_64-c16-bare-intel/20261115T024533Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195700Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 83 ns | 1.00x |
| branch_if | 82 ns | 1.00x |

- [20260510T213316Z-b042648](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20260510T213316Z-b042648/report.md)
- [20261114T210134Z-266650fb-dirty](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20261114T210134Z-266650fb-dirty/report.md)
- [20261116T195700Z-07a9e9a3](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20261116T195700Z-07a9e9a3/report.md)

## branch_predictability/branch_predictability_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025809Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 1.15 us | 1.00x |
| bp_skew_high | 1.15 us | 1.00x |
| bp_skew_low | 1.15 us | 1.00x |

- [20261115T025809Z-266650fb-dirty](branch_predictability/branch_predictability_n1024/linux-x86_64-c16-bare-intel/20261115T025809Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195740Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 1.26 us | 1.00x |
| bp_skew_high | 1.25 us | 1.00x |
| bp_skew_low | 1.26 us | 1.00x |

- [20260511T011011Z-c61ad63](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20260511T011011Z-c61ad63/report.md)
- [20261114T210220Z-266650fb-dirty](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20261114T210220Z-266650fb-dirty/report.md)
- [20261116T195740Z-07a9e9a3](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20261116T195740Z-07a9e9a3/report.md)

## branch_predictability/branch_predictability_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030315Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 18.27 us | 1.00x |
| bp_skew_high | 18.27 us | 1.00x |
| bp_skew_low | 18.27 us | 1.00x |

- [20261115T030315Z-266650fb-dirty](branch_predictability/branch_predictability_n16384/linux-x86_64-c16-bare-intel/20261115T030315Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195740Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 20.51 us | 1.00x |
| bp_skew_high | 20.51 us | 1.00x |
| bp_skew_low | 20.51 us | 1.00x |

- [20260511T011016Z-c61ad63](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20260511T011016Z-c61ad63/report.md)
- [20261114T210227Z-266650fb-dirty](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20261114T210227Z-266650fb-dirty/report.md)
- [20261116T195740Z-07a9e9a3](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20261116T195740Z-07a9e9a3/report.md)

## branch_predictability/branch_predictability_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025534Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 292 ns | 1.00x |
| bp_skew_high | 292 ns | 1.00x |
| bp_skew_low | 292 ns | 1.00x |

- [20261115T025534Z-266650fb-dirty](branch_predictability/branch_predictability_n256/linux-x86_64-c16-bare-intel/20261115T025534Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195740Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 294 ns | 1.00x |
| bp_skew_high | 294 ns | 1.00x |
| bp_skew_low | 294 ns | 1.00x |

- [20260511T011010Z-c61ad63](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20260511T011010Z-c61ad63/report.md)
- [20261114T210218Z-266650fb-dirty](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20261114T210218Z-266650fb-dirty/report.md)
- [20261116T195740Z-07a9e9a3](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20261116T195740Z-07a9e9a3/report.md)

## branch_predictability/branch_predictability_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030041Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 4.57 us | 1.22x |
| bp_skew_high | 3.75 us | 1.00x |
| bp_skew_low | 4.57 us | 1.22x |

- [20261115T030041Z-266650fb-dirty](branch_predictability/branch_predictability_n4096/linux-x86_64-c16-bare-intel/20261115T030041Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195740Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 5.10 us | 1.00x |
| bp_skew_high | 5.11 us | 1.00x |
| bp_skew_low | 5.11 us | 1.00x |

- [20260511T011012Z-c61ad63](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20260511T011012Z-c61ad63/report.md)
- [20261114T210223Z-266650fb-dirty](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20261114T210223Z-266650fb-dirty/report.md)
- [20261116T195740Z-07a9e9a3](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20261116T195740Z-07a9e9a3/report.md)

## build-static/clean_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 16864.64 ms | 1.08x |
| llvm_non_incremental | 15672.26 ms | 1.00x |

- [20261116T063249Z-f04daa0a-dirty](build-static/clean_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 14791.32 ms | 1.05x |
| llvm_non_incremental | 14119.47 ms | 1.00x |

- [20261117T020249Z-07a9e9a3](build-static/clean_n25/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/clean_n97

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 106876.51 ms | 1.05x |
| llvm_non_incremental | 101996.65 ms | 1.00x |

- [20261117T020249Z-07a9e9a3](build-static/clean_n97/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/leaf_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 3379.88 ms | 1.00x |
| llvm_non_incremental | 5451.34 ms | 1.61x |

- [20261116T063249Z-f04daa0a-dirty](build-static/leaf_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 2682.51 ms | 1.00x |
| llvm_non_incremental | 4043.19 ms | 1.51x |

- [20261117T020249Z-07a9e9a3](build-static/leaf_n25/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/leaf_n97

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 3244.23 ms | 1.00x |
| llvm_non_incremental | 4663.18 ms | 1.44x |

- [20261117T020249Z-07a9e9a3](build-static/leaf_n97/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/mid_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 693.37 ms | 1.00x |
| llvm_non_incremental | 5582.15 ms | 8.05x |

- [20261116T063249Z-f04daa0a-dirty](build-static/mid_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 581.68 ms | 1.00x |
| llvm_non_incremental | 4991.35 ms | 8.58x |

- [20261117T020249Z-07a9e9a3](build-static/mid_n25/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/mid_n97

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 2908.28 ms | 1.00x |
| llvm_non_incremental | 25917.33 ms | 8.91x |

- [20261117T020249Z-07a9e9a3](build-static/mid_n97/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/root_n25

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T063249Z-f04daa0a-dirty`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 1119.72 ms | 1.00x |
| llvm_non_incremental | 14310.33 ms | 12.78x |

- [20261116T063249Z-f04daa0a-dirty](build-static/root_n25/linux-x86_64-c4-container-intel/20261116T063249Z-f04daa0a-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 988.71 ms | 1.00x |
| llvm_non_incremental | 12129.91 ms | 12.27x |

- [20261117T020249Z-07a9e9a3](build-static/root_n25/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## build-static/root_n97

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T020249Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| llvm_incremental | 7421.37 ms | 1.00x |
| llvm_non_incremental | 78005.12 ms | 10.51x |

- [20261117T020249Z-07a9e9a3](build-static/root_n97/macos-aarch64-c8-bare-apple/20261117T020249Z-07a9e9a3/report.md)

## cache_layout/cache_layout_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030857Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 28 ns | 1.00x |
| layout_soa | 28 ns | 1.00x |

- [20261115T030857Z-266650fb-dirty](cache_layout/cache_layout_n1024/linux-x86_64-c16-bare-intel/20261115T030857Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195821Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 12 ns | 1.00x |
| layout_soa | 13 ns | 1.06x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210300Z-266650fb-dirty](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20261114T210300Z-266650fb-dirty/report.md)
- [20261116T195821Z-07a9e9a3](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20261116T195821Z-07a9e9a3/report.md)

## cache_layout/cache_layout_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031223Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 333 ns | 1.00x |
| layout_soa | 333 ns | 1.00x |

- [20261115T031223Z-266650fb-dirty](cache_layout/cache_layout_n16384/linux-x86_64-c16-bare-intel/20261115T031223Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195821Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 336 ns | 1.00x |
| layout_soa | 340 ns | 1.01x |

- [20260510T214108Z-f6ee560](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20260510T214108Z-f6ee560/report.md)
- [20261114T210303Z-266650fb-dirty](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20261114T210303Z-266650fb-dirty/report.md)
- [20261116T195821Z-07a9e9a3](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20261116T195821Z-07a9e9a3/report.md)

## cache_layout/cache_layout_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030714Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 12 ns | 1.01x |
| layout_soa | 12 ns | 1.00x |

- [20261115T030714Z-266650fb-dirty](cache_layout/cache_layout_n256/linux-x86_64-c16-bare-intel/20261115T030714Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195821Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 1 ns | 1.00x |
| layout_soa | 1 ns | 1.00x |

- [20260510T214105Z-f6ee560](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20260510T214105Z-f6ee560/report.md)
- [20261114T210259Z-266650fb-dirty](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20261114T210259Z-266650fb-dirty/report.md)
- [20261116T195821Z-07a9e9a3](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20261116T195821Z-07a9e9a3/report.md)

## cache_layout/cache_layout_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031040Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 90 ns | 1.01x |
| layout_soa | 89 ns | 1.00x |

- [20261115T031040Z-266650fb-dirty](cache_layout/cache_layout_n4096/linux-x86_64-c16-bare-intel/20261115T031040Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195821Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 62 ns | 1.00x |
| layout_soa | 64 ns | 1.03x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210301Z-266650fb-dirty](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20261114T210301Z-266650fb-dirty/report.md)
- [20261116T195821Z-07a9e9a3](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20261116T195821Z-07a9e9a3/report.md)

## cache_layout/cache_layout_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030527Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 10 ns | 1.00x |
| layout_soa | 10 ns | 1.00x |

- [20261115T030527Z-266650fb-dirty](cache_layout/cache_layout_n64/linux-x86_64-c16-bare-intel/20261115T030527Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195821Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| layout_aos | 0 ns | 1.00x |
| layout_soa | 0 ns | 1.00x |

- [20260510T214104Z-f6ee560](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20260510T214104Z-f6ee560/report.md)
- [20261114T210257Z-266650fb-dirty](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20261114T210257Z-266650fb-dirty/report.md)
- [20261116T195821Z-07a9e9a3](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20261116T195821Z-07a9e9a3/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n128

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032537Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 746 ns | 1.02x |
| dep_graph_dense | 23.42 us | 31.93x |
| dep_graph_hybrid | 734 ns | 1.00x |

- [20261115T032537Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n128/linux-x86_64-c16-bare-intel/20261115T032537Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195859Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 779 ns | 1.03x |
| dep_graph_dense | 26.46 us | 35.08x |
| dep_graph_hybrid | 754 ns | 1.00x |

- [20260510T212550Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20260510T212550Z-771392d/report.md)
- [20261114T210349Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20261114T210349Z-266650fb-dirty/report.md)
- [20261116T195859Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20261116T195859Z-07a9e9a3/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031800Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 89 ns | 1.00x |
| dep_graph_dense | 439 ns | 4.93x |
| dep_graph_hybrid | 447 ns | 5.02x |

- [20261115T031800Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n16/linux-x86_64-c16-bare-intel/20261115T031800Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195859Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 51 ns | 1.00x |
| dep_graph_dense | 857 ns | 16.74x |
| dep_graph_hybrid | 432 ns | 8.44x |

- [20260510T212544Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20260510T212544Z-771392d/report.md)
- [20261114T210341Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20261114T210341Z-266650fb-dirty/report.md)
- [20261116T195859Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20261116T195859Z-07a9e9a3/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032812Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 1.49 us | 1.02x |
| dep_graph_dense | 92.67 us | 63.39x |
| dep_graph_hybrid | 1.46 us | 1.00x |

- [20261115T032812Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n256/linux-x86_64-c16-bare-intel/20261115T032812Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195859Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 1.62 us | 1.03x |
| dep_graph_dense | 103.92 us | 66.03x |
| dep_graph_hybrid | 1.57 us | 1.00x |

- [20260510T212557Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20260510T212557Z-771392d/report.md)
- [20261114T210355Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20261114T210355Z-266650fb-dirty/report.md)
- [20261116T195859Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20261116T195859Z-07a9e9a3/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n32

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032032Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 193 ns | 1.00x |
| dep_graph_dense | 1.60 us | 8.29x |
| dep_graph_hybrid | 208 ns | 1.08x |

- [20261115T032032Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n32/linux-x86_64-c16-bare-intel/20261115T032032Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195859Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 150 ns | 1.00x |
| dep_graph_dense | 2.12 us | 14.19x |
| dep_graph_hybrid | 154 ns | 1.03x |

- [20260510T212545Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20260510T212545Z-771392d/report.md)
- [20261114T210343Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20261114T210343Z-266650fb-dirty/report.md)
- [20261116T195859Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20261116T195859Z-07a9e9a3/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T032304Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 377 ns | 1.00x |
| dep_graph_dense | 6.02 us | 15.96x |
| dep_graph_hybrid | 404 ns | 1.07x |

- [20261115T032304Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n64/linux-x86_64-c16-bare-intel/20261115T032304Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195859Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 356 ns | 1.00x |
| dep_graph_dense | 7.02 us | 19.69x |
| dep_graph_hybrid | 369 ns | 1.04x |

- [20260510T212547Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20260510T212547Z-771392d/report.md)
- [20261114T210346Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20261114T210346Z-266650fb-dirty/report.md)
- [20261116T195859Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20261116T195859Z-07a9e9a3/report.md)

## dep_graph_csr_9/dep_graph_csr_9_n8

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031525Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 43 ns | 1.00x |
| dep_graph_dense | 124 ns | 2.88x |
| dep_graph_hybrid | 125 ns | 2.89x |

- [20261115T031525Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n8/linux-x86_64-c16-bare-intel/20261115T031525Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195859Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 25 ns | 1.00x |
| dep_graph_dense | 556 ns | 22.59x |
| dep_graph_hybrid | 101 ns | 4.09x |

- [20260510T212543Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20260510T212543Z-771392d/report.md)
- [20261114T210339Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20261114T210339Z-266650fb-dirty/report.md)
- [20261116T195859Z-07a9e9a3](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20261116T195859Z-07a9e9a3/report.md)

## dispatch_dynamic/dispatch_dynamic_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033621Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 150 ns | 1.00x |
| dispatch_runtime | 193 ns | 1.29x |
| dispatch_volatile | 182 ns | 1.22x |

- [20261115T033621Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n1024/linux-x86_64-c16-bare-intel/20261115T033621Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 111 ns | 1.00x |
| dispatch_runtime | 288 ns | 2.59x |
| dispatch_volatile | 160 ns | 1.44x |

- [20260510T204023Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20260510T204023Z-3c0d0d5/report.md)
- [20261114T210435Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20261114T210435Z-266650fb-dirty/report.md)
- [20261116T195952Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20261116T195952Z-07a9e9a3/report.md)

## dispatch_dynamic/dispatch_dynamic_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034125Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2.29 us | 1.00x |
| dispatch_runtime | 11.44 us | 5.00x |
| dispatch_volatile | 2.85 us | 1.25x |

- [20261115T034125Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n16384/linux-x86_64-c16-bare-intel/20261115T034125Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2.51 us | 1.00x |
| dispatch_runtime | 10.33 us | 4.12x |
| dispatch_volatile | 2.56 us | 1.02x |

- [20260510T204026Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20260510T204026Z-3c0d0d5/report.md)
- [20261114T210440Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20261114T210440Z-266650fb-dirty/report.md)
- [20261116T195952Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20261116T195952Z-07a9e9a3/report.md)

## dispatch_dynamic/dispatch_dynamic_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033349Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 43 ns | 1.05x |
| dispatch_runtime | 41 ns | 1.00x |
| dispatch_volatile | 46 ns | 1.13x |

- [20261115T033349Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n256/linux-x86_64-c16-bare-intel/20261115T033349Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 15 ns | 1.00x |
| dispatch_runtime | 43 ns | 2.97x |
| dispatch_volatile | 41 ns | 2.82x |

- [20260510T204022Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20260510T204022Z-3c0d0d5/report.md)
- [20261114T210433Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20261114T210433Z-266650fb-dirty/report.md)
- [20261116T195952Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20261116T195952Z-07a9e9a3/report.md)

## dispatch_dynamic/dispatch_dynamic_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033853Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 578 ns | 1.00x |
| dispatch_runtime | 741 ns | 1.28x |
| dispatch_volatile | 718 ns | 1.24x |

- [20261115T033853Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n4096/linux-x86_64-c16-bare-intel/20261115T033853Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 592 ns | 1.00x |
| dispatch_runtime | 1.10 us | 1.86x |
| dispatch_volatile | 645 ns | 1.09x |

- [20260510T204024Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20260510T204024Z-3c0d0d5/report.md)
- [20261114T210437Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20261114T210437Z-266650fb-dirty/report.md)
- [20261116T195952Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20261116T195952Z-07a9e9a3/report.md)

## dispatch_dynamic/dispatch_dynamic_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T033114Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 16 ns | 1.02x |
| dispatch_runtime | 16 ns | 1.00x |
| dispatch_volatile | 16 ns | 1.03x |

- [20261115T033114Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n64/linux-x86_64-c16-bare-intel/20261115T033114Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T195952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2 ns | 1.00x |
| dispatch_runtime | 9 ns | 5.06x |
| dispatch_volatile | 7 ns | 3.94x |

- [20260510T204021Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20260510T204021Z-3c0d0d5/report.md)
- [20261114T210430Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20261114T210430Z-266650fb-dirty/report.md)
- [20261116T195952Z-07a9e9a3](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20261116T195952Z-07a9e9a3/report.md)

## dispatch_static/dispatch_static_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034930Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 150 ns | 1.00x |
| dispatch_direct | 150 ns | 1.00x |
| dispatch_trait | 150 ns | 1.00x |

- [20261115T034930Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/linux-x86_64-c16-bare-intel/20261115T034930Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T200038Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 111 ns | 1.01x |
| dispatch_direct | 111 ns | 1.01x |
| dispatch_trait | 110 ns | 1.00x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210515Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20261114T210515Z-266650fb-dirty/report.md)
- [20261116T200038Z-07a9e9a3](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20261116T200038Z-07a9e9a3/report.md)

## dispatch_static/dispatch_static_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T035434Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2.29 us | 1.00x |
| dispatch_direct | 2.29 us | 1.00x |
| dispatch_trait | 2.29 us | 1.00x |

- [20261115T035434Z-266650fb-dirty](dispatch_static/dispatch_static_n16384/linux-x86_64-c16-bare-intel/20261115T035434Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200038Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2.51 us | 1.00x |
| dispatch_direct | 2.51 us | 1.00x |
| dispatch_trait | 2.51 us | 1.00x |

- [20260510T204037Z-3c0d0d5](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20260510T204037Z-3c0d0d5/report.md)
- [20261114T210520Z-266650fb-dirty](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20261114T210520Z-266650fb-dirty/report.md)
- [20261116T200038Z-07a9e9a3](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20261116T200038Z-07a9e9a3/report.md)

## dispatch_static/dispatch_static_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034658Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 43 ns | 1.00x |
| dispatch_direct | 43 ns | 1.00x |
| dispatch_trait | 43 ns | 1.00x |

- [20261115T034658Z-266650fb-dirty](dispatch_static/dispatch_static_n256/linux-x86_64-c16-bare-intel/20261115T034658Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200038Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 15 ns | 1.00x |
| dispatch_direct | 15 ns | 1.00x |
| dispatch_trait | 15 ns | 1.00x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210513Z-266650fb-dirty](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20261114T210513Z-266650fb-dirty/report.md)
- [20261116T200038Z-07a9e9a3](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20261116T200038Z-07a9e9a3/report.md)

## dispatch_static/dispatch_static_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T035202Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 578 ns | 1.00x |
| dispatch_direct | 578 ns | 1.00x |
| dispatch_trait | 578 ns | 1.00x |

- [20261115T035202Z-266650fb-dirty](dispatch_static/dispatch_static_n4096/linux-x86_64-c16-bare-intel/20261115T035202Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200038Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 598 ns | 1.01x |
| dispatch_direct | 594 ns | 1.00x |
| dispatch_trait | 592 ns | 1.00x |

- [20260510T204035Z-3c0d0d5](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20260510T204035Z-3c0d0d5/report.md)
- [20261114T210517Z-266650fb-dirty](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20261114T210517Z-266650fb-dirty/report.md)
- [20261116T200038Z-07a9e9a3](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20261116T200038Z-07a9e9a3/report.md)

## dispatch_static/dispatch_static_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T034423Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 16 ns | 1.00x |
| dispatch_direct | 16 ns | 1.00x |
| dispatch_trait | 16 ns | 1.00x |

- [20261115T034423Z-266650fb-dirty](dispatch_static/dispatch_static_n64/linux-x86_64-c16-bare-intel/20261115T034423Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200038Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2 ns | 1.00x |
| dispatch_direct | 2 ns | 1.00x |
| dispatch_trait | 2 ns | 1.00x |

- [20260510T204033Z-3c0d0d5](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20260510T204033Z-3c0d0d5/report.md)
- [20261114T210511Z-266650fb-dirty](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20261114T210511Z-266650fb-dirty/report.md)
- [20261116T200038Z-07a9e9a3](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20261116T200038Z-07a9e9a3/report.md)

## ema_axis_h/ema_axis_h_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040242Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 196 ns | 1.66x |
| ema_neon | 197 ns | 1.67x |
| ema_scalar | 118 ns | 1.00x |

- [20261115T040242Z-266650fb-dirty](ema_axis_h/ema_axis_h_n1024/linux-x86_64-c16-bare-intel/20261115T040242Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200123Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 383 ns | 9.19x |
| ema_neon | 42 ns | 1.00x |
| ema_scalar | 85 ns | 2.05x |

- [20260510T204029Z-3c0d0d5](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20260510T204029Z-3c0d0d5/report.md)
- [20261114T210600Z-266650fb-dirty](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20261114T210600Z-266650fb-dirty/report.md)
- [20261116T200123Z-07a9e9a3](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20261116T200123Z-07a9e9a3/report.md)

## ema_axis_h/ema_axis_h_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040747Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 2.93 us | 1.66x |
| ema_neon | 2.94 us | 1.67x |
| ema_scalar | 1.76 us | 1.00x |

- [20261115T040747Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/linux-x86_64-c16-bare-intel/20261115T040747Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T200123Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 6.53 us | 4.37x |
| ema_neon | 1.49 us | 1.00x |
| ema_scalar | 1.58 us | 1.06x |

- [20260510T204031Z-3c0d0d5](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20260510T204031Z-3c0d0d5/report.md)
- [20261114T210605Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20261114T210605Z-266650fb-dirty/report.md)
- [20261116T200123Z-07a9e9a3](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20261116T200123Z-07a9e9a3/report.md)

## ema_axis_h/ema_axis_h_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040011Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 59 ns | 1.62x |
| ema_neon | 59 ns | 1.62x |
| ema_scalar | 36 ns | 1.00x |

- [20261115T040011Z-266650fb-dirty](ema_axis_h/ema_axis_h_n256/linux-x86_64-c16-bare-intel/20261115T040011Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200123Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 77 ns | 63.92x |
| ema_neon | 1 ns | 1.00x |
| ema_scalar | 24 ns | 19.75x |

- [20260510T204028Z-3c0d0d5](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20260510T204028Z-3c0d0d5/report.md)
- [20261114T210558Z-266650fb-dirty](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20261114T210558Z-266650fb-dirty/report.md)
- [20261116T200123Z-07a9e9a3](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20261116T200123Z-07a9e9a3/report.md)

## ema_axis_h/ema_axis_h_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T040514Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 742 ns | 1.66x |
| ema_neon | 744 ns | 1.67x |
| ema_scalar | 446 ns | 1.00x |

- [20261115T040514Z-266650fb-dirty](ema_axis_h/ema_axis_h_n4096/linux-x86_64-c16-bare-intel/20261115T040514Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200123Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 1.61 us | 5.50x |
| ema_neon | 293 ns | 1.00x |
| ema_scalar | 380 ns | 1.30x |

- [20260510T204030Z-3c0d0d5](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20260510T204030Z-3c0d0d5/report.md)
- [20261114T210602Z-266650fb-dirty](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20261114T210602Z-266650fb-dirty/report.md)
- [20261116T200123Z-07a9e9a3](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20261116T200123Z-07a9e9a3/report.md)

## ema_axis_h/ema_axis_h_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T035736Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 13 ns | 1.06x |
| ema_neon | 12 ns | 1.00x |
| ema_scalar | 15 ns | 1.20x |

- [20261115T035736Z-266650fb-dirty](ema_axis_h/ema_axis_h_n64/linux-x86_64-c16-bare-intel/20261115T035736Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200123Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 0 ns | 1.00x |
| ema_neon | 0 ns | 1.00x |
| ema_scalar | 3 ns | 7.25x |

- [20260510T204028Z-3c0d0d5](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20260510T204028Z-3c0d0d5/report.md)
- [20261114T210556Z-266650fb-dirty](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20261114T210556Z-266650fb-dirty/report.md)
- [20261116T200123Z-07a9e9a3](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20261116T200123Z-07a9e9a3/report.md)

## ema_formulation/ema_formulation_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041327Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 2.00 us | 2.32x |
| ema_form_intshift | 863 ns | 1.00x |
| ema_form_q32 | 1.43 us | 1.66x |

- [20261115T041327Z-266650fb-dirty](ema_formulation/ema_formulation_n1024/linux-x86_64-c16-bare-intel/20261115T041327Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200207Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 2.09 us | 1.71x |
| ema_form_intshift | 1.22 us | 1.00x |
| ema_form_q32 | 1.86 us | 1.52x |

- [20260510T233132Z-5dcdb36](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20260510T233132Z-5dcdb36/report.md)
- [20261114T210643Z-266650fb-dirty](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20261114T210643Z-266650fb-dirty/report.md)
- [20261116T200207Z-07a9e9a3](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20261116T200207Z-07a9e9a3/report.md)

## ema_formulation/ema_formulation_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041834Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 31.96 us | 2.33x |
| ema_form_intshift | 13.70 us | 1.00x |
| ema_form_q32 | 22.83 us | 1.67x |

- [20261115T041834Z-266650fb-dirty](ema_formulation/ema_formulation_n16384/linux-x86_64-c16-bare-intel/20261115T041834Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200207Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 35.70 us | 1.75x |
| ema_form_intshift | 20.40 us | 1.00x |
| ema_form_q32 | 30.65 us | 1.50x |

- [20260510T233138Z-5dcdb36](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20260510T233138Z-5dcdb36/report.md)
- [20261114T210651Z-266650fb-dirty](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20261114T210651Z-266650fb-dirty/report.md)
- [20261116T200207Z-07a9e9a3](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20261116T200207Z-07a9e9a3/report.md)

## ema_formulation/ema_formulation_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041052Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 509 ns | 2.29x |
| ema_form_intshift | 222 ns | 1.00x |
| ema_form_q32 | 363 ns | 1.64x |

- [20261115T041052Z-266650fb-dirty](ema_formulation/ema_formulation_n256/linux-x86_64-c16-bare-intel/20261115T041052Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200207Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 412 ns | 1.56x |
| ema_form_intshift | 264 ns | 1.00x |
| ema_form_q32 | 418 ns | 1.58x |

- [20260510T233131Z-5dcdb36](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20260510T233131Z-5dcdb36/report.md)
- [20261114T210641Z-266650fb-dirty](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20261114T210641Z-266650fb-dirty/report.md)
- [20261116T200207Z-07a9e9a3](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20261116T200207Z-07a9e9a3/report.md)

## ema_formulation/ema_formulation_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T041559Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 8.00 us | 2.33x |
| ema_form_intshift | 3.43 us | 1.00x |
| ema_form_q32 | 5.72 us | 1.67x |

- [20261115T041559Z-266650fb-dirty](ema_formulation/ema_formulation_n4096/linux-x86_64-c16-bare-intel/20261115T041559Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200207Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 8.80 us | 1.74x |
| ema_form_intshift | 5.06 us | 1.00x |
| ema_form_q32 | 7.61 us | 1.50x |

- [20260510T233134Z-5dcdb36](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20260510T233134Z-5dcdb36/report.md)
- [20261114T210646Z-266650fb-dirty](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20261114T210646Z-266650fb-dirty/report.md)
- [20261116T200207Z-07a9e9a3](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20261116T200207Z-07a9e9a3/report.md)

## engine_vs_std/accumulator_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013654Z-23953983](engine_vs_std/accumulator_n1048576/linux-x86_64-c4-container-intel/20261114T013654Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 736.86 us | 7.86x |
| evs_engine_par | 234.43 us | 2.50x |
| evs_std | 105.34 us | 1.12x |
| evs_std_par | 93.80 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/accumulator_n1048576/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/accumulator_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013321Z-c894a779](engine_vs_std/accumulator_n4096/linux-x86_64-c4-container-intel/20261114T013321Z-c894a779/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 13.02 us | 31.25x |
| evs_engine_par | 52.94 us | 127.05x |
| evs_std | 417 ns | 1.00x |
| evs_std_par | 28.40 us | 68.15x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/accumulator_n4096/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/accumulator_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013855Z-23953983](engine_vs_std/accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T013855Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 2.83 ms | 5.26x |
| evs_engine_par | 735.53 us | 1.37x |
| evs_std | 587.00 us | 1.09x |
| evs_std_par | 538.24 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/accumulator_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/accumulator_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013507Z-23953983](engine_vs_std/accumulator_n65536/linux-x86_64-c4-container-intel/20261114T013507Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 54.92 us | 8.49x |
| evs_engine_par | 68.55 us | 10.60x |
| evs_std | 6.47 us | 1.00x |
| evs_std_par | 35.35 us | 5.47x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/accumulator_n65536/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/branching_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014141Z-23953983](engine_vs_std/branching_n1048576/linux-x86_64-c4-container-intel/20261114T014141Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 338.66 us | 2.69x |
| evs_std | 125.93 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/branching_n1048576/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/branching_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013949Z-23953983](engine_vs_std/branching_n4096/linux-x86_64-c4-container-intel/20261114T013949Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 11.22 us | 23.02x |
| evs_std | 488 ns | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/branching_n4096/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/branching_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014245Z-23953983](engine_vs_std/branching_n4194304/linux-x86_64-c4-container-intel/20261114T014245Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 1.97 ms | 3.33x |
| evs_std | 589.48 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/branching_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/branching_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014044Z-23953983](engine_vs_std/branching_n65536/linux-x86_64-c4-container-intel/20261114T014044Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 27.90 us | 3.60x |
| evs_std | 7.75 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/branching_n65536/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/element_wise_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014531Z-23953983](engine_vs_std/element_wise_n1048576/linux-x86_64-c4-container-intel/20261114T014531Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 107.28 us | 1.02x |
| evs_std | 105.00 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/element_wise_n1048576/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/element_wise_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014339Z-23953983](engine_vs_std/element_wise_n4096/linux-x86_64-c4-container-intel/20261114T014339Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 458 ns | 1.12x |
| evs_std | 408 ns | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/element_wise_n4096/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/element_wise_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014634Z-23953983](engine_vs_std/element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014634Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 570.99 us | 1.00x |
| evs_std | 584.11 us | 1.02x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/element_wise_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/element_wise_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014434Z-23953983](engine_vs_std/element_wise_n65536/linux-x86_64-c4-container-intel/20261114T014434Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 6.78 us | 1.05x |
| evs_std | 6.47 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/element_wise_n65536/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/startup_accumulator_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014741Z-23953983](engine_vs_std/startup_accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T014741Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 283.08 us | 1.00x |
| evs_std_startup | 465.69 us | 1.65x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/startup_accumulator_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/startup_branching_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014850Z-23953983](engine_vs_std/startup_branching_n4194304/linux-x86_64-c4-container-intel/20261114T014850Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 1.08 ms | 2.60x |
| evs_std_startup | 415.35 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/startup_branching_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/startup_element_wise_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014957Z-23953983](engine_vs_std/startup_element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014957Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 1.34 ms | 3.33x |
| evs_std_startup | 401.86 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/startup_element_wise_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/startup_wide_parallel_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015143Z-23953983](engine_vs_std/startup_wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T015143Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 2.07 ms | 1.07x |
| evs_std_startup | 1.93 ms | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/startup_wide_parallel_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/wide_parallel_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015712Z-23953983](engine_vs_std/wide_parallel_n1048576/linux-x86_64-c4-container-intel/20261114T015712Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 2.97 ms | 3.44x |
| evs_engine_par | 892.94 us | 1.03x |
| evs_std | 2.95 ms | 3.41x |
| evs_std_par | 862.84 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/wide_parallel_n1048576/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/wide_parallel_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015326Z-23953983](engine_vs_std/wide_parallel_n4096/linux-x86_64-c4-container-intel/20261114T015326Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 21.70 us | 1.87x |
| evs_engine_par | 64.00 us | 5.51x |
| evs_std | 11.62 us | 1.00x |
| evs_std_par | 31.97 us | 2.75x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/wide_parallel_n4096/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/wide_parallel_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T020005Z-23953983](engine_vs_std/wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T020005Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 11.81 ms | 3.61x |
| evs_engine_par | 3.28 ms | 1.00x |
| evs_std | 11.82 ms | 3.61x |
| evs_std_par | 3.28 ms | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/wide_parallel_n4194304/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## engine_vs_std/wide_parallel_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015511Z-23953983](engine_vs_std/wide_parallel_n65536/linux-x86_64-c4-container-intel/20261114T015511Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T200318Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| evs_engine | 194.99 us | 1.91x |
| evs_engine_par | 114.25 us | 1.12x |
| evs_std | 183.18 us | 1.80x |
| evs_std_par | 102.03 us | 1.00x |

- [20261116T200318Z-07a9e9a3](engine_vs_std/wide_parallel_n65536/macos-aarch64-c8-bare-apple/20261116T200318Z-07a9e9a3/report.md)

## fiber_theory/fiber_theory_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042302Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 95 ns | 1.00x |
| fib_spectral | 72.54 us | 761.22x |

- [20261115T042302Z-266650fb-dirty](fiber_theory/fiber_theory_n16/linux-x86_64-c16-bare-intel/20261115T042302Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200701Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 91 ns | 1.00x |
| fib_spectral | 64.31 us | 705.18x |

- [20260719T190052Z-e4cd1757-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20260719T190052Z-e4cd1757-dirty/report.md)
- [20261114T210756Z-266650fb-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20261114T210756Z-266650fb-dirty/report.md)
- [20261116T200701Z-07a9e9a3](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20261116T200701Z-07a9e9a3/report.md)

## fiber_theory/fiber_theory_n32

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042449Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 191 ns | 1.00x |
| fib_spectral | 86.81 us | 455.22x |

- [20261115T042449Z-266650fb-dirty](fiber_theory/fiber_theory_n32/linux-x86_64-c16-bare-intel/20261115T042449Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200701Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 183 ns | 1.00x |
| fib_spectral | 79.65 us | 435.46x |

- [20260719T190056Z-e4cd1757-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20260719T190056Z-e4cd1757-dirty/report.md)
- [20261114T210800Z-266650fb-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20261114T210800Z-266650fb-dirty/report.md)
- [20261116T200701Z-07a9e9a3](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20261116T200701Z-07a9e9a3/report.md)

## fiber_theory/fiber_theory_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042637Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 401 ns | 1.00x |
| fib_spectral | 113.57 us | 282.93x |

- [20261115T042637Z-266650fb-dirty](fiber_theory/fiber_theory_n64/linux-x86_64-c16-bare-intel/20261115T042637Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200701Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 381 ns | 1.00x |
| fib_spectral | 124.26 us | 325.80x |

- [20260719T190101Z-e4cd1757-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20260719T190101Z-e4cd1757-dirty/report.md)
- [20261114T210807Z-266650fb-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20261114T210807Z-266650fb-dirty/report.md)
- [20261116T200701Z-07a9e9a3](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20261116T200701Z-07a9e9a3/report.md)

## fiber_theory/fiber_theory_n8

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042113Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 50 ns | 1.00x |
| fib_spectral | 16.08 us | 320.29x |

- [20261115T042113Z-266650fb-dirty](fiber_theory/fiber_theory_n8/linux-x86_64-c16-bare-intel/20261115T042113Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200701Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 35 ns | 1.00x |
| fib_spectral | 16.50 us | 476.84x |

- [20260719T190048Z-e4cd1757-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20260719T190048Z-e4cd1757-dirty/report.md)
- [20261114T210752Z-266650fb-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20261114T210752Z-266650fb-dirty/report.md)
- [20261116T200701Z-07a9e9a3](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20261116T200701Z-07a9e9a3/report.md)

## float_width/float_width_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043214Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 2.57 us | 4.44x |
| float_f64 | 2.53 us | 4.38x |
| float_i64_scaled | 578 ns | 1.00x |

- [20261115T043214Z-266650fb-dirty](float_width/float_width_n1024/linux-x86_64-c16-bare-intel/20261115T043214Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200753Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 250 ns | 1.00x |
| float_f64 | 312 ns | 1.25x |
| float_i64_scaled | 423 ns | 1.69x |

- [20260511T041156Z-e203deb](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20260511T041156Z-e203deb/report.md)
- [20261114T210846Z-266650fb-dirty](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20261114T210846Z-266650fb-dirty/report.md)
- [20261116T200753Z-07a9e9a3](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20261116T200753Z-07a9e9a3/report.md)

## float_width/float_width_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043721Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 40.51 us | 4.43x |
| float_f64 | 40.51 us | 4.43x |
| float_i64_scaled | 9.14 us | 1.00x |

- [20261115T043721Z-266650fb-dirty](float_width/float_width_n16384/linux-x86_64-c16-bare-intel/20261115T043721Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200753Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 5.04 us | 1.00x |
| float_f64 | 5.11 us | 1.01x |
| float_i64_scaled | 6.42 us | 1.27x |

- [20260511T041159Z-e203deb](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20260511T041159Z-e203deb/report.md)
- [20261114T210851Z-266650fb-dirty](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20261114T210851Z-266650fb-dirty/report.md)
- [20261116T200753Z-07a9e9a3](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20261116T200753Z-07a9e9a3/report.md)

## float_width/float_width_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042939Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 527 ns | 3.49x |
| float_f64 | 532 ns | 3.52x |
| float_i64_scaled | 151 ns | 1.00x |

- [20261115T042939Z-266650fb-dirty](float_width/float_width_n256/linux-x86_64-c16-bare-intel/20261115T042939Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200753Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 46 ns | 1.00x |
| float_f64 | 73 ns | 1.60x |
| float_i64_scaled | 120 ns | 2.63x |

- [20260511T041155Z-e203deb](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20260511T041155Z-e203deb/report.md)
- [20261114T210843Z-266650fb-dirty](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20261114T210843Z-266650fb-dirty/report.md)
- [20261116T200753Z-07a9e9a3](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20261116T200753Z-07a9e9a3/report.md)

## float_width/float_width_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043446Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| float_f32 | 10.41 us | 4.55x |
| float_f64 | 10.55 us | 4.61x |
| float_i64_scaled | 2.29 us | 1.00x |

- [20261115T043446Z-266650fb-dirty](float_width/float_width_n4096/linux-x86_64-c16-bare-intel/20261115T043446Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200753Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| float_f32 | 1.21 us | 1.00x |
| float_f64 | 1.27 us | 1.05x |
| float_i64_scaled | 1.62 us | 1.34x |

- [20260511T041157Z-e203deb](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20260511T041157Z-e203deb/report.md)
- [20261114T210848Z-266650fb-dirty](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20261114T210848Z-266650fb-dirty/report.md)
- [20261116T200753Z-07a9e9a3](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20261116T200753Z-07a9e9a3/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T044118Z-266650fb`

| variant | median | vs best |
|---|---|---|
| fpt_table_16 | 178 ns | 1.00x |

- [20261115T044117Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/linux-x86_64-c16-bare-intel/20261115T044117Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200835Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 208 ns | 1.00x |
| fpt_table_16 | 317 ns | 1.52x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210922Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20261114T210922Z-266650fb-dirty/report.md)
- [20261116T200835Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20261116T200835Z-07a9e9a3/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044445Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 12.99 us | 1.00x |
| fpt_table_16 | 15.12 us | 1.16x |

- [20261115T044445Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/linux-x86_64-c16-bare-intel/20261115T044445Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200835Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 10.66 us | 1.00x |
| fpt_table_16 | 11.41 us | 1.07x |

- [20260511T051112Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20260511T051112Z-9b76747/report.md)
- [20261114T210926Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20261114T210926Z-266650fb-dirty/report.md)
- [20261116T200835Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20261116T200835Z-07a9e9a3/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043931Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 34 ns | 1.13x |
| fpt_table_16 | 30 ns | 1.00x |

- [20261115T043931Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/linux-x86_64-c16-bare-intel/20261115T043931Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200835Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 64 ns | 1.53x |
| fpt_table_16 | 42 ns | 1.00x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210920Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20261114T210920Z-266650fb-dirty/report.md)
- [20261116T200835Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20261116T200835Z-07a9e9a3/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044301Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 590 ns | 1.00x |
| fpt_table_16 | 1.99 us | 3.36x |

- [20261115T044301Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/linux-x86_64-c16-bare-intel/20261115T044301Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200835Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 787 ns | 1.00x |
| fpt_table_16 | 1.24 us | 1.58x |

- [20260511T051110Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20260511T051110Z-9b76747/report.md)
- [20261114T210923Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20261114T210923Z-266650fb-dirty/report.md)
- [20261116T200835Z-07a9e9a3](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20261116T200835Z-07a9e9a3/report.md)

## fold_strategy/fold_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045022Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 79 ns | 1.76x |
| fold_quad | 45 ns | 1.00x |
| fold_sequential | 150 ns | 3.31x |

- [20261115T045022Z-266650fb-dirty](fold_strategy/fold_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T045022Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200920Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 67 ns | 1.29x |
| fold_quad | 52 ns | 1.00x |
| fold_sequential | 108 ns | 2.06x |

- [20260511T023054Z-7ed697d](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T023054Z-7ed697d/report.md)
- [20261114T211005Z-266650fb-dirty](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211005Z-266650fb-dirty/report.md)
- [20261116T200920Z-07a9e9a3](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T200920Z-07a9e9a3/report.md)

## fold_strategy/fold_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045526Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 1.15 us | 1.98x |
| fold_quad | 580 ns | 1.00x |
| fold_sequential | 2.29 us | 3.94x |

- [20261115T045526Z-266650fb-dirty](fold_strategy/fold_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T045526Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200920Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 1.27 us | 1.93x |
| fold_quad | 658 ns | 1.00x |
| fold_sequential | 2.51 us | 3.81x |

- [20260511T023056Z-7ed697d](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T023056Z-7ed697d/report.md)
- [20261114T211009Z-266650fb-dirty](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211009Z-266650fb-dirty/report.md)
- [20261116T200920Z-07a9e9a3](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T200920Z-07a9e9a3/report.md)

## fold_strategy/fold_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044747Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 26 ns | 1.23x |
| fold_quad | 21 ns | 1.00x |
| fold_sequential | 43 ns | 2.03x |

- [20261115T044747Z-266650fb-dirty](fold_strategy/fold_strategy_n256/linux-x86_64-c16-bare-intel/20261115T044747Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200920Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 15 ns | 1.03x |
| fold_quad | 15 ns | 1.00x |
| fold_sequential | 15 ns | 1.00x |

- [20260511T023053Z-7ed697d](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20260511T023053Z-7ed697d/report.md)
- [20261114T211003Z-266650fb-dirty](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211003Z-266650fb-dirty/report.md)
- [20261116T200920Z-07a9e9a3](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20261116T200920Z-07a9e9a3/report.md)

## fold_strategy/fold_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045253Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fold_paired | 294 ns | 1.93x |
| fold_quad | 152 ns | 1.00x |
| fold_sequential | 578 ns | 3.79x |

- [20261115T045253Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T045253Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T200920Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fold_paired | 311 ns | 1.77x |
| fold_quad | 176 ns | 1.00x |
| fold_sequential | 589 ns | 3.35x |

- [20260511T023055Z-7ed697d](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T023055Z-7ed697d/report.md)
- [20261114T211007Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211007Z-266650fb-dirty/report.md)
- [20261116T200920Z-07a9e9a3](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T200920Z-07a9e9a3/report.md)

## frame_wide/heavy_n1048576

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 9.35 ms | 3.63x |
| engine_par | 7.97 ms | 3.10x |
| hand | 13.37 ms | 5.19x |
| hand_par | 3.11 ms | 1.21x |
| rust_nostd | 9.31 ms | 3.62x |
| rust_std | 9.48 ms | 3.69x |
| rust_std_par | 2.57 ms | 1.00x |

- [20261116T201221Z-07a9e9a3](frame_wide/heavy_n1048576/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/heavy_n16777216

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 149.96 ms | 3.44x |
| engine_par | 126.23 ms | 2.89x |
| hand | 213.98 ms | 4.90x |
| hand_par | 47.18 ms | 1.08x |
| rust_nostd | 148.64 ms | 3.41x |
| rust_std | 149.46 ms | 3.43x |
| rust_std_par | 43.63 ms | 1.00x |

- [20261116T201221Z-07a9e9a3](frame_wide/heavy_n16777216/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/heavy_n4194304

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 37.42 ms | 3.37x |
| engine_par | 31.56 ms | 2.85x |
| hand | 53.67 ms | 4.84x |
| hand_par | 12.33 ms | 1.11x |
| rust_nostd | 37.35 ms | 3.37x |
| rust_std | 37.38 ms | 3.37x |
| rust_std_par | 11.09 ms | 1.00x |

- [20261116T201221Z-07a9e9a3](frame_wide/heavy_n4194304/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/heavy_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151355Z-c1eee3f5-dirty](frame_wide/heavy_n65536/linux-x86_64-c4-container-intel/20261113T151355Z-c1eee3f5-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 590.39 us | 2.77x |
| engine_par | 541.68 us | 2.54x |
| hand | 834.58 us | 3.92x |
| hand_par | 246.02 us | 1.15x |
| rust_nostd | 577.08 us | 2.71x |
| rust_std | 581.70 us | 2.73x |
| rust_std_par | 213.09 us | 1.00x |

- [20261116T201221Z-07a9e9a3](frame_wide/heavy_n65536/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/light_n1048576

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 1.85 ms | 8.90x |
| engine_par | 1.84 ms | 8.84x |
| hand | 270.26 us | 1.30x |
| hand_par | 207.75 us | 1.00x |
| rust_nostd | 1.70 ms | 8.18x |
| rust_std | 1.48 ms | 7.12x |
| rust_std_par | 1.27 ms | 6.09x |

- [20261116T201221Z-07a9e9a3](frame_wide/light_n1048576/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/light_n16777216

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 33.63 ms | 4.31x |
| engine_par | 31.97 ms | 4.10x |
| hand | 7.81 ms | 1.00x |
| hand_par | 8.13 ms | 1.04x |
| rust_nostd | 29.68 ms | 3.80x |
| rust_std | 29.94 ms | 3.84x |
| rust_std_par | 44.23 ms | 5.67x |

- [20261116T201221Z-07a9e9a3](frame_wide/light_n16777216/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/light_n4194304

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 8.25 ms | 4.53x |
| engine_par | 8.04 ms | 4.41x |
| hand | 1.82 ms | 1.00x |
| hand_par | 1.83 ms | 1.00x |
| rust_nostd | 7.45 ms | 4.09x |
| rust_std | 7.47 ms | 4.10x |
| rust_std_par | 10.54 ms | 5.79x |

- [20261116T201221Z-07a9e9a3](frame_wide/light_n4194304/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## frame_wide/light_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151715Z-c1eee3f5-dirty](frame_wide/light_n65536/linux-x86_64-c4-container-intel/20261113T151715Z-c1eee3f5-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T201221Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| engine | 78.05 us | 5.22x |
| engine_par | 111.08 us | 7.42x |
| hand | 14.96 us | 1.00x |
| hand_par | 44.44 us | 2.97x |
| rust_nostd | 70.08 us | 4.68x |
| rust_std | 71.33 us | 4.77x |
| rust_std_par | 85.07 us | 5.69x |

- [20261116T201221Z-07a9e9a3](frame_wide/light_n65536/macos-aarch64-c8-bare-apple/20261116T201221Z-07a9e9a3/report.md)

## fxpmul_strategy/fxpmul_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050123Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 97 ns | 1.00x |
| fxpmul_precise | 151 ns | 1.55x |

- [20261115T050123Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T050123Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202732Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 92 ns | 1.00x |
| fxpmul_precise | 108 ns | 1.17x |

- [20260510T213422Z-b042648](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213422Z-b042648/report.md)
- [20261114T211059Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211059Z-266650fb-dirty/report.md)
- [20261116T202732Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T202732Z-07a9e9a3/report.md)

## fxpmul_strategy/fxpmul_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050449Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.43 us | 1.00x |
| fxpmul_precise | 2.29 us | 1.60x |

- [20261115T050449Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T050449Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202732Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.58 us | 1.00x |
| fxpmul_precise | 1.91 us | 1.20x |

- [20260510T213424Z-b042648](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213424Z-b042648/report.md)
- [20261114T211102Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211102Z-266650fb-dirty/report.md)
- [20261116T202732Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T202732Z-07a9e9a3/report.md)

## fxpmul_strategy/fxpmul_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045940Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 30 ns | 1.00x |
| fxpmul_precise | 44 ns | 1.44x |

- [20261115T045940Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/linux-x86_64-c16-bare-intel/20261115T045940Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202732Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 18 ns | 1.00x |
| fxpmul_precise | 25 ns | 1.40x |

- [20260510T213421Z-b042648](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213421Z-b042648/report.md)
- [20261114T211058Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211058Z-266650fb-dirty/report.md)
- [20261116T202732Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20261116T202732Z-07a9e9a3/report.md)

## fxpmul_strategy/fxpmul_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050306Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 364 ns | 1.00x |
| fxpmul_precise | 578 ns | 1.59x |

- [20261115T050306Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T050306Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202732Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 387 ns | 1.00x |
| fxpmul_precise | 468 ns | 1.21x |

- [20260510T213423Z-b042648](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213423Z-b042648/report.md)
- [20261114T211101Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211101Z-266650fb-dirty/report.md)
- [20261116T202732Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T202732Z-07a9e9a3/report.md)

## fxpmul_strategy/fxpmul_strategy_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045753Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 14 ns | 1.00x |
| fxpmul_precise | 17 ns | 1.25x |

- [20261115T045753Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/linux-x86_64-c16-bare-intel/20261115T045753Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202732Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 2 ns | 1.00x |
| fxpmul_precise | 3 ns | 1.16x |

- [20260510T213420Z-b042648](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213420Z-b042648/report.md)
- [20261114T211056Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211056Z-266650fb-dirty/report.md)
- [20261116T202732Z-07a9e9a3](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20261116T202732Z-07a9e9a3/report.md)

## hash_algos/hash_algos_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051258Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 1.15 us | 6.03x |
| hash_sip | 202 ns | 1.06x |
| hash_xxh | 190 ns | 1.00x |

- [20261115T051258Z-266650fb-dirty](hash_algos/hash_algos_n1024/linux-x86_64-c16-bare-intel/20261115T051258Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202817Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 1.23 us | 5.32x |
| hash_sip | 269 ns | 1.17x |
| hash_xxh | 231 ns | 1.00x |

- [20260510T213309Z-b042648](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20260510T213309Z-b042648/report.md)
- [20261114T211142Z-266650fb-dirty](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20261114T211142Z-266650fb-dirty/report.md)
- [20261116T202817Z-07a9e9a3](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20261116T202817Z-07a9e9a3/report.md)

## hash_algos/hash_algos_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051803Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 18.27 us | 6.39x |
| hash_sip | 3.07 us | 1.07x |
| hash_xxh | 2.86 us | 1.00x |

- [20261115T051803Z-266650fb-dirty](hash_algos/hash_algos_n16384/linux-x86_64-c16-bare-intel/20261115T051803Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202817Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 20.41 us | 4.50x |
| hash_sip | 4.54 us | 1.00x |
| hash_xxh | 4.62 us | 1.02x |

- [20260510T213314Z-b042648](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20260510T213314Z-b042648/report.md)
- [20261114T211148Z-266650fb-dirty](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20261114T211148Z-266650fb-dirty/report.md)
- [20261116T202817Z-07a9e9a3](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20261116T202817Z-07a9e9a3/report.md)

## hash_algos/hash_algos_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051026Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 292 ns | 5.65x |
| hash_sip | 58 ns | 1.13x |
| hash_xxh | 52 ns | 1.00x |

- [20261115T051026Z-266650fb-dirty](hash_algos/hash_algos_n256/linux-x86_64-c16-bare-intel/20261115T051026Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202817Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 268 ns | 7.91x |
| hash_sip | 56 ns | 1.66x |
| hash_xxh | 34 ns | 1.00x |

- [20260510T213307Z-b042648](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20260510T213307Z-b042648/report.md)
- [20261114T211140Z-266650fb-dirty](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20261114T211140Z-266650fb-dirty/report.md)
- [20261116T202817Z-07a9e9a3](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20261116T202817Z-07a9e9a3/report.md)

## hash_algos/hash_algos_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T051530Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 4.57 us | 6.30x |
| hash_sip | 782 ns | 1.08x |
| hash_xxh | 725 ns | 1.00x |

- [20261115T051530Z-266650fb-dirty](hash_algos/hash_algos_n4096/linux-x86_64-c16-bare-intel/20261115T051530Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202817Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 5.06 us | 4.73x |
| hash_sip | 1.12 us | 1.05x |
| hash_xxh | 1.07 us | 1.00x |

- [20260510T213311Z-b042648](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20260510T213311Z-b042648/report.md)
- [20261114T211145Z-266650fb-dirty](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20261114T211145Z-266650fb-dirty/report.md)
- [20261116T202817Z-07a9e9a3](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20261116T202817Z-07a9e9a3/report.md)

## hash_algos/hash_algos_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050751Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 78 ns | 4.19x |
| hash_sip | 22 ns | 1.21x |
| hash_xxh | 19 ns | 1.00x |

- [20261115T050751Z-266650fb-dirty](hash_algos/hash_algos_n64/linux-x86_64-c16-bare-intel/20261115T050751Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202817Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 35 ns | 9.51x |
| hash_sip | 10 ns | 2.81x |
| hash_xxh | 4 ns | 1.00x |

- [20260510T213306Z-b042648](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20260510T213306Z-b042648/report.md)
- [20261114T211138Z-266650fb-dirty](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20261114T211138Z-266650fb-dirty/report.md)
- [20261116T202817Z-07a9e9a3](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20261116T202817Z-07a9e9a3/report.md)

## inline_strategy/inline_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T052342Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 1.15 us | 1.00x |
| inline_default | 1.15 us | 1.00x |
| inline_never | 1.15 us | 1.00x |

- [20261115T052342Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T052342Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T202910Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 1.23 us | 1.00x |
| inline_default | 1.23 us | 1.00x |
| inline_never | 1.27 us | 1.04x |

- [20260511T003101Z-63d18f0](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T003101Z-63d18f0/report.md)
- [20261114T211226Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211226Z-266650fb-dirty/report.md)
- [20261116T202910Z-07a9e9a3](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T202910Z-07a9e9a3/report.md)

## inline_strategy/inline_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T052848Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 18.27 us | 1.00x |
| inline_default | 18.27 us | 1.00x |
| inline_never | 18.27 us | 1.00x |

- [20261115T052848Z-266650fb-dirty](inline_strategy/inline_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T052848Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202910Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 20.41 us | 1.00x |
| inline_default | 20.41 us | 1.00x |
| inline_never | 20.47 us | 1.00x |

- [20260511T003107Z-63d18f0](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T003107Z-63d18f0/report.md)
- [20261114T211234Z-266650fb-dirty](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211234Z-266650fb-dirty/report.md)
- [20261116T202910Z-07a9e9a3](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T202910Z-07a9e9a3/report.md)

## inline_strategy/inline_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T052107Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| inline_always | 292 ns | 1.00x |
| inline_default | 292 ns | 1.00x |
| inline_never | 293 ns | 1.00x |

- [20261115T052107Z-266650fb-dirty](inline_strategy/inline_strategy_n256/linux-x86_64-c16-bare-intel/20261115T052107Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202910Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 268 ns | 1.00x |
| inline_default | 268 ns | 1.00x |
| inline_never | 317 ns | 1.18x |

- [20260511T003100Z-63d18f0](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20260511T003100Z-63d18f0/report.md)
- [20261114T211224Z-266650fb-dirty](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211224Z-266650fb-dirty/report.md)
- [20261116T202910Z-07a9e9a3](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20261116T202910Z-07a9e9a3/report.md)

## inline_strategy/inline_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T052615Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inline_never | 4.57 us | 1.00x |

- [20261115T052614Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T052614Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T202910Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| inline_always | 5.06 us | 1.00x |
| inline_default | 5.06 us | 1.00x |
| inline_never | 5.11 us | 1.01x |

- [20260511T003103Z-63d18f0](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T003103Z-63d18f0/report.md)
- [20261114T211229Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211229Z-266650fb-dirty/report.md)
- [20261116T202910Z-07a9e9a3](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T202910Z-07a9e9a3/report.md)

## iter_patterns/iter_patterns_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053425Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 297 ns | 11.80x |
| iter_seq | 150 ns | 5.93x |
| iter_strided | 25 ns | 1.00x |

- [20261115T053425Z-266650fb-dirty](iter_patterns/iter_patterns_n1024/linux-x86_64-c16-bare-intel/20261115T053425Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 248 ns | 70.94x |
| iter_seq | 108 ns | 30.94x |
| iter_strided | 4 ns | 1.00x |

- [20260510T231055Z-b4fbd1a](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20260510T231055Z-b4fbd1a/report.md)
- [20261114T211312Z-266650fb-dirty](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20261114T211312Z-266650fb-dirty/report.md)
- [20261116T203000Z-07a9e9a3](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20261116T203000Z-07a9e9a3/report.md)

## iter_patterns/iter_patterns_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053929Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 4.63 us | 15.86x |
| iter_seq | 2.29 us | 7.83x |
| iter_strided | 292 ns | 1.00x |

- [20261115T053929Z-266650fb-dirty](iter_patterns/iter_patterns_n16384/linux-x86_64-c16-bare-intel/20261115T053929Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 4.59 us | 17.05x |
| iter_seq | 2.51 us | 9.32x |
| iter_strided | 269 ns | 1.00x |

- [20260510T231058Z-b4fbd1a](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20260510T231058Z-b4fbd1a/report.md)
- [20261114T211316Z-266650fb-dirty](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20261114T211316Z-266650fb-dirty/report.md)
- [20261116T203000Z-07a9e9a3](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20261116T203000Z-07a9e9a3/report.md)

## iter_patterns/iter_patterns_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053150Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 82 ns | 7.10x |
| iter_seq | 43 ns | 3.71x |
| iter_strided | 12 ns | 1.00x |

- [20261115T053150Z-266650fb-dirty](iter_patterns/iter_patterns_n256/linux-x86_64-c16-bare-intel/20261115T053150Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 43 ns | 53.62x |
| iter_seq | 16 ns | 20.25x |
| iter_strided | 1 ns | 1.00x |

- [20260510T231054Z-b4fbd1a](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20260510T231054Z-b4fbd1a/report.md)
- [20261114T211310Z-266650fb-dirty](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20261114T211310Z-266650fb-dirty/report.md)
- [20261116T203000Z-07a9e9a3](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20261116T203000Z-07a9e9a3/report.md)

## iter_patterns/iter_patterns_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T053657Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| iter_gather | 1.16 us | 14.85x |
| iter_seq | 577 ns | 7.36x |
| iter_strided | 78 ns | 1.00x |

- [20261115T053657Z-266650fb-dirty](iter_patterns/iter_patterns_n4096/linux-x86_64-c16-bare-intel/20261115T053657Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203000Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| iter_gather | 1.09 us | 21.17x |
| iter_seq | 591 ns | 11.44x |
| iter_strided | 52 ns | 1.00x |

- [20260510T231056Z-b4fbd1a](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20260510T231056Z-b4fbd1a/report.md)
- [20261114T211314Z-266650fb-dirty](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20261114T211314Z-266650fb-dirty/report.md)
- [20261116T203000Z-07a9e9a3](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20261116T203000Z-07a9e9a3/report.md)

## link-static/dev_n25

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

### linux-x86_64-c4-container-intel, 1 run(s)

Latest: `20261116T083526Z-b2bbf4ba`

| variant | median | vs best |
|---|---|---|
| default_relink | 137.90 ms | 2.21x |
| default_replay | 62.37 ms | 1.00x |

- [20261116T083526Z-b2bbf4ba](link-static/release_n25/linux-x86_64-c4-container-intel/20261116T083526Z-b2bbf4ba/report.md)

## load_alignment/load_alignment_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T054506Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 150 ns | 1.00x |
| load_byte_pack | 150 ns | 1.00x |
| load_unaligned | 150 ns | 1.00x |

- [20261115T054506Z-266650fb-dirty](load_alignment/load_alignment_n1024/linux-x86_64-c16-bare-intel/20261115T054506Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203046Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 107 ns | 1.00x |
| load_byte_pack | 108 ns | 1.00x |
| load_unaligned | 107 ns | 1.00x |

- [20260511T001004Z-1fc2e8d](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20260511T001004Z-1fc2e8d/report.md)
- [20261114T211354Z-266650fb-dirty](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20261114T211354Z-266650fb-dirty/report.md)
- [20261116T203046Z-07a9e9a3](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20261116T203046Z-07a9e9a3/report.md)

## load_alignment/load_alignment_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055010Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 2.29 us | 1.00x |
| load_byte_pack | 2.29 us | 1.00x |
| load_unaligned | 2.29 us | 1.00x |

- [20261115T055010Z-266650fb-dirty](load_alignment/load_alignment_n16384/linux-x86_64-c16-bare-intel/20261115T055010Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203046Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 2.51 us | 1.00x |
| load_byte_pack | 2.51 us | 1.00x |
| load_unaligned | 2.51 us | 1.00x |

- [20260511T001006Z-1fc2e8d](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20260511T001006Z-1fc2e8d/report.md)
- [20261114T211359Z-266650fb-dirty](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20261114T211359Z-266650fb-dirty/report.md)
- [20261116T203046Z-07a9e9a3](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20261116T203046Z-07a9e9a3/report.md)

## load_alignment/load_alignment_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T054231Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 43 ns | 1.00x |
| load_byte_pack | 43 ns | 1.00x |
| load_unaligned | 43 ns | 1.00x |

- [20261115T054231Z-266650fb-dirty](load_alignment/load_alignment_n256/linux-x86_64-c16-bare-intel/20261115T054231Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203046Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 16 ns | 1.00x |
| load_byte_pack | 16 ns | 1.01x |
| load_unaligned | 16 ns | 1.01x |

- [20260511T001003Z-1fc2e8d](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20260511T001003Z-1fc2e8d/report.md)
- [20261114T211352Z-266650fb-dirty](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20261114T211352Z-266650fb-dirty/report.md)
- [20261116T203046Z-07a9e9a3](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20261116T203046Z-07a9e9a3/report.md)

## load_alignment/load_alignment_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T054738Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| load_aligned | 578 ns | 1.00x |
| load_byte_pack | 578 ns | 1.00x |
| load_unaligned | 578 ns | 1.00x |

- [20261115T054738Z-266650fb-dirty](load_alignment/load_alignment_n4096/linux-x86_64-c16-bare-intel/20261115T054738Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203046Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| load_aligned | 590 ns | 1.00x |
| load_byte_pack | 590 ns | 1.00x |
| load_unaligned | 590 ns | 1.00x |

- [20260511T001005Z-1fc2e8d](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20260511T001005Z-1fc2e8d/report.md)
- [20261114T211357Z-266650fb-dirty](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20261114T211357Z-266650fb-dirty/report.md)
- [20261116T203046Z-07a9e9a3](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20261116T203046Z-07a9e9a3/report.md)

## match_arm_count/match_arm_count_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055547Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 106 ns | 1.00x |
| match_64_arms | 181 ns | 1.70x |
| match_8_arms | 161 ns | 1.51x |

- [20261115T055547Z-266650fb-dirty](match_arm_count/match_arm_count_n1024/linux-x86_64-c16-bare-intel/20261115T055547Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203133Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 97 ns | 1.00x |
| match_64_arms | 232 ns | 2.40x |
| match_8_arms | 157 ns | 1.62x |

- [20260511T035129Z-4802e6f](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20260511T035129Z-4802e6f/report.md)
- [20261114T211437Z-266650fb-dirty](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20261114T211437Z-266650fb-dirty/report.md)
- [20261116T203133Z-07a9e9a3](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20261116T203133Z-07a9e9a3/report.md)

## match_arm_count/match_arm_count_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060052Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 1.58 us | 1.00x |
| match_64_arms | 11.31 us | 7.17x |
| match_8_arms | 11.37 us | 7.21x |

- [20261115T060052Z-266650fb-dirty](match_arm_count/match_arm_count_n16384/linux-x86_64-c16-bare-intel/20261115T060052Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203133Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 1.35 us | 1.00x |
| match_64_arms | 11.95 us | 8.85x |
| match_8_arms | 2.47 us | 1.83x |

- [20260511T035132Z-4802e6f](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20260511T035132Z-4802e6f/report.md)
- [20261114T211443Z-266650fb-dirty](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20261114T211443Z-266650fb-dirty/report.md)
- [20261116T203133Z-07a9e9a3](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20261116T203133Z-07a9e9a3/report.md)

## match_arm_count/match_arm_count_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055312Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 33 ns | 1.08x |
| match_64_arms | 39 ns | 1.29x |
| match_8_arms | 30 ns | 1.00x |

- [20261115T055312Z-266650fb-dirty](match_arm_count/match_arm_count_n256/linux-x86_64-c16-bare-intel/20261115T055312Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203133Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 26 ns | 1.00x |
| match_64_arms | 69 ns | 2.64x |
| match_8_arms | 93 ns | 3.56x |

- [20260511T035128Z-4802e6f](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20260511T035128Z-4802e6f/report.md)
- [20261114T211435Z-266650fb-dirty](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20261114T211435Z-266650fb-dirty/report.md)
- [20261116T203133Z-07a9e9a3](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20261116T203133Z-07a9e9a3/report.md)

## match_arm_count/match_arm_count_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T055819Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 404 ns | 1.00x |
| match_64_arms | 747 ns | 1.85x |
| match_8_arms | 663 ns | 1.64x |

- [20261115T055819Z-266650fb-dirty](match_arm_count/match_arm_count_n4096/linux-x86_64-c16-bare-intel/20261115T055819Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203133Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 348 ns | 1.00x |
| match_64_arms | 880 ns | 2.53x |
| match_8_arms | 619 ns | 1.78x |

- [20260511T035130Z-4802e6f](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20260511T035130Z-4802e6f/report.md)
- [20261114T211439Z-266650fb-dirty](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20261114T211439Z-266650fb-dirty/report.md)
- [20261116T203133Z-07a9e9a3](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20261116T203133Z-07a9e9a3/report.md)

## memcpy_patterns/memcpy_patterns_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060900Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 1.23 us | 1.00x |
| memcpy_naive | 1.23 us | 1.00x |
| memcpy_unrolled | 1.23 us | 1.00x |

- [20261115T060900Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n1024/linux-x86_64-c16-bare-intel/20261115T060900Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203220Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 1.42 us | 1.00x |
| memcpy_naive | 1.42 us | 1.00x |
| memcpy_unrolled | 1.43 us | 1.00x |

- [20260510T214729Z-f222646](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20260510T214729Z-f222646/report.md)
- [20261114T211523Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20261114T211523Z-266650fb-dirty/report.md)
- [20261116T203220Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20261116T203220Z-07a9e9a3/report.md)

## memcpy_patterns/memcpy_patterns_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061407Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 18.27 us | 1.00x |
| memcpy_naive | 18.27 us | 1.00x |
| memcpy_unrolled | 18.27 us | 1.00x |

- [20261115T061407Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n16384/linux-x86_64-c16-bare-intel/20261115T061407Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203220Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 20.41 us | 1.00x |
| memcpy_naive | 20.41 us | 1.00x |
| memcpy_unrolled | 20.41 us | 1.00x |

- [20260510T214734Z-f222646](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20260510T214734Z-f222646/report.md)
- [20261114T211530Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20261114T211530Z-266650fb-dirty/report.md)
- [20261116T203220Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20261116T203220Z-07a9e9a3/report.md)

## memcpy_patterns/memcpy_patterns_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060629Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 376 ns | 1.00x |
| memcpy_naive | 375 ns | 1.00x |
| memcpy_unrolled | 376 ns | 1.00x |

- [20261115T060629Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n256/linux-x86_64-c16-bare-intel/20261115T060629Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203220Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 463 ns | 1.00x |
| memcpy_naive | 464 ns | 1.00x |
| memcpy_unrolled | 468 ns | 1.01x |

- [20260510T214728Z-f222646](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20260510T214728Z-f222646/report.md)
- [20261114T211521Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20261114T211521Z-266650fb-dirty/report.md)
- [20261116T203220Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20261116T203220Z-07a9e9a3/report.md)

## memcpy_patterns/memcpy_patterns_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061133Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 4.25 us | 1.00x |
| memcpy_naive | 4.66 us | 1.10x |
| memcpy_unrolled | 4.66 us | 1.10x |

- [20261115T061133Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n4096/linux-x86_64-c16-bare-intel/20261115T061133Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203220Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 5.27 us | 1.00x |
| memcpy_naive | 5.27 us | 1.00x |
| memcpy_unrolled | 5.27 us | 1.00x |

- [20260510T214730Z-f222646](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20260510T214730Z-f222646/report.md)
- [20261114T211526Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20261114T211526Z-266650fb-dirty/report.md)
- [20261116T203220Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20261116T203220Z-07a9e9a3/report.md)

## memcpy_patterns/memcpy_patterns_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T060354Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 78 ns | 1.00x |
| memcpy_naive | 78 ns | 1.00x |
| memcpy_unrolled | 78 ns | 1.00x |

- [20261115T060354Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n64/linux-x86_64-c16-bare-intel/20261115T060354Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203220Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 36 ns | 1.01x |
| memcpy_naive | 35 ns | 1.00x |
| memcpy_unrolled | 36 ns | 1.02x |

- [20260510T214727Z-f222646](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20260510T214727Z-f222646/report.md)
- [20261114T211518Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20261114T211518Z-266650fb-dirty/report.md)
- [20261116T203220Z-07a9e9a3](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20261116T203220Z-07a9e9a3/report.md)

## minmax_strategy/minmax_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061944Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 51 ns | 1.01x |
| minmax_branchless | 50 ns | 1.00x |
| minmax_intrinsic | 52 ns | 1.02x |

- [20261115T061944Z-266650fb-dirty](minmax_strategy/minmax_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T061944Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203314Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 40 ns | 1.00x |
| minmax_branchless | 40 ns | 1.01x |
| minmax_intrinsic | 40 ns | 1.01x |

- [20260511T043126Z-da11af0](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T043126Z-da11af0/report.md)
- [20261114T211609Z-266650fb-dirty](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211609Z-266650fb-dirty/report.md)
- [20261116T203314Z-07a9e9a3](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T203314Z-07a9e9a3/report.md)

## minmax_strategy/minmax_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T062448Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 720 ns | 1.08x |
| minmax_branchless | 670 ns | 1.00x |
| minmax_intrinsic | 720 ns | 1.07x |

- [20261115T062448Z-266650fb-dirty](minmax_strategy/minmax_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T062448Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203314Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 379 ns | 1.00x |
| minmax_branchless | 383 ns | 1.01x |
| minmax_intrinsic | 379 ns | 1.00x |

- [20260511T043128Z-da11af0](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T043128Z-da11af0/report.md)
- [20261114T211613Z-266650fb-dirty](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211613Z-266650fb-dirty/report.md)
- [20261116T203314Z-07a9e9a3](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T203314Z-07a9e9a3/report.md)

## minmax_strategy/minmax_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T061709Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 34 ns | 1.00x |
| minmax_branchless | 34 ns | 1.00x |
| minmax_intrinsic | 34 ns | 1.00x |

- [20261115T061709Z-266650fb-dirty](minmax_strategy/minmax_strategy_n256/linux-x86_64-c16-bare-intel/20261115T061709Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203314Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 23 ns | 1.02x |
| minmax_branchless | 23 ns | 1.02x |
| minmax_intrinsic | 22 ns | 1.00x |

- [20260511T043125Z-da11af0](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20260511T043125Z-da11af0/report.md)
- [20261114T211607Z-266650fb-dirty](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211607Z-266650fb-dirty/report.md)
- [20261116T203314Z-07a9e9a3](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20261116T203314Z-07a9e9a3/report.md)

## minmax_strategy/minmax_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T062215Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 185 ns | 1.00x |
| minmax_branchless | 188 ns | 1.01x |
| minmax_intrinsic | 188 ns | 1.01x |

- [20261115T062215Z-266650fb-dirty](minmax_strategy/minmax_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T062215Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203314Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 109 ns | 1.00x |
| minmax_branchless | 110 ns | 1.01x |
| minmax_intrinsic | 109 ns | 1.00x |

- [20260511T043127Z-da11af0](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T043127Z-da11af0/report.md)
- [20261114T211611Z-266650fb-dirty](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211611Z-266650fb-dirty/report.md)
- [20261116T203314Z-07a9e9a3](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T203314Z-07a9e9a3/report.md)

## modulo_strategy/modulo_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063029Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 2.93 us | 2.41x |
| mod_pow2_const | 1.22 us | 1.00x |
| mod_var | 6.29 us | 5.16x |

- [20261115T063029Z-266650fb-dirty](modulo_strategy/modulo_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T063029Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203404Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 3.46 us | 2.23x |
| mod_pow2_const | 1.55 us | 1.00x |
| mod_var | 4.39 us | 2.83x |

- [20260510T234953Z-4ccf554](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T234953Z-4ccf554/report.md)
- [20261114T211652Z-266650fb-dirty](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211652Z-266650fb-dirty/report.md)
- [20261116T203404Z-07a9e9a3](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T203404Z-07a9e9a3/report.md)

## modulo_strategy/modulo_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063541Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 57.06 us | 2.94x |
| mod_pow2_const | 19.41 us | 1.00x |
| mod_var | 100.43 us | 5.18x |

- [20261115T063541Z-266650fb-dirty](modulo_strategy/modulo_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T063541Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203404Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 56.39 us | 2.21x |
| mod_pow2_const | 25.55 us | 1.00x |
| mod_var | 71.60 us | 2.80x |

- [20260510T235003Z-4ccf554](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T235003Z-4ccf554/report.md)
- [20261114T211705Z-266650fb-dirty](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211705Z-266650fb-dirty/report.md)
- [20261116T203404Z-07a9e9a3](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T203404Z-07a9e9a3/report.md)

## modulo_strategy/modulo_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T062754Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 898 ns | 2.90x |
| mod_pow2_const | 310 ns | 1.00x |
| mod_var | 1.58 us | 5.09x |

- [20261115T062754Z-266650fb-dirty](modulo_strategy/modulo_strategy_n256/linux-x86_64-c16-bare-intel/20261115T062754Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203404Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 820 ns | 2.32x |
| mod_pow2_const | 353 ns | 1.00x |
| mod_var | 1.04 us | 2.94x |

- [20260510T234952Z-4ccf554](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20260510T234952Z-4ccf554/report.md)
- [20261114T211650Z-266650fb-dirty](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211650Z-266650fb-dirty/report.md)
- [20261116T203404Z-07a9e9a3](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20261116T203404Z-07a9e9a3/report.md)

## modulo_strategy/modulo_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063303Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 14.27 us | 2.94x |
| mod_pow2_const | 4.85 us | 1.00x |
| mod_var | 25.11 us | 5.18x |

- [20261115T063303Z-266650fb-dirty](modulo_strategy/modulo_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T063303Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203404Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 14.04 us | 2.21x |
| mod_pow2_const | 6.35 us | 1.00x |
| mod_var | 17.83 us | 2.81x |

- [20260510T234956Z-4ccf554](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T234956Z-4ccf554/report.md)
- [20261114T211656Z-266650fb-dirty](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211656Z-266650fb-dirty/report.md)
- [20261116T203404Z-07a9e9a3](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T203404Z-07a9e9a3/report.md)

## popcount_strategy/popcount_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064350Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 1.80 us | 19.36x |
| popcount_std | 93 ns | 1.00x |
| popcount_swar | 189 ns | 2.03x |

- [20261115T064350Z-266650fb-dirty](popcount_strategy/popcount_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T064350Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 148 ns | 4.39x |
| popcount_std | 34 ns | 1.00x |
| popcount_swar | 149 ns | 4.41x |

- [20260510T213414Z-b042648](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213414Z-b042648/report.md)
- [20261114T211746Z-266650fb-dirty](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211746Z-266650fb-dirty/report.md)
- [20261116T203458Z-07a9e9a3](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T203458Z-07a9e9a3/report.md)

## popcount_strategy/popcount_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064856Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 37.29 us | 29.04x |
| popcount_std | 1.28 us | 1.00x |
| popcount_swar | 2.73 us | 2.13x |

- [20261115T064856Z-266650fb-dirty](popcount_strategy/popcount_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T064856Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 1.95 us | 7.19x |
| popcount_std | 271 ns | 1.00x |
| popcount_swar | 1.95 us | 7.20x |

- [20260510T213418Z-b042648](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213418Z-b042648/report.md)
- [20261114T211751Z-266650fb-dirty](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211751Z-266650fb-dirty/report.md)
- [20261116T203458Z-07a9e9a3](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T203458Z-07a9e9a3/report.md)

## popcount_strategy/popcount_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064118Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 354 ns | 10.83x |
| popcount_std | 33 ns | 1.00x |
| popcount_swar | 67 ns | 2.05x |

- [20261115T064118Z-266650fb-dirty](popcount_strategy/popcount_strategy_n256/linux-x86_64-c16-bare-intel/20261115T064118Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 38 ns | 10.24x |
| popcount_std | 4 ns | 1.00x |
| popcount_swar | 38 ns | 10.30x |

- [20260510T213413Z-b042648](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213413Z-b042648/report.md)
- [20261114T211744Z-266650fb-dirty](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211744Z-266650fb-dirty/report.md)
- [20261116T203458Z-07a9e9a3](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20261116T203458Z-07a9e9a3/report.md)

## popcount_strategy/popcount_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T064622Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 8.04 us | 24.30x |
| popcount_std | 331 ns | 1.00x |
| popcount_swar | 693 ns | 2.09x |

- [20261115T064622Z-266650fb-dirty](popcount_strategy/popcount_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T064622Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 514 ns | 6.02x |
| popcount_std | 85 ns | 1.00x |
| popcount_swar | 510 ns | 5.97x |

- [20260510T213416Z-b042648](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213416Z-b042648/report.md)
- [20261114T211749Z-266650fb-dirty](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211749Z-266650fb-dirty/report.md)
- [20261116T203458Z-07a9e9a3](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T203458Z-07a9e9a3/report.md)

## popcount_strategy/popcount_strategy_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T063843Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 91 ns | 5.96x |
| popcount_std | 15 ns | 1.00x |
| popcount_swar | 22 ns | 1.42x |

- [20261115T063843Z-266650fb-dirty](popcount_strategy/popcount_strategy_n64/linux-x86_64-c16-bare-intel/20261115T063843Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T203458Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 5 ns | 11.50x |
| popcount_std | 0 ns | 1.00x |
| popcount_swar | 5 ns | 11.50x |

- [20260510T213412Z-b042648](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213412Z-b042648/report.md)
- [20261114T211742Z-266650fb-dirty](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211742Z-266650fb-dirty/report.md)
- [20261116T203458Z-07a9e9a3](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20261116T203458Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 1.87 ms | 1.05x |
| rcm_half | 1.89 ms | 1.06x |
| rcm_rev | 1.78 ms | 1.00x |
| rcm_scr | 2.00 ms | 1.12x |

- [20260719T150703Z-2c337b67-dirty](rcm_order/rcm_order_n1024/macos-aarch64-c8-bare-apple/20260719T150703Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n1024/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 33.87 ms | 1.00x |
| rcm_half | 33.79 ms | 1.00x |
| rcm_rev | 33.97 ms | 1.01x |
| rcm_scr | 33.80 ms | 1.00x |

- [20260719T160923Z-2c337b67-dirty](rcm_order/rcm_order_n16384/macos-aarch64-c8-bare-apple/20260719T160923Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n16384/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n2048

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 4.01 ms | 1.01x |
| rcm_half | 4.09 ms | 1.03x |
| rcm_rev | 3.97 ms | 1.00x |
| rcm_scr | 4.13 ms | 1.04x |

- [20260719T151110Z-2c337b67-dirty](rcm_order/rcm_order_n2048/macos-aarch64-c8-bare-apple/20260719T151110Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n2048/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 311.47 us | 1.00x |
| rcm_half | 321.67 us | 1.03x |
| rcm_rev | 315.93 us | 1.01x |
| rcm_scr | 322.77 us | 1.04x |

- [20260719T150459Z-2c337b67-dirty](rcm_order/rcm_order_n256/macos-aarch64-c8-bare-apple/20260719T150459Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n256/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n4096

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 8.47 ms | 1.01x |
| rcm_half | 8.40 ms | 1.00x |
| rcm_rev | 8.48 ms | 1.01x |
| rcm_scr | 8.57 ms | 1.02x |

- [20260719T151919Z-2c337b67-dirty](rcm_order/rcm_order_n4096/macos-aarch64-c8-bare-apple/20260719T151919Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n4096/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n64

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 80.53 us | 1.00x |
| rcm_half | 80.67 us | 1.00x |
| rcm_rev | 80.39 us | 1.00x |
| rcm_scr | 80.75 us | 1.00x |

- [20260719T150428Z-2c337b67-dirty](rcm_order/rcm_order_n64/macos-aarch64-c8-bare-apple/20260719T150428Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n64/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_order/rcm_order_n8192

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T203640Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 17.10 ms | 1.00x |
| rcm_half | 17.05 ms | 1.00x |
| rcm_rev | 17.28 ms | 1.01x |
| rcm_scr | 17.12 ms | 1.00x |

- [20260719T153530Z-2c337b67-dirty](rcm_order/rcm_order_n8192/macos-aarch64-c8-bare-apple/20260719T153530Z-2c337b67-dirty/report.md)
- [20261116T203640Z-07a9e9a3](rcm_order/rcm_order_n8192/macos-aarch64-c8-bare-apple/20261116T203640Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 3.76 ms | 1.04x |
| riv_rowm | 3.65 ms | 1.01x |
| riv_scr | 3.74 ms | 1.03x |
| riv_snake | 3.62 ms | 1.00x |
| riv_spec | 3.71 ms | 1.02x |

- [20260719T161426Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n1024/macos-aarch64-c8-bare-apple/20260719T161426Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n1024/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 66.98 ms | 1.01x |
| riv_rowm | 66.22 ms | 1.00x |
| riv_scr | 68.63 ms | 1.04x |
| riv_snake | 67.10 ms | 1.01x |
| riv_spec | 67.49 ms | 1.02x |

- [20260719T181527Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n16384/macos-aarch64-c8-bare-apple/20260719T181527Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n16384/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n2048

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 7.74 ms | 1.01x |
| riv_rowm | 7.66 ms | 1.00x |
| riv_scr | 7.66 ms | 1.00x |
| riv_snake | 7.69 ms | 1.00x |
| riv_spec | 7.72 ms | 1.01x |

- [20260719T162210Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n2048/macos-aarch64-c8-bare-apple/20260719T162210Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n2048/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 802.34 us | 1.01x |
| riv_rowm | 803.76 us | 1.02x |
| riv_scr | 812.37 us | 1.03x |
| riv_snake | 792.25 us | 1.00x |
| riv_spec | 791.80 us | 1.00x |

- [20260719T161037Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n256/macos-aarch64-c8-bare-apple/20260719T161037Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n256/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n4096

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 15.68 ms | 1.00x |
| riv_rowm | 15.72 ms | 1.00x |
| riv_scr | 15.78 ms | 1.01x |
| riv_snake | 15.81 ms | 1.01x |
| riv_spec | 15.68 ms | 1.00x |

- [20260719T163719Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n4096/macos-aarch64-c8-bare-apple/20260719T163719Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n4096/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n64

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 180.45 us | 1.00x |
| riv_rowm | 181.12 us | 1.01x |
| riv_scr | 180.97 us | 1.01x |
| riv_snake | 180.21 us | 1.00x |
| riv_spec | 179.76 us | 1.00x |

- [20260719T160939Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n64/macos-aarch64-c8-bare-apple/20260719T160939Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n64/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

## rcm_rivals/rcm_rivals_n8192

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T214130Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 32.48 ms | 1.01x |
| riv_rowm | 32.33 ms | 1.01x |
| riv_scr | 32.29 ms | 1.00x |
| riv_snake | 32.13 ms | 1.00x |
| riv_spec | 32.14 ms | 1.00x |

- [20260719T170734Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n8192/macos-aarch64-c8-bare-apple/20260719T170734Z-2c337b67-dirty/report.md)
- [20261116T214130Z-07a9e9a3](rcm_rivals/rcm_rivals_n8192/macos-aarch64-c8-bare-apple/20261116T214130Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234145Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 849 ns | 1.01x |
| reg_bound_record | 21.87 us | 25.90x |
| reg_bound_strided | 4.97 us | 5.88x |
| reg_bound_units | 862 ns | 1.02x |
| reg_typed | 845 ns | 1.00x |
| reg_typed_twin | 844 ns | 1.00x |

- [20261114T231940Z-266650fb-dirty](registration_binding/registration_binding_n16384/macos-aarch64-c8-bare-apple/20261114T231940Z-266650fb-dirty/report.md)
- [20261116T234145Z-07a9e9a3](registration_binding/registration_binding_n16384/macos-aarch64-c8-bare-apple/20261116T234145Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T234145Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 17.22 us | 1.04x |
| reg_bound_record | 352.84 us | 21.33x |
| reg_bound_strided | 79.49 us | 4.81x |
| reg_bound_units | 17.68 us | 1.07x |
| reg_typed | 16.54 us | 1.00x |
| reg_typed_twin | 16.54 us | 1.00x |

- [20261114T232254Z-266650fb-dirty](registration_binding/registration_binding_n262144/macos-aarch64-c8-bare-apple/20261114T232254Z-266650fb-dirty/report.md)
- [20261116T234145Z-07a9e9a3](registration_binding/registration_binding_n262144/macos-aarch64-c8-bare-apple/20261116T234145Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T234145Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 217 ns | 1.01x |
| reg_bound_record | 5.47 us | 25.33x |
| reg_bound_strided | 1.25 us | 5.81x |
| reg_bound_units | 216 ns | 1.00x |
| reg_typed | 230 ns | 1.07x |
| reg_typed_twin | 231 ns | 1.07x |

- [20261114T231927Z-266650fb-dirty](registration_binding/registration_binding_n4096/macos-aarch64-c8-bare-apple/20261114T231927Z-266650fb-dirty/report.md)
- [20261116T234145Z-07a9e9a3](registration_binding/registration_binding_n4096/macos-aarch64-c8-bare-apple/20261116T234145Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261116T234145Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| reg_bound_columns | 4.50 us | 1.03x |
| reg_bound_record | 88.47 us | 20.34x |
| reg_bound_strided | 19.99 us | 4.60x |
| reg_bound_units | 4.62 us | 1.06x |
| reg_typed | 4.35 us | 1.00x |
| reg_typed_twin | 4.36 us | 1.00x |

- [20261114T232022Z-266650fb-dirty](registration_binding/registration_binding_n65536/macos-aarch64-c8-bare-apple/20261114T232022Z-266650fb-dirty/report.md)
- [20261116T234145Z-07a9e9a3](registration_binding/registration_binding_n65536/macos-aarch64-c8-bare-apple/20261116T234145Z-07a9e9a3/report.md)

## resource_storage/false_sharing_n1024

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fs_packed | 2.01 ms | 2.38x |
| fs_padded | 844.42 us | 1.00x |

- [20261116T234625Z-07a9e9a3](resource_storage/false_sharing_n1024/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/false_sharing_n16384

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fs_packed | 31.42 ms | 2.70x |
| fs_padded | 11.64 ms | 1.00x |

- [20261116T234625Z-07a9e9a3](resource_storage/false_sharing_n16384/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/false_sharing_n65536

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| fs_packed | 124.61 ms | 3.02x |
| fs_padded | 41.28 ms | 1.00x |

- [20261116T234625Z-07a9e9a3](resource_storage/false_sharing_n65536/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_bwide_n16384

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wide_blob | 4.11 us | 1.00x |
| wide_decomposed | 7.81 us | 1.90x |

- [20260619T182437Z-9d530d12](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20260619T182437Z-9d530d12/report.md)
- [20260702T102749Z-677b60f8](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20260702T102749Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_bwide_n16384/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_bwide_n256

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wide_blob | 16.85 us | 1.00x |
| wide_decomposed | 32.01 us | 1.90x |

- [20260619T182434Z-9d530d12](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20260619T182434Z-9d530d12/report.md)
- [20260702T102747Z-677b60f8](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20260702T102747Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_bwide_n256/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_bwide_n4194304

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wide_blob | 46 ns | 1.00x |
| wide_decomposed | 136 ns | 2.98x |

- [20260619T182613Z-9d530d12](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20260619T182613Z-9d530d12/report.md)
- [20260702T102929Z-677b60f8](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20260702T102929Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_bwide_n4194304/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_clean_n1024

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 28 ns | 1.00x |
| v1_snapshot | 29 ns | 1.03x |
| v2_decomposed | 28 ns | 1.02x |
| v3_shapebound | 28 ns | 1.01x |
| v4_erased | 28 ns | 1.02x |
| v5_handletable | 30 ns | 1.10x |

- [20260619T183622Z-9d530d12](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20260619T183622Z-9d530d12/report.md)
- [20260702T103426Z-677b60f8](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20260702T103426Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_clean_n1024/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_clean_n16384

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 437 ns | 1.00x |
| v1_snapshot | 436 ns | 1.00x |
| v2_decomposed | 436 ns | 1.00x |
| v3_shapebound | 436 ns | 1.00x |
| v4_erased | 437 ns | 1.00x |
| v5_handletable | 441 ns | 1.01x |

- [20260619T183639Z-9d530d12](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20260619T183639Z-9d530d12/report.md)
- [20260702T103444Z-677b60f8](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20260702T103444Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_clean_n16384/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_clean_n262144

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 6.53 us | 1.00x |
| v1_snapshot | 6.52 us | 1.00x |
| v2_decomposed | 6.52 us | 1.00x |
| v3_shapebound | 6.51 us | 1.00x |
| v4_erased | 6.54 us | 1.00x |
| v5_handletable | 6.55 us | 1.01x |

- [20260619T184039Z-9d530d12](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20260619T184039Z-9d530d12/report.md)
- [20260702T103856Z-a2410c71](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20260702T103856Z-a2410c71/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_clean_n262144/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_heavy_thrash_n1024

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 28 ns | 1.00x |
| v1_snapshot | 28 ns | 1.00x |
| v2_decomposed | 28 ns | 1.02x |
| v3_shapebound | 28 ns | 1.00x |
| v4_erased | 28 ns | 1.03x |
| v5_handletable | 31 ns | 1.10x |

- [20260619T183150Z-9d530d12](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20260619T183150Z-9d530d12/report.md)
- [20260702T102934Z-677b60f8](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20260702T102934Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_heavy_thrash_n1024/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_heavy_thrash_n16384

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 437 ns | 1.00x |
| v1_snapshot | 436 ns | 1.00x |
| v2_decomposed | 436 ns | 1.00x |
| v3_shapebound | 435 ns | 1.00x |
| v4_erased | 436 ns | 1.00x |
| v5_handletable | 441 ns | 1.01x |

- [20260619T183213Z-9d530d12](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20260619T183213Z-9d530d12/report.md)
- [20260702T102953Z-677b60f8](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20260702T102953Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_heavy_thrash_n16384/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_heavy_thrash_n262144

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 6.51 us | 1.00x |
| v1_snapshot | 6.52 us | 1.00x |
| v2_decomposed | 6.52 us | 1.00x |
| v3_shapebound | 6.50 us | 1.00x |
| v4_erased | 6.53 us | 1.00x |
| v5_handletable | 6.56 us | 1.01x |

- [20260619T183619Z-9d530d12](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20260619T183619Z-9d530d12/report.md)
- [20260702T103423Z-677b60f8](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20260702T103423Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_heavy_thrash_n262144/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_light_thrash_n1024

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 28 ns | 1.00x |
| v1_snapshot | 28 ns | 1.01x |
| v2_decomposed | 28 ns | 1.01x |
| v3_shapebound | 28 ns | 1.00x |
| v4_erased | 29 ns | 1.03x |
| v5_handletable | 31 ns | 1.10x |

- [20260619T182616Z-9d530d12](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20260619T182616Z-9d530d12/report.md)
- [20260702T102311Z-677b60f8](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20260702T102311Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_light_thrash_n1024/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_light_thrash_n16384

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 437 ns | 1.00x |
| v1_snapshot | 436 ns | 1.00x |
| v2_decomposed | 436 ns | 1.00x |
| v3_shapebound | 436 ns | 1.00x |
| v4_erased | 437 ns | 1.00x |
| v5_handletable | 441 ns | 1.01x |

- [20260619T182633Z-9d530d12](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20260619T182633Z-9d530d12/report.md)
- [20260702T102329Z-677b60f8](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20260702T102329Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_light_thrash_n16384/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_light_thrash_n262144

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| v0_blob | 6.51 us | 1.00x |
| v1_snapshot | 6.50 us | 1.00x |
| v2_decomposed | 6.52 us | 1.00x |
| v3_shapebound | 6.50 us | 1.00x |
| v4_erased | 6.51 us | 1.00x |
| v5_handletable | 6.56 us | 1.01x |

- [20260619T183144Z-9d530d12](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20260619T183144Z-9d530d12/report.md)
- [20260702T102742Z-677b60f8](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20260702T102742Z-677b60f8/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_light_thrash_n262144/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_seqd_n1048576

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 330.33 us | 1.00x |
| seq_snapshot | 402.42 us | 1.22x |

- [20260619T184658Z-9d530d12](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20260619T184658Z-9d530d12/report.md)
- [20260702T104547Z-a2410c71](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20260702T104547Z-a2410c71/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_seqd_n1048576/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_seqd_n16777216

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 1.52 ms | 1.00x |
| seq_snapshot | 3.70 ms | 2.43x |

- [20260619T202925Z-9d530d12](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20260619T202925Z-9d530d12/report.md)
- [20260702T121947Z-a2410c71](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20260702T121947Z-a2410c71/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_seqd_n16777216/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_seqd_n4194304

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 368.31 us | 1.00x |
| seq_snapshot | 759.55 us | 2.06x |

- [20260619T190539Z-9d530d12](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20260619T190539Z-9d530d12/report.md)
- [20260702T110508Z-a2410c71](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20260702T110508Z-a2410c71/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_seqd_n4194304/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## resource_storage/rsb_seqd_n65536

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T234625Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| seq_live | 328.49 us | 1.00x |
| seq_snapshot | 334.21 us | 1.02x |

- [20260619T184145Z-9d530d12](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20260619T184145Z-9d530d12/report.md)
- [20260702T104015Z-a2410c71](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20260702T104015Z-a2410c71/report.md)
- [20261116T234625Z-07a9e9a3](resource_storage/rsb_seqd_n65536/macos-aarch64-c8-bare-apple/20261116T234625Z-07a9e9a3/report.md)

## rotate_strategy/rotate_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T091951Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 65 ns | 1.00x |
| rot_manual_shifts | 79 ns | 1.22x |
| rot_variable_amount | 181 ns | 2.80x |

- [20261115T091951Z-266650fb-dirty](rotate_strategy/rotate_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T091951Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T013952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 72 ns | 1.01x |
| rot_manual_shifts | 71 ns | 1.00x |
| rot_variable_amount | 236 ns | 3.32x |

- [20260511T031129Z-cfbc15c](rotate_strategy/rotate_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T031129Z-cfbc15c/report.md)
- [20261114T232350Z-266650fb-dirty](rotate_strategy/rotate_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T232350Z-266650fb-dirty/report.md)
- [20261117T013952Z-07a9e9a3](rotate_strategy/rotate_strategy_n1024/macos-aarch64-c8-bare-apple/20261117T013952Z-07a9e9a3/report.md)

## rotate_strategy/rotate_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T092455Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 1.15 us | 1.00x |
| rot_manual_shifts | 1.15 us | 1.00x |
| rot_variable_amount | 3.43 us | 2.99x |

- [20261115T092455Z-266650fb-dirty](rotate_strategy/rotate_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T092455Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T013952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 1.26 us | 1.00x |
| rot_manual_shifts | 1.25 us | 1.00x |
| rot_variable_amount | 3.83 us | 3.05x |

- [20260511T031131Z-cfbc15c](rotate_strategy/rotate_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T031131Z-cfbc15c/report.md)
- [20261114T232355Z-266650fb-dirty](rotate_strategy/rotate_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T232355Z-266650fb-dirty/report.md)
- [20261117T013952Z-07a9e9a3](rotate_strategy/rotate_strategy_n16384/macos-aarch64-c8-bare-apple/20261117T013952Z-07a9e9a3/report.md)

## rotate_strategy/rotate_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T091717Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 21 ns | 1.00x |
| rot_manual_shifts | 25 ns | 1.22x |
| rot_variable_amount | 59 ns | 2.81x |

- [20261115T091717Z-266650fb-dirty](rotate_strategy/rotate_strategy_n256/linux-x86_64-c16-bare-intel/20261115T091717Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T013952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 6 ns | 1.00x |
| rot_manual_shifts | 6 ns | 1.00x |
| rot_variable_amount | 47 ns | 8.16x |

- [20260511T031128Z-cfbc15c](rotate_strategy/rotate_strategy_n256/macos-aarch64-c8-bare-apple/20260511T031128Z-cfbc15c/report.md)
- [20261114T232348Z-266650fb-dirty](rotate_strategy/rotate_strategy_n256/macos-aarch64-c8-bare-apple/20261114T232348Z-266650fb-dirty/report.md)
- [20261117T013952Z-07a9e9a3](rotate_strategy/rotate_strategy_n256/macos-aarch64-c8-bare-apple/20261117T013952Z-07a9e9a3/report.md)

## rotate_strategy/rotate_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T092224Z-266650fb`

| variant | median | vs best |
|---|---|---|
| rot_manual_shifts | 293 ns | 1.00x |
| rot_variable_amount | 862 ns | 2.94x |

- [20261115T092223Z-266650fb-dirty](rotate_strategy/rotate_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T092223Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T013952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| rot_intrinsic_const | 297 ns | 1.00x |
| rot_manual_shifts | 298 ns | 1.00x |
| rot_variable_amount | 953 ns | 3.21x |

- [20260511T031130Z-cfbc15c](rotate_strategy/rotate_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T031130Z-cfbc15c/report.md)
- [20261114T232352Z-266650fb-dirty](rotate_strategy/rotate_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T232352Z-266650fb-dirty/report.md)
- [20261117T013952Z-07a9e9a3](rotate_strategy/rotate_strategy_n4096/macos-aarch64-c8-bare-apple/20261117T013952Z-07a9e9a3/report.md)

## sat_arithmetic/sat_arithmetic_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T093033Z-266650fb`

| variant | median | vs best |
|---|---|---|
| sat_saturate | 504 ns | 1.71x |
| sat_wrap | 295 ns | 1.00x |

- [20261115T093032Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n1024/linux-x86_64-c16-bare-intel/20261115T093032Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014036Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 655 ns | 1.87x |
| sat_saturate | 655 ns | 1.87x |
| sat_wrap | 351 ns | 1.00x |

- [20260511T025001Z-15b5fe0](sat_arithmetic/sat_arithmetic_n1024/macos-aarch64-c8-bare-apple/20260511T025001Z-15b5fe0/report.md)
- [20261114T232433Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n1024/macos-aarch64-c8-bare-apple/20261114T232433Z-266650fb-dirty/report.md)
- [20261117T014036Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n1024/macos-aarch64-c8-bare-apple/20261117T014036Z-07a9e9a3/report.md)

## sat_arithmetic/sat_arithmetic_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T093537Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 7.80 us | 1.71x |
| sat_saturate | 7.80 us | 1.71x |
| sat_wrap | 4.57 us | 1.00x |

- [20261115T093537Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n16384/linux-x86_64-c16-bare-intel/20261115T093537Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014036Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 10.25 us | 1.94x |
| sat_saturate | 10.24 us | 1.94x |
| sat_wrap | 5.27 us | 1.00x |

- [20260511T025005Z-15b5fe0](sat_arithmetic/sat_arithmetic_n16384/macos-aarch64-c8-bare-apple/20260511T025005Z-15b5fe0/report.md)
- [20261114T232439Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n16384/macos-aarch64-c8-bare-apple/20261114T232439Z-266650fb-dirty/report.md)
- [20261117T014036Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n16384/macos-aarch64-c8-bare-apple/20261117T014036Z-07a9e9a3/report.md)

## sat_arithmetic/sat_arithmetic_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T092757Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 140 ns | 1.72x |
| sat_saturate | 140 ns | 1.72x |
| sat_wrap | 81 ns | 1.00x |

- [20261115T092757Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n256/linux-x86_64-c16-bare-intel/20261115T092757Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014036Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 175 ns | 1.70x |
| sat_saturate | 176 ns | 1.70x |
| sat_wrap | 103 ns | 1.00x |

- [20260511T025000Z-15b5fe0](sat_arithmetic/sat_arithmetic_n256/macos-aarch64-c8-bare-apple/20260511T025000Z-15b5fe0/report.md)
- [20261114T232431Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n256/macos-aarch64-c8-bare-apple/20261114T232431Z-266650fb-dirty/report.md)
- [20261117T014036Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n256/macos-aarch64-c8-bare-apple/20261117T014036Z-07a9e9a3/report.md)

## sat_arithmetic/sat_arithmetic_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T093304Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sat_checked | 1.97 us | 1.71x |
| sat_saturate | 1.97 us | 1.72x |
| sat_wrap | 1.15 us | 1.00x |

- [20261115T093304Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n4096/linux-x86_64-c16-bare-intel/20261115T093304Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014036Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sat_checked | 2.57 us | 1.93x |
| sat_saturate | 2.58 us | 1.93x |
| sat_wrap | 1.33 us | 1.00x |

- [20260511T025002Z-15b5fe0](sat_arithmetic/sat_arithmetic_n4096/macos-aarch64-c8-bare-apple/20260511T025002Z-15b5fe0/report.md)
- [20261114T232435Z-266650fb-dirty](sat_arithmetic/sat_arithmetic_n4096/macos-aarch64-c8-bare-apple/20261114T232435Z-266650fb-dirty/report.md)
- [20261117T014036Z-07a9e9a3](sat_arithmetic/sat_arithmetic_n4096/macos-aarch64-c8-bare-apple/20261117T014036Z-07a9e9a3/report.md)

## shift_amount_origin/shift_amount_origin_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T094114Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 19 ns | 1.00x |
| shr_loop_invariant | 20 ns | 1.03x |
| shr_per_iter | 94 ns | 4.90x |

- [20261115T094114Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n1024/linux-x86_64-c16-bare-intel/20261115T094114Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014120Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 5 ns | 1.00x |
| shr_loop_invariant | 5 ns | 1.08x |
| shr_per_iter | 62 ns | 12.46x |

- [20260511T033044Z-4f44ee4](shift_amount_origin/shift_amount_origin_n1024/macos-aarch64-c8-bare-apple/20260511T033044Z-4f44ee4/report.md)
- [20261114T232517Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n1024/macos-aarch64-c8-bare-apple/20261114T232517Z-266650fb-dirty/report.md)
- [20261117T014120Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n1024/macos-aarch64-c8-bare-apple/20261117T014120Z-07a9e9a3/report.md)

## shift_amount_origin/shift_amount_origin_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T094618Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 131 ns | 1.00x |
| shr_loop_invariant | 203 ns | 1.56x |
| shr_per_iter | 1.33 us | 10.22x |

- [20261115T094618Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n16384/linux-x86_64-c16-bare-intel/20261115T094618Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014120Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 191 ns | 1.00x |
| shr_loop_invariant | 192 ns | 1.01x |
| shr_per_iter | 667 ns | 3.50x |

- [20260511T033046Z-4f44ee4](shift_amount_origin/shift_amount_origin_n16384/macos-aarch64-c8-bare-apple/20260511T033046Z-4f44ee4/report.md)
- [20261114T232522Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n16384/macos-aarch64-c8-bare-apple/20261114T232522Z-266650fb-dirty/report.md)
- [20261117T014120Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n16384/macos-aarch64-c8-bare-apple/20261117T014120Z-07a9e9a3/report.md)

## shift_amount_origin/shift_amount_origin_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T093839Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 12 ns | 1.00x |
| shr_loop_invariant | 13 ns | 1.06x |
| shr_per_iter | 26 ns | 2.14x |

- [20261115T093839Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n256/linux-x86_64-c16-bare-intel/20261115T093839Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014120Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 1 ns | 1.00x |
| shr_loop_invariant | 2 ns | 3.12x |
| shr_per_iter | 7 ns | 8.38x |

- [20260511T033043Z-4f44ee4](shift_amount_origin/shift_amount_origin_n256/macos-aarch64-c8-bare-apple/20260511T033043Z-4f44ee4/report.md)
- [20261114T232515Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n256/macos-aarch64-c8-bare-apple/20261114T232515Z-266650fb-dirty/report.md)
- [20261117T014120Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n256/macos-aarch64-c8-bare-apple/20261117T014120Z-07a9e9a3/report.md)

## shift_amount_origin/shift_amount_origin_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T094346Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| shr_const | 50 ns | 1.00x |
| shr_loop_invariant | 61 ns | 1.23x |
| shr_per_iter | 342 ns | 6.89x |

- [20261115T094346Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n4096/linux-x86_64-c16-bare-intel/20261115T094346Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T014120Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| shr_const | 66 ns | 1.00x |
| shr_loop_invariant | 68 ns | 1.03x |
| shr_per_iter | 184 ns | 2.79x |

- [20260511T033045Z-4f44ee4](shift_amount_origin/shift_amount_origin_n4096/macos-aarch64-c8-bare-apple/20260511T033045Z-4f44ee4/report.md)
- [20261114T232519Z-266650fb-dirty](shift_amount_origin/shift_amount_origin_n4096/macos-aarch64-c8-bare-apple/20261114T232519Z-266650fb-dirty/report.md)
- [20261117T014120Z-07a9e9a3](shift_amount_origin/shift_amount_origin_n4096/macos-aarch64-c8-bare-apple/20261117T014120Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w1_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091914Z-ed2bf53-dirty](single-trunk-convergence/append_w1_big_n16777216/linux-x86_64-c4-container-intel/20261113T091914Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.44 ms | 1.07x |
| head_tail_dyn | 2.36 ms | 1.03x |
| kernel_nway | 2.29 ms | 1.00x |
| kernel_seq | 2.36 ms | 1.03x |
| nway | 11.15 ms | 4.87x |
| sequential | 11.71 ms | 5.11x |
| std_parallel | 2.37 ms | 1.04x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w1_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w1_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091545Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n1048576/linux-x86_64-c4-container-intel/20261113T091545Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 110.70 us | 1.00x |
| head_tail_dyn | 111.51 us | 1.01x |
| kernel_nway | 110.70 us | 1.00x |
| kernel_seq | 110.71 us | 1.00x |
| nway | 716.44 us | 6.47x |
| sequential | 744.22 us | 6.72x |
| std_parallel | 130.26 us | 1.18x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w1_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w1_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091048Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n4096/linux-x86_64-c4-container-intel/20261113T091048Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.98 us | 1.04x |
| head_tail_dyn | 3.25 us | 1.14x |
| kernel_nway | 2.86 us | 1.00x |
| kernel_seq | 3.18 us | 1.11x |
| nway | 15.14 us | 5.29x |
| sequential | 13.13 us | 4.59x |
| std_parallel | 17.74 us | 6.20x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w1_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w1_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T091304Z-ed2bf53-dirty](single-trunk-convergence/append_w1_n65536/linux-x86_64-c4-container-intel/20261113T091304Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.64 us | 1.01x |
| head_tail_dyn | 9.62 us | 1.01x |
| kernel_nway | 9.55 us | 1.00x |
| kernel_seq | 9.89 us | 1.04x |
| nway | 59.05 us | 6.18x |
| sequential | 55.01 us | 5.76x |
| std_parallel | 24.45 us | 2.56x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w1_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w2_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T100114Z-74174bb](single-trunk-convergence/append_w2_big_n16777216/linux-x86_64-c4-container-intel/20261113T100114Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.25 ms | 1.00x |
| head_tail_dyn | 2.28 ms | 1.01x |
| kernel_nway | 2.36 ms | 1.04x |
| kernel_seq | 2.30 ms | 1.02x |
| nway | 5.79 ms | 2.57x |
| sequential | 11.57 ms | 5.13x |
| std_parallel | 2.32 ms | 1.03x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w2_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w2_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095753Z-74174bb](single-trunk-convergence/append_w2_n1048576/linux-x86_64-c4-container-intel/20261113T095753Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 112.01 us | 1.37x |
| head_tail_dyn | 112.73 us | 1.37x |
| kernel_nway | 82.04 us | 1.00x |
| kernel_seq | 110.67 us | 1.35x |
| nway | 399.71 us | 4.87x |
| sequential | 746.66 us | 9.10x |
| std_parallel | 90.28 us | 1.10x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w2_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w2_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095257Z-74174bb](single-trunk-convergence/append_w2_n4096/linux-x86_64-c4-container-intel/20261113T095257Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.98 us | 1.06x |
| head_tail_dyn | 2.80 us | 1.00x |
| kernel_nway | 4.17 us | 1.49x |
| kernel_seq | 2.95 us | 1.05x |
| nway | 21.48 us | 7.67x |
| sequential | 13.06 us | 4.67x |
| std_parallel | 29.07 us | 10.38x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w2_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w2_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095513Z-74174bb](single-trunk-convergence/append_w2_n65536/linux-x86_64-c4-container-intel/20261113T095513Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.63 us | 1.00x |
| head_tail_dyn | 9.67 us | 1.00x |
| kernel_nway | 10.12 us | 1.05x |
| kernel_seq | 9.65 us | 1.00x |
| nway | 64.67 us | 6.71x |
| sequential | 55.92 us | 5.81x |
| std_parallel | 35.41 us | 3.68x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w2_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w4_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T111219Z-f12bdc4](single-trunk-convergence/append_w4_big_n16777216/linux-x86_64-c4-container-intel/20261113T111219Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.43 ms | 1.07x |
| head_tail_dyn | 2.27 ms | 1.00x |
| kernel_nway | 2.43 ms | 1.07x |
| kernel_seq | 2.30 ms | 1.01x |
| nway | 3.26 ms | 1.44x |
| sequential | 11.67 ms | 5.15x |
| std_parallel | 2.48 ms | 1.09x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w4_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w4_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110856Z-f12bdc4](single-trunk-convergence/append_w4_n1048576/linux-x86_64-c4-container-intel/20261113T110856Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 110.62 us | 1.61x |
| head_tail_dyn | 110.47 us | 1.60x |
| kernel_nway | 68.87 us | 1.00x |
| kernel_seq | 110.37 us | 1.60x |
| nway | 241.39 us | 3.50x |
| sequential | 746.93 us | 10.85x |
| std_parallel | 89.67 us | 1.30x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w4_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w4_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110400Z-f12bdc4](single-trunk-convergence/append_w4_n4096/linux-x86_64-c4-container-intel/20261113T110400Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.83 us | 1.02x |
| head_tail_dyn | 2.79 us | 1.00x |
| kernel_nway | 11.47 us | 4.12x |
| kernel_seq | 2.91 us | 1.04x |
| nway | 40.01 us | 14.35x |
| sequential | 12.79 us | 4.59x |
| std_parallel | 48.07 us | 17.24x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w4_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w4_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110615Z-f12bdc4](single-trunk-convergence/append_w4_n65536/linux-x86_64-c4-container-intel/20261113T110615Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.57 us | 1.00x |
| head_tail_dyn | 9.55 us | 1.00x |
| kernel_nway | 20.79 us | 2.18x |
| kernel_seq | 9.59 us | 1.00x |
| nway | 60.61 us | 6.35x |
| sequential | 55.55 us | 5.82x |
| std_parallel | 53.14 us | 5.57x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w4_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w8_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T120018Z-d11e290-dirty](single-trunk-convergence/append_w8_big_n16777216/linux-x86_64-c4-container-intel/20261113T120018Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.27 ms | 1.00x |
| head_tail_dyn | 2.29 ms | 1.01x |
| kernel_nway | 2.92 ms | 1.29x |
| kernel_seq | 2.33 ms | 1.03x |
| nway | 2.92 ms | 1.29x |
| sequential | 11.66 ms | 5.15x |
| std_parallel | 2.93 ms | 1.30x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w8_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w8_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115657Z-d11e290-dirty](single-trunk-convergence/append_w8_n1048576/linux-x86_64-c4-container-intel/20261113T115657Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 110.90 us | 1.56x |
| head_tail_dyn | 111.34 us | 1.57x |
| kernel_nway | 71.00 us | 1.00x |
| kernel_seq | 111.00 us | 1.56x |
| nway | 218.20 us | 3.07x |
| sequential | 745.31 us | 10.50x |
| std_parallel | 103.76 us | 1.46x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w8_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w8_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115200Z-d11e290-dirty](single-trunk-convergence/append_w8_n4096/linux-x86_64-c4-container-intel/20261113T115200Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.83 us | 1.01x |
| head_tail_dyn | 2.81 us | 1.01x |
| kernel_nway | 22.32 us | 8.02x |
| kernel_seq | 2.78 us | 1.00x |
| nway | 53.33 us | 19.16x |
| sequential | 13.24 us | 4.76x |
| std_parallel | 78.34 us | 28.15x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w8_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/append_w8_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T115416Z-d11e290-dirty](single-trunk-convergence/append_w8_n65536/linux-x86_64-c4-container-intel/20261113T115416Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 9.64 us | 1.01x |
| head_tail_dyn | 9.59 us | 1.00x |
| kernel_nway | 26.85 us | 2.81x |
| kernel_seq | 9.55 us | 1.00x |
| nway | 63.50 us | 6.65x |
| sequential | 55.89 us | 5.85x |
| std_parallel | 81.64 us | 8.55x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/append_w8_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w1_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090834Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_big_n1048576/linux-x86_64-c4-container-intel/20261113T090834Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 744.89 us | 1.00x |
| head_tail_dyn | 748.17 us | 1.00x |
| kernel_nway | 744.77 us | 1.00x |
| kernel_seq | 745.17 us | 1.00x |
| nway | 768.79 us | 1.03x |
| sequential | 753.33 us | 1.01x |
| std_parallel | 761.45 us | 1.02x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w1_big_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w1_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090400Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_n4096/linux-x86_64-c4-container-intel/20261113T090400Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 5.60 us | 1.03x |
| head_tail_dyn | 5.51 us | 1.01x |
| kernel_nway | 5.51 us | 1.01x |
| kernel_seq | 5.45 us | 1.00x |
| nway | 15.31 us | 2.81x |
| sequential | 12.85 us | 2.36x |
| std_parallel | 23.70 us | 4.35x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w1_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w1_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090616Z-ed2bf53-dirty](single-trunk-convergence/fold_heavy_w1_n65536/linux-x86_64-c4-container-intel/20261113T090616Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 52.69 us | 1.01x |
| head_tail_dyn | 52.08 us | 1.00x |
| kernel_nway | 52.11 us | 1.00x |
| kernel_seq | 52.14 us | 1.00x |
| nway | 61.67 us | 1.18x |
| sequential | 56.07 us | 1.08x |
| std_parallel | 69.27 us | 1.33x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w1_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w2_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T095043Z-74174bb](single-trunk-convergence/fold_heavy_w2_big_n1048576/linux-x86_64-c4-container-intel/20261113T095043Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 449.79 us | 1.09x |
| head_tail_dyn | 439.18 us | 1.06x |
| kernel_nway | 412.93 us | 1.00x |
| kernel_seq | 754.96 us | 1.83x |
| nway | 418.59 us | 1.01x |
| sequential | 752.21 us | 1.82x |
| std_parallel | 437.34 us | 1.06x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w2_big_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w2_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094611Z-74174bb](single-trunk-convergence/fold_heavy_w2_n4096/linux-x86_64-c4-container-intel/20261113T094611Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 6.56 us | 1.16x |
| head_tail_dyn | 6.64 us | 1.18x |
| kernel_nway | 6.51 us | 1.16x |
| kernel_seq | 5.63 us | 1.00x |
| nway | 21.29 us | 3.78x |
| sequential | 12.90 us | 2.29x |
| std_parallel | 33.67 us | 5.98x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w2_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w2_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094827Z-74174bb](single-trunk-convergence/fold_heavy_w2_n65536/linux-x86_64-c4-container-intel/20261113T094827Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 53.72 us | 1.59x |
| head_tail_dyn | 33.88 us | 1.00x |
| kernel_nway | 51.86 us | 1.53x |
| kernel_seq | 51.94 us | 1.53x |
| nway | 48.72 us | 1.44x |
| sequential | 56.26 us | 1.66x |
| std_parallel | 58.60 us | 1.73x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w2_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w4_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T110146Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_big_n1048576/linux-x86_64-c4-container-intel/20261113T110146Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 436.88 us | 1.80x |
| head_tail_dyn | 440.22 us | 1.81x |
| kernel_nway | 242.93 us | 1.00x |
| kernel_seq | 754.96 us | 3.11x |
| nway | 249.50 us | 1.03x |
| sequential | 750.80 us | 3.09x |
| std_parallel | 287.54 us | 1.18x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w4_big_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w4_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105714Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_n4096/linux-x86_64-c4-container-intel/20261113T105714Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 7.05 us | 1.16x |
| head_tail_dyn | 6.55 us | 1.08x |
| kernel_nway | 21.29 us | 3.50x |
| kernel_seq | 6.08 us | 1.00x |
| nway | 39.38 us | 6.47x |
| sequential | 12.98 us | 2.13x |
| std_parallel | 62.36 us | 10.25x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w4_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w4_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105929Z-f12bdc4](single-trunk-convergence/fold_heavy_w4_n65536/linux-x86_64-c4-container-intel/20261113T105929Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 54.16 us | 1.60x |
| head_tail_dyn | 33.87 us | 1.00x |
| kernel_nway | 47.79 us | 1.41x |
| kernel_seq | 52.02 us | 1.54x |
| nway | 59.23 us | 1.75x |
| sequential | 56.73 us | 1.68x |
| std_parallel | 71.35 us | 2.11x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w4_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w8_big_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114945Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_big_n1048576/linux-x86_64-c4-container-intel/20261113T114945Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 436.54 us | 1.98x |
| head_tail_dyn | 441.47 us | 2.00x |
| kernel_nway | 220.79 us | 1.00x |
| kernel_seq | 745.73 us | 3.38x |
| nway | 231.58 us | 1.05x |
| sequential | 760.34 us | 3.44x |
| std_parallel | 260.33 us | 1.18x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w8_big_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w8_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114513Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_n4096/linux-x86_64-c4-container-intel/20261113T114513Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 7.04 us | 1.28x |
| head_tail_dyn | 6.38 us | 1.16x |
| kernel_nway | 23.41 us | 4.25x |
| kernel_seq | 5.50 us | 1.00x |
| nway | 51.62 us | 9.38x |
| sequential | 12.84 us | 2.33x |
| std_parallel | 86.69 us | 15.75x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w8_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_heavy_w8_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114729Z-d11e290-dirty](single-trunk-convergence/fold_heavy_w8_n65536/linux-x86_64-c4-container-intel/20261113T114729Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 53.43 us | 1.57x |
| head_tail_dyn | 34.12 us | 1.00x |
| kernel_nway | 49.36 us | 1.45x |
| kernel_seq | 52.21 us | 1.53x |
| nway | 68.59 us | 2.01x |
| sequential | 56.43 us | 1.65x |
| std_parallel | 92.69 us | 2.72x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_heavy_w8_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w1_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T090146Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_big_n134217728/linux-x86_64-c4-container-intel/20261113T090146Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 12.09 ms | 1.05x |
| head_tail_dyn | 11.67 ms | 1.01x |
| kernel_nway | 12.10 ms | 1.05x |
| kernel_seq | 11.55 ms | 1.00x |
| nway | 11.61 ms | 1.00x |
| sequential | 12.18 ms | 1.05x |
| std_parallel | 11.88 ms | 1.03x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w1_big_n134217728/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w1_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085554Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_big_n16777216/linux-x86_64-c4-container-intel/20261113T085554Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 1.44 ms | 1.01x |
| head_tail_dyn | 1.43 ms | 1.00x |
| kernel_nway | 1.43 ms | 1.00x |
| kernel_seq | 1.43 ms | 1.00x |
| nway | 1.50 ms | 1.05x |
| sequential | 1.43 ms | 1.01x |
| std_parallel | 1.52 ms | 1.06x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w1_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w1_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085316Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n1048576/linux-x86_64-c4-container-intel/20261113T085316Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 92.11 us | 1.00x |
| head_tail_dyn | 92.60 us | 1.01x |
| kernel_nway | 96.91 us | 1.05x |
| kernel_seq | 94.39 us | 1.02x |
| nway | 103.06 us | 1.12x |
| sequential | 97.26 us | 1.06x |
| std_parallel | 109.95 us | 1.19x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w1_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w1_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084844Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n4096/linux-x86_64-c4-container-intel/20261113T084844Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.79 us | 1.03x |
| head_tail_dyn | 2.84 us | 1.05x |
| kernel_nway | 2.72 us | 1.00x |
| kernel_seq | 2.79 us | 1.03x |
| nway | 12.42 us | 4.57x |
| sequential | 10.28 us | 3.78x |
| std_parallel | 20.13 us | 7.41x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w1_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w1_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T085057Z-ed2bf53-dirty](single-trunk-convergence/fold_light_w1_n65536/linux-x86_64-c4-container-intel/20261113T085057Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.35 us | 1.00x |
| head_tail_dyn | 8.63 us | 1.03x |
| kernel_nway | 8.57 us | 1.03x |
| kernel_seq | 8.47 us | 1.01x |
| nway | 18.20 us | 2.18x |
| sequential | 15.95 us | 1.91x |
| std_parallel | 27.15 us | 3.25x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w1_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w2_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T094358Z-74174bb](single-trunk-convergence/fold_light_w2_big_n134217728/linux-x86_64-c4-container-intel/20261113T094358Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.68 ms | 1.00x |
| head_tail_dyn | 15.47 ms | 1.79x |
| kernel_nway | 8.80 ms | 1.02x |
| kernel_seq | 12.17 ms | 1.41x |
| nway | 9.10 ms | 1.05x |
| sequential | 11.55 ms | 1.33x |
| std_parallel | 8.66 ms | 1.00x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w2_big_n134217728/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w2_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093857Z-74174bb](single-trunk-convergence/fold_light_w2_big_n16777216/linux-x86_64-c4-container-intel/20261113T093857Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 1.09 ms | 1.00x |
| head_tail_dyn | 1.98 ms | 1.82x |
| kernel_nway | 1.10 ms | 1.01x |
| kernel_seq | 1.45 ms | 1.33x |
| nway | 1.11 ms | 1.02x |
| sequential | 1.44 ms | 1.32x |
| std_parallel | 1.32 ms | 1.20x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w2_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w2_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093622Z-74174bb](single-trunk-convergence/fold_light_w2_n1048576/linux-x86_64-c4-container-intel/20261113T093622Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 57.22 us | 1.00x |
| head_tail_dyn | 136.51 us | 2.39x |
| kernel_nway | 74.93 us | 1.31x |
| kernel_seq | 94.27 us | 1.65x |
| nway | 79.21 us | 1.38x |
| sequential | 98.12 us | 1.71x |
| std_parallel | 76.99 us | 1.35x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w2_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w2_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093149Z-74174bb](single-trunk-convergence/fold_light_w2_n4096/linux-x86_64-c4-container-intel/20261113T093149Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.48 us | 1.60x |
| head_tail_dyn | 4.43 us | 1.58x |
| kernel_nway | 4.40 us | 1.57x |
| kernel_seq | 2.80 us | 1.00x |
| nway | 18.16 us | 6.49x |
| sequential | 9.99 us | 3.57x |
| std_parallel | 35.25 us | 12.59x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w2_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w2_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T093403Z-74174bb](single-trunk-convergence/fold_light_w2_n65536/linux-x86_64-c4-container-intel/20261113T093403Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.56 us | 1.00x |
| head_tail_dyn | 12.16 us | 1.43x |
| kernel_nway | 8.80 us | 1.03x |
| kernel_seq | 8.53 us | 1.00x |
| nway | 22.16 us | 2.60x |
| sequential | 15.42 us | 1.81x |
| std_parallel | 37.92 us | 4.45x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w2_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w4_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T105500Z-f12bdc4](single-trunk-convergence/fold_light_w4_big_n134217728/linux-x86_64-c4-container-intel/20261113T105500Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.81 ms | 1.00x |
| head_tail_dyn | 15.82 ms | 1.80x |
| kernel_nway | 8.94 ms | 1.01x |
| kernel_seq | 11.78 ms | 1.34x |
| nway | 8.89 ms | 1.01x |
| sequential | 12.46 ms | 1.41x |
| std_parallel | 8.89 ms | 1.01x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w4_big_n134217728/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w4_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104952Z-f12bdc4](single-trunk-convergence/fold_light_w4_big_n16777216/linux-x86_64-c4-container-intel/20261113T104952Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 1.10 ms | 1.00x |
| head_tail_dyn | 1.99 ms | 1.81x |
| kernel_nway | 1.10 ms | 1.00x |
| kernel_seq | 1.45 ms | 1.32x |
| nway | 1.29 ms | 1.17x |
| sequential | 1.45 ms | 1.32x |
| std_parallel | 1.16 ms | 1.06x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w4_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w4_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104719Z-f12bdc4](single-trunk-convergence/fold_light_w4_n1048576/linux-x86_64-c4-container-intel/20261113T104719Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 71.89 us | 1.29x |
| head_tail_dyn | 137.69 us | 2.47x |
| kernel_nway | 55.83 us | 1.00x |
| kernel_seq | 94.73 us | 1.70x |
| nway | 73.86 us | 1.32x |
| sequential | 100.38 us | 1.80x |
| std_parallel | 78.64 us | 1.41x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w4_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w4_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104247Z-f12bdc4](single-trunk-convergence/fold_light_w4_n4096/linux-x86_64-c4-container-intel/20261113T104247Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.42 us | 1.49x |
| head_tail_dyn | 4.33 us | 1.46x |
| kernel_nway | 11.30 us | 3.81x |
| kernel_seq | 2.96 us | 1.00x |
| nway | 34.09 us | 11.51x |
| sequential | 10.16 us | 3.43x |
| std_parallel | 56.08 us | 18.93x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w4_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w4_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104501Z-f12bdc4](single-trunk-convergence/fold_light_w4_n65536/linux-x86_64-c4-container-intel/20261113T104501Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.74 us | 1.01x |
| head_tail_dyn | 12.30 us | 1.42x |
| kernel_nway | 21.04 us | 2.43x |
| kernel_seq | 8.65 us | 1.00x |
| nway | 40.09 us | 4.63x |
| sequential | 15.57 us | 1.80x |
| std_parallel | 57.33 us | 6.63x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w4_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w8_big_n134217728

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T114259Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_big_n134217728/linux-x86_64-c4-container-intel/20261113T114259Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.62 ms | 1.00x |
| head_tail_dyn | 15.69 ms | 1.82x |
| kernel_nway | 9.20 ms | 1.07x |
| kernel_seq | 11.56 ms | 1.34x |
| nway | 9.55 ms | 1.11x |
| sequential | 11.73 ms | 1.36x |
| std_parallel | 9.44 ms | 1.09x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w8_big_n134217728/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w8_big_n16777216

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113804Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_big_n16777216/linux-x86_64-c4-container-intel/20261113T113804Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 1.10 ms | 1.00x |
| head_tail_dyn | 1.98 ms | 1.80x |
| kernel_nway | 1.17 ms | 1.06x |
| kernel_seq | 1.45 ms | 1.32x |
| nway | 1.17 ms | 1.07x |
| sequential | 1.45 ms | 1.32x |
| std_parallel | 1.23 ms | 1.12x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w8_big_n16777216/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w8_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113530Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n1048576/linux-x86_64-c4-container-intel/20261113T113530Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 72.01 us | 1.29x |
| head_tail_dyn | 137.16 us | 2.46x |
| kernel_nway | 55.76 us | 1.00x |
| kernel_seq | 95.95 us | 1.72x |
| nway | 72.69 us | 1.30x |
| sequential | 100.45 us | 1.80x |
| std_parallel | 101.14 us | 1.81x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w8_n1048576/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w8_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113057Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n4096/linux-x86_64-c4-container-intel/20261113T113057Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.60 us | 1.61x |
| head_tail_dyn | 4.67 us | 1.64x |
| kernel_nway | 17.51 us | 6.15x |
| kernel_seq | 2.85 us | 1.00x |
| nway | 49.96 us | 17.53x |
| sequential | 10.56 us | 3.70x |
| std_parallel | 84.83 us | 29.76x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w8_n4096/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/fold_light_w8_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T113312Z-d11e290-dirty](single-trunk-convergence/fold_light_w8_n65536/linux-x86_64-c4-container-intel/20261113T113312Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 8.65 us | 1.00x |
| head_tail_dyn | 12.24 us | 1.42x |
| kernel_nway | 24.74 us | 2.86x |
| kernel_seq | 8.65 us | 1.00x |
| nway | 53.98 us | 6.24x |
| sequential | 16.29 us | 1.88x |
| std_parallel | 90.41 us | 10.45x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/fold_light_w8_n65536/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w1_n1024

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083547Z-e486b5f-dirty](single-trunk-convergence/short_w1_n1024/linux-x86_64-c2-container-intel/20261113T083547Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084629Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n1024/linux-x86_64-c4-container-intel/20261113T084629Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.88 us | 1.10x |
| head_tail_dyn | 2.63 us | 1.00x |
| kernel_nway | 2.63 us | 1.00x |
| kernel_seq | 2.66 us | 1.01x |
| nway | 12.55 us | 4.78x |
| sequential | 10.10 us | 3.85x |
| std_parallel | 19.98 us | 7.61x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w1_n1024/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w1_n16

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T082904Z-e486b5f-dirty](single-trunk-convergence/short_w1_n16/linux-x86_64-c2-container-intel/20261113T082904Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T083944Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n16/linux-x86_64-c4-container-intel/20261113T083944Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.55 us | 1.02x |
| head_tail_dyn | 2.56 us | 1.02x |
| kernel_nway | 2.50 us | 1.00x |
| kernel_seq | 2.54 us | 1.02x |
| nway | 12.19 us | 4.88x |
| sequential | 9.97 us | 3.99x |
| std_parallel | 18.64 us | 7.46x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w1_n16/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w1_n256

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083333Z-e486b5f-dirty](single-trunk-convergence/short_w1_n256/linux-x86_64-c2-container-intel/20261113T083333Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084415Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n256/linux-x86_64-c4-container-intel/20261113T084415Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.61 us | 1.03x |
| head_tail_dyn | 2.62 us | 1.03x |
| kernel_nway | 2.55 us | 1.01x |
| kernel_seq | 2.53 us | 1.00x |
| nway | 12.17 us | 4.80x |
| sequential | 9.97 us | 3.93x |
| std_parallel | 20.47 us | 8.08x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w1_n256/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w1_n64

### linux-x86_64-c2-container-intel, 1 run(s)

- [20261113T083120Z-e486b5f-dirty](single-trunk-convergence/short_w1_n64/linux-x86_64-c2-container-intel/20261113T083120Z-e486b5f-dirty/report.md)

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T084201Z-ed2bf53-dirty](single-trunk-convergence/short_w1_n64/linux-x86_64-c4-container-intel/20261113T084201Z-ed2bf53-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 2.53 us | 1.02x |
| head_tail_dyn | 2.48 us | 1.00x |
| kernel_nway | 2.52 us | 1.01x |
| kernel_seq | 2.49 us | 1.00x |
| nway | 12.07 us | 4.86x |
| sequential | 10.34 us | 4.17x |
| std_parallel | 19.05 us | 7.67x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w1_n64/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w2_n1024

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092935Z-74174bb](single-trunk-convergence/short_w2_n1024/linux-x86_64-c4-container-intel/20261113T092935Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.13 us | 1.61x |
| head_tail_dyn | 4.19 us | 1.63x |
| kernel_nway | 4.09 us | 1.59x |
| kernel_seq | 2.57 us | 1.00x |
| nway | 17.70 us | 6.87x |
| sequential | 10.07 us | 3.91x |
| std_parallel | 32.04 us | 12.45x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w2_n1024/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w2_n16

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092249Z-74174bb](single-trunk-convergence/short_w2_n16/linux-x86_64-c4-container-intel/20261113T092249Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.23 us | 1.55x |
| head_tail_dyn | 4.11 us | 1.51x |
| kernel_nway | 4.49 us | 1.65x |
| kernel_seq | 2.72 us | 1.00x |
| nway | 18.09 us | 6.64x |
| sequential | 10.23 us | 3.76x |
| std_parallel | 30.36 us | 11.15x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w2_n16/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w2_n256

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092721Z-74174bb](single-trunk-convergence/short_w2_n256/linux-x86_64-c4-container-intel/20261113T092721Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.45 us | 1.68x |
| head_tail_dyn | 4.08 us | 1.54x |
| kernel_nway | 4.18 us | 1.58x |
| kernel_seq | 2.65 us | 1.00x |
| nway | 17.63 us | 6.66x |
| sequential | 9.99 us | 3.78x |
| std_parallel | 30.93 us | 11.69x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w2_n256/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w2_n64

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T092506Z-74174bb](single-trunk-convergence/short_w2_n64/linux-x86_64-c4-container-intel/20261113T092506Z-74174bb/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.05 us | 1.58x |
| head_tail_dyn | 4.23 us | 1.65x |
| kernel_nway | 4.18 us | 1.63x |
| kernel_seq | 2.56 us | 1.00x |
| nway | 19.33 us | 7.55x |
| sequential | 10.17 us | 3.97x |
| std_parallel | 31.61 us | 12.35x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w2_n64/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w4_n1024

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T104033Z-f12bdc4](single-trunk-convergence/short_w4_n1024/linux-x86_64-c4-container-intel/20261113T104033Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.15 us | 1.60x |
| head_tail_dyn | 4.14 us | 1.60x |
| kernel_nway | 9.60 us | 3.71x |
| kernel_seq | 2.59 us | 1.00x |
| nway | 33.03 us | 12.76x |
| sequential | 10.19 us | 3.94x |
| std_parallel | 54.12 us | 20.91x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w4_n1024/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w4_n16

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103346Z-f12bdc4](single-trunk-convergence/short_w4_n16/linux-x86_64-c4-container-intel/20261113T103346Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.21 us | 1.62x |
| head_tail_dyn | 4.13 us | 1.58x |
| kernel_nway | 9.90 us | 3.80x |
| kernel_seq | 2.60 us | 1.00x |
| nway | 33.55 us | 12.89x |
| sequential | 10.26 us | 3.94x |
| std_parallel | 55.09 us | 21.16x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w4_n16/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w4_n256

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103818Z-f12bdc4](single-trunk-convergence/short_w4_n256/linux-x86_64-c4-container-intel/20261113T103818Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.14 us | 1.65x |
| head_tail_dyn | 4.13 us | 1.65x |
| kernel_nway | 9.17 us | 3.66x |
| kernel_seq | 2.51 us | 1.00x |
| nway | 32.80 us | 13.09x |
| sequential | 10.16 us | 4.05x |
| std_parallel | 54.82 us | 21.87x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w4_n256/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w4_n64

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T103604Z-f12bdc4](single-trunk-convergence/short_w4_n64/linux-x86_64-c4-container-intel/20261113T103604Z-f12bdc4/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.25 us | 1.70x |
| head_tail_dyn | 4.13 us | 1.65x |
| kernel_nway | 9.30 us | 3.73x |
| kernel_seq | 2.50 us | 1.00x |
| nway | 33.41 us | 13.38x |
| sequential | 10.18 us | 4.08x |
| std_parallel | 53.57 us | 21.46x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w4_n64/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w8_n1024

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112843Z-d11e290-dirty](single-trunk-convergence/short_w8_n1024/linux-x86_64-c4-container-intel/20261113T112843Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.15 us | 1.57x |
| head_tail_dyn | 4.22 us | 1.59x |
| kernel_nway | 15.31 us | 5.77x |
| kernel_seq | 2.65 us | 1.00x |
| nway | 50.50 us | 19.04x |
| sequential | 9.99 us | 3.77x |
| std_parallel | 84.54 us | 31.88x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w8_n1024/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w8_n16

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112154Z-d11e290-dirty](single-trunk-convergence/short_w8_n16/linux-x86_64-c4-container-intel/20261113T112154Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.42 us | 1.78x |
| head_tail_dyn | 4.20 us | 1.69x |
| kernel_nway | 14.71 us | 5.93x |
| kernel_seq | 2.48 us | 1.00x |
| nway | 48.52 us | 19.56x |
| sequential | 10.17 us | 4.10x |
| std_parallel | 82.03 us | 33.07x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w8_n16/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w8_n256

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112627Z-d11e290-dirty](single-trunk-convergence/short_w8_n256/linux-x86_64-c4-container-intel/20261113T112627Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.30 us | 1.68x |
| head_tail_dyn | 4.11 us | 1.61x |
| kernel_nway | 15.48 us | 6.06x |
| kernel_seq | 2.56 us | 1.00x |
| nway | 51.06 us | 19.97x |
| sequential | 10.09 us | 3.95x |
| std_parallel | 82.42 us | 32.24x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w8_n256/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## single-trunk-convergence/short_w8_n64

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T112412Z-d11e290-dirty](single-trunk-convergence/short_w8_n64/linux-x86_64-c4-container-intel/20261113T112412Z-d11e290-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261117T014224Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| head_tail | 4.08 us | 1.64x |
| head_tail_dyn | 4.18 us | 1.68x |
| kernel_nway | 15.33 us | 6.15x |
| kernel_seq | 2.49 us | 1.00x |
| nway | 49.80 us | 19.98x |
| sequential | 10.26 us | 4.12x |
| std_parallel | 85.15 us | 34.15x |

- [20261117T014224Z-07a9e9a3](single-trunk-convergence/short_w8_n64/macos-aarch64-c8-bare-apple/20261117T014224Z-07a9e9a3/report.md)

## swap_band/swap_band_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T095229Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 9.90 us | 1.00x |
| sb_plan | 12.61 us | 1.27x |
| sb_value | 235.25 us | 23.77x |

- [20261115T095229Z-266650fb-dirty](swap_band/swap_band_n1024/linux-x86_64-c16-bare-intel/20261115T095229Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T015426Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sb_none | 9.97 us | 1.00x |
| sb_plan | 12.26 us | 1.23x |
| sb_value | 156.61 us | 15.70x |

- [20260719T185027Z-8c3a949d-dirty](swap_band/swap_band_n1024/macos-aarch64-c8-bare-apple/20260719T185027Z-8c3a949d-dirty/report.md)
- [20261114T232820Z-266650fb-dirty](swap_band/swap_band_n1024/macos-aarch64-c8-bare-apple/20261114T232820Z-266650fb-dirty/report.md)
- [20261117T015426Z-07a9e9a3](swap_band/swap_band_n1024/macos-aarch64-c8-bare-apple/20261117T015426Z-07a9e9a3/report.md)

## swap_band/swap_band_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T095023Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 7.90 us | 1.00x |
| sb_plan | 10.11 us | 1.28x |
| sb_value | 16.20 us | 2.05x |

- [20261115T095023Z-266650fb-dirty](swap_band/swap_band_n64/linux-x86_64-c16-bare-intel/20261115T095023Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261117T015426Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sb_none | 10.02 us | 1.00x |
| sb_plan | 12.43 us | 1.24x |
| sb_value | 17.77 us | 1.77x |

- [20260719T185015Z-8c3a949d-dirty](swap_band/swap_band_n64/macos-aarch64-c8-bare-apple/20260719T185015Z-8c3a949d-dirty/report.md)
- [20261114T232808Z-266650fb-dirty](swap_band/swap_band_n64/macos-aarch64-c8-bare-apple/20261114T232808Z-266650fb-dirty/report.md)
- [20261117T015426Z-07a9e9a3](swap_band/swap_band_n64/macos-aarch64-c8-bare-apple/20261117T015426Z-07a9e9a3/report.md)

## swap_band/swap_band_n8192

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T095526Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sb_none | 11.02 us | 1.00x |
| sb_plan | 13.92 us | 1.26x |
| sb_value | 5.17 ms | 468.55x |

- [20261115T095526Z-266650fb-dirty](swap_band/swap_band_n8192/linux-x86_64-c16-bare-intel/20261115T095526Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 4 run(s)

Latest: `20261117T015426Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sb_none | 12.66 us | 1.00x |
| sb_plan | 15.09 us | 1.19x |
| sb_value | 2.91 ms | 229.60x |

- [20260719T185155Z-8c3a949d-dirty](swap_band/swap_band_n8192/macos-aarch64-c8-bare-apple/20260719T185155Z-8c3a949d-dirty/report.md)
- [20261114T232949Z-266650fb-dirty](swap_band/swap_band_n8192/macos-aarch64-c8-bare-apple/20261114T232949Z-266650fb-dirty/report.md)
- [20261117T015426Z-07a9e9a3](swap_band/swap_band_n8192/macos-aarch64-c8-bare-apple/20261117T015426Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261117T015650Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| cs_exch | 205.06 us | 1.00x |
| cs_plain | 205.91 us | 1.00x |
| cs_snap | 291.80 us | 1.42x |
| cs_snap_twin | 291.35 us | 1.42x |

- [20261114T233320Z-266650fb-dirty](swap_column/swap_column_n1024/macos-aarch64-c8-bare-apple/20261114T233320Z-266650fb-dirty/report.md)
- [20261117T015650Z-07a9e9a3](swap_column/swap_column_n1024/macos-aarch64-c8-bare-apple/20261117T015650Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261117T015650Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| cs_exch | 21.09 us | 1.00x |
| cs_plain | 21.41 us | 1.02x |
| cs_snap | 27.07 us | 1.28x |
| cs_snap_twin | 26.80 us | 1.27x |

- [20261114T233303Z-266650fb-dirty](swap_column/swap_column_n64/macos-aarch64-c8-bare-apple/20261114T233303Z-266650fb-dirty/report.md)
- [20261117T015650Z-07a9e9a3](swap_column/swap_column_n64/macos-aarch64-c8-bare-apple/20261117T015650Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261117T015650Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| cs_exch | 2.33 ms | 1.00x |
| cs_plain | 3.29 ms | 1.41x |
| cs_snap | 3.19 ms | 1.37x |
| cs_snap_twin | 3.24 ms | 1.39x |

- [20261114T233524Z-266650fb-dirty](swap_column/swap_column_n8192/macos-aarch64-c8-bare-apple/20261114T233524Z-266650fb-dirty/report.md)
- [20261117T015650Z-07a9e9a3](swap_column/swap_column_n8192/macos-aarch64-c8-bare-apple/20261117T015650Z-07a9e9a3/report.md)

## swap_cost/swap_cost_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T101140Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| sc_v1k | 16 ns | 1.62x |
| sc_v64b | 10 ns | 1.00x |
| sc_v64k | 2.16 us | 213.46x |

- [20261115T101140Z-266650fb-dirty](swap_cost/swap_cost_n64/linux-x86_64-c16-bare-intel/20261115T101140Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 5 run(s)

Latest: `20261117T015952Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| sc_v1k | 68 ns | 40.18x |
| sc_v64b | 2 ns | 1.00x |
| sc_v64k | 1.72 us | 1010.59x |

- [20260719T185012Z-8c3a949d-dirty](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20260719T185012Z-8c3a949d-dirty/report.md)
- [20261114T233753Z-266650fb-dirty](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20261114T233753Z-266650fb-dirty/report.md)
- [20261117T015952Z-07a9e9a3](swap_cost/swap_cost_n64/macos-aarch64-c8-bare-apple/20261117T015952Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 6 run(s)

Latest: `20261117T020112Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 11.80 us | 1.34x |
| wp_lp0 | 9.35 us | 1.06x |
| wp_nh128 | 9.15 us | 1.04x |
| wp_nh2k | 13.05 us | 1.48x |
| wp_nh8k | 22.34 us | 2.53x |
| wp_park | 8.84 us | 1.00x |
| wp_spin | 13.32 us | 1.51x |

- [20260719T201051Z-a12070de-dirty](wake_policy/wake_policy_n1024/macos-aarch64-c8-bare-apple/20260719T201051Z-a12070de-dirty/report.md)
- [20261114T234250Z-266650fb-dirty](wake_policy/wake_policy_n1024/macos-aarch64-c8-bare-apple/20261114T234250Z-266650fb-dirty/report.md)
- [20261117T020112Z-07a9e9a3](wake_policy/wake_policy_n1024/macos-aarch64-c8-bare-apple/20261117T020112Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 6 run(s)

Latest: `20261117T020112Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 5.46 us | 1.00x |
| wp_lp0 | 8.93 us | 1.64x |
| wp_nh128 | 5.56 us | 1.02x |
| wp_nh2k | 9.40 us | 1.72x |
| wp_nh8k | 16.70 us | 3.06x |
| wp_park | 6.08 us | 1.11x |
| wp_spin | 10.78 us | 1.97x |

- [20260719T201050Z-a12070de-dirty](wake_policy/wake_policy_n64/macos-aarch64-c8-bare-apple/20260719T201050Z-a12070de-dirty/report.md)
- [20261114T234248Z-266650fb-dirty](wake_policy/wake_policy_n64/macos-aarch64-c8-bare-apple/20261114T234248Z-266650fb-dirty/report.md)
- [20261117T020112Z-07a9e9a3](wake_policy/wake_policy_n64/macos-aarch64-c8-bare-apple/20261117T020112Z-07a9e9a3/report.md)

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

### macos-aarch64-c8-bare-apple, 6 run(s)

Latest: `20261117T020112Z-07a9e9a3`

| variant | median | vs best |
|---|---|---|
| wp_isb8 | 35.70 us | 1.13x |
| wp_lp0 | 36.08 us | 1.14x |
| wp_nh128 | 35.71 us | 1.13x |
| wp_nh2k | 38.92 us | 1.23x |
| wp_nh8k | 54.70 us | 1.73x |
| wp_park | 34.53 us | 1.09x |
| wp_spin | 31.57 us | 1.00x |

- [20260719T201053Z-a12070de-dirty](wake_policy/wake_policy_n8192/macos-aarch64-c8-bare-apple/20260719T201053Z-a12070de-dirty/report.md)
- [20261114T234254Z-266650fb-dirty](wake_policy/wake_policy_n8192/macos-aarch64-c8-bare-apple/20261114T234254Z-266650fb-dirty/report.md)
- [20261117T020112Z-07a9e9a3](wake_policy/wake_policy_n8192/macos-aarch64-c8-bare-apple/20261117T020112Z-07a9e9a3/report.md)
