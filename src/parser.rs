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

pub enum ParseExpressionResult {
    Ok(types::Expression),
    Err(types::Location, String),
}

pub fn parse(mut tokens: types::Tokens) -> ParseResult {
    println!("parse {tokens:?}\n---");
    let mut program = types::Program::new();
    while let Some((token, rest)) = tokens.next() {
        // let (token, rest) = ast.split_at(1);
        match parse_declaration(&token, &mut rest.to_owned()) {
            ParseTypeResult::Err(loc, err) => return ParseResult::Err(loc, err),
            ParseTypeResult::Ok(parsed, rest) => {
                program.add_declaration(parsed);
                tokens = rest;
            }
        }
    }
    return ParseResult::Ok(program);
}

fn parse_declaration(
    token: &types::Token,
    rest: &mut types::Tokens,
) -> ParseTypeResult<types::Declaration> {
    println!("parse_declaration\n\t{token:?}\n\t{rest:?}\n---");
    match token {
        types::Token::Symbol(loc, symbol) => match symbol.as_str() {
            "test" => parse_test(loc, rest),
            _ => todo!(),
        },
        _ => todo!(),
    }
}

fn parse_test(
    loc: &types::Location,
    tokens: &mut types::Tokens,
) -> ParseTypeResult<types::Declaration> {
    println!("parse_test\n\t{loc:?}\n\t{tokens:?}\n---");
    if let Some((types::Token::String(.., name), rest)) = tokens.next() {
        todo!()
        // if let Some((types::Token::BlockStart(..), mut rest)) = rest.to_owned().next() {
        //     match parse_block(&loc, &mut rest) {
        //         ParseTypeResult::Err(loc, err) => ParseTypeResult::Err(loc, err),
        //         ParseTypeResult::Ok(block, rest) => ParseTypeResult::<types::Declaration>::Ok(
        //             types::Declaration::Test(loc.clone(), name, block),
        //             rest,
        //         ),
        //     }
        // } else {
        //     ParseTypeResult::Err(loc.clone(), "Expected block".to_string())
        // }
    } else {
        ParseTypeResult::Err(loc.clone(), "Expected test name".to_string())
    }
}

fn parse_block(
    location: &types::Location,
    tokens: &mut types::Tokens,
) -> ParseTypeResult<types::Block> {
    println!("parse_block\n\t{location:?}\n\t{tokens:?}\n---");
    let mut block = types::Block::new(location.clone());
    while let Some((token, rest)) = &tokens.next() {
        println!("token {token:?}");
        todo!()
        // match token {
        //     types::Token::BlockEnd(..) => {
        //         return ParseTypeResult::<types::Block>::Ok(block, rest.clone())
        //     }
        //     _ => {
        //         tokens.push_front(token.clone());
        //         match parse_lines(tokens, &mut block.2) {
        //             ParseTypeResult::Err(loc, err) => return ParseTypeResult::Err(loc, err),
        //             ParseTypeResult::Ok(Some(parsed), rest) => {
        //                 block.add_expression(parsed);
        //                 *tokens = rest.clone();
        //             }
        //             _ => (),
        //         }
        //     }
        // }
    }
    return ParseTypeResult::Err(location.clone(), "Missing end for this block".to_string());
}

fn parse_lines(
    tokens: &mut types::Tokens,
    scope: &mut types::Scope,
) -> ParseTypeResult<Option<types::Expression>> {
    println!("parse_lines\n\t{scope:?}\n\t{tokens:?}\n---");
    match parse_line(tokens, scope) {
        ParseTypeResult::Err(loc, err) => ParseTypeResult::Err(loc, err),
        ParseTypeResult::Ok(line, rest) => {
            if line.len() > 0 {
                match parse_expression(&line, scope) {
                    ParseExpressionResult::Err(loc, err) => ParseTypeResult::Err(loc, err),
                    ParseExpressionResult::Ok(expression) => {
                        ParseTypeResult::Ok(Some(expression), rest)
                    }
                }
            } else {
                ParseTypeResult::Ok(None, rest)
            }
        }
    }
}

fn parse_line(tokens: &mut types::Tokens, scope: &types::Scope) -> ParseTypeResult<types::Line> {
    println!("parse_line\n\t{scope:?}\n\t{tokens:?}\n---");
    let mut line = types::Line::new();
    while let Some((token, rest)) = tokens.next() {
        println!("token {token:?}");
        match token {
            types::Token::Pipe(..) => match parse_line(&mut rest.to_owned(), scope) {
                ParseTypeResult::Err(loc, err) => return ParseTypeResult::Err(loc, err),
                ParseTypeResult::Ok(l, ..) => {
                    line.pipe_line(l);
                    return ParseTypeResult::Ok(line, rest);
                }
            },
            types::Token::Eol(..) => {
                return ParseTypeResult::Ok(line, rest);
            }
            _ => line.add_token(token),
        }
    }
    ParseTypeResult::Err(types::Location::unknown(), "Empty line".to_string())
}

