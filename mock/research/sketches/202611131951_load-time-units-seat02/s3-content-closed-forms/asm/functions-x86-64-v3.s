# s3_scale: 120 lines, 0 call instructions, 65 lines touching vector registers or packed ops
s3_scale:
	.cfi_startproc
	cmpq	%rsi, %r8
	cmovbq	%r8, %rsi
	testq	%rsi, %rsi
	je	.LBB10_13
	cmpq	$4, %rsi
	jae	.LBB10_4
	xorl	%eax, %eax
	jmp	.LBB10_3
.LBB10_4:
	movabsq	$1152921504606846960, %r8
	vmovq	%rdx, %xmm0
	cmpq	$16, %rsi
	jae	.LBB10_6
	xorl	%eax, %eax
	jmp	.LBB10_10
.LBB10_6:
	movq	%rsi, %rax
	andq	%r8, %rax
	vpbroadcastq	%xmm0, %ymm1
	leaq	(,%rsi,8), %r9
	andq	$-128, %r9
	xorl	%r10d, %r10d
	vpsrlq	$32, %ymm1, %ymm2
	.p2align	4
.LBB10_7:
	vmovdqu	(%rdi,%r10), %ymm3
	vmovdqu	32(%rdi,%r10), %ymm4
	vmovdqu	64(%rdi,%r10), %ymm5
	vmovdqu	96(%rdi,%r10), %ymm6
	vpmuludq	%ymm2, %ymm3, %ymm7
	vpsrlq	$32, %ymm3, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm3, %ymm3
	vpaddq	%ymm7, %ymm3, %ymm3
	vpmuludq	%ymm2, %ymm4, %ymm7
	vpsrlq	$32, %ymm4, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm4, %ymm4
	vpaddq	%ymm7, %ymm4, %ymm4
	vpmuludq	%ymm2, %ymm5, %ymm7
	vpsrlq	$32, %ymm5, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm5, %ymm5
	vpaddq	%ymm7, %ymm5, %ymm5
	vpmuludq	%ymm2, %ymm6, %ymm7
	vpsrlq	$32, %ymm6, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm6, %ymm6
	vpaddq	%ymm7, %ymm6, %ymm6
	vpsrad	$16, %ymm3, %ymm7
	vpsrlq	$16, %ymm3, %ymm3
	vpblendd	$170, %ymm7, %ymm3, %ymm3
	vpsrad	$16, %ymm4, %ymm7
	vpsrlq	$16, %ymm4, %ymm4
	vpblendd	$170, %ymm7, %ymm4, %ymm4
	vpsrad	$16, %ymm5, %ymm7
	vpsrlq	$16, %ymm5, %ymm5
	vpblendd	$170, %ymm7, %ymm5, %ymm5
	vpsrad	$16, %ymm6, %ymm7
	vpsrlq	$16, %ymm6, %ymm6
	vpblendd	$170, %ymm7, %ymm6, %ymm6
	vmovdqu	%ymm3, (%rcx,%r10)
	vmovdqu	%ymm4, 32(%rcx,%r10)
	vmovdqu	%ymm5, 64(%rcx,%r10)
	vmovdqu	%ymm6, 96(%rcx,%r10)
	subq	$-128, %r10
	cmpq	%r10, %r9
	jne	.LBB10_7
	cmpq	%rax, %rsi
	je	.LBB10_13
	testb	$12, %sil
	je	.LBB10_3
.LBB10_10:
	movq	%rax, %r9
	addq	$12, %r8
	movq	%r8, %rax
	andq	%rsi, %rax
	vpbroadcastq	%xmm0, %ymm0
	vpsrlq	$32, %ymm0, %ymm1
	.p2align	4
.LBB10_11:
	vmovdqu	(%rdi,%r9,8), %ymm2
	vpmuludq	%ymm1, %ymm2, %ymm3
	vpsrlq	$32, %ymm2, %ymm4
	vpmuludq	%ymm0, %ymm4, %ymm4
	vpaddq	%ymm4, %ymm3, %ymm3
	vpsllq	$32, %ymm3, %ymm3
	vpmuludq	%ymm0, %ymm2, %ymm2
	vpaddq	%ymm3, %ymm2, %ymm2
	vpsrad	$16, %ymm2, %ymm3
	vpsrlq	$16, %ymm2, %ymm2
	vpblendd	$170, %ymm3, %ymm2, %ymm2
	vmovdqu	%ymm2, (%rcx,%r9,8)
	addq	$4, %r9
	cmpq	%r9, %rax
	jne	.LBB10_11
	jmp	.LBB10_12
