use core::panic::PanicInfo;

#![no_std]

// Panic handler
#[panic_handler]
fn panic(_info: &PanicInfo) -> {
    loop{}
}

fn main() {}
