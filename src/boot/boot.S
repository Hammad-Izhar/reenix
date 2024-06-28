.global _start

.set KERNEL_OFFSET, 0xC0000000
.set KERNEL_STACK_SIZE, 0x1000 // 4 KiB

.set KERNEL_STACK_START_PA, __kernel_stack_start - KERNEL_OFFSET
.set PML4_PA, pml4 - KERNEL_OFFSET  
.set PDPT_PA, pdpt - KERNEL_OFFSET
.set PDT_PA, pdt - KERNEL_OFFSET

.section .bss
    .align 0x1000
    pml4:
        .skip 0x1000
    pdpt:
        .skip 0x1000
    pdt:
        .skip 0x1000
    __kernel_stack_end:
        .skip KERNEL_STACK_SIZE
    __kernel_stack_start:

.section .rodata
    gdt64:
        .quad 0
        .quad (1<<43) | (1<<44) | (1<<47) | (1<<53)
    gdt64_pointer:
        .word gdt64_pointer - gdt64 - 1
        .quad gdt64

.section .text
    .code32
    _start:
        cli

        mov esp, offset KERNEL_STACK_START_PA
        mov ebp, offset KERNEL_STACK_START_PA

        call setup_page_tables
        call enable_paging

        lgdt [gdt64_pointer]

        ljmp 0x8, offset _start64

    setup_page_tables:
        mov eax, offset PDPT_PA
        or eax, 0b11
        mov [PML4_PA], eax

        mov eax, offset PDT_PA
        or eax, 0b11
        mov [PDPT_PA], eax
        mov [PDPT_PA + 3 * 8], eax
        
        mov ecx, 0
        .map_pdt:
            mov eax, 0x200000
            mul ecx
            or eax, 0b10000011
            mov [PDT_PA + ecx * 8], eax

            inc ecx
            cmp ecx, 512
            jne .map_pdt

        ret

    enable_paging:
        mov eax, offset PML4_PA
        mov cr3, eax

        mov eax, cr4
        or eax, 1 << 5
        mov cr4, eax


        mov ecx, 0xC0000080
        rdmsr
        or eax, 1 << 8
        wrmsr

        mov eax, cr0
        or eax, 1 << 31
        mov cr0, eax

        ret
    
    error:
        mov dword ptr [0xb8000], 0x4f524f45
        mov dword ptr [0xb8004], 0x4f3a4f52
        mov dword ptr [0xb8008], 0x4f204f20
        mov byte  ptr [0xb800a], al
        hlt
    
    .code64
    _start64:
        mov ax, 0
        mov ss, ax
        mov ds, ax
        mov es, ax
        mov fs, ax
        mov gs, ax

        .extern kmain
        call kmain
        call error