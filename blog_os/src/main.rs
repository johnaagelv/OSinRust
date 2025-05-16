// in src/main.rs

// Do not include the standard library
#![no_std]
// Do not use the normal entry point chain
#![no_main]

// Include the PanicInfo for our own panic handler
use core::panic::PanicInfo;

// Include the module vga_buffer
mod vga_buffer;

// Define the entry point as _start using C calling convention
#[unsafe(no_mangle)]
unsafe extern "C" fn _start() -> ! {
	vga_buffer::print_test();
	// Loop forever
	loop {}
}

// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
	// PanicInfo parameter contains the file and line where the panic happened and the optional panic message
	// The function should never return, so it is marked as a diverging function by returning the “never” type !

	// Loop forever as we had a panic
	loop {}
}