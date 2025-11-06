#[derive(Debug)]
pub enum Cats {
    Toki,
    Kotli,
    Pyth,
    Pillow
}

pub struct Error {
    pub cat: Cats, 
    pub error_message: &'static str, 
    pub cause: &'static str,
    pub line: u64
}

pub fn print_error_message(error: Error) {
    let Error {cat, error_message, cause, line} = error;
    println!("{:?} - {error_message}", cat);
    cause.split("\n").into_iter().for_each(
        |line| {
            println!(": {line}");
        }
    );
}