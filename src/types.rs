#[derive(Debug, Clone)]
pub struct Location(pub String, pub usize, pub usize);
impl Location {
    pub fn to_string(&self) -> String {
        format!("{}:{}:{}", self.0, self.1, self.2)
    }
}
