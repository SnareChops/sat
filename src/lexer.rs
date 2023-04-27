use crate::types;
use std::collections::VecDeque;

struct Reader {
    file: String,
    row: usize,
    col: usize,
    was_newline: bool,
    contents: VecDeque<char>,
}
impl Reader {
    fn new(file: String, contents: String) -> Reader {
        Reader {
            file: file,
            row: 1,
            col: 0,
            was_newline: false,
            contents: VecDeque::<char>::from_iter(contents.chars()),
        }
    }
    fn next(&mut self) -> Option<char> {
        let char = self.contents.pop_front();
        if self.was_newline {
            self.row += 1;
            self.col = 0;
            self.was_newline = false;
        }
        if let Some(c) = char {
            if c == '\n' {
                self.was_newline = true;
            }
        }
        self.col += 1;
        return char;
    }
    fn peek(&self) -> Option<&char> {
        self.contents.get(0)
    }
    fn loc(&self) -> types::Location {
        types::Location(self.file.to_owned(), self.row, self.col)
    }
}

pub fn lex_file(file: String, contents: String) -> types::Tokens {
    let mut reader = Reader::new(file, contents);
    lex_until(&mut reader, None)
}

fn lex_until(mut reader: &mut Reader, until: Option<char>) -> types::Tokens {
    let mut tokens = types::Tokens::new();
    while let Some(rune) = reader.next() {
        if let Some(c) = until {
            if rune == c {
                return tokens;
            }
        }
        // \n
        if rune == '\n' {
            tokens.add_eol(reader.loc())
        // :
        } else if rune == ':' {
            tokens.add_pipe(reader.loc())
        } else if rune == ';' {
            tokens.add_eol(reader.loc())
        // =
        } else if rune == '=' {
            if let Some(c) = reader.peek() {
                if *c == '=' {
                    tokens.add_symbol(reader.loc(), "==".to_string());
                    continue;
                }
            }
            tokens.add_symbol(reader.loc(), "=".to_string());
        // "
        } else if rune == '"' {
            if let Some(token) = lex_string(&mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        // {
        } else if rune == '{' {
            if let Some(token) = lex_block(&mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        // Letters
        } else if rune.is_alphabetic() {
            if let Some(token) = lex_symbol(rune, reader.loc(), &mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        // Numbers
        } else if rune.is_numeric() {
            if let Some(token) = lex_number(rune, reader.loc(), &mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        }
    }
    return tokens;
}

fn lex_string(reader: &mut Reader) -> Option<types::Token> {
    let location = reader.loc();
    let mut result = "".to_string();
    while let Some(rune) = reader.next() {
        match rune {
            '\\' => {
                if let Some(c) = reader.next() {
                    result.push(c)
                }
            }
            '"' => return Some(types::Token::String(location, result)),
            _ => result.push(rune),
        }
    }
    return None;
}

fn lex_block(reader: &mut Reader) -> Option<types::Token> {
    let loc = reader.loc();
    Some(types::Token::Block(loc, lex_until(reader, Some('}'))))
}

fn lex_symbol(start: char, loc: types::Location, reader: &mut Reader) -> Option<types::Token> {
    let mut symbol = "".to_string();
    symbol.push(start);
    loop {
        let next = reader.next();
        match next {
            Some(rune) => {
                if rune.is_whitespace() {
                    return Some(types::Token::Symbol(loc, symbol));
                }
                symbol.push(rune);
            }
            None => return Some(types::Token::Symbol(loc, symbol)),
        }
    }
}

fn lex_number(start: char, loc: types::Location, reader: &mut Reader) -> Option<types::Token> {
    let mut number = "".to_string();
    number.push(start);
    loop {
        let next = reader.next();
        match next {
            Some(rune) => {
                if rune.is_whitespace() {
                    return Some(types::Token::Number(loc, number));
                }
                if rune.is_numeric() {
                    number.push(rune)
                } else if rune == '_' {
                    continue;
                } else if rune == '.' {
                    number.push(rune)
                } else {
                    todo!()
                }
            }
            None => return Some(types::Token::Number(loc, number)),
        }
    }
}

#[test]
fn test_block() {
    let result = lex_file("file".to_string(), " test { contents }".to_string());
    println!("result {result:?}");
    let tokens: Vec<&types::Token> = result.iter().collect();
    assert!(tokens.len() == 2, "expected 2 tokens");
    match tokens.get(0) {
        Some(types::Token::Symbol(types::Location(file, row, col), value)) => {
            assert!(*file == "file".to_string(), "expected file name");
            assert!(*row == 1, "expected row number");
            assert!(*col == 2, "expected col number");
            assert!(value == "test", "expected symbol value");
        }
        _ => assert!(false, "expected symbol"),
    }
    match tokens.get(1) {
        Some(types::Token::Block(types::Location(file, row, col), sub_tokens)) => {
            assert!(*file == "file".to_string(), "expected file name");
            assert!(*row == 1, "expected row number");
            assert!(*col == 7, "expected col number");
            let sub: Vec<&types::Token> = sub_tokens.iter().collect();
            assert!(sub.len() == 1, "expected 1 token");
            match sub.get(0) {
                Some(types::Token::Symbol(types::Location(file, row, col), value)) => {
                    assert!(*file == "file".to_string(), "expected file name");
                    assert!(*row == 1, "expected row number");
                    assert!(*col == 9, "expected col number");
                    assert!(value == "contents", "expected symbol value");
                }
                _ => assert!(false, "expected symbol"),
            }
        }
        _ => assert!(false, "expected sub block"),
    }
}

#[test]
fn test_symbol() {
    let result = lex_file("file".to_string(), " simple".to_string());
    assert!(result.len() == 1, "expected 1 token");
    match result.iter().next() {
        Some(types::Token::Symbol(types::Location(file, row, col), symbol)) => {
            assert!(file == "file", "expected file name");
            assert!(*row == 1, "expected row number");
            assert!(*col == 2, "expected col number");
            assert!(symbol == "simple", "expected symbol value");
        }
        _ => assert!(false, "expected symbol"),
    }
}

#[test]
fn test_string() {
    let result = lex_file(
        "file".to_string(),
        " \"some 123 \\\"string\\\" \n34\"".to_string(),
    );
    assert!(result.len() == 1, "expected 1 token");
    match result.iter().next() {
        Some(types::Token::String(types::Location(file, row, col), value)) => {
            assert!(file == "file", "expected file name");
            assert!(*row == 1, "expected row number");
            assert!(*col == 2, "expected col number");
            assert!(value == "some 123 \"string\" \n34", "expected string value");
        }
        _ => assert!(false, "expected string"),
    }
}

#[test]
fn test_number() {
    let file_name = "file";

    let result = lex_file(file_name.to_string(), "123.456".to_string());
    assert!(result.len() == 1, "expected 1 token");
    match result.iter().next() {
        Some(types::Token::Number(types::Location(file, row, col), value)) => {
            assert!(*file == file_name.to_string(), "expected file name");
            assert!(*row == 1, "expected row number");
            assert!(*col == 1, "expected col number");
            assert!(value == "123.456", "expected number value");
        }
        _ => assert!(false, "expected number"),
    }

    let result = lex_file(file_name.to_string(), " 1_000_000 ".to_string());
    assert!(result.len() == 1, "expected 1 token");
    match result.iter().next() {
        Some(types::Token::Number(types::Location(file, row, col), value)) => {
            assert!(*file == file_name.to_string(), "expected file name");
            assert!(*row == 1, "expected row number");
            assert!(*col == 2, "expected col number");
            assert!(value == "1000000", "expected number value");
        }
        _ => assert!(false, "expected number"),
    }
}
