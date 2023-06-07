/* BL808 D0 */

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
    flash      (rx) : ORIGIN = 0x58100000, LENGTH = 1M
    /* peripheral (rx) : ORIGIN = 0x3EF80000, LENGTH = 448K */
    ram        (wx) : ORIGIN = 0x3EFF0000, LENGTH = 64K + 32K
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

        /* Purposedly placed after data. */
        *(.rodata .rodata.*)
        
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
        _ld_heap_start = .;

        . = ORIGIN(ram) + LENGTH(ram);
        _ld_heap_end = .;

    } >ram

}
