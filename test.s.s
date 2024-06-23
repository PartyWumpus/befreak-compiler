	.text
	.file	"test.s"
	.globl	main                            # -- Begin function main
	.p2align	4, 0x90
	.type	main,@function
main:                                   # @main
	.cfi_startproc
# %bb.0:
	pushq	%rbx
	.cfi_def_cfa_offset 16
	.cfi_offset %rbx, -16
	movq	primary_offset@GOTPCREL(%rip), %rax
	movl	(%rax), %ecx
	incl	%ecx
	movl	%ecx, (%rax)
	movslq	%ecx, %rax
	movq	primary_stack@GOTPCREL(%rip), %rsi
	leaq	(%rsi,%rax,4), %rbx
	addl	$25, (%rsi,%rax,4)
	movl	$.Lint_str, %edi
	xorl	%eax, %eax
	callq	printf
	movl	$.Lint_str, %edi
	movq	%rbx, %rsi
	xorl	%eax, %eax
	callq	printf
	movl	$5, %eax
	popq	%rbx
	.cfi_def_cfa_offset 8
	retq
.Lfunc_end0:
	.size	main, .Lfunc_end0-main
	.cfi_endproc
                                        # -- End function
	.globl	increment_offset                # -- Begin function increment_offset
	.p2align	4, 0x90
	.type	increment_offset,@function
increment_offset:                       # @increment_offset
	.cfi_startproc
# %bb.0:
	movq	primary_offset@GOTPCREL(%rip), %rax
	addl	$16, (%rax)
	retq
.Lfunc_end1:
	.size	increment_offset, .Lfunc_end1-increment_offset
	.cfi_endproc
                                        # -- End function
	.globl	stack_operation                 # -- Begin function stack_operation
	.p2align	4, 0x90
	.type	stack_operation,@function
stack_operation:                        # @stack_operation
	.cfi_startproc
# %bb.0:
	movq	primary_stack@GOTPCREL(%rip), %rax
	movl	$1, 24(%rax)
	retq
.Lfunc_end2:
	.size	stack_operation, .Lfunc_end2-stack_operation
	.cfi_endproc
                                        # -- End function
	.type	.Lint_str,@object               # @int_str
	.section	.rodata.str1.1,"aMS",@progbits,1
.Lint_str:
	.asciz	"%d\n"
	.size	.Lint_str, 4

	.type	primary_stack,@object           # @primary_stack
	.bss
	.globl	primary_stack
	.p2align	2, 0x0
primary_stack:
	.zero	160
	.size	primary_stack, 160

	.type	primary_offset,@object          # @primary_offset
	.globl	primary_offset
	.p2align	2, 0x0
primary_offset:
	.long	0                               # 0x0
	.size	primary_offset, 4

	.type	control_stack,@object           # @control_stack
	.globl	control_stack
	.p2align	2, 0x0
control_stack:
	.zero	160
	.size	control_stack, 160

	.type	control_offset,@object          # @control_offset
	.globl	control_offset
	.p2align	2, 0x0
control_offset:
	.long	0                               # 0x0
	.size	control_offset, 4

	.section	".note.GNU-stack","",@progbits
