#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(pypro_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use pypro_os::*;

const AUTHOR: &str = "kazuha046 - (discord)";
const OS_NAME: &str = "PyPro OS";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() {
    pypro_os::init();

    println!("Hello, World!");
    println!("This is {} 64-bit {}", AUTHOR, OS_NAME);

    print!("\nThis operating system (OS) ");
    print!("written in Rust");

    println!("\n\nWelcome!\n");

    #[cfg(test)]
    test_main();

    pypro_os::hlt_loop();
}