.LBB10_3:
	movq	(%rdi,%rax,8), %r8
	imulq	%rdx, %r8
	sarq	$16, %r8
	movq	%r8, (%rcx,%rax,8)
	incq	%rax
.LBB10_12:
	cmpq	%rax, %rsi
	jne	.LBB10_3
.LBB10_13:
	vzeroupper
	retq
.Lfunc_end10:

# s3_per_record: 308 lines, 2 call instructions, 114 lines touching vector registers or packed ops
s3_per_record:
	.cfi_startproc
	subq	$56, %rsp
	.cfi_def_cfa_offset 64
	movq	%rdi, (%rsp)
	movq	%rsp, %rax
	#APP
	#NO_APP
	cmpq	%rdx, %r8
	cmovbq	%r8, %rdx
	testq	%rdx, %rdx
	je	.LBB9_40
	movq	(%rsp), %rax
	movq	(%rax), %rdi
	leaq	.LJTI9_0(%rip), %r8
	movslq	(%r8,%rdi,4), %rdi
	addq	%r8, %rdi
	jmpq	*%rdi
.LBB9_3:
	movq	8(%rax), %rax
	cmpq	$4, %rdx
	jae	.LBB9_5
	xorl	%edi, %edi
	jmp	.LBB9_14
.LBB9_27:
	movq	8(%rax), %rdi
	movq	16(%rax), %rax
	cmpq	%rax, %rdi
	jg	.LBB9_41
	cmpq	$4, %rdx
	jae	.LBB9_31
	xorl	%r8d, %r8d
	jmp	.LBB9_30
.LBB9_2:
	shlq	$3, %rdx
	movq	%rcx, %rdi
	callq	*memcpy@GOTPCREL(%rip)
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	retq
.LBB9_15:
	.cfi_def_cfa_offset 64
	movq	8(%rax), %rax
	cmpq	$4, %rdx
	jae	.LBB9_17
	xorl	%edi, %edi
	jmp	.LBB9_26
.LBB9_5:
	movabsq	$1152921504606846960, %r8
	vmovq	%rax, %xmm0
	cmpq	$16, %rdx
	jae	.LBB9_7
	xorl	%edi, %edi
	jmp	.LBB9_11
.LBB9_17:
	movabsq	$1152921504606846960, %r8
	vmovq	%rax, %xmm0
	cmpq	$16, %rdx
	jae	.LBB9_19
	xorl	%edi, %edi
	jmp	.LBB9_23
.LBB9_31:
	movabsq	$1152921504606846960, %r9
	vmovq	%rdi, %xmm0
	vmovq	%rax, %xmm1
	cmpq	$16, %rdx
	jae	.LBB9_33
	xorl	%r8d, %r8d
	jmp	.LBB9_37
.LBB9_7:
	movq	%rdx, %rdi
	andq	%r8, %rdi
	vpbroadcastq	%xmm0, %ymm1
	leaq	(,%rdx,8), %r9
	andq	$-128, %r9
	xorl	%r10d, %r10d
	vpsrlq	$32, %ymm1, %ymm2
	.p2align	4
.LBB9_8:
	vmovdqu	(%rsi,%r10), %ymm3
	vmovdqu	32(%rsi,%r10), %ymm4
	vmovdqu	64(%rsi,%r10), %ymm5
	vmovdqu	96(%rsi,%r10), %ymm6
	vpmuludq	%ymm2, %ymm3, %ymm7
	vpsrlq	$32, %ymm3, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm3, %ymm3
	vpaddq	%ymm7, %ymm3, %ymm3
	vpmuludq	%ymm2, %ymm4, %ymm7
	vpsrlq	$32, %ymm4, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm4, %ymm4
	vpaddq	%ymm7, %ymm4, %ymm4
	vpmuludq	%ymm2, %ymm5, %ymm7
	vpsrlq	$32, %ymm5, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm5, %ymm5
	vpaddq	%ymm7, %ymm5, %ymm5
	vpmuludq	%ymm2, %ymm6, %ymm7
	vpsrlq	$32, %ymm6, %ymm8
	vpmuludq	%ymm1, %ymm8, %ymm8
	vpaddq	%ymm7, %ymm8, %ymm7
	vpsllq	$32, %ymm7, %ymm7
	vpmuludq	%ymm1, %ymm6, %ymm6
	vpaddq	%ymm7, %ymm6, %ymm6
	vpsrad	$16, %ymm3, %ymm7
	vpsrlq	$16, %ymm3, %ymm3
	vpblendd	$170, %ymm7, %ymm3, %ymm3
	vpsrad	$16, %ymm4, %ymm7
	vpsrlq	$16, %ymm4, %ymm4
	vpblendd	$170, %ymm7, %ymm4, %ymm4
	vpsrad	$16, %ymm5, %ymm7
	vpsrlq	$16, %ymm5, %ymm5
	vpblendd	$170, %ymm7, %ymm5, %ymm5
	vpsrad	$16, %ymm6, %ymm7
	vpsrlq	$16, %ymm6, %ymm6
	vpblendd	$170, %ymm7, %ymm6, %ymm6
	vmovdqu	%ymm3, (%rcx,%r10)
	vmovdqu	%ymm4, 32(%rcx,%r10)
	vmovdqu	%ymm5, 64(%rcx,%r10)
	vmovdqu	%ymm6, 96(%rcx,%r10)
	subq	$-128, %r10
	cmpq	%r10, %r9
	jne	.LBB9_8
	cmpq	%rdi, %rdx
	je	.LBB9_40
	testb	$12, %dl
	je	.LBB9_14
