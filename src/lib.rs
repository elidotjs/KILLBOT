use regex::Regex;
use std::collections::HashMap;
use std::sync::LazyLock;

///
/// "text",
/// "number"
///
pub static REGEXES: LazyLock<HashMap<String, Regex>> = LazyLock::new(|| {
    let mut temp = HashMap::new();
    temp.insert("text".to_string(), Regex::new(r"[a-zA-Z0-9_]").unwrap());
    temp.insert("number".to_string(), Regex::new(r"\d+\.\d+|\d+").unwrap());

    temp
});
