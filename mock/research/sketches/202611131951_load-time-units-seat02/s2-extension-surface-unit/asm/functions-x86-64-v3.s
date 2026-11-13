# s2_ext_compiled: 121 lines, 0 call instructions, 25 lines touching vector registers or packed ops
s2_ext_compiled:
	.cfi_startproc
	movq	16(%rdi), %rax
	testq	%rax, %rax
	je	.LBB4_19
	movq	(%rdi), %rdx
	movq	8(%rdi), %rsi
	movq	(%rdx), %rcx
	movq	8(%rdx), %rdx
	movq	(%rsi), %rsi
	cmpq	$4, %rax
	jae	.LBB4_3
	xorl	%edi, %edi
.LBB4_14:
	movq	%rax, %r9
	movq	%rdi, %r8
	andq	$3, %r9
	je	.LBB4_17
	movq	%rdi, %r8
	.p2align	4
.LBB4_16:
	movq	(%rcx,%r8,8), %r10
	leaq	(%r10,%r10,2), %r10
	addq	(%rdx,%r8,8), %r10
	movq	%r10, (%rsi,%r8,8)
	incq	%r8
	decq	%r9
	jne	.LBB4_16
.LBB4_17:
	subq	%rax, %rdi
	cmpq	$-4, %rdi
	ja	.LBB4_19
	.p2align	4
.LBB4_18:
	movq	(%rcx,%r8,8), %rdi
	leaq	(%rdi,%rdi,2), %rdi
	addq	(%rdx,%r8,8), %rdi
	movq	%rdi, (%rsi,%r8,8)
	movq	8(%rcx,%r8,8), %rdi
	leaq	(%rdi,%rdi,2), %rdi
	addq	8(%rdx,%r8,8), %rdi
	movq	%rdi, 8(%rsi,%r8,8)
	movq	16(%rcx,%r8,8), %rdi
	leaq	(%rdi,%rdi,2), %rdi
	addq	16(%rdx,%r8,8), %rdi
	movq	%rdi, 16(%rsi,%r8,8)
	movq	24(%rcx,%r8,8), %rdi
	leaq	(%rdi,%rdi,2), %rdi
	addq	24(%rdx,%r8,8), %rdi
	movq	%rdi, 24(%rsi,%r8,8)
	addq	$4, %r8
	cmpq	%r8, %rax
	jne	.LBB4_18
.LBB4_19:
	vzeroupper
	retq
.LBB4_3:
	movq	%rsi, %r8
	subq	%rcx, %r8
	xorl	%edi, %edi
	cmpq	$128, %r8
	jb	.LBB4_14
	movq	%rsi, %r8
	subq	%rdx, %r8
	cmpq	$128, %r8
	jb	.LBB4_14
	cmpq	$16, %rax
	jae	.LBB4_7
	xorl	%edi, %edi
	jmp	.LBB4_11
.LBB4_7:
	movq	%rax, %rdi
	andq	$-16, %rdi
	xorl	%r8d, %r8d
	.p2align	4
.LBB4_8:
	vmovdqu	(%rcx,%r8,8), %ymm0
	vmovdqu	32(%rcx,%r8,8), %ymm1
	vmovdqu	64(%rcx,%r8,8), %ymm2
	vmovdqu	96(%rcx,%r8,8), %ymm3
	vpaddq	%ymm0, %ymm0, %ymm4
	vpaddq	%ymm0, %ymm4, %ymm0
	vpaddq	%ymm1, %ymm1, %ymm4
	vpaddq	%ymm1, %ymm4, %ymm1
	vpaddq	%ymm2, %ymm2, %ymm4
	vpaddq	%ymm2, %ymm4, %ymm2
	vpaddq	%ymm3, %ymm3, %ymm4
	vpaddq	%ymm3, %ymm4, %ymm3
	vpaddq	(%rdx,%r8,8), %ymm0, %ymm0
	vpaddq	32(%rdx,%r8,8), %ymm1, %ymm1
	vpaddq	64(%rdx,%r8,8), %ymm2, %ymm2
	vpaddq	96(%rdx,%r8,8), %ymm3, %ymm3
	vmovdqu	%ymm0, (%rsi,%r8,8)
	vmovdqu	%ymm1, 32(%rsi,%r8,8)
	vmovdqu	%ymm2, 64(%rsi,%r8,8)
	vmovdqu	%ymm3, 96(%rsi,%r8,8)
	addq	$16, %r8
	cmpq	%r8, %rdi
	jne	.LBB4_8
	cmpq	%rdi, %rax
	je	.LBB4_19
	testb	$12, %al
	je	.LBB4_14
