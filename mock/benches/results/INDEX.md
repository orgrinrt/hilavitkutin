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

Latest: `20261116T074608Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 2.42 us | 12104.00x |
| inc_fetch_add | 2.10 us | 10500.00x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015102Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T015102Z-88d91ab/report.md)
- [20261114T205811Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T205811Z-266650fb-dirty/report.md)
- [20261116T074608Z-dea557ae](atomic_inc_strategy/atomic_inc_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T074608Z-dea557ae/report.md)

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

Latest: `20261116T074608Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 38.84 us | 97092.25x |
| inc_fetch_add | 34.36 us | 85893.75x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015108Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T015108Z-88d91ab/report.md)
- [20261114T205819Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T205819Z-266650fb-dirty/report.md)
- [20261116T074608Z-dea557ae](atomic_inc_strategy/atomic_inc_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T074608Z-dea557ae/report.md)

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

Latest: `20261116T074608Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 610 ns | 1525.00x |
| inc_fetch_add | 496 ns | 1240.50x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015101Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20260511T015101Z-88d91ab/report.md)
- [20261114T205809Z-266650fb](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20261114T205809Z-266650fb/report.md)
- [20261116T074608Z-dea557ae](atomic_inc_strategy/atomic_inc_strategy_n256/macos-aarch64-c8-bare-apple/20261116T074608Z-dea557ae/report.md)

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

Latest: `20261116T074608Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inc_cas_loop | 9.77 us | 24433.75x |
| inc_fetch_add | 8.55 us | 21372.50x |
| inc_load_store | 0 ns | 1.00x |

