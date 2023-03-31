#[derive(Clone, Debug)]
pub enum Token {
    None,
    Symbol(Location, String),
    String(Location, String),
    Number(Location, String),
    BlockStart(Location),
    BlockEnd(Location),
}

#[derive(Debug, Clone)]
pub struct Location(String, i32, i32);

impl Location {
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
pub struct Tokens(Vec<Token>);
impl Tokens {
    pub fn new() -> Tokens {
        Tokens(Vec::<Token>::new())
    }
    pub fn next(&self) -> Option<(Token, Tokens)> {
        if let Some((first, rest)) = self.0.split_first() {
            Some((first.clone(), Tokens(rest.to_vec())))
        } else {
            None
        }
    }

    pub fn add(&mut self, token: Token) -> &mut Tokens {
        self.0.push(token);
        return self;
    }
}

pub fn lex_file(file: String, contents: String) -> Tokens {
    let mut tokens = Tokens::new();
    let mut token = Token::None;
    let mut row = 1;
    let mut col = 0;

    for rune in contents.chars() {
        col += 1;
        // Whitespace
        if rune.is_whitespace() {
            match token {
                Token::Symbol(..) | Token::Number(..) => {
                    tokens.add(token);
                    token = Token::None;
                }
                Token::String(.., ref mut s) => s.push(rune),
                _ => (),
            }
            if rune == '\n' {
                col = 0;
                row += 1;
            }
        // Dot
        } else if rune == '.' {
            match token {
                Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),

                Token::Number(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }

        // Strings
        } else if rune == '"' {
            match token {
                Token::String(.., ref mut s) => {
                    if s.ends_with("\\") {
                        let (str, _) = s.split_at(s.len() - 1);
                        *s = str.to_owned() + "\""
                    } else {
                        tokens.add(token.clone());
                        token = Token::None;
                    }
                }
                Token::None => {
                    token = Token::String(Location(file.to_owned(), row, col), "".to_string())
                }
                _ => (),
            }
        // Block Start
        } else if rune == '{' {
            match token {
                Token::None => (),
                Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token.clone());
                }
            };
            tokens.add(Token::BlockStart(Location(file.to_owned(), row, col)));
        // Block End
        } else if rune == '}' {
            match token {
                Token::None => (),
                Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => {
                    tokens.add(token.clone());
                }
            }
            tokens.add(Token::BlockEnd(Location(file.to_owned(), row, col)));
        // Letters
        } else if rune.is_alphabetic() {
            match token {
                Token::None => {
                    token = Token::Symbol(Location(file.to_owned(), row, col), rune.to_string())
                }
                Token::Symbol(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        // Numbers
        } else if rune.is_numeric() {
            match token {
                Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Token::Symbol(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Token::Number(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                Token::None => {
                    token = Token::Number(Location(file.to_owned(), row, col), rune.to_string())
                }
                _ => (),
            }
        } else {
            match token {
                Token::String(.., ref mut s) => *s = s.to_owned() + &rune.to_string(),
                _ => (),
            }
        }
    }
    match token {
        Token::None => (),
        _ => {
            tokens.add(token);
        }
    }
    return tokens;
}
