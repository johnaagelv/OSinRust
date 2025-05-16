// main.rs

// Do not include the standard library
#![no_std]
// Do not use the normal entry point chain
#![no_main]

// Include the PanicInfo for our own panic handler
use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello world";

// Define the entry point as _start using C calling convention
unsafe extern "C" fn _start() -> ! {
	let vga_bugger = 0xb8000 as *mut u8;

	for (i, &byte) in HELLO.iter().enumerate() {
		unsafe {
			*vga_buffer.offset(i as isize * 2) = byte;
			*vga_buffer.offset(i as isize * 2 + 1) = 0xb;
		}
	}
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