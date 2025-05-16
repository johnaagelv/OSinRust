// in src/vga_buffer.rs

// Use Volatile wrapper to ensure the the Buffer is not optimized away
use volatile::Volatile;
// Supporting Rust formatting
use core::fmt;
// Use the lazy static feature
use lazy_static::lazy_static;
// Use the spinning mutexes
use spin::Mutex;

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
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_position = 0;
	}

	fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar {
			ascii_character: b' ',
			colour_code: self.colour_code,
		};
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
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

// Implement Write using macros
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		column_position: 0,
		colour_code: ColourCode::new(Colour::White, Colour::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

// TODO: DELETE THIS
pub fn print_test() {
	use core::fmt::Write;

	let mut writer = Writer {
		column_position: 0,
		colour_code: ColourCode::new(Colour::White, Colour::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	};

	writer.write_byte(b'H');
	writer.write_string("ello ");
	write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}