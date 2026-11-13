# ASM-dispatch gate report

Written by `mock/benches/src/bin/asm_gate.rs` from the staged `asm_gate_fixtures` release build; `asm_gate --check` regenerates it and fails when this file differs. Check 1 gates every target but the control, which it must fail; checks 2 to 5 are reported only.

- target: `x86_64`
- toolchain: `rustc 1.98.0-nightly (57d06900f 2026-05-27)`
- disassembler: `GNU objdump (GNU Binutils for Ubuntu) 2.42`

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_accumulator`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (47 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 27 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 295 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_column_chain`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (6 to imports via the GOT: free, malloc, memcpy, syscall) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 30 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_dirty_gated`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (6 to imports via the GOT: free, malloc, memcpy, syscall) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 31 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_fused_chain`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (8 to imports via the GOT: clock_gettime, free, malloc, memcpy, syscall) |
| 2. indexed addressing on column loads | PASS | 20 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 58 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_parallel_fiber_windows`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (27 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 68 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 158 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_resource_snapshot`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (47 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 50 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 294 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_windowed_fibers`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (23 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 123 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 231 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `Scheduler::run mono #1`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (13 to imports via the GOT: clock_gettime, memcpy, memset) |
| 2. indexed addressing on column loads | PASS | 223 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 103 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `run_parallel worker body #1`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (17 to imports via the GOT: clock_gettime, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 160 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 189 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `control asm_gate_control_stored_pointer`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | FAIL | 1 indirect calls through a stored pointer, at +0x22 (0 more to imports via the GOT) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 6 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

