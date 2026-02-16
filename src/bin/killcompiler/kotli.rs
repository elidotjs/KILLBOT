//
// CAT: Kotli
// JOB: Execution
// PIPELINE: Compress into Instructions -> Execute
// EXECUTE: execute(Vec<Instructions>)
//
use killbot::Token;
use std::collections::HashMap;

use crate::rusty::Instructions;

#[derive(Debug, Default)]
struct VM<'me> {
    instruction_index: usize,
    variables: HashMap<&'me str, &'me Token>,
}

impl<'me> VM<'me> {
    fn set_variable(&mut self, name: &'me String, value: &'me Token) -> Option<&'me Token> {
        self.variables.insert(name, value)
    }

    fn get_variable(&self, name: &'me String) -> &Token {
        self.variables.get(name.as_str()).unwrap()
    }
}

fn print(vm: &VM, what: &Token) {
    match what {
        Token::Text(value) => {
            if value.starts_with("\"") {
                println!("{}", value[1..value.len() - 1].to_string());
            } else {
                print(vm, vm.get_variable(&value));
            }
        }
        Token::Number(value) => {
            println!("{}", value)
        }
        _ => {
            panic!("cannot print that!")
        }
    }
}

pub fn execute(instructions: Vec<Instructions>) {
    let mut vm = VM::default();

    while vm.instruction_index < instructions.len() {
        match instructions.get(vm.instruction_index).unwrap() {
            Instructions::VariableAssignment(variable, value) => {
                vm.set_variable(variable, value);
            }
            Instructions::Print(what) => print(&vm, what),
        };

        vm.instruction_index += 1
    }

    println!("{:?}", vm)
}
