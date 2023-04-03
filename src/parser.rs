use crate::types;

pub type ParseResult = Result<types::Tokens<types::AST>, types::ParseError>;
pub type ParseBlockResult = Result<(types::Block, types::Tokens<types::Token>), types::ParseError>;
pub type ParsePartialResult =
    Result<(types::Tokens<types::AST>, types::Tokens<types::Token>), types::ParseError>;

pub fn parse(mut tokens: types::Tokens<types::Token>) -> ParseResult {
    // let mut ast = ast;
    let mut ast = types::Tokens::<types::AST>::new();
    while let Some((token, rest)) = tokens.next() {
        // let (token, rest) = ast.split_at(1);
        match parse_one(token, rest) {
            Err(err) => return Err(err),
            Ok((parsed, rest)) => {
                ast.concat(parsed);
                tokens = rest;
            }
        }
    }
    return Ok(ast);
}

fn parse_block(loc: &types::Location, mut tokens: types::Tokens<types::Token>) -> ParseBlockResult {
    let mut ast = types::Tokens::<types::AST>::new();
    while let Some((token, rest)) = tokens.next() {
        match token {
            types::Token::BlockEnd(..) => return Ok((types::Block(loc.clone(), ast), rest)),
            _ => match parse_one(token, rest) {
                Err(err) => return Err(err),
                Ok((parsed, rest)) => {
                    ast.concat(parsed);
                    tokens = rest;
                }
            },
        }
    }
    return Err(types::ParseError(
        loc.clone(),
        "Missing end for this block".to_string(),
    ));
}

fn parse_one(token: types::Token, rest: types::Tokens<types::Token>) -> ParsePartialResult {
    match token {
        types::Token::Symbol(loc, val) => parse_symbol(loc, val.to_string(), rest),
        types::Token::String(loc, val) => todo!(),
        types::Token::Number(loc, val) => todo!(),
        types::Token::BlockStart(loc) => todo!(),
        types::Token::BlockEnd(loc) => todo!(),
        types::Token::None => todo!(),
    }
}

fn parse_symbol(
    loc: types::Location,
    val: String,
    tokens: types::Tokens<types::Token>,
) -> ParsePartialResult {
    match val.as_str() {
        "test" => parse_test(loc, tokens),
        _ => Err(types::ParseError(loc, "Invalid symbol".to_string())),
    }
}

fn parse_test(loc: types::Location, tokens: types::Tokens<types::Token>) -> ParsePartialResult {
    if let Some((types::Token::String(.., name), rest)) = tokens.next() {
        if let Some((types::Token::BlockStart(..), rest)) = rest.next() {
            match parse_block(&loc, rest) {
                Err(err) => Err(err),
                Ok((block, rest)) => {
                    let mut ast = types::Tokens::<types::AST>::new();
                    ast.add(types::AST::Test(loc, name, block));
                    Ok((ast, rest))
                }
            }
        } else {
            Err(types::ParseError(loc, "Expected block".to_string()))
        }
    } else {
        Err(types::ParseError(loc, "Expected test name".to_string()))
    }
}
