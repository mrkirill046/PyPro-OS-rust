#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pypro_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;
use memory::BootInfoFrameAllocator;
use pypro_os::task::executor::Executor;
use pypro_os::task::keyboard;
use pypro_os::*;
use task::Task;
use x86_64::VirtAddr;

extern crate alloc;

entry_point!(kernel_main);

const AUTHOR: &str = "kazuha046 - (discord)";
const OS_NAME: &str = "PyPro OS";

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    pypro_os::init();

    println!("Hello, World!");
    println!("This is {} 64-bit {}", AUTHOR, OS_NAME);
    print_info!("\nThis operating system (OS) written on Rust");
    println!("\nWelcome!");

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = unsafe { BootInfoFrameAllocator::init(&boot_info.memory_map) };

    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Heap initialization failed");

    #[cfg(test)]
    test_main();

    print!("\nType something: ");

    let mut executor = Executor::new();
    executor.spawn(Task::new(keyboard::print_key_presses()));
    executor.run();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    print_err!("{}", info);
    loop {}
}
