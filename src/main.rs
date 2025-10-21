mod toki;
mod kotli;
mod pillow;

use std::env;
use std::io::Read;
use std::fs::File;
use std::process::exit;
use crate::kotli::execute_instructions;

fn open_file(path: &str) -> std::io::Result<String> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let contents = open_file(if args.len() > 1 {
        args[1].as_str()
    } else {
        println!("Hey, i kind of need a file here, you know...");
        exit(200)
    }).expect("WHAT");
    let tokens = toki::split_into_text_tokens(contents);
    let cleaned_up = toki::clean_up_whitespace(tokens);
    let tokenized = toki::text_to_tokens(cleaned_up);
    let lines = toki::to_lines(tokenized);
    let instructions = pillow::to_instructions(lines);

    execute_instructions(instructions);
}
