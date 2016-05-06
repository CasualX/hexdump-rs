/*! Hex dump utility.

Format the contents of a byte slice as a classic hex dump.

```
use hexdump::hexdump;

const BYTES: &'static [u8] = b"\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF";

assert_eq!(format!("{}", hexdump(BYTES, 32)),
	"00000020:  00 11 22 33 44 55 66 77  88 99 AA BB CC DD EE FF  |..\"3DUfw........|\n");
```

The formatted dump tries to align itself as 16 byte boundaries, relative to the input bytes.

```
use hexdump::hexdump;

const BYTES: &'static [u8] = b"\x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF\x68\x65\x78\x64\x75\x6D\x70\x00";

assert_eq!(format!("{}", hexdump(&BYTES[12..24], 12)),
	"0000000C:                                       CC DD EE FF  |            ....|\n\
	 00000010:  68 65 78 64 75 6D 70 00                           |hexdump.        |\n");
```

Can also be used to dump arbitrary data structures.

```
use hexdump::datadump;

assert_eq!(format!("{}", datadump(&42)),
	"00000000:  2A 00 00 00                                       |*...            |\n");
```
*/

#[derive(Debug, Clone)]
pub struct HexDump<'a> {
	bytes: &'a [u8],
	offset: usize,
}

const SPACES: &'static str = "|                                                    |";

use ::std::fmt;
impl<'a> fmt::Display for HexDump<'a> {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Print the hex dump
		let mut addr = self.offset;
		while addr < self.bytes.len() + self.offset {
			// Print offset header
			try!(write!(f, "{:08X}: ", addr));

			// Get alignment information
			let start = addr;
			let skip = start % 16; // Offset from the left-hand side for this row
			let end = ::std::cmp::min(start + (16 - skip), self.bytes.len() + self.offset);
			let skep = 15 - ((end - 1) % 16); // Offset from the right-hand side for this row
			//println!("offset:{} start:{} skip:{} end:{} skep:{}", self.offset, start, skip, end, skep);

			//----------------------------------------------------------------
			// HEX BYTES

			try!(write!(f, "{}", &SPACES[1..2 + skip * 3 + if skip > 8 {1} else {0}]));
			for (i, byte) in self.bytes[start - self.offset..end - self.offset].iter().enumerate() {
				// Double space every 8 bytes
				if skip + i == 8 {
					try!(write!(f, " "));
				}
				try!(write!(f, "{:02X} ", byte));
			}
			try!(write!(f, "{}", &SPACES[1..2 + skep * 3 + if skep >= 8 {1} else {0}]));

			//----------------------------------------------------------------
			// ASCII BYTES

			try!(write!(f, "{}", &SPACES[0..1 + skip]));
			for &byte in &self.bytes[start - self.offset..end - self.offset] {
				let c = if byte < 0x20 || byte >= 0x80 { '.' }
				else { unsafe { ::std::char::from_u32_unchecked(byte as u32) } };
				try!(write!(f, "{}", c));
			}
			try!(write!(f, "{}", &SPACES[SPACES.len() - (1 + skep)..]));

			//----------------------------------------------------------------

			// Newline and advance
			try!(write!(f, "\n"));
			addr = end;
		}
		Ok(())
	}
}

#[inline]
pub fn hexdump(bytes: &[u8], offset: usize) -> HexDump {
	HexDump {
		bytes: bytes,
		offset: offset,
	}
}
#[inline]
pub fn datadump<T>(data: &T) -> HexDump {
	HexDump {
		bytes: unsafe {
			::std::slice::from_raw_parts(
				data as *const T as *const u8,
				::std::mem::size_of_val(data))
		},
		offset: 0,
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	const BYTES: &'static [u8] = b"\x48\x83\xEC\x28\xE8\x1B\x03\x00\x00\x48\x83\xC4\x28\xE9\x66\xFE\
	                               \x45\x72\x72\x6F\x72\x20\x63\x6F\x64\x65\x20\x00\x00\x00\x00\x00\
								   \x00\x11\x22\x33\x44\x55\x66\x77\x88\x99\xAA\xBB\xCC\xDD\xEE\xFF";

	fn header() {
		print!("_OFFSET_:  +0 +1 +2 +3 +4 +5 +6 +7  +8 +9 +A +B +C +D +E +F  |___ASCII_DUMP___|\n\
		        --------:----------------------------------------------------+----------------+\n")
	}

	#[test]
	fn units() {
		header();
		println!("{}", hexdump(&BYTES[32..48], 32));
		header();
		println!("{}", hexdump(&BYTES[5..28], 5));
		header();
		println!("{}", hexdump(&BYTES[12..20], 12));

		assert_eq!(format!("{}", hexdump(&BYTES[32..48], 32)),
			"00000020:  00 11 22 33 44 55 66 77  88 99 AA BB CC DD EE FF  |..\"3DUfw........|\n");
		assert_eq!(format!("{}", hexdump(&BYTES[5..28], 5)),
			"00000005:                 1B 03 00  00 48 83 C4 28 E9 66 FE  |     ....H..(.f.|\n\
			 00000010:  45 72 72 6F 72 20 63 6F  64 65 20 00              |Error code .    |\n");
		assert_eq!(format!("{}", hexdump(&BYTES[12..20], 12)),
			"0000000C:                                       28 E9 66 FE  |            (.f.|\n\
			 00000010:  45 72 72 6F                                       |Erro            |\n");
		assert_eq!(format!("{}", hexdump(&BYTES[8..24], 8)),
			"00000008:                           00 48 83 C4 28 E9 66 FE  |        .H..(.f.|\n\
			 00000010:  45 72 72 6F 72 20 63 6F                           |Error co        |\n");
	}
}
