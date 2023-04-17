use crate::types;

pub enum ParseResult {
    Ok(types::Program),
    Err(types::Location, String),
}
impl ParseResult {
    pub fn message(&self) -> String {
        match self {
            ParseResult::Err(loc, err) => format!("ParseError: {}: {}", loc.to_string(), err),
            _ => "".to_string(),
        }
    }
}
pub enum ParseTypeResult<T> {
    Ok(T, types::Tokens),
    Err(types::Location, String),
}

pub fn parse(mut tokens: types::Tokens) -> ParseResult {
    println!("parse {tokens:?}");
    let mut program = types::Program::new();
    while let Some((token, rest)) = tokens.next() {
        // let (token, rest) = ast.split_at(1);
        match parse_declaration(token, rest) {
            ParseTypeResult::Err(loc, err) => return ParseResult::Err(loc, err),
            ParseTypeResult::Ok(parsed, rest) => {
                program.add_declaration(parsed);
                tokens = rest;
            }
        }
    }
    return ParseResult::Ok(program);
}

fn parse_block(
    location: &types::Location,
    mut tokens: types::Tokens,
) -> ParseTypeResult<types::Block> {
    println!("parse_block {location:?} {tokens:?}");
    let mut block = types::Block::new(location.clone());
    while let Some((token, rest)) = tokens.next() {
        match token {
            types::Token::BlockEnd(..) => return ParseTypeResult::<types::Block>::Ok(block, rest),
            _ => match parse_expression(token, rest, &mut block) {
                ParseTypeResult::Err(loc, err) => return ParseTypeResult::Err(loc, err),
                ParseTypeResult::Ok(parsed, rest) => {
                    block.add_expression(parsed);
                    tokens = rest;
                }
            },
        }
    }
    return ParseTypeResult::Err(location.clone(), "Missing end for this block".to_string());
}

fn parse_expression(
    token: types::Token,
    rest: types::Tokens,
    block: &mut types::Block,
) -> ParseTypeResult<types::Expression> {
    println!("parse_expression {token:?} {block:?} {rest:?}");
    match token {
        types::Token::Symbol(loc, symbol) => match symbol.as_str() {
            "assert" => parse_assert(loc, rest, block),
            "true" => ParseTypeResult::Ok(
                types::Expression::Primitive(loc, types::Primitive::Boolean(true)),
                rest,
            ),
            "false" => ParseTypeResult::Ok(
                types::Expression::Primitive(loc, types::Primitive::Boolean(false)),
                rest,
            ),
            _ => {
                if block.has_var(&symbol) {
                    return ParseTypeResult::Ok(types::Expression::Ref(loc, symbol.clone()), rest);
                } else {
                    parse_assignment(loc, &symbol, rest, block)
                }
            }
        },
        _ => todo!(),
    }
}

fn parse_assignment(
    location: types::Location,
    symbol: &String,
    rest: types::Tokens,
    block: &mut types::Block,
) -> ParseTypeResult<types::Expression> {
    println!("parse_assignment: {location:?} {symbol} {block:?} {rest:?}");
    if let Some((token, rest)) = rest.next() {
        match token {
            types::Token::Symbol(loc, value) => {
                println!("token: {}", value);
                if value != "=" {
                    return ParseTypeResult::Err(loc, "Expected '='".to_string());
                }
                if let Some((token, rest)) = rest.next() {
                    match parse_expression(token, rest, block) {
                        ParseTypeResult::Err(loc, err) => ParseTypeResult::Err(loc, err),
                        ParseTypeResult::Ok(expression, rest) => {
                            block.add_var(&symbol);
                            return ParseTypeResult::Ok(
                                types::Expression::Assignment(
                                    loc,
                                    symbol.to_string(),
                                    Box::new(expression),
                                ),
                                rest,
                            );
                        }
                    }
                } else {
                    ParseTypeResult::Err(loc, "Unexpected EOF".to_string())
                }
            }
            _ => ParseTypeResult::Err(location, "Invalid assignment syntax".to_string()),
        }
    } else {
        ParseTypeResult::Err(location, "Invalid expression found".to_string())
    }
}

fn parse_declaration(
    token: types::Token,
    rest: types::Tokens,
) -> ParseTypeResult<types::Declaration> {
    println!("parse_declaration {token:?} {rest:?}");
    match token {
        types::Token::Symbol(loc, symbol) => match symbol.as_str() {
            "test" => parse_test(loc, rest),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn parse_test(loc: types::Location, tokens: types::Tokens) -> ParseTypeResult<types::Declaration> {
    println!("parse_test {loc:?} {tokens:?}");
    if let Some((types::Token::String(.., name), rest)) = tokens.next() {
        if let Some((types::Token::BlockStart(..), rest)) = rest.next() {
            match parse_block(&loc, rest) {
                ParseTypeResult::Err(loc, err) => ParseTypeResult::Err(loc, err),
                ParseTypeResult::Ok(block, rest) => ParseTypeResult::<types::Declaration>::Ok(
                    types::Declaration::Test(loc, name, block),
                    rest,
                ),
            }
        } else {
            ParseTypeResult::Err(loc, "Expected block".to_string())
        }
    } else {
        ParseTypeResult::Err(loc, "Expected test name".to_string())
    }
}

fn parse_assert(
    assert_loc: types::Location,
    tokens: types::Tokens,
    block: &types::Block,
) -> ParseTypeResult<types::Expression> {
    println!("parse_assert {assert_loc:?} {tokens:?}");
    if let Some((token, rest)) = tokens.next() {
        match token {
            types::Token::Symbol(loc, s) => match s.as_str() {
                "true" => ParseTypeResult::<types::Expression>::Ok(
                    types::Expression::Assert(
                        assert_loc,
                        Box::new(types::Expression::Primitive(
                            loc,
                            types::Primitive::Boolean(true),
                        )),
                    ),
                    rest,
                ),
                "false" => ParseTypeResult::<types::Expression>::Ok(
                    types::Expression::Assert(
                        assert_loc,
                        Box::new(types::Expression::Primitive(
                            loc,
                            types::Primitive::Boolean(false),
                        )),
                    ),
                    rest,
                ),
                _ => {
                    if block.has_var(&s) {
                        return match block.get_var(&s) {
                            Some(..) => ParseTypeResult::Ok(
                                types::Expression::Assert(
                                    assert_loc,
                                    Box::new(types::Expression::Ref(loc, s)),
                                ),
                                rest,
                            ),
                            _ => ParseTypeResult::Err(loc, "Invalid symbol found".to_string()),
                        };
                    }
                    ParseTypeResult::Err(loc, "Invalid symbol".to_string())
                }
            },
            _ => ParseTypeResult::Err(assert_loc, "Unexpected type".to_string()),
        }
    } else {
        ParseTypeResult::Err(assert_loc, "Expected token".to_string())
    }
}
