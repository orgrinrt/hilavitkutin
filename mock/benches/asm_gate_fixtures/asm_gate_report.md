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
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (59 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 29 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 388 stack-relative accesses |
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
| 3. no stack-relative accesses in inner loop | FAIL | 33 stack-relative accesses |
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
| 3. no stack-relative accesses in inner loop | FAIL | 34 stack-relative accesses |
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
| 3. no stack-relative accesses in inner loop | FAIL | 78 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_parallel_fiber_windows`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (58 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 78 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 302 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_resource_snapshot`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (59 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 58 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 395 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `fixture asm_gate_windowed_fibers`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (35 to imports via the GOT: clock_gettime, free, malloc, memcpy, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 139 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 238 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `Scheduler::run mono #1`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (13 to imports via the GOT: clock_gettime, memcpy, memset) |
| 2. indexed addressing on column loads | PASS | 235 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 102 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |

### variant `libasm_gate_fixtures`

- dylib: `libasm_gate_fixtures.so`
- symbol: `run_parallel worker body #1`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (36 to imports via the GOT: clock_gettime, memset, syscall) |
| 2. indexed addressing on column loads | PASS | 155 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 245 stack-relative accesses |
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

