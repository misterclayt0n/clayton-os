#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_buffer;

/// this function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

/// actual start of the program
#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}
