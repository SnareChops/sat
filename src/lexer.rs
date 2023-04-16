use crate::types;

pub fn lex_file(file: String, contents: String) -> types::Tokens {
    let mut tokens = types::Tokens::new();
    let mut token = types::Token::None;
    let mut row = 1;
    let mut col = 0;

    for rune in contents.chars() {
        col += 1;
        // Whitespace
        if rune.is_whitespace() {
            match token {
                types::Token::Symbol(..) | types::Token::Number(..) => {
                    tokens.add(token);
                    token = types::Token::None;
                }
                types::Token::String(.., ref mut s) => s.push(rune),
                _ => (),
            }
            if rune == '\n' {
                col = 0;
                row += 1;
            }
        // Dot
        } else if rune == '.' {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),

                types::Token::Number(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }

        // Strings
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
                types::Token::None => {
                    token = types::Token::String(
                        types::Location(file.to_owned(), row, col),
                        "".to_string(),
                    )
                }
                _ => (),
            }
        // Block Start
        } else if rune == '{' {
            match token {
                types::Token::None => (),
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token.clone());
                }
            };
            tokens.add(types::Token::BlockStart(types::Location(
                file.to_owned(),
                row,
                col,
            )));
        // Block End
        } else if rune == '}' {
            match token {
                types::Token::None => (),
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token.clone());
                }
            }
            tokens.add(types::Token::BlockEnd(types::Location(
                file.to_owned(),
                row,
                col,
            )));
        // Letters
        } else if rune.is_alphabetic() {
            match token {
                types::Token::None => {
                    token = types::Token::Symbol(
                        types::Location(file.to_owned(), row, col),
                        rune.to_string(),
                    )
                }
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
                types::Token::None => {
                    token = types::Token::Number(
                        types::Location(file.to_owned(), row, col),
                        rune.to_string(),
                    )
                }
                _ => (),
            }
        } else {
            match token {
                types::Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        }
    }
    match token {
        types::Token::None => (),
        _ => {
            tokens.add(token);
        }
    }
    return tokens;
}
