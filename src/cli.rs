use std::io::{stdin, stdout, Write};

pub fn get_input(prompt: &str) -> String {
	let mut buf = String::new();
	print!("{prompt}");
	stdout().flush().unwrap();
	stdin().read_line(&mut buf).unwrap();
	buf.trim().to_lowercase()
}