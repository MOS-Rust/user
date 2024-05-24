    .globl msyscall;
    .align 2;                         
    .type msyscall, @function;
    .ent msyscall;
msyscall:
	.frame $29, 0, $31

    syscall
    jr $31

	.end msyscall;                                                                             
	.size msyscall, .- msyscall
