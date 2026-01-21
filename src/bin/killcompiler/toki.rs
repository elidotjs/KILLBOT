use regex::Regex;
use crate::toki::Keywords::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    NoOp,
    Print,
    Var,
    Add
}

#[derive(Debug, Clone, PartialEq)]
pub enum Operators {
    Plus,
    Minus,
    Multiplication,
    Divide,
    Equals,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Types {
    Purr,
    Wool,
    DoublePurr,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Specials {
    Period,
    EndOfLine,
}



#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    TypeToken { r#type: Types },
    PurrValue { value: i32 },
    WoolValue { value: String },
    DoublePurrValue { value: f64 },
    KeywordToken { keyword: Keywords },
    ExpressionToken { expression: Vec<Token> },
    OperatorToken { operator: Operators },
    IdentificationToken { name: String },
    SpecialToken { value: Specials },
}

fn separate_into_chunks(string: String) -> Vec<String> {
    let mut current_token = String::new();
    let mut chunks = Vec::new();

    let mut collecting_string = false;

    for char in string.chars() {
        if char == '"' {
            collecting_string = !collecting_string
        }
        if char.is_alphanumeric() || char == '_' || char == '.' {
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

fn categorize_into_tokens(chunks: Vec<String>) -> Vec<Token> {
    let iter = chunks.iter().peekable();

    chunks
        .iter()
        .peekable()
        .map(|chunk| match chunk.as_str() {
            "print" => {
                Token::KeywordToken {
                    keyword: Print
                }
            },
            "var" => {
                Token::KeywordToken {
                    keyword: Var
                }
            },
            "add" => {
                Token::KeywordToken {
                    keyword: Add
                }
            },
            "wool" => Token::TypeToken {
                r#type: Types::Wool,
            },
            "purr" => Token::TypeToken {
                r#type: Types::Purr,
            },
            "doublepurr" => Token::TypeToken {
                r#type: Types::DoublePurr,
            },
            "=" => Token::OperatorToken {
                operator: Operators::Equals,
            },
            "+" => Token::OperatorToken {
                operator: Operators::Plus,
            },
            "-" => Token::OperatorToken {
                operator: Operators::Minus,
            },
            "/" => Token::OperatorToken {
                operator: Operators::Divide,
            },
            "." => Token::SpecialToken {
                value: Specials::Period,
            },
            ";" => Token::SpecialToken {
                value: Specials::EndOfLine,
            },
            _ => {
                let is_purr = {
                    let validator = Regex::new(r"^(\d+)$").unwrap();
                    validator.is_match(chunk)
                };

                let is_double_purr = {
                    let validator = Regex::new(r"^(\d+\.\d+)$").unwrap();
                    validator.is_match(chunk)
                };

                let is_wool = {
                    let validator = Regex::new("^\"(.+)\"$").unwrap();
                    validator.is_match(chunk)
                };

                if is_purr {
                    Token::PurrValue {
                        value: chunk.parse().unwrap(),
                    }
                } else if is_double_purr {
                    Token::DoublePurrValue {
                        value: chunk.parse().unwrap(),
                    }
                } else if is_wool {
                    Token::WoolValue {
                        value: chunk.parse().unwrap(),
                    }
                } else {
                    Token::IdentificationToken {
                        name: chunk.clone(),
                    }
                }
            }
        })
        .collect()
}

pub fn tokenize(string: String) -> Vec<Token> {
    categorize_into_tokens(separate_into_chunks(string))
}