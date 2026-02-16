//
// CAT: Rusty
// JOB: Validator / Instruction Making
// PIPELINE: Validate structure -> Compress into Instructions
// EXECUTE: instructionize(Vec<Token>)
//
//
#![allow(dead_code)]
use crate::toki;
use killbot::*;
use std::sync::LazyLock;

static TABLE: LazyLock<Vec<(Vec<Token>, GenericInstructions)>> = LazyLock::new(|| {
    vec![
        (
            vec![
                Token::from("var"),
                Token::Generic(Types::Text),
                Token::from("="),
                Token::Generic(Types::Any),
                Token::from(";"),
            ],
            GenericInstructions::VariableAssignment,
        ),
        (
            vec![
                Token::from("print"),
                Token::Generic(Types::Any),
                Token::from(";"),
            ],
            GenericInstructions::Print,
        ),
    ]
});

#[derive(Debug)]
pub enum Instructions {
    VariableAssignment(String, Token),
    Print(Token),
}

enum GenericInstructions {
    VariableAssignment,
    VariableReassignment,
    Print,
}

pub fn instructionize<'a>(tokens: Vec<Token>) -> Vec<Instructions> {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut instructions_vec = Vec::new();

    for token in tokens {
        current_line.push(token.clone());

        if token == Token::Symbol(';') {
            lines.push(current_line.clone());
            current_line.clear();
            continue;
        }
    }

    for line in lines {
        for (potential_match, instruction) in TABLE.iter() {
            if !line.starts_with_tokens(potential_match) {
                continue;
            }

            instructions_vec.push(match instruction {
                GenericInstructions::VariableAssignment => {
                    let name;

                    if let Token::Text(value) = line.get(1).unwrap() {
                        name = value;
                    } else {
                        unreachable!("This is supposed to be text, but apparently it isn't?!")
                    }

                    Instructions::VariableAssignment(name.to_owned(), line.get(3).unwrap().clone())
                }
                GenericInstructions::VariableReassignment => {
                    let name;

                    if let Token::Text(value) = line.first().unwrap() {
                        name = value;
                    } else {
                        unreachable!("This is supposed to be text, but apparently it isn't?!")
                    }

                    Instructions::VariableAssignment(name.to_owned(), line.get(2).unwrap().clone())
                }
                GenericInstructions::Print => Instructions::Print(line.get(1).unwrap().clone()),
            })
        }
    }

    instructions_vec
}

#[test]
fn test_sw() {
    assert!(
        toki::tokenize("var literally anything here".to_string())
            .starts_with_tokens(&[Token::from("var")])
    );

    assert!(
        toki::tokenize("var what".to_string())
            .starts_with_tokens(&[Token::from("var"), Token::Generic(Types::Text)])
    );
    assert!(
        toki::tokenize("var thing ;".to_string()).starts_with_tokens(&[
            Token::from("var"),
            Token::Generic(Types::Text),
            Token::from(";")
        ])
    );

    assert!(
        !&toki::tokenize("var taiwan = 521;".to_string()).starts_with_tokens(&[
            Token::Generic(Types::Text),
            Token::Symbol('='),
            Token::Generic(Types::Any),
            Token::Symbol(';'),
        ])
    );
}
