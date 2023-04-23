use crate::types;

pub fn lex_file(file: String, contents: String) -> types::Tokens {
    let mut tokens = types::Tokens::new();
    let mut token = types::Token::None;
    let mut row = 1;
    let mut col = 0;
    let mut chars = contents.chars().peekable();

    while let Some(rune) = chars.next() {
        col += 1;
        let location = types::Location(file.to_owned(), row, col);
        // \n
        if rune == '\n' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token);
                    token = types::Token::None;
                    tokens.add_eol(location);
                }
            }
            col = 0;
            row += 1;
        // Whitespace
        } else if rune.is_whitespace() {
            match token {
                types::Token::Symbol(..) | types::Token::Number(..) => {
                    tokens.add(token);
                    token = types::Token::None;
                }
                types::Token::String(.., ref mut s) => s.push(rune),
                _ => (),
            }
        // :
        } else if rune == ':' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token);
                    token = types::Token::None;
                    tokens.add_pipe(location)
                }
            }
        // Dot
        } else if rune == '.' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                types::Token::Number(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        // =
        } else if rune == '=' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token);
                    token = types::Token::None;
                    if let Some(val) = chars.peek() {
                        if *val == '=' {
                            tokens.add_symbol(location, "==".to_string());
                            chars.next(); // Advance the iterator
                        } else {
                            tokens.add_symbol(location, "=".to_string())
                        }
                    } else {
                        tokens.add_symbol(location, "=".to_string())
                    }
                }
            }
        // "
        } else if rune == '"' {
            match token {
                types::Token::String(.., ref mut s) => {
                    if s.ends_with("\\") {
                        let (str, _) = s.split_at(s.len() - 1);
                        *s = str.to_owned() + "\""
                    } else {
                        tokens.add(token.clone());
                        token = types::Token::None;
                    }
                }
                types::Token::None => token = types::Token::String(location, "".to_string()),
                _ => (),
            }
        // {
        } else if rune == '{' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token.clone());
                    token = types::Token::None;
                    tokens.add_block_start(location);
                }
            };
        // }
        } else if rune == '}' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token.clone());
                    token = types::Token::None;
                    tokens.add_block_end(location);
                }
            }
        // Letters
        } else if rune.is_alphabetic() {
            match token {
                types::Token::None => token = types::Token::Symbol(location, rune.to_string()),
                types::Token::Symbol(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        // Numbers
        } else if rune.is_numeric() {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                types::Token::Symbol(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                types::Token::Number(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                types::Token::None => token = types::Token::Number(location, rune.to_string()),
                _ => (),
            }
        } else {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        }
    }
    tokens.add(token);
    return tokens;
}
