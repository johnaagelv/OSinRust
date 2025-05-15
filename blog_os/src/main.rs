// main.rs

// Do not include the standard library
#![no_std]
// Do not use the normal entry point chain
#![no_main]

// Include the PanicInfo for our own panic handler
use core::panic::PanicInfo;

// Define the entry point as _start using C calling convention
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
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