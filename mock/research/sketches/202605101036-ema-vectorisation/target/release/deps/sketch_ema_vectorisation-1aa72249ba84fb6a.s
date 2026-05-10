	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCslCglZAVJM4l_24sketch_ema_vectorisation16ema_update_batch
	.p2align	2
__RNvCslCglZAVJM4l_24sketch_ema_vectorisation16ema_update_batch:
	.cfi_startproc
	b	__RNvCslCglZAVJM4l_24sketch_ema_vectorisation21ema_update_batch_simd
	.cfi_endproc

	.globl	__RNvCslCglZAVJM4l_24sketch_ema_vectorisation21ema_update_batch_simd
	.p2align	2
__RNvCslCglZAVJM4l_24sketch_ema_vectorisation21ema_update_batch_simd:
	.cfi_startproc
	ldr	q0, [x0]
	ldr	q1, [x1]
	movi.4s	v2, #7
	mla.4s	v1, v0, v2
	ushr.4s	v0, v1, #3
	str	q0, [x0]
	ret
	.cfi_endproc

	.globl	__RNvCslCglZAVJM4l_24sketch_ema_vectorisation23ema_update_batch_scalar
	.p2align	2
__RNvCslCglZAVJM4l_24sketch_ema_vectorisation23ema_update_batch_scalar:
	.cfi_startproc
	ldp	w8, w9, [x0]
	ldp	w10, w11, [x1]
	sub	x10, x10, x8
	add	x8, x10, x8, lsl #3
	lsr	x8, x8, #3
	sub	x10, x11, x9
	add	x9, x10, x9, lsl #3
	lsr	x9, x9, #3
	stp	w8, w9, [x0]
	ldp	w8, w9, [x0, #8]
	ldp	w10, w11, [x1, #8]
	sub	x10, x10, x8
	add	x8, x10, x8, lsl #3
	lsr	x8, x8, #3
	sub	x10, x11, x9
	add	x9, x10, x9, lsl #3
	lsr	x9, x9, #3
	stp	w8, w9, [x0, #8]
	ret
	.cfi_endproc

.subsections_via_symbols
