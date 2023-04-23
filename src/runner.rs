use crate::types;

pub fn run(mut program: types::Program) -> types::RunResult {
    println!("run {program:?}\n---");
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
    println!("run_test\n\t{location:?}\n\t{name}\n\t{block:?}\n---");
    let types::Block(loc, expressions, scope) = block;
    let mut result = types::RunResult::ok("Running test ".to_owned() + name);
    let mut asserts = Vec::<(types::Location, bool)>::new();
    for expression in expressions.iter() {
        let r = run_expression(location, expression, scope, &mut asserts);
        match r {
            types::RunResult::Ok(..) => result.merge(&r),
            types::RunResult::Err(..) => return r,
        }
    }
    let mut failed = Vec::<String>::new();
    for (loc, passed) in asserts {
        if !passed {
            failed.push(format!("Failed assert: {}", loc.to_string()))
        }
    }
    if failed.len() > 0 {
        result.merge(&types::RunResult::Ok(failed))
    } else {
        result.merge(&types::RunResult::ok("Test passed".to_string()))
    }
    return result;
}

pub fn run_expression(
    location: &types::Location,
    expression: &types::Expression,
    scope: &mut types::Scope,
    asserts: &mut Vec<(types::Location, bool)>,
) -> types::RunResult {
    println!("run_expression\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");
    match expression {
        types::Expression::Assert(loc, expression) => run_assert(loc, &expression, scope, asserts),
        types::Expression::Assignment(loc, left, right) => run_assignment(loc, left, right, scope),
        _ => types::RunResult::Ok(vec![]),
    }
}

pub fn run_assignment(
    location: &types::Location,
    left: &types::Expression,
    right: &types::Expression,
    scope: &mut types::Scope,
) -> types::RunResult {
    println!("run_assignment\n\t{location:?}\n\t{left:?}\n\t{right:?}\n\t{scope:?}\n---");
    if let types::Expression::Ref(.., name) = left {
        scope.set_var(name, &right.eval_in_scope(scope));
        types::RunResult::Ok(vec![])
    } else {
        types::RunResult::Err(left.loc().clone(), "Invalid assignment".to_string())
    }
}

pub fn run_assert(
    location: &types::Location,
    expression: &types::Expression,
    scope: &types::Scope,
    asserts: &mut Vec<(types::Location, bool)>,
) -> types::RunResult {
    println!("run_assert\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");
    match expression.eval_in_scope(scope) {
        types::Primitive::Boolean(value) => {
            asserts.push((location.clone(), value));
            return types::RunResult::Ok(vec![]);
        }
        _ => types::RunResult::ok("Expected boolean value or expression".to_string()),
    }
}
