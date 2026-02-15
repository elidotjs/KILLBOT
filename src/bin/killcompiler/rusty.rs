//
// CAT: Rusty
// JOB: Validator / Instruction Making
// PIPELINE: Validate structure -> Compress into Instructions
// EXECUTE: instructionize(Vec<Token>)
//
//
#![allow(dead_code)]
use crate::toki::{Token, Types};
use std::sync::LazyLock;

static TABLE: LazyLock<Vec<(Vec<Token>, GenericInstructions)>> = LazyLock::new(|| {
    vec![
        (
            vec![
                Token::Text(String::from("var")),
                Token::Generic(Types::Text),
                Token::Symbol('='),
                Token::Generic(Types::Any),
                Token::Symbol(';'),
            ],
            GenericInstructions::VariableAssignment,
        ),
        (
            vec![
                Token::Generic(Types::Text),
                Token::Symbol('='),
                Token::Generic(Types::Any),
                Token::Symbol(';'),
            ],
            GenericInstructions::VariableReassignment,
        ),
    ]
});

#[derive(Debug)]
enum Instructions {
    VariableAssignment(String, Token),
}

enum GenericInstructions {
    VariableAssignment,
    VariableReassignment,
}

fn starts_with(tokens: &[Token], with_what: &[Token]) -> bool {
    if tokens.len() < with_what.len() {
        return false;
    }

    println!("{:?}, {:?}", tokens, with_what);

    for (index, token) in tokens.iter().enumerate() {
        let with_what_item = with_what.get(index).unwrap();
        println!("{:?} ({:?})", with_what_item, token);

        token.equals_to(with_what_item).unwrap();
    }

    true
}

pub fn instructionize(tokens: Vec<Token>) {
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut instructions_vec: Vec<Instructions> = Vec::new();

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
            if !starts_with(&line, potential_match) {
                continue;
            }

            instructions_vec.push(match instruction {
                GenericInstructions::VariableAssignment => {
                    let name;

                    if let Token::Text(value) = line.get(1).unwrap() {
                        name = value;
                    } else {
                        panic!("what")
                    }

                    Instructions::VariableAssignment(name.to_owned(), line.get(3).unwrap().clone())
                }
                GenericInstructions::VariableReassignment => {
                    let name;

                    if let Token::Text(value) = line.first().unwrap() {
                        name = value;
                    } else {
                        panic!("what")
                    }

                    Instructions::VariableAssignment(name.to_owned(), line.get(2).unwrap().clone())
                }
            })
        }
    }

    println!("{:?}", instructions_vec)
}
