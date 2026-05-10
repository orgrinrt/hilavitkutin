	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena19load_progress_arena
	.p2align	2
__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena19load_progress_arena:
	.cfi_startproc
	add	x8, x0, x1, lsl #3
	ldapr	x0, [x8]
	ret
	.cfi_endproc

	.globl	__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena20load_progress_direct
	.p2align	2
__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena20load_progress_direct:
	.cfi_startproc
	ldapr	x0, [x0]
	ret
	.cfi_endproc

	.globl	__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena20store_progress_arena
	.p2align	2
__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena20store_progress_arena:
	.cfi_startproc
	add	x8, x0, x1, lsl #3
	stlr	x2, [x8]
	ret
	.cfi_endproc

	.globl	__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena21store_progress_direct
	.p2align	2
__RNvCsjQt3ZRHswBt_29sketch_progress_counter_arena21store_progress_direct:
	.cfi_startproc
	stlr	x1, [x0]
	ret
	.cfi_endproc

.subsections_via_symbols