.LBB9_11:
	movq	%rdi, %r9
	addq	$12, %r8
	movq	%r8, %rdi
	andq	%rdx, %rdi
	vpbroadcastq	%xmm0, %ymm0
	vpsrlq	$32, %ymm0, %ymm1
	.p2align	4
.LBB9_12:
	vmovdqu	(%rsi,%r9,8), %ymm2
	vpmuludq	%ymm1, %ymm2, %ymm3
	vpsrlq	$32, %ymm2, %ymm4
	vpmuludq	%ymm0, %ymm4, %ymm4
	vpaddq	%ymm4, %ymm3, %ymm3
	vpsllq	$32, %ymm3, %ymm3
	vpmuludq	%ymm0, %ymm2, %ymm2
	vpaddq	%ymm3, %ymm2, %ymm2
	vpsrad	$16, %ymm2, %ymm3
	vpsrlq	$16, %ymm2, %ymm2
	vpblendd	$170, %ymm3, %ymm2, %ymm2
	vmovdqu	%ymm2, (%rcx,%r9,8)
	addq	$4, %r9
	cmpq	%r9, %rdi
	jne	.LBB9_12
	jmp	.LBB9_13
.LBB9_19:
	movq	%rdx, %rdi
	andq	%r8, %rdi
	vpbroadcastq	%xmm0, %ymm1
	leaq	(,%rdx,8), %r9
	andq	$-128, %r9
	xorl	%r10d, %r10d
	.p2align	4
.LBB9_20:
	vpaddq	(%rsi,%r10), %ymm1, %ymm2
	vpaddq	32(%rsi,%r10), %ymm1, %ymm3
	vpaddq	64(%rsi,%r10), %ymm1, %ymm4
	vpaddq	96(%rsi,%r10), %ymm1, %ymm5
	vmovdqu	%ymm2, (%rcx,%r10)
	vmovdqu	%ymm3, 32(%rcx,%r10)
	vmovdqu	%ymm4, 64(%rcx,%r10)
	vmovdqu	%ymm5, 96(%rcx,%r10)
	subq	$-128, %r10
	cmpq	%r10, %r9
	jne	.LBB9_20
	cmpq	%rdi, %rdx
	je	.LBB9_40
	testb	$12, %dl
	je	.LBB9_26
.LBB9_23:
	movq	%rdi, %r9
	addq	$12, %r8
	movq	%r8, %rdi
	andq	%rdx, %rdi
	vpbroadcastq	%xmm0, %ymm0
	.p2align	4
.LBB9_24:
	vpaddq	(%rsi,%r9,8), %ymm0, %ymm1
	vmovdqu	%ymm1, (%rcx,%r9,8)
	addq	$4, %r9
	cmpq	%r9, %rdi
	jne	.LBB9_24
	jmp	.LBB9_25
.LBB9_33:
	movq	%rdx, %r8
	andq	%r9, %r8
	vpbroadcastq	%xmm0, %ymm2
	vpbroadcastq	%xmm1, %ymm3
	leaq	(,%rdx,8), %r10
	andq	$-128, %r10
	xorl	%r11d, %r11d
	.p2align	4
