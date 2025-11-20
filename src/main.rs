#![no_std]
#![no_main]

extern crate panic_halt;
extern crate riscv;

core::arch::global_asm!(
    "
.section .text._start
.global _start
_start:
    // Initialize stack
    la sp, __stack

    // Clear .bss section
    la a0, __bss_start
    la a1, __bss_end
    beq a0, a1, 2f
1:
    sw zero, 0(a0)
    addi a0, a0, 4
    blt a0, a1, 1b
2:

    // Copy .data section from flash to ram
    la a0, __sdata
    la a1, __edata
    la a2, __sidata
    beq a0, a1, 4f
3:
    lw x9, 0(a2)
    sw x9, 0(a0)
    addi a0, a0, 4
    addi a2, a2, 4
    blt a0, a1, 3b
4:
    // Jump to Rust
    call {MAIN}
        ",
    MAIN = sym main
);

unsafe extern "C" {
    fn usb_serial_device_tx_one_char(b: u8);
}

unsafe extern "C" fn main() -> ! {
    rtt_target::rtt_init_print!(rtt_target::ChannelMode::BlockIfFull, 256);
    rtt_target::rprintln!("Starting up...");
    for c in b"Waiting for RTT connection...\r\n" {
        unsafe { usb_serial_device_tx_one_char(*c) };
    }
    loop {
        rtt_target::rprintln!("Hello, world!");
        for c in b"Hello, world!\r\n" {
            unsafe { usb_serial_device_tx_one_char(*c) };
        }
    }
}
