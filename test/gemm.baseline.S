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
	addi	sp, sp, -112
	sw	ra, 108(sp)                     # 4-byte Folded Spill
	sw	s0, 104(sp)                     # 4-byte Folded Spill
	sw	s1, 100(sp)                     # 4-byte Folded Spill
	sw	s2, 96(sp)                      # 4-byte Folded Spill
	sw	s3, 92(sp)                      # 4-byte Folded Spill
	sw	s4, 88(sp)                      # 4-byte Folded Spill
	sw	s5, 84(sp)                      # 4-byte Folded Spill
	sw	s6, 80(sp)                      # 4-byte Folded Spill
	sw	s7, 76(sp)                      # 4-byte Folded Spill
	sw	s8, 72(sp)                      # 4-byte Folded Spill
	sw	s9, 68(sp)                      # 4-byte Folded Spill
	sw	s10, 64(sp)                     # 4-byte Folded Spill
	sw	s11, 60(sp)                     # 4-byte Folded Spill
	#APP
	csrr	a5, mhartid
	#NO_APP
	srli	a0, a5, 2
	andi	a2, a0, -4
	li	a0, 63
	bltu	a0, a2, .LBB0_5
# %bb.1:                                # %for.body.lr.ph
	slli	a0, a5, 28
	srli	a0, a0, 26
	lw	a3, %lo(A)(zero)
	lw	a6, %lo(B)(zero)
	lw	a1, %lo(C)(zero)
	sw	a1, 28(sp)                      # 4-byte Folded Spill
	addi	a1, a0, 1
	sw	a1, 24(sp)                      # 4-byte Folded Spill
	addi	a1, a0, 2
	sw	a1, 20(sp)                      # 4-byte Folded Spill
	sw	a0, 32(sp)                      # 4-byte Folded Spill
	addi	a0, a0, 3
	sw	a0, 16(sp)                      # 4-byte Folded Spill
	srli	a7, a5, 4
	slli	a7, a7, 10
	add	a3, a7, a3
	addi	a0, a3, 512
	andi	a5, a5, 15
	slli	a5, a5, 10
	add	a5, a5, a6
	addi	a1, a5, 512
	sw	a1, 12(sp)                      # 4-byte Folded Spill
.LBB0_2:                                # %for.body
                                        # =>This Loop Header: Depth=1
                                        #     Child Loop BB0_3 Depth 2
	li	ra, 0
	li	s11, 0
	li	s9, 0
	li	s8, 0
	li	s5, 0
	li	t0, 0
	li	t1, 0
	li	t3, 0
	li	t5, 0
	li	t6, 0
	li	s0, 0
	li	s1, 0
	li	s3, 0
	li	s4, 0
	li	s6, 0
	li	s10, 0
	sw	a2, 52(sp)                      # 4-byte Folded Spill
	slli	a2, a2, 6
	ori	a1, a2, 64
	sw	a1, 40(sp)                      # 4-byte Folded Spill
	ori	a1, a2, 128
	sw	a1, 44(sp)                      # 4-byte Folded Spill
	sw	a2, 36(sp)                      # 4-byte Folded Spill
	ori	a1, a2, 192
	sw	a1, 48(sp)                      # 4-byte Folded Spill
	li	a6, 64
	lw	a2, 12(sp)                      # 4-byte Folded Reload
	sw	a0, 56(sp)                      # 4-byte Folded Spill
	mv	a5, a0
