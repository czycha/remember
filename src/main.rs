#[macro_use]
extern crate clap;
extern crate csv;
use clap::App;
use std::env;
use std::ascii::AsciiExt;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::BufWriter;
use std::process;

fn find_in_csv(key: &str, path: &str) -> Option<(String, String)> {
	match csv::Reader::from_file(path) {
		Ok(f) => {
			let mut rdr = f.has_headers(false);
			for record in rdr.decode() {
				let (k, v): (String, String) = record.unwrap();
				if k.eq_ignore_ascii_case(key) {
					return Some((k, v));
				}
			}
			return None;
		},
		Err(_) => {
			return None;
		}
	}
}

fn confirm(msg: &str) -> bool {
	print!("{} (y/n) ", msg);
	std::io::stdout().flush().unwrap();
	let mut response = String::new();
	match std::io::stdin().read_line(&mut response) {
		Ok(_) => {}
		Err(_) => {
			return false;
		}
	}
	let trimmed = response.trim().to_lowercase();
	return trimmed == "yes" || trimmed == "y";
}

fn main() {
	let homeval = match env::var("HOME") {
		Ok(val) => val,
		Err(_) => panic!("Can't find home directory")
	};
	let path = homeval + "/.remember";
	let yaml = load_yaml!("cli.yml");
	let matches = App::from_yaml(yaml).get_matches();
	let mut exit_code = 0;
	if matches.is_present("list") {
		let submatches = matches.subcommand_matches("list").unwrap();
		match csv::Reader::from_file(&path) {
			Ok(f) => {
				let mut rdr = f.has_headers(false);
				if submatches.is_present("keys") {
					for record in rdr.decode() {
						let (k, _): (String, String) = record.unwrap();
						println!("{}", k);
					}
				} else {
					for record in rdr.decode() {
						let (k, v): (String, String) = record.unwrap();
						println!("{} → {}", k, v);
					}
				}
			}
			Err(_) => {}
		}
	} else if matches.is_present("wipe") {
		let submatches = matches.subcommand_matches("wipe").unwrap();
		if submatches.is_present("force") || confirm("This will remove all information. Are you sure?") {
			match OpenOptions::new().write(true).truncate(true).create(true).open(&path) {
				Ok(_) => {}
				Err(_) => {
					println!("Unable to write to {}", &path);
					exit_code = -1;
				}
			}
		} else {
			exit_code = -1;
		}
	} else if matches.is_present("remove") {
		let submatches = matches.subcommand_matches("remove").unwrap();
		let key = submatches.value_of("key").unwrap();
		let mut found = false;
		match csv::Reader::from_file(&path) {
			Ok(f) => {
				let mut rdr = f.has_headers(false);
				let mut wtr = csv::Writer::from_memory();
				for record in rdr.decode() {
					let (k, v): (String, String) = record.unwrap();
					if k.eq_ignore_ascii_case(key) {
						found = true;
					} else {
						match wtr.encode((k, v)) {
							Ok(_) => {},
							Err(_) => {
								println!("Unable to encode key/value");
								exit_code = -1;
							}
						}
					}
				}
				if found && exit_code != -1 {
					let file = OpenOptions::new()
						.write(true)
						.truncate(true)
						.create(true)
						.open(&path);
					match file {
						Ok(f) => {
							let mut writer = BufWriter::new(&f);
							let bytes = wtr.as_string().as_bytes();
							match writer.write(&bytes) {
								Ok(_) => {}
								Err(_) => {
									println!("Unable to write to {}", &path);
									exit_code = -1;
								}
							}
						}
						Err(_) => {
							exit_code = -1;
							println!("Unable to write to {}", &path)
						}
					}
				}
			}
			Err(_) => {
				exit_code = -1;
				println!("Unable to read from {}", &path)
			}
		}
		if !found && exit_code != -1 {
			exit_code = -1;
			println!("Couldn't find key");
		}
	} else if matches.is_present("change") {
		let submatches = matches.subcommand_matches("change").unwrap();
		let key = submatches.value_of("key").unwrap();
		let val = submatches.value_of("value").unwrap();
		let mut wtr;
		match csv::Reader::from_file(&path) {
			Ok(f) => {
				let mut rdr = f.has_headers(false);
				wtr = csv::Writer::from_memory();
				for record in rdr.decode() {
					let (k, v): (String, String) = record.unwrap();
					match if k.eq_ignore_ascii_case(key) { wtr.encode((key, val)) } else { wtr.encode((k, v)) } {
						Ok(_) => {},
						Err(_) => {
							println!("Unable to encode key/value");
							exit_code = -1;
						}
					}
				}
			}
			Err(_) => {
				wtr = csv::Writer::from_memory();
				match wtr.encode((key, val)) {
					Ok(_) => {},
					Err(_) => {
						println!("Unable to encode key/value");
						exit_code = -1;
					}
				}
			}
		}
		if exit_code != -1 {
			let file = OpenOptions::new()
				.write(true)
				.truncate(true)
				.create(true)
				.open(&path);
			match file {
				Ok(f) => {
					let mut writer = BufWriter::new(&f);
					let bytes = wtr.as_string().as_bytes();
					match writer.write(&bytes) {
						Ok(_) => {},
						Err(_) => {
							println!("Unable to write to {}", &path);
							exit_code = -1;
						}
					}
				}
				Err(_) => {
					exit_code = -1;
					println!("Unable to write to {}", &path)
				}
			}
		}
	} else if matches.is_present("add") {
		let submatches = matches.subcommand_matches("add").unwrap();
		let key = submatches.value_of("key").unwrap();
		let val = submatches.value_of("value").unwrap();
		match find_in_csv(&key, &path) {
			Some((_, v)) => {
				if v == val {
					println!("Key/value pair already exists");
				} else {
					println!("Key/value pair with different value exists. Use the 'change' command to change value.");
				}
				exit_code = -1;
			},
			None => {
				// Generate line, add it to beginning (or end) of file
				let mut wtr = csv::Writer::from_memory();
				match wtr.encode((key, val)) {
					Ok(_) => {
						let file = OpenOptions::new()
							.write(true)
							.append(true)
							.create(true)
							.open(&path);
						match file {
							Ok(f) => {
								let mut writer = BufWriter::new(&f);
								let bytes = wtr.as_string().as_bytes();
								match writer.write(&bytes) {
									Ok(_) => {},
									Err(_) => {
										println!("Unable to write to {}", &path);
										exit_code = -1;
									}
								}
							}
							Err(_) => {
								println!("Unable to write to {}", &path);
							}
						}
					},
					Err(_) => {
						println!("Unable to encode key/value");
						exit_code = -1;
					}
				}
			}
		}
	} else if matches.is_present("find") {
		let submatches = matches.subcommand_matches("find").unwrap();
		let key = submatches.value_of("key").unwrap();
		match find_in_csv(&key, &path) {
			Some((_, v)) => {
				println!("{}", v);
			},
			None => {
				exit_code = -1;
			}
		}
	}
	if exit_code != 0 {
		process::exit(exit_code);
	}
}