- [20260511T015104Z-88d91ab](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T015104Z-88d91ab/report.md)
- [20261114T205814Z-266650fb-dirty](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T205814Z-266650fb-dirty/report.md)
- [20261116T074608Z-dea557ae](atomic_inc_strategy/atomic_inc_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T074608Z-dea557ae/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T074652Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 842 ns | 3.64x |
| atomic_relaxed | 231 ns | 1.00x |
| atomic_seqcst | 846 ns | 3.66x |

- [20260510T214037Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n1024/macos-aarch64-c8-bare-apple/20260510T214037Z-f6ee560/report.md)
- [20261116T074652Z-dea557ae](atomic_ordering_cost/atomic_ordering_cost_n1024/macos-aarch64-c8-bare-apple/20261116T074652Z-dea557ae/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T074652Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 14.28 us | 3.29x |
| atomic_relaxed | 4.34 us | 1.00x |
| atomic_seqcst | 14.17 us | 3.26x |

- [20260510T214043Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n16384/macos-aarch64-c8-bare-apple/20260510T214043Z-f6ee560/report.md)
- [20261116T074652Z-dea557ae](atomic_ordering_cost/atomic_ordering_cost_n16384/macos-aarch64-c8-bare-apple/20261116T074652Z-dea557ae/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T074652Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 168 ns | 5.37x |
| atomic_relaxed | 31 ns | 1.00x |
| atomic_seqcst | 170 ns | 5.46x |

- [20260510T214035Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n256/macos-aarch64-c8-bare-apple/20260510T214035Z-f6ee560/report.md)
- [20261116T074652Z-dea557ae](atomic_ordering_cost/atomic_ordering_cost_n256/macos-aarch64-c8-bare-apple/20261116T074652Z-dea557ae/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n4096

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T074652Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 3.53 us | 3.36x |
| atomic_relaxed | 1.05 us | 1.00x |
| atomic_seqcst | 3.53 us | 3.36x |

- [20260510T214039Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n4096/macos-aarch64-c8-bare-apple/20260510T214039Z-f6ee560/report.md)
- [20261116T074652Z-dea557ae](atomic_ordering_cost/atomic_ordering_cost_n4096/macos-aarch64-c8-bare-apple/20261116T074652Z-dea557ae/report.md)

## atomic_ordering_cost/atomic_ordering_cost_n64

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T074652Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| atomic_acquire | 7 ns | 1.69x |
| atomic_relaxed | 4 ns | 1.00x |
| atomic_seqcst | 4 ns | 1.00x |

- [20260510T214034Z-f6ee560](atomic_ordering_cost/atomic_ordering_cost_n64/macos-aarch64-c8-bare-apple/20260510T214034Z-f6ee560/report.md)
- [20261116T074652Z-dea557ae](atomic_ordering_cost/atomic_ordering_cost_n64/macos-aarch64-c8-bare-apple/20261116T074652Z-dea557ae/report.md)

## atomic_vs_plain/atomic_vs_plain_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T021656Z-266650fb`

| variant | median | vs best |
|---|---|---|
| acc_plain | 1.15 us | 1.00x |

- [20261115T021655Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/linux-x86_64-c16-bare-intel/20261115T021655Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074737Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 1.25 us | 1.00x |
| acc_atomic_seqcst | 1.25 us | 1.00x |
| acc_plain | 1.25 us | 1.00x |

- [20260511T013000Z-a410443](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20260511T013000Z-a410443/report.md)
- [20261114T205932Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20261114T205932Z-266650fb-dirty/report.md)
- [20261116T074737Z-dea557ae](atomic_vs_plain/atomic_vs_plain_n1024/macos-aarch64-c8-bare-apple/20261116T074737Z-dea557ae/report.md)

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

Latest: `20261116T074737Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 20.46 us | 1.00x |
| acc_atomic_seqcst | 20.44 us | 1.00x |
| acc_plain | 20.43 us | 1.00x |

- [20260511T013005Z-a410443](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20260511T013005Z-a410443/report.md)
- [20261114T205940Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20261114T205940Z-266650fb-dirty/report.md)
- [20261116T074737Z-dea557ae](atomic_vs_plain/atomic_vs_plain_n16384/macos-aarch64-c8-bare-apple/20261116T074737Z-dea557ae/report.md)

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

Latest: `20261116T074737Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 273 ns | 1.01x |
| acc_atomic_seqcst | 272 ns | 1.00x |
| acc_plain | 272 ns | 1.00x |

- [20260511T012959Z-a410443](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20260511T012959Z-a410443/report.md)
- [20261114T205930Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20261114T205930Z-266650fb-dirty/report.md)
- [20261116T074737Z-dea557ae](atomic_vs_plain/atomic_vs_plain_n256/macos-aarch64-c8-bare-apple/20261116T074737Z-dea557ae/report.md)

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

Latest: `20261116T074737Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| acc_atomic_relaxed | 5.15 us | 1.02x |
| acc_atomic_seqcst | 5.06 us | 1.00x |
| acc_plain | 5.15 us | 1.02x |

- [20260511T013001Z-a410443](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20260511T013001Z-a410443/report.md)
- [20261114T205935Z-266650fb-dirty](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20261114T205935Z-266650fb-dirty/report.md)
- [20261116T074737Z-dea557ae](atomic_vs_plain/atomic_vs_plain_n4096/macos-aarch64-c8-bare-apple/20261116T074737Z-dea557ae/report.md)

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

Latest: `20261116T074820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 60 ns | 1.00x |
| ctz_intrinsic | 60 ns | 1.01x |
| ctz_loop | 150 ns | 2.52x |

- [20260511T021038Z-315383f](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20260511T021038Z-315383f/report.md)
- [20261114T210018Z-266650fb-dirty](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20261114T210018Z-266650fb-dirty/report.md)
- [20261116T074820Z-dea557ae](bit_count_ops/bit_count_ops_n1024/macos-aarch64-c8-bare-apple/20261116T074820Z-dea557ae/report.md)

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

Latest: `20261116T074820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 501 ns | 1.00x |
| ctz_intrinsic | 503 ns | 1.00x |
| ctz_loop | 2.04 us | 4.07x |

- [20260511T021040Z-315383f](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20260511T021040Z-315383f/report.md)
- [20261114T210022Z-266650fb-dirty](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20261114T210022Z-266650fb-dirty/report.md)
- [20261116T074820Z-dea557ae](bit_count_ops/bit_count_ops_n16384/macos-aarch64-c8-bare-apple/20261116T074820Z-dea557ae/report.md)

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

Latest: `20261116T074820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 15 ns | 1.00x |
| ctz_intrinsic | 15 ns | 1.00x |
| ctz_loop | 52 ns | 3.38x |

- [20260511T021037Z-315383f](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20260511T021037Z-315383f/report.md)
- [20261114T210015Z-266650fb-dirty](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20261114T210015Z-266650fb-dirty/report.md)
- [20261116T074820Z-dea557ae](bit_count_ops/bit_count_ops_n256/macos-aarch64-c8-bare-apple/20261116T074820Z-dea557ae/report.md)

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

Latest: `20261116T074820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ctz_debruijn | 153 ns | 1.00x |
| ctz_intrinsic | 154 ns | 1.00x |
| ctz_loop | 526 ns | 3.43x |

- [20260511T021039Z-315383f](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20260511T021039Z-315383f/report.md)
- [20261114T210020Z-266650fb-dirty](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20261114T210020Z-266650fb-dirty/report.md)
- [20261116T074820Z-dea557ae](bit_count_ops/bit_count_ops_n4096/macos-aarch64-c8-bare-apple/20261116T074820Z-dea557ae/report.md)

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

Latest: `20261116T074859Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bc_index | 35 ns | 1.00x |
| bc_iter | 36 ns | 1.05x |
| bc_unchecked | 39 ns | 1.14x |

- [20260511T005008Z-f10ba65](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20260511T005008Z-f10ba65/report.md)
- [20261114T210100Z-266650fb-dirty](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20261114T210100Z-266650fb-dirty/report.md)
- [20261116T074859Z-dea557ae](bounds_check/bounds_check_n1024/macos-aarch64-c8-bare-apple/20261116T074859Z-dea557ae/report.md)

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

Latest: `20261116T074859Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bc_index | 358 ns | 1.00x |
| bc_iter | 359 ns | 1.01x |
| bc_unchecked | 633 ns | 1.77x |

- [20260511T005013Z-f10ba65](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20260511T005013Z-f10ba65/report.md)
- [20261114T210104Z-266650fb-dirty](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20261114T210104Z-266650fb-dirty/report.md)
- [20261116T074859Z-dea557ae](bounds_check/bounds_check_n16384/macos-aarch64-c8-bare-apple/20261116T074859Z-dea557ae/report.md)

## bounds_check/bounds_check_n256

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T023548Z-266650fb`

| variant | median | vs best |
|---|---|---|
| bc_unchecked | 37 ns | 1.00x |

- [20261115T023544Z-266650fb-dirty](bounds_check/bounds_check_n256/linux-x86_64-c16-bare-intel/20261115T023544Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074859Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bc_index | 5 ns | 1.00x |
| bc_iter | 6 ns | 1.24x |
| bc_unchecked | 7 ns | 1.34x |

- [20260511T005007Z-f10ba65](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20260511T005007Z-f10ba65/report.md)
- [20261114T210058Z-266650fb-dirty](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20261114T210058Z-266650fb-dirty/report.md)
- [20261116T074859Z-dea557ae](bounds_check/bounds_check_n256/macos-aarch64-c8-bare-apple/20261116T074859Z-dea557ae/report.md)

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

Latest: `20261116T074859Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bc_index | 93 ns | 1.00x |
| bc_iter | 94 ns | 1.01x |
| bc_unchecked | 145 ns | 1.55x |

- [20260511T005009Z-f10ba65](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20260511T005009Z-f10ba65/report.md)
- [20261114T210102Z-266650fb-dirty](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20261114T210102Z-266650fb-dirty/report.md)
- [20261116T074859Z-dea557ae](bounds_check/bounds_check_n4096/macos-aarch64-c8-bare-apple/20261116T074859Z-dea557ae/report.md)

## branch_pattern/branch_pattern_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024902Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.71 us | 1.99x |
| branch_if | 861 ns | 1.00x |

- [20261115T024902Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/linux-x86_64-c16-bare-intel/20261115T024902Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074936Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 1.61 us | 1.00x |
| branch_if | 1.64 us | 1.01x |

- [20260510T213318Z-b042648](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20260510T213318Z-b042648/report.md)
- [20261114T210137Z-266650fb-dirty](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20261114T210137Z-266650fb-dirty/report.md)
- [20261116T074936Z-dea557ae](branch_pattern/branch_pattern_n1024/macos-aarch64-c8-bare-apple/20261116T074936Z-dea557ae/report.md)

## branch_pattern/branch_pattern_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025232Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 27.44 us | 1.00x |
| branch_if | 29.93 us | 1.09x |

- [20261115T025232Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/linux-x86_64-c16-bare-intel/20261115T025232Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074936Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 26.07 us | 1.00x |
| branch_if | 26.07 us | 1.00x |

- [20260510T213324Z-b042648](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20260510T213324Z-b042648/report.md)
- [20261114T210143Z-266650fb-dirty](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20261114T210143Z-266650fb-dirty/report.md)
- [20261116T074936Z-dea557ae](branch_pattern/branch_pattern_n16384/macos-aarch64-c8-bare-apple/20261116T074936Z-dea557ae/report.md)

## branch_pattern/branch_pattern_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024719Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 429 ns | 1.94x |
| branch_if | 221 ns | 1.00x |

- [20261115T024719Z-266650fb-dirty](branch_pattern/branch_pattern_n256/linux-x86_64-c16-bare-intel/20261115T024719Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074936Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 392 ns | 1.00x |
| branch_if | 392 ns | 1.00x |

- [20260510T213317Z-b042648](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20260510T213317Z-b042648/report.md)
- [20261114T210136Z-266650fb-dirty](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20261114T210136Z-266650fb-dirty/report.md)
- [20261116T074936Z-dea557ae](branch_pattern/branch_pattern_n256/macos-aarch64-c8-bare-apple/20261116T074936Z-dea557ae/report.md)

## branch_pattern/branch_pattern_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T025046Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.29 us | 2.00x |
| branch_if | 3.15 us | 1.00x |

- [20261115T025046Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/linux-x86_64-c16-bare-intel/20261115T025046Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074936Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 6.49 us | 1.00x |
| branch_if | 6.49 us | 1.00x |

- [20260510T213320Z-b042648](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20260510T213320Z-b042648/report.md)
- [20261114T210139Z-266650fb-dirty](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20261114T210139Z-266650fb-dirty/report.md)
- [20261116T074936Z-dea557ae](branch_pattern/branch_pattern_n4096/macos-aarch64-c8-bare-apple/20261116T074936Z-dea557ae/report.md)

## branch_pattern/branch_pattern_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T024533Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 112 ns | 1.79x |
| branch_if | 63 ns | 1.00x |

- [20261115T024533Z-266650fb-dirty](branch_pattern/branch_pattern_n64/linux-x86_64-c16-bare-intel/20261115T024533Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T074936Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| branch_cmov | 82 ns | 1.00x |
| branch_if | 83 ns | 1.01x |

- [20260510T213316Z-b042648](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20260510T213316Z-b042648/report.md)
- [20261114T210134Z-266650fb-dirty](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20261114T210134Z-266650fb-dirty/report.md)
- [20261116T074936Z-dea557ae](branch_pattern/branch_pattern_n64/macos-aarch64-c8-bare-apple/20261116T074936Z-dea557ae/report.md)

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

Latest: `20261116T075016Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 1.28 us | 1.01x |
| bp_skew_high | 1.27 us | 1.01x |
| bp_skew_low | 1.26 us | 1.00x |

- [20260511T011011Z-c61ad63](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20260511T011011Z-c61ad63/report.md)
- [20261114T210220Z-266650fb-dirty](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20261114T210220Z-266650fb-dirty/report.md)
- [20261116T075016Z-dea557ae](branch_predictability/branch_predictability_n1024/macos-aarch64-c8-bare-apple/20261116T075016Z-dea557ae/report.md)

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

Latest: `20261116T075016Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 20.52 us | 1.00x |
| bp_skew_high | 20.54 us | 1.00x |
| bp_skew_low | 20.55 us | 1.00x |

- [20260511T011016Z-c61ad63](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20260511T011016Z-c61ad63/report.md)
- [20261114T210227Z-266650fb-dirty](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20261114T210227Z-266650fb-dirty/report.md)
- [20261116T075016Z-dea557ae](branch_predictability/branch_predictability_n16384/macos-aarch64-c8-bare-apple/20261116T075016Z-dea557ae/report.md)

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

Latest: `20261116T075016Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 295 ns | 1.00x |
| bp_skew_high | 295 ns | 1.00x |
| bp_skew_low | 296 ns | 1.00x |

- [20260511T011010Z-c61ad63](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20260511T011010Z-c61ad63/report.md)
- [20261114T210218Z-266650fb-dirty](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20261114T210218Z-266650fb-dirty/report.md)
- [20261116T075016Z-dea557ae](branch_predictability/branch_predictability_n256/macos-aarch64-c8-bare-apple/20261116T075016Z-dea557ae/report.md)

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

Latest: `20261116T075016Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| bp_balanced | 5.11 us | 1.00x |
| bp_skew_high | 5.11 us | 1.00x |
| bp_skew_low | 5.11 us | 1.00x |

- [20260511T011012Z-c61ad63](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20260511T011012Z-c61ad63/report.md)
- [20261114T210223Z-266650fb-dirty](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20261114T210223Z-266650fb-dirty/report.md)
- [20261116T075016Z-dea557ae](branch_predictability/branch_predictability_n4096/macos-aarch64-c8-bare-apple/20261116T075016Z-dea557ae/report.md)

## cache_layout/cache_layout_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030857Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 28 ns | 1.00x |
| layout_soa | 28 ns | 1.00x |

- [20261115T030857Z-266650fb-dirty](cache_layout/cache_layout_n1024/linux-x86_64-c16-bare-intel/20261115T030857Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075057Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| layout_aos | 8 ns | 1.00x |
| layout_soa | 8 ns | 1.05x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210300Z-266650fb-dirty](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20261114T210300Z-266650fb-dirty/report.md)
- [20261116T075057Z-dea557ae](cache_layout/cache_layout_n1024/macos-aarch64-c8-bare-apple/20261116T075057Z-dea557ae/report.md)

## cache_layout/cache_layout_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031223Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 333 ns | 1.00x |
| layout_soa | 333 ns | 1.00x |

- [20261115T031223Z-266650fb-dirty](cache_layout/cache_layout_n16384/linux-x86_64-c16-bare-intel/20261115T031223Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075057Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| layout_aos | 338 ns | 1.00x |
| layout_soa | 338 ns | 1.00x |

- [20260510T214108Z-f6ee560](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20260510T214108Z-f6ee560/report.md)
- [20261114T210303Z-266650fb-dirty](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20261114T210303Z-266650fb-dirty/report.md)
- [20261116T075057Z-dea557ae](cache_layout/cache_layout_n16384/macos-aarch64-c8-bare-apple/20261116T075057Z-dea557ae/report.md)

## cache_layout/cache_layout_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030714Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 12 ns | 1.01x |
| layout_soa | 12 ns | 1.00x |

- [20261115T030714Z-266650fb-dirty](cache_layout/cache_layout_n256/linux-x86_64-c16-bare-intel/20261115T030714Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075057Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| layout_aos | 1 ns | 1.00x |
| layout_soa | 1 ns | 1.00x |

- [20260510T214105Z-f6ee560](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20260510T214105Z-f6ee560/report.md)
- [20261114T210259Z-266650fb-dirty](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20261114T210259Z-266650fb-dirty/report.md)
- [20261116T075057Z-dea557ae](cache_layout/cache_layout_n256/macos-aarch64-c8-bare-apple/20261116T075057Z-dea557ae/report.md)

## cache_layout/cache_layout_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T031040Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 90 ns | 1.01x |
| layout_soa | 89 ns | 1.00x |

- [20261115T031040Z-266650fb-dirty](cache_layout/cache_layout_n4096/linux-x86_64-c16-bare-intel/20261115T031040Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075057Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| layout_aos | 63 ns | 1.00x |
| layout_soa | 65 ns | 1.03x |

- [20260510T214106Z-f6ee560](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20260510T214106Z-f6ee560/report.md)
- [20261114T210301Z-266650fb-dirty](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20261114T210301Z-266650fb-dirty/report.md)
- [20261116T075057Z-dea557ae](cache_layout/cache_layout_n4096/macos-aarch64-c8-bare-apple/20261116T075057Z-dea557ae/report.md)

## cache_layout/cache_layout_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T030527Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| layout_aos | 10 ns | 1.00x |
| layout_soa | 10 ns | 1.00x |

- [20261115T030527Z-266650fb-dirty](cache_layout/cache_layout_n64/linux-x86_64-c16-bare-intel/20261115T030527Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075057Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| layout_aos | 0 ns | 1.00x |
| layout_soa | 0 ns | 1.00x |

- [20260510T214104Z-f6ee560](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20260510T214104Z-f6ee560/report.md)
- [20261114T210257Z-266650fb-dirty](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20261114T210257Z-266650fb-dirty/report.md)
- [20261116T075057Z-dea557ae](cache_layout/cache_layout_n64/macos-aarch64-c8-bare-apple/20261116T075057Z-dea557ae/report.md)

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

Latest: `20261116T075133Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 780 ns | 1.03x |
| dep_graph_dense | 26.46 us | 35.08x |
| dep_graph_hybrid | 754 ns | 1.00x |

- [20260510T212550Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20260510T212550Z-771392d/report.md)
- [20261114T210349Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20261114T210349Z-266650fb-dirty/report.md)
- [20261116T075133Z-dea557ae](dep_graph_csr_9/dep_graph_csr_9_n128/macos-aarch64-c8-bare-apple/20261116T075133Z-dea557ae/report.md)

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

Latest: `20261116T075133Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 52 ns | 1.00x |
| dep_graph_dense | 866 ns | 16.61x |
| dep_graph_hybrid | 434 ns | 8.33x |

- [20260510T212544Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20260510T212544Z-771392d/report.md)
- [20261114T210341Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20261114T210341Z-266650fb-dirty/report.md)
- [20261116T075133Z-dea557ae](dep_graph_csr_9/dep_graph_csr_9_n16/macos-aarch64-c8-bare-apple/20261116T075133Z-dea557ae/report.md)

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

Latest: `20261116T075133Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 1.63 us | 1.04x |
| dep_graph_dense | 104.01 us | 66.11x |
| dep_graph_hybrid | 1.57 us | 1.00x |

- [20260510T212557Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20260510T212557Z-771392d/report.md)
- [20261114T210355Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20261114T210355Z-266650fb-dirty/report.md)
- [20261116T075133Z-dea557ae](dep_graph_csr_9/dep_graph_csr_9_n256/macos-aarch64-c8-bare-apple/20261116T075133Z-dea557ae/report.md)

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

Latest: `20261116T075133Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 150 ns | 1.00x |
| dep_graph_dense | 2.13 us | 14.24x |
| dep_graph_hybrid | 155 ns | 1.04x |

- [20260510T212545Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20260510T212545Z-771392d/report.md)
- [20261114T210343Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20261114T210343Z-266650fb-dirty/report.md)
- [20261116T075133Z-dea557ae](dep_graph_csr_9/dep_graph_csr_9_n32/macos-aarch64-c8-bare-apple/20261116T075133Z-dea557ae/report.md)

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

Latest: `20261116T075133Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 359 ns | 1.00x |
| dep_graph_dense | 7.02 us | 19.56x |
| dep_graph_hybrid | 369 ns | 1.03x |

- [20260510T212547Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20260510T212547Z-771392d/report.md)
- [20261114T210346Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20261114T210346Z-266650fb-dirty/report.md)
- [20261116T075133Z-dea557ae](dep_graph_csr_9/dep_graph_csr_9_n64/macos-aarch64-c8-bare-apple/20261116T075133Z-dea557ae/report.md)

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

Latest: `20261116T075133Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dep_graph_csr | 25 ns | 1.00x |
| dep_graph_dense | 554 ns | 21.80x |
| dep_graph_hybrid | 102 ns | 4.00x |

- [20260510T212543Z-771392d](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20260510T212543Z-771392d/report.md)
- [20261114T210339Z-266650fb-dirty](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20261114T210339Z-266650fb-dirty/report.md)
- [20261116T075133Z-dea557ae](dep_graph_csr_9/dep_graph_csr_9_n8/macos-aarch64-c8-bare-apple/20261116T075133Z-dea557ae/report.md)

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

Latest: `20261116T075227Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 110 ns | 1.00x |
| dispatch_runtime | 291 ns | 2.63x |
| dispatch_volatile | 160 ns | 1.45x |

- [20260510T204023Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20260510T204023Z-3c0d0d5/report.md)
- [20261114T210435Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20261114T210435Z-266650fb-dirty/report.md)
- [20261116T075227Z-dea557ae](dispatch_dynamic/dispatch_dynamic_n1024/macos-aarch64-c8-bare-apple/20261116T075227Z-dea557ae/report.md)

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

Latest: `20261116T075227Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2.51 us | 1.00x |
| dispatch_runtime | 9.86 us | 3.93x |
| dispatch_volatile | 2.56 us | 1.02x |

- [20260510T204026Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20260510T204026Z-3c0d0d5/report.md)
- [20261114T210440Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20261114T210440Z-266650fb-dirty/report.md)
- [20261116T075227Z-dea557ae](dispatch_dynamic/dispatch_dynamic_n16384/macos-aarch64-c8-bare-apple/20261116T075227Z-dea557ae/report.md)

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

Latest: `20261116T075227Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 10 ns | 1.00x |
| dispatch_runtime | 46 ns | 4.40x |
| dispatch_volatile | 45 ns | 4.33x |

- [20260510T204022Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20260510T204022Z-3c0d0d5/report.md)
- [20261114T210433Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20261114T210433Z-266650fb-dirty/report.md)
- [20261116T075227Z-dea557ae](dispatch_dynamic/dispatch_dynamic_n256/macos-aarch64-c8-bare-apple/20261116T075227Z-dea557ae/report.md)

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

Latest: `20261116T075227Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 593 ns | 1.00x |
| dispatch_runtime | 1.10 us | 1.86x |
| dispatch_volatile | 646 ns | 1.09x |

- [20260510T204024Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20260510T204024Z-3c0d0d5/report.md)
- [20261114T210437Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20261114T210437Z-266650fb-dirty/report.md)
- [20261116T075227Z-dea557ae](dispatch_dynamic/dispatch_dynamic_n4096/macos-aarch64-c8-bare-apple/20261116T075227Z-dea557ae/report.md)

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

Latest: `20261116T075227Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_direct | 2 ns | 1.00x |
| dispatch_runtime | 8 ns | 3.95x |
| dispatch_volatile | 8 ns | 3.57x |

- [20260510T204021Z-3c0d0d5](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20260510T204021Z-3c0d0d5/report.md)
- [20261114T210430Z-266650fb-dirty](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20261114T210430Z-266650fb-dirty/report.md)
- [20261116T075227Z-dea557ae](dispatch_dynamic/dispatch_dynamic_n64/macos-aarch64-c8-bare-apple/20261116T075227Z-dea557ae/report.md)

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

Latest: `20261116T075315Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 111 ns | 1.01x |
| dispatch_direct | 110 ns | 1.00x |
| dispatch_trait | 111 ns | 1.01x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210515Z-266650fb-dirty](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20261114T210515Z-266650fb-dirty/report.md)
- [20261116T075315Z-dea557ae](dispatch_static/dispatch_static_n1024/macos-aarch64-c8-bare-apple/20261116T075315Z-dea557ae/report.md)

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

Latest: `20261116T075315Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2.51 us | 1.00x |
| dispatch_direct | 2.51 us | 1.00x |
| dispatch_trait | 2.51 us | 1.00x |

- [20260510T204037Z-3c0d0d5](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20260510T204037Z-3c0d0d5/report.md)
- [20261114T210520Z-266650fb-dirty](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20261114T210520Z-266650fb-dirty/report.md)
- [20261116T075315Z-dea557ae](dispatch_static/dispatch_static_n16384/macos-aarch64-c8-bare-apple/20261116T075315Z-dea557ae/report.md)

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

Latest: `20261116T075315Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 10 ns | 1.00x |
| dispatch_direct | 11 ns | 1.04x |
| dispatch_trait | 11 ns | 1.04x |

- [20260510T204034Z-3c0d0d5](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20260510T204034Z-3c0d0d5/report.md)
- [20261114T210513Z-266650fb-dirty](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20261114T210513Z-266650fb-dirty/report.md)
- [20261116T075315Z-dea557ae](dispatch_static/dispatch_static_n256/macos-aarch64-c8-bare-apple/20261116T075315Z-dea557ae/report.md)

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

Latest: `20261116T075315Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 597 ns | 1.01x |
| dispatch_direct | 591 ns | 1.00x |
| dispatch_trait | 594 ns | 1.01x |

- [20260510T204035Z-3c0d0d5](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20260510T204035Z-3c0d0d5/report.md)
- [20261114T210517Z-266650fb-dirty](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20261114T210517Z-266650fb-dirty/report.md)
- [20261116T075315Z-dea557ae](dispatch_static/dispatch_static_n4096/macos-aarch64-c8-bare-apple/20261116T075315Z-dea557ae/report.md)

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

Latest: `20261116T075315Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| dispatch_const_ptr | 2 ns | 1.00x |
| dispatch_direct | 2 ns | 1.00x |
| dispatch_trait | 2 ns | 1.00x |

- [20260510T204033Z-3c0d0d5](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20260510T204033Z-3c0d0d5/report.md)
- [20261114T210511Z-266650fb-dirty](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20261114T210511Z-266650fb-dirty/report.md)
- [20261116T075315Z-dea557ae](dispatch_static/dispatch_static_n64/macos-aarch64-c8-bare-apple/20261116T075315Z-dea557ae/report.md)

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

Latest: `20261116T075359Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 382 ns | 8.99x |
| ema_neon | 42 ns | 1.00x |
| ema_scalar | 86 ns | 2.02x |

- [20260510T204029Z-3c0d0d5](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20260510T204029Z-3c0d0d5/report.md)
- [20261114T210600Z-266650fb-dirty](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20261114T210600Z-266650fb-dirty/report.md)
- [20261116T075359Z-dea557ae](ema_axis_h/ema_axis_h_n1024/macos-aarch64-c8-bare-apple/20261116T075359Z-dea557ae/report.md)

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

Latest: `20261116T075359Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 6.53 us | 4.36x |
| ema_neon | 1.50 us | 1.00x |
| ema_scalar | 1.58 us | 1.06x |

- [20260510T204031Z-3c0d0d5](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20260510T204031Z-3c0d0d5/report.md)
- [20261114T210605Z-266650fb-dirty](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20261114T210605Z-266650fb-dirty/report.md)
- [20261116T075359Z-dea557ae](ema_axis_h/ema_axis_h_n16384/macos-aarch64-c8-bare-apple/20261116T075359Z-dea557ae/report.md)

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

Latest: `20261116T075359Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 79 ns | 65.58x |
| ema_neon | 1 ns | 1.00x |
| ema_scalar | 18 ns | 15.25x |

- [20260510T204028Z-3c0d0d5](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20260510T204028Z-3c0d0d5/report.md)
- [20261114T210558Z-266650fb-dirty](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20261114T210558Z-266650fb-dirty/report.md)
- [20261116T075359Z-dea557ae](ema_axis_h/ema_axis_h_n256/macos-aarch64-c8-bare-apple/20261116T075359Z-dea557ae/report.md)

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

Latest: `20261116T075359Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 1.61 us | 5.46x |
| ema_neon | 295 ns | 1.00x |
| ema_scalar | 380 ns | 1.29x |

- [20260510T204030Z-3c0d0d5](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20260510T204030Z-3c0d0d5/report.md)
- [20261114T210602Z-266650fb-dirty](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20261114T210602Z-266650fb-dirty/report.md)
- [20261116T075359Z-dea557ae](ema_axis_h/ema_axis_h_n4096/macos-aarch64-c8-bare-apple/20261116T075359Z-dea557ae/report.md)

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

Latest: `20261116T075359Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_autovec | 0 ns | 1.00x |
| ema_neon | 0 ns | 1.00x |
| ema_scalar | 3 ns | 7.25x |

- [20260510T204028Z-3c0d0d5](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20260510T204028Z-3c0d0d5/report.md)
- [20261114T210556Z-266650fb-dirty](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20261114T210556Z-266650fb-dirty/report.md)
- [20261116T075359Z-dea557ae](ema_axis_h/ema_axis_h_n64/macos-aarch64-c8-bare-apple/20261116T075359Z-dea557ae/report.md)

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

Latest: `20261116T075444Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 2.09 us | 1.71x |
| ema_form_intshift | 1.22 us | 1.00x |
| ema_form_q32 | 1.85 us | 1.52x |

- [20260510T233132Z-5dcdb36](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20260510T233132Z-5dcdb36/report.md)
- [20261114T210643Z-266650fb-dirty](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20261114T210643Z-266650fb-dirty/report.md)
- [20261116T075444Z-dea557ae](ema_formulation/ema_formulation_n1024/macos-aarch64-c8-bare-apple/20261116T075444Z-dea557ae/report.md)

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

Latest: `20261116T075444Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 35.70 us | 1.75x |
| ema_form_intshift | 20.44 us | 1.00x |
| ema_form_q32 | 30.69 us | 1.50x |

- [20260510T233138Z-5dcdb36](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20260510T233138Z-5dcdb36/report.md)
- [20261114T210651Z-266650fb-dirty](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20261114T210651Z-266650fb-dirty/report.md)
- [20261116T075444Z-dea557ae](ema_formulation/ema_formulation_n16384/macos-aarch64-c8-bare-apple/20261116T075444Z-dea557ae/report.md)

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

Latest: `20261116T075444Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 411 ns | 1.56x |
| ema_form_intshift | 264 ns | 1.00x |
| ema_form_q32 | 418 ns | 1.58x |

- [20260510T233131Z-5dcdb36](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20260510T233131Z-5dcdb36/report.md)
- [20261114T210641Z-266650fb-dirty](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20261114T210641Z-266650fb-dirty/report.md)
- [20261116T075444Z-dea557ae](ema_formulation/ema_formulation_n256/macos-aarch64-c8-bare-apple/20261116T075444Z-dea557ae/report.md)

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

Latest: `20261116T075444Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| ema_form_float | 8.80 us | 1.74x |
| ema_form_intshift | 5.06 us | 1.00x |
| ema_form_q32 | 7.61 us | 1.50x |

- [20260510T233134Z-5dcdb36](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20260510T233134Z-5dcdb36/report.md)
- [20261114T210646Z-266650fb-dirty](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20261114T210646Z-266650fb-dirty/report.md)
- [20261116T075444Z-dea557ae](ema_formulation/ema_formulation_n4096/macos-aarch64-c8-bare-apple/20261116T075444Z-dea557ae/report.md)

## engine_vs_std/accumulator_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013654Z-23953983](engine_vs_std/accumulator_n1048576/linux-x86_64-c4-container-intel/20261114T013654Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 726.20 us | 7.87x |
| evs_engine_par | 231.61 us | 2.51x |
| evs_std | 106.13 us | 1.15x |
| evs_std_par | 92.22 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/accumulator_n1048576/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/accumulator_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013321Z-c894a779](engine_vs_std/accumulator_n4096/linux-x86_64-c4-container-intel/20261114T013321Z-c894a779/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 13.01 us | 31.86x |
| evs_engine_par | 55.13 us | 135.02x |
| evs_std | 408 ns | 1.00x |
| evs_std_par | 27.15 us | 66.51x |

- [20261116T075555Z-dea557ae](engine_vs_std/accumulator_n4096/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/accumulator_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013855Z-23953983](engine_vs_std/accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T013855Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 2.79 ms | 5.04x |
| evs_engine_par | 761.92 us | 1.38x |
| evs_std | 592.70 us | 1.07x |
| evs_std_par | 553.65 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/accumulator_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/accumulator_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013507Z-23953983](engine_vs_std/accumulator_n65536/linux-x86_64-c4-container-intel/20261114T013507Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 54.61 us | 8.41x |
| evs_engine_par | 72.27 us | 11.13x |
| evs_std | 6.50 us | 1.00x |
| evs_std_par | 34.77 us | 5.35x |

- [20261116T075555Z-dea557ae](engine_vs_std/accumulator_n65536/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/branching_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014141Z-23953983](engine_vs_std/branching_n1048576/linux-x86_64-c4-container-intel/20261114T014141Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 335.44 us | 2.66x |
| evs_std | 126.08 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/branching_n1048576/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/branching_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T013949Z-23953983](engine_vs_std/branching_n4096/linux-x86_64-c4-container-intel/20261114T013949Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 11.45 us | 22.91x |
| evs_std | 500 ns | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/branching_n4096/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/branching_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014245Z-23953983](engine_vs_std/branching_n4194304/linux-x86_64-c4-container-intel/20261114T014245Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 2.01 ms | 3.35x |
| evs_std | 599.02 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/branching_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/branching_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014044Z-23953983](engine_vs_std/branching_n65536/linux-x86_64-c4-container-intel/20261114T014044Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 28.20 us | 3.63x |
| evs_std | 7.76 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/branching_n65536/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/element_wise_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014531Z-23953983](engine_vs_std/element_wise_n1048576/linux-x86_64-c4-container-intel/20261114T014531Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 108.03 us | 1.03x |
| evs_std | 105.39 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/element_wise_n1048576/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/element_wise_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014339Z-23953983](engine_vs_std/element_wise_n4096/linux-x86_64-c4-container-intel/20261114T014339Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 467 ns | 1.13x |
| evs_std | 412 ns | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/element_wise_n4096/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/element_wise_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014634Z-23953983](engine_vs_std/element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014634Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 578.18 us | 1.00x |
| evs_std | 594.49 us | 1.03x |

- [20261116T075555Z-dea557ae](engine_vs_std/element_wise_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/element_wise_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014434Z-23953983](engine_vs_std/element_wise_n65536/linux-x86_64-c4-container-intel/20261114T014434Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 6.73 us | 1.03x |
| evs_std | 6.55 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/element_wise_n65536/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/startup_accumulator_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014741Z-23953983](engine_vs_std/startup_accumulator_n4194304/linux-x86_64-c4-container-intel/20261114T014741Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 285.15 us | 1.00x |
| evs_std_startup | 445.54 us | 1.56x |

- [20261116T075555Z-dea557ae](engine_vs_std/startup_accumulator_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/startup_branching_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014850Z-23953983](engine_vs_std/startup_branching_n4194304/linux-x86_64-c4-container-intel/20261114T014850Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 1.09 ms | 2.56x |
| evs_std_startup | 423.82 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/startup_branching_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/startup_element_wise_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T014957Z-23953983](engine_vs_std/startup_element_wise_n4194304/linux-x86_64-c4-container-intel/20261114T014957Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 1.36 ms | 3.18x |
| evs_std_startup | 429.73 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/startup_element_wise_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/startup_wide_parallel_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015143Z-23953983](engine_vs_std/startup_wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T015143Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine_startup | 2.22 ms | 1.10x |
| evs_std_startup | 2.02 ms | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/startup_wide_parallel_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/wide_parallel_n1048576

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015712Z-23953983](engine_vs_std/wide_parallel_n1048576/linux-x86_64-c4-container-intel/20261114T015712Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 3.02 ms | 2.74x |
| evs_engine_par | 1.14 ms | 1.04x |
| evs_std | 2.98 ms | 2.71x |
| evs_std_par | 1.10 ms | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/wide_parallel_n1048576/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/wide_parallel_n4096

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015326Z-23953983](engine_vs_std/wide_parallel_n4096/linux-x86_64-c4-container-intel/20261114T015326Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 22.41 us | 1.89x |
| evs_engine_par | 59.58 us | 5.03x |
| evs_std | 11.85 us | 1.00x |
| evs_std_par | 28.87 us | 2.44x |

- [20261116T075555Z-dea557ae](engine_vs_std/wide_parallel_n4096/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/wide_parallel_n4194304

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T020005Z-23953983](engine_vs_std/wide_parallel_n4194304/linux-x86_64-c4-container-intel/20261114T020005Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 12.16 ms | 2.97x |
| evs_engine_par | 4.44 ms | 1.08x |
| evs_std | 12.08 ms | 2.95x |
| evs_std_par | 4.10 ms | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/wide_parallel_n4194304/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## engine_vs_std/wide_parallel_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261114T015511Z-23953983](engine_vs_std/wide_parallel_n65536/linux-x86_64-c4-container-intel/20261114T015511Z-23953983/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T075555Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| evs_engine | 197.55 us | 1.79x |
| evs_engine_par | 115.83 us | 1.05x |
| evs_std | 189.40 us | 1.71x |
| evs_std_par | 110.44 us | 1.00x |

- [20261116T075555Z-dea557ae](engine_vs_std/wide_parallel_n65536/macos-aarch64-c8-bare-apple/20261116T075555Z-dea557ae/report.md)

## fiber_theory/fiber_theory_n16

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042302Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 95 ns | 1.00x |
| fib_spectral | 72.54 us | 761.22x |

- [20261115T042302Z-266650fb-dirty](fiber_theory/fiber_theory_n16/linux-x86_64-c16-bare-intel/20261115T042302Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075943Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 95 ns | 1.00x |
| fib_spectral | 65.69 us | 688.54x |

- [20260719T190052Z-e4cd1757-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20260719T190052Z-e4cd1757-dirty/report.md)
- [20261114T210756Z-266650fb-dirty](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20261114T210756Z-266650fb-dirty/report.md)
- [20261116T075943Z-dea557ae](fiber_theory/fiber_theory_n16/macos-aarch64-c8-bare-apple/20261116T075943Z-dea557ae/report.md)

## fiber_theory/fiber_theory_n32

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042449Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 191 ns | 1.00x |
| fib_spectral | 86.81 us | 455.22x |

- [20261115T042449Z-266650fb-dirty](fiber_theory/fiber_theory_n32/linux-x86_64-c16-bare-intel/20261115T042449Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075943Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 185 ns | 1.00x |
| fib_spectral | 82.10 us | 442.80x |

- [20260719T190056Z-e4cd1757-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20260719T190056Z-e4cd1757-dirty/report.md)
- [20261114T210800Z-266650fb-dirty](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20261114T210800Z-266650fb-dirty/report.md)
- [20261116T075943Z-dea557ae](fiber_theory/fiber_theory_n32/macos-aarch64-c8-bare-apple/20261116T075943Z-dea557ae/report.md)

## fiber_theory/fiber_theory_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042637Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 401 ns | 1.00x |
| fib_spectral | 113.57 us | 282.93x |

- [20261115T042637Z-266650fb-dirty](fiber_theory/fiber_theory_n64/linux-x86_64-c16-bare-intel/20261115T042637Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075943Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 387 ns | 1.00x |
| fib_spectral | 127.41 us | 329.15x |

- [20260719T190101Z-e4cd1757-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20260719T190101Z-e4cd1757-dirty/report.md)
- [20261114T210807Z-266650fb-dirty](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20261114T210807Z-266650fb-dirty/report.md)
- [20261116T075943Z-dea557ae](fiber_theory/fiber_theory_n64/macos-aarch64-c8-bare-apple/20261116T075943Z-dea557ae/report.md)

## fiber_theory/fiber_theory_n8

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T042113Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 50 ns | 1.00x |
| fib_spectral | 16.08 us | 320.29x |

- [20261115T042113Z-266650fb-dirty](fiber_theory/fiber_theory_n8/linux-x86_64-c16-bare-intel/20261115T042113Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T075943Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fib_greedy | 35 ns | 1.00x |
| fib_spectral | 16.85 us | 487.02x |

- [20260719T190048Z-e4cd1757-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20260719T190048Z-e4cd1757-dirty/report.md)
- [20261114T210752Z-266650fb-dirty](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20261114T210752Z-266650fb-dirty/report.md)
- [20261116T075943Z-dea557ae](fiber_theory/fiber_theory_n8/macos-aarch64-c8-bare-apple/20261116T075943Z-dea557ae/report.md)

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

Latest: `20261116T080036Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| float_f32 | 255 ns | 1.00x |
| float_f64 | 318 ns | 1.24x |
| float_i64_scaled | 432 ns | 1.69x |

- [20260511T041156Z-e203deb](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20260511T041156Z-e203deb/report.md)
- [20261114T210846Z-266650fb-dirty](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20261114T210846Z-266650fb-dirty/report.md)
- [20261116T080036Z-dea557ae](float_width/float_width_n1024/macos-aarch64-c8-bare-apple/20261116T080036Z-dea557ae/report.md)

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

Latest: `20261116T080036Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| float_f32 | 5.15 us | 1.00x |
| float_f64 | 5.23 us | 1.01x |
| float_i64_scaled | 6.55 us | 1.27x |

- [20260511T041159Z-e203deb](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20260511T041159Z-e203deb/report.md)
- [20261114T210851Z-266650fb-dirty](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20261114T210851Z-266650fb-dirty/report.md)
- [20261116T080036Z-dea557ae](float_width/float_width_n16384/macos-aarch64-c8-bare-apple/20261116T080036Z-dea557ae/report.md)

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

Latest: `20261116T080036Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| float_f32 | 47 ns | 1.00x |
| float_f64 | 76 ns | 1.61x |
| float_i64_scaled | 122 ns | 2.60x |

- [20260511T041155Z-e203deb](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20260511T041155Z-e203deb/report.md)
- [20261114T210843Z-266650fb-dirty](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20261114T210843Z-266650fb-dirty/report.md)
- [20261116T080036Z-dea557ae](float_width/float_width_n256/macos-aarch64-c8-bare-apple/20261116T080036Z-dea557ae/report.md)

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

Latest: `20261116T080036Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| float_f32 | 1.23 us | 1.00x |
| float_f64 | 1.30 us | 1.05x |
| float_i64_scaled | 1.65 us | 1.34x |

- [20260511T041157Z-e203deb](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20260511T041157Z-e203deb/report.md)
- [20261114T210848Z-266650fb-dirty](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20261114T210848Z-266650fb-dirty/report.md)
- [20261116T080036Z-dea557ae](float_width/float_width_n4096/macos-aarch64-c8-bare-apple/20261116T080036Z-dea557ae/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n1024

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T044118Z-266650fb`

| variant | median | vs best |
|---|---|---|
| fpt_table_16 | 178 ns | 1.00x |

- [20261115T044117Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/linux-x86_64-c16-bare-intel/20261115T044117Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T080118Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 213 ns | 1.00x |
| fpt_table_16 | 325 ns | 1.52x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210922Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20261114T210922Z-266650fb-dirty/report.md)
- [20261116T080118Z-dea557ae](fnptr_table_dispatch/fnptr_table_dispatch_n1024/macos-aarch64-c8-bare-apple/20261116T080118Z-dea557ae/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044445Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 12.99 us | 1.00x |
| fpt_table_16 | 15.12 us | 1.16x |

- [20261115T044445Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/linux-x86_64-c16-bare-intel/20261115T044445Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T080118Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 10.09 us | 1.00x |
| fpt_table_16 | 11.38 us | 1.13x |

- [20260511T051112Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20260511T051112Z-9b76747/report.md)
- [20261114T210926Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20261114T210926Z-266650fb-dirty/report.md)
- [20261116T080118Z-dea557ae](fnptr_table_dispatch/fnptr_table_dispatch_n16384/macos-aarch64-c8-bare-apple/20261116T080118Z-dea557ae/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T043931Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 34 ns | 1.13x |
| fpt_table_16 | 30 ns | 1.00x |

- [20261115T043931Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/linux-x86_64-c16-bare-intel/20261115T043931Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T080118Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 65 ns | 1.47x |
| fpt_table_16 | 44 ns | 1.00x |

- [20260511T051109Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20260511T051109Z-9b76747/report.md)
- [20261114T210920Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20261114T210920Z-266650fb-dirty/report.md)
- [20261116T080118Z-dea557ae](fnptr_table_dispatch/fnptr_table_dispatch_n256/macos-aarch64-c8-bare-apple/20261116T080118Z-dea557ae/report.md)

## fnptr_table_dispatch/fnptr_table_dispatch_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T044301Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 590 ns | 1.00x |
| fpt_table_16 | 1.99 us | 3.36x |

- [20261115T044301Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/linux-x86_64-c16-bare-intel/20261115T044301Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T080118Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fpt_match_16 | 805 ns | 1.00x |
| fpt_table_16 | 1.27 us | 1.58x |

- [20260511T051110Z-9b76747](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20260511T051110Z-9b76747/report.md)
- [20261114T210923Z-266650fb-dirty](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20261114T210923Z-266650fb-dirty/report.md)
- [20261116T080118Z-dea557ae](fnptr_table_dispatch/fnptr_table_dispatch_n4096/macos-aarch64-c8-bare-apple/20261116T080118Z-dea557ae/report.md)

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

Latest: `20261116T080201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fold_paired | 69 ns | 1.28x |
| fold_quad | 54 ns | 1.00x |
| fold_sequential | 108 ns | 2.00x |

- [20260511T023054Z-7ed697d](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T023054Z-7ed697d/report.md)
- [20261114T211005Z-266650fb-dirty](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211005Z-266650fb-dirty/report.md)
- [20261116T080201Z-dea557ae](fold_strategy/fold_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T080201Z-dea557ae/report.md)

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

Latest: `20261116T080201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fold_paired | 1.29 us | 1.93x |
| fold_quad | 671 ns | 1.00x |
| fold_sequential | 2.55 us | 3.81x |

- [20260511T023056Z-7ed697d](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T023056Z-7ed697d/report.md)
- [20261114T211009Z-266650fb-dirty](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211009Z-266650fb-dirty/report.md)
- [20261116T080201Z-dea557ae](fold_strategy/fold_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T080201Z-dea557ae/report.md)

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

Latest: `20261116T080201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fold_paired | 10 ns | 1.04x |
| fold_quad | 10 ns | 1.00x |
| fold_sequential | 10 ns | 1.04x |

- [20260511T023053Z-7ed697d](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20260511T023053Z-7ed697d/report.md)
- [20261114T211003Z-266650fb-dirty](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211003Z-266650fb-dirty/report.md)
- [20261116T080201Z-dea557ae](fold_strategy/fold_strategy_n256/macos-aarch64-c8-bare-apple/20261116T080201Z-dea557ae/report.md)

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

Latest: `20261116T080201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fold_paired | 317 ns | 1.76x |
| fold_quad | 180 ns | 1.00x |
| fold_sequential | 600 ns | 3.32x |

- [20260511T023055Z-7ed697d](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T023055Z-7ed697d/report.md)
- [20261114T211007Z-266650fb-dirty](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211007Z-266650fb-dirty/report.md)
- [20261116T080201Z-dea557ae](fold_strategy/fold_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T080201Z-dea557ae/report.md)

## frame_wide/heavy_n1048576

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 9.73 ms | 3.18x |
| engine_par | 8.32 ms | 2.72x |
| hand | 13.69 ms | 4.47x |
| hand_par | 3.51 ms | 1.15x |
| rust_nostd | 9.57 ms | 3.12x |
| rust_std | 9.68 ms | 3.16x |
| rust_std_par | 3.06 ms | 1.00x |

- [20261116T080453Z-dea557ae](frame_wide/heavy_n1048576/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/heavy_n16777216

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 153.99 ms | 3.07x |
| engine_par | 130.13 ms | 2.59x |
| hand | 218.90 ms | 4.36x |
| hand_par | 52.50 ms | 1.05x |
| rust_nostd | 152.37 ms | 3.04x |
| rust_std | 154.40 ms | 3.08x |
| rust_std_par | 50.20 ms | 1.00x |

- [20261116T080453Z-dea557ae](frame_wide/heavy_n16777216/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/heavy_n4194304

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 38.97 ms | 2.84x |
| engine_par | 33.21 ms | 2.42x |
| hand | 55.06 ms | 4.01x |
| hand_par | 14.37 ms | 1.05x |
| rust_nostd | 38.32 ms | 2.79x |
| rust_std | 38.68 ms | 2.82x |
| rust_std_par | 13.73 ms | 1.00x |

- [20261116T080453Z-dea557ae](frame_wide/heavy_n4194304/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/heavy_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151355Z-c1eee3f5-dirty](frame_wide/heavy_n65536/linux-x86_64-c4-container-intel/20261113T151355Z-c1eee3f5-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 603.90 us | 2.76x |
| engine_par | 550.52 us | 2.52x |
| hand | 854.90 us | 3.91x |
| hand_par | 255.48 us | 1.17x |
| rust_nostd | 591.63 us | 2.71x |
| rust_std | 596.58 us | 2.73x |
| rust_std_par | 218.49 us | 1.00x |

- [20261116T080453Z-dea557ae](frame_wide/heavy_n65536/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/light_n1048576

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 1.86 ms | 7.69x |
| engine_par | 1.88 ms | 7.77x |
| hand | 271.49 us | 1.12x |
| hand_par | 241.71 us | 1.00x |
| rust_nostd | 1.71 ms | 7.07x |
| rust_std | 1.48 ms | 6.11x |
| rust_std_par | 1.25 ms | 5.17x |

- [20261116T080453Z-dea557ae](frame_wide/light_n1048576/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/light_n16777216

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 33.71 ms | 4.17x |
| engine_par | 32.55 ms | 4.02x |
| hand | 8.09 ms | 1.00x |
| hand_par | 8.39 ms | 1.04x |
| rust_nostd | 30.43 ms | 3.76x |
| rust_std | 30.39 ms | 3.76x |
| rust_std_par | 44.74 ms | 5.53x |

- [20261116T080453Z-dea557ae](frame_wide/light_n16777216/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/light_n4194304

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 8.53 ms | 4.03x |
| engine_par | 8.13 ms | 3.84x |
| hand | 2.12 ms | 1.00x |
| hand_par | 2.18 ms | 1.03x |
| rust_nostd | 8.17 ms | 3.85x |
| rust_std | 7.62 ms | 3.60x |
| rust_std_par | 10.59 ms | 5.00x |

- [20261116T080453Z-dea557ae](frame_wide/light_n4194304/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## frame_wide/light_n65536

### linux-x86_64-c4-container-intel, 1 run(s)

- [20261113T151715Z-c1eee3f5-dirty](frame_wide/light_n65536/linux-x86_64-c4-container-intel/20261113T151715Z-c1eee3f5-dirty/report.md)

### macos-aarch64-c8-bare-apple, 1 run(s)

Latest: `20261116T080453Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| engine | 79.70 us | 5.23x |
| engine_par | 106.45 us | 6.99x |
| hand | 15.24 us | 1.00x |
| hand_par | 46.88 us | 3.08x |
| rust_nostd | 71.67 us | 4.70x |
| rust_std | 72.64 us | 4.77x |
| rust_std_par | 78.53 us | 5.15x |

- [20261116T080453Z-dea557ae](frame_wide/light_n65536/macos-aarch64-c8-bare-apple/20261116T080453Z-dea557ae/report.md)

## fxpmul_strategy/fxpmul_strategy_n1024

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050123Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 97 ns | 1.00x |
| fxpmul_precise | 151 ns | 1.55x |

- [20261115T050123Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/linux-x86_64-c16-bare-intel/20261115T050123Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T082026Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 95 ns | 1.00x |
| fxpmul_precise | 110 ns | 1.16x |

- [20260510T213422Z-b042648](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213422Z-b042648/report.md)
- [20261114T211059Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211059Z-266650fb-dirty/report.md)
- [20261116T082026Z-dea557ae](fxpmul_strategy/fxpmul_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T082026Z-dea557ae/report.md)

## fxpmul_strategy/fxpmul_strategy_n16384

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050449Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.43 us | 1.00x |
| fxpmul_precise | 2.29 us | 1.60x |

- [20261115T050449Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/linux-x86_64-c16-bare-intel/20261115T050449Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T082026Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 1.62 us | 1.00x |
| fxpmul_precise | 1.94 us | 1.20x |

- [20260510T213424Z-b042648](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213424Z-b042648/report.md)
- [20261114T211102Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211102Z-266650fb-dirty/report.md)
- [20261116T082026Z-dea557ae](fxpmul_strategy/fxpmul_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T082026Z-dea557ae/report.md)

## fxpmul_strategy/fxpmul_strategy_n256

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045940Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 30 ns | 1.00x |
| fxpmul_precise | 44 ns | 1.44x |

- [20261115T045940Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/linux-x86_64-c16-bare-intel/20261115T045940Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T082026Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 13 ns | 1.00x |
| fxpmul_precise | 19 ns | 1.41x |

- [20260510T213421Z-b042648](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213421Z-b042648/report.md)
- [20261114T211058Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211058Z-266650fb-dirty/report.md)
- [20261116T082026Z-dea557ae](fxpmul_strategy/fxpmul_strategy_n256/macos-aarch64-c8-bare-apple/20261116T082026Z-dea557ae/report.md)

## fxpmul_strategy/fxpmul_strategy_n4096

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T050306Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 364 ns | 1.00x |
| fxpmul_precise | 578 ns | 1.59x |

- [20261115T050306Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T050306Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T082026Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 394 ns | 1.00x |
| fxpmul_precise | 477 ns | 1.21x |

- [20260510T213423Z-b042648](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213423Z-b042648/report.md)
- [20261114T211101Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211101Z-266650fb-dirty/report.md)
- [20261116T082026Z-dea557ae](fxpmul_strategy/fxpmul_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T082026Z-dea557ae/report.md)

## fxpmul_strategy/fxpmul_strategy_n64

### linux-x86_64-c16-bare-intel, 1 run(s)

Latest: `20261115T045753Z-266650fb-dirty`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 14 ns | 1.00x |
| fxpmul_precise | 17 ns | 1.25x |

- [20261115T045753Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/linux-x86_64-c16-bare-intel/20261115T045753Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T082026Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| fxpmul_hot | 5 ns | 1.00x |
| fxpmul_precise | 5 ns | 1.04x |

- [20260510T213420Z-b042648](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213420Z-b042648/report.md)
- [20261114T211056Z-266650fb-dirty](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211056Z-266650fb-dirty/report.md)
- [20261116T082026Z-dea557ae](fxpmul_strategy/fxpmul_strategy_n64/macos-aarch64-c8-bare-apple/20261116T082026Z-dea557ae/report.md)

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

Latest: `20261116T082110Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 1.25 us | 5.38x |
| hash_sip | 274 ns | 1.18x |
| hash_xxh | 233 ns | 1.00x |

- [20260510T213309Z-b042648](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20260510T213309Z-b042648/report.md)
- [20261114T211142Z-266650fb-dirty](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20261114T211142Z-266650fb-dirty/report.md)
- [20261116T082110Z-dea557ae](hash_algos/hash_algos_n1024/macos-aarch64-c8-bare-apple/20261116T082110Z-dea557ae/report.md)

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

Latest: `20261116T082110Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 20.46 us | 4.43x |
| hash_sip | 4.62 us | 1.00x |
| hash_xxh | 4.75 us | 1.03x |

- [20260510T213314Z-b042648](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20260510T213314Z-b042648/report.md)
- [20261114T211148Z-266650fb-dirty](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20261114T211148Z-266650fb-dirty/report.md)
- [20261116T082110Z-dea557ae](hash_algos/hash_algos_n16384/macos-aarch64-c8-bare-apple/20261116T082110Z-dea557ae/report.md)

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

Latest: `20261116T082110Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 272 ns | 8.60x |
| hash_sip | 58 ns | 1.84x |
| hash_xxh | 32 ns | 1.00x |

- [20260510T213307Z-b042648](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20260510T213307Z-b042648/report.md)
- [20261114T211140Z-266650fb-dirty](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20261114T211140Z-266650fb-dirty/report.md)
- [20261116T082110Z-dea557ae](hash_algos/hash_algos_n256/macos-aarch64-c8-bare-apple/20261116T082110Z-dea557ae/report.md)

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

Latest: `20261116T082110Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 5.16 us | 4.74x |
| hash_sip | 1.14 us | 1.05x |
| hash_xxh | 1.09 us | 1.00x |

- [20260510T213311Z-b042648](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20260510T213311Z-b042648/report.md)
- [20261114T211145Z-266650fb-dirty](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20261114T211145Z-266650fb-dirty/report.md)
- [20261116T082110Z-dea557ae](hash_algos/hash_algos_n4096/macos-aarch64-c8-bare-apple/20261116T082110Z-dea557ae/report.md)

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

Latest: `20261116T082110Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| hash_fnv1a | 36 ns | 9.68x |
| hash_sip | 8 ns | 2.14x |
| hash_xxh | 4 ns | 1.00x |

- [20260510T213306Z-b042648](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20260510T213306Z-b042648/report.md)
- [20261114T211138Z-266650fb-dirty](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20261114T211138Z-266650fb-dirty/report.md)
- [20261116T082110Z-dea557ae](hash_algos/hash_algos_n64/macos-aarch64-c8-bare-apple/20261116T082110Z-dea557ae/report.md)

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

Latest: `20261116T082201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inline_always | 1.25 us | 1.00x |
| inline_default | 1.25 us | 1.00x |
| inline_never | 1.31 us | 1.04x |

- [20260511T003101Z-63d18f0](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T003101Z-63d18f0/report.md)
- [20261114T211226Z-266650fb-dirty](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211226Z-266650fb-dirty/report.md)
- [20261116T082201Z-dea557ae](inline_strategy/inline_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T082201Z-dea557ae/report.md)

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

Latest: `20261116T082201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inline_always | 20.85 us | 1.00x |
| inline_default | 20.84 us | 1.00x |
| inline_never | 20.93 us | 1.00x |

- [20260511T003107Z-63d18f0](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T003107Z-63d18f0/report.md)
- [20261114T211234Z-266650fb-dirty](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211234Z-266650fb-dirty/report.md)
- [20261116T082201Z-dea557ae](inline_strategy/inline_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T082201Z-dea557ae/report.md)

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

Latest: `20261116T082201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inline_always | 273 ns | 1.00x |
| inline_default | 274 ns | 1.00x |
| inline_never | 324 ns | 1.19x |

- [20260511T003100Z-63d18f0](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20260511T003100Z-63d18f0/report.md)
- [20261114T211224Z-266650fb-dirty](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211224Z-266650fb-dirty/report.md)
- [20261116T082201Z-dea557ae](inline_strategy/inline_strategy_n256/macos-aarch64-c8-bare-apple/20261116T082201Z-dea557ae/report.md)

## inline_strategy/inline_strategy_n4096

### linux-x86_64-c16-bare-intel, 2 run(s)

Latest: `20261115T052615Z-266650fb`

| variant | median | vs best |
|---|---|---|
| inline_never | 4.57 us | 1.00x |

- [20261115T052614Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/linux-x86_64-c16-bare-intel/20261115T052614Z-266650fb-dirty/report.md)

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T082201Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| inline_always | 5.16 us | 1.00x |
| inline_default | 5.17 us | 1.00x |
| inline_never | 5.25 us | 1.02x |

- [20260511T003103Z-63d18f0](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T003103Z-63d18f0/report.md)
- [20261114T211229Z-266650fb-dirty](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211229Z-266650fb-dirty/report.md)
- [20261116T082201Z-dea557ae](inline_strategy/inline_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T082201Z-dea557ae/report.md)

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

Latest: `20261116T082251Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| iter_gather | 255 ns | 68.81x |
| iter_seq | 110 ns | 29.84x |
| iter_strided | 4 ns | 1.00x |

- [20260510T231055Z-b4fbd1a](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20260510T231055Z-b4fbd1a/report.md)
- [20261114T211312Z-266650fb-dirty](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20261114T211312Z-266650fb-dirty/report.md)
- [20261116T082251Z-dea557ae](iter_patterns/iter_patterns_n1024/macos-aarch64-c8-bare-apple/20261116T082251Z-dea557ae/report.md)

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

Latest: `20261116T082251Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| iter_gather | 4.67 us | 16.95x |
| iter_seq | 2.56 us | 9.28x |
| iter_strided | 276 ns | 1.00x |

- [20260510T231058Z-b4fbd1a](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20260510T231058Z-b4fbd1a/report.md)
- [20261114T211316Z-266650fb-dirty](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20261114T211316Z-266650fb-dirty/report.md)
- [20261116T082251Z-dea557ae](iter_patterns/iter_patterns_n16384/macos-aarch64-c8-bare-apple/20261116T082251Z-dea557ae/report.md)

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

Latest: `20261116T082251Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| iter_gather | 44 ns | 55.25x |
| iter_seq | 10 ns | 13.00x |
| iter_strided | 1 ns | 1.00x |

- [20260510T231054Z-b4fbd1a](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20260510T231054Z-b4fbd1a/report.md)
- [20261114T211310Z-266650fb-dirty](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20261114T211310Z-266650fb-dirty/report.md)
- [20261116T082251Z-dea557ae](iter_patterns/iter_patterns_n256/macos-aarch64-c8-bare-apple/20261116T082251Z-dea557ae/report.md)

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

Latest: `20261116T082251Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| iter_gather | 1.12 us | 21.33x |
| iter_seq | 603 ns | 11.52x |
| iter_strided | 52 ns | 1.00x |

- [20260510T231056Z-b4fbd1a](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20260510T231056Z-b4fbd1a/report.md)
- [20261114T211314Z-266650fb-dirty](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20261114T211314Z-266650fb-dirty/report.md)
- [20261116T082251Z-dea557ae](iter_patterns/iter_patterns_n4096/macos-aarch64-c8-bare-apple/20261116T082251Z-dea557ae/report.md)

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

Latest: `20261116T082337Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| load_aligned | 110 ns | 1.01x |
| load_byte_pack | 109 ns | 1.00x |
| load_unaligned | 109 ns | 1.00x |

- [20260511T001004Z-1fc2e8d](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20260511T001004Z-1fc2e8d/report.md)
- [20261114T211354Z-266650fb-dirty](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20261114T211354Z-266650fb-dirty/report.md)
- [20261116T082337Z-dea557ae](load_alignment/load_alignment_n1024/macos-aarch64-c8-bare-apple/20261116T082337Z-dea557ae/report.md)

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

Latest: `20261116T082337Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| load_aligned | 2.56 us | 1.00x |
| load_byte_pack | 2.56 us | 1.00x |
| load_unaligned | 2.56 us | 1.00x |

- [20260511T001006Z-1fc2e8d](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20260511T001006Z-1fc2e8d/report.md)
- [20261114T211359Z-266650fb-dirty](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20261114T211359Z-266650fb-dirty/report.md)
- [20261116T082337Z-dea557ae](load_alignment/load_alignment_n16384/macos-aarch64-c8-bare-apple/20261116T082337Z-dea557ae/report.md)

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

Latest: `20261116T082337Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| load_aligned | 11 ns | 1.00x |
| load_byte_pack | 11 ns | 1.00x |
| load_unaligned | 11 ns | 1.00x |

- [20260511T001003Z-1fc2e8d](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20260511T001003Z-1fc2e8d/report.md)
- [20261114T211352Z-266650fb-dirty](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20261114T211352Z-266650fb-dirty/report.md)
- [20261116T082337Z-dea557ae](load_alignment/load_alignment_n256/macos-aarch64-c8-bare-apple/20261116T082337Z-dea557ae/report.md)

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

Latest: `20261116T082337Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| load_aligned | 602 ns | 1.00x |
| load_byte_pack | 602 ns | 1.00x |
| load_unaligned | 602 ns | 1.00x |

- [20260511T001005Z-1fc2e8d](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20260511T001005Z-1fc2e8d/report.md)
- [20261114T211357Z-266650fb-dirty](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20261114T211357Z-266650fb-dirty/report.md)
- [20261116T082337Z-dea557ae](load_alignment/load_alignment_n4096/macos-aarch64-c8-bare-apple/20261116T082337Z-dea557ae/report.md)

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

Latest: `20261116T082613Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 100 ns | 1.00x |
| match_64_arms | 237 ns | 2.38x |
| match_8_arms | 160 ns | 1.61x |

- [20260511T035129Z-4802e6f](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20260511T035129Z-4802e6f/report.md)
- [20261114T211437Z-266650fb-dirty](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20261114T211437Z-266650fb-dirty/report.md)
- [20261116T082613Z-dea557ae](match_arm_count/match_arm_count_n1024/macos-aarch64-c8-bare-apple/20261116T082613Z-dea557ae/report.md)

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

Latest: `20261116T082613Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 1.38 us | 1.00x |
| match_64_arms | 12.88 us | 9.34x |
| match_8_arms | 2.52 us | 1.83x |

- [20260511T035132Z-4802e6f](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20260511T035132Z-4802e6f/report.md)
- [20261114T211443Z-266650fb-dirty](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20261114T211443Z-266650fb-dirty/report.md)
- [20261116T082613Z-dea557ae](match_arm_count/match_arm_count_n16384/macos-aarch64-c8-bare-apple/20261116T082613Z-dea557ae/report.md)

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

Latest: `20261116T082613Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 20 ns | 1.00x |
| match_64_arms | 71 ns | 3.54x |
| match_8_arms | 95 ns | 4.76x |

- [20260511T035128Z-4802e6f](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20260511T035128Z-4802e6f/report.md)
- [20261114T211435Z-266650fb-dirty](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20261114T211435Z-266650fb-dirty/report.md)
- [20261116T082613Z-dea557ae](match_arm_count/match_arm_count_n256/macos-aarch64-c8-bare-apple/20261116T082613Z-dea557ae/report.md)

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

Latest: `20261116T082613Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| match_2_arms | 355 ns | 1.00x |
| match_64_arms | 894 ns | 2.52x |
| match_8_arms | 632 ns | 1.78x |

- [20260511T035130Z-4802e6f](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20260511T035130Z-4802e6f/report.md)
- [20261114T211439Z-266650fb-dirty](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20261114T211439Z-266650fb-dirty/report.md)
- [20261116T082613Z-dea557ae](match_arm_count/match_arm_count_n4096/macos-aarch64-c8-bare-apple/20261116T082613Z-dea557ae/report.md)

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

Latest: `20261116T082654Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 1.45 us | 1.00x |
| memcpy_naive | 1.45 us | 1.00x |
| memcpy_unrolled | 1.45 us | 1.00x |

- [20260510T214729Z-f222646](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20260510T214729Z-f222646/report.md)
- [20261114T211523Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20261114T211523Z-266650fb-dirty/report.md)
- [20261116T082654Z-dea557ae](memcpy_patterns/memcpy_patterns_n1024/macos-aarch64-c8-bare-apple/20261116T082654Z-dea557ae/report.md)

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

Latest: `20261116T082654Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 20.82 us | 1.00x |
| memcpy_naive | 20.83 us | 1.00x |
| memcpy_unrolled | 20.82 us | 1.00x |

- [20260510T214734Z-f222646](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20260510T214734Z-f222646/report.md)
- [20261114T211530Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20261114T211530Z-266650fb-dirty/report.md)
- [20261116T082654Z-dea557ae](memcpy_patterns/memcpy_patterns_n16384/macos-aarch64-c8-bare-apple/20261116T082654Z-dea557ae/report.md)

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

Latest: `20261116T082654Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 475 ns | 1.00x |
| memcpy_naive | 474 ns | 1.00x |
| memcpy_unrolled | 476 ns | 1.00x |

- [20260510T214728Z-f222646](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20260510T214728Z-f222646/report.md)
- [20261114T211521Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20261114T211521Z-266650fb-dirty/report.md)
- [20261116T082654Z-dea557ae](memcpy_patterns/memcpy_patterns_n256/macos-aarch64-c8-bare-apple/20261116T082654Z-dea557ae/report.md)

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

Latest: `20261116T082654Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 5.37 us | 1.00x |
| memcpy_naive | 5.37 us | 1.00x |
| memcpy_unrolled | 5.37 us | 1.00x |

- [20260510T214730Z-f222646](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20260510T214730Z-f222646/report.md)
- [20261114T211526Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20261114T211526Z-266650fb-dirty/report.md)
- [20261116T082654Z-dea557ae](memcpy_patterns/memcpy_patterns_n4096/macos-aarch64-c8-bare-apple/20261116T082654Z-dea557ae/report.md)

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

Latest: `20261116T082654Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| memcpy_intrinsic | 36 ns | 1.00x |
| memcpy_naive | 37 ns | 1.02x |
| memcpy_unrolled | 37 ns | 1.02x |

- [20260510T214727Z-f222646](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20260510T214727Z-f222646/report.md)
- [20261114T211518Z-266650fb-dirty](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20261114T211518Z-266650fb-dirty/report.md)
- [20261116T082654Z-dea557ae](memcpy_patterns/memcpy_patterns_n64/macos-aarch64-c8-bare-apple/20261116T082654Z-dea557ae/report.md)

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

Latest: `20261116T082741Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 40 ns | 1.00x |
| minmax_branchless | 41 ns | 1.02x |
| minmax_intrinsic | 40 ns | 1.00x |

- [20260511T043126Z-da11af0](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20260511T043126Z-da11af0/report.md)
- [20261114T211609Z-266650fb-dirty](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211609Z-266650fb-dirty/report.md)
- [20261116T082741Z-dea557ae](minmax_strategy/minmax_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T082741Z-dea557ae/report.md)

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

Latest: `20261116T082741Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 395 ns | 1.01x |
| minmax_branchless | 400 ns | 1.02x |
| minmax_intrinsic | 392 ns | 1.00x |

- [20260511T043128Z-da11af0](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20260511T043128Z-da11af0/report.md)
- [20261114T211613Z-266650fb-dirty](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211613Z-266650fb-dirty/report.md)
- [20261116T082741Z-dea557ae](minmax_strategy/minmax_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T082741Z-dea557ae/report.md)

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

Latest: `20261116T082741Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 20 ns | 1.03x |
| minmax_branchless | 20 ns | 1.04x |
| minmax_intrinsic | 19 ns | 1.00x |

- [20260511T043125Z-da11af0](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20260511T043125Z-da11af0/report.md)
- [20261114T211607Z-266650fb-dirty](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211607Z-266650fb-dirty/report.md)
- [20261116T082741Z-dea557ae](minmax_strategy/minmax_strategy_n256/macos-aarch64-c8-bare-apple/20261116T082741Z-dea557ae/report.md)

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

Latest: `20261116T082741Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| minmax_branch | 112 ns | 1.00x |
| minmax_branchless | 112 ns | 1.00x |
| minmax_intrinsic | 112 ns | 1.00x |

- [20260511T043127Z-da11af0](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20260511T043127Z-da11af0/report.md)
- [20261114T211611Z-266650fb-dirty](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211611Z-266650fb-dirty/report.md)
- [20261116T082741Z-dea557ae](minmax_strategy/minmax_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T082741Z-dea557ae/report.md)

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

Latest: `20261116T082820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 3.53 us | 2.23x |
| mod_pow2_const | 1.58 us | 1.00x |
| mod_var | 4.47 us | 2.83x |

- [20260510T234953Z-4ccf554](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T234953Z-4ccf554/report.md)
- [20261114T211652Z-266650fb-dirty](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211652Z-266650fb-dirty/report.md)
- [20261116T082820Z-dea557ae](modulo_strategy/modulo_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T082820Z-dea557ae/report.md)

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

Latest: `20261116T082820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 56.51 us | 2.21x |
| mod_pow2_const | 25.58 us | 1.00x |
| mod_var | 71.72 us | 2.80x |

- [20260510T235003Z-4ccf554](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T235003Z-4ccf554/report.md)
- [20261114T211705Z-266650fb-dirty](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211705Z-266650fb-dirty/report.md)
- [20261116T082820Z-dea557ae](modulo_strategy/modulo_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T082820Z-dea557ae/report.md)

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

Latest: `20261116T082820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 834 ns | 2.33x |
| mod_pow2_const | 359 ns | 1.00x |
| mod_var | 1.06 us | 2.94x |

- [20260510T234952Z-4ccf554](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20260510T234952Z-4ccf554/report.md)
- [20261114T211650Z-266650fb-dirty](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211650Z-266650fb-dirty/report.md)
- [20261116T082820Z-dea557ae](modulo_strategy/modulo_strategy_n256/macos-aarch64-c8-bare-apple/20261116T082820Z-dea557ae/report.md)

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

Latest: `20261116T082820Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| mod_npow2_const | 14.09 us | 2.18x |
| mod_pow2_const | 6.47 us | 1.00x |
| mod_var | 17.88 us | 2.76x |

- [20260510T234956Z-4ccf554](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T234956Z-4ccf554/report.md)
- [20261114T211656Z-266650fb-dirty](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211656Z-266650fb-dirty/report.md)
- [20261116T082820Z-dea557ae](modulo_strategy/modulo_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T082820Z-dea557ae/report.md)

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

Latest: `20261116T082908Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 152 ns | 4.94x |
| popcount_std | 31 ns | 1.00x |
| popcount_swar | 152 ns | 4.93x |

- [20260510T213414Z-b042648](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20260510T213414Z-b042648/report.md)
- [20261114T211746Z-266650fb-dirty](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20261114T211746Z-266650fb-dirty/report.md)
- [20261116T082908Z-dea557ae](popcount_strategy/popcount_strategy_n1024/macos-aarch64-c8-bare-apple/20261116T082908Z-dea557ae/report.md)

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

Latest: `20261116T082908Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 1.99 us | 7.19x |
| popcount_std | 276 ns | 1.00x |
| popcount_swar | 1.99 us | 7.20x |

- [20260510T213418Z-b042648](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20260510T213418Z-b042648/report.md)
- [20261114T211751Z-266650fb-dirty](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20261114T211751Z-266650fb-dirty/report.md)
- [20261116T082908Z-dea557ae](popcount_strategy/popcount_strategy_n16384/macos-aarch64-c8-bare-apple/20261116T082908Z-dea557ae/report.md)

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

Latest: `20261116T082908Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 40 ns | 10.92x |
| popcount_std | 4 ns | 1.00x |
| popcount_swar | 41 ns | 11.03x |

- [20260510T213413Z-b042648](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20260510T213413Z-b042648/report.md)
- [20261114T211744Z-266650fb-dirty](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20261114T211744Z-266650fb-dirty/report.md)
- [20261116T082908Z-dea557ae](popcount_strategy/popcount_strategy_n256/macos-aarch64-c8-bare-apple/20261116T082908Z-dea557ae/report.md)

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

Latest: `20261116T082908Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 522 ns | 5.92x |
| popcount_std | 88 ns | 1.00x |
| popcount_swar | 519 ns | 5.87x |

- [20260510T213416Z-b042648](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20260510T213416Z-b042648/report.md)
- [20261114T211749Z-266650fb-dirty](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20261114T211749Z-266650fb-dirty/report.md)
- [20261116T082908Z-dea557ae](popcount_strategy/popcount_strategy_n4096/macos-aarch64-c8-bare-apple/20261116T082908Z-dea557ae/report.md)

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

Latest: `20261116T082908Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| popcount_handrolled | 5 ns | 13.00x |
| popcount_std | 0 ns | 1.00x |
| popcount_swar | 6 ns | 15.50x |

- [20260510T213412Z-b042648](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20260510T213412Z-b042648/report.md)
- [20261114T211742Z-266650fb-dirty](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20261114T211742Z-266650fb-dirty/report.md)
- [20261116T082908Z-dea557ae](popcount_strategy/popcount_strategy_n64/macos-aarch64-c8-bare-apple/20261116T082908Z-dea557ae/report.md)

## rcm_order/rcm_order_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 1.86 ms | 1.02x |
| rcm_half | 1.92 ms | 1.05x |
| rcm_rev | 1.82 ms | 1.00x |
| rcm_scr | 2.01 ms | 1.11x |

- [20260719T150703Z-2c337b67-dirty](rcm_order/rcm_order_n1024/macos-aarch64-c8-bare-apple/20260719T150703Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n1024/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_order/rcm_order_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 34.82 ms | 1.00x |
| rcm_half | 34.72 ms | 1.00x |
| rcm_rev | 36.49 ms | 1.05x |
| rcm_scr | 35.08 ms | 1.01x |

- [20260719T160923Z-2c337b67-dirty](rcm_order/rcm_order_n16384/macos-aarch64-c8-bare-apple/20260719T160923Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n16384/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_order/rcm_order_n2048

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 4.07 ms | 1.00x |
| rcm_half | 4.13 ms | 1.02x |
| rcm_rev | 4.06 ms | 1.00x |
| rcm_scr | 4.18 ms | 1.03x |

- [20260719T151110Z-2c337b67-dirty](rcm_order/rcm_order_n2048/macos-aarch64-c8-bare-apple/20260719T151110Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n2048/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_order/rcm_order_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 354.18 us | 1.04x |
| rcm_half | 340.33 us | 1.00x |
| rcm_rev | 357.02 us | 1.05x |
| rcm_scr | 350.79 us | 1.03x |

- [20260719T150459Z-2c337b67-dirty](rcm_order/rcm_order_n256/macos-aarch64-c8-bare-apple/20260719T150459Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n256/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_order/rcm_order_n4096

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 8.59 ms | 1.00x |
| rcm_half | 8.58 ms | 1.00x |
| rcm_rev | 8.63 ms | 1.01x |
| rcm_scr | 8.59 ms | 1.00x |

- [20260719T151919Z-2c337b67-dirty](rcm_order/rcm_order_n4096/macos-aarch64-c8-bare-apple/20260719T151919Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n4096/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_order/rcm_order_n64

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 80.88 us | 1.00x |
| rcm_half | 81.25 us | 1.00x |
| rcm_rev | 81.20 us | 1.00x |
| rcm_scr | 81.03 us | 1.00x |

- [20260719T150428Z-2c337b67-dirty](rcm_order/rcm_order_n64/macos-aarch64-c8-bare-apple/20260719T150428Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n64/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_order/rcm_order_n8192

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T083028Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| rcm_adj | 17.36 ms | 1.00x |
| rcm_half | 17.39 ms | 1.00x |
| rcm_rev | 17.48 ms | 1.01x |
| rcm_scr | 17.32 ms | 1.00x |

- [20260719T153530Z-2c337b67-dirty](rcm_order/rcm_order_n8192/macos-aarch64-c8-bare-apple/20260719T153530Z-2c337b67-dirty/report.md)
- [20261116T083028Z-dea557ae](rcm_order/rcm_order_n8192/macos-aarch64-c8-bare-apple/20261116T083028Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n1024

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 3.84 ms | 1.03x |
| riv_rowm | 3.76 ms | 1.01x |
| riv_scr | 3.82 ms | 1.03x |
| riv_snake | 3.72 ms | 1.00x |
| riv_spec | 3.78 ms | 1.02x |

- [20260719T161426Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n1024/macos-aarch64-c8-bare-apple/20260719T161426Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n1024/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n16384

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 190.75 ms | 2.00x |
| riv_rowm | 106.86 ms | 1.12x |
| riv_scr | 101.40 ms | 1.06x |
| riv_snake | 95.34 ms | 1.00x |
| riv_spec | 102.56 ms | 1.08x |

- [20260719T181527Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n16384/macos-aarch64-c8-bare-apple/20260719T181527Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n16384/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n2048

### macos-aarch64-c8-bare-apple, 3 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 7.97 ms | 1.02x |
| riv_rowm | 7.85 ms | 1.00x |
| riv_scr | 7.95 ms | 1.01x |
| riv_snake | 7.91 ms | 1.01x |
| riv_spec | 7.98 ms | 1.02x |

- [20260719T162210Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n2048/macos-aarch64-c8-bare-apple/20260719T162210Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n2048/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n256

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 830.96 us | 1.02x |
| riv_rowm | 830.39 us | 1.02x |
| riv_scr | 835.81 us | 1.03x |
| riv_snake | 812.33 us | 1.00x |
| riv_spec | 817.16 us | 1.01x |

- [20260719T161037Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n256/macos-aarch64-c8-bare-apple/20260719T161037Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n256/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n4096

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 15.94 ms | 1.00x |
| riv_rowm | 15.94 ms | 1.00x |
| riv_scr | 16.00 ms | 1.00x |
| riv_snake | 15.97 ms | 1.00x |
| riv_spec | 16.05 ms | 1.01x |

- [20260719T163719Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n4096/macos-aarch64-c8-bare-apple/20260719T163719Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n4096/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n64

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 184.51 us | 1.01x |
| riv_rowm | 184.26 us | 1.00x |
| riv_scr | 184.34 us | 1.00x |
| riv_snake | 184.42 us | 1.00x |
| riv_spec | 183.59 us | 1.00x |

- [20260719T160939Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n64/macos-aarch64-c8-bare-apple/20260719T160939Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n64/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

## rcm_rivals/rcm_rivals_n8192

### macos-aarch64-c8-bare-apple, 2 run(s)

Latest: `20261116T093842Z-dea557ae`

| variant | median | vs best |
|---|---|---|
| riv_rcm | 39.43 ms | 1.00x |
| riv_rowm | 40.76 ms | 1.03x |
| riv_scr | 39.67 ms | 1.01x |
| riv_snake | 39.56 ms | 1.00x |
| riv_spec | 39.84 ms | 1.01x |

- [20260719T170734Z-2c337b67-dirty](rcm_rivals/rcm_rivals_n8192/macos-aarch64-c8-bare-apple/20260719T170734Z-2c337b67-dirty/report.md)
- [20261116T093842Z-dea557ae](rcm_rivals/rcm_rivals_n8192/macos-aarch64-c8-bare-apple/20261116T093842Z-dea557ae/report.md)

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
