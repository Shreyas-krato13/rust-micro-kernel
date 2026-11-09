#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// KERNEL ENTRY POINT

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {

    // Kernel starts here

    loop {
        core::hint::spin_loop();
    }
}

// PANIC HANDLER

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {

    loop {
        core::hint::spin_loop();
    }
}
