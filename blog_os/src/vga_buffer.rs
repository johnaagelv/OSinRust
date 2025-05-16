// in src/vga_buffer.rs

// Use Volatile wrapper to ensure the the Buffer is not optimized away
use volatile::Volatile;

// Colours
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour {
	Black = 0,
	Blue = 1,
	Green = 2,
	Cyan = 3,
	Red = 4,
	Magenta = 5,
	Brown = 6,
	LightGray = 7,
	DarkGray = 8,
	LightBlue = 9,
	LightGreen = 10,
	LightCyan = 11,
	LightRed = 12,
	Pink = 13,
	Yellow = 14,
	White = 15,
}

// Colour codes as background and foreground
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColourCode(u8);

impl ColourCode {
	fn new(foreground: Colour, background: Colour) -> ColourCode {
		ColourCode((background as u8) << 4 | (foreground as u8))
	}
}

// Text buffer definitions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
	ascii_character: u8,
	colour_code: ColourCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

//#[repr(transparent)]
struct Buffer {
	chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

// Writer type definition
pub struct Writer {
	column_position: usize,
	colour_code: ColourCode,
	buffer: &'static mut Buffer,
}

// Writer implementation for single char
impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_position >= BUFFER_WIDTH {
					self.new_line();
				}
				let row = BUFFER_HEIGHT - 1;
				let col = self.column_position;
				let colour_code = self.colour_code;
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_character: byte,
					colour_code,
				});
				self.column_position += 1;
			}
		}
	}

	fn new_line(&mut self) {
		/* TODO */
	}
}

// Writer implementation for string
impl Writer {
	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				// printable ASCII byte or newline
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				// non-printable ASCII byte
				_ => self.write_byte(0xfe),
			}
		}
	}
}

pub fn print_test() {
	let mut writer = Writer {
		column_position: 0,
		colour_code: ColourCode::new(Colour::White, Colour::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};

	writer.write_byte(b'H');
	writer.write_string("ello ");
	writer.write_string("World! - This is the EW OS");
}