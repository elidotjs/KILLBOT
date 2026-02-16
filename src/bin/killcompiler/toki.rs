use std::vec::IntoIter;

//
// CAT: Toki
// JOB: Tokenization (hence... the name Toki)
// PIPELINE: Separate Into Chunks -> Compress Into Tokens
// EXECUTE: tokenize(String)
//
// rate my docstrings
use killbot::*;

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
                // we're closing a string. Add the quotations so it's easier to differentiate.
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
fn compress_into_raw_tokens<'a>(chunks: Vec<String>) -> IntoIter<Token> {
    let tokens: Vec<_> = chunks.iter().map(|chunk| Token::from(chunk)).collect();

    tokens.into_iter()
}

pub fn tokenize<'a>(text: String) -> Vec<Token> {
    let raw_tokens = compress_into_raw_tokens(separate_into_chunks(text));
    raw_tokens.collect()
}

#[test]
fn test_raw_equals() {
    assert!(
        Token::Number("34.2".to_string())
            .equals_to(&Token::Number("34.2".to_string()))
            .unwrap()
    );

    assert!(
        !Token::Number("34.2".to_string())
            .equals_to(&Token::Number("34.3".to_string()))
            .unwrap()
    );
}

#[test]
fn test_generic_equals() {
    assert!(
        Token::Number("34.2".to_string())
            .equals_to(&Token::Generic(Types::Number))
            .unwrap()
    );

    assert!(
        !Token::Number("34.2".to_string())
            .equals_to(&Token::Generic(Types::Text))
            .unwrap()
    );
}
