use std::io::Error;

//
// CAT: Toki
// JOB: Tokenization (hence... the name Toki)
// PIPELINE: Separate Into Chunks -> Compress Into Tokens
// EXECUTE: tokenize(String)
//
// rate my docstrings
use killbot::REGEXES;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Types {
    Text,
    Number,
    Symbol,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    Number(String),
    Symbol(char),
    Generic(Types),
}

pub enum BlueprintToken {
    Text,
    Number,
    Symbol,
}

impl Token {
    pub fn equals_to(&self, other: &Self) -> Result<bool, &'static str> {
        match self {
            Token::Text(_) => Ok(self == other || other == &Token::Generic(Types::Text)),
            Token::Number(_) => Ok(self == other || other == &Token::Generic(Types::Number)),
            Token::Symbol(_) => Ok(self == other || other == &Token::Generic(Types::Symbol)),
            Token::Generic(_) => {
                if let Token::Generic(_) = other {
                    Err("Cannot compare Generic to Generic.")
                } else {
                    other.to_owned().equals_to(self)
                }
            }
        }
    }
}

///
/// "var thing = 10;" -> ["var", "thing", "=", "10", ";"]
///
fn separate_into_chunks(string: String) -> Vec<String> {
    let text_regex = REGEXES.get("text").unwrap();

    let mut current_token = String::new();
    let mut chunks = Vec::new();

    let mut collecting_string = false;

    for char in string.chars() {
        if char == '"' {
            collecting_string = !collecting_string;

            if !collecting_string {
                // we're closing a string. Add the quotations so it's easier to diferentiate.
                let temp = current_token.clone();
                current_token = "\"".to_string();
                current_token.push_str(&temp);
                current_token.push('"');
            }
            continue;
        }

        if collecting_string {
            current_token.push(char);
            continue;
        }

        if text_regex.is_match(&String::from(char)) {
            // shut the hell up, if it matches, it should be pushed.
            current_token.push(char);
        } else {
            if !current_token.trim().is_empty() {
                chunks.push(current_token.clone());
            }
            if !char.to_string().trim().is_empty() {
                chunks.push(char.to_string());
            }
            current_token.clear()
        }
    }

    if !current_token.trim().is_empty() {
        chunks.push(current_token);
    }
    chunks
}

///
/// (also see separate_into_chunks)
/// ["var", "thing", "=", "10", ";"] -> [Text("var"), Text("thing"), Symbol("="), Number("10"), Symbol(";")]
///
fn compress_into_raw_tokens(chunks: Vec<String>) -> Vec<Token> {
    let text_regex = REGEXES.get("text").unwrap();
    let number_regex = REGEXES.get("number").unwrap();

    let mut tokens = vec![];

    for chunk in chunks {
        if number_regex.is_match(&chunk) {
            tokens.push(Token::Number(chunk));
        } else if text_regex.is_match(&chunk) {
            tokens.push(Token::Text(chunk));
        } else {
            tokens.push(Token::Symbol(chunk.chars().nth(0).unwrap()))
        }
    }

    tokens
}

pub fn tokenize(text: String) -> Vec<Token> {
    compress_into_raw_tokens(separate_into_chunks(text))
}

#[test]
fn test_raw_equals() {
    assert!(
        Token::Number(String::from("34.2"))
            .equals_to(&Token::Number(String::from("34.2")))
            .unwrap()
    );

    assert!(
        !Token::Number(String::from("34.2"))
            .equals_to(&Token::Number(String::from("34.3")))
            .unwrap()
    );
}

#[test]
fn test_generic_equals() {
    assert!(
        Token::Number(String::from("34.2"))
            .equals_to(&Token::Generic(Types::Number))
            .unwrap()
    );

    assert!(
        !Token::Number(String::from("34.2"))
            .equals_to(&Token::Generic(Types::Text))
            .unwrap()
    );
}
