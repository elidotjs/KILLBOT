use crate::toki::TokenType::Special;

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum ValueType {
    Purr,
    DoublePurr,
    Wool
}

#[derive(Debug)]
#[derive(Copy, Clone)]
pub enum TokenType {
    Keyword,
    Type,
    Identification,
    Operator,
    ArithmeticOperator,
    Special,
    Registry,
    Value {
        value_type: ValueType
    }
}

#[derive(Debug)]
#[derive(Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType
}

pub fn split_into_text_tokens(string: String) -> Vec<String> {
    let mut current_token = String::new();

    let mut tokens: Vec<String> = Vec::new();

    let mut collecting_string = false;

    for char in string.chars() {
        if char.is_alphanumeric() || char == '_' || char == '$' {
            current_token.push(char);
        } else if char == '"' {
            current_token.push(char);
            collecting_string = !collecting_string;
        } else {
            if collecting_string {
                current_token.push(char);
                continue
            }

            tokens.push(
                current_token.clone()
            );
            tokens.push(char.to_string());
            current_token.clear()
        }
    }

    if !current_token.is_empty() {
        tokens.push(current_token);
    }

    tokens
}

pub fn clean_up_whitespace(tokens: Vec<String>) -> Vec<String> {
    tokens.into_iter().filter(
        |x| {!x.trim().is_empty()}
    ).collect()
}

pub fn text_to_tokens(tokens: Vec<String>) -> Vec<Token> {
    let mut token_vector: Vec<Token> = Vec::new();

    for token in tokens {
        let token_type: TokenType;

        match token.to_lowercase().as_str() {
            "purr" => token_type = TokenType::Type,
            "wool" => token_type = TokenType::Type,
            "doublepurr" => token_type = TokenType::Type,

            "+" => token_type = TokenType::ArithmeticOperator,
            "-" => token_type = TokenType::ArithmeticOperator,
            "*" => token_type = TokenType::ArithmeticOperator,
            "/" => token_type = TokenType::ArithmeticOperator,

            "=" => token_type = TokenType::Operator,

            ";" => {
                token_type = Special
            },
            _ => {
                let mut only_numbers = true;

                for letter in token.chars() {
                    if !letter.is_numeric() {
                        only_numbers = false;
                        break
                    }
                }

                if only_numbers {
                    token_type = TokenType::Value {
                        value_type: ValueType::Purr
                    }
                } else if token.starts_with("$") {
                    token_type = TokenType::Registry
                } else if token.starts_with('"') && token.starts_with('"') {
                    token_type = TokenType::Value {
                        value_type: ValueType::Wool
                    }
                } else {
                    token_type = TokenType::Identification
                }
            }
        };

        let result = Token {
            value: token,
            token_type
        };

        let _ = &token_vector.push(result);
    }

    token_vector
}

pub fn to_lines(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let mut current_line: Vec<Token> = Vec::new();
    let mut result: Vec<Vec<Token>> = Vec::new();
    for token in tokens {
        match token.token_type {
            Special => {
                if token.value == ";" {
                    result.push(current_line.clone());
                    current_line.clear();
                } else {
                    current_line.push(token);
                }
            }
            _ => {
                current_line.push(token);
            }
        }
    }

    result
}