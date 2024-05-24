#include <asm/asm.h>

.text
.globl _start
_start:
	lw      $4, 0($29)
	lw      $5, 4($29)
	jal     libmain
