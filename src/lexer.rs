use crate::parser;
use crate::types;
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
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
    fn peek(&self) -> Option<char> {
        self.contents.get(0).copied()
    }
    fn take(&mut self) -> Option<char> {
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
    fn loc(&self) -> types::Location {
        if self.was_newline {
            types::Location(self.file.to_owned(), self.row + 1, 1)
        } else {
            types::Location(self.file.to_owned(), self.row, self.col + 1)
        }
    }
}

pub fn lex_file(file: String, contents: String) -> parser::Tokens {
    println!("lex_file\n\t{file}\n\t{contents}\n---");
    let mut reader = Reader::new(file, contents);
    let (tokens, ..) = lex_until(&mut reader, None);
    return tokens;
}

fn lex_until(mut reader: &mut Reader, until: Option<Vec<char>>) -> (parser::Tokens, Option<char>) {
    println!("lex_until\n\t{until:?}\n\t{reader:?}\n---");
    let mut tokens = parser::Tokens::new();
    while let Some(rune) = reader.peek() {
        if let Some(ref vec) = until {
            if vec.contains(&rune) {
                return (tokens, Some(rune));
            }
        }
        // \n
        if rune == '\n' || rune == ';' {
            tokens.add(parser::Token::Eol(reader.loc()));
            reader.take();
        // :
        } else if rune == ':' {
            reader.take();
            if let Some(token) = lex_pipe(reader) {
                tokens.add(token);
            }
        // =
        } else if rune == '=' {
            let loc = reader.loc();
            reader.take();
            if let Some(c) = reader.peek() {
                if c == '=' {
                    tokens.add(parser::Token::Special(loc, parser::Special::Equality));
                    reader.take();
                    continue;
                }
            }
            tokens.add(parser::Token::Special(loc, parser::Special::Assign));
        // .
        } else if rune == '.' {
            reader.take();
            tokens.add(parser::Token::Dot(reader.loc()));
        // ,
        } else if rune == ',' {
            reader.take();
            tokens.add(parser::Token::Comma(reader.loc()));
        // "
        } else if rune == '"' {
            if let Some(token) = lex_string(&mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        // {
        } else if rune == '{' {
            match tokens.last() {
                Some(
                    parser::Token::Special(.., parser::Special::Assign)
                    | parser::Token::Special(.., parser::Special::Equality)
                    | parser::Token::Pipe(..),
                ) => {
                    if let Some(token) = lex_object(&mut reader) {
                        tokens.add(token)
                    } else {
                        todo!()
                    }
                }
                _ => {
                    if let Some(token) = lex_block(&mut reader) {
                        tokens.add(token)
                    } else {
                        todo!()
                    }
                }
            }
        // [
        } else if rune == '[' {
            if let Some(token) = lex_array(&mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        // Letters
        } else if rune.is_alphabetic() {
            if let Some(token) = lex_symbol(&mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        // Numbers
        } else if rune.is_numeric() {
            if let Some(token) = lex_number(&mut reader) {
                tokens.add(token)
            } else {
                todo!()
            }
        } else {
            reader.take();
        }
    }
    return (tokens, None);
}

fn lex_string(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_string\n\t{reader:?}\n---");
    let loc = reader.loc();
    let mut result = "".to_string();
    reader.take(); // Consume the '"'
    loop {
        let rune = reader.peek();
        match rune {
            Some('\\') => {
                reader.take();
                let c = reader.peek().unwrap().clone();
                reader.take();
                result.push(c)
            }
            Some('"') => {
                reader.take();
                return Some(parser::Token::String(loc, result));
            }
            None => return Some(parser::Token::String(loc, result)),
            Some(val) => {
                reader.take();
                result.push(val)
            }
        }
    }
}

fn lex_symbol(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_symbol\n\t{reader:?}\n---");
    let loc = reader.loc();
    let mut symbol = "".to_string();
    loop {
        match reader.peek().clone() {
            Some(rune) => {
                if !rune.is_alphanumeric() {
                    return Some(parser::Token::Symbol(loc, symbol));
                }
                reader.take();
                symbol.push(rune);
            }
            None => return Some(parser::Token::Symbol(loc, symbol)),
        }
    }
}

fn lex_number(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_number\n\t{reader:?}\n---");
    let loc = reader.loc();
    let mut number = "".to_string();
    loop {
        let next = reader.peek();
        match next {
            Some(rune) => {
                if rune.is_numeric() {
                    reader.take();
                    number.push(rune)
                } else if rune == '_' {
                    reader.take();
                    continue;
                } else if rune == '.' {
                    reader.take();
                    number.push(rune)
                } else {
                    return Some(parser::Token::Number(loc, number));
                }
            }
            None => return Some(parser::Token::Number(loc, number)),
        }
    }
}

fn lex_pipe(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_pipe\n\t{reader:?}\n---");
    let loc = reader.loc();
    let (tokens, ..) = lex_until(reader, Some(vec![';', '\n']));
    Some(parser::Token::Pipe(loc, tokens))
}

fn lex_object(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_object\n\t{reader:?}\n---");
    let loc = reader.loc();
    let mut object = HashMap::<String, parser::Tokens>::new();
    let mut prop: Option<String> = None;
    reader.take();
    loop {
        if let Some(rune) = reader.peek() {
            if rune == '"' {
                if let Some(parser::Token::String(.., name)) = lex_string(reader) {
                    prop = Some(name);
                } else {
                    todo!()
                }
            } else if rune.is_alphanumeric() {
                if let Some(parser::Token::Symbol(.., name)) = lex_symbol(reader) {
                    prop = Some(name);
                } else {
                    todo!()
                }
            } else if rune == ':' {
                reader.take();
                let (tokens, ended_with) = lex_until(reader, Some(vec!['}', ',']));
                if let Some(ref key) = prop {
                    object.insert(key.to_string(), tokens);
                    if let Some(char) = ended_with {
                        reader.take();
                        if char == '}' {
                            return Some(parser::Token::Object(loc, object));
                        }
                    }
                } else {
                    todo!()
                }
            } else if rune.is_whitespace() {
                reader.take();
            }
        } else {
            todo!()
        }
    }
}

fn lex_array(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_array\n\t{reader:?}\n---");
    let loc = reader.loc();
    let mut array = Vec::<parser::Tokens>::new();
    reader.take();
    loop {
        let (tokens, ended_with) = lex_until(reader, Some(vec![',', ']']));
        array.push(tokens);
        if let Some(char) = ended_with {
            if char == ']' {
                return Some(parser::Token::Array(loc, array));
            }
            if char == ',' {
                reader.take();
            }
        } else {
            todo!()
        }
    }
}

fn lex_block(reader: &mut Reader) -> Option<parser::Token> {
    println!("lex_block\n\t{reader:?}\n---");
    let loc = reader.loc();
    reader.take();
    let mut block = Vec::<parser::Tokens>::new();
    loop {
        let (tokens, ended_with) = lex_until(reader, Some(vec!['\n', ';', '}']));
        println!("tokens {tokens:?}");
        if let Some(ending) = ended_with {
            reader.take();
            if tokens.len() > 0 {
                block.push(tokens);
            }
            if ending == '}' {
                return Some(parser::Token::Block(loc, block));
            }
        } else {
            todo!()
        }
    }
}

#[test]
fn test_block() {
    let mut result = lex_file("file".to_string(), " test { contents }".to_string());
    assert!(result.len() == 2, "expected 2 tokens");
    match result.take() {
        Some(parser::Token::Symbol(types::Location(file, row, col), value)) => {
            assert!(*file == "file".to_string(), "expected file name");
            assert!(row == 1, "expected row number");
            assert!(col == 2, "expected col number");
            assert!(value == "test", "expected symbol value");
        }
        _ => assert!(false, "expected symbol"),
    }
    match result.take() {
        Some(parser::Token::Block(types::Location(file, row, col), ref mut lines)) => {
            assert!(*file == "file".to_string(), "expected file name");
            assert!(row == 1, "expected row number");
            assert!(col == 7, "expected col number");
            assert!(lines.get(0).unwrap().len() == 1, "expected 1 token");
            match lines.get(0).unwrap().clone().take() {
                Some(parser::Token::Symbol(types::Location(file, row, col), value)) => {
                    assert!(*file == "file".to_string(), "expected file name");
                    assert!(row == 1, "expected row number");
                    assert!(col == 9, "expected col number");
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
    let mut result = lex_file("file".to_string(), " simple".to_string());
    assert!(result.len() == 1, "expected 1 token");
    match result.take() {
        Some(parser::Token::Symbol(types::Location(file, row, col), symbol)) => {
            assert_eq!(file, "file", "expected file name");
            assert_eq!(row, 1, "expected row number");
            assert_eq!(col, 2, "expected col number");
            assert_eq!(symbol, "simple", "expected symbol value");
        }
        _ => assert!(false, "expected symbol"),
    }
}

#[test]
fn test_string() {
    let mut result = lex_file(
        "file".to_string(),
        " \"some 123 \\\"string\\\" \n34\"".to_string(),
    );
    println!("{result:?}");
    assert_eq!(result.len(), 1, "expected 1 token");
    match result.take() {
        Some(parser::Token::String(types::Location(file, row, col), value)) => {
            assert_eq!(file, "file", "expected file name");
            assert_eq!(row, 1, "expected row number");
            assert_eq!(col, 2, "expected col number");
            assert_eq!(value, "some 123 \"string\" \n34", "expected string value");
        }
        _ => assert!(false, "expected string"),
    }
}

#[test]
fn test_number() {
    let file_name = "file";

    let mut result = lex_file(file_name.to_string(), "123.456".to_string());
    assert!(result.len() == 1, "expected 1 token");
    match result.take() {
        Some(parser::Token::Number(types::Location(file, row, col), value)) => {
            assert!(*file == file_name.to_string(), "expected file name");
            assert!(row == 1, "expected row number");
            assert!(col == 1, "expected col number");
            assert!(value == "123.456", "expected number value");
        }
        _ => assert!(false, "expected number"),
    }

    let mut result = lex_file(file_name.to_string(), " 1_000_000 ".to_string());
    assert!(result.len() == 1, "expected 1 token");
    match result.take() {
        Some(parser::Token::Number(types::Location(file, row, col), value)) => {
            assert!(*file == file_name.to_string(), "expected file name");
            assert!(row == 1, "expected row number");
            assert!(col == 2, "expected col number");
            assert!(value == "1000000", "expected number value");
        }
        _ => assert!(false, "expected number"),
    }
}

#[test]
fn test_object() {
    let file_name = "file";
    let mut result = lex_file(
        file_name.to_string(),
        "test {\nobj = {hello: \"world\", 1: 23.0, \"other\": true == true}\na=b\n}".to_string(),
    );
    assert!(result.len() == 2, "expected 2 tokens");
    match result.take() {
        Some(parser::Token::Symbol(types::Location(file, row, col), symbol)) => {
            assert_eq!(file, file_name.to_string(), "expected file name");
            assert_eq!(row, 1, "expected row number");
            assert_eq!(col, 1, "expected col number");
            assert_eq!(symbol, "test", "expected symbol value");
        }
        _ => assert!(false, "expected symbol"),
    }
    match result.take() {
        Some(parser::Token::Block(types::Location(file, row, col), ref mut tokens)) => {
            assert_eq!(file, file_name.to_string(), "expected file name");
            assert_eq!(row, 1, "expected row number");
            assert_eq!(col, 6, "expected col number");
            assert_eq!(tokens.len(), 2, "expected 2 sets of tokens");
            match tokens.get_mut(0) {
                Some(tokens) => {
                    match tokens.take() {
                        Some(parser::Token::Symbol(types::Location(file, row, col), symbol)) => {
                            assert_eq!(*file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 2, "expected row number");
                            assert_eq!(col, 1, "expected col number");
                            assert_eq!(symbol, "obj", "expected symbol value");
                        }
                        _ => assert!(false, "expected symbol"),
                    }
                    match tokens.take() {
                        Some(parser::Token::Special(
                            types::Location(file, row, col),
                            parser::Special::Assign,
                        )) => {
                            assert_eq!(*file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 2, "expected row number");
                            assert_eq!(col, 5, "expected col number");
                        }
                        _ => assert!(false, "expected special"),
                    }
                    match tokens.take() {
                        Some(parser::Token::Object(types::Location(file, row, col), map)) => {
                            println!("map {map:?}");
                            assert_eq!(*file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 2, "expected row number");
                            assert_eq!(col, 7, "expected col number");
                            assert_eq!(map["hello"].len(), 1, "expected 1 token for \"hello\"");
                            assert_eq!(map["1"].len(), 1, "expected 1 token for \"1\"");
                            assert_eq!(map["other"].len(), 3, "expected 3 tokens for \"other\"");
                            let mut hello = map.get("hello").unwrap().clone();
                            match hello.take() {
                                Some(parser::Token::String(
                                    types::Location(file, row, col),
                                    value,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 2, "expected row number");
                                    assert_eq!(col, 15, "expected col number");
                                    assert_eq!(value, "world", "expected symbol value");
                                }
                                _ => assert!(false, "expected symbol"),
                            }
                            let mut one = map.get("1").unwrap().clone();
                            match one.take() {
                                Some(parser::Token::Number(
                                    types::Location(file, row, col),
                                    number,
                                )) => {
                                    assert_eq!(file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 2, "expected row number");
                                    assert_eq!(col, 27, "expected col number");
                                    assert_eq!(number, "23.0", "expected number value");
                                }
                                _ => assert!(false, "expected number"),
                            }
                            let mut other = map.get("other").unwrap().clone();
                            match other.take() {
                                Some(parser::Token::Symbol(
                                    types::Location(file, row, col),
                                    symbol,
                                )) => {
                                    assert_eq!(file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 2, "expected row number");
                                    assert_eq!(col, 42, "expected col number");
                                    assert_eq!(symbol, "true", "expected symbol value");
                                }
                                _ => assert!(false, "expected symbol"),
                            }
                            match other.take() {
                                Some(parser::Token::Special(
                                    types::Location(file, row, col),
                                    parser::Special::Equality,
                                )) => {
                                    assert_eq!(file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 2, "expected row number");
                                    assert_eq!(col, 47, "expected col number");
                                }
                                _ => assert!(false, "expected equality"),
                            }
                            match other.take() {
                                Some(parser::Token::Symbol(
                                    types::Location(file, row, col),
                                    symbol,
                                )) => {
                                    assert_eq!(file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 2, "expected row number");
                                    assert_eq!(col, 50, "expected col number");
                                    assert_eq!(symbol, "true", "expected symbol value")
                                }
                                _ => assert!(false, "expected symbol"),
                            }
                        }
                        _ => assert!(false, "expeted object"),
                    }
                }
                _ => assert!(false, "expected tokens"),
            }
            match tokens.get_mut(1) {
                Some(tokens) => {
                    match tokens.take() {
                        Some(parser::Token::Symbol(types::Location(file, row, col), symbol)) => {
                            assert_eq!(file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 3, "expected row number");
                            assert_eq!(col, 1, "expected col number");
                            assert_eq!(symbol, "a", "expected symbol value");
                        }
                        _ => assert!(false, "expected symbol"),
                    }
                    match tokens.take() {
                        Some(parser::Token::Special(
                            types::Location(file, row, col),
                            parser::Special::Assign,
                        )) => {
                            assert_eq!(file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 3, "expected row number");
                            assert_eq!(col, 2, "expected col number");
                        }
                        _ => assert!(false, "expected assign"),
                    }
                    match tokens.take() {
                        Some(parser::Token::Symbol(types::Location(file, row, col), symbol)) => {
                            assert_eq!(file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 3, "expected row number");
                            assert_eq!(col, 3, "expected col number");
                            assert_eq!(symbol, "b", "expected symbol value");
                        }
                        _ => assert!(false, "expected symbol"),
                    }
                }
                _ => assert!(false, "expected set of tokens"),
            }
        }
        _ => assert!(false, "expected block"),
    }
}

#[test]
fn test_array() {
    let file_name = "file";
    let mut result = lex_file(
        file_name.to_string(),
        "array = [\"haha\", 45.6, false, true == true]".to_string(),
    );
    assert!(result.len() == 3);
    match result.take() {
        Some(parser::Token::Symbol(types::Location(file, row, col), symbol)) => {
            assert_eq!(*file, file_name.to_string(), "expected file name");
            assert_eq!(row, 1, "expected row number");
            assert_eq!(col, 1, "expected col number");
            assert_eq!(symbol, "array", "expected symbol value");
            match result.take() {
                Some(parser::Token::Special(
                    types::Location(file, row, col),
                    parser::Special::Assign,
                )) => {
                    assert_eq!(*file, file_name.to_string(), "expected file name");
                    assert_eq!(row, 1, "expected row number");
                    assert_eq!(col, 7, "expected col number");
                    match result.take() {
                        Some(parser::Token::Array(types::Location(file, row, col), items)) => {
                            assert_eq!(*file, file_name.to_string(), "expected file name");
                            assert_eq!(row, 1, "expected row number");
                            assert_eq!(col, 9, "expected col number");
                            assert_eq!(items.len(), 4, "expected 4 items in the array");
                            let mut haha = items.get(0).unwrap().clone();
                            match haha.take() {
                                Some(parser::Token::String(
                                    types::Location(file, row, col),
                                    value,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 1, "expected row number");
                                    assert_eq!(col, 10, "expected col number");
                                    assert_eq!(value, "haha", "expected string value");
                                }
                                _ => assert!(false, "expected string"),
                            }
                            let mut num = items.get(1).unwrap().clone();
                            match num.take() {
                                Some(parser::Token::Number(
                                    types::Location(file, row, col),
                                    value,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 1, "expected row number");
                                    assert_eq!(col, 18, "expected col number");
                                    assert_eq!(value, "45.6", "expected number value");
                                }
                                _ => assert!(false, "expected number"),
                            }
                            let mut fal = items.get(2).unwrap().clone();
                            match fal.take() {
                                Some(parser::Token::Symbol(
                                    types::Location(file, row, col),
                                    value,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 1, "expected row number");
                                    assert_eq!(col, 24, "expected col number");
                                    assert_eq!(value, "false", "expected symbol value");
                                }
                                _ => assert!(false, "expected symbol"),
                            }
                            let mut cond = items.get(3).unwrap().clone();
                            match cond.take() {
                                Some(parser::Token::Symbol(
                                    types::Location(file, row, col),
                                    value,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 1, "expected row number");
                                    assert_eq!(col, 31, "expected col number");
                                    assert_eq!(value, "true", "expected symbol value");
                                }
                                _ => assert!(false, "expected symbol"),
                            }
                            match cond.take() {
                                Some(parser::Token::Special(
                                    types::Location(file, row, col),
                                    parser::Special::Equality,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 1, "expected row number");
                                    assert_eq!(col, 36, "expected col number");
                                }
                                _ => assert!(false, "expected equality"),
                            }
                            match cond.take() {
                                Some(parser::Token::Symbol(
                                    types::Location(file, row, col),
                                    value,
                                )) => {
                                    assert_eq!(*file, file_name.to_string(), "expected file name");
                                    assert_eq!(row, 1, "expected row number");
                                    assert_eq!(col, 39, "expected col number");
                                    assert_eq!(value, "true", "expected symbol value");
                                }
                                _ => assert!(false, "expected symbol"),
                            }
                        }
                        _ => assert!(false, "expected array"),
                    }
                }
                _ => assert!(false, "expected assign"),
            }
        }
        _ => assert!(false, "expected symbol"),
    }
}

#[test]
fn test_block_starting_with_newline() {
    let mut reader = Reader::new("file".to_owned(), "\n    assert : true\n}".to_owned());
    let mut result = lex_block(&mut reader);
    match result {
        Some(parser::Token::Block(types::Location(..), ref mut tokens)) => {
            assert_eq!(tokens.len(), 2, "expected 2 items");
            assert_eq!(tokens.get(0).unwrap().len(), 1, "expected 1 item");
            assert_eq!(tokens.get(1).unwrap().len(), 4, "expected 4 items");
            match tokens.get_mut(0).unwrap().take() {
                Some(parser::Token::Eol(..)) => (),
                _ => assert!(false, "expected eol"),
            }
            match tokens.get_mut(1).unwrap().take() {
                Some(parser::Token::Symbol(.., symbol)) => {
                    assert_eq!(symbol, "assert".to_owned(), "expected symbol value");
                }
                _ => assert!(false, "expected symbol"),
            }
            match tokens.get_mut(1).unwrap().take() {
                Some(parser::Token::Pipe(..)) => (),
                _ => assert!(false, "expected pipe"),
            }
            match tokens.get_mut(1).unwrap().take() {
                Some(parser::Token::Symbol(.., symbol)) => {
                    assert_eq!(symbol, "true".to_owned(), "expected symbol value")
                }
                _ => assert!(false, "expected symbol"),
            }
            match tokens.get_mut(1).unwrap().take() {
                Some(parser::Token::Eol(..)) => (),
                _ => assert!(false, "expected eol"),
            }
        }
        _ => assert!(false, "expected block"),
    }
}