.LBB0_3:                                # %for.body10
                                        #   Parent Loop BB0_2 Depth=1
                                        # =>  This Inner Loop Header: Depth=2
	lw	a0, -512(a5)
	lw	a4, -256(a5)
	lw	t2, 0(a5)
	lw	t4, 256(a5)
	lw	s2, -512(a2)
	lw	a3, -256(a2)
	lw	s7, 0(a2)
	lw	a7, 256(a2)
	mul	a1, s2, a0
	add	ra, a1, ra
	mul	a1, a3, a0
	add	s11, a1, s11
	mul	a1, s7, a0
	add	s9, a1, s9
	mul	a0, a7, a0
	add	s8, a0, s8
	mul	a0, s2, a4
	add	s5, a0, s5
	mul	a0, a3, a4
	add	s10, a0, s10
	mul	a0, s7, a4
	add	s6, a0, s6
	mul	a0, a7, a4
	add	s4, a0, s4
	mul	a0, s2, t2
	add	s3, a0, s3
	mul	a0, a3, t2
	add	s1, a0, s1
	mul	a0, s7, t2
	add	s0, a0, s0
	mul	a0, a7, t2
	add	t6, a0, t6
	mul	a0, s2, t4
	add	t5, a0, t5
	mul	a0, a3, t4
	add	t3, a0, t3
	mul	a0, s7, t4
	add	t1, a0, t1
	mul	a0, a7, t4
	add	t0, a0, t0
	addi	a6, a6, -1
	addi	a5, a5, 4
	addi	a2, a2, 4
	bnez	a6, .LBB0_3
# %bb.4:                                # %for.cond.cleanup9
                                        #   in Loop: Header=BB0_2 Depth=1
	lw	a0, 32(sp)                      # 4-byte Folded Reload
	lw	a6, 36(sp)                      # 4-byte Folded Reload
	or	a2, a0, a6
	slli	a2, a2, 2
	lw	a1, 28(sp)                      # 4-byte Folded Reload
	add	a2, a1, a2
	sw	ra, 0(a2)
	lw	a3, 24(sp)                      # 4-byte Folded Reload
	or	a2, a3, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s11, 0(a2)
	lw	a4, 20(sp)                      # 4-byte Folded Reload
	or	a2, a4, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s9, 0(a2)
	lw	a5, 16(sp)                      # 4-byte Folded Reload
	or	a2, a5, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s8, 0(a2)
	lw	a6, 40(sp)                      # 4-byte Folded Reload
	or	a2, a0, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s5, 0(a2)
	or	a2, a3, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s10, 0(a2)
	or	a2, a4, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s6, 0(a2)
	or	a2, a5, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s4, 0(a2)
	lw	a6, 44(sp)                      # 4-byte Folded Reload
	or	a2, a0, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s3, 0(a2)
	or	a2, a3, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s1, 0(a2)
	or	a2, a4, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	s0, 0(a2)
	or	a2, a5, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	t6, 0(a2)
	lw	a6, 48(sp)                      # 4-byte Folded Reload
	or	a2, a0, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	t5, 0(a2)
	or	a2, a3, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	t3, 0(a2)
	or	a2, a4, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	t1, 0(a2)
	or	a2, a5, a6
	slli	a2, a2, 2
	add	a2, a1, a2
	sw	t0, 0(a2)
	lw	a3, 52(sp)                      # 4-byte Folded Reload
	addi	a2, a3, 4
	lw	a0, 56(sp)                      # 4-byte Folded Reload
	addi	a0, a0, 1024
	li	a1, 60
	bltu	a3, a1, .LBB0_2
.LBB0_5:                                # %for.cond.cleanup
	lw	ra, 108(sp)                     # 4-byte Folded Reload
	lw	s0, 104(sp)                     # 4-byte Folded Reload
	lw	s1, 100(sp)                     # 4-byte Folded Reload
	lw	s2, 96(sp)                      # 4-byte Folded Reload
	lw	s3, 92(sp)                      # 4-byte Folded Reload
	lw	s4, 88(sp)                      # 4-byte Folded Reload
	lw	s5, 84(sp)                      # 4-byte Folded Reload
	lw	s6, 80(sp)                      # 4-byte Folded Reload
	lw	s7, 76(sp)                      # 4-byte Folded Reload
	lw	s8, 72(sp)                      # 4-byte Folded Reload
	lw	s9, 68(sp)                      # 4-byte Folded Reload
	lw	s10, 64(sp)                     # 4-byte Folded Reload
	lw	s11, 60(sp)                     # 4-byte Folded Reload
	addi	sp, sp, 112
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

	.ident	"clang version 16.0.6 (https://github.com/richardyrh/llvm.git 73378b24a7b03f8325640b18250ea3f4df9ce7a0)"
	.section	".note.GNU-stack","",@progbits
	.addrsig
	.addrsig_sym gemm
