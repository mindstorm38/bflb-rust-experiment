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
    flash      (rx) : ORIGIN = 0x58000000, LENGTH = 32M
    /* peripheral (rx) : ORIGIN = 0x3EF80000, LENGTH = 448K */
    stack       (w) : ORIGIN = 0x62020000, LENGTH = 4K
    ram         (w) : ORIGIN = 0x62021000, LENGTH = 160K + 64K - 4K
    xram        (w) : ORIGIN = 0x40000000, LENGTH = 16K
}

/*
 * text: executable code
 * data: initialized data
 * rodata: initialized data (read only)
 * bss: uninitialized data
 */
SECTIONS {

    /*
     * Executable code section.
     * Note that the init section is intentionnaly added first.
     */
    .text : {

        . = ALIGN(4);
        _ld_text_start = .;

        *(.text.init)
        *(.text .text.*)

        . = ALIGN(4);
        _ld_text_end = .;

    } >flash

    /*
     * The read only initialized data is kept in flash.
     */
    .rodata : {

        . = ALIGN(4);
        _ld_rodata_start = .;

        *(.rodata .rodata.*)

        . = ALIGN(4);
        _ld_rodata_end = .;

    } >flash

    /* 
     * Here we save the start address where the data will be 
     * initialy placed in flash. It will be later copied to
     * read/write RAM.
     */
    . = ALIGN(4);
    _ld_data_load_start = .;

    .data : AT(_ld_data_load_start) {

        . = ALIGN(4);
        _ld_data_start = .;
        _ld_global_pointer = . + 0x800;

        *(.sdata .sdata.*) 
        *(.data .data.*)
        
        . = ALIGN(4);
        _ld_data_end = .;

    } >ram

    .bss (NOLOAD) : {

        . = ALIGN(4);
        _ld_bss_start = .;

        *(.sbss .sbss.*) 
        *(.bss .bss.*)

        . = ALIGN(4);
        _ld_bss_end = .;

    } > ram

    _ld_stack_origin = ORIGIN(stack);
    _ld_stack_top = _ld_stack_origin + LENGTH(stack);

}
