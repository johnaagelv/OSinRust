// in src/main.rs

// Do not include the standard library
#![no_std]
// Do not use the normal entry point chain
#![no_main]

// Adding test feature
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
	println!("Running {} tests", tests.len());
	for test in tests {
		test();
	}
	exit_qemu(QemuExitCode::Success);
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

// Include the PanicInfo for our own panic handler
use core::panic::PanicInfo;

// Include the module vga_buffer
mod vga_buffer;


// Define the entry point as _start using C calling convention
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
	println!("Hello World{}", "!");

	#[cfg(test)]
	test_main();

	// Loop forever
	loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	// PanicInfo parameter contains the file and line where the panic happened and the optional panic message
	// The function should never return, so it is marked as a diverging function by returning the “never” type !
	println!("{}", info);
	// Loop forever as we had a panic
	loop {}
}

#[test_case]
fn trivial_assertion() {
	print!("Trivial assertion ...");
	assert_eq!(1, 1);
	println!("[ok]");
}