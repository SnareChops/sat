use std::env;
use std::fs;

mod lexer;
mod parser;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("an error occurred attempting to open file");
    let ast = lexer::lex_file(args[1].clone(), contents);
    println!("{:?}", ast);
    let program = parser::parse(ast);
    match program {
        Err(e) => eprintln!("{}", e.message()),
        Ok(p) => println!("{:#?}", p),
    }
}
