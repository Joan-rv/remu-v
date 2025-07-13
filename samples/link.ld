ENTRY(_start)

SECTIONS {
    . = 0x0;
    .text : {
        *(.text._start)   /* Ensure _start goes first */
        *(.text*)
    }

    .rodata : {
        *(.rodata*)
    }

    .data : {
        *(.data*)
     }

    .bss : {
        *(.bss*)
        *(COMMON)
    }
}
