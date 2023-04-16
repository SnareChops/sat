use std::slice;

#[derive(Clone, Debug)]
pub enum Token {
    None,
    Symbol(Location, String),
    String(Location, String),
    Number(Location, String),
    BlockStart(Location),
    BlockEnd(Location),
}

#[derive(Debug, Clone)]
pub struct Tokens(Vec<Token>);
impl Tokens {
    pub fn new() -> Tokens {
        Tokens(Vec::new())
    }

    pub fn one(value: Token) -> Tokens {
        let mut tokens = Tokens::new();
        tokens.add(value);
        return tokens;
    }

    pub fn next(&self) -> Option<(Token, Tokens)> {
        if let Some((first, rest)) = self.0.split_first() {
            Some((first.clone(), Tokens(rest.to_vec())))
        } else {
            None
        }
    }

    pub fn add(&mut self, token: Token) -> &mut Tokens {
        self.0.push(token);
        return self;
    }

    pub fn concat(&mut self, tokens: Tokens) -> &mut Tokens {
        self.0 = [self.0.clone(), tokens.0].concat();
        return self;
    }
}

#[derive(Debug, Clone)]
pub struct Location(pub String, pub i32, pub i32);

impl Location {
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

    pub fn merge(&mut self, result: RunResult) -> &mut RunResult {
        match self {
            RunResult::Ok(results) => match result {
                RunResult::Ok(values) => {
                    for value in values {
                        results.push(value)
                    }
                }
                RunResult::Err(..) => panic!("Attempted to merge RunResult::Err"),
            },
            RunResult::Err(..) => panic!("Attempted to merge RunResult::Err"),
        }
        return self;
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
}
#[derive(Debug, Clone)]
pub enum Expression {
    Boolean(Location, bool),
    Assert(Location, Box<Expression>),
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
pub struct Block(pub Location, pub Expressions);
impl Block {
    pub fn next(&self) -> Option<(Expression, Expressions)> {
        return self.1.next();
    }
}
