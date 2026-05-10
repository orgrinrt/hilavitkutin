	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_10SequentialEB2_
	.p2align	2
__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_10SequentialEB2_:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	cbz	x2, LBB0_9
	mov	x8, x0
	mov	x0, #0
	mov	x15, #0
	add	x9, x8, #64
	b	LBB0_3
LBB0_2:
	mov	x15, x11
	cmp	x10, x2
	b.hs	LBB0_10
LBB0_3:
	add	x10, x15, #256
	cmp	x2, x10
	csel	x11, x2, x10, lo
	subs	x12, x11, x15
	b.ls	LBB0_2
	cmp	x15, x1
	csel	x8, x15, x1, hi
	sub	x13, x8, x15
	add	x14, x9, x15, lsl #3
	add	x15, x15, #8
LBB0_5:
	cmp	x15, x1
	b.hs	LBB0_7
	ldr	x16, [x14]
	add	x0, x16, x0
LBB0_7:
	cbz	x13, LBB0_11
	ldur	x16, [x14, #-64]
	add	x0, x16, x0
	sub	x13, x13, #1
	add	x14, x14, #8
	add	x15, x15, #1
	sub	x12, x12, #1
	cbnz	x12, LBB0_5
	b	LBB0_2
LBB0_9:
	mov	x0, #0
LBB0_10:
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB0_11:
	.cfi_restore_state
Lloh0:
	adrp	x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGE
Lloh1:
	add	x2, x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGEOFF
	mov	x0, x8
	bl	__RNvNtCs17GL9LZ7GE8_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh0, Lloh1
	.cfi_endproc

	.globl	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_7StridedEB2_
	.p2align	2
__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_7StridedEB2_:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov	x8, #0
	cbz	x2, LBB1_9
	mov	x9, #0
	b	LBB1_3
LBB1_2:
	mov	x9, x11
	cmp	x10, x2
	b.hs	LBB1_9
LBB1_3:
	add	x10, x9, #128
	cmp	x2, x10
	csel	x11, x2, x10, lo
	cmp	x9, x11
	b.hs	LBB1_2
	add	x12, x0, x9, lsl #3
	mov	x13, x10
LBB1_5:
	cmp	x13, x1
	b.hs	LBB1_7
	ldr	x9, [x12, #1024]
	add	x8, x9, x8
LBB1_7:
	sub	x9, x13, #128
	cmp	x9, x1
	b.hs	LBB1_10
	ldr	x9, [x12], #64
	add	x8, x9, x8
	sub	x9, x13, #120
	add	x13, x13, #8
	cmp	x9, x11
	b.lo	LBB1_5
	b	LBB1_2
LBB1_9:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB1_10:
	.cfi_restore_state
Lloh2:
	adrp	x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGE
Lloh3:
	add	x2, x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs17GL9LZ7GE8_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh2, Lloh3
	.cfi_endproc

	.globl	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_9PointwiseEB2_
	.p2align	2
__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_9PointwiseEB2_:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov	x8, #0
	cbz	x2, LBB2_7
	mov	x14, #0
	b	LBB2_3
LBB2_2:
	mov	x14, x11
	cmp	x10, x2
	b.hs	LBB2_7
LBB2_3:
	add	x10, x14, #512
	cmp	x2, x10
	csel	x11, x2, x10, lo
	subs	x12, x11, x14
	b.ls	LBB2_2
	cmp	x14, x1
	csel	x9, x14, x1, hi
	add	x13, x0, x14, lsl #3
	sub	x14, x9, x14
LBB2_5:
	cbz	x14, LBB2_8
	ldr	x15, [x13], #8
	add	x8, x8, x15, lsl #1
	sub	x14, x14, #1
	sub	x12, x12, #1
	cbnz	x12, LBB2_5
	b	LBB2_2
LBB2_7:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB2_8:
	.cfi_restore_state
Lloh4:
	adrp	x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGE
Lloh5:
	add	x2, x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs17GL9LZ7GE8_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh4, Lloh5
	.cfi_endproc

	.globl	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_9ScatteredEB2_
	.p2align	2
__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_9ScatteredEB2_:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	mov	x8, #0
	cbz	x2, LBB3_9
	mov	x9, #0
	b	LBB3_3
LBB3_2:
	mov	x9, x11
	cmp	x10, x2
	b.hs	LBB3_9
LBB3_3:
	add	x10, x9, #64
	cmp	x2, x10
	csel	x11, x2, x10, lo
	cmp	x9, x11
	b.hs	LBB3_2
	add	x13, x9, #2048
	add	x12, x0, x9, lsl #3
LBB3_5:
	cmp	x13, x1
	b.hs	LBB3_7
	ldr	x9, [x12, #16384]
	add	x8, x9, x8
LBB3_7:
	sub	x9, x13, #2048
	cmp	x9, x1
	b.hs	LBB3_10
	ldr	x9, [x12]
	add	x8, x9, x8
	sub	x9, x13, #1984
	add	x13, x13, #64
	add	x12, x12, #512
	cmp	x9, x11
	b.lo	LBB3_5
	b	LBB3_2
LBB3_9:
	mov	x0, x8
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB3_10:
	.cfi_restore_state
Lloh6:
	adrp	x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGE
Lloh7:
	add	x2, x2, l_anon.85049f8d85e85573e6ff82381c9fb736.1@PAGEOFF
	mov	x0, x9
	bl	__RNvNtCs17GL9LZ7GE8_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh6, Lloh7
	.cfi_endproc

	.globl	__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing12call_strided
	.p2align	2
__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing12call_strided:
	.cfi_startproc
	b	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_7StridedEB2_
	.cfi_endproc

	.globl	__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing14call_pointwise
	.p2align	2
__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing14call_pointwise:
	.cfi_startproc
	b	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_9PointwiseEB2_
	.cfi_endproc

	.globl	__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing14call_scattered
	.p2align	2
__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing14call_scattered:
	.cfi_startproc
	b	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_9ScatteredEB2_
	.cfi_endproc

	.globl	__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing15call_sequential
	.p2align	2
__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing15call_sequential:
	.cfi_startproc
	b	__RINvCsl3ZAkMZNWe4_24sketch_fibershape_typing18dispatch_per_shapeNtB2_10SequentialEB2_
	.cfi_endproc

	.globl	__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match
	.p2align	2
__RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match:
	.cfi_startproc
	cbz	x3, LBB8_9
	mov	x9, x0
	mov	x0, #0
	mov	x8, #0
	ubfiz	x11, x9, #3, #8
Lloh8:
	adrp	x9, l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match@PAGE
Lloh9:
	add	x9, x9, l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match@PAGEOFF
	ldr	x9, [x9, x11]
Lloh10:
	adrp	x10, l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match.1@PAGE
Lloh11:
	add	x10, x10, l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match.1@PAGEOFF
	ldr	x10, [x10, x11]
Lloh12:
	adrp	x12, l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match.2@PAGE
Lloh13:
	add	x12, x12, l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match.2@PAGEOFF
	ldr	x11, [x12, x11]
	add	x12, x1, x9, lsl #3
	add	x13, x8, x10
	cmp	x3, x13
	csel	x14, x3, x13, lo
	cmp	x8, x14
	b.lo	LBB8_4
LBB8_2:
	mov	x8, x14
	cmp	x13, x3
	b.hs	LBB8_8
	add	x13, x8, x10
	cmp	x3, x13
	csel	x14, x3, x13, lo
	cmp	x8, x14
	b.hs	LBB8_2
LBB8_4:
	add	x15, x9, x8
	cmp	x15, x2
	b.hs	LBB8_6
	ldr	x15, [x12, x8, lsl #3]
	add	x0, x15, x0
LBB8_6:
	cmp	x8, x2
	b.hs	LBB8_10
	ldr	x15, [x1, x8, lsl #3]
	add	x0, x15, x0
	add	x8, x8, x11
	cmp	x8, x14
	b.lo	LBB8_4
	b	LBB8_2
LBB8_8:
	ret
LBB8_9:
	mov	x0, #0
	ret
LBB8_10:
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
Lloh14:
	adrp	x9, l_anon.85049f8d85e85573e6ff82381c9fb736.2@PAGE
Lloh15:
	add	x9, x9, l_anon.85049f8d85e85573e6ff82381c9fb736.2@PAGEOFF
	mov	x0, x8
	mov	x1, x2
	mov	x2, x9
	bl	__RNvNtCs17GL9LZ7GE8_4core9panicking18panic_bounds_check
	.loh AdrpAdd	Lloh12, Lloh13
	.loh AdrpAdd	Lloh10, Lloh11
	.loh AdrpAdd	Lloh8, Lloh9
	.loh AdrpAdd	Lloh14, Lloh15
	.cfi_endproc

	.section	__TEXT,__cstring,cstring_literals
l_anon.85049f8d85e85573e6ff82381c9fb736.0:
	.asciz	"src/lib.rs"

	.section	__DATA,__const
	.p2align	3, 0x0
l_anon.85049f8d85e85573e6ff82381c9fb736.1:
	.quad	l_anon.85049f8d85e85573e6ff82381c9fb736.0
	.asciz	"\n\000\000\000\000\000\000\000X\000\000\000$\000\000"

	.p2align	3, 0x0
l_anon.85049f8d85e85573e6ff82381c9fb736.2:
	.quad	l_anon.85049f8d85e85573e6ff82381c9fb736.0
	.asciz	"\n\000\000\000\000\000\000\000\226\000\000\000$\000\000"

	.section	__TEXT,__const
	.p2align	3, 0x0
l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match:
	.quad	8
	.quad	128
	.quad	2048
	.quad	0

	.p2align	3, 0x0
l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match.1:
	.quad	256
	.quad	128
	.quad	64
	.quad	512

	.p2align	3, 0x0
l_switch.table._RNvCsl3ZAkMZNWe4_24sketch_fibershape_typing22dispatch_runtime_match.2:
	.quad	1
	.quad	8
	.quad	64
	.quad	1

.subsections_via_symbols
