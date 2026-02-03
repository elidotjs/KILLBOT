//
// CAT: Rusty
// JOB: Validator / Instruction Making
// PIPELINE: Validate structure -> Compress into Instructions
// EXECUTE: instructionize(Vec<Token>)
//

use std::sync::LazyLock;

static TABLE: LazyLock<Vec<Vec<Token>>> =
    LazyLock::new(|| vec![vec![Token::Text(String::from("var"))]]);

use crate::toki::Token;

enum Instructions {
    VariableAssignment(Token),
}

fn starts_with(tokens: &Vec<Token>, with_what: Vec<Token>) -> bool {
    if tokens.len() < with_what.len() {
        return false;
    }

    for (index, token) in tokens.iter().enumerate() {
        let with_what_item = with_what.get(index).unwrap();

        token.equals_to(with_what_item).unwrap();
    }

    true
}

pub fn instructionize(tokens: Vec<Token>) {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();

    for token in tokens {
        if token == Token::Symbol(';') {
            lines.push(current_line.clone());
            current_line.clear();
            continue;
        }

        current_line.push(token);
    }

    println!("{:?}", lines)
}
