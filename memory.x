MEMORY
{
    FLASH_LMA : ORIGIN = 0, LENGTH = 4M
    FLASH : ORIGIN = 0x40000000, LENGTH = 4M
    RAM : ORIGIN = 0x4FF70000, LENGTH = 256K
}

usb_serial_device_tx_one_char = 0x4fc008bc;
