use crate::types;

pub type ParseResult = Result<types::Program, types::ParseError>;
pub type ParseBlockResult = Result<(types::Block, types::Tokens), types::ParseError>;
pub type ParsePartialResult = Result<(types::Declarations, types::Tokens), types::ParseError>;
pub type ParseExpressionResult = Result<(types::Expression, types::Tokens), types::ParseError>;
pub type ParseDeclarationResult = Result<(types::Declaration, types::Tokens), types::ParseError>;

pub fn parse(mut tokens: types::Tokens) -> ParseResult {
    let mut program = types::Program::new();
    while let Some((token, rest)) = tokens.next() {
        // let (token, rest) = ast.split_at(1);
        match parse_declaration(token, rest) {
            Err(err) => return Err(err),
            Ok((parsed, rest)) => {
                program.add_declaration(parsed);
                tokens = rest;
            }
        }
    }
    return Ok(program);
}

fn parse_block(loc: &types::Location, mut tokens: types::Tokens) -> ParseBlockResult {
    let mut expressions = types::Expressions::new();
    while let Some((token, rest)) = tokens.next() {
        match token {
            types::Token::BlockEnd(..) => {
                return Ok((types::Block(loc.clone(), expressions), rest))
            }
            _ => match parse_expression(token, rest) {
                Err(err) => return Err(err),
                Ok((parsed, rest)) => {
                    expressions.add(parsed);
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

fn parse_expression(token: types::Token, rest: types::Tokens) -> ParseExpressionResult {
    match token {
        types::Token::Symbol(loc, symbol) => match symbol.as_str() {
            "assert" => parse_assert(loc, rest),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn parse_declaration(token: types::Token, rest: types::Tokens) -> ParseDeclarationResult {
    match token {
        types::Token::Symbol(loc, symbol) => match symbol.as_str() {
            "test" => parse_test(loc, rest),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn parse_test(loc: types::Location, tokens: types::Tokens) -> ParseDeclarationResult {
    if let Some((types::Token::String(.., name), rest)) = tokens.next() {
        if let Some((types::Token::BlockStart(..), rest)) = rest.next() {
            match parse_block(&loc, rest) {
                Err(err) => Err(err),
                Ok((block, rest)) => Ok((types::Declaration::Test(loc, name, block), rest)),
            }
        } else {
            Err(types::ParseError(loc, "Expected block".to_string()))
        }
    } else {
        Err(types::ParseError(loc, "Expected test name".to_string()))
    }
}

fn parse_assert(assert_loc: types::Location, tokens: types::Tokens) -> ParseExpressionResult {
    if let Some((token, rest)) = tokens.next() {
        match token {
            types::Token::Symbol(loc, s) => match s.as_str() {
                "true" => Ok((
                    types::Expression::Assert(
                        assert_loc,
                        Box::new(types::Expression::Boolean(loc, true)),
                    ),
                    rest,
                )),
                "false" => Ok((
                    types::Expression::Assert(
                        assert_loc,
                        Box::new(types::Expression::Boolean(loc, false)),
                    ),
                    rest,
                )),
                _ => Err(types::ParseError(loc, "Invalid symbol".to_string())),
            },
            _ => Err(types::ParseError(assert_loc, "Unexpected type".to_string())),
        }
    } else {
        Err(types::ParseError(assert_loc, "Expected token".to_string()))
    }
}
