// Disable standard library
#![no_std]
// Overwrite entry point
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;


static HELLO: &[u8] = b"Hello, World";

// disable name mangling so the "_start" function is actually named "_start" and not
// changed into random letters and numbers
#[no_mangle]
// ENTRY POINT
// 'extern "C"' makes sure to use the C language calling convention
// this function is named "_start" because that is the default entry point name for
// most systems
pub extern "C" fn _start() -> ! { // the return type "!" means the function is never
                                  // allowed to return
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop{}
}

// PANIC HANDLER
#[panic_handler]
// this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
