#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_os::test_runner)]

use test_os::*;
use core::panic::PanicInfo;

const AUTHOR: &str = "kazuha046 - (discord)";
const OS_NAME: &str = "PyPro OS";

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_os::test_panic_handler(info)
}

#[no_mangle]
pub extern "C" fn _start() {
    println!("Hello, World!");
    println!("This is {} 64-bit {}", AUTHOR, OS_NAME);

    print!("\nThis operating system (OS) ");
    print!("written in Rust");

    println!("\n\nWelcome!");

    loop {}
}
