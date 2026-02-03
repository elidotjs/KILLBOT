//
// CAT: Kotli
// JOB: Execution
// PIPELINE: Compress into Instructions -> Execute
// EXECUTE: execute(Vec<RawToken>)
//
use crate::toki::*;
use std::collections::HashMap;

#[derive(Debug)]
struct VM {
    instruction_index: usize,
    variables: HashMap<String, Token>,
}

impl VM {
    fn default() -> VM {
        VM {
            instruction_index: 0,
            variables: HashMap::new(),
        }
    }
}

fn compress_into_instructions(tokens: Vec<RawToken>) {}
