#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simpleos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use simpleos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    test_main();
    simpleos::halt();
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    simpleos::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