.LBB9_34:
	vmovdqu	(%rsi,%r11), %ymm4
	vmovdqu	32(%rsi,%r11), %ymm5
	vmovdqu	64(%rsi,%r11), %ymm6
	vmovdqu	96(%rsi,%r11), %ymm7
	vpcmpgtq	%ymm4, %ymm2, %ymm8
	vpcmpgtq	%ymm5, %ymm2, %ymm9
	vpcmpgtq	%ymm6, %ymm2, %ymm10
	vpcmpgtq	%ymm4, %ymm3, %ymm11
	vblendvpd	%ymm11, %ymm4, %ymm3, %ymm4
	vpcmpgtq	%ymm7, %ymm2, %ymm11
	vpcmpgtq	%ymm5, %ymm3, %ymm12
	vblendvpd	%ymm12, %ymm5, %ymm3, %ymm5
	vpcmpgtq	%ymm6, %ymm3, %ymm12
	vblendvpd	%ymm12, %ymm6, %ymm3, %ymm6
	vpcmpgtq	%ymm7, %ymm3, %ymm12
	vblendvpd	%ymm12, %ymm7, %ymm3, %ymm7
	vblendvpd	%ymm8, %ymm2, %ymm4, %ymm4
	vblendvpd	%ymm9, %ymm2, %ymm5, %ymm5
	vblendvpd	%ymm10, %ymm2, %ymm6, %ymm6
	vblendvpd	%ymm11, %ymm2, %ymm7, %ymm7
	vmovupd	%ymm4, (%rcx,%r11)
	vmovupd	%ymm5, 32(%rcx,%r11)
	vmovupd	%ymm6, 64(%rcx,%r11)
	vmovupd	%ymm7, 96(%rcx,%r11)
	subq	$-128, %r11
	cmpq	%r11, %r10
	jne	.LBB9_34
	cmpq	%r8, %rdx
	je	.LBB9_40
	testb	$12, %dl
	je	.LBB9_30
.LBB9_37:
	movq	%r8, %r10
	addq	$12, %r9
	movq	%r9, %r8
	andq	%rdx, %r8
	vpbroadcastq	%xmm0, %ymm0
	vpbroadcastq	%xmm1, %ymm1
	.p2align	4
.LBB9_38:
	vmovdqu	(%rsi,%r10,8), %ymm2
	vpcmpgtq	%ymm2, %ymm0, %ymm3
	vpcmpgtq	%ymm2, %ymm1, %ymm4
	vblendvpd	%ymm4, %ymm2, %ymm1, %ymm2
	vblendvpd	%ymm3, %ymm0, %ymm2, %ymm2
	vmovupd	%ymm2, (%rcx,%r10,8)
	addq	$4, %r10
	cmpq	%r10, %r8
	jne	.LBB9_38
	jmp	.LBB9_39
.LBB9_41:
	movq	%rdi, 8(%rsp)
	movq	%rax, 16(%rsp)
	leaq	8(%rsp), %rax
	movq	%rax, 24(%rsp)
	leaq	_RNvXsR_NtNtCslfAf2Mv2Tp5_4core3fmt3numxNtB7_5Debug3fmt(%rip), %rax
	movq	%rax, 32(%rsp)
	leaq	16(%rsp), %rcx
	movq	%rcx, 40(%rsp)
	movq	%rax, 48(%rsp)
	leaq	.Lanon.e48a1b3c7bbee6ab5f41152d172fb202.26(%rip), %rdi
	leaq	.Lanon.e48a1b3c7bbee6ab5f41152d172fb202.29(%rip), %rdx
	leaq	24(%rsp), %rsi
	callq	*_RNvNtCslfAf2Mv2Tp5_4core9panicking9panic_fmt@GOTPCREL(%rip)
.LBB9_13:
	cmpq	%rdi, %rdx
	je	.LBB9_40
.LBB9_14:
	movq	(%rsi,%rdi,8), %r8
	imulq	%rax, %r8
	sarq	$16, %r8
	movq	%r8, (%rcx,%rdi,8)
	incq	%rdi
	jmp	.LBB9_13
.LBB9_25:
	cmpq	%rdi, %rdx
	je	.LBB9_40
.LBB9_26:
	movq	(%rsi,%rdi,8), %r8
	addq	%rax, %r8
	movq	%r8, (%rcx,%rdi,8)
	incq	%rdi
	jmp	.LBB9_25
.LBB9_30:
	movq	(%rsi,%r8,8), %r9
	cmpq	%rax, %r9
	movq	%rax, %r10
	cmovlq	%r9, %r10
	cmpq	%rdi, %r9
	cmovlq	%rdi, %r10
	movq	%r10, (%rcx,%r8,8)
	incq	%r8
.LBB9_39:
	cmpq	%r8, %rdx
	jne	.LBB9_30
.LBB9_40:
	addq	$56, %rsp
	.cfi_def_cfa_offset 8
	vzeroupper
	retq
.Lfunc_end9:

