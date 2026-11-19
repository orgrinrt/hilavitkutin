	.file	"0jpudeloc3ow10gwxaz24evjc"
	.section	.text.probe_blk_drift,"ax",@progbits
	.globl	probe_blk_drift
	.p2align	4
	.type	probe_blk_drift,@function
probe_blk_drift:
	.cfi_startproc
	movq	112(%rdi), %rax
	testq	%rax, %rax
	je	.LBB0_3
	movq	104(%rdi), %rcx
	shlq	$3, %rcx
	movq	72(%rdi), %rdx
	addq	%rcx, %rdx
	movq	32(%rdi), %rsi
	addq	%rcx, %rsi
	addq	(%rdi), %rcx
	xorl	%edi, %edi
	.p2align	4
.LBB0_2:
	movq	(%rsi,%rdi,8), %r8
	addq	(%rcx,%rdi,8), %r8
	movq	%r8, (%rdx,%rdi,8)
	incq	%rdi
	cmpq	%rdi, %rax
	jne	.LBB0_2
.LBB0_3:
	retq
.Lfunc_end0:
	.size	probe_blk_drift, .Lfunc_end0-probe_blk_drift
	.cfi_endproc

	.section	.text.probe_blk_stamp,"ax",@progbits
	.globl	probe_blk_stamp
	.p2align	4
	.type	probe_blk_stamp,@function
probe_blk_stamp:
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
	.size	probe_blk_stamp, .Lfunc_end1-probe_blk_stamp
	.cfi_endproc

	.section	.text.probe_blk_warm,"ax",@progbits
	.globl	probe_blk_warm
	.p2align	4
	.type	probe_blk_warm,@function
probe_blk_warm:
	.cfi_startproc
	movq	80(%rdi), %rax
	testq	%rax, %rax
	je	.LBB2_3
	movq	72(%rdi), %rcx
	shlq	$3, %rcx
	movq	40(%rdi), %rdx
	addq	%rcx, %rdx
	addq	(%rdi), %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB2_2:
	movq	(%rcx,%rsi,8), %rdi
	addq	%rdi, %rdi
	movq	%rdi, (%rdx,%rsi,8)
	incq	%rsi
	cmpq	%rsi, %rax
	jne	.LBB2_2
.LBB2_3:
	retq
.Lfunc_end2:
	.size	probe_blk_warm, .Lfunc_end2-probe_blk_warm
	.cfi_endproc

	.section	.text.probe_block_drift,"ax",@progbits
	.globl	probe_block_drift
	.p2align	4
	.type	probe_block_drift,@function
probe_block_drift:
	.cfi_startproc
	movq	112(%rdi), %rax
	testq	%rax, %rax
	je	.LBB3_3
	movq	104(%rdi), %rcx
	shlq	$3, %rcx
	movq	72(%rdi), %rdx
	addq	%rcx, %rdx
	movq	32(%rdi), %rsi
	addq	%rcx, %rsi
	addq	(%rdi), %rcx
	xorl	%edi, %edi
	.p2align	4
.LBB3_2:
	movq	(%rsi,%rdi,8), %r8
	addq	(%rcx,%rdi,8), %r8
	movq	%r8, (%rdx,%rdi,8)
	incq	%rdi
	cmpq	%rdi, %rax
	jne	.LBB3_2
.LBB3_3:
	retq
.Lfunc_end3:
	.size	probe_block_drift, .Lfunc_end3-probe_block_drift
	.cfi_endproc

	.section	.text.probe_block_stamp,"ax",@progbits
	.globl	probe_block_stamp
	.p2align	4
	.type	probe_block_stamp,@function
probe_block_stamp:
	.cfi_startproc
	movq	48(%rdi), %rax
	testq	%rax, %rax
	je	.LBB4_3
	movq	40(%rdi), %rcx
	shlq	$3, %rcx
	addq	8(%rdi), %rcx
	xorl	%edx, %edx
	.p2align	4
.LBB4_2:
	movq	%rdx, (%rcx,%rdx,8)
	incq	%rdx
	cmpq	%rdx, %rax
	jne	.LBB4_2
