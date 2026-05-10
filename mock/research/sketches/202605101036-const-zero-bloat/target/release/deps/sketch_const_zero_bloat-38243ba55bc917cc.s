	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	_read_const
	.p2align	2
_read_const:
	.cfi_startproc
	mov	x0, #48879
	movk	x0, #57005, lsl #16
	orr	x0, x0, x0, lsl #32
	ret
	.cfi_endproc

	.globl	_read_static
	.p2align	2
_read_static:
	.cfi_startproc
	mov	x0, #47806
	movk	x0, #51966, lsl #16
	orr	x0, x0, x0, lsl #32
	ret
	.cfi_endproc

	.section	__TEXT,__const
	.globl	_SKETCH_STATIC_DEFAULT_DECAY
	.p2align	3, 0x0
_SKETCH_STATIC_DEFAULT_DECAY:
	.ascii	"\276\272\376\312\276\272\376\312"

.subsections_via_symbols
