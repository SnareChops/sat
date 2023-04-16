use crate::types;

pub fn run(program: types::Program) -> types::RunResult {
    let declarations = program.declarations.iter();
    let mut result = types::RunResult::ok("Running tests".to_string());
    for declaration in declarations {
        match declaration {
            types::Declaration::Test(loc, name, block) => result.merge(run_test(loc, name, block)),
        };
    }
    return result;
}

pub fn run_test(
    location: &types::Location,
    name: &String,
    block: &types::Block,
) -> types::RunResult {
    let types::Block(loc, expressions) = block;
    let mut result = types::RunResult::ok("Running test ".to_owned() + name);
    for expression in expressions.iter() {
        match expression {
            types::Expression::Assert(loc, expression) => result.merge(run_assert(loc, expression)),
            _ => &mut types::RunResult::err(location.clone(), "Unexpected expression".to_string()),
        };
    }
    return result;
}

pub fn run_assert(location: &types::Location, expression: &types::Expression) -> types::RunResult {
    match expression {
        types::Expression::Boolean(loc, value) => {
            if value.to_owned() {
                types::RunResult::ok("Test passed".to_string())
            } else {
                types::RunResult::ok("Expected true, found false".to_string())
            }
        }
        _ => types::RunResult::err(location.clone(), "Unexpected expression".to_string()),
    }
}
