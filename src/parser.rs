use crate::lexer;
use std::vec::Vec;

#[derive(Debug, PartialEq)]
pub enum ASTStatement {
	None
}

#[derive(Debug, PartialEq)]
pub enum ASTFile {
	Function(String, Vec<(String, String)>, ASTStatement),
}

pub fn read_file(path: &str) -> String {
	std::fs::read_to_string(path).expect("Failed to read file").chars().rev().collect()
}

fn func(mut s: String) -> Option<(String, ASTFile)> {
	#[allow(unused_assignments)]
	let mut tok = lexer::Token::None;
	
	(s, tok) = lexer::lex(s);
	let lexer::Token::Symbol(name) = tok else {
		if tok != lexer::Token::EOF {
			println!("No arg name");
		}
		return None;
	};

	(s, tok) = lexer::lex(s);
	if tok != lexer::Token::Colon {
		if tok != lexer::Token::EOF {
			println!("No colon after name");
		}
		return None;
	}
	
	(s, tok) = lexer::lex(s);
	if tok != lexer::Token::Colon {
		if tok != lexer::Token::EOF {
			println!("No colon after colon");
		}
		return None;
	}

	(s, tok) = lexer::lex(s);
	if tok != lexer::Token::OpenParentheses {
		if tok != lexer::Token::EOF {
			println!("No no ( after colon");
		}
		return None;
	}

	let mut params: Vec<(String, String)> = Vec::new();
	loop {
		// (param: Type, ...)
		
		(s, tok) = lexer::lex(s);
		let lexer::Token::Symbol(param) = tok else {
			if tok != lexer::Token::EOF {
				println!("No arg name");
			}
			return None;
		};

		(s, tok) = lexer::lex(s);
		if tok != lexer::Token::Colon {
			if tok != lexer::Token::EOF {
				println!("No colon after arg");
			}
			return None;
		}

		(s, tok) = lexer::lex(s);
		let lexer::Token::Symbol(typ) = tok else {
			if tok != lexer::Token::EOF {
				println!("No type after colon");
			}
			return None;
		};

		(s, tok) = lexer::lex(s);
		if tok != lexer::Token::Comma {
			if tok == lexer::Token::CloseParentheses {
				params.push((param, typ));
				break;
			}
			if tok != lexer::Token::EOF {
				println!("No comma after param... expected a ')'");
			}
			return None;
		}

		params.push((param, typ));
	}

	println!("{:?}", &params);
	Some((s, ASTFile::Function(name, params, ASTStatement::None)))
}

pub fn parse(path: &str) -> Vec<ASTFile> {
	let mut text = read_file(path);
	let mut ast: Vec<ASTFile> = Vec::new();

	loop {
		if let Some((s, ast_func)) = func(text.clone()) {
			ast.push(ast_func);
			text = s;
		} else {
			break;
		}
	}
	ast
}
