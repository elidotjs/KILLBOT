use crate::toki::Token;

#[derive(Clone, Debug)]
pub enum Instruction {
    Add {
        // KB1
        op: Token,
        operand: Token,
        to: Token,
    },
    Var {
        // KB0
        name: String,
        value: Token,
    },
    Show {
        // KB2
        what: Token,
    },
    GoBack {
        // KB3
        how_much: i32,
    },
    GoForward {
        // KB4
        how_much: i32,
    },
    Kill, // Murder
    Compare {
        what: Token,
        to: Token,
        comparator: Comparator,
    },
    Check {
        instruction: Box<Instruction>,
    },
    NCheck {
        instruction: Box<Instruction>,
    },
    NoOp,
}

#[derive(Clone, Debug)]
pub enum Comparator {
    GreaterThan,
    LessThan,
    EqualsTo,
    NotGreaterThan,
    NotLessThan,
    NotEqualsTo,
}

fn to_instruction(line: &Vec<Token>) -> Instruction {
    match line.first().unwrap().value.to_lowercase().as_str() {
        "var" => {
            // var
            Instruction::Var {
                name: line[1].value.clone(),
                value: line[2].clone(),
            }
        }
        "add" => {
            // add
            Instruction::Add {
                op: line[1].clone(),
                operand: line[2].clone(),
                to: line[3].clone(),
            }
        }
        "show" => {
            // show
            Instruction::Show {
                what: line[1].clone(),
            }
        }
        "gobk" => {
            // go back
            let how_much: i32 = line[1].value.parse().expect("what");
            Instruction::GoBack { how_much }
        }
        "gofw" => {
            // go forward
            let how_much: i32 = line[1].value.parse().expect("what");
            Instruction::GoForward { how_much }
        }
        "murder" => Instruction::Kill,
        "cmp" => {
            let comparator = match line[2].value.to_lowercase().as_str() {
                "lt" => Comparator::LessThan,
                "gt" => Comparator::GreaterThan,
                "et" => Comparator::EqualsTo,
                "nlt" => Comparator::NotLessThan,
                "ngt" => Comparator::NotGreaterThan,
                "net" => Comparator::NotEqualsTo,
                _ => {
                    panic!("WHAT THE FUCK NO WHAT")
                }
            };

            Instruction::Compare {
                what: line[1].clone(),
                to: line[3].clone(),
                comparator,
            }
        }

        "chk" => {
            let mut new_line = line.clone();
            new_line.remove(0);
            Instruction::Check {
                instruction: Box::from(to_instruction(&new_line)),
            }
        }

        "nchk" => {
            let mut new_line = line.clone();
            new_line.remove(0);
            Instruction::NCheck {
                instruction: Box::from(to_instruction(&new_line)),
            }
        }

        _ => Instruction::NoOp,
    }
}

pub fn to_instructions(lines: Vec<Vec<Token>>) -> Vec<Instruction> {
    let mut instructions = Vec::new();
    for line in lines {
        instructions.push(to_instruction(&line))
    }

    instructions
}
