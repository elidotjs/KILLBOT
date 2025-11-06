use std::collections::HashMap;
use killbot::{Cats, Error, print_error_message};

use crate::pillow::Instruction::{Assignment, Reassignment};
use crate::rusty::{check_structure, LineType};
use crate::toki::Specials::EndOfLine;
use crate::toki::{BlueprintToken, Token, Types};

fn to_lines(tokens: Vec<Token>) -> Vec<Vec<Token>> {
    let (mut current_line, mut result) = (Vec::new(), Vec::new());

    for token in tokens {
        if let Token::SpecialToken {
            value: EndOfLine
        } = token {
            result.push(current_line.clone());
            current_line.clear();
            continue
        }

        current_line.push(token.clone())
    }

    if !current_line.is_empty() {
        result.push(current_line)
    }

    result
}

fn compare_type_value(type1: &Types, value: &Token) -> bool {
    if let BlueprintToken::ValueToken = value.to_blueprint() {
        match type1 {
            Types::Purr => {
                if let Token::PurrValue {..} = value {
                    true
                } else {
                    false
                }
            }
            Types::Wool => {
                if let Token::PurrValue {..} = value {
                    true
                } else {
                    false
                }
            }
            Types::DoublePurr => {
                if let Token::DoublePurrValue {..} = value {
                    true
                } else {
                    false
                }
            }
        }
    } else {
        false
    }
}

fn handle_assignment(tokens: &Vec<Token>, types: &mut HashMap<String, Token>) -> Instruction {
    let identification = if let Token::IdentificationToken { name } = tokens[1].clone() {
        name
    } else {
        // if this runs...
        // rusty literally checked and told me that this is an identification token
        // WHAT DO YOU MEAN IT ISNT????
        // TODO: question reality
        panic!("421")
    };

    let type_compare = if let Token::TypeToken { r#type } = tokens.first().unwrap() {
        r#type
    } else {
        // TODO: what
        panic!("345")
    };

    if !compare_type_value(type_compare, tokens.last().unwrap()) {
        // TODO: incorrect type error
        panic!("400")
    }

    types.insert(identification.clone(), tokens.first().unwrap().clone());

    Assignment {
        what: tokens.last().unwrap().clone(),
        to: identification
    }
}

fn to_instruction(tokens: &Vec<Token>, types: &mut HashMap<String, Token>) -> Instruction {
    let line_type = check_structure(tokens);

    if line_type.is_none() {
        // TODO: invalid line error
        print_error_message(
            Error {
                cat: Cats::Pillow,
                error_message: "Invalid Line Error",
                cause: "The line doesn't adhere to the language's grammar.",
                line: 0
            }
        );
    }

    match check_structure(tokens).unwrap() {
        LineType::Assignment => {
            handle_assignment(tokens, types)
        }
        LineType::Reassignment => {
            let identification = if let Token::IdentificationToken { name } = &tokens[0] {
                name.to_owned()
            } else {
                String::from("")
            };

            if !types.contains_key(&identification) {
                // TODO: trying to reassign to non existent variable
                panic!("100")
            }

            let type_compare = if let Token::TypeToken { r#type } = &types[&identification] {
                r#type
            } else {
                // TODO: what
                panic!("345")
            };

            if !compare_type_value(type_compare, tokens.last().unwrap()) {
                // TODO: incorrect type error
                panic!("400")
            }

            Reassignment {
                what: tokens.last().unwrap().clone(),
                to: identification
            }
        }
        _ => {
            panic!("Yea bro, not yet, hold on my brotha")
        }
    }
}

enum Instruction {
    Assignment {
        what: Token,
        to: String
    },
    Reassignment {
        what: Token,
        to: String
    }
}

pub fn to_instructions(tokens: Vec<Token>) {
    let mut types = HashMap::new();
    
    to_instruction(&tokens, &mut types);
}