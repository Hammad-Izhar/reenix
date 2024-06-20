ENTRY(_start);

SECTIONS
{
    . = 1M;

    .multiboot :
    {
        KEEP(.multiboot)
    }

    .text :
    {
        *(.text .text.*)
        *(.rodata .rodata.*)
    }

    .data : { *(.data .data.*) }

    .bss : { *(.bss .bss.*) }

    /DISCARD/ : { *(.eh_frame) }
}