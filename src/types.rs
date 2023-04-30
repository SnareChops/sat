use std::{
    collections::{HashMap, VecDeque},
    slice,
};

pub const ASSIGNMENT: &str = "=";
pub const EQUALITY: &str = "==";

#[derive(Clone, Debug)]
pub enum Token {
    None,
    Symbol(Location, String),
    String(Location, String),
    Number(Location, String),
    Block(Location, Tokens),
    Object(Location, HashMap<String, Tokens>),
    Assign(Location),
    Equality(Location),
    Pipe(Location),
    Eol(Location),
}
impl Token {
    pub fn loc(&self) -> Option<&Location> {
        match self {
            Token::Symbol(loc, ..) => Some(loc),
            Token::String(loc, ..) => Some(loc),
            Token::Number(loc, ..) => Some(loc),
            Token::Block(loc, ..) => Some(loc),
            Token::Object(loc, ..) => Some(loc),
            Token::Assign(loc) => Some(loc),
            Token::Equality(loc) => Some(loc),
            Token::Pipe(loc) => Some(loc),
            Token::Eol(loc) => Some(loc),
            _ => None,
        }
    }
}

#[derive(Debug)]
pub struct Line(pub Vec<Token>, pub Option<Box<Line>>);
impl Line {
    pub fn new() -> Line {
        Line(vec![], None)
    }
    pub fn init(tokens: Vec<Token>) -> Line {
        Line(tokens, None)
    }
    pub fn add_token(&mut self, token: Token) {
        self.0.push(token)
    }
    pub fn pipe_line(&mut self, line: Line) {
        self.1 = Some(Box::new(line))
    }
    pub fn find_symbol(&self, symbol: &str) -> Option<usize> {
        for i in 0..self.0.len() {
            if let Some(Token::Symbol(.., value)) = self.0.get(i) {
                if value.as_str() == symbol {
                    return Some(i);
                }
            }
        }
        return None;
    }
    pub fn loc(&self) -> Location {
        if let Some(token) = self.0.first() {
            if let Some(loc) = token.loc() {
                loc.clone()
            } else {
                Location::unknown()
            }
        } else {
            Location::unknown()
        }
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
}

#[derive(Debug, Clone)]
pub struct Tokens(VecDeque<Token>);
impl Tokens {
    pub fn new() -> Tokens {
        Tokens(VecDeque::new())
    }
    pub fn one(value: Token) -> Tokens {
        let mut tokens = Tokens::new();
        tokens.add(value);
        return tokens;
    }
    pub fn next(&mut self) -> Option<(Token, Tokens)> {
        if let Some(first) = self.0.pop_front() {
            Some((first.clone(), Tokens(self.0.to_owned())))
        } else {
            None
        }
    }
    pub fn next_new(&mut self) -> Option<Token> {
        self.0.pop_front()
    }
    pub fn add(&mut self, token: Token) {
        match token {
            Token::None => (),
            _ => self.0.push_back(token),
        }
    }
    pub fn concat(&mut self, tokens: Tokens) {
        let mut clone = self.0.clone();
        for token in tokens.iter() {
            clone.push_back(token.to_owned())
        }
        self.0 = clone;
    }
    pub fn iter(&self) -> std::collections::vec_deque::Iter<Token> {
        self.0.iter()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn last(&self) -> Option<&Token> {
        self.0.get(self.0.len() - 1)
    }
    pub fn push_front(&mut self, token: Token) {
        self.0.push_front(token)
    }
    pub fn add_symbol(&mut self, location: Location, value: String) {
        self.add(Token::Symbol(location, value))
    }
    pub fn add_string(&mut self, location: Location, value: String) {
        self.add(Token::String(location, value))
    }
    pub fn add_number(&mut self, location: Location, value: String) {
        self.add(Token::Number(location, value))
    }
    pub fn add_pipe(&mut self, location: Location) {
        self.add(Token::Pipe(location))
    }
    pub fn add_eol(&mut self, location: Location) {
        self.add(Token::Eol(location))
    }
}

#[derive(Debug, Clone)]
pub struct Location(pub String, pub usize, pub usize);
impl Location {
    pub fn unknown() -> Location {
        Location("".to_string(), 0, 0)
    }
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.0, self.1, self.2)
    }
}

#[derive(Debug)]
pub struct ParseError(pub Location, pub String);
impl ParseError {
    pub fn message(&self) -> String {
        format!("ParseError: {}: {}", self.0.to_string(), self.1,)
    }
}

#[derive(Debug)]
pub struct RunError(pub Location, pub String);
impl RunError {
    pub fn message(&self) -> String {
        format!("RuntimeError: {}: {}", self.0.to_string(), self.1)
    }
}

