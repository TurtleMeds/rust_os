// Disable standard library
#![no_std]
// Overwrite entry point
#![no_main]

mod vga_buffer;

use core::panic::PanicInfo;


// disable name mangling so the "_start" function is actually named "_start" and not
// changed into random letters and numbers
#[no_mangle]
// ENTRY POINT
// 'extern "C"' makes sure to use the C language calling convention
// this function is named "_start" because that is the default entry point name for
// most systems
pub extern "C" fn _start() -> ! { // the return type "!" means the function is never
                                  // allowed to return
    vga_buffer::print_smth();

    loop{}
}

// PANIC HANDLER
#[panic_handler]
// this function is called on panic
fn panic(_info: &PanicInfo) -> ! {
    loop{}
}
