	.file	"probe.72ecb1e08027cc8b-cgu.0"
	.section	.text.probe_control_indirect,"ax",@progbits
	.globl	probe_control_indirect
	.p2align	4
	.type	probe_control_indirect,@function
probe_control_indirect:
	.cfi_startproc
	movq	%rdi, %rax
	movq	%rsi, %rdi
	jmpq	*%rax
.Lfunc_end0:
	.size	probe_control_indirect, .Lfunc_end0-probe_control_indirect
	.cfi_endproc

	.section	.text.probe_native_drift,"ax",@progbits
	.globl	probe_native_drift
	.p2align	4
	.type	probe_native_drift,@function
probe_native_drift:
	.cfi_startproc
	movq	112(%rdi), %rax
	testq	%rax, %rax
	je	.LBB1_3
	movq	104(%rdi), %rcx
	shlq	$3, %rcx
	movq	72(%rdi), %rdx
	addq	%rcx, %rdx
	movq	32(%rdi), %rsi
	addq	%rcx, %rsi
	addq	(%rdi), %rcx
	xorl	%edi, %edi
	.p2align	4
.LBB1_2:
	movq	(%rsi,%rdi,8), %r8
	addq	(%rcx,%rdi,8), %r8
	movq	%r8, (%rdx,%rdi,8)
	incq	%rdi
	cmpq	%rdi, %rax
	jne	.LBB1_2
.LBB1_3:
	retq
.Lfunc_end1:
	.size	probe_native_drift, .Lfunc_end1-probe_native_drift
	.cfi_endproc

	.section	.text.probe_native_warm,"ax",@progbits
	.globl	probe_native_warm
	.p2align	4
	.type	probe_native_warm,@function
probe_native_warm:
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
	.size	probe_native_warm, .Lfunc_end2-probe_native_warm
	.cfi_endproc

	.section	.text.probe_value_drift,"ax",@progbits
	.globl	probe_value_drift
	.p2align	4
	.type	probe_value_drift,@function
probe_value_drift:
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
	.size	probe_value_drift, .Lfunc_end3-probe_value_drift
	.cfi_endproc

	.section	.text.probe_value_warm,"ax",@progbits
	.globl	probe_value_warm
	.p2align	4
	.type	probe_value_warm,@function
probe_value_warm:
	.cfi_startproc
	movq	80(%rdi), %rax
	testq	%rax, %rax
	je	.LBB4_3
	movq	72(%rdi), %rcx
	shlq	$3, %rcx
	movq	40(%rdi), %rdx
	addq	%rcx, %rdx
	addq	(%rdi), %rcx
	xorl	%esi, %esi
	.p2align	4
.LBB4_2:
	movq	(%rcx,%rsi,8), %rdi
	addq	%rdi, %rdi
	movq	%rdi, (%rdx,%rsi,8)
	incq	%rsi
	cmpq	%rsi, %rax
	jne	.LBB4_2
.LBB4_3:
	retq
.Lfunc_end4:
	.size	probe_value_warm, .Lfunc_end4-probe_value_warm
	.cfi_endproc

	.ident	"rustc version 1.98.0-nightly (57d06900f 2026-05-27)"
	.section	".note.GNU-stack","",@progbits
