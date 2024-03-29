/* BL808 M0 */

/* Configure the CPU type */
OUTPUT_ARCH("riscv")
/* Configure the entry point */
ENTRY(_start)


/*
 * Here we define all memory ranges.
 * The memory map can be found in the datasheet.
 * Note that part of the DRAM is used for peripherals 
 *   and TCM. The real RAM for the application is split
 *   between DRAM and VRAM.
 */
MEMORY {
    flash      (rx) : ORIGIN = 0x58000000, LENGTH = 1M
    ram        (wx) : ORIGIN = 0x62020000, LENGTH = 160K + 64K
    xram        (w) : ORIGIN = 0x40000000, LENGTH = 16K
}

SECTIONS {

    /* Executable code section.
     * Note that the init section is intentionnaly added first.
     */
    .text : {

        . = ALIGN(4);
        _ld_text_start = .;

        *(.text.init)
        *(.text.vector)
        *(.text .text.*)

        . = ALIGN(4);
        _ld_text_end = .;

    } >flash

    /*
     * The read only initialized data is kept in flash.
     * For now, we can't place it in RAM: it crashes.
     */
    .rodata : {

        . = ALIGN(4);
        _ld_rodata_start = .;

        *(.rodata .rodata.*)

        . = ALIGN(4);
        _ld_rodata_end = .;

    } >flash

    /* Here we save the start address where the data will be 
     * initialy placed in flash. It will be later copied to
     * read/write RAM.
     */
    . = ALIGN(4);
    _ld_data_load_start = .;

    /* Data with initial value saved in Flash and dynamically loaded 
     * in RAM. This also contains executable some ramtext.
     */
    .data : AT(_ld_data_load_start) {

        . = ALIGN(4);
        _ld_data_start = .;
        _ld_global_pointer = . + 0x800;

        *(.sdata .sdata.*) 
        *(.data .data.*)

        /* This special section can be used to copy some text at
         * the end of the data section, in RAM. This is made for
         * function that need to execute fast.
         */
        *(.ramtext)
        
        . = ALIGN(4);
        _ld_data_end = .;

    } >ram

    /* Data without initial value, this RAM section will be written 
     * with all zeros at startup.
     */
    .bss (NOLOAD) : {

        . = ALIGN(4);
        _ld_bss_start = .;

        *(.sbss .sbss.*) 
        *(.bss .bss.*)

        . = ALIGN(4);
        _ld_bss_end = .;

    } >ram

    /* Heap space, taking the remaining space available in RAM.
     */
    .heap (NOLOAD) : {

        . = ALIGN(4);
        _ld_stack_start = .;
        _ld_stack_end = . + 4096;

        _ld_heap_start = _ld_stack_end;
        . = ORIGIN(ram) + LENGTH(ram);
        _ld_heap_end = .;

    } >ram

}
