	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCseie9J0hk28f_27sketch_codegen_tait_capture15call_bench_test
	.p2align	2
__RNvCseie9J0hk28f_27sketch_codegen_tait_capture15call_bench_test:
	.cfi_startproc
	ldr	x8, [x0, #40]
	ldr	w9, [x8]
	cmn	x1, #256
	b.hs	LBB0_13
	mov	x10, #0
	ldp	x13, x12, [x0]
	mov	w14, #256
	ldp	x16, x15, [x0, #16]
	mov	x17, #31765
	movk	x17, #32586, lsl #16
	movk	x17, #31161, lsl #32
	movk	x17, #40503, lsl #48
	mov	x2, #58809
	movk	x2, #7396, lsl #16
	movk	x2, #18285, lsl #32
	movk	x2, #48984, lsl #48
	mov	x3, x9
	ldr	x11, [x0, #32]
	b	LBB0_3
LBB0_2:
	ldr	x4, [x11]
	add	x10, x0, x10
	add	x10, x10, x5
	add	x10, x10, x4
	add	x1, x1, #1
	subs	x14, x14, #1
	b.eq	LBB0_14
LBB0_3:
	cmp	x1, x12
	b.hs	LBB0_6
	ldr	x0, [x13, x1, lsl #3]
	mul	x0, x0, x17
	tbnz	w0, #0, LBB0_9
	add	w3, w3, #1
	str	w3, [x8]
	ror	x0, x0, #47
	cmp	x1, x15
	b.lo	LBB0_7
	b	LBB0_10
LBB0_6:
	mov	x0, #0
	cmp	x1, x15
	b.hs	LBB0_10
LBB0_7:
	ldr	w4, [x16, x1, lsl #2]
	madd	x4, x4, x2, x1
	mul	x0, x4, x0
	cmp	x1, x12
	b.lo	LBB0_11
LBB0_8:
	mov	x5, #0
	cmp	x1, x15
	b.hs	LBB0_2
	b	LBB0_12
LBB0_9:
	ror	x0, x0, #13
	cmp	x1, x15
	b.lo	LBB0_7
LBB0_10:
	mov	x0, #0
	cmp	x1, x12
	b.hs	LBB0_8
LBB0_11:
	ldr	x5, [x13, x1, lsl #3]
	cmp	x1, x15
	b.hs	LBB0_2
LBB0_12:
	ldr	w4, [x16, x1, lsl #2]
	eor	x5, x5, x4, lsl #11
	b	LBB0_2
LBB0_13:
	mov	x10, #0
	ldr	x11, [x0, #32]
	ldr	x4, [x11]
LBB0_14:
	add	x10, x10, x4
	str	x10, [x11]
	ldr	w10, [x8]
	lsl	w10, w10, #1
	sub	w9, w10, w9
	str	w9, [x8]
	ret
	.cfi_endproc

	.globl	__RNvCseie9J0hk28f_27sketch_codegen_tait_capture17call_standard_alt
	.p2align	2
__RNvCseie9J0hk28f_27sketch_codegen_tait_capture17call_standard_alt:
	.cfi_startproc
	cmn	x1, #128
	b.hs	LBB1_13
	mov	x8, #0
	ldp	x11, x10, [x0]
	ldp	x13, x12, [x0, #16]
	mov	w14, #128
	mov	x15, #31765
	movk	x15, #32586, lsl #16
	movk	x15, #31161, lsl #32
	movk	x15, #40503, lsl #48
	mov	x16, #58809
	movk	x16, #7396, lsl #16
	movk	x16, #18285, lsl #32
	movk	x16, #48984, lsl #48
	ldp	x9, x17, [x0, #32]
	b	LBB1_3
LBB1_2:
	ldr	x2, [x9]
	add	x3, x2, x3
	eor	x0, x3, x0
	add	x8, x0, x8
	add	x1, x1, #1
	subs	x14, x14, #1
	b.eq	LBB1_14
LBB1_3:
	cmp	x1, x10
	b.hs	LBB1_6
	ldr	x0, [x11, x1, lsl #3]
	mul	x0, x0, x15
	tbnz	w0, #0, LBB1_12
	ldr	w2, [x17]
	add	w2, w2, #1
	str	w2, [x17]
	ror	x0, x0, #47
	cmp	x1, x12
	b.lo	LBB1_7
	b	LBB1_8
LBB1_6:
	mov	x0, #0
	cmp	x1, x12
	b.hs	LBB1_8
LBB1_7:
	ldr	w2, [x13, x1, lsl #2]
	madd	x2, x2, x16, x1
	eor	x0, x2, x0
LBB1_8:
	cmp	x1, x10
	b.hs	LBB1_10
	ldr	x3, [x11, x1, lsl #3]
	cmp	x1, x12
	b.hs	LBB1_2
	b	LBB1_11
LBB1_10:
	mov	x3, #0
	cmp	x1, x12
	b.hs	LBB1_2
LBB1_11:
	ldr	w2, [x13, x1, lsl #2]
	eor	x3, x3, x2, lsl #11
	b	LBB1_2
LBB1_12:
	ror	x0, x0, #13
	cmp	x1, x12
	b.lo	LBB1_7
	b	LBB1_8
LBB1_13:
	mov	x8, #0
	ldr	x9, [x0, #32]
	ldr	x2, [x9]
LBB1_14:
	add	x8, x8, x2
	str	x8, [x9]
	ret
	.cfi_endproc

	.globl	__RNvCseie9J0hk28f_27sketch_codegen_tait_capture18call_standard_test
	.p2align	2
__RNvCseie9J0hk28f_27sketch_codegen_tait_capture18call_standard_test:
	.cfi_startproc
	cmn	x1, #256
	b.hs	LBB2_13
	mov	x8, #0
	ldp	x11, x10, [x0]
	ldp	x13, x12, [x0, #16]
	mov	w14, #256
	mov	x15, #31765
	movk	x15, #32586, lsl #16
	movk	x15, #31161, lsl #32
	movk	x15, #40503, lsl #48
	mov	x16, #58809
	movk	x16, #7396, lsl #16
	movk	x16, #18285, lsl #32
	movk	x16, #48984, lsl #48
	ldp	x9, x17, [x0, #32]
	b	LBB2_3
LBB2_2:
	ldr	x2, [x9]
	add	x3, x2, x3
	eor	x0, x3, x0
	add	x8, x0, x8
	add	x1, x1, #1
	subs	x14, x14, #1
	b.eq	LBB2_14
LBB2_3:
	cmp	x1, x10
	b.hs	LBB2_6
	ldr	x0, [x11, x1, lsl #3]
	mul	x0, x0, x15
	tbnz	w0, #0, LBB2_12
	ldr	w2, [x17]
	add	w2, w2, #1
	str	w2, [x17]
	ror	x0, x0, #47
	cmp	x1, x12
	b.lo	LBB2_7
	b	LBB2_8
LBB2_6:
	mov	x0, #0
	cmp	x1, x12
	b.hs	LBB2_8
LBB2_7:
	ldr	w2, [x13, x1, lsl #2]
	madd	x2, x2, x16, x1
	eor	x0, x2, x0
LBB2_8:
	cmp	x1, x10
	b.hs	LBB2_10
	ldr	x3, [x11, x1, lsl #3]
	cmp	x1, x12
	b.hs	LBB2_2
	b	LBB2_11
LBB2_10:
	mov	x3, #0
	cmp	x1, x12
	b.hs	LBB2_2
LBB2_11:
	ldr	w2, [x13, x1, lsl #2]
	eor	x3, x3, x2, lsl #11
	b	LBB2_2
LBB2_12:
	ror	x0, x0, #13
	cmp	x1, x12
	b.lo	LBB2_7
	b	LBB2_8
LBB2_13:
	mov	x8, #0
	ldr	x9, [x0, #32]
	ldr	x2, [x9]
LBB2_14:
	add	x8, x8, x2
	str	x8, [x9]
	ret
	.cfi_endproc

.subsections_via_symbols
