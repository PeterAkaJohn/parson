#[derive(Debug)]
pub enum Value {
    String(String),
    Number(f64),
    Null,
    Boolean(bool),
}
