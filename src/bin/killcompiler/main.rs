mod kotli;
mod toki;

fn main() {
    let path = "/home/elidotkt/Desktop/KILLBOT/main.kb";
    toki::tokenize(std::fs::read_to_string(path).expect("Brazil"))
}
