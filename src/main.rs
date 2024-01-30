// Disable standard library
#![no_std]
// Overwrite entry point
#![no_main]

#![feature(custom_test_frameworks)]

#![test_runner(crate::test_runner)]

#![reexport_test_harness_main = "test_main"]

mod vga_buffer;
mod serial;

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
    println!("Hello, World{}", "!");
    println!("Second line.");
    println!("Third!");

    #[cfg(test)]
    test_main();

    loop{}
}

// PANIC HANDLER
#[cfg(not(test))]
#[panic_handler]
// and this one in non-test mode
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
// and this one in test mode
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop{}
}
// TESTING

pub trait Testable {
    fn run(&self) -> ();
}
impl Testable for T where T: Fn() {
    fn run(&self) -> () {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    println!("Running {} tests...", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
