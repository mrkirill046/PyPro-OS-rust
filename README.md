
# Setup
- ```rustup target add x86_64-unknown-none```
- ```rustup update```
- ```rustup override set nightly```
- ```rustup component add llvm-tools-preview```
- ```cargo install bootimage```

# Build
- ```cargo build```
- ```cargo bootimage```

# Run 
## on Cargo
- ```cargo run``` 
## on QEMU
- ```qemu-system-x86_64 -drive format=raw,file=target/x86_64-os/debug/bootimage-test-os.bin```
## on Real Machine
### Write the operating system image to USB with using Linux 
- ```sudo dd if=target/x86_64-os/debug/bootimage-test-os.bin of=/dev/sdX status=progress && sync``` // sdX - path to your usb
- run ```lsblk``` and find your usb device, then copy name, then paste it into /dev/your_usb
- This command will write the operating system image to USB. After writing the image to a USB drive, you can run it on real hardware by booting from it. To boot from a USB drive, you will probably need to use a special boot menu or change the boot order in the BIOS configuration. Please note that this does not currently work on UEFI machines, as the bootloader module does not yet have UEFI support.
