#![no_std]
#![no_main]

use core::panic::PanicInfo;

/// This function is called when a panic occurs
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}

// Linux
// #[no_mangle]
// pub extern "C" fn _start() -> ! {
//     loop{}
// }

// Linux Compile
// cargo rustc -- -Z pre-link-arg=-nostartfiles

/// Windows
#[no_mangle]
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    loop{}
}

// Windows Compile
// cargo build