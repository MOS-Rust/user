OUTPUT_ARCH(mips)

ENTRY(_start)

PHDRS {
	code PT_LOAD FLAGS (5);
	data PT_LOAD FLAGS (6);
}

SECTIONS {
    /DISCARD/ : {
	    *(.comment)
	    *(.eh_frame)
	    *(.eh_frame_hdr)
    }
	. = 0x00400000;

	.text : {
		*(.text)
		*(.rodata)
		*(.rodata.*)
	} : code

	.data ALIGN(4096) : {
		*(.data)
	} : data

	.bss ALIGN(4096) : {
		*(.bss)
	} : data

	end = . ;
}
