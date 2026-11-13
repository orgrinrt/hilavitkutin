# Chain Shapes, An Ad-Hoc Spike

This is an ad-hoc spike written for the stack audit of 13 November 2026 (`mock/research/202611131215_stack-audit/`). It is no bench. It sits outside the bench harness, it uses a plain `std::time::Instant` loop, it ran on one shared cloud guest, and it has one kernel. Every number below is an ad-hoc spike number and has the standing of one instance. Nothing here is a measurement of hilavitkutin: no hilavitkutin crate is linked.

The question it was written for is narrow. The audit has to estimate how an engine built to the intended design would stand against hand-written code, and the estimate turns on a few orders of magnitude that the record does not hold for the region op named (millions of records). The spike gives those orders of magnitude for one kernel, so that the reasoning in the audit's `ESTIMATES.md` has something to point at beside the literature.

The kernel is a four-stage map over `u32` columns, three inputs and one output, the same family as `mock/benches/engine_vs_std`: `t1 = a*3 + b`, `t2 = t1 ^ (t1 >> 7)`, `t3 = t2*0x9E3779B1 + c`, `out = rotl(t3, 5) ^ a`. The arms are the shapes that kernel can take, and the driver refuses to time an arm whose output checksum differs from the first arm's.

## The arms

| Arm | What it is |
| --- | --- |
| `fused_slices` | one loop over slices, all four stages inlined, no intermediate stored |
| `fused_rawptr` | the same loop over four raw pointers read out of a struct, the way a context hands columns out |
| `iter_chain` | the same as a zipped iterator chain |
| `hand_avx2` | the same written by hand with AVX2 intrinsics, two vectors per step; the stand-in for hand-written assembly, and not assembly |
| `pass_per_stage` | one whole pass over the data per stage, the three intermediates stored as full columns |
| `morsel_columns_M` | morsel by morsel of M records, one loop per stage inside the morsel, intermediates stored as full columns |
| `morsel_scratch_M` | the same, intermediates in three reused scratch buffers of M records |
| `morsel_fnptr_M` | `morsel_columns_M` with each stage behind a function pointer called once per morsel |
| `call_per_record` | one call through a function pointer per record |
| `par4_fused` | `fused_slices` on four scoped threads over four slices, threads spawned per call |
| `par4_pass_per_stage` | the four stage loops with full intermediate columns, on four scoped threads |

## Where and how it ran

The host is a KVM guest with 4 virtual CPUs of an Intel Xeon (family 6 model 173), 48 KiB of L1d and a reported 8 MiB of L2 per core, Linux 6.18, rustc 1.97.0 with LLVM 22.1.6. The profile is release, `opt-level = 3`, fat LTO, one codegen unit. Two builds ran: the default `x86_64-unknown-linux-gnu` target (SSE2 baseline), output in `output-default.md`, and `-C target-cpu=native`, output in `output-native.md`. Each cell is nine samples of about 40 ms each; the tables hold the lowest and the median in nanoseconds per record. Nothing was pinned, nothing else was controlled, and neighbours on the host were not visible.

Run it again with `cargo run --release --manifest-path mock/research/sketches/202611131400_chain-shapes-adhoc/Cargo.toml`.

## What the two runs showed

Medians, as a multiple of `fused_slices` at the same size and build. The full tables are the two output files.

| Arm | 2^16, default | 2^16, native | 2^24, default | 2^24, native | 2^26, default | 2^26, native |
| --- | --: | --: | --: | --: | --: | --: |
| `fused_slices`, ns per record | 0.376 | 0.253 | 1.474 | 1.035 | 1.497 | 1.440 |
| `fused_rawptr` | 1.01x | 1.01x | 0.80x | 1.01x | 0.85x | 1.02x |
| `hand_avx2` | 0.78x | 0.83x | 0.74x | 0.92x | 0.73x | 0.80x |
| `pass_per_stage` | 2.23x | 1.99x | 2.80x | 3.02x | 2.51x | 2.22x |
| `morsel_columns_256` | 1.67x | 2.29x | 1.84x | 1.94x | 1.63x | 1.47x |
| `morsel_columns_4096` | 1.52x | 2.58x | 2.06x | 2.76x | 2.03x | 2.31x |
| `morsel_scratch_256` | 1.19x | 1.22x | 1.27x | 1.24x | 0.98x | 1.15x |
| `morsel_scratch_4096` | 1.24x | 1.78x | 1.06x | 1.70x | 1.26x | 1.11x |
| `morsel_fnptr_256` | 1.49x | 1.63x | 1.35x | 2.76x | 1.90x | 1.48x |
| `call_per_record` | 2.87x | 7.02x | 1.08x | 1.81x | 1.22x | 1.17x |
| `par4_fused` | 5.86x | 7.56x | 0.36x | 0.38x | 0.27x | 0.29x |
| `par4_pass_per_stage` | 6.76x | 13.11x | 0.76x | 1.00x | 1.14x | 0.82x |

Read with the spread of this host in mind: the same arm moved by up to about 40 percent between the two builds at sizes where the build should not matter (`fused_slices` at 2^24 is 1.474 against 1.035), and `fused_slices` itself ran slower than `iter_chain` and `fused_rawptr` at 2^20 and above in the default build for a reason this spike did not find. A ratio under about 1.4x in either direction is inside what the host does by itself.

With that said, five things stand above the spread in both builds.

1. Storing the intermediates as full columns costs 2x to 3x once the data is out of cache. `pass_per_stage` is 2.2x to 3.0x of the fused loop at 2^24 and 2^26, and the morsel-wise arm that still writes three intermediate columns is 1.5x to 2.8x. The kernel is memory-bound there: the fused loop moves 16 bytes per record and the stored-intermediate arms move 40.
2. Cutting the work into morsels does not by itself recover that. `morsel_columns_*` keeps the three extra write streams, so it stays near the per-stage arm. What recovers it is the intermediates not being columns: `morsel_scratch_256` is 0.64x to 1.27x of the fused loop at every size from 2^16 up.
3. A call through a function pointer once per morsel per stage is not the cost. `morsel_fnptr_256` sits within the spread of `morsel_columns_256` in five of the six columns. A call per record is the cost only while the data is in cache (2.9x and 7.0x at 2^16); out of cache it is 1.1x to 1.8x, because memory is the limit.
4. Four threads over one fused loop give 2.6x to 3.7x over one thread from 2^24 up. Four threads over the stored-intermediate shape give about one thread's worth of the fused loop back (0.76x to 1.14x). At 2^16 the per-call thread spawn makes both threaded arms 6x to 13x slower than one thread, which is the cost of `std::thread::scope`, not of threading.
5. Hand-written AVX2 against the compiler: 0.70x to 0.78x of the fused loop in the default build and 0.80x to 1.03x in the native build at 2^16 and above. For this kernel the hand-written vector code buys what the target features buy and little more.

## What it does not show

The kernel has three inputs and one output. It says nothing about loops over many columns, where the number of runtime overlap checks LLVM is willing to emit for raw pointers runs out and the loop stays scalar; `fused_rawptr` matching `fused_slices` here holds for four pointers and no further. It has no branch, no gather, no reduction, no append and no neighbour read. It has one record count. It has no incremental skip. It ran on x86_64 only, on a guest. It compares shapes written by hand; whether the engine's type-level machinery lowers to the fused shape, or to the scratch-morsel shape, is the engine's question and is not touched here.

Holds for: this kernel, `u32`, three read columns and one written; sizes 2^16 to 2^26; x86_64 KVM guest named above; release, fat LTO, one codegen unit; both target-feature settings named above; unpinned; ratios above about 1.4x only.
