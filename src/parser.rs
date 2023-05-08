use crate::runner;
use crate::types;
use std::collections::{HashMap, VecDeque};

#[derive(Clone, Debug)]
pub enum Special {
    Assign,
    Equality,
}

#[derive(Clone, Debug)]
pub enum Token {
    Symbol(types::Location, String),
    String(types::Location, String),
    Number(types::Location, String),
    Block(types::Location, Vec<Tokens>),
    Object(types::Location, HashMap<String, Tokens>),
    Array(types::Location, Vec<Tokens>),
    Special(types::Location, Special),
    Pipe(types::Location, Tokens),
    Dot(types::Location),
    Eol(types::Location),
}
impl Token {
    pub fn loc(&self) -> types::Location {
        match self {
            Token::Symbol(loc, ..) => loc.clone(),
            Token::String(loc, ..) => loc.clone(),
            Token::Number(loc, ..) => loc.clone(),
            Token::Block(loc, ..) => loc.clone(),
            Token::Object(loc, ..) => loc.clone(),
            Token::Array(loc, ..) => loc.clone(),
            Token::Special(loc, ..) => loc.clone(),
            Token::Pipe(loc, ..) => loc.clone(),
            Token::Dot(loc) => loc.clone(),
            Token::Eol(loc) => loc.clone(),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Tokens(VecDeque<Token>);
impl Tokens {
    pub fn new() -> Tokens {
        Tokens(VecDeque::new())
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn peek(&self) -> Option<Token> {
        self.0.get(0).cloned()
    }
    pub fn peek_at(&self, index: usize) -> Option<Token> {
        self.0.get(index).cloned()
    }
    pub fn last(&self) -> Option<Token> {
        self.0.get(self.0.len() - 1).cloned()
    }
    pub fn split_on_first_special(&mut self) -> Option<(Special, Tokens, Tokens)> {
        let mut i = 0;
        while i < self.0.len() {
            if let Some(Token::Special(.., special)) = self.0.get(i) {
                let mut left = Tokens::new();
                let mut right = Tokens::new();
                let mut x = 0;
                while x < i {
                    left.add(self.0[x].clone());
                    x += 1;
                }
                // Skip the special token
                x += 1;
                while x < self.0.len() {
                    right.add(self.0[x].clone());
                    x += 1;
                }
                return Some((special.clone(), left, right));
            }
            i += 1;
        }
        None
    }
    pub fn take(&mut self) -> Option<Token> {
        self.0.pop_front()
    }
    pub fn add(&mut self, token: Token) {
        self.0.push_back(token)
    }
}

pub enum ParseResult<T> {
    Ok(T),
    Err(types::Location, String),
}
impl<T> ParseResult<T> {
    pub fn message(&self) -> String {
        match self {
            ParseResult::Err(loc, err) => format!("ParseError: {}: {}", loc.to_string(), err),
            _ => "".to_string(),
        }
    }
}

pub fn parse(tokens: &mut Tokens) -> ParseResult<runner::Program> {
    println!("parse {tokens:?}\n---");
    let mut program = runner::Program::new();
    while let Some(..) = tokens.peek() {
        match parse_declaration(tokens) {
            ParseResult::Err(loc, err) => return ParseResult::Err(loc, err),
            ParseResult::Ok(declaration) => program.add_declaration(declaration),
        }
    }
    return ParseResult::Ok(program);
}

fn parse_declaration(tokens: &mut Tokens) -> ParseResult<runner::Declaration> {
    println!("parse_declaration\n\t{:?}\n---", tokens.peek());
    match tokens.take() {
        Some(Token::Symbol(loc, symbol)) => match symbol.as_str() {
            "test" => parse_test(loc, tokens),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn parse_test(location: types::Location, tokens: &mut Tokens) -> ParseResult<runner::Declaration> {
    println!("parse_test\n\t{location:?}\n\t{:?}\n---", tokens);
    if let Some(Token::String(.., name)) = tokens.take() {
        if let Some(Token::Block(loc, ref mut lines)) = tokens.take() {
            match parse_block(loc.clone(), lines) {
                ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
                ParseResult::Ok(block) => {
                    ParseResult::Ok(runner::Declaration::Test(location.clone(), name, block))
                }
            }
        } else {
            ParseResult::Err(location.clone(), "Expected block".to_string())
        }
    } else {
        ParseResult::Err(location.clone(), "Expected test name".to_string())
    }
}

fn parse_block(location: types::Location, lines: &mut Vec<Tokens>) -> ParseResult<runner::Block> {
    println!("parse_block\n\t{location:?}\n\t{lines:?}\n---");
    let mut block = runner::Block::new(location.clone());
    let mut iter = lines.iter_mut();
    while let Some(tokens) = iter.next() {
        match parse_expression(tokens) {
            ParseResult::Ok(expression) => block.add_expression(expression),
            ParseResult::Err(loc, err) => return ParseResult::Err(loc, err),
        }
    }
    ParseResult::Ok(block)
}

fn parse_expression(tokens: &mut Tokens) -> ParseResult<runner::Expression> {
    println!("parse_expression\n\t{tokens:?}\n---");

    // Look for special expression tokens within the expression
    match tokens.split_on_first_special() {
        Some((Special::Assign, ref mut left, ref mut right)) => parse_assign(left, right),
        Some((Special::Equality, ref mut left, ref mut right)) => parse_equality(left, right),
        None => match tokens.peek() {
            Some(Token::Symbol(loc, symbol)) => {
                if let Some(Token::Dot(..)) = tokens.peek_at(1) {
                    parse_ref(tokens)
                } else {
                    match symbol.as_str() {
                        "true" => {
                            tokens.take();
                            ParseResult::Ok(runner::Expression::Primitive(
                                loc,
                                runner::Primitive::Boolean(true),
                            ))
                        }
                        "false" => {
                            tokens.take();
                            ParseResult::Ok(runner::Expression::Primitive(
                                loc,
                                runner::Primitive::Boolean(false),
                            ))
                        }
                        "assert" => {
                            tokens.take();
                            parse_assert(loc.clone(), tokens)
                        }
                        "get" => {
                            tokens.take();
                            parse_get(loc.clone(), tokens)
                        }
                        _ => parse_ref(tokens),
                    }
                }
            }
            Some(Token::Number(loc, number)) => {
                tokens.take();
                if let Ok(number) = number.parse::<f32>() {
                    ParseResult::Ok(runner::Expression::Primitive(
                        loc,
                        runner::Primitive::Number(number),
                    ))
                } else {
                    ParseResult::Err(loc, format!("Unable to parse {number:?} as a number"))
                }
            }
            Some(Token::String(loc, string)) => {
                tokens.take();
                ParseResult::Ok(runner::Expression::Primitive(
                    loc,
                    runner::Primitive::String(string),
                ))
            }
            Some(Token::Object(loc, ref mut map)) => {
                tokens.take();
                parse_object(loc, map)
            }
            Some(Token::Array(loc, ref mut array)) => {
                tokens.take();
                parse_array(loc, array)
            }
            _ => ParseResult::Err(
                tokens
                    .peek()
                    .unwrap_or(Token::Eol(types::Location("".to_owned(), 0, 0)))
                    .loc(),
                "Unknown expression type".to_string(),
            ),
        },
    }
}

fn parse_ref(tokens: &mut Tokens) -> ParseResult<runner::Expression> {
    println!("parse_ref\n\t{tokens:?}\n---");
    let loc = tokens.peek().unwrap().loc();
    let mut symbol = "".to_string();
    while let Some(token) = tokens.peek() {
        match token {
            Token::Dot(..) => {
                tokens.take();
                symbol.push('.')
            }
            Token::Symbol(.., value) => {
                tokens.take();
                symbol += value.as_str()
            }
            Token::Number(.., value) => {
                tokens.take();
                symbol += value.as_str()
            }
            _ => return ParseResult::Ok(runner::Expression::Ref(loc, symbol)),
        }
    }
    ParseResult::Ok(runner::Expression::Ref(loc, symbol))
}

fn parse_object(
    loc: types::Location,
    map: &mut HashMap<String, Tokens>,
) -> ParseResult<runner::Expression> {
    println!("parse_object\n\t{loc:?}\n\t{map:?}\n---");
    let mut new_map = HashMap::<String, runner::Expression>::new();
    for key in map.keys() {
        if let Some(ref mut tokens) = map.get(key).cloned() {
            match parse_expression(tokens) {
                ParseResult::Err(loc, err) => return ParseResult::Err(loc, err),
                ParseResult::Ok(expression) => {
                    new_map.insert(key.to_owned(), expression);
                }
            }
        }
    }
    ParseResult::Ok(runner::Expression::Primitive(
        loc,
        runner::Primitive::Object(new_map),
    ))
}

fn parse_array(loc: types::Location, array: &mut Vec<Tokens>) -> ParseResult<runner::Expression> {
    println!("parse_array\n\t{loc:?}\n\t{array:?}\n---");
    let mut result = Vec::<runner::Expression>::new();
    for tokens in array {
        match parse_expression(tokens) {
            ParseResult::Err(loc, err) => return ParseResult::Err(loc, err),
            ParseResult::Ok(expression) => result.push(expression),
        }
    }
    ParseResult::Ok(runner::Expression::Primitive(
        loc,
        runner::Primitive::Array(result),
    ))
}

fn parse_assign(left: &mut Tokens, right: &mut Tokens) -> ParseResult<runner::Expression> {
    println!("parse_assignment\n\t{left:?}\n\t{right:?}\n---");
    match parse_expression(left) {
        ParseResult::Ok(left) => match parse_expression(right) {
            ParseResult::Ok(right) => ParseResult::Ok(runner::Expression::Assignment(
                left.loc(),
                Box::new(left),
                Box::new(right),
            )),
            ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
        },
        ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
    }
}

fn parse_equality(left: &mut Tokens, right: &mut Tokens) -> ParseResult<runner::Expression> {
    println!("parse_equality\n\t{left:?}\n\t{right:?}\n----");
    match parse_expression(left) {
        ParseResult::Ok(left) => match parse_expression(right) {
            ParseResult::Ok(right) => ParseResult::Ok(runner::Expression::Equality(
                left.loc(),
                Box::new(left),
                Box::new(right),
            )),
            ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
        },
        ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
    }
}

fn parse_assert(location: types::Location, tokens: &mut Tokens) -> ParseResult<runner::Expression> {
    println!("parse_assert\n\t{location:?}\n\t{tokens:?}\n---");
    match tokens.take() {
        Some(Token::Pipe(.., ref mut tokens)) => match parse_expression(tokens) {
            ParseResult::Ok(expression) => {
                ParseResult::Ok(runner::Expression::Assert(location, Box::new(expression)))
            }
            ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
        },
        _ => ParseResult::Err(location, "expected pipe".to_string()),
    }
}

fn parse_get(location: types::Location, tokens: &mut Tokens) -> ParseResult<runner::Expression> {
    println!("parse_get\n\t{location:?}\n\t{tokens:?}\n---");
    match tokens.take() {
        Some(Token::Pipe(.., ref mut tokens)) => match parse_expression(tokens) {
            ParseResult::Ok(expression) => {
                ParseResult::Ok(runner::Expression::Get(location, Box::new(expression)))
            }
            ParseResult::Err(loc, err) => ParseResult::Err(loc, err),
        },
        _ => ParseResult::Err(location, "expected pipe".to_string()),
    }
}

#[test]
fn test_assert() {
    let test = Token::Symbol(types::Location("file".to_owned(), 1, 1), "test".to_owned());
    let name = Token::String(types::Location("file".to_owned(), 1, 6), "name".to_owned());
    let assert = Token::Symbol(
        types::Location("file".to_owned(), 2, 4),
        "assert".to_owned(),
    );
    let bool = Token::Symbol(types::Location("file".to_owned(), 2, 13), "true".to_owned());
    let mut tokens = Tokens::new();
    tokens.add(bool);
    let pipe = Token::Pipe(types::Location("file".to_owned(), 2, 10), tokens);

    let mut tokens = Tokens::new();
    tokens.add(assert);
    tokens.add(pipe);
    let block = Token::Block(types::Location("file".to_owned(), 1, 11), vec![tokens]);

    let mut tokens = Tokens::new();
    tokens.add(test);
    tokens.add(name);
    tokens.add(block);

    let result = parse(&mut tokens);
    match result {
        ParseResult::Err(.., err) => assert!(false, "parse failed with {err}"),
        ParseResult::Ok(program) => match program.declarations.0.get(0).unwrap() {
            runner::Declaration::Test(loc, name, block) => {
                match loc {
                    types::Location(file, row, col) => {
                        assert_eq!(*file, "file".to_owned(), "expected file name");
                        assert_eq!(*row, 1, "expected row number");
                        assert_eq!(*col, 1, "expected col number");
                    }
                }
                assert_eq!(*name, "name".to_owned(), "expected test name");
                match &block.loc() {
                    types::Location(file, row, col) => {
                        assert_eq!(*file, "file".to_owned());
                        assert_eq!(*row, 1, "expected row number");
                        assert_eq!(*col, 11, "expected col number");
                    }
                }
                assert_eq!(
                    block.expressions().len(),
                    1,
                    "expected 1 expression in the block"
                );
                match block.expressions().get(0).unwrap() {
                    runner::Expression::Assert(types::Location(file, row, col), expression) => {
                        assert_eq!(*file, "file".to_owned(), "expected file name");
                        assert_eq!(*row, 2, "expected row number");
                        assert_eq!(*col, 4, "expected col number");
                        match **expression {
                            runner::Expression::Primitive(
                                types::Location(.., row, col),
                                runner::Primitive::Boolean(bool),
                            ) => {
                                assert_eq!(row, 2, "expected row number");
                                assert_eq!(col, 13, "expected col number");
                                assert!(bool, "expected true value");
                            }
                            _ => assert!(false, "expected primitive expression"),
                        }
                    }
                    _ => assert!(false, "expected assert expression"),
                }
            }
        },
    }
}

#[test]
fn test_split_on_first_special() {
    let loc = types::Location("file".to_owned(), 1, 2);
    let symbol = Token::Symbol(loc.clone(), "something".to_owned());
    let special = Token::Special(loc.clone(), Special::Assign);
    let string = Token::String(loc.clone(), "other".to_owned());
    let mut tokens = Tokens::new();
    tokens.add(symbol);
    tokens.add(special);
    tokens.add(string);

    if let Some((special, left, right)) = tokens.split_on_first_special() {
        match special {
            Special::Equality => assert!(false, "unexpected equality"),
            _ => (),
        }
        assert_eq!(left.len(), 1, "expected 1 item");
        assert_eq!(right.len(), 1, "expected 1 item");
        match left.peek() {
            Some(Token::Symbol(types::Location(file, row, col), symbol)) => {
                assert_eq!(file, "file".to_owned(), "expected file name");
                assert_eq!(row, 1, "expected row number");
                assert_eq!(col, 2, "expected col number");
                assert_eq!(symbol, "something".to_owned(), "expected symbol value");
            }
            _ => assert!(false, "expected symbol value"),
        }
        match right.peek() {
            Some(Token::String(types::Location(file, row, col), string)) => {
                assert_eq!(file, "file".to_owned(), "expected file name");
                assert_eq!(row, 1, "expected row number");
                assert_eq!(col, 2, "expected col number");
                assert_eq!(string, "other".to_owned(), "expected string value");
            }
            _ => assert!(false, "expected string value"),
        }
    }
}
