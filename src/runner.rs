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
    pub fn eval_in_scope(&self, scope: &Scope) -> Primitive {
        match self {
            Expression::Primitive(.., primitive) => primitive.clone(),
            Expression::Assert(.., expression) => expression.eval_in_scope(scope),
            Expression::Assignment(.., expression) => expression.eval_in_scope(scope),
            Expression::Get(loc, expression) => {
                match run_get(loc.clone(), *expression.clone(), scope) {
                    RunResult::Ok(primitive) => primitive,
                    RunResult::Err(loc, err) => panic!("{err}"),
                }
            }
            Expression::Equality(.., left, right) => {
                let l = left.eval_in_scope(scope);
                let r = right.eval_in_scope(scope);
                match l {
                    Primitive::Void => match r {
                        Primitive::Void => Primitive::Boolean(true),
                        _ => Primitive::Boolean(false),
                    },
                    Primitive::Boolean(a) => match r {
                        Primitive::Boolean(b) => {
                            if a == b {
                                Primitive::Boolean(true)
                            } else {
                                Primitive::Boolean(false)
                            }
                        }
                        _ => Primitive::Boolean(false),
                    },
                    Primitive::Number(a) => match r {
                        Primitive::Number(b) => {
                            if a == b {
                                Primitive::Boolean(true)
                            } else {
                                Primitive::Boolean(false)
                            }
                        }
                        _ => Primitive::Boolean(false),
                    },
                    Primitive::String(a) => match r {
                        Primitive::String(b) => {
                            if a == b {
                                Primitive::Boolean(true)
                            } else {
                                Primitive::Boolean(false)
                            }
                        }
                        _ => Primitive::Boolean(false),
                    },
                    Primitive::Object(map) => todo!(),
                    Primitive::Array(expressions) => todo!(),
                }
            }
            Expression::Ref(.., name) => match scope.get_var(name) {
                Some(primitive) => primitive.clone(),
                None => Primitive::Void,
            },
        }
    }
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
pub struct Block(pub types::Location, pub Vec<Expression>, pub Scope);
impl Block {
    pub fn new(location: types::Location) -> Block {
        Block(location, Vec::<Expression>::new(), Scope::new())
    }
    pub fn add_expression(&mut self, expression: Expression) {
        self.1.push(expression);
    }
}

#[derive(Debug, Clone)]
pub struct Scope {
    vars: HashMap<String, Primitive>,
}
impl Scope {
    pub fn new() -> Scope {
        Scope {
            vars: HashMap::new(),
        }
    }
    pub fn get_var(&self, name: &String) -> Option<Primitive> {
        if !name.contains('.') {
            self.vars.get(name).cloned()
        } else {
            let mut parts = name.split('.');
            if let Some(part) = parts.next() {
                let var = self.vars.get(part);
                while let Some(part) = parts.next() {
                    match var {
                        Some(Primitive::Object(map)) => match map.get(part) {
                            Some(expression) => return Some(expression.eval_in_scope(self)),
                            None => todo!(),
                        },
                        Some(Primitive::Array(expressions)) => match part.parse::<usize>() {
                            Ok(index) => match expressions.get(index) {
                                Some(expression) => return Some(expression.eval_in_scope(self)),
                                None => todo!(),
                            },
                            Err(err) => todo!(),
                        },
                        _ => todo!(),
                    }
                }
                None
            } else {
                None
            }
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
    let mut asserts = Vec::<(types::Location, bool)>::new();
    for expression in expressions.iter() {
        match run_expression(location.clone(), expression.clone(), scope, &mut asserts) {
            RunResult::Err(loc, err) => return RunResult::Err(loc, err),
            RunResult::Ok(..) => (),
        }
    }
    let mut failed = Vec::<types::Location>::new();
    for (loc, passed) in asserts {
        if !passed {
            failed.push(loc)
        }
    }
    if failed.len() > 0 {
        feedback.add_test_fail(name, failed)
    } else {
        feedback.add_test_pass(name)
    }
    return RunResult::Ok(());
}

fn run_expression(
    location: types::Location,
    expression: Expression,
    scope: &mut Scope,
    asserts: &mut Vec<(types::Location, bool)>,
) -> RunResult<Primitive> {
    println!("run_expression\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");
    match expression {
        Expression::Assert(loc, expression) => run_assert(loc, *expression, scope, asserts),
        Expression::Assignment(loc, left, right) => run_assignment(loc, *left, *right, scope),
        Expression::Get(loc, expression) => run_get(loc, *expression, scope),
        _ => RunResult::Ok(expression.eval_in_scope(scope)),
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
        let primitive = right.eval_in_scope(scope);
        scope.set_var(name, &primitive);
        RunResult::Ok(primitive)
    } else {
        RunResult::Err(left.loc().clone(), "Invalid assignment".to_string())
    }
}

fn run_assert(
    location: types::Location,
    expression: Expression,
    scope: &Scope,
    asserts: &mut Vec<(types::Location, bool)>,
) -> RunResult<Primitive> {
    println!("run_assert\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");
    match expression.eval_in_scope(scope) {
        Primitive::Boolean(value) => {
            asserts.push((location.clone(), value));
            RunResult::Ok(Primitive::Boolean(value))
        }
        _ => RunResult::Err(
            location,
            format!("Expected bool value as expression result for assert"),
        ),
    }
}

fn run_get(
    location: types::Location,
    expression: Expression,
    scope: &Scope,
) -> RunResult<Primitive> {
    println!("run_get\n\t{location:?}\n\t{expression:?}\n\t{scope:?}\n---");
    match expression.eval_in_scope(scope) {
        Primitive::String(url) => match minreq::get(url).with_timeout(5).send() {
            Ok(res) => match res.as_str() {
                Ok(body) => RunResult::Ok(Primitive::String(body.to_string())),
                Err(err) => RunResult::Err(location, err.to_string()),
            },
            Err(err) => RunResult::Err(location, err.to_string()),
        },
        _ => RunResult::Err(
            location,
            format!("Expected string value as expression result for get"),
        ),
    }
}

#[test]
fn test_scope_get_var() {
    let mut scope = Scope {
        vars: HashMap::new(),
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

    match scope.get_var(&"bool".to_string()) {
        Some(Primitive::Boolean(value)) => assert!(value, "expected bool value"),
        _ => assert!(false, "expected bool"),
    }

    scope.vars.insert("obj".to_string(), Primitive::Object(map));

    match scope.get_var(&"obj.hello".to_string()) {
        Some(Primitive::String(string)) => assert_eq!(string, "world", "expected string value"),
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
        RunResult::Ok(..) => match scope.get_var(&"hello".to_string()) {
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
    let scope = Scope::new();
    let mut asserts = Vec::<(types::Location, bool)>::new();
    match run_assert(
        types::Location(file.clone(), row, col),
        assert,
        &scope,
        &mut asserts,
    ) {
        RunResult::Err(..) => assert!(false, "expected run_assert to return Ok"),
        RunResult::Ok(..) => {
            assert_eq!(asserts.len(), 1, "expected 1 run assert");
            assert_eq!(
                asserts.get(0).unwrap().1,
                true,
                "expected assert to have succeeded"
            );
        }
    }
}
