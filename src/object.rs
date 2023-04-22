use core::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Object {
    Integer(i32),
    Boolean(bool),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Error(e) => write!(f, "Error: {}", e),
        }
    }
}
