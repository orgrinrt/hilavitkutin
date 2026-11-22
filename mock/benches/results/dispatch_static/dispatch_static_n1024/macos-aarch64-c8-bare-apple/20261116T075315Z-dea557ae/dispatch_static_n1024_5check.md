# 5-check disasm report: dispatch_static n=1024

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
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


### variant `libdispatch_trait`

- dylib: `libdispatch_trait.dylib`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (0 to imports via the GOT: none) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 4 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


### variant `libdispatch_const_ptr`

- dylib: `libdispatch_const_ptr.dylib`
- symbol: `bench_entry`
- status: FAIL

| Check | Result | Detail |
|---|---|---|
| 1. zero indirect call in inner body | PASS | 0 indirect calls through a stored pointer (0 to imports via the GOT: none) |
| 2. indexed addressing on column loads | FAIL | 0 indexed-addressing instructions |
| 3. no stack-relative accesses in inner loop | FAIL | 4 stack-relative accesses |
| 4. immediate-constant morsel size baked | PASS | found 1 of 1 expected immediates |
| 5. no direct call to morsel/fiber/wu_fn helpers | PASS | 0 direct calls to helper symbols |


