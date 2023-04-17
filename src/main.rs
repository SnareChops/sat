use std::env;
use std::fs;

mod lexer;
mod parser;
mod runner;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("an error occurred attempting to open file");
    let ast = lexer::lex_file(args[1].clone(), contents);
    let parse_result = parser::parse(ast);
    match parse_result {
        parser::ParseResult::Err(..) => eprintln!("{}", parse_result.message()),
        parser::ParseResult::Ok(p) => {
            let result = runner::run(p);
            println!("{}", result.message());
        }
    }
}
