#![no_std]
#![no_main]

use test_os::*;
use core::panic::PanicInfo;

const AUTHOR: &str = "kazuha046 - (discord)";
const OS_NAME: &str = "PyPro OS";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("\n{}", info);
    loop {}
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
