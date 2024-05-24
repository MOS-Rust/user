    .globl msyscall;
    .align 2;                         
    .type msyscall, @function;
    .ent msyscall;
msyscall:
	.frame sp, 0, ra

    syscall
    jr $31

	.end msyscall;                                                                             
	.size msyscall, .- msyscall
