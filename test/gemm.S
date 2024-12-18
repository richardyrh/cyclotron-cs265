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
	addi	sp, sp, -16
	sw	ra, 12(sp)                      # 4-byte Folded Spill
	sw	s57, 8(sp)                      # 4-byte Folded Spill
	sw	s58, 4(sp)                      # 4-byte Folded Spill
	sw	s59, 0(sp)                      # 4-byte Folded Spill
	#APP
	csrr	a6, mhartid
	#NO_APP
	srli	a0, a6, 2
	andi	t2, a0, -4
	li	a0, 63
	bltu	a0, t2, .LBB0_5
# %bb.1:                                # %for.body.lr.ph
	slli	a1, a6, 28
	srli	a0, a1, 26
	lw	a4, %lo(A)(zero)
	lw	a7, %lo(B)(zero)
	lw	a1, %lo(C)(zero)
	addi	a2, a0, 1
	addi	a3, a0, 2
	addi	a5, a0, 3
	srli	a9, a6, 4
	slli	a8, a9, 10
	add	a9, a8, a4
	addi	a4, a9, 512
	andi	a9, a6, 15
	slli	a6, a9, 10
	add	a10, a6, a7
	addi	a6, a10, 512
	li	a7, 60
.LBB0_2:                                # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_3 Depth 2
	li	t0, 0
	li	a23, 0
	li	a21, 0
	li	a20, 0
	li	a18, 0
	li	a8, 0
	li	a9, 0
	li	a11, 0
	li	a12, 0
	li	a13, 0
	li	a14, 0
	li	a15, 0
	li	a16, 0
	li	a17, 0
	li	a19, 0
	li	a22, 0
	mv	a10, t2
	slli	s57, t2, 6
	ori	s58, s57, 64
	ori	s59, s57, 128
	ori	ra, s57, 192
	li	t1, 64
	mv	t2, a6
	mv	t3, a4
.LBB0_3:                                # %for.body10
                                        #   Parent Loop BB0_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lw	t4, -512(t3)
	lw	t5, -256(t3)
	lw	t6, 0(t3)
	lw	t8, 256(t3)
	lw	t7, -512(t2)
	lw	t11, -256(t2)
	lw	t15, 0(t2)
	lw	t19, 256(t2)
	mul	t9, t7, t4
	add	t0, t9, t0
	mul	t9, t11, t4
	add	a23, t9, a23
	mul	t9, t15, t4
	add	a21, t9, a21
	mul	t9, t19, t4
	add	a20, t9, a20
	mul	t4, t7, t5
	add	a18, t4, a18
	mul	t4, t11, t5
	add	a22, t4, a22
	mul	t4, t15, t5
	add	a19, t4, a19
	mul	t10, t19, t5
	add	a17, t10, a17
	mul	t4, t7, t6
	add	a16, t4, a16
	mul	t4, t11, t6
	add	a15, t4, a15
	mul	t4, t15, t6
	add	a14, t4, a14
	mul	t5, t19, t6
	add	a13, t5, a13
	mul	t5, t7, t8
	add	a12, t5, a12
	mul	t5, t11, t8
	add	a11, t5, a11
	mul	t5, t15, t8
	add	a9, t5, a9
	mul	t5, t19, t8
	add	a8, t5, a8
	addi	t1, t1, -1
	addi	t3, t3, 4
	addi	t2, t2, 4
	bnez	t1, .LBB0_3
# %bb.4:                                # %for.cond.cleanup9
                                        #   in Loop: Header=BB0_2 Depth=1
	or	t2, a0, s57
	slli	t1, t2, 2
	add	t3, a1, t1
	sw	t0, 0(t3)
	or	t1, a2, s57
	slli	t0, t1, 2
	add	t1, a1, t0
	sw	a23, 0(t1)
	or	a23, a3, s57
	slli	t0, a23, 2
	add	t1, a1, t0
	sw	a21, 0(t1)
	or	t1, a5, s57
	slli	a21, t1, 2
	add	t1, a1, a21
	sw	a20, 0(t1)
	or	a20, a0, s58
	slli	a21, a20, 2
	add	a20, a1, a21
	sw	a18, 0(a20)
	or	a21, a2, s58
	slli	a18, a21, 2
	add	a20, a1, a18
	sw	a22, 0(a20)
	or	a20, a3, s58
	slli	a18, a20, 2
	add	a20, a1, a18
	sw	a19, 0(a20)
	or	a20, a5, s58
	slli	a18, a20, 2
	add	a20, a1, a18
	sw	a17, 0(a20)
	or	a17, a0, s59
	slli	a18, a17, 2
	add	a17, a1, a18
	sw	a16, 0(a17)
	or	a17, a2, s59
	slli	a16, a17, 2
	add	a17, a1, a16
	sw	a15, 0(a17)
	or	a16, a3, s59
	slli	a17, a16, 2
	add	a16, a1, a17
	sw	a14, 0(a16)
	or	a14, a5, s59
	slli	a16, a14, 2
	add	a14, a1, a16
	sw	a13, 0(a14)
	or	a15, a0, ra
	slli	a13, a15, 2
	add	a14, a1, a13
	sw	a12, 0(a14)
	or	a14, a2, ra
	slli	a12, a14, 2
	add	a13, a1, a12
	sw	a11, 0(a13)
	or	a11, a3, ra
	slli	a12, a11, 2
	add	a14, a1, a12
	sw	a9, 0(a14)
	or	a12, a5, ra
	slli	a9, a12, 2
	add	a14, a1, a9
	sw	a8, 0(a14)
	addi	t2, a10, 4
	addi	a4, a4, 1024
	bltu	a10, a7, .LBB0_2
.LBB0_5:                                # %for.cond.cleanup
	lw	ra, 12(sp)                      # 4-byte Folded Reload
	lw	s57, 8(sp)                      # 4-byte Folded Reload
	lw	s58, 4(sp)                      # 4-byte Folded Reload
	lw	s59, 0(sp)                      # 4-byte Folded Reload
	addi	sp, sp, 16
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

	.ident	"clang version 16.0.6 (https://github.com/richardyrh/llvm.git 467ec379ec18af27cb9cf125162b1976a37b4d8a)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym gemm
