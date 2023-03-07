
use std::vec::Vec;

pub mod lexer;
pub mod parser;


fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("Not enough arguments. it should be 'mortr [file]'");
		std::process::exit(1);
	}
	let path: &str = &args[1];

	println!("{:?}", parser::parse(path));
}


