#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_v1::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(main);

fn main(boot_info: &'static BootInfo) -> ! {
    unimplemented!()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_v1::test_panic_handler(info)
}
