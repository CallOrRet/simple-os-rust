#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(simpleos::test_runner)]
#![reexport_test_harness_main = "test_main"]

use simpleos::gdt;
use simpleos::interrupts;
use simpleos::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    gdt::init();
    interrupts::init();

    #[cfg(test)]
    test_main();

    println!("Hello World!");

    simpleos::halt();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("{}", info);
    simpleos::halt();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    simpleos::test_panic_handler(info);
}
