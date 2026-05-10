	.build_version macos, 11, 0
	.section	__TEXT,__text,regular,pure_instructions
	.globl	__RNvCs133NfS5DO5d_19sketch_codegen_tait18call_through_trait
	.p2align	2
__RNvCs133NfS5DO5d_19sketch_codegen_tait18call_through_trait:
	.cfi_startproc
	cbz	x0, LBB0_2
	sub	x8, x0, #1
	sub	x9, x0, #2
	umulh	x10, x8, x9
	mul	x8, x8, x9
	sub	x9, x0, #3
	mul	x9, x8, x9
	lsr	x9, x9, #1
	mov	x11, #6148914691236517205
	movk	x11, #21850
	extr	x8, x10, x8, #1
	mov	w10, #35
	mul	x8, x8, x10
	madd	x8, x9, x11, x8
	mov	w9, #21
	madd	x8, x0, x9, x8
	sub	x0, x8, #21
LBB0_2:
	ret
	.cfi_endproc

	.globl	__RNvCs133NfS5DO5d_19sketch_codegen_tait25call_through_struct_field
	.p2align	2
__RNvCs133NfS5DO5d_19sketch_codegen_tait25call_through_struct_field:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x22, x21, [sp, #16]
	stp	x20, x19, [sp, #32]
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_offset w19, -24
	.cfi_offset w20, -32
	.cfi_offset w21, -40
	.cfi_offset w22, -48
	cbz	x0, LBB1_3
	mov	x20, x0
	mov	x19, #0
	mov	x21, #0
	ldr	x22, [x1]
LBB1_2:
	stp	x21, xzr, [sp]
	mov	x0, sp
	blr	x22
	add	x19, x0, x19
	add	x21, x21, #1
	cmp	x20, x21
	b.ne	LBB1_2
	b	LBB1_4
LBB1_3:
	mov	x19, #0
LBB1_4:
	mov	x0, x19
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	ldp	x20, x19, [sp, #32]
	ldp	x22, x21, [sp, #16]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	.cfi_restore w19
	.cfi_restore w20
	.cfi_restore w21
	.cfi_restore w22
	ret
	.cfi_endproc

	.globl	__RNvXs0_Cs133NfS5DO5d_19sketch_codegen_taitNtB5_15StandardCodegenINtB5_15DispatchCodegenNtB5_5MyCfgE5buildB5_
	.p2align	2
__RNvXs0_Cs133NfS5DO5d_19sketch_codegen_taitNtB5_15StandardCodegenINtB5_15DispatchCodegenNtB5_5MyCfgE5buildB5_:
	.cfi_startproc
	ret
	.cfi_endproc

.subsections_via_symbols
