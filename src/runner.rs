use crate::types;

pub fn run(mut program: types::Program) -> types::RunResult {
    println!("run {program:?}");
    let declarations = program.declarations.iter_mut();
    let mut result = types::RunResult::ok("Running tests".to_string());
    for declaration in declarations {
        match declaration {
            types::Declaration::Test(loc, name, ref mut block) => {
                let r = run_test(loc, name, block);
                match r {
                    types::RunResult::Ok(..) => result.merge(&r),
                    _ => (),
                }
                return r;
            }
        };
    }
    return result;
}

pub fn run_test(
    location: &types::Location,
    name: &String,
    block: &mut types::Block,
) -> types::RunResult {
    println!("run_test {location:?} {name} {block:?}");
    let types::Block(loc, expressions, scope) = block;
    let mut result = types::RunResult::ok("Running test ".to_owned() + name);
    for expression in expressions.iter() {
        let r = run_expression(location, expression, scope);
        match r {
            types::RunResult::Ok(..) => result.merge(&r),
            types::RunResult::Err(..) => return r,
        }
    }
    return result;
}

pub fn run_expression(
    location: &types::Location,
    expression: &types::Expression,
    scope: &mut types::Scope,
) -> types::RunResult {
    println!("run_expression {location:?} {expression:?} {scope:?}");
    match expression {
        types::Expression::Assert(loc, expression) => run_assert(loc, &expression, scope),
        types::Expression::Assignment(loc, name, expression) => {
            run_assignment(loc, name, expression, scope)
        }
        _ => types::RunResult::Ok(vec![]),
    }
}

pub fn run_assignment(
    location: &types::Location,
    name: &String,
    expression: &types::Expression,
    scope: &mut types::Scope,
) -> types::RunResult {
    println!("run_assignment {location:?} {name:?} {expression:?} {scope:?}");
    scope.set_var(name, &expression.eval_in_scope(scope));
    types::RunResult::Ok(vec![])
}

pub fn run_assert(
    location: &types::Location,
    expression: &types::Expression,
    scope: &types::Scope,
) -> types::RunResult {
    println!("run_assert {location:?} {expression:?} {scope:?}");
    match expression.eval_in_scope(scope) {
        types::Primitive::Boolean(value) => {
            if value {
                types::RunResult::ok("Test passed".to_string())
            } else {
                types::RunResult::ok("Expected true, found false".to_string())
            }
        }
        _ => types::RunResult::ok("Expected boolean value or expression".to_string()),
    }
}