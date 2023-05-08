use std::{collections::HashMap, slice::IterMut};

use crate::types;

pub enum RunResult<T> {
    Ok(T),
    Err(types::Location, String),
}

#[derive(Debug)]
pub struct Program {
    pub declarations: Declarations,
}
impl Program {
    pub fn new() -> Program {
        Program {
            declarations: Declarations::new(),
        }
    }
    pub fn add_declaration(&mut self, declaration: Declaration) {
        self.declarations.0.push(declaration);
    }
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Test(types::Location, String, Block),
}
#[derive(Debug, Clone)]
pub struct Declarations(pub Vec<Declaration>);
impl Declarations {
    pub fn new() -> Declarations {
        return Declarations(vec![]);
    }
    pub fn iter_mut(&mut self) -> IterMut<Declaration> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Clone)]
pub enum Expression {
    // Basics
    Primitive(types::Location, Primitive),
    Assert(types::Location, Box<Expression>),
    Assignment(types::Location, Box<Expression>, Box<Expression>),
    Equality(types::Location, Box<Expression>, Box<Expression>),
    Ref(types::Location, String),
    // Http
    Get(types::Location, Box<Expression>),
}
impl Expression {
    pub fn loc(&self) -> types::Location {
        match self {
            Expression::Assert(loc, ..) => loc.clone(),
            Expression::Assignment(loc, ..) => loc.clone(),
            Expression::Equality(loc, ..) => loc.clone(),
            Expression::Primitive(loc, ..) => loc.clone(),
            Expression::Ref(loc, ..) => loc.clone(),
            Expression::Get(loc, ..) => loc.clone(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Void,
    Boolean(bool),
    Number(f32),
    String(String),
    Object(HashMap<String, Expression>),
    Array(Vec<Expression>),
}

#[derive(Debug, Clone)]
pub struct Block(types::Location, Vec<Expression>, Scope);
impl Block {
    pub fn new(location: types::Location) -> Block {
        Block(location, Vec::<Expression>::new(), Scope::new())
    }
    pub fn add_expression(&mut self, expression: Expression) {
        self.1.push(expression);
    }
    pub fn loc(&self) -> types::Location {
        self.0.clone()
    }
    pub fn expressions(&self) -> Vec<Expression> {
        self.1.clone()
    }
    pub fn scope(&self) -> Scope {
        self.2.clone()
    }
}

#[derive(Debug, Clone)]
struct Assert(types::Location, bool);
impl Assert {
    fn loc(&self) -> types::Location {
        self.0.clone()
    }
    fn passed(&self) -> bool {
        self.1
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, Primitive>,
    asserts: Vec<Assert>,
}
impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
            asserts: Vec::new(),
        }
    }
    pub fn set_var(&mut self, name: &String, value: &Primitive) {
        self.vars.insert(name.to_string(), value.clone());
    }
}

#[derive(Clone)]
pub struct Feedback(Vec<String>);
impl Feedback {
    pub fn new() -> Feedback {
        Feedback(vec![])
    }
    pub fn add_test_pass(&mut self, name: String) {
        self.0.push(format!("\t✓\t{name}"))
    }
    pub fn add_test_fail(&mut self, name: String, failed: Vec<types::Location>) {
        self.0.push(format!(
            "\t❌\t{name}\n\t\t- assert {}",
            failed
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<String>>()
                .join("\n\t\t- assert ")
        ))
    }
    pub fn to_string(&self) -> String {
        self.0.join("\n")
    }
}

pub fn run(mut program: Program) -> RunResult<Feedback> {
    println!("run {program:?}\n---");
    let declarations = program.declarations.iter_mut();
    let ref mut feedback = Feedback::new();
    for declaration in declarations {
        match declaration {
            Declaration::Test(loc, name, ref mut block) => {
                match run_test(feedback, loc.clone(), name.clone(), block) {
                    RunResult::Err(loc, err) => return RunResult::Err(loc, err),
                    RunResult::Ok(..) => (),
                }
            }
        };
    }
    RunResult::Ok(feedback.clone())
}

fn run_test(
    feedback: &mut Feedback,
    location: types::Location,
    name: String,
    block: &mut Block,
) -> RunResult<()> {
    println!("run_test\n\t{location:?}\n\t{name}\n\t{block:?}\n---");
    let Block(.., expressions, scope) = block;
    for expression in expressions.iter() {
        match run_expression(expression.clone(), scope) {
            RunResult::Err(loc, err) => return RunResult::Err(loc, err),
            RunResult::Ok(..) => (),
        }
    }
    let mut failed = Vec::<types::Location>::new();
    for ref ass in &scope.asserts {
        if !ass.passed() {
            failed.push(ass.loc())
        }
    }
    if failed.len() > 0 {
        feedback.add_test_fail(name, failed)
    } else {
        feedback.add_test_pass(name)
    }
    return RunResult::Ok(());
}

fn run_expression(expression: Expression, scope: &mut Scope) -> RunResult<Primitive> {
    println!("run_expression\n\t{expression:?}\n\t{scope:?}\n---");
    match expression {
        Expression::Assert(loc, expression) => run_assert(loc, *expression, scope),
        Expression::Assignment(loc, left, right) => run_assignment(loc, *left, *right, scope),
        Expression::Get(loc, expression) => run_get(loc, *expression, scope),
        Expression::Primitive(.., primitive) => RunResult::Ok(primitive),
        Expression::Equality(loc, left, right) => run_equality(loc, *left, *right, scope),
        Expression::Ref(loc, name) => run_ref(loc, name, scope),
    }
}

fn run_ref(location: types::Location, name: String, scope: &mut Scope) -> RunResult<Primitive> {
    println!("run_ref\n\t{location:?}\n\t{name}\n\t{scope:?}\n---");
    if !name.contains('.') {
        match scope.vars.get(&name) {
            Some(primitive) => RunResult::Ok(primitive.clone()),
            None => todo!(),
        }
    } else {
        let mut parts = name.split('.');
        if let Some(part) = parts.next() {
            while let Some(next) = parts.next() {
                return match scope.vars.get(part) {
                    Some(Primitive::Object(map)) => match map.get(next) {
                        Some(expression) => match run_expression(expression.clone(), scope) {
                            RunResult::Ok(primitive) => RunResult::Ok(primitive),
                            RunResult::Err(loc, err) => RunResult::Err(loc, err),
                        },
                        None => todo!(),
                    },
                    Some(Primitive::Array(expressions)) => match next.parse::<usize>() {
                        Ok(index) => match expressions.get(index) {
                            Some(expression) => match run_expression(expression.clone(), scope) {
                                RunResult::Ok(primitive) => RunResult::Ok(primitive),
                                RunResult::Err(loc, err) => RunResult::Err(loc, err),
                            },
                            None => todo!(),
                        },
                        Err(err) => todo!(),
                    },
                    _ => todo!(),
                };
            }
            todo!()
        } else {
            todo!()
        }
    }
}

fn run_assignment(
    location: types::Location,
    left: Expression,
    right: Expression,
    scope: &mut Scope,
) -> RunResult<Primitive> {
    println!("run_assignment\n\t{location:?}\n\t{left:?}\n\t{right:?}\n\t{scope:?}\n---");
    if let Expression::Ref(.., ref name) = left {
        match run_expression(right, scope) {
            RunResult::Ok(primitive) => {
                scope.set_var(name, &primitive);
                RunResult::Ok(primitive)
            }
            RunResult::Err(loc, err) => RunResult::Err(loc, err),
        }
    } else {
        RunResult::Err(left.loc().clone(), "Invalid assignment".to_string())
    }
}

fn run_equality(
    location: types::Location,
    left: Expression,
    right: Expression,
    scope: &mut Scope,
) -> RunResult<Primitive> {
    println!("run_equality\n\t{location:?}\n\t{left:?}\n\t{right:?}\n\t{scope:?}\n---");
    match run_expression(left, scope) {
        RunResult::Err(loc, err) => RunResult::Err(loc, err),
        RunResult::Ok(l) => match run_expression(right, scope) {
            RunResult::Err(loc, err) => RunResult::Err(loc, err),
            RunResult::Ok(r) => match l {
                Primitive::Void => match r {
                    Primitive::Void => RunResult::Ok(Primitive::Boolean(true)),
                    _ => RunResult::Ok(Primitive::Boolean(false)),
                },
                Primitive::Boolean(a) => match r {
                    Primitive::Boolean(b) => {
                        if a == b {
                            RunResult::Ok(Primitive::Boolean(true))
                        } else {
                            RunResult::Ok(Primitive::Boolean(false))
                        }
                    }
                    _ => RunResult::Ok(Primitive::Boolean(false)),
                },
                Primitive::Number(a) => match r {
                    Primitive::Number(b) => {
                        if a == b {
                            RunResult::Ok(Primitive::Boolean(true))
                        } else {
                            RunResult::Ok(Primitive::Boolean(false))
                        }
                    }
                    _ => RunResult::Ok(Primitive::Boolean(false)),
                },
                Primitive::String(a) => match r {
                    Primitive::String(b) => {
                        if a == b {
                            RunResult::Ok(Primitive::Boolean(true))
                        } else {
                            RunResult::Ok(Primitive::Boolean(false))
                        }
                    }
                    _ => RunResult::Ok(Primitive::Boolean(false)),
                },
                Primitive::Object(map) => todo!(),
                Primitive::Array(expressions) => todo!(),
            },
        },
    }
}

fn run_assert(
    location: types::Location,
    expression: Expression,
    scope: &mut Scope,
) -> RunResult<Primitive> {
    println!("run_assert\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");

    match run_expression(expression, scope) {
        RunResult::Ok(Primitive::Boolean(value)) => {
            scope.asserts.push(Assert(location.clone(), value));
            RunResult::Ok(Primitive::Boolean(value))
        }
        RunResult::Err(loc, err) => RunResult::Err(loc, err),
        _ => RunResult::Err(
            location,
            format!("Expected bool value as expression result for assert"),
        ),
    }
}

fn run_get(
    location: types::Location,
    expression: Expression,
    scope: &mut Scope,
) -> RunResult<Primitive> {
    println!("run_get\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");
    match run_expression(expression, scope) {
        RunResult::Ok(Primitive::String(url)) => match minreq::get(url).with_timeout(5).send() {
            Ok(res) => match res.as_str() {
                Ok(body) => RunResult::Ok(Primitive::String(body.to_string())),
                Err(err) => RunResult::Err(location, err.to_string()),
            },
            Err(err) => RunResult::Err(location, err.to_string()),
        },
        RunResult::Err(loc, err) => RunResult::Err(loc, err),
        _ => RunResult::Err(
            location,
            format!("Expected string value as expression result for get"),
        ),
    }
}

#[test]
fn test_ref() {
    let location = types::Location("file".to_string(), 4, 3);
    let mut scope = Scope {
        vars: HashMap::new(),
        asserts: Vec::new(),
    };
    scope
        .vars
        .insert("bool".to_string(), Primitive::Boolean(true));
    let loc = types::Location("file".to_string(), 1, 1);
    let mut map = HashMap::<String, Expression>::new();
    map.insert(
        "hello".to_string(),
        Expression::Primitive(loc, Primitive::String("world".to_string())),
    );

    match run_ref(location.clone(), "bool".to_string(), &mut scope) {
        RunResult::Ok(Primitive::Boolean(value)) => assert!(value, "expected bool value"),
        _ => assert!(false, "expected bool"),
    }

    scope.vars.insert("obj".to_string(), Primitive::Object(map));

    match run_ref(location.clone(), "obj.hello".to_string(), &mut scope) {
        RunResult::Ok(Primitive::String(string)) => {
            assert_eq!(string, "world", "expected string value")
        }
        _ => assert!(false, "expected string"),
    }
}

#[test]
fn test_assignment() {
    let file = "file".to_string();
    let row = 4;
    let col = 7;
    let loc = types::Location(file.to_owned(), row, col);
    let left = Expression::Ref(loc.clone(), "hello".to_string());
    let right = Expression::Primitive(loc.clone(), Primitive::String("world".to_string()));
    let mut scope = Scope::new();
    match run_assignment(loc, left, right, &mut scope) {
        RunResult::Err(..) => assert!(false, "expected run_assignment to return Ok"),
        RunResult::Ok(..) => match scope.vars.get(&"hello".to_string()) {
            Some(Primitive::String(value)) => {
                assert_eq!(
                    value.to_owned(),
                    "world".to_string(),
                    "expected value of variable"
                )
            }
            _ => assert!(false, "expected variable in scope"),
        },
    }
}
#[test]
fn test_assert() {
    let file = "file".to_string();
    let row = 3;
    let col = 5;
    let loc = types::Location(file.to_owned(), row, col);
    let primitive = Expression::Primitive(loc.clone(), Primitive::Number(2.3));
    let equality = Expression::Equality(
        loc.clone(),
        Box::new(primitive.clone()),
        Box::new(primitive.clone()),
    );
    let assert = Expression::Assert(
        types::Location(file.to_owned(), row, col),
        Box::new(equality),
    );
    let mut scope = Scope::new();
    match run_assert(types::Location(file.clone(), row, col), assert, &mut scope) {
        RunResult::Err(..) => assert!(false, "expected run_assert to return Ok"),
        RunResult::Ok(..) => {
            assert_eq!(scope.asserts.len(), 1, "expected 1 run assert");
            assert_eq!(
                scope.asserts.get(0).unwrap().1,
                true,
                "expected assert to have succeeded"
            );
        }
    }
}
