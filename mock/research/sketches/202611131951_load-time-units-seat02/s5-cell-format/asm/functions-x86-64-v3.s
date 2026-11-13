# s5_mul_q16: 49 lines, 0 call instructions, 11 lines touching vector registers or packed ops
s5_mul_q16:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB0_7
	cmpq	$8, %r9
	jae	.LBB0_4
	xorl	%eax, %eax
	jmp	.LBB0_3
.LBB0_4:
	movabsq	$2305843009213693944, %rax
	andq	%r9, %rax
	leaq	(,%r9,4), %rcx
	andq	$-32, %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB0_5:
	vpmovsxdq	16(%rdi,%rsi), %ymm0
	vpmovsxdq	(%rdi,%rsi), %ymm1
	vpmovsxdq	16(%rdx,%rsi), %ymm2
	vpmuldq	%ymm0, %ymm2, %ymm0
	vpmovsxdq	(%rdx,%rsi), %ymm2
	vpmuldq	%ymm1, %ymm2, %ymm1
	vpsrlq	$16, %ymm0, %ymm0
	vpsrlq	$16, %ymm1, %ymm1
	vshufps	$136, %ymm0, %ymm1, %ymm0
	vpermpd	$216, %ymm0, %ymm0
	vmovups	%ymm0, (%r8,%rsi)
	addq	$32, %rsi
	cmpq	%rsi, %rcx
	jne	.LBB0_5
	jmp	.LBB0_6
.LBB0_3:
	movslq	(%rdi,%rax,4), %rcx
	movslq	(%rdx,%rax,4), %rsi
	imulq	%rcx, %rsi
	shrq	$16, %rsi
	movl	%esi, (%r8,%rax,4)
	incq	%rax
.LBB0_6:
	cmpq	%rax, %r9
	jne	.LBB0_3
.LBB0_7:
	vzeroupper
	retq
.Lfunc_end0:

# s5_mul_q32: 99 lines, 0 call instructions, 8 lines touching vector registers or packed ops
s5_mul_q32:
	.cfi_startproc
	cmpq	%r9, %rsi
	cmovbq	%rsi, %r9
	cmpq	%r9, %rcx
	cmovbq	%rcx, %r9
	testq	%r9, %r9
	je	.LBB1_8
	pushq	%rbp
	.cfi_def_cfa_offset 16
	pushq	%r15
	.cfi_def_cfa_offset 24
	pushq	%r14
	.cfi_def_cfa_offset 32
	pushq	%r13
	.cfi_def_cfa_offset 40
	pushq	%r12
	.cfi_def_cfa_offset 48
	pushq	%rbx
	.cfi_def_cfa_offset 56
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%rdx, %r11
	cmpq	$4, %r9
	jae	.LBB1_4
	xorl	%r12d, %r12d
	jmp	.LBB1_3
.LBB1_4:
	movabsq	$1152921504606846972, %r12
	andq	%r9, %r12
	leaq	(,%r9,8), %r13
	andq	$-32, %r13
	xorl	%ebp, %ebp
	.p2align	4
.LBB1_5:
	movq	8(%r11,%rbp), %rax
	imulq	8(%rdi,%rbp)
	movq	%rax, %rsi
	movq	%rdx, %r10
	movq	(%r11,%rbp), %rax
	imulq	(%rdi,%rbp)
	movq	%rax, %rcx
	movq	%rdx, %rbx
	movq	24(%r11,%rbp), %rax
	imulq	24(%rdi,%rbp)
	movq	%rax, %r14
	movq	%rdx, %r15
	movq	16(%r11,%rbp), %rax
	imulq	16(%rdi,%rbp)
	shldq	$32, %rax, %rdx
	shldq	$32, %r14, %r15
	shldq	$32, %rcx, %rbx
	shldq	$32, %rsi, %r10
	vmovq	%r10, %xmm0
	vmovq	%rbx, %xmm1
	vpunpcklqdq	%xmm0, %xmm1, %xmm0
	vmovq	%r15, %xmm1
	vmovq	%rdx, %xmm2
	vpunpcklqdq	%xmm1, %xmm2, %xmm1
	vmovdqu	%xmm1, 16(%r8,%rbp)
	vmovdqu	%xmm0, (%r8,%rbp)
	addq	$32, %rbp
	cmpq	%rbp, %r13
	jne	.LBB1_5
	jmp	.LBB1_6
.LBB1_3:
	movq	(%r11,%r12,8), %rax
	imulq	(%rdi,%r12,8)
	shldq	$32, %rax, %rdx
	movq	%rdx, (%r8,%r12,8)
	incq	%r12
.LBB1_6:
	cmpq	%r12, %r9
	jne	.LBB1_3
	popq	%rbx
	.cfi_def_cfa_offset 48
	popq	%r12
	.cfi_def_cfa_offset 40
	popq	%r13
	.cfi_def_cfa_offset 32
	popq	%r14
	.cfi_def_cfa_offset 24
	popq	%r15
	.cfi_def_cfa_offset 16
	popq	%rbp
	.cfi_def_cfa_offset 8
	.cfi_restore %rbx
	.cfi_restore %r12
	.cfi_restore %r13
	.cfi_restore %r14
	.cfi_restore %r15
	.cfi_restore %rbp
.LBB1_8:
	retq
.Lfunc_end1:

