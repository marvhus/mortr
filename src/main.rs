
use std::fs;

pub mod lexer;
use lexer::lex_file;

fn read_file(path: &str) -> String {
	fs::read_to_string(path).expect("Failed to read file")
		.chars()
		.filter(|&x| x != '\n')
		.collect()
}

fn main() {
    let inp: String = read_file("test.mortr");
	println!("{:?}", lex_file(inp));
}


