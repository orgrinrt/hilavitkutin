# 5-check disasm report: dispatch_dynamic n=16384

Canonical 5 checks (see `codegen_dispatch_axis_a_assertions.md`):

1. Zero indirect call in inner body.
2. Indexed addressing on column loads.
3. No stack-relative accesses inside the inner loop.
4. Immediate-constant morsel size baked in.
5. No direct call to morsel-size, fiber-dispatch, wu_fn helpers.

Status: AT LEAST ONE FAIL

### variant `libdispatch_direct`

- dylib: `libdispatch_direct.dylib`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (0 to imports via the GOT: none) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 4 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


### variant `libdispatch_volatile`

- dylib: `libdispatch_volatile.dylib`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | FAIL | 43 indirect calls through a stored pointer, at +0x78, +0xc0, +0xd0, +0xe0, +0xf0, +0x100, +0x110, +0x120, +0x130, +0x164, +0x1b0, +0x1fc, +0x20c, +0x21c, +0x22c, +0x23c, +0x24c, +0x25c, +0x26c, +0x27c, +0x28c, +0x29c, +0x2ac, +0x2bc, +0x2cc, +0x2dc, +0x2ec, +0x2fc, +0x30c, +0x31c, +0x32c, +0x33c, +0x34c, +0x35c, +0x36c, +0x37c, +0x38c, +0x39c, +0x3ac, +0x3bc, +0x3cc, +0x3dc, +0x3ec (0 more to imports via the GOT) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 10 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


### variant `libdispatch_runtime`

- dylib: `libdispatch_runtime.dylib`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | FAIL | 43 indirect calls through a stored pointer, at +0x80, +0xd0, +0xe4, +0xf8, +0x10c, +0x120, +0x134, +0x148, +0x15c, +0x198, +0x1ec, +0x240, +0x254, +0x268, +0x27c, +0x290, +0x2a4, +0x2b8, +0x2cc, +0x2e0, +0x2f4, +0x308, +0x31c, +0x330, +0x344, +0x358, +0x36c, +0x380, +0x394, +0x3a8, +0x3bc, +0x3d0, +0x3e4, +0x3f8, +0x40c, +0x420, +0x434, +0x448, +0x45c, +0x470, +0x484, +0x498, +0x4ac (0 more to imports via the GOT) |
| 2. indexed addressing on column loads | PASS | 43 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 10 stack-relative accesses |
| 4. immediate-constant morsel size baked | FAIL | found 0 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


