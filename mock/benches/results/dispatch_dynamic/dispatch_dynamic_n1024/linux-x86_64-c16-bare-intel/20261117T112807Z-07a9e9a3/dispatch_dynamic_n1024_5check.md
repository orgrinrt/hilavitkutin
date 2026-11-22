# 5-check disasm report: dispatch_dynamic n=1024

Canonical 5 checks (see `codegen_dispatch_axis_a_assertions.md`):

1. Zero indirect call in inner body.
2. Indexed addressing on column loads.
3. No stack-relative accesses inside the inner loop.
4. Immediate-constant morsel size baked in.
5. No direct call to morsel-size, fiber-dispatch, wu_fn helpers.

Status: AT LEAST ONE FAIL

### variant `libdispatch_direct`

- dylib: `libdispatch_direct.so`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (0 to imports via the GOT: none) |
| 2. indexed addressing on column loads | PASS | 13 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 4 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


### variant `libdispatch_volatile`

- dylib: `libdispatch_volatile.so`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | FAIL | 43 indirect calls through a stored pointer, at +0x6c, +0x7d, +0x8e, +0x9f, +0xb0, +0xc1, +0xd2, +0xe3, +0x12b, +0x171, +0x182, +0x193, +0x1a4, +0x1b5, +0x1c6, +0x1d7, +0x1e8, +0x1f9, +0x20a, +0x21b, +0x22c, +0x23d, +0x24e, +0x25f, +0x270, +0x284, +0x298, +0x2ac, +0x2c0, +0x2d4, +0x2e8, +0x2fc, +0x310, +0x324, +0x338, +0x34c, +0x360, +0x374, +0x388, +0x39c, +0x3b0, +0x3fb, +0x44b (0 more to imports via the GOT) |
| 2. indexed addressing on column loads | PASS | 4 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 4 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


### variant `libdispatch_runtime`

- dylib: `libdispatch_runtime.so`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | FAIL | 43 indirect calls through a stored pointer, at +0x71, +0x83, +0x95, +0xa7, +0xb9, +0xcb, +0xdd, +0xef, +0x139, +0x186, +0x198, +0x1aa, +0x1bc, +0x1ce, +0x1e0, +0x1f2, +0x204, +0x216, +0x228, +0x23a, +0x24c, +0x25e, +0x270, +0x282, +0x294, +0x2a9, +0x2be, +0x2d3, +0x2e8, +0x2fd, +0x312, +0x327, +0x33c, +0x351, +0x366, +0x37b, +0x390, +0x3a5, +0x3ba, +0x3cf, +0x3e4, +0x449, +0x4a9 (0 more to imports via the GOT) |
| 2. indexed addressing on column loads | PASS | 47 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 9 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


