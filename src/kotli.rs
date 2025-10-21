mod pyth;
mod errorthecat;

use std::collections::HashMap;
use std::io::{Error, ErrorKind};
use crate::kotli::pyth::get_purr;
use crate::pillow;
use crate::pillow::Instruction;
use crate::toki::Token;
use crate::toki::TokenType::*;

struct VM {
    ip: usize, // instruction pointer
    registries: HashMap<String, Token>,
    comparator_bit: bool
}

impl VM {
    fn add_to_registry(&mut self, name: String, what: Token) {
        self.registries.insert(name, what);
    }
}

fn get_token<'a>(token: &Token, vm: &VM) -> Result<Token, Error> {
    match token.token_type {
        Registry => {
            if !vm.registries.contains_key(&token.value) {
                return Err(Error::new(ErrorKind::NotFound, format!("Variable {} not found.", &token.value) ))
            }
            Ok(vm.registries.get(&token.value).unwrap().clone())
        }
        _ => {
            Ok(token.clone())
        }
    }
}

fn execute_instruction(instruction: &Instruction, vm: &mut VM) {
    match instruction {
        Instruction::Kill => {
            println!("KILLBOT: TERMINATED");
            return
        },
        Instruction::Add { op, operand, to } => {
            let op = get_token(op, &vm).expect("what");

            let operand = get_token(operand, &vm).expect("what");

            match pyth::add(&op, &operand) {
                Ok(token) => {
                    vm.registries.insert(to.value.clone(), token);
                }
                Err(e) => {
                    println!("{e}")
                }
            };
        },
        Instruction::Show { what } => {
            println!("{}", get_token(what, &vm).expect("wg").value)
        },
        Instruction::Var { name, value } => {
            vm.add_to_registry(name.clone(), value.clone())
        },
        Instruction::GoBack {
            how_much
        } => {
            vm.ip = (vm.ip as i32 - how_much - 1) as usize;
        },
        Instruction::GoForward {
            how_much
        } => {
            vm.ip = (vm.ip as i32 + how_much - 1 ) as usize;
        }
        Instruction::NoOp => {

        }
        Instruction::Check {
            instruction
        } => {
            if vm.comparator_bit {
                execute_instruction(instruction, vm);
            }
        }

        Instruction::NCheck {
            instruction
        } => {
            if !vm.comparator_bit {
                execute_instruction(instruction, vm);
            }
        }

        Instruction::Compare {
            what, to, comparator
        } => {
            match comparator {
                pillow::Comparator::LessThan => {
                    vm.comparator_bit = get_purr(what).unwrap() < get_purr(to).unwrap()
                }
                pillow::Comparator::GreaterThan => {
                    vm.comparator_bit = get_purr(what).unwrap() > get_purr(to).unwrap()
                }
                pillow::Comparator::EqualsTo => {
                    vm.comparator_bit = get_purr(what).unwrap() == get_purr(to).unwrap()
                }
                pillow::Comparator::NotLessThan => {
                    vm.comparator_bit = get_purr(what).unwrap() >= get_purr(to).unwrap()
                }
                pillow::Comparator::NotGreaterThan => {
                    vm.comparator_bit = get_purr(what).unwrap() <= get_purr(to).unwrap()
                }
                pillow::Comparator::NotEqualsTo => {
                    vm.comparator_bit = get_purr(what).unwrap() != get_purr(to).unwrap()
                }
            }
        }
        _ => {
        }
    }
}
pub fn execute_instructions(instructions: Vec<Instruction>) {
    let mut vm = VM {
        ip: 0,
        registries: HashMap::new(),
        comparator_bit: false
    };

    while vm.ip < instructions.len() {
        execute_instruction(&instructions[vm.ip], &mut vm);
        vm.ip += 1;
    }
}