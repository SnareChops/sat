use crate::lexer;

#[derive(Debug)]
pub struct ParseError(lexer::Location, String);
impl ParseError {
    pub fn message(&self) -> String {
        format!("ParseError: {}: {}", self.0.to_string(), self.1,)
    }
}

#[derive(Debug, Clone)]
pub enum Token {
    Test(lexer::Location, String, Block),
}

#[derive(Debug, Clone)]
pub struct Block(lexer::Location, Tokens);

#[derive(Debug, Clone)]
pub struct Tokens(Vec<Token>);
impl Tokens {
    pub fn new() -> Tokens {
        Tokens(Vec::<Token>::new())
    }
    pub fn init(tokens: &[Token]) -> Tokens {
        let mut new = Tokens(Vec::<Token>::new());
        for token in tokens {
            new.add(token.clone());
        }
        return new;
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

    pub fn concat(&mut self, tokens: Tokens) -> &mut Tokens {
        self.0 = [self.0.clone(), tokens.0].concat();
        return self;
    }
}

pub type ParseResult = Result<Tokens, ParseError>;
pub type ParseBlockResult = Result<(Block, lexer::Tokens), ParseError>;
pub type ParsePartialResult = Result<(Tokens, lexer::Tokens), ParseError>;

pub fn parse(mut ast: lexer::Tokens) -> ParseResult {
    // let mut ast = ast;
    let mut tokens = Tokens::new();
    while let Some((token, rest)) = ast.next() {
        // let (token, rest) = ast.split_at(1);
        match parse_one(token, rest) {
            Err(err) => return Err(err),
            Ok((parsed, rest)) => {
                tokens.concat(parsed);
                ast = rest;
            }
        }
    }
    return Ok(tokens);
}

fn parse_block(loc: &lexer::Location, mut ast: lexer::Tokens) -> ParseBlockResult {
    let mut tokens = Tokens::new();
    while let Some((token, rest)) = ast.next() {
        match token {
            lexer::Token::BlockEnd(..) => return Ok((Block(loc.clone(), tokens), rest)),
            _ => match parse_one(token, rest) {
                Err(err) => return Err(err),
                Ok((parsed, rest)) => {
                    tokens.concat(parsed);
                    ast = rest;
                }
            },
        }
    }
    return Err(ParseError(
        loc.clone(),
        "Missing end for this block".to_string(),
    ));
}

fn parse_one(token: lexer::Token, rest: lexer::Tokens) -> ParsePartialResult {
    match token {
        lexer::Token::Symbol(loc, val) => parse_symbol(loc, val.to_string(), rest),
        lexer::Token::String(loc, val) => todo!(),
        lexer::Token::Number(loc, val) => todo!(),
        lexer::Token::BlockStart(loc) => todo!(),
        lexer::Token::BlockEnd(loc) => todo!(),
        lexer::Token::None => todo!(),
    }
}

fn parse_symbol(loc: lexer::Location, val: String, ast: lexer::Tokens) -> ParsePartialResult {
    match val.as_str() {
        "test" => parse_test(loc, ast),
        _ => Err(ParseError(loc, "Invalid symbol".to_string())),
    }
}

fn parse_test(loc: lexer::Location, ast: lexer::Tokens) -> ParsePartialResult {
    if let Some((lexer::Token::String(.., name), rest)) = ast.next() {
        if let Some((lexer::Token::BlockStart(..), rest)) = rest.next() {
            match parse_block(&loc, rest) {
                Err(err) => Err(err),
                Ok((block, rest)) => {
                    let mut tokens = Tokens::new();
                    tokens.add(Token::Test(loc, name, block));
                    Ok((tokens, rest))
                }
            }
        } else {
            Err(ParseError(loc, "Expected block".to_string()))
        }
    } else {
        Err(ParseError(loc, "Expected test name".to_string()))
    }
}
