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
pub struct Tokens<T>(Vec<T>);
impl<T: Clone> Tokens<T> {
    pub fn new() -> Tokens<T> {
        Tokens(Vec::<T>::new())
    }
    pub fn next(&self) -> Option<(T, Tokens<T>)> {
        if let Some((first, rest)) = self.0.split_first() {
            Some((first.clone(), Tokens(rest.to_vec())))
        } else {
            None
        }
    }

    pub fn add(&mut self, token: T) -> &mut Tokens<T> {
        self.0.push(token);
        return self;
    }

    pub fn concat(&mut self, tokens: Tokens<T>) -> &mut Tokens<T> {
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

#[derive(Debug, Clone)]
pub enum AST {
    Test(Location, String, Block),
}

#[derive(Debug, Clone)]
pub struct Block(pub Location, pub Tokens<AST>);
