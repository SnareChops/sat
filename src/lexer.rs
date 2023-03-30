#[derive(Clone, Debug)]
pub enum Tokens {
    None,
    Symbol(i32, i32, String),
    String(i32, i32, String),
    Number(i32, i32, String),
    BlockStart(i32, i32),
    BlockEnd(i32, i32),
}

pub fn lex_file(contents: String) -> Vec<Tokens> {
    let mut tokens: Vec<Tokens> = Vec::new();
    let mut token: Tokens = Tokens::None;
    let mut row = 1;
    let mut col = 0;

    for rune in contents.chars() {
        col += 1;
        // Whitespace
        if rune.is_whitespace() {
            match token {
                Tokens::Symbol(_, _, _) | Tokens::Number(_, _, _) => {
                    tokens.push(token.clone());
                    token = Tokens::None;
                }
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
            if rune == '\n' {
                col = 0;
                row += 1;
            }
        // Dot
        } else if rune == '.' {
            match token {
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Tokens::Number(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }

        // Strings
        } else if rune == '"' {
            match token {
                Tokens::String(_, _, ref mut s) => {
                    if s.ends_with("\\") {
                        let (str, _) = s.split_at(s.len() - 1);
                        *s = str.to_owned() + "\""
                    } else {
                        tokens.push(token.clone());
                        token = Tokens::None;
                    }
                }
                Tokens::None => token = Tokens::String(row, col, "".to_string()),
                _ => (),
            }
        // Block Start
        } else if rune == '{' {
            match token {
                Tokens::None => (),
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => tokens.push(token.clone()),
            }
            tokens.push(Tokens::BlockStart(row, col))
        // Block End
        } else if rune == '}' {
            match token {
                Tokens::None => (),
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => tokens.push(token.clone()),
            }
            tokens.push(Tokens::BlockEnd(row, col))
        // Letters
        } else if rune.is_alphabetic() {
            match token {
                Tokens::None => token = Tokens::Symbol(row, col, rune.to_string()),
                Tokens::Symbol(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        // Numbers
        } else if rune.is_numeric() {
            match token {
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Tokens::Symbol(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Tokens::Number(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Tokens::None => token = Tokens::Number(row, col, rune.to_string()),
                _ => (),
            }
        } else {
            match token {
                Tokens::String(_, _, ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        }
    }
    match token {
        Tokens::None => (),
        _ => tokens.push(token),
    }
    return tokens;
}
