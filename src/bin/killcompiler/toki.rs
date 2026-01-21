use std::collections::HashMap;
use std::sync::LazyLock;

const pairs: LazyLock<HashMap<&str, Token>, fn() -> HashMap<&'static str, Token>> = LazyLock::new(|| {
    HashMap::from(
        [
            ("print", Token::Keyword(Keyword::Print)),
            ("add", Token::Keyword(Keyword::Add)),
            ("var", Token::Keyword(Keyword::Var)),
            (";", Token::Special(Special::EndOfLine))
        ]
    )
});


enum Value {
    Wool(String),
    Purr(i32),
    DoublePurr(f64)
}

enum Keyword {
    Add,
    Print,
    Var
}

enum Special {
    EndOfLine
}

enum Token {
    Value(Value),
    Keyword(Keyword),
    Special(Special)
}

fn separate_into_chunks(string: String) -> Vec<String> {
    let mut current_token = String::new();
    let mut chunks = Vec::new();

    let mut collecting_string = false;

    for char in string.chars() {
        if char == '"' {
            collecting_string = !collecting_string;
            continue
        }

        if collecting_string {
            current_token.push(char);
            continue
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

pub fn tokenize(text: String) {
    println!("{:?}", separate_into_chunks(text))
}