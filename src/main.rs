
use std::vec::Vec;

pub mod lexer;

fn read_file(path: &str) -> String {
	std::fs::read_to_string(path).expect("Failed to read file")
}

fn lex_file(path: &str) -> Vec<lexer::Token> {
	lex_string(read_file(path).chars().rev().collect())
}

fn lex_string(mut text: String) -> Vec<lexer::Token> {
	let mut tokens: Vec<lexer::Token> = Vec::new();

	let mut token: lexer::Token = lexer::Token::None;
	while token != lexer::Token::EOF {
		(text, token) = lexer::lex(text);
		if let lexer::Token::Macro(name) = token.clone() {
			match name.as_str() {
				"load" => {
					'macro_loop:
					while token != lexer::Token::EOF {
						(text, token) = lexer::lex(text);
						match &token {
							lexer::Token::String(name) => {
								let loaded_tokens = &mut lex_file(&name);
								loaded_tokens.pop(); // Remove EOF
								tokens.append(loaded_tokens);
							},
							lexer::Token::SemiColon => break 'macro_loop,
							_ => panic!("You can't use the '{:?}' token with the 'load' macro", token.clone()),
						} 
					} 
				},
				"code" => {
					'macro_loop:
					while token != lexer::Token::EOF {
						(text, token) = lexer::lex(text);
						match &token {
							lexer::Token::String(code) => {
								let mut loaded_tokens = lex_string(code.chars().rev().collect());
								loaded_tokens.pop(); // Removed EOF
								tokens.append(&mut loaded_tokens);
							},
							lexer::Token::SemiColon => break 'macro_loop,
							_ => panic!("You can't use the '{:?}' token with the 'load' macro", token.clone()),
						}
					}
				}
				// Macros that can't be figured out at this stage
				_ => tokens.push(token.clone())
			}
		} else {
			tokens.push(token.clone())
		}
	}

	tokens
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() < 2 {
		eprintln!("Not enough arguments. it should be 'mortr [file]'");
		std::process::exit(1);
	}
	let path: &String = &args[1];
	
	let tokens = lex_file(path);

	for token in tokens {
		println!("{:?}", token);
	}
}


