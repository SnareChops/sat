use std::env;
use std::fs;

mod lexer;
mod parser;
mod runner;
mod types;

fn main() {
    let args: Vec<String> = env::args().collect();
    let contents = fs::read_to_string(&args[1]).expect("an error occurred attempting to open file");
    println!("starting lex...");
    let mut ast = lexer::lex_file(args[1].clone(), contents);
    println!("starting parse...");
    let parse_result = parser::parse(&mut ast);
    println!("starting run...");
    match parse_result {
        parser::ParseResult::Err(..) => eprintln!("{}", parse_result.message()),
        parser::ParseResult::Ok(p) => match runner::run(p) {
            runner::RunResult::Ok(feedback) => println!("{}", feedback.to_string()),
            runner::RunResult::Err(loc, err) => println!("{}:\n{err}", loc.to_string()),
        },
    }
}
