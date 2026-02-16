use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

///
/// "text",
/// "number"
///
pub static REGEXES: LazyLock<HashMap<String, Regex>> = LazyLock::new(|| {
    let mut temp = HashMap::new();
    temp.insert("text".to_string(), Regex::new(r"[a-zA-Z0-9_]").unwrap());
    temp.insert("number".to_string(), Regex::new(r"\d+\.\d+|\d+").unwrap());

    temp
});

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Types {
    Text,
    Number,
    Symbol,
    Any,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Text(String),
    Number(String),
    Symbol(char),
    Generic(Types),
}

impl From<&str> for Token {
    fn from(what: &str) -> Token {
        let text_regex = REGEXES.get("text").unwrap();
        let number_regex = REGEXES.get("number").unwrap();

        if number_regex.is_match(what) {
            Token::Number(what.to_string())
        } else if text_regex.is_match(what) {
            Token::Text(what.to_string())
        } else {
            Token::Symbol(what.chars().next().unwrap())
        }
    }
}

impl From<&String> for Token {
    fn from(what: &String) -> Token {
        Token::from(what.as_str())
    }
}

/// implements StartsWith for tokens
pub trait TokenCollection {
    fn starts_with_tokens(&self, with_what: &[Token]) -> bool;
}

impl TokenCollection for [Token] {
    fn starts_with_tokens(&self, with_what: &[Token]) -> bool {
        if self.len() < with_what.len() {
            return false;
        }

        for (index, token) in self.iter().enumerate() {
            let with_what_item = with_what.get(index);

            if with_what_item.is_none() {
                break;
            }

            if !token.equals_to(with_what_item.unwrap()).unwrap() {
                return false;
            };
        }

        true
    }
}

impl Token {
    pub fn equals_to(&self, other: &Self) -> Result<bool, &'static str> {
        if other == &Token::Generic(Types::Any) {
            return Ok(true);
        }
        let result = match self {
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
        };
        if cfg!(test) {
            println!("({:?}, {:?}), {:?}", self, other, result);
        }
        result
    }
}
