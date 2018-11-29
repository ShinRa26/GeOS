#![no_std]
#![no_main]

mod vga_buffer;
use core::panic::PanicInfo;

/// This function is called when a panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

static HELLO: &[u8] = b"Hello world!";
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world{}", "!");

    loop{}
}
