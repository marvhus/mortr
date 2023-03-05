#[derive(PartialEq, Debug, Clone)]
pub enum Token {
	Symbol(String),
	Number(String),
	String(String),
	Macro(String),
	Comma,
	Hyphen,
	Plus,
	GreaterThan,
	LessThan,
	Colon,
	SemiColon,
	Equals,
	OpenBraces,
	CloseBraces,
	OpenParentheses,
	CloseParentheses,
	EOF,
	None,
}

pub fn lex(mut text: String) -> (String, Token) {
	fn get_next(s: &mut String) -> char {
		s.pop().expect("End of string")
	}
	
	let mut gathred: String = String::new();
	
	while text.len() > 0 {
		let mut c = get_next(&mut text);
		match c {
			' '|'\n' => continue,
			',' => return (text, Token::Comma),
			'-' => return (text, Token::Hyphen),
			'+' => return (text, Token::Plus),
			'>' => return (text, Token::GreaterThan),
			'<' => return (text, Token::LessThan),
			':' => return (text, Token::Colon),
			';' => return (text, Token::SemiColon),
			'=' => return (text, Token::Equals),
			'{' => return (text, Token::OpenBraces),
			'}' => return (text, Token::CloseBraces),
			'(' => return (text, Token::OpenParentheses),
			')' => return (text, Token::CloseParentheses),
			_ => {
				gathred.push(c);

				// Symbol a..zA..Z_
				if c.is_alphabetic() {
					while !c.is_whitespace() {
						if text.len() <= 0 {
							return (text, Token::String(gathred));
						}
						c = get_next(&mut text);
						if !c.is_alphanumeric() && c != '_' {
							text.push(c);
							return (text, Token::Symbol(gathred));
						}
						gathred.push(c);
					}
					return (text, Token::Symbol(gathred));
				}
				// Number 0..9
				if c.is_numeric() {
					while !c.is_whitespace() {
						if text.len() <= 0 {
							return (text, Token::String(gathred));
						}
						c = get_next(&mut text);
						if !c.is_numeric() {
							text.push(c);
							return (text, Token::Symbol(gathred));
						}
						gathred.push(c);
					}
					return (text, Token::Symbol(gathred));
				}

				// Macro #
				if c == '#' {
					gathred = "".into(); // do not include the #
					while !c.is_whitespace() {
						if text.len() <= 0 {
							return (text, Token::String(gathred));
						}
						c = get_next(&mut text);
						if !c.is_alphanumeric() && c != '_' {
							text.push(c);
							return (text, Token::Macro(gathred));
						}
						gathred.push(c);
					}
					return (text, Token::Symbol(gathred));
				}

				// String "..."
				if c == '"' { 
					gathred = "".into(); // do not include the "
					while text.len() > 0 {
						c = get_next(&mut text);
						if c == '"' {
							return (text, Token::String(gathred));
						}
						gathred.push(c);
					}
					panic!("String ended with EOF, '{}'", gathred);
				}

				panic!("Unexpected char '{}'", c);
			}
		}
	}
	(text, Token::EOF)
}