.LBB4_3:
	retq
.Lfunc_end4:
	.size	probe_block_stamp, .Lfunc_end4-probe_block_stamp
	.cfi_endproc

	.section	.text.probe_block_warm,"ax",@progbits
	.globl	probe_block_warm
	.p2align	4
	.type	probe_block_warm,@function
probe_block_warm:
	.cfi_startproc
	movq	80(%rdi), %rax
	testq	%rax, %rax
	je	.LBB5_3
	movq	72(%rdi), %rcx
	shlq	$3, %rcx
	movq	40(%rdi), %rdx
	addq	%rcx, %rdx
	addq	(%rdi), %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB5_2:
	movq	(%rcx,%rsi,8), %rdi
	addq	%rdi, %rdi
	movq	%rdi, (%rdx,%rsi,8)
	incq	%rsi
	cmpq	%rsi, %rax
	jne	.LBB5_2
.LBB5_3:
	retq
.Lfunc_end5:
	.size	probe_block_warm, .Lfunc_end5-probe_block_warm
	.cfi_endproc

	.section	.text.probe_native_drift,"ax",@progbits
	.globl	probe_native_drift
	.p2align	4
	.type	probe_native_drift,@function
probe_native_drift:
	.cfi_startproc
	movq	112(%rdi), %rax
	testq	%rax, %rax
	je	.LBB6_3
	movq	104(%rdi), %rcx
	shlq	$3, %rcx
	movq	(%rdi), %rdx
	addq	%rcx, %rdx
	movq	32(%rdi), %rsi
	addq	%rcx, %rsi
	addq	72(%rdi), %rcx
	xorl	%edi, %edi
	.p2align	4
.LBB6_2:
	movq	(%rsi,%rdi,8), %r8
	addq	(%rdx,%rdi,8), %r8
	movq	%r8, (%rcx,%rdi,8)
	incq	%rdi
	cmpq	%rdi, %rax
	jne	.LBB6_2
.LBB6_3:
	retq
.Lfunc_end6:
	.size	probe_native_drift, .Lfunc_end6-probe_native_drift
	.cfi_endproc

	.section	.text.probe_native_stamp,"ax",@progbits
	.globl	probe_native_stamp
	.p2align	4
	.type	probe_native_stamp,@function
probe_native_stamp:
	.cfi_startproc
	movq	48(%rdi), %rax
	testq	%rax, %rax
	je	.LBB7_3
	movq	40(%rdi), %rcx
	shlq	$3, %rcx
	addq	8(%rdi), %rcx
	xorl	%edx, %edx
	.p2align	4
.LBB7_2:
	movq	%rdx, (%rcx,%rdx,8)
	incq	%rdx
	cmpq	%rdx, %rax
	jne	.LBB7_2
.LBB7_3:
	retq
.Lfunc_end7:
	.size	probe_native_stamp, .Lfunc_end7-probe_native_stamp
	.cfi_endproc

	.section	.text.probe_native_warm,"ax",@progbits
	.globl	probe_native_warm
	.p2align	4
	.type	probe_native_warm,@function
probe_native_warm:
	.cfi_startproc
	movq	80(%rdi), %rax
	testq	%rax, %rax
	je	.LBB8_3
	movq	72(%rdi), %rcx
	shlq	$3, %rcx
	movq	(%rdi), %rdx
	addq	%rcx, %rdx
	addq	40(%rdi), %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB8_2:
	movq	(%rdx,%rsi,8), %rdi
	addq	%rdi, %rdi
	movq	%rdi, (%rcx,%rsi,8)
	incq	%rsi
	cmpq	%rsi, %rax
	jne	.LBB8_2
.LBB8_3:
	retq
.Lfunc_end8:
	.size	probe_native_warm, .Lfunc_end8-probe_native_warm
	.cfi_endproc

	.ident	"rustc version 1.98.0-nightly (57d06900f 2026-05-27)"
	.section	".note.GNU-stack","",@progbits
