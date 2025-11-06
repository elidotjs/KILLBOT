use std::collections::HashMap;
use std::sync::LazyLock;
use regex::Regex;

static OPERATOR_PRECEDENCE: LazyLock<HashMap<&str, i8>> = LazyLock::new(|| {
    HashMap::from(
        [
            ("add", 0),
            ("subtract", 0),
            ("multiply", 1),
            ("divide", 1),
        ]
    )
});

#[derive(Debug, Clone, PartialEq)]
pub enum Keywords {
    NoOp,
    Print { what: String },
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


#[derive(Debug, PartialEq)]
pub enum AbstractToken {
    Type,
    Special,
    Value,
    Identification,
    ExpressionToken
}

#[derive(Debug, PartialEq)]
pub enum MetaToken {
    Any,
    End,
    WhateverIsNext
}

#[derive(Debug, PartialEq)]
pub enum BlueprintToken {
    // Types
    Abstract {
        token: AbstractToken
    },

    Meta {
        token: MetaToken
    },

    PurrType,
    DoublePurrType,
    WoolType,

    Period,
    EndOfLine,

    ExpressionToken,

    // Values
    ValueToken,
    PurrValue,
    DoublePurrValue,
    WoolValue,

    KeywordToken,
    IdentificationToken,

    // operators
    OperatorToken,
    EqualsToken,
    MinusToken,
    PlusToken,
    MultiplicationToken,
    DivideToken,
}

impl Token {
    pub fn to_blueprint(&self) -> BlueprintToken {
        match self {
            Token::PurrValue { .. } => BlueprintToken::PurrValue,
            Token::WoolValue { .. } => BlueprintToken::WoolValue,
            Token::DoublePurrValue { .. } => BlueprintToken::DoublePurrValue,

            Token::SpecialToken { value } => {
                match value {
                    Specials::Period => BlueprintToken::Period,
                    Specials::EndOfLine => BlueprintToken::EndOfLine,
                }
            },

            Token::IdentificationToken { .. } => BlueprintToken::IdentificationToken,
            

            Token::KeywordToken { .. } => BlueprintToken::KeywordToken,
            Token::ExpressionToken { .. } => BlueprintToken::ExpressionToken,

            Token::TypeToken { r#type } => {
                match r#type {
                    Types::Purr => BlueprintToken::PurrType,
                    Types::Wool => BlueprintToken::WoolType,
                    Types::DoublePurr => BlueprintToken::DoublePurrType,
                }
            },


            Token::OperatorToken { operator } => {
                match operator {
                    Operators::Plus => BlueprintToken::PlusToken,
                    Operators::Minus => BlueprintToken::MinusToken,
                    Operators::Multiplication => BlueprintToken::MultiplicationToken,
                    Operators::Divide => BlueprintToken::DivideToken,
                    Operators::Equals => BlueprintToken::EqualsToken,
                }
            },
        }
    }
}

pub fn to_blueprint_tokens(tokens: &Vec<Token>) -> Vec<BlueprintToken> {
    tokens
        .iter()
        .map(|token| token.to_blueprint())
        .collect()
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
    chunks
        .iter()
        .map(|chunk| match chunk.as_str() {
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