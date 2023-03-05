
use std::{
	fs,
	vec::Vec
};

pub mod lexer;

fn read_file(path: &str) -> String {
	fs::read_to_string(path).expect("Failed to read file")
}

fn main() {
    let mut text: String = read_file("test.mortr").chars().rev().collect();
	let mut tokens: Vec<lexer::Token> = Vec::new();

	let mut token: lexer::Token = lexer::Token::None;
	while token != lexer::Token::EOF {
		(text, token) = lexer::lex(text);
		tokens.push(token.clone());
	}

	println!("Tokens: {:?}", tokens);
}


