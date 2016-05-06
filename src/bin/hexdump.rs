extern crate hexdump;

use ::std::{env, fs};
use ::std::path::{PathBuf, Path};
use ::std::io::{self, stderr, Seek, SeekFrom, Read, Write};
use ::std::process::{exit};
use hexdump::hexdump;

//----------------------------------------------------------------
// All error handling.

fn err_unexpected_end(arg: &str) -> ! {
	let _ = writeln!(stderr(), "hexdump: {}: unexpected end.", arg);
	exit(1);
}
fn err_nan(arg: &str) -> ! {
	let _ = writeln!(stderr(), "hexdump: {}: not a number.", arg);
	exit(1);
}
fn err_flag(arg: &str) -> ! {
	let _ = writeln!(stderr(), "hexdump: {}: unknown flag.", arg);
	exit(1);
}
fn err_file_error(err: io::Error, path: &Path) -> ! {
	let _ = writeln!(stderr(), "hexdump: file error {:?}: {}.", path, err);
	exit(1);
}

//----------------------------------------------------------------
// Parse the command line arguments.


struct Parameters {
	length: Option<usize>,
	skip: Option<usize>,
	paths: Vec<PathBuf>,
}
impl Default for Parameters {
	fn default() -> Parameters {
		let mut params = Parameters {
			length: None,
			skip: None,
			paths: Vec::new(),
		};

		let mut args = env::args_os();
		args.next();

		while let Some(arg) = args.next() {
			if let Some(arg) = arg.to_str() {
				if arg.starts_with("-") {
					match arg.as_ref() {
						"-n" => {
							params.length = Some(args
								.next().unwrap_or_else(|| err_unexpected_end(arg))
								.into_string().unwrap_or_else(|_| err_nan(arg))
								.parse().unwrap_or_else(|_| err_nan(arg)));
						},
						"-s" => {
							params.skip = Some(args
								.next().unwrap_or_else(|| err_unexpected_end(arg))
								.into_string().unwrap_or_else(|_| err_nan(arg))
								.parse().unwrap_or_else(|_| err_nan(arg)));
						},
						"--" => break,
						_ => err_flag(arg),
					}
					continue;
				}
			}
			params.paths.push(arg.into());
			break;
		}

		params.paths.extend(args.map(|os_str| os_str.into()));
		params
	}
}

//----------------------------------------------------------------
// Read from file and dump hex.

fn dump(params: &Parameters, path: &Path) {
	println!("Hex dump for {:?}:", path);

	let mut file = fs::File::open(path)
		.unwrap_or_else(|e| err_file_error(e, path));

	if let Some(skip) = params.skip {
		file.seek(SeekFrom::Current(skip as i64))
		.unwrap_or_else(|e| err_file_error(e, path));
	}

	let mut data: Vec<u8> = Vec::new();
	if let Some(length) = params.length {
		data.resize(length, 0);
		file.read_exact(&mut data)
			.unwrap_or_else(|e| err_file_error(e, path));
	}
	else {
		file.read_to_end(&mut data)
			.unwrap_or_else(|e| err_file_error(e, path));
	}

	println!("--------:----------------------------------------------------+----------------+");
	print!("{}", hexdump(&data, params.skip.unwrap_or(0)));
	println!("--------:----------------------------------------------------+----------------+");
}

//----------------------------------------------------------------

fn main() {
	let params = Parameters::default();
	for path in &params.paths {
		dump(&params, &path);
	}
}