pub enum RunResult {
    Ok(Vec<String>),
    Err(Location, String),
}
impl RunResult {
    pub fn ok(value: String) -> RunResult {
        RunResult::Ok(vec![value])
    }
    pub fn err(loc: Location, err: String) -> RunResult {
        RunResult::Err(loc, err)
    }
    pub fn merge(&mut self, result: &RunResult) {
        match self {
            RunResult::Ok(results) => match result {
                RunResult::Ok(values) => {
                    for value in values {
                        results.push(value.to_string())
                    }
                }
                RunResult::Err(..) => panic!("Attempted to merge RunResult::Err"),
            },
            RunResult::Err(..) => panic!("Attempted to merge RunResult::Err"),
        }
    }
    pub fn message(&self) -> String {
        match self {
            RunResult::Ok(results) => {
                let mut message: String = "".to_string();
                for result in results {
                    message += &(result.to_owned() + "\n");
                }
                return message;
            }
            RunResult::Err(loc, err) => {
                format!("RunError: {}: {}", loc.to_string(), err)
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum Declaration {
    Test(Location, String, Block),
}
#[derive(Debug, Clone)]
pub struct Declarations(Vec<Declaration>);
impl Declarations {
    pub fn new() -> Declarations {
        return Declarations(vec![]);
    }
    pub fn one(value: Declaration) -> Declarations {
        let mut declarations = Declarations::new();
        declarations.add(value);
        return declarations;
    }
    pub fn add(&mut self, value: Declaration) {
        self.0.push(value);
    }
    pub fn iter(&self) -> slice::Iter<Declaration> {
        self.0.iter()
    }
    pub fn iter_mut(&mut self) -> slice::IterMut<Declaration> {
        self.0.iter_mut()
    }
}

#[derive(Debug, Clone)]
pub enum Primitive {
    Void,
    Boolean(bool),
    Number(f32),
    String(String),
}

#[derive(Debug, Clone)]
pub enum Expression {
    Primitive(Location, Primitive),
    Assert(Location, Box<Expression>),
    Assignment(Location, Box<Expression>, Box<Expression>),
    Equality(Location, Box<Expression>, Box<Expression>),
    Ref(Location, String),
}
impl Expression {
    pub fn eval_in_scope(&self, scope: &Scope) -> Primitive {
        match self {
            Expression::Primitive(.., primitive) => primitive.clone(),
            Expression::Assert(.., expression) => expression.eval_in_scope(scope),
            Expression::Assignment(.., expression) => expression.eval_in_scope(scope),
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
                }
            }
            Expression::Ref(.., name) => match scope.get_var(name) {
                Some(primitive) => primitive.clone(),
                None => Primitive::Void,
            },
        }
    }
    pub fn loc(&self) -> &Location {
        match self {
            Expression::Assert(loc, ..) => loc,
            Expression::Assignment(loc, ..) => loc,
            Expression::Equality(loc, ..) => loc,
            Expression::Primitive(loc, ..) => loc,
            Expression::Ref(loc, ..) => loc,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Expressions(Vec<Expression>);
impl Expressions {
    pub fn new() -> Expressions {
        return Expressions(Vec::<Expression>::new());
    }
    pub fn next(&self) -> Option<(Expression, Expressions)> {
        if let Some((first, rest)) = self.0.split_first() {
            Some((first.clone(), Expressions(rest.to_vec())))
        } else {
            None
        }
    }
    pub fn concat(&mut self, expressions: Expressions) {
        self.0 = [self.0.clone(), expressions.0].concat();
    }
    pub fn add(&mut self, expression: Expression) {
        self.0.push(expression);
    }
    pub fn iter(&self) -> slice::Iter<Expression> {
        self.0.iter()
    }
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
pub struct Block(pub Location, pub Expressions, pub Scope);
impl Block {
    pub fn new(location: Location) -> Block {
        Block(location, Expressions::new(), Scope::new())
    }
    pub fn next(&self) -> Option<(Expression, Expressions)> {
        return self.1.next();
    }
    pub fn add_var(&mut self, name: &String) {
        self.2.add_var(name);
    }
    pub fn has_var(&self, name: &String) -> bool {
        self.2.has_var(name)
    }
    pub fn get_var(&self, name: &String) -> Option<&Primitive> {
        self.2.get_var(name)
    }
    pub fn set_var(&mut self, name: &String, value: &Primitive) {
        self.2.set_var(name, value);
    }
    pub fn add_expression(&mut self, expression: Expression) {
        self.1.add(expression);
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
    pub fn add_var(&mut self, name: &String) {
        if !self.has_var(name) {
            self.vars.insert(name.to_string(), Primitive::Void);
        }
    }
    pub fn has_var(&self, name: &String) -> bool {
        self.vars.contains_key(name)
    }
    pub fn get_var(&self, name: &String) -> Option<&Primitive> {
        self.vars.get(name)
    }
    pub fn set_var(&mut self, name: &String, value: &Primitive) {
        self.vars.insert(name.to_string(), value.clone());
    }
}
