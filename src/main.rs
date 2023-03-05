
use std::{
	fs,
	vec::Vec
};

pub mod lexer;

fn read_file(path: &str) -> String {
	fs::read_to_string(path).expect("Failed to read file")
}

fn lex_file(path: &str) -> Vec<lexer::Token> {
	let mut text: String = read_file(path).chars().rev().collect();
	
	let mut tokens: Vec<lexer::Token> = Vec::new();

	let mut token: lexer::Token = lexer::Token::None;
	while token != lexer::Token::EOF {
		(text, token) = lexer::lex(text);
		if let lexer::Token::Macro(name) = token.clone() {
			match name.as_str() {
				"load" => {
					'macro_loop:
					while token != lexer::Token::EOF && token != lexer::Token::SemiColon {
						(text, token) = lexer::lex(text);
						match &token {
							lexer::Token::String(name) => {
								let loaded_tokens = &mut lex_file(&name);
								loaded_tokens.pop(); // Remove EOF
								tokens.append(loaded_tokens);
							},
							lexer::Token::SemiColon => break 'macro_loop,
							_ => panic!("The '{:?}' Token is not supported with the 'load' macro", token),
						} 
					} 
				}
				_ => unimplemented!("The '{}' does not exist", name)
			}
		} else {
			tokens.push(token.clone())
		}
	}

	tokens
}

fn main() {
	let tokens = lex_file("test.mortr");

	for token in tokens {
		println!("{:?}", token);
	}
}


