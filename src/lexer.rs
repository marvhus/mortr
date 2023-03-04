use std::vec::Vec;

#[derive(Debug)]
pub enum Token {
	Symbol(String),
	Number(String),
	String(String),
	Macro(String),
	Colon,
	SemiColon,
	Equals,
	OpenCurlyBracket,
	CloseCurlyBracket,
	OpenCurvyBracket,
	CloseCurvyBracket,
}

#[derive(PartialEq)]
pub enum ReadingType {
	Alphanumeric,
	Numeric,
	Macro,
	String,
	#[allow(dead_code)]
	Symbol,
	None,
}

fn lex_symbol(text: String) -> Token {
	match text.as_str() {
		":" => Token::Colon,
		";" => Token::SemiColon,
		"=" => Token::Equals,
		"{" => Token::OpenCurlyBracket,
		"}" => Token::CloseCurlyBracket,
		"(" => Token::OpenCurvyBracket,
		")" => Token::CloseCurvyBracket,
		_ => panic!("during: Unknown symbol '{}'", text)
	}
}

fn is_symbol(c: char) -> bool {
	match c {
		':'|';'|'='|'{'|'}'|'('|')' => true,
		_ => false,
	}
}

pub fn lex_file(text: String) -> Vec<Token> {
	let mut tokens: Vec<Token> = Vec::new();
	let mut current_type: ReadingType = ReadingType::None;

	let mut end: bool = false;
	let mut inserted_symbol: bool = false;
	let mut curr: String = "".into();
	for c in text.chars() {
		match current_type {
			ReadingType::Alphanumeric => {
				end = match c {
					'a'..='z'|'A'..='Z'|'_'|'0'..='9' => false,
					_ => true,
				};
				if !end {
					curr.push(c);
				} else if is_symbol(c) {
					tokens.push(lex_symbol(String::from(c)));
					inserted_symbol = true;
				} 
			},
			ReadingType::Numeric => {
				end = match c {
					'0'..='9' => false,
					_ => true,
				};
				if !end {
					curr.push(c);
				} else if is_symbol(c) {
					tokens.push(lex_symbol(String::from(c)));
					inserted_symbol = true;
				} 
			},
			ReadingType::String => {
				end = match c {
					'"' => true,
					_ => false,
				};
				if !end {
					curr.push(c);
				} 
			},
			ReadingType::Macro => {
				end = match c {
					'a'..='z' => false,
					_ => true,
				};
				if !end {
					curr.push(c);
				} else if is_symbol(c) {
					tokens.push(lex_symbol(String::from(c)));
					inserted_symbol = true;
				} 
			}
			ReadingType::Symbol => {},
			ReadingType::None => {
				current_type = match c {
					'a'..='z'|'A'..='Z'|'_' => ReadingType::Alphanumeric,
					'0'..='9' => ReadingType::Numeric,
					':'|';'|'='|'{'|'}'|'('|')' => {
						tokens.push(lex_symbol(String::from(c)));
						ReadingType::None
					},
					' ' => ReadingType::None,
					'"' => {
						end = false;
						ReadingType::String
					},
					'#' => ReadingType::Macro,
					_ => panic!("Invalid! {}", c)
				};
				if current_type != ReadingType::None
				&& current_type != ReadingType::String {
					curr.push(c);
					end = false;
				}
			},
		};
		if end {
			let mut symbol: Option<Token> = None;
			if inserted_symbol {
				symbol = tokens.pop(); // pop returns Option<>
				inserted_symbol = false;
			}
			match current_type {
				ReadingType::String =>       tokens.push(Token::String(curr)),
				ReadingType::Alphanumeric => tokens.push(Token::Symbol(curr)),
				ReadingType::Numeric =>      tokens.push(Token::Number(curr)),
				ReadingType::Macro =>        tokens.push(Token::Macro(curr)),
				ReadingType::Symbol => {},
				ReadingType::None => {},
			};
			if let Some(symbol) = symbol {
				tokens.push(symbol);
			}
			curr = "".into();
			current_type = ReadingType::None;
		}
	}
	if curr != "" {
		match current_type {
			ReadingType::String =>       tokens.push(Token::String(curr)),
			ReadingType::Alphanumeric => tokens.push(Token::Symbol(curr)),
			ReadingType::Numeric =>      tokens.push(Token::Number(curr)),
			ReadingType::Macro =>        tokens.push(Token::Macro(curr)),
			ReadingType::Symbol =>       tokens.push(lex_symbol(curr)),
			ReadingType::None => {},
		};
	}
	
	tokens
}