fn parse_expression(line: &types::Line, scope: &mut types::Scope) -> ParseExpressionResult {
    println!("parse_expression\n\t{line:?}\n\t{scope:?}\n---");

    // Look for special expression tokens within the expression
    if let Some(index) = line.find_symbol("==") {
        if let ParseExpressionResult::Ok(left, ..) =
            parse_expression(&types::Line::init(line.0[0..index].to_vec()), scope)
        {
            if let ParseExpressionResult::Ok(right, ..) = parse_expression(
                &types::Line::init(line.0[index + 1..line.0.len()].to_vec()),
                scope,
            ) {
                parse_equality(&left, &right)
            } else {
                ParseExpressionResult::Err(
                    line.loc().clone(),
                    "Expected right side expression for equality".to_string(),
                )
            }
        } else {
            ParseExpressionResult::Err(
                line.loc().clone(),
                "Expected left side expression for equality".to_string(),
            )
        }
    } else if let Some(index) = line.find_symbol("=") {
        if let ParseExpressionResult::Ok(left, ..) =
            parse_expression(&types::Line::init(line.0[0..index].to_vec()), scope)
        {
            if let ParseExpressionResult::Ok(right, ..) = parse_expression(
                &types::Line::init(line.0[index + 1..line.0.len()].to_vec()),
                scope,
            ) {
                parse_assignment(&left, &right, scope)
            } else {
                ParseExpressionResult::Err(
                    line.loc().clone(),
                    "Expected right side expression for assignment".to_string(),
                )
            }
        } else {
            ParseExpressionResult::Err(
                line.loc().clone(),
                "Expected left side expression for assignment".to_string(),
            )
        }
    } else {
        if let Some(types::Token::Symbol(loc, symbol)) = line.0.first() {
            match symbol.as_str() {
                "assert" => {
                    if let Some(line) = &line.1 {
                        if let ParseExpressionResult::Ok(expression) =
                            parse_expression(&line, scope)
                        {
                            parse_assert(&expression)
                        } else {
                            ParseExpressionResult::Err(
                                loc.clone(),
                                "Expecteded expression for assert".to_string(),
                            )
                        }
                    } else {
                        ParseExpressionResult::Err(
                            loc.clone(),
                            "Expected sub line for assert expression".to_string(),
                        )
                    }
                }
                "true" => ParseExpressionResult::Ok(types::Expression::Primitive(
                    loc.clone(),
                    types::Primitive::Boolean(true),
                )),
                "false" => ParseExpressionResult::Ok(types::Expression::Primitive(
                    loc.clone(),
                    types::Primitive::Boolean(false),
                )),
                _ => ParseExpressionResult::Ok(types::Expression::Ref(loc.clone(), symbol.clone())),
            }
        } else if let Some(types::Token::Number(loc, number)) = line.0.first() {
            if let Ok(num) = number.parse::<f32>() {
                ParseExpressionResult::Ok(types::Expression::Primitive(
                    loc.clone(),
                    types::Primitive::Number(num),
                ))
            } else {
                ParseExpressionResult::Err(loc.clone(), "Unhandled number expression".to_string())
            }
        } else if let Some(types::Token::String(loc, value)) = line.0.first() {
            ParseExpressionResult::Ok(types::Expression::Primitive(
                loc.clone(),
                types::Primitive::String(value.clone()),
            ))
        } else {
            ParseExpressionResult::Err(
                types::Location::unknown(),
                "Unknown expression type".to_string(),
            )
        }
    }
}

fn parse_assignment(
    left: &types::Expression,
    right: &types::Expression,
    scope: &mut types::Scope,
) -> ParseExpressionResult {
    println!("parse_assignment\n\t{left:?}\n\t{right:?}\n---");
    if let types::Expression::Ref(.., symbol) = left {
        scope.add_var(&*symbol)
    }
    return ParseExpressionResult::Ok(types::Expression::Assignment(
        left.loc().clone(),
        Box::new(left.clone()),
        Box::new(right.clone()),
    ));
}

fn parse_equality(left: &types::Expression, right: &types::Expression) -> ParseExpressionResult {
    println!("parse_equality\n\t{left:?}\n\t{right:?}\n----");
    return ParseExpressionResult::Ok(types::Expression::Equality(
        left.loc().clone(),
        Box::new(left.clone()),
        Box::new(right.clone()),
    ));
}

fn parse_assert(expression: &types::Expression) -> ParseExpressionResult {
    println!("parse_assert\n\t{expression:?}\n---");
    return ParseExpressionResult::Ok(types::Expression::Assert(
        expression.loc().clone(),
        Box::new(expression.clone()),
    ));
}
