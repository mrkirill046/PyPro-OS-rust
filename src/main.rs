#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pypro_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use memory::BootInfoFrameAllocator;
use core::panic::PanicInfo;
use pypro_os::*;
use x86_64::structures::paging::Page;
use x86_64::VirtAddr;

entry_point!(kernel_main);

const AUTHOR: &str = "kazuha046 - (discord)";
const OS_NAME: &str = "PyPro OS";

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    pypro_os::init();

    println!("Hello, World!");
    println!("This is {} 64-bit {}", AUTHOR, OS_NAME);

    print!("\nThis operating system (OS) ");
    print!("written on Rust");

    println!("\n\nWelcome!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    print!("\nType something: ");

    #[cfg(test)]
    test_main();

    pypro_os::hlt_loop();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_err!("{}", info);
    loop {}
}
