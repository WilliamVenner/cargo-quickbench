use std::{path::PathBuf, process::Command};

const RUST_TOOLCHAIN: &str = r#"[toolchain]
channel = "nightly"
"#;

const LIB_RS: &str = r#"#![feature(test)]
extern crate test;
use test::Bencher;

#[bench]
fn bench_hello_world(b: &mut Bencher) {
    b.iter(|| {
        println!("Hello, world!");
    });
}
"#;

fn _main() -> Result<i32, Box<dyn std::error::Error>> {
	let mut args = std::env::args().skip(2);
	let (status, dir) = match args.next().as_deref() {
		Some("init") => (
			Command::new("cargo")
				.arg("init")
				.arg("--lib")
				.args(&args.collect::<Vec<_>>())
				.spawn()?
				.wait()?,
			PathBuf::from("."),
		),

		Some("new") => {
			let name = match args.next() {
				Some(name) => name,
				None => {
					return Ok(Command::new("cargo")
						.arg("new")
						.spawn()?
						.wait()?
						.code()
						.unwrap_or(1))
				}
			};

			(
				Command::new("cargo")
					.arg("new")
					.arg(&name)
					.arg("--lib")
					.args(&args.collect::<Vec<_>>())
					.spawn()?
					.wait()?,
				PathBuf::from(name),
			)
		}

		_ => {
			eprintln!("expected `init` or `new` as first argument");
			std::process::exit(1);
		}
	};

	if !status.success() {
		return Ok(status.code().unwrap_or(1));
	}

	// TODO: get rid of this when `cargo bench` is stabilized
	std::fs::write(dir.join("rust-toolchain.toml"), RUST_TOOLCHAIN)?;
	std::fs::write(dir.join("src/lib.rs"), LIB_RS)?;

	Ok(0)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
	match _main() {
		Ok(0) => Ok(()),
		Ok(code) => std::process::exit(code),
		Err(err) => Err(err),
	}
}
