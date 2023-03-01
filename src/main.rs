#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_v1::test_runner)]
#![reexport_test_harness_main = "test_main"]

use rust_os_v1::println;
use core::panic::PanicInfo;
use bootloader::{entry_point, BootInfo};
use x86_64::paging::PageTable;

entry_point!(kernel_main);

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    rust_os_v1::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_v1::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use rust_os_v1::memory::active_level_4_table;
    use x86_64::VirtAddr;

    println!("Hello World{}", "!");
    rust_os_v1::init();
    
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let 14_table = unsafe { active_level_4_table(phys_mem_offset) };

    for (i, entry) in 14_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("L4 Entry {}: {:?}", i, entry);
        }
    }

    #[cfg(test)]
    test_main();

    println!("It did not crash!");
    rust_os_v1::hlt_loop();
}

if !entry.is_unused() {
    println!("L4 Entry {}: {:?}", i, entry);

    let phys = entry.frame().unwrap().start_address();
    let virt = phys.as_u64() + boot_info.physical_memory_offset;
    let ptr = VirtAddr::new(virt).as_mut_ptr();
    let l3_table: &PageTable = unsafe { &*ptr };

    for (i, entry) in 13_table.iter().enumerate() {
        if !entry.is_unused() {
            println!("  L3 Entry {}: {:?}", i, entry);
        }
    }
}
