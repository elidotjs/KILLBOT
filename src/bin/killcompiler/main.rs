mod kotli;
mod rusty;
mod toki;

fn main() {
    let path = "/home/elidotkt/Desktop/KILLBOT/main.kb";
    println!(
        "{:?}",
        kotli::execute(rusty::instructionize(toki::tokenize(
            std::fs::read_to_string(path).unwrap()
        )))
    )
}
