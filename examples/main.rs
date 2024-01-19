use css_parser_rs::Lexer;
use std::fs;

fn main() {
    let content = fs::read_to_string("./examples/styles.css").expect("ERROR: couldn't read file.");

    for event in Lexer::new(&content).parse() {
        println!("{event:?}");
    }
}
