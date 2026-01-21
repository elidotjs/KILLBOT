// use std::collections::HashMap;
// use crate::toki::{Keywords, Specials, Token};
// use crate::toki::Token::SpecialToken;
// 
// 
// #[derive(Debug)]
// struct VM {
//     instruction_index: usize,
//     variables: HashMap<String, Token>
// }
// 
// impl VM {
//     fn default() -> VM {
//         VM {
//             instruction_index: 0,
//             variables: HashMap::new()
//         }
//     }
// }
// 
// fn get_value(of: Token, vm: VM) -> Token {
//     if let Token::IdentificationToken {name} = &of {
//         if !vm.variables.contains_key(name) {
//             panic!("THERE'S NO VARIABLE NAMED {}!!", name)
//         }
// 
//         return vm.variables[name].clone()
//     }
// 
//     of
// }
// 
// fn execute_instruction(instruction: & Vec<Token>, vm: &mut VM) {
//     if instruction.is_empty() {
//         return
//     }
// 
//     if let Token::KeywordToken {
//         keyword
//     } = instruction.first().unwrap() {
//         match keyword {
//             Keywords::NoOp => {}
//             Keywords::Print => {
// 
//             }
//             Keywords::Var => {
//                 let variable_name = if let Token::IdentificationToken {name} = &instruction[1] {
//                     name
//                 } else {
//                     panic!("SOMETHING WENT WRONG! {}", vm.instruction_index)
//                 };
//                 vm.variables.insert(variable_name.clone(), instruction[2].clone());
//             },
// 
//             Keywords::Add => {
//                 let variable_name = if let Token::IdentificationToken {name} = &instruction[3] {
//                     name
//                 } else {
//                     panic!("SOMETHING WENT WRONG! {}", vm.instruction_index)
//                 };
//             }
//         };
//     }
// }
// 
// pub fn execute(tokens: Vec<Token>) {
//     let mut vm = VM::default();
//     let mut lines: Vec<Vec<Token>> = Vec::new();
//     let mut current_line = Vec::new();
// 
//     for token in tokens {
//         if let SpecialToken {value: Specials::EndOfLine} = token {
//             lines.push(current_line.clone());
//             current_line.clear();
//             continue
//         }
// 
//         current_line.push(token.clone());
//     }
// 
//     while vm.instruction_index < lines.len() {
//         execute_instruction(lines.get(vm.instruction_index).expect("Yea"), &mut vm);
// 
//         vm.instruction_index += 1
//     }
// 
//     println!("{:?}", vm);
// }