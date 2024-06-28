OUTPUT_FORMAT(elf64-x86-64)
ENTRY(_start);

KERNEL_PMA = 1M;
KERNEL_VMA = 0xC0000000;

SECTIONS
{
    . = KERNEL_PMA;

    .multiboot : { KEEP(*(.multiboot)) }


    . += KERNEL_VMA;

    __kernel_start = .;
    
    __text_start = .;
    .text  ALIGN(4K) : AT(ADDR(.text) - KERNEL_VMA) { *(.text); *(.text.*) }
    __text_end = .;

    __rodata_start = .;
    .rodata ALIGN(4K) : AT(ADDR(.rodata) - KERNEL_VMA) { *(.rodata); *(.rodata.*) }
    __rodata_end = .;

    __data_start = .;
    .data ALIGN(4K) : AT(ADDR(.data) - KERNEL_VMA) { *(.data); *(.data.*) }
    __data_end = .;

    __bss_start = .;
    .bss ALIGN(4K) : AT(ADDR(.bss) - KERNEL_VMA) { *(.bss); *(.bss.*) }
    __bss_end = .;
    
    __kernel_end = .;
}