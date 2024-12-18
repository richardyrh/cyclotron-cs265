	.text
	.attribute	4, 16
	.attribute	5, "rv32i2p0_m2p0_a2p0_f2p0"
	.file	"vecadd.c"
	.section	.text.vecadd,"ax",@progbits
	.globl	vecadd                          # -- Begin function vecadd
	.p2align	2
	.type	vecadd,@function
vecadd:                                 # @vecadd
# %bb.0:                                # %entry
	#APP
	csrr	a0, mhartid
	#NO_APP
	lw	a1, %lo(A)(zero)
	lw	a2, %lo(B)(zero)
	lw	a3, %lo(C)(zero)
	slli	a0, a0, 2
	addi	a4, a0, 1024
	add	a0, a1, a4
	add	a1, a2, a4
	add	a2, a3, a4
	li	a3, -16
	li	a4, 2032
.LBB0_1:                                # %for.body
                                        # =>This Inner Loop Header: Depth=1
	lw	a5, -1024(a0)
	lw	a6, -1024(a1)
	add	a5, a6, a5
	sw	a5, -1024(a2)
	lw	a5, -960(a0)
	lw	a6, -960(a1)
	add	a5, a6, a5
	sw	a5, -960(a2)
	lw	a5, -896(a0)
	lw	a6, -896(a1)
	add	a5, a6, a5
	sw	a5, -896(a2)
	lw	a5, -832(a0)
	lw	a6, -832(a1)
	add	a5, a6, a5
	sw	a5, -832(a2)
	lw	a5, -768(a0)
	lw	a6, -768(a1)
	add	a5, a6, a5
	sw	a5, -768(a2)
	lw	a5, -704(a0)
	lw	a6, -704(a1)
	add	a5, a6, a5
	sw	a5, -704(a2)
	lw	a5, -640(a0)
	lw	a6, -640(a1)
	add	a5, a6, a5
	sw	a5, -640(a2)
	lw	a5, -576(a0)
	lw	a6, -576(a1)
	add	a5, a6, a5
	sw	a5, -576(a2)
	lw	a5, -512(a0)
	lw	a6, -512(a1)
	add	a5, a6, a5
	sw	a5, -512(a2)
	lw	a5, -448(a0)
	lw	a6, -448(a1)
	add	a5, a6, a5
	sw	a5, -448(a2)
	lw	a5, -384(a0)
	lw	a6, -384(a1)
	add	a5, a6, a5
	sw	a5, -384(a2)
	lw	a5, -320(a0)
	lw	a6, -320(a1)
	add	a5, a6, a5
	sw	a5, -320(a2)
	lw	a5, -256(a0)
	lw	a6, -256(a1)
	add	a5, a6, a5
	sw	a5, -256(a2)
	lw	a5, -192(a0)
	lw	a6, -192(a1)
	add	a5, a6, a5
	sw	a5, -192(a2)
	lw	a5, -128(a0)
	lw	a6, -128(a1)
	add	a5, a6, a5
	sw	a5, -128(a2)
	lw	a5, -64(a0)
	lw	a6, -64(a1)
	add	a5, a6, a5
	sw	a5, -64(a2)
	lw	a5, 0(a0)
	lw	a6, 0(a1)
	add	a5, a6, a5
	sw	a5, 0(a2)
	lw	a5, 64(a0)
	lw	a6, 64(a1)
	add	a5, a6, a5
	sw	a5, 64(a2)
	lw	a5, 128(a0)
	lw	a6, 128(a1)
	add	a5, a6, a5
	sw	a5, 128(a2)
	lw	a5, 192(a0)
	lw	a6, 192(a1)
	add	a5, a6, a5
	sw	a5, 192(a2)
	lw	a5, 256(a0)
	lw	a6, 256(a1)
	add	a5, a6, a5
	sw	a5, 256(a2)
	lw	a5, 320(a0)
	lw	a6, 320(a1)
	add	a5, a6, a5
	sw	a5, 320(a2)
	lw	a5, 384(a0)
	lw	a6, 384(a1)
	add	a5, a6, a5
	sw	a5, 384(a2)
	lw	a5, 448(a0)
	lw	a6, 448(a1)
	add	a5, a6, a5
	sw	a5, 448(a2)
	lw	a5, 512(a0)
	lw	a6, 512(a1)
	add	a5, a6, a5
	sw	a5, 512(a2)
	lw	a5, 576(a0)
	lw	a6, 576(a1)
	add	a5, a6, a5
	sw	a5, 576(a2)
	lw	a5, 640(a0)
	lw	a6, 640(a1)
	add	a5, a6, a5
	sw	a5, 640(a2)
	lw	a5, 704(a0)
	lw	a6, 704(a1)
	add	a5, a6, a5
	sw	a5, 704(a2)
	lw	a5, 768(a0)
	lw	a6, 768(a1)
	add	a5, a6, a5
	sw	a5, 768(a2)
	lw	a5, 832(a0)
	lw	a6, 832(a1)
	add	a5, a6, a5
	sw	a5, 832(a2)
	lw	a5, 896(a0)
	lw	a6, 896(a1)
	add	a5, a6, a5
	sw	a5, 896(a2)
	lw	a5, 960(a0)
	lw	a6, 960(a1)
	add	a5, a6, a5
	sw	a5, 960(a2)
	addi	a3, a3, 512
	addi	a0, a0, 2048
	addi	a1, a1, 2048
	addi	a2, a2, 2048
	bltu	a3, a4, .LBB0_1
# %bb.2:                                # %for.cond.cleanup
	ret
.Lfunc_end0:
	.size	vecadd, .Lfunc_end0-vecadd
                                        # -- End function
	.section	.text.startup,"ax",@progbits
	.globl	main                            # -- Begin function main
	.p2align	2
	.type	main,@function
main:                                   # @main
# %bb.0:                                # %entry
	addi	sp, sp, -16
	sw	ra, 12(sp)                      # 4-byte Folded Spill
	addi	a0, zero, %lo(vecadd)
	li	a1, 4
	#APP
	.insn r 11, 1, 0, zero, a1, a0
	#NO_APP
	call	vecadd
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
	.addrsig_sym vecadd
