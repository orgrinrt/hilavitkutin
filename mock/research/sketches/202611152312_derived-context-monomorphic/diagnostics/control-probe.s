	.file	"99ga7fuq8z7oewu0p3ifype4z"
	.section	.text.control_blk_drift,"ax",@progbits
	.globl	control_blk_drift
	.p2align	4
	.type	control_blk_drift,@function
control_blk_drift:
	.cfi_startproc
	movq	112(%rdi), %rax
	testq	%rax, %rax
	je	.LBB0_3
	movq	104(%rdi), %rcx
	shlq	$3, %rcx
	movq	(%rdi), %rdx
	addq	%rcx, %rdx
	movq	32(%rdi), %rsi
	addq	%rcx, %rsi
	addq	72(%rdi), %rcx
	xorl	%edi, %edi
	.p2align	4
.LBB0_2:
	movq	(%rsi,%rdi,8), %r8
	addq	(%rdx,%rdi,8), %r8
	movq	%r8, (%rcx,%rdi,8)
	incq	%rdi
	cmpq	%rdi, %rax
	jne	.LBB0_2
.LBB0_3:
	retq
.Lfunc_end0:
	.size	control_blk_drift, .Lfunc_end0-control_blk_drift
	.cfi_endproc

	.section	.text.control_blk_stamp,"ax",@progbits
	.globl	control_blk_stamp
	.p2align	4
	.type	control_blk_stamp,@function
control_blk_stamp:
	.cfi_startproc
	movq	48(%rdi), %rax
	testq	%rax, %rax
	je	.LBB1_3
	movq	40(%rdi), %rcx
	shlq	$3, %rcx
	addq	8(%rdi), %rcx
	xorl	%edx, %edx
	.p2align	4
.LBB1_2:
	movq	%rdx, (%rcx,%rdx,8)
	incq	%rdx
	cmpq	%rdx, %rax
	jne	.LBB1_2
.LBB1_3:
	retq
.Lfunc_end1:
	.size	control_blk_stamp, .Lfunc_end1-control_blk_stamp
	.cfi_endproc

	.section	.text.control_blk_warm,"ax",@progbits
	.globl	control_blk_warm
	.p2align	4
	.type	control_blk_warm,@function
control_blk_warm:
	.cfi_startproc
	movq	80(%rdi), %rax
	testq	%rax, %rax
	je	.LBB2_3
	movq	72(%rdi), %rcx
	shlq	$3, %rcx
	movq	(%rdi), %rdx
	addq	%rcx, %rdx
	addq	40(%rdi), %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB2_2:
	movq	(%rdx,%rsi,8), %rdi
	addq	%rdi, %rdi
	movq	%rdi, (%rcx,%rsi,8)
	incq	%rsi
	cmpq	%rsi, %rax
	jne	.LBB2_2
.LBB2_3:
	retq
.Lfunc_end2:
	.size	control_blk_warm, .Lfunc_end2-control_blk_warm
	.cfi_endproc

	.section	.text.control_body_drift,"ax",@progbits
	.globl	control_body_drift
	.p2align	4
	.type	control_body_drift,@function
control_body_drift:
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
	movq	24(%rdi), %rbp
	testq	%rbp, %rbp
	je	.LBB3_3
	movq	(%rdi), %rbx
	movq	8(%rdi), %r13
	movq	16(%rdi), %rcx
	movq	%rcx, 16(%rsp)
	xorl	%r14d, %r14d
	leaq	8(%rsp), %r15
	.p2align	4
.LBB3_2:
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*(%r13)
	movq	8(%rsp), %r12
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*8(%r13)
	addq	8(%rsp), %r12
	movq	%r12, 8(%rsp)
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	movq	16(%rsp), %rax
	callq	*(%rax)
	incq	%r14
	cmpq	%r14, %rbp
	jne	.LBB3_2
.LBB3_3:
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
.Lfunc_end3:
	.size	control_body_drift, .Lfunc_end3-control_body_drift
	.cfi_endproc

	.section	.text.control_body_stamp,"ax",@progbits
	.globl	control_body_stamp
	.p2align	4
	.type	control_body_stamp,@function
control_body_stamp:
	.cfi_startproc
	pushq	%r15
	.cfi_def_cfa_offset 16
	pushq	%r14
	.cfi_def_cfa_offset 24
	pushq	%r13
	.cfi_def_cfa_offset 32
	pushq	%r12
	.cfi_def_cfa_offset 40
	pushq	%rbx
	.cfi_def_cfa_offset 48
	subq	$16, %rsp
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -48
	.cfi_offset %r12, -40
	.cfi_offset %r13, -32
	.cfi_offset %r14, -24
	.cfi_offset %r15, -16
	movq	24(%rdi), %r12
	testq	%r12, %r12
	je	.LBB4_3
	movq	(%rdi), %rbx
	movq	16(%rdi), %r13
	xorl	%r14d, %r14d
	leaq	8(%rsp), %r15
	.p2align	4
.LBB4_2:
	movq	%r14, 8(%rsp)
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*(%r13)
	incq	%r14
	cmpq	%r14, %r12
	jne	.LBB4_2
.LBB4_3:
	addq	$16, %rsp
	.cfi_def_cfa_offset 48
	popq	%rbx
	.cfi_def_cfa_offset 40
	popq	%r12
	.cfi_def_cfa_offset 32
	popq	%r13
	.cfi_def_cfa_offset 24
	popq	%r14
	.cfi_def_cfa_offset 16
	popq	%r15
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end4:
	.size	control_body_stamp, .Lfunc_end4-control_body_stamp
	.cfi_endproc

	.section	.text.control_body_warm,"ax",@progbits
	.globl	control_body_warm
	.p2align	4
	.type	control_body_warm,@function
control_body_warm:
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
	pushq	%rax
	.cfi_def_cfa_offset 64
	.cfi_offset %rbx, -56
	.cfi_offset %r12, -48
	.cfi_offset %r13, -40
	.cfi_offset %r14, -32
	.cfi_offset %r15, -24
	.cfi_offset %rbp, -16
	movq	24(%rdi), %r12
	testq	%r12, %r12
	je	.LBB5_3
	movq	(%rdi), %rbx
	movq	8(%rdi), %r13
	movq	16(%rdi), %rbp
	xorl	%r14d, %r14d
	movq	%rsp, %r15
	.p2align	4
.LBB5_2:
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*(%r13)
	movq	(%rsp), %rax
	addq	%rax, %rax
	movq	%rax, (%rsp)
	movq	%rbx, %rdi
	movq	%r14, %rsi
	movq	%r15, %rdx
	callq	*(%rbp)
	incq	%r14
	cmpq	%r14, %r12
	jne	.LBB5_2
.LBB5_3:
	addq	$8, %rsp
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
.Lfunc_end5:
	.size	control_body_warm, .Lfunc_end5-control_body_warm
	.cfi_endproc

	.ident	"rustc version 1.98.0-nightly (57d06900f 2026-05-27)"
	.section	".note.GNU-stack","",@progbits
