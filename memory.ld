ENTRY(reset_handler)

/* Define memory regions */
MEMORY
{
    FLASH (rx) : ORIGIN = 0x08000000, LENGTH = 64K
    RAM (rwx) : ORIGIN = 0x20000000, LENGTH = 12K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM);
_min_stack_size = 0x400; /* 1KB stack size */
_min_heap_size = 0x400; /* 1KB heap size */

SECTIONS
{
    .text :
    {
        . = ORIGIN(FLASH); /*ALIGN(4);*/
        LONG(_start_of_stack)
        KEEP(*(.isr_vector));
        *(.text)
        *(.text.*)
        . = ALIGN(4);
    } > FLASH

    .rodata :
    {
        . = ALIGN(4);
        *(.rodata)
        *(.rodata.*)
        . = ALIGN(4);
    } > FLASH

    .data :
    {
        _sidata = LOADADDR(.data); /* This returns the FLASH (LMA) address of the data section */
        . = ALIGN(4);
        _sdata = .; /* start of data section in VMA (Virtual Memory Address, i.e. RAM) */
        *(.data)
        *(.data.*)
        . = ALIGN(4);
        _edata = .; /* end of data section */
    } > RAM AT> FLASH /* Flash is LMA, RAM is VMA */

    .bss :
    {
        _sbss = .;
        . = ALIGN(4);
        *(.bss)
        *(.bss.*)
        . = ALIGN(4);
        _ebss = .;
    } > RAM

    .ram_usage_check :
    {
        . = ALIGN(8);
        . = . + _min_stack_size;
        . = . + _min_heap_size;
        . = ALIGN(8);
    } > RAM

}

INCLUDE device_STM32F303.x
PROVIDE(FPU_Handler = Default_Handler);