.LBB4_11:
	movq	%rdi, %r8
	movq	%rax, %rdi
	andq	$-4, %rdi
	.p2align	4
.LBB4_12:
	vmovdqu	(%rcx,%r8,8), %ymm0
	vpaddq	%ymm0, %ymm0, %ymm1
	vpaddq	%ymm0, %ymm1, %ymm0
	vpaddq	(%rdx,%r8,8), %ymm0, %ymm0
	vmovdqu	%ymm0, (%rsi,%r8,8)
	addq	$4, %r8
	cmpq	%r8, %rdi
	jne	.LBB4_12
	cmpq	%rdi, %rax
	jne	.LBB4_14
	jmp	.LBB4_19
.Lfunc_end4:

# s2_host_per_record: 76 lines, 3 call instructions, 0 lines touching vector registers or packed ops
s2_host_per_record:
	.cfi_startproc
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
	subq	$24, %rsp
	.cfi_def_cfa_offset 80
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	%r8, 8(%rsp)
	movq	%rdi, 16(%rsp)
	leaq	16(%rsp), %rax
	#APP
	#NO_APP
	cmpq	$0, 80(%rsp)
	je	.LBB7_5
	movq	%r9, %r15
	movq	%rcx, %r12
	movq	%rdx, %r14
	movq	%rsi, %r13
	xorl	%ebp, %ebp
	.p2align	4
.LBB7_2:
	cmpq	%rbp, %r14
	je	.LBB7_7
	cmpq	%rbp, 8(%rsp)
	je	.LBB7_6
	leaq	1(%rbp), %rbx
	movq	(%r13,%rbp,8), %rdi
	movq	(%r12,%rbp,8), %rsi
	callq	*16(%rsp)
	movq	%rax, (%r15,%rbp,8)
	movq	%rbx, %rbp
	cmpq	%rbx, 80(%rsp)
	jne	.LBB7_2
.LBB7_5:
	addq	$24, %rsp
	.cfi_def_cfa_offset 56
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
	retq
.LBB7_7:
	.cfi_def_cfa_offset 80
	leaq	.Lanon.76a5ae483c26648fe94dd3ead3944b91.8(%rip), %rdx
	movq	%r14, %rdi
	movq	%r14, %rsi
	callq	*_RNvNtCslfAf2Mv2Tp5_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.LBB7_6:
	leaq	.Lanon.76a5ae483c26648fe94dd3ead3944b91.9(%rip), %rdx
	movq	8(%rsp), %rdi
	movq	%rdi, %rsi
	callq	*_RNvNtCslfAf2Mv2Tp5_4core9panicking18panic_bounds_check@GOTPCREL(%rip)
.Lfunc_end7:

# s2_host_per_morsel: 74 lines, 1 call instructions, 0 lines touching vector registers or packed ops
s2_host_per_morsel:
	.cfi_startproc
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
	subq	$56, %rsp
	.cfi_def_cfa_offset 112
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	112(%rsp), %rbp
	movq	%rdi, (%rsp)
	movq	%rsp, %rax
	#APP
	#NO_APP
	testq	%rbp, %rbp
	je	.LBB6_3
	movq	%r9, %r14
	movq	%rcx, %r15
	movq	%rsi, %r12
	xorl	%r13d, %r13d
	.p2align	4
.LBB6_2:
	movq	%r13, %rbx
	movq	%rbp, %r13
	subq	%rbx, %r13
	movq	120(%rsp), %rax
	cmpq	%rax, %r13
	cmovaeq	%rax, %r13
	leaq	(%r12,%rbx,8), %rax
	leaq	(%r15,%rbx,8), %rcx
	movq	%rax, 16(%rsp)
	movq	%rcx, 24(%rsp)
	leaq	(%r14,%rbx,8), %rax
	movq	%rax, 8(%rsp)
	leaq	16(%rsp), %rax
	movq	%rax, 32(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 40(%rsp)
	movq	%r13, 48(%rsp)
	leaq	32(%rsp), %rdi
	callq	*(%rsp)
	addq	%rbx, %r13
	cmpq	%rbp, %r13
	jb	.LBB6_2
.LBB6_3:
	addq	$56, %rsp
	.cfi_def_cfa_offset 56
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
	retq
.Lfunc_end6:

# s2_ext_one_record: 6 lines, 0 call instructions, 0 lines touching vector registers or packed ops
s2_ext_one_record:
	.cfi_startproc
	leaq	(%rdi,%rdi,2), %rax
	addq	%rsi, %rax
	retq
.Lfunc_end5:

