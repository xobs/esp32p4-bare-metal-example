# ESP32P4 Bare Metal Example

This project is a bare-metal example running on ESP32P4. It starts an RTT server, and sends characters out the USB serial port. RTT is opened as a blocking device, so the serial output will stop shortly after initialization until a debugger attaches.

## Usage

To use, build with:

```
rustup target add riscv32imafc-unknown-none-elf
cargo build
```

Then, flash with:

```
probe-rs run target/riscv32imafc-unknown-none-elf/debug/esp32p4-bare-metal-example
```
