SECTIONS
{
    .init : ALIGN(8) {
        LONG(0xaedb041d);
        LONG(0xaedb041d);
        KEEP(*(.text._start))
    } > FLASH AT > FLASH_LMA

    .text : ALIGN(8) {
        *(.text .text.*) 
    } > FLASH AT > FLASH_LMA

    .rodata : ALIGN(8) {
        *(.rodata .rodata.*)
        *(.srodata .srodata.*)
        *(.rodata_*.*)
        . = ALIGN(8);
    } > FLASH AT > FLASH_LMA

    .data : ALIGN(8) {
        __sdata = .;
        *(.sdata .sdata.* .sdata2 .sdata2.*);
        *(.data .data.*);
        . = ALIGN(8);
        __edata = .;
    } > RAM AT > FLASH_LMA

    __sidata = LOADADDR(.data) - ORIGIN(FLASH_LMA) + ORIGIN(FLASH);

    .bss (NOLOAD): ALIGN(8) {
        __bss_start = .;
        *(.sbss .sbss.* .bss .bss.*);
        . = ALIGN(8);
        __bss_end = .;
    } > RAM

    __stack = ORIGIN(RAM) + LENGTH(RAM) - 16;

    /DISCARD/ :
    {
        *(.eh_frame);
    }
}
