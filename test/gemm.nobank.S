	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_f2p0"
	.file	"gemm.c"
	.section	.text.gemm,"ax",@progbits
	.globl	gemm                            # -- Begin function gemm
	.p2align	2
	.type	gemm,@function
gemm:                                   # @gemm
# %bb.0:                                # %entry
	#APP
	csrr	a6, mhartid
	#NO_APP
	srli	a0, a6, 2
	andi	a12, a0, -4
	li	a0, 63
	bltu	a0, a12, .LBB0_5
# %bb.1:                                # %for.body.lr.ph
	slli	a0, a6, 28
	srli	a0, a0, 26
	lw	a5, %lo(A)(zero)
	lw	a7, %lo(B)(zero)
	lw	a1, %lo(C)(zero)
	addi	a2, a0, 1
	addi	a3, a0, 2
	addi	a4, a0, 3
	srli	a8, a6, 4
	slli	a8, a8, 10
	add	a5, a8, a5
	addi	a5, a5, 512
	andi	a6, a6, 15
	slli	a6, a6, 10
	add	a6, a6, a7
	addi	a6, a6, 512
	li	a7, 60
.LBB0_2:                                # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_3 Depth 2
	li	t3, 0
	li	t2, 0
	li	t0, 0
	li	a23, 0
	li	a20, 0
	li	a8, 0
	li	a9, 0
	li	a11, 0
	li	a13, 0
	li	a14, 0
	li	a15, 0
	li	a16, 0
	li	a18, 0
	li	a19, 0
	li	a21, 0
	li	t1, 0
	mv	a10, a12
	slli	t4, a12, 6
	ori	a22, t4, 64
	ori	a17, t4, 128
	ori	a12, t4, 192
	li	t5, 64
	mv	t6, a6
	mv	t7, a5
.LBB0_3:                                # %for.body10
                                        #   Parent Loop BB0_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lw	t8, -512(t7)
	lw	t9, -256(t7)
	lw	t10, 0(t7)
	lw	t11, 256(t7)
	lw	t12, -512(t6)
	lw	t13, -256(t6)
	lw	t14, 0(t6)
	lw	t15, 256(t6)
	mul	t16, t12, t8
	add	t3, t16, t3
	mul	t16, t13, t8
	add	t2, t16, t2
	mul	t16, t14, t8
	add	t0, t16, t0
	mul	t8, t15, t8
	add	a23, t8, a23
	mul	t8, t12, t9
	add	a20, t8, a20
	mul	t8, t13, t9
	add	t1, t8, t1
	mul	t8, t14, t9
	add	a21, t8, a21
	mul	t8, t15, t9
	add	a19, t8, a19
	mul	t8, t12, t10
	add	a18, t8, a18
	mul	t8, t13, t10
	add	a16, t8, a16
	mul	t8, t14, t10
	add	a15, t8, a15
	mul	t8, t15, t10
	add	a14, t8, a14
	mul	t8, t12, t11
	add	a13, t8, a13
	mul	t8, t13, t11
	add	a11, t8, a11
	mul	t8, t14, t11
	add	a9, t8, a9
	mul	t8, t15, t11
	add	a8, t8, a8
	addi	t5, t5, -1
	addi	t7, t7, 4
	addi	t6, t6, 4
	bnez	t5, .LBB0_3
# %bb.4:                                # %for.cond.cleanup9
                                        #   in Loop: Header=BB0_2 Depth=1
	or	t5, a0, t4
	slli	t5, t5, 2
	add	t5, a1, t5
	sw	t3, 0(t5)
	or	t3, a2, t4
	slli	t3, t3, 2
	add	t3, a1, t3
	sw	t2, 0(t3)
	or	t2, a3, t4
	slli	t2, t2, 2
	add	t2, a1, t2
	sw	t0, 0(t2)
	or	t0, a4, t4
	slli	t0, t0, 2
	add	t0, a1, t0
	sw	a23, 0(t0)
	or	a23, a0, a22
	slli	a23, a23, 2
	add	a23, a1, a23
	sw	a20, 0(a23)
	or	a20, a2, a22
	slli	a20, a20, 2
	add	a20, a1, a20
	sw	t1, 0(a20)
	or	a20, a3, a22
	slli	a20, a20, 2
	add	a20, a1, a20
	sw	a21, 0(a20)
	or	a20, a4, a22
	slli	a20, a20, 2
	add	a20, a1, a20
	sw	a19, 0(a20)
	or	a19, a0, a17
	slli	a19, a19, 2
	add	a19, a1, a19
	sw	a18, 0(a19)
	or	a18, a2, a17
	slli	a18, a18, 2
	add	a18, a1, a18
	sw	a16, 0(a18)
	or	a16, a3, a17
	slli	a16, a16, 2
	add	a16, a1, a16
	sw	a15, 0(a16)
	or	a15, a4, a17
	slli	a15, a15, 2
	add	a15, a1, a15
	sw	a14, 0(a15)
	or	a14, a0, a12
	slli	a14, a14, 2
	add	a14, a1, a14
	sw	a13, 0(a14)
	or	a13, a2, a12
	slli	a13, a13, 2
	add	a13, a1, a13
	sw	a11, 0(a13)
	or	a11, a3, a12
	slli	a11, a11, 2
	add	a11, a1, a11
	sw	a9, 0(a11)
	or	a9, a4, a12
	slli	a9, a9, 2
	add	a9, a1, a9
	sw	a8, 0(a9)
	addi	a12, a10, 4
	addi	a5, a5, 1024
	bltu	a10, a7, .LBB0_2
.LBB0_5:                                # %for.cond.cleanup
	ret
.Lfunc_end0:
	.size	gemm, .Lfunc_end0-gemm
                                        # -- End function
	.section	.text.startup,"ax",@progbits
	.globl	main                            # -- Begin function main
	.p2align	2
	.type	main,@function
main:                                   # @main
# %bb.0:                                # %entry
	addi	sp, sp, -16
	sw	ra, 12(sp)                      # 4-byte Folded Spill
	addi	a0, zero, %lo(gemm)
	li	a1, 4
	#APP
	.insn r 11, 1, 0, zero, a1, a0
	#NO_APP
	call	gemm
	li	a0, 0
	lw	ra, 12(sp)                      # 4-byte Folded Reload
	addi	sp, sp, 16
	ret
.Lfunc_end1:
	.size	main, .Lfunc_end1-main
                                        # -- End function
	.type	A,@object                       # @A
	.section	.sdata,"aw",@progbits
	.globl	A
	.p2align	2
A:
	.word	2684354560
	.size	A, 4

	.type	B,@object                       # @B
	.globl	B
	.p2align	2
B:
	.word	2701131776
	.size	B, 4

	.type	C,@object                       # @C
	.globl	C
	.p2align	2
C:
	.word	3221225472
	.size	C, 4

	.ident	"clang version 16.0.6 (https://github.com/richardyrh/llvm.git dc8d74a262a58ffc85dbc3813e57bd92a2827954)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym gemm
