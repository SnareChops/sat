use std::env;
use std::fs;

mod lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("an error occurred attempting to open file");
    println!("{:?}", lexer::lex_file(contents))
}
