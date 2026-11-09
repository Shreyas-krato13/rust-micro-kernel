#![no_std]
#![no_main]

pub mod vga_buffer;
pub mod logger;

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

// KERNEL ENTRY POINT

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {

    // STARTUP BANNER

    kprintln!("================================");
    kprintln!("       Micro-OS | CA-6 Rust");
    kprintln!("================================");

    kprintln!("");
    kprintln!("Layer 1: Boot confirmed");
    kprintln!("Layer 2: VGA Console ACTIVE");

    kprintln!("");
    kprintln!("Welcome to Micro OS!");



    // KERNEL LOGGER DEMONSTRATION

kinfo!("Kernel boot successful");

kwarn!("Warning demonstration");

kerror!("Error demonstration");

kdebug!("Debug message");

kprintln!("");
 

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